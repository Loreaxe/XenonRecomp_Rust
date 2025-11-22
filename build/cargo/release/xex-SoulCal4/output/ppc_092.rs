pub fn sub_82602C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602C88 size=112
    let mut pc: u32 = 0x82602C88;
    'dispatch: loop {
        match pc {
            0x82602C88 => {
    //   block [0x82602C88..0x82602CF8)
	// 82602C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602C94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602C98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602C9C: 38AA365C  addi r5, r10, 0x365c
	ctx.r[5].s64 = ctx.r[10].s64 + 13916;
	// 82602CA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602CA4: 390B6AE0  addi r8, r11, 0x6ae0
	ctx.r[8].s64 = ctx.r[11].s64 + 27360;
	// 82602CA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82602CAC: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 82602CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602CB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602CC0: 386A35FC  addi r3, r10, 0x35fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13820;
	// 82602CC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602CE4: 4BE6413D  bl 0x82466e20
	ctx.lr = 0x82602CE8;
	sub_82466E20(ctx, base);
	// 82602CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602CF8 size=112
    let mut pc: u32 = 0x82602CF8;
    'dispatch: loop {
        match pc {
            0x82602CF8 => {
    //   block [0x82602CF8..0x82602D68)
	// 82602CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602D04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D08: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602D0C: 38AA368C  addi r5, r10, 0x368c
	ctx.r[5].s64 = ctx.r[10].s64 + 13964;
	// 82602D10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602D14: 390B6B10  addi r8, r11, 0x6b10
	ctx.r[8].s64 = ctx.r[11].s64 + 27408;
	// 82602D18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82602D1C: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 82602D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602D24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602D30: 386A362C  addi r3, r10, 0x362c
	ctx.r[3].s64 = ctx.r[10].s64 + 13868;
	// 82602D34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602D4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602D54: 4BE640CD  bl 0x82466e20
	ctx.lr = 0x82602D58;
	sub_82466E20(ctx, base);
	// 82602D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602D68 size=112
    let mut pc: u32 = 0x82602D68;
    'dispatch: loop {
        match pc {
            0x82602D68 => {
    //   block [0x82602D68..0x82602DD8)
	// 82602D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602D74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602D7C: 38AA37AC  addi r5, r10, 0x37ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14252;
	// 82602D80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602D84: 390B6B28  addi r8, r11, 0x6b28
	ctx.r[8].s64 = ctx.r[11].s64 + 27432;
	// 82602D88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82602D8C: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 82602D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602D94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602DA0: 386A365C  addi r3, r10, 0x365c
	ctx.r[3].s64 = ctx.r[10].s64 + 13916;
	// 82602DA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602DC4: 4BE6405D  bl 0x82466e20
	ctx.lr = 0x82602DC8;
	sub_82466E20(ctx, base);
	// 82602DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602DD8 size=112
    let mut pc: u32 = 0x82602DD8;
    'dispatch: loop {
        match pc {
            0x82602DD8 => {
    //   block [0x82602DD8..0x82602E48)
	// 82602DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602DE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602DE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602DEC: 38AA365C  addi r5, r10, 0x365c
	ctx.r[5].s64 = ctx.r[10].s64 + 13916;
	// 82602DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602DF4: 390B6B58  addi r8, r11, 0x6b58
	ctx.r[8].s64 = ctx.r[11].s64 + 27480;
	// 82602DF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82602DFC: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 82602E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602E04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602E08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602E10: 386A368C  addi r3, r10, 0x368c
	ctx.r[3].s64 = ctx.r[10].s64 + 13964;
	// 82602E14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602E34: 4BE63FED  bl 0x82466e20
	ctx.lr = 0x82602E38;
	sub_82466E20(ctx, base);
	// 82602E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602E48 size=108
    let mut pc: u32 = 0x82602E48;
    'dispatch: loop {
        match pc {
            0x82602E48 => {
    //   block [0x82602E48..0x82602EB4)
	// 82602E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602E54: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602E5C: 38EB6B70  addi r7, r11, 0x6b70
	ctx.r[7].s64 = ctx.r[11].s64 + 27504;
	// 82602E60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82602E64: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 82602E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602E6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602E70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602E78: 386A36BC  addi r3, r10, 0x36bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14012;
	// 82602E7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602EA0: 4BE63F81  bl 0x82466e20
	ctx.lr = 0x82602EA4;
	sub_82466E20(ctx, base);
	// 82602EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602EB8 size=112
    let mut pc: u32 = 0x82602EB8;
    'dispatch: loop {
        match pc {
            0x82602EB8 => {
    //   block [0x82602EB8..0x82602F28)
	// 82602EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602EC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602EC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602ECC: 38AA368C  addi r5, r10, 0x368c
	ctx.r[5].s64 = ctx.r[10].s64 + 13964;
	// 82602ED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602ED4: 390B6B88  addi r8, r11, 0x6b88
	ctx.r[8].s64 = ctx.r[11].s64 + 27528;
	// 82602ED8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82602EDC: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 82602EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602EF0: 386A36EC  addi r3, r10, 0x36ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14060;
	// 82602EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602F14: 4BE63F0D  bl 0x82466e20
	ctx.lr = 0x82602F18;
	sub_82466E20(ctx, base);
	// 82602F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602F28 size=116
    let mut pc: u32 = 0x82602F28;
    'dispatch: loop {
        match pc {
            0x82602F28 => {
    //   block [0x82602F28..0x82602F9C)
	// 82602F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602F34: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82602F38: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82602F3C: 390A6BA0  addi r8, r10, 0x6ba0
	ctx.r[8].s64 = ctx.r[10].s64 + 27552;
	// 82602F40: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602F44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82602F48: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82602F4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602F50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82602F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602F58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602F5C: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 82602F60: 396B98EC  addi r11, r11, -0x6714
	ctx.r[11].s64 = ctx.r[11].s64 + -26388;
	// 82602F64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602F68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602F6C: 386A371C  addi r3, r10, 0x371c
	ctx.r[3].s64 = ctx.r[10].s64 + 14108;
	// 82602F70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82602F74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602F78: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82602F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602F88: 4BE63E99  bl 0x82466e20
	ctx.lr = 0x82602F8C;
	sub_82466E20(ctx, base);
	// 82602F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602FA0 size=116
    let mut pc: u32 = 0x82602FA0;
    'dispatch: loop {
        match pc {
            0x82602FA0 => {
    //   block [0x82602FA0..0x82603014)
	// 82602FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602FAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602FB0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82602FB4: 392A9A14  addi r9, r10, -0x65ec
	ctx.r[9].s64 = ctx.r[10].s64 + -26092;
	// 82602FB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602FBC: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 82602FC0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82602FC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602FC8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82602FCC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602FD0: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 82602FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602FD8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82602FDC: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 82602FE0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602FE8: 386A374C  addi r3, r10, 0x374c
	ctx.r[3].s64 = ctx.r[10].s64 + 14156;
	// 82602FEC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82602FF0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82602FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602FF8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82602FFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82603000: 4BE63E21  bl 0x82466e20
	ctx.lr = 0x82603004;
	sub_82466E20(ctx, base);
	// 82603004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260300C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82603018 size=48
    let mut pc: u32 = 0x82603018;
    'dispatch: loop {
        match pc {
            0x82603018 => {
    //   block [0x82603018..0x82603048)
	// 82603018: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260301C: 814B72B8  lwz r10, 0x72b8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29368 as u32) ) } as u64;
	// 82603020: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603024: 396BB4F0  addi r11, r11, -0x4b10
	ctx.r[11].s64 = ctx.r[11].s64 + -19216;
	// 82603028: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8260302C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82603030: 814A72B4  lwz r10, 0x72b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29364 as u32) ) } as u64;
	// 82603034: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82603038: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 8260303C: 814A72B0  lwz r10, 0x72b0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29360 as u32) ) } as u64;
	// 82603040: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 82603044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603048 size=116
    let mut pc: u32 = 0x82603048;
    'dispatch: loop {
        match pc {
            0x82603048 => {
    //   block [0x82603048..0x826030BC)
	// 82603048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260304C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603054: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82603058: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260305C: 392B9AF0  addi r9, r11, -0x6510
	ctx.r[9].s64 = ctx.r[11].s64 + -25872;
	// 82603060: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603064: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603068: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8260306C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82603070: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603074: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 82603078: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260307C: 396BB4F0  addi r11, r11, -0x4b10
	ctx.r[11].s64 = ctx.r[11].s64 + -19216;
	// 82603080: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82603084: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603088: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260308C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603090: 386A377C  addi r3, r10, 0x377c
	ctx.r[3].s64 = ctx.r[10].s64 + 14204;
	// 82603094: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82603098: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260309C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826030A0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826030A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826030A8: 4BE63D79  bl 0x82466e20
	ctx.lr = 0x826030AC;
	sub_82466E20(ctx, base);
	// 826030AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826030B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826030B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826030B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826030C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826030C0 size=116
    let mut pc: u32 = 0x826030C0;
    'dispatch: loop {
        match pc {
            0x826030C0 => {
    //   block [0x826030C0..0x82603134)
	// 826030C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826030C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826030C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826030CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826030D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826030D4: 390B72C8  addi r8, r11, 0x72c8
	ctx.r[8].s64 = ctx.r[11].s64 + 29384;
	// 826030D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826030DC: 392A9C8C  addi r9, r10, -0x6374
	ctx.r[9].s64 = ctx.r[10].s64 + -25460;
	// 826030E0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826030E4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826030E8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826030EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826030F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826030F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826030F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826030FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603104: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82603108: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 8260310C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82603110: 386B37AC  addi r3, r11, 0x37ac
	ctx.r[3].s64 = ctx.r[11].s64 + 14252;
	// 82603114: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82603118: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260311C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603120: 4BE63D01  bl 0x82466e20
	ctx.lr = 0x82603124;
	sub_82466E20(ctx, base);
	// 82603124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260312C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603138 size=112
    let mut pc: u32 = 0x82603138;
    'dispatch: loop {
        match pc {
            0x82603138 => {
    //   block [0x82603138..0x826031A8)
	// 82603138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603144: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603148: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260314C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603154: 390B7358  addi r8, r11, 0x7358
	ctx.r[8].s64 = ctx.r[11].s64 + 29528;
	// 82603158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260315C: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 82603160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603164: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260316C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603170: 386A37DC  addi r3, r10, 0x37dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14300;
	// 82603174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260317C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260318C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603194: 4BE63C8D  bl 0x82466e20
	ctx.lr = 0x82603198;
	sub_82466E20(ctx, base);
	// 82603198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260319C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826031A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826031A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826031A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826031A8 size=36
    let mut pc: u32 = 0x826031A8;
    'dispatch: loop {
        match pc {
            0x826031A8 => {
    //   block [0x826031A8..0x826031CC)
	// 826031A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826031AC: 814B7374  lwz r10, 0x7374(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29556 as u32) ) } as u64;
	// 826031B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826031B4: 396BB898  addi r11, r11, -0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + -18280;
	// 826031B8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826031BC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 826031C0: 814A72C4  lwz r10, 0x72c4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29380 as u32) ) } as u64;
	// 826031C4: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 826031C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826031D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826031D0 size=116
    let mut pc: u32 = 0x826031D0;
    'dispatch: loop {
        match pc {
            0x826031D0 => {
    //   block [0x826031D0..0x82603244)
	// 826031D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826031D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826031D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826031DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826031E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826031E4: 392A9CF4  addi r9, r10, -0x630c
	ctx.r[9].s64 = ctx.r[10].s64 + -25356;
	// 826031E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826031EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826031F0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826031F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826031F8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826031FC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603200: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 82603204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603208: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260320C: 396BB898  addi r11, r11, -0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + -18280;
	// 82603210: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603218: 386A380C  addi r3, r10, 0x380c
	ctx.r[3].s64 = ctx.r[10].s64 + 14348;
	// 8260321C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82603220: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82603224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603228: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8260322C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82603230: 4BE63BF1  bl 0x82466e20
	ctx.lr = 0x82603234;
	sub_82466E20(ctx, base);
	// 82603234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260323C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603248 size=108
    let mut pc: u32 = 0x82603248;
    'dispatch: loop {
        match pc {
            0x82603248 => {
    //   block [0x82603248..0x826032B4)
	// 82603248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260324C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603254: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260325C: 38EB7378  addi r7, r11, 0x7378
	ctx.r[7].s64 = ctx.r[11].s64 + 29560;
	// 82603260: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82603264: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 82603268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260326C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603278: 386A383C  addi r3, r10, 0x383c
	ctx.r[3].s64 = ctx.r[10].s64 + 14396;
	// 8260327C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260328C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260329C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826032A0: 4BE63B81  bl 0x82466e20
	ctx.lr = 0x826032A4;
	sub_82466E20(ctx, base);
	// 826032A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826032A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826032AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826032B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826032B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826032B8 size=112
    let mut pc: u32 = 0x826032B8;
    'dispatch: loop {
        match pc {
            0x826032B8 => {
    //   block [0x826032B8..0x82603328)
	// 826032B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826032BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826032C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826032C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826032C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826032CC: 38AA155C  addi r5, r10, 0x155c
	ctx.r[5].s64 = ctx.r[10].s64 + 5468;
	// 826032D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826032D4: 390B73F0  addi r8, r11, 0x73f0
	ctx.r[8].s64 = ctx.r[11].s64 + 29680;
	// 826032D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826032DC: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 826032E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826032E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826032E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826032EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826032F0: 386A386C  addi r3, r10, 0x386c
	ctx.r[3].s64 = ctx.r[10].s64 + 14444;
	// 826032F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826032F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826032FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260330C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603314: 4BE63B0D  bl 0x82466e20
	ctx.lr = 0x82603318;
	sub_82466E20(ctx, base);
	// 82603318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260331C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603328 size=108
    let mut pc: u32 = 0x82603328;
    'dispatch: loop {
        match pc {
            0x82603328 => {
    //   block [0x82603328..0x82603394)
	// 82603328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260332C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603334: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260333C: 38EB7408  addi r7, r11, 0x7408
	ctx.r[7].s64 = ctx.r[11].s64 + 29704;
	// 82603340: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603344: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 82603348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260334C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603358: 386A389C  addi r3, r10, 0x389c
	ctx.r[3].s64 = ctx.r[10].s64 + 14492;
	// 8260335C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260336C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260337C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603380: 4BE63AA1  bl 0x82466e20
	ctx.lr = 0x82603384;
	sub_82466E20(ctx, base);
	// 82603384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260338C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603398 size=112
    let mut pc: u32 = 0x82603398;
    'dispatch: loop {
        match pc {
            0x82603398 => {
    //   block [0x82603398..0x82603408)
	// 82603398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260339C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826033A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826033A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826033A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826033AC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826033B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826033B4: 390B7420  addi r8, r11, 0x7420
	ctx.r[8].s64 = ctx.r[11].s64 + 29728;
	// 826033B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826033BC: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 826033C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826033C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826033C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826033CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826033D0: 386A38CC  addi r3, r10, 0x38cc
	ctx.r[3].s64 = ctx.r[10].s64 + 14540;
	// 826033D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826033D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826033DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826033E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826033E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826033E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826033EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826033F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826033F4: 4BE63A2D  bl 0x82466e20
	ctx.lr = 0x826033F8;
	sub_82466E20(ctx, base);
	// 826033F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826033FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603408 size=108
    let mut pc: u32 = 0x82603408;
    'dispatch: loop {
        match pc {
            0x82603408 => {
    //   block [0x82603408..0x82603474)
	// 82603408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260340C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603414: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260341C: 38EB7468  addi r7, r11, 0x7468
	ctx.r[7].s64 = ctx.r[11].s64 + 29800;
	// 82603420: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603424: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 82603428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260342C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603438: 386A38FC  addi r3, r10, 0x38fc
	ctx.r[3].s64 = ctx.r[10].s64 + 14588;
	// 8260343C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260344C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260345C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603460: 4BE639C1  bl 0x82466e20
	ctx.lr = 0x82603464;
	sub_82466E20(ctx, base);
	// 82603464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260346C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603478 size=108
    let mut pc: u32 = 0x82603478;
    'dispatch: loop {
        match pc {
            0x82603478 => {
    //   block [0x82603478..0x826034E4)
	// 82603478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260347C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603484: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603488: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260348C: 38EB7498  addi r7, r11, 0x7498
	ctx.r[7].s64 = ctx.r[11].s64 + 29848;
	// 82603490: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603494: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 82603498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260349C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826034A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826034A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826034A8: 386A392C  addi r3, r10, 0x392c
	ctx.r[3].s64 = ctx.r[10].s64 + 14636;
	// 826034AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826034B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826034B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826034B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826034BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826034C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826034C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826034C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826034CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826034D0: 4BE63951  bl 0x82466e20
	ctx.lr = 0x826034D4;
	sub_82466E20(ctx, base);
	// 826034D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826034D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826034DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826034E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826034E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826034E8 size=112
    let mut pc: u32 = 0x826034E8;
    'dispatch: loop {
        match pc {
            0x826034E8 => {
    //   block [0x826034E8..0x82603558)
	// 826034E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826034EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826034F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826034F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826034F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826034FC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603500: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603504: 390B74B0  addi r8, r11, 0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29872;
	// 82603508: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260350C: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 82603510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603514: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260351C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603520: 386A395C  addi r3, r10, 0x395c
	ctx.r[3].s64 = ctx.r[10].s64 + 14684;
	// 82603524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260352C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260353C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603544: 4BE638DD  bl 0x82466e20
	ctx.lr = 0x82603548;
	sub_82466E20(ctx, base);
	// 82603548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260354C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603558 size=112
    let mut pc: u32 = 0x82603558;
    'dispatch: loop {
        match pc {
            0x82603558 => {
    //   block [0x82603558..0x826035C8)
	// 82603558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260355C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603568: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260356C: 38AA281C  addi r5, r10, 0x281c
	ctx.r[5].s64 = ctx.r[10].s64 + 10268;
	// 82603570: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603574: 390B74E0  addi r8, r11, 0x74e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29920;
	// 82603578: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260357C: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 82603580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603584: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603588: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260358C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603590: 386A398C  addi r3, r10, 0x398c
	ctx.r[3].s64 = ctx.r[10].s64 + 14732;
	// 82603594: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260359C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826035A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826035A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826035A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826035AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826035B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826035B4: 4BE6386D  bl 0x82466e20
	ctx.lr = 0x826035B8;
	sub_82466E20(ctx, base);
	// 826035B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826035BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826035C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826035C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826035C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826035C8 size=112
    let mut pc: u32 = 0x826035C8;
    'dispatch: loop {
        match pc {
            0x826035C8 => {
    //   block [0x826035C8..0x82603638)
	// 826035C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826035CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826035D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826035D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826035D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826035DC: 38AA281C  addi r5, r10, 0x281c
	ctx.r[5].s64 = ctx.r[10].s64 + 10268;
	// 826035E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826035E4: 390B7528  addi r8, r11, 0x7528
	ctx.r[8].s64 = ctx.r[11].s64 + 29992;
	// 826035E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826035EC: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 826035F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826035F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826035F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826035FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603600: 386A39BC  addi r3, r10, 0x39bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14780;
	// 82603604: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260360C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260361C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603624: 4BE637FD  bl 0x82466e20
	ctx.lr = 0x82603628;
	sub_82466E20(ctx, base);
	// 82603628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260362C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603638 size=112
    let mut pc: u32 = 0x82603638;
    'dispatch: loop {
        match pc {
            0x82603638 => {
    //   block [0x82603638..0x826036A8)
	// 82603638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260363C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603648: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260364C: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 82603650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603654: 390B7588  addi r8, r11, 0x7588
	ctx.r[8].s64 = ctx.r[11].s64 + 30088;
	// 82603658: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260365C: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 82603660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260366C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603670: 386A39EC  addi r3, r10, 0x39ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14828;
	// 82603674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260367C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260368C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603694: 4BE6378D  bl 0x82466e20
	ctx.lr = 0x82603698;
	sub_82466E20(ctx, base);
	// 82603698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260369C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826036A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826036A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826036A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826036A8 size=112
    let mut pc: u32 = 0x826036A8;
    'dispatch: loop {
        match pc {
            0x826036A8 => {
    //   block [0x826036A8..0x82603718)
	// 826036A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826036AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826036B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826036B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826036B8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826036BC: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 826036C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826036C4: 390B75E8  addi r8, r11, 0x75e8
	ctx.r[8].s64 = ctx.r[11].s64 + 30184;
	// 826036C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826036CC: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 826036D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826036D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826036D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826036DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826036E0: 386A3A1C  addi r3, r10, 0x3a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 14876;
	// 826036E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826036E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826036EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826036F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826036F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826036F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826036FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603704: 4BE6371D  bl 0x82466e20
	ctx.lr = 0x82603708;
	sub_82466E20(ctx, base);
	// 82603708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260370C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603718 size=112
    let mut pc: u32 = 0x82603718;
    'dispatch: loop {
        match pc {
            0x82603718 => {
    //   block [0x82603718..0x82603788)
	// 82603718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260371C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603728: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260372C: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 82603730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603734: 390B76A8  addi r8, r11, 0x76a8
	ctx.r[8].s64 = ctx.r[11].s64 + 30376;
	// 82603738: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260373C: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 82603740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260374C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603750: 386A3A4C  addi r3, r10, 0x3a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 14924;
	// 82603754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260375C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260376C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603774: 4BE636AD  bl 0x82466e20
	ctx.lr = 0x82603778;
	sub_82466E20(ctx, base);
	// 82603778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260377C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603788 size=112
    let mut pc: u32 = 0x82603788;
    'dispatch: loop {
        match pc {
            0x82603788 => {
    //   block [0x82603788..0x826037F8)
	// 82603788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260378C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603794: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603798: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260379C: 38AA281C  addi r5, r10, 0x281c
	ctx.r[5].s64 = ctx.r[10].s64 + 10268;
	// 826037A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826037A4: 390B7708  addi r8, r11, 0x7708
	ctx.r[8].s64 = ctx.r[11].s64 + 30472;
	// 826037A8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826037AC: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 826037B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826037B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826037B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826037BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826037C0: 386A3A7C  addi r3, r10, 0x3a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 14972;
	// 826037C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826037C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826037CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826037D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826037D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826037D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826037DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826037E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826037E4: 4BE6363D  bl 0x82466e20
	ctx.lr = 0x826037E8;
	sub_82466E20(ctx, base);
	// 826037E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826037EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826037F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826037F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826037F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826037F8 size=112
    let mut pc: u32 = 0x826037F8;
    'dispatch: loop {
        match pc {
            0x826037F8 => {
    //   block [0x826037F8..0x82603868)
	// 826037F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826037FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603804: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82603808: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8260380C: 38EA77C8  addi r7, r10, 0x77c8
	ctx.r[7].s64 = ctx.r[10].s64 + 30664;
	// 82603810: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603814: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82603818: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 8260381C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603820: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603824: 396B9D20  addi r11, r11, -0x62e0
	ctx.r[11].s64 = ctx.r[11].s64 + -25312;
	// 82603828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260382C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603830: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603834: 386A3AAC  addi r3, r10, 0x3aac
	ctx.r[3].s64 = ctx.r[10].s64 + 15020;
	// 82603838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260383C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82603840: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603844: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82603848: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260384C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603850: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603854: 4BE635CD  bl 0x82466e20
	ctx.lr = 0x82603858;
	sub_82466E20(ctx, base);
	// 82603858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260385C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603868 size=112
    let mut pc: u32 = 0x82603868;
    'dispatch: loop {
        match pc {
            0x82603868 => {
    //   block [0x82603868..0x826038D8)
	// 82603868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260386C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603874: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603878: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260387C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603884: 390B7990  addi r8, r11, 0x7990
	ctx.r[8].s64 = ctx.r[11].s64 + 31120;
	// 82603888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260388C: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 82603890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603894: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260389C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826038A0: 386A3ADC  addi r3, r10, 0x3adc
	ctx.r[3].s64 = ctx.r[10].s64 + 15068;
	// 826038A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826038A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826038AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826038B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826038B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826038B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826038BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826038C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826038C4: 4BE6355D  bl 0x82466e20
	ctx.lr = 0x826038C8;
	sub_82466E20(ctx, base);
	// 826038C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826038CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826038D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826038D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826038D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826038D8 size=108
    let mut pc: u32 = 0x826038D8;
    'dispatch: loop {
        match pc {
            0x826038D8 => {
    //   block [0x826038D8..0x82603944)
	// 826038D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826038DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826038E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826038E4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826038E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826038EC: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 826038F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826038F4: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826038F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826038FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603908: 386A3B0C  addi r3, r10, 0x3b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15116;
	// 8260390C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260391C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260392C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603930: 4BE634F1  bl 0x82466e20
	ctx.lr = 0x82603934;
	sub_82466E20(ctx, base);
	// 82603934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260393C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603948 size=112
    let mut pc: u32 = 0x82603948;
    'dispatch: loop {
        match pc {
            0x82603948 => {
    //   block [0x82603948..0x826039B8)
	// 82603948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260394C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603958: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260395C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603960: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603964: 390B79D8  addi r8, r11, 0x79d8
	ctx.r[8].s64 = ctx.r[11].s64 + 31192;
	// 82603968: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260396C: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 82603970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603974: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260397C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603980: 386A3B3C  addi r3, r10, 0x3b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15164;
	// 82603984: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260398C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603994: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260399C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826039A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826039A4: 4BE6347D  bl 0x82466e20
	ctx.lr = 0x826039A8;
	sub_82466E20(ctx, base);
	// 826039A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826039AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826039B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826039B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826039B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826039B8 size=108
    let mut pc: u32 = 0x826039B8;
    'dispatch: loop {
        match pc {
            0x826039B8 => {
    //   block [0x826039B8..0x82603A24)
	// 826039B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826039BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826039C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826039C4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826039C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826039CC: 38EB79F0  addi r7, r11, 0x79f0
	ctx.r[7].s64 = ctx.r[11].s64 + 31216;
	// 826039D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826039D4: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826039D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826039DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826039E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826039E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826039E8: 386A3B6C  addi r3, r10, 0x3b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15212;
	// 826039EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826039F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826039F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826039F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826039FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603A0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603A10: 4BE63411  bl 0x82466e20
	ctx.lr = 0x82603A14;
	sub_82466E20(ctx, base);
	// 82603A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603A28 size=108
    let mut pc: u32 = 0x82603A28;
    'dispatch: loop {
        match pc {
            0x82603A28 => {
    //   block [0x82603A28..0x82603A94)
	// 82603A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603A34: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603A38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603A3C: 38EB7A20  addi r7, r11, 0x7a20
	ctx.r[7].s64 = ctx.r[11].s64 + 31264;
	// 82603A40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82603A44: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 82603A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603A4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603A58: 386A3B9C  addi r3, r10, 0x3b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15260;
	// 82603A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603A80: 4BE633A1  bl 0x82466e20
	ctx.lr = 0x82603A84;
	sub_82466E20(ctx, base);
	// 82603A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603A98 size=112
    let mut pc: u32 = 0x82603A98;
    'dispatch: loop {
        match pc {
            0x82603A98 => {
    //   block [0x82603A98..0x82603B08)
	// 82603A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603AA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603AA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603AAC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603AB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603AB4: 390B7A68  addi r8, r11, 0x7a68
	ctx.r[8].s64 = ctx.r[11].s64 + 31336;
	// 82603AB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82603ABC: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 82603AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603AC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603AD0: 386A3BCC  addi r3, r10, 0x3bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 15308;
	// 82603AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603AF4: 4BE6332D  bl 0x82466e20
	ctx.lr = 0x82603AF8;
	sub_82466E20(ctx, base);
	// 82603AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603B08 size=112
    let mut pc: u32 = 0x82603B08;
    'dispatch: loop {
        match pc {
            0x82603B08 => {
    //   block [0x82603B08..0x82603B78)
	// 82603B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603B14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603B18: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603B1C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603B20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603B24: 390B7AB0  addi r8, r11, 0x7ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 31408;
	// 82603B28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82603B2C: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 82603B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603B34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603B40: 386A3BFC  addi r3, r10, 0x3bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 15356;
	// 82603B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603B54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603B64: 4BE632BD  bl 0x82466e20
	ctx.lr = 0x82603B68;
	sub_82466E20(ctx, base);
	// 82603B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603B78 size=112
    let mut pc: u32 = 0x82603B78;
    'dispatch: loop {
        match pc {
            0x82603B78 => {
    //   block [0x82603B78..0x82603BE8)
	// 82603B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603B84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603B88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603B8C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603B90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603B94: 390B7AC8  addi r8, r11, 0x7ac8
	ctx.r[8].s64 = ctx.r[11].s64 + 31432;
	// 82603B98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82603B9C: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 82603BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603BA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603BB0: 386A3C2C  addi r3, r10, 0x3c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15404;
	// 82603BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603BD4: 4BE6324D  bl 0x82466e20
	ctx.lr = 0x82603BD8;
	sub_82466E20(ctx, base);
	// 82603BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603BE8 size=112
    let mut pc: u32 = 0x82603BE8;
    'dispatch: loop {
        match pc {
            0x82603BE8 => {
    //   block [0x82603BE8..0x82603C58)
	// 82603BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603BF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603BF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603BFC: 38AA371C  addi r5, r10, 0x371c
	ctx.r[5].s64 = ctx.r[10].s64 + 14108;
	// 82603C00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603C04: 390B7AF8  addi r8, r11, 0x7af8
	ctx.r[8].s64 = ctx.r[11].s64 + 31480;
	// 82603C08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82603C0C: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 82603C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603C14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603C20: 386A3C5C  addi r3, r10, 0x3c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15452;
	// 82603C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603C44: 4BE631DD  bl 0x82466e20
	ctx.lr = 0x82603C48;
	sub_82466E20(ctx, base);
	// 82603C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603C58 size=108
    let mut pc: u32 = 0x82603C58;
    'dispatch: loop {
        match pc {
            0x82603C58 => {
    //   block [0x82603C58..0x82603CC4)
	// 82603C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603C64: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603C68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603C6C: 38EB7B10  addi r7, r11, 0x7b10
	ctx.r[7].s64 = ctx.r[11].s64 + 31504;
	// 82603C70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603C74: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 82603C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603C7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603C88: 386A3C8C  addi r3, r10, 0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 15500;
	// 82603C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603CB0: 4BE63171  bl 0x82466e20
	ctx.lr = 0x82603CB4;
	sub_82466E20(ctx, base);
	// 82603CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603CC8 size=112
    let mut pc: u32 = 0x82603CC8;
    'dispatch: loop {
        match pc {
            0x82603CC8 => {
    //   block [0x82603CC8..0x82603D38)
	// 82603CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603CD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603CD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603CDC: 38AA3C8C  addi r5, r10, 0x3c8c
	ctx.r[5].s64 = ctx.r[10].s64 + 15500;
	// 82603CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603CE4: 390B7B40  addi r8, r11, 0x7b40
	ctx.r[8].s64 = ctx.r[11].s64 + 31552;
	// 82603CE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82603CEC: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 82603CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603D00: 386A3CBC  addi r3, r10, 0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 15548;
	// 82603D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603D24: 4BE630FD  bl 0x82466e20
	ctx.lr = 0x82603D28;
	sub_82466E20(ctx, base);
	// 82603D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82603D38 size=24
    let mut pc: u32 = 0x82603D38;
    'dispatch: loop {
        match pc {
            0x82603D38 => {
    //   block [0x82603D38..0x82603D50)
	// 82603D38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603D3C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82603D40: 394AB958  addi r10, r10, -0x46a8
	ctx.r[10].s64 = ctx.r[10].s64 + -18088;
	// 82603D44: 816B7B70  lwz r11, 0x7b70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31600 as u32) ) } as u64;
	// 82603D48: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82603D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603D50 size=116
    let mut pc: u32 = 0x82603D50;
    'dispatch: loop {
        match pc {
            0x82603D50 => {
    //   block [0x82603D50..0x82603DC4)
	// 82603D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603D5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603D60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603D64: 390BB958  addi r8, r11, -0x46a8
	ctx.r[8].s64 = ctx.r[11].s64 + -18088;
	// 82603D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603D6C: 392A9DC0  addi r9, r10, -0x6240
	ctx.r[9].s64 = ctx.r[10].s64 + -25152;
	// 82603D70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603D74: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 82603D78: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603D7C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603D84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603D94: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82603D98: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 82603D9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82603DA0: 386B3CEC  addi r3, r11, 0x3cec
	ctx.r[3].s64 = ctx.r[11].s64 + 15596;
	// 82603DA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82603DA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603DB0: 4BE63071  bl 0x82466e20
	ctx.lr = 0x82603DB4;
	sub_82466E20(ctx, base);
	// 82603DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603DC8 size=108
    let mut pc: u32 = 0x82603DC8;
    'dispatch: loop {
        match pc {
            0x82603DC8 => {
    //   block [0x82603DC8..0x82603E34)
	// 82603DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603DD4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603DD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603DDC: 38EB7B78  addi r7, r11, 0x7b78
	ctx.r[7].s64 = ctx.r[11].s64 + 31608;
	// 82603DE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82603DE4: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 82603DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603DEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603DF8: 386A3D1C  addi r3, r10, 0x3d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 15644;
	// 82603DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603E20: 4BE63001  bl 0x82466e20
	ctx.lr = 0x82603E24;
	sub_82466E20(ctx, base);
	// 82603E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603E38 size=108
    let mut pc: u32 = 0x82603E38;
    'dispatch: loop {
        match pc {
            0x82603E38 => {
    //   block [0x82603E38..0x82603EA4)
	// 82603E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603E44: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603E48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603E4C: 38EB7BC0  addi r7, r11, 0x7bc0
	ctx.r[7].s64 = ctx.r[11].s64 + 31680;
	// 82603E50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603E54: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 82603E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603E5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603E68: 386A3D4C  addi r3, r10, 0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 15692;
	// 82603E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603E90: 4BE62F91  bl 0x82466e20
	ctx.lr = 0x82603E94;
	sub_82466E20(ctx, base);
	// 82603E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603EA8 size=108
    let mut pc: u32 = 0x82603EA8;
    'dispatch: loop {
        match pc {
            0x82603EA8 => {
    //   block [0x82603EA8..0x82603F14)
	// 82603EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603EB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603EBC: 38EB7BF0  addi r7, r11, 0x7bf0
	ctx.r[7].s64 = ctx.r[11].s64 + 31728;
	// 82603EC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603EC4: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 82603EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603ED8: 386A3D7C  addi r3, r10, 0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 15740;
	// 82603EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603F00: 4BE62F21  bl 0x82466e20
	ctx.lr = 0x82603F04;
	sub_82466E20(ctx, base);
	// 82603F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603F18 size=112
    let mut pc: u32 = 0x82603F18;
    'dispatch: loop {
        match pc {
            0x82603F18 => {
    //   block [0x82603F18..0x82603F88)
	// 82603F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603F24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603F28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603F2C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603F30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603F34: 390B7C20  addi r8, r11, 0x7c20
	ctx.r[8].s64 = ctx.r[11].s64 + 31776;
	// 82603F38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82603F3C: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 82603F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603F44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603F50: 386A3DAC  addi r3, r10, 0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + 15788;
	// 82603F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603F74: 4BE62EAD  bl 0x82466e20
	ctx.lr = 0x82603F78;
	sub_82466E20(ctx, base);
	// 82603F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603F88 size=112
    let mut pc: u32 = 0x82603F88;
    'dispatch: loop {
        match pc {
            0x82603F88 => {
    //   block [0x82603F88..0x82603FF8)
	// 82603F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603F94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603F98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603F9C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603FA4: 390B7C50  addi r8, r11, 0x7c50
	ctx.r[8].s64 = ctx.r[11].s64 + 31824;
	// 82603FA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82603FAC: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 82603FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603FB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603FC0: 386A3DDC  addi r3, r10, 0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 15836;
	// 82603FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603FE4: 4BE62E3D  bl 0x82466e20
	ctx.lr = 0x82603FE8;
	sub_82466E20(ctx, base);
	// 82603FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603FF8 size=112
    let mut pc: u32 = 0x82603FF8;
    'dispatch: loop {
        match pc {
            0x82603FF8 => {
    //   block [0x82603FF8..0x82604068)
	// 82603FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604008: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260400C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604014: 390B7C68  addi r8, r11, 0x7c68
	ctx.r[8].s64 = ctx.r[11].s64 + 31848;
	// 82604018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260401C: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 82604020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260402C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604030: 386A3E0C  addi r3, r10, 0x3e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15884;
	// 82604034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260403C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260404C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604054: 4BE62DCD  bl 0x82466e20
	ctx.lr = 0x82604058;
	sub_82466E20(ctx, base);
	// 82604058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260405C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604068 size=108
    let mut pc: u32 = 0x82604068;
    'dispatch: loop {
        match pc {
            0x82604068 => {
    //   block [0x82604068..0x826040D4)
	// 82604068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260406C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604074: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82604078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260407C: 38EB7C80  addi r7, r11, 0x7c80
	ctx.r[7].s64 = ctx.r[11].s64 + 31872;
	// 82604080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82604084: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 82604088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260408C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604098: 386A3E3C  addi r3, r10, 0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15932;
	// 8260409C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826040A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826040A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826040A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826040AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826040B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826040B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826040B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826040BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826040C0: 4BE62D61  bl 0x82466e20
	ctx.lr = 0x826040C4;
	sub_82466E20(ctx, base);
	// 826040C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826040C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826040CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826040D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826040D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826040D8 size=112
    let mut pc: u32 = 0x826040D8;
    'dispatch: loop {
        match pc {
            0x826040D8 => {
    //   block [0x826040D8..0x82604148)
	// 826040D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826040DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826040E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826040E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826040E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826040EC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826040F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826040F4: 390B7CB0  addi r8, r11, 0x7cb0
	ctx.r[8].s64 = ctx.r[11].s64 + 31920;
	// 826040F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826040FC: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 82604100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260410C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604110: 386A3E6C  addi r3, r10, 0x3e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15980;
	// 82604114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260411C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260412C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604134: 4BE62CED  bl 0x82466e20
	ctx.lr = 0x82604138;
	sub_82466E20(ctx, base);
	// 82604138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260413C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604148 size=108
    let mut pc: u32 = 0x82604148;
    'dispatch: loop {
        match pc {
            0x82604148 => {
    //   block [0x82604148..0x826041B4)
	// 82604148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260414C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604154: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82604158: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260415C: 38EB7CC8  addi r7, r11, 0x7cc8
	ctx.r[7].s64 = ctx.r[11].s64 + 31944;
	// 82604160: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82604164: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 82604168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260416C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604178: 386A3E9C  addi r3, r10, 0x3e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 16028;
	// 8260417C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260418C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260419C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826041A0: 4BE62C81  bl 0x82466e20
	ctx.lr = 0x826041A4;
	sub_82466E20(ctx, base);
	// 826041A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826041A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826041AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826041B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826041B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826041B8 size=116
    let mut pc: u32 = 0x826041B8;
    'dispatch: loop {
        match pc {
            0x826041B8 => {
    //   block [0x826041B8..0x8260422C)
	// 826041B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826041BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826041C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826041C4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 826041C8: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826041CC: 390A7DB8  addi r8, r10, 0x7db8
	ctx.r[8].s64 = ctx.r[10].s64 + 32184;
	// 826041D0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826041D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826041D8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826041DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826041E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826041E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826041E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826041EC: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826041F0: 396B9DD8  addi r11, r11, -0x6228
	ctx.r[11].s64 = ctx.r[11].s64 + -25128;
	// 826041F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826041F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826041FC: 386A3ECC  addi r3, r10, 0x3ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 16076;
	// 82604200: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82604204: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604208: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260420C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604218: 4BE62C09  bl 0x82466e20
	ctx.lr = 0x8260421C;
	sub_82466E20(ctx, base);
	// 8260421C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604230 size=108
    let mut pc: u32 = 0x82604230;
    'dispatch: loop {
        match pc {
            0x82604230 => {
    //   block [0x82604230..0x8260429C)
	// 82604230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260423C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82604240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604244: 38EB7F80  addi r7, r11, 0x7f80
	ctx.r[7].s64 = ctx.r[11].s64 + 32640;
	// 82604248: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8260424C: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 82604250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260425C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604260: 386A3EFC  addi r3, r10, 0x3efc
	ctx.r[3].s64 = ctx.r[10].s64 + 16124;
	// 82604264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260426C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260427C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604288: 4BE62B99  bl 0x82466e20
	ctx.lr = 0x8260428C;
	sub_82466E20(ctx, base);
	// 8260428C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826042A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826042A0 size=112
    let mut pc: u32 = 0x826042A0;
    'dispatch: loop {
        match pc {
            0x826042A0 => {
    //   block [0x826042A0..0x82604310)
	// 826042A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826042A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826042A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826042AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826042B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826042B4: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 826042B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826042BC: 390B8118  addi r8, r11, -0x7ee8
	ctx.r[8].s64 = ctx.r[11].s64 + -32488;
	// 826042C0: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826042C4: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826042C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826042CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826042D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826042D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826042D8: 386A3F2C  addi r3, r10, 0x3f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 16172;
	// 826042DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826042E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826042E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826042E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826042EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826042F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826042F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826042F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826042FC: 4BE62B25  bl 0x82466e20
	ctx.lr = 0x82604300;
	sub_82466E20(ctx, base);
	// 82604300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260430C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604310 size=100
    let mut pc: u32 = 0x82604310;
    'dispatch: loop {
        match pc {
            0x82604310 => {
    //   block [0x82604310..0x82604374)
	// 82604310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260431C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604324: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604328: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260432C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604330: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 82604334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260433C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604344: 386A3F5C  addi r3, r10, 0x3f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 16220;
	// 82604348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260434C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604350: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604358: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260435C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604360: 4BE62AC1  bl 0x82466e20
	ctx.lr = 0x82604364;
	sub_82466E20(ctx, base);
	// 82604364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260436C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604378 size=112
    let mut pc: u32 = 0x82604378;
    'dispatch: loop {
        match pc {
            0x82604378 => {
    //   block [0x82604378..0x826043E8)
	// 82604378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604388: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260438C: 38AA3F5C  addi r5, r10, 0x3f5c
	ctx.r[5].s64 = ctx.r[10].s64 + 16220;
	// 82604390: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604394: 390B8370  addi r8, r11, -0x7c90
	ctx.r[8].s64 = ctx.r[11].s64 + -31888;
	// 82604398: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260439C: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826043A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826043A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826043A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826043AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826043B0: 386A3F8C  addi r3, r10, 0x3f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 16268;
	// 826043B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826043B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826043BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826043C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826043C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826043C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826043CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826043D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826043D4: 4BE62A4D  bl 0x82466e20
	ctx.lr = 0x826043D8;
	sub_82466E20(ctx, base);
	// 826043D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826043DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826043E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826043E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826043E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826043E8 size=100
    let mut pc: u32 = 0x826043E8;
    'dispatch: loop {
        match pc {
            0x826043E8 => {
    //   block [0x826043E8..0x8260444C)
	// 826043E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826043EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826043F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826043F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826043F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826043FC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604408: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 8260440C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260441C: 386A3FBC  addi r3, r10, 0x3fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 16316;
	// 82604420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604424: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604428: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260442C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604430: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82604434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604438: 4BE629E9  bl 0x82466e20
	ctx.lr = 0x8260443C;
	sub_82466E20(ctx, base);
	// 8260443C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604450 size=108
    let mut pc: u32 = 0x82604450;
    'dispatch: loop {
        match pc {
            0x82604450 => {
    //   block [0x82604450..0x826044BC)
	// 82604450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260445C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604464: 38EB83E8  addi r7, r11, -0x7c18
	ctx.r[7].s64 = ctx.r[11].s64 + -31768;
	// 82604468: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260446C: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 82604470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604474: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604478: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260447C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604480: 386A3FEC  addi r3, r10, 0x3fec
	ctx.r[3].s64 = ctx.r[10].s64 + 16364;
	// 82604484: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260448C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260449C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826044A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826044A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826044A8: 4BE62979  bl 0x82466e20
	ctx.lr = 0x826044AC;
	sub_82466E20(ctx, base);
	// 826044AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826044B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826044B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826044B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826044C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826044C0 size=112
    let mut pc: u32 = 0x826044C0;
    'dispatch: loop {
        match pc {
            0x826044C0 => {
    //   block [0x826044C0..0x82604530)
	// 826044C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826044C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826044C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826044CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826044D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826044D4: 38AA3FBC  addi r5, r10, 0x3fbc
	ctx.r[5].s64 = ctx.r[10].s64 + 16316;
	// 826044D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826044DC: 390B8430  addi r8, r11, -0x7bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -31696;
	// 826044E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826044E4: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826044E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826044EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826044F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826044F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826044F8: 386A401C  addi r3, r10, 0x401c
	ctx.r[3].s64 = ctx.r[10].s64 + 16412;
	// 826044FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260450C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260451C: 4BE62905  bl 0x82466e20
	ctx.lr = 0x82604520;
	sub_82466E20(ctx, base);
	// 82604520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260452C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604530 size=100
    let mut pc: u32 = 0x82604530;
    'dispatch: loop {
        match pc {
            0x82604530 => {
    //   block [0x82604530..0x82604594)
	// 82604530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260453C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604544: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260454C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604550: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 82604554: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260455C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604564: 386A404C  addi r3, r10, 0x404c
	ctx.r[3].s64 = ctx.r[10].s64 + 16460;
	// 82604568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260456C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604570: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604578: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260457C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604580: 4BE628A1  bl 0x82466e20
	ctx.lr = 0x82604584;
	sub_82466E20(ctx, base);
	// 82604584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260458C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604598 size=100
    let mut pc: u32 = 0x82604598;
    'dispatch: loop {
        match pc {
            0x82604598 => {
    //   block [0x82604598..0x826045FC)
	// 82604598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826045A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826045A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826045A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826045AC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826045B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826045B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826045B8: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826045BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826045C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826045C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826045C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826045CC: 386A407C  addi r3, r10, 0x407c
	ctx.r[3].s64 = ctx.r[10].s64 + 16508;
	// 826045D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826045D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826045D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826045DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826045E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826045E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826045E8: 4BE62839  bl 0x82466e20
	ctx.lr = 0x826045EC;
	sub_82466E20(ctx, base);
	// 826045EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826045F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826045F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826045F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604600 size=112
    let mut pc: u32 = 0x82604600;
    'dispatch: loop {
        match pc {
            0x82604600 => {
    //   block [0x82604600..0x82604670)
	// 82604600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260460C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604610: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604614: 38AA404C  addi r5, r10, 0x404c
	ctx.r[5].s64 = ctx.r[10].s64 + 16460;
	// 82604618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260461C: 390B8460  addi r8, r11, -0x7ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -31648;
	// 82604620: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82604624: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 82604628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260462C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604638: 386A40AC  addi r3, r10, 0x40ac
	ctx.r[3].s64 = ctx.r[10].s64 + 16556;
	// 8260463C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260464C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260465C: 4BE627C5  bl 0x82466e20
	ctx.lr = 0x82604660;
	sub_82466E20(ctx, base);
	// 82604660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260466C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604670 size=112
    let mut pc: u32 = 0x82604670;
    'dispatch: loop {
        match pc {
            0x82604670 => {
    //   block [0x82604670..0x826046E0)
	// 82604670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260467C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604680: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604684: 38AA407C  addi r5, r10, 0x407c
	ctx.r[5].s64 = ctx.r[10].s64 + 16508;
	// 82604688: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260468C: 390B84C0  addi r8, r11, -0x7b40
	ctx.r[8].s64 = ctx.r[11].s64 + -31552;
	// 82604690: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82604694: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 82604698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260469C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826046A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826046A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826046A8: 386A40DC  addi r3, r10, 0x40dc
	ctx.r[3].s64 = ctx.r[10].s64 + 16604;
	// 826046AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826046B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826046B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826046B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826046BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826046C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826046C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826046C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826046CC: 4BE62755  bl 0x82466e20
	ctx.lr = 0x826046D0;
	sub_82466E20(ctx, base);
	// 826046D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826046D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826046D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826046DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826046E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826046E0 size=100
    let mut pc: u32 = 0x826046E0;
    'dispatch: loop {
        match pc {
            0x826046E0 => {
    //   block [0x826046E0..0x82604744)
	// 826046E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826046E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826046E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826046EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826046F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826046F4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826046F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826046FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604700: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 82604704: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260470C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604714: 386A410C  addi r3, r10, 0x410c
	ctx.r[3].s64 = ctx.r[10].s64 + 16652;
	// 82604718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260471C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604720: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604728: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260472C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604730: 4BE626F1  bl 0x82466e20
	ctx.lr = 0x82604734;
	sub_82466E20(ctx, base);
	// 82604734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260473C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604748 size=112
    let mut pc: u32 = 0x82604748;
    'dispatch: loop {
        match pc {
            0x82604748 => {
    //   block [0x82604748..0x826047B8)
	// 82604748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260474C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604754: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604758: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260475C: 38AA410C  addi r5, r10, 0x410c
	ctx.r[5].s64 = ctx.r[10].s64 + 16652;
	// 82604760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604764: 390B8520  addi r8, r11, -0x7ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -31456;
	// 82604768: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8260476C: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 82604770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604774: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260477C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604780: 386A413C  addi r3, r10, 0x413c
	ctx.r[3].s64 = ctx.r[10].s64 + 16700;
	// 82604784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260478C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260479C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826047A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826047A4: 4BE6267D  bl 0x82466e20
	ctx.lr = 0x826047A8;
	sub_82466E20(ctx, base);
	// 826047A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826047AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826047B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826047B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826047B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826047B8 size=108
    let mut pc: u32 = 0x826047B8;
    'dispatch: loop {
        match pc {
            0x826047B8 => {
    //   block [0x826047B8..0x82604824)
	// 826047B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826047BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826047C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826047C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826047C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826047CC: 38EB8610  addi r7, r11, -0x79f0
	ctx.r[7].s64 = ctx.r[11].s64 + -31216;
	// 826047D0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826047D4: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826047D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826047DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826047E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826047E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826047E8: 386A416C  addi r3, r10, 0x416c
	ctx.r[3].s64 = ctx.r[10].s64 + 16748;
	// 826047EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826047F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826047F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826047F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826047FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260480C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604810: 4BE62611  bl 0x82466e20
	ctx.lr = 0x82604814;
	sub_82466E20(ctx, base);
	// 82604814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260481C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604828 size=108
    let mut pc: u32 = 0x82604828;
    'dispatch: loop {
        match pc {
            0x82604828 => {
    //   block [0x82604828..0x82604894)
	// 82604828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260482C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604834: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604838: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260483C: 38EB8700  addi r7, r11, -0x7900
	ctx.r[7].s64 = ctx.r[11].s64 + -30976;
	// 82604840: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82604844: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 82604848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260484C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604858: 386A419C  addi r3, r10, 0x419c
	ctx.r[3].s64 = ctx.r[10].s64 + 16796;
	// 8260485C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260486C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260487C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604880: 4BE625A1  bl 0x82466e20
	ctx.lr = 0x82604884;
	sub_82466E20(ctx, base);
	// 82604884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260488C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604898 size=108
    let mut pc: u32 = 0x82604898;
    'dispatch: loop {
        match pc {
            0x82604898 => {
    //   block [0x82604898..0x82604904)
	// 82604898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260489C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826048A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826048A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826048A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826048AC: 38EB8748  addi r7, r11, -0x78b8
	ctx.r[7].s64 = ctx.r[11].s64 + -30904;
	// 826048B0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826048B4: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826048B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826048BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826048C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826048C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826048C8: 386A41CC  addi r3, r10, 0x41cc
	ctx.r[3].s64 = ctx.r[10].s64 + 16844;
	// 826048CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826048D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826048D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826048D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826048DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826048E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826048E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826048E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826048EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826048F0: 4BE62531  bl 0x82466e20
	ctx.lr = 0x826048F4;
	sub_82466E20(ctx, base);
	// 826048F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826048F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826048FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604908 size=108
    let mut pc: u32 = 0x82604908;
    'dispatch: loop {
        match pc {
            0x82604908 => {
    //   block [0x82604908..0x82604974)
	// 82604908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260490C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604914: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604918: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260491C: 38EB8820  addi r7, r11, -0x77e0
	ctx.r[7].s64 = ctx.r[11].s64 + -30688;
	// 82604920: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82604924: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 82604928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260492C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604938: 386A41FC  addi r3, r10, 0x41fc
	ctx.r[3].s64 = ctx.r[10].s64 + 16892;
	// 8260493C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260494C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260495C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604960: 4BE624C1  bl 0x82466e20
	ctx.lr = 0x82604964;
	sub_82466E20(ctx, base);
	// 82604964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260496C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604978 size=100
    let mut pc: u32 = 0x82604978;
    'dispatch: loop {
        match pc {
            0x82604978 => {
    //   block [0x82604978..0x826049DC)
	// 82604978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260497C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260498C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604998: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 8260499C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826049A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826049A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826049A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826049AC: 386A422C  addi r3, r10, 0x422c
	ctx.r[3].s64 = ctx.r[10].s64 + 16940;
	// 826049B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826049B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826049B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826049BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826049C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826049C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826049C8: 4BE62459  bl 0x82466e20
	ctx.lr = 0x826049CC;
	sub_82466E20(ctx, base);
	// 826049CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826049D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826049D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826049D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826049E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826049E0 size=112
    let mut pc: u32 = 0x826049E0;
    'dispatch: loop {
        match pc {
            0x826049E0 => {
    //   block [0x826049E0..0x82604A50)
	// 826049E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826049E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826049E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826049EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826049F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826049F4: 38AA422C  addi r5, r10, 0x422c
	ctx.r[5].s64 = ctx.r[10].s64 + 16940;
	// 826049F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826049FC: 390B8838  addi r8, r11, -0x77c8
	ctx.r[8].s64 = ctx.r[11].s64 + -30664;
	// 82604A00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604A04: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 82604A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604A18: 386A425C  addi r3, r10, 0x425c
	ctx.r[3].s64 = ctx.r[10].s64 + 16988;
	// 82604A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604A3C: 4BE623E5  bl 0x82466e20
	ctx.lr = 0x82604A40;
	sub_82466E20(ctx, base);
	// 82604A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604A50 size=108
    let mut pc: u32 = 0x82604A50;
    'dispatch: loop {
        match pc {
            0x82604A50 => {
    //   block [0x82604A50..0x82604ABC)
	// 82604A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604A5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604A64: 38EB8880  addi r7, r11, -0x7780
	ctx.r[7].s64 = ctx.r[11].s64 + -30592;
	// 82604A68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82604A6C: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 82604A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604A80: 386A428C  addi r3, r10, 0x428c
	ctx.r[3].s64 = ctx.r[10].s64 + 17036;
	// 82604A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604AA8: 4BE62379  bl 0x82466e20
	ctx.lr = 0x82604AAC;
	sub_82466E20(ctx, base);
	// 82604AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604AC0 size=112
    let mut pc: u32 = 0x82604AC0;
    'dispatch: loop {
        match pc {
            0x82604AC0 => {
    //   block [0x82604AC0..0x82604B30)
	// 82604AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604AD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604AD4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604AD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604ADC: 390B88C8  addi r8, r11, -0x7738
	ctx.r[8].s64 = ctx.r[11].s64 + -30520;
	// 82604AE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604AE4: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 82604AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604AEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604AF8: 386A42BC  addi r3, r10, 0x42bc
	ctx.r[3].s64 = ctx.r[10].s64 + 17084;
	// 82604AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604B1C: 4BE62305  bl 0x82466e20
	ctx.lr = 0x82604B20;
	sub_82466E20(ctx, base);
	// 82604B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604B30 size=108
    let mut pc: u32 = 0x82604B30;
    'dispatch: loop {
        match pc {
            0x82604B30 => {
    //   block [0x82604B30..0x82604B9C)
	// 82604B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604B3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604B40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604B44: 38EB88E0  addi r7, r11, -0x7720
	ctx.r[7].s64 = ctx.r[11].s64 + -30496;
	// 82604B48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82604B4C: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 82604B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604B60: 386A42EC  addi r3, r10, 0x42ec
	ctx.r[3].s64 = ctx.r[10].s64 + 17132;
	// 82604B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604B88: 4BE62299  bl 0x82466e20
	ctx.lr = 0x82604B8C;
	sub_82466E20(ctx, base);
	// 82604B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604BA0 size=112
    let mut pc: u32 = 0x82604BA0;
    'dispatch: loop {
        match pc {
            0x82604BA0 => {
    //   block [0x82604BA0..0x82604C10)
	// 82604BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604BB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604BB4: 38AA42BC  addi r5, r10, 0x42bc
	ctx.r[5].s64 = ctx.r[10].s64 + 17084;
	// 82604BB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604BBC: 390B8928  addi r8, r11, -0x76d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30424;
	// 82604BC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604BC4: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 82604BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604BCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604BD8: 386A431C  addi r3, r10, 0x431c
	ctx.r[3].s64 = ctx.r[10].s64 + 17180;
	// 82604BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604BFC: 4BE62225  bl 0x82466e20
	ctx.lr = 0x82604C00;
	sub_82466E20(ctx, base);
	// 82604C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604C10 size=100
    let mut pc: u32 = 0x82604C10;
    'dispatch: loop {
        match pc {
            0x82604C10 => {
    //   block [0x82604C10..0x82604C74)
	// 82604C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604C1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604C24: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604C28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604C30: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 82604C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604C44: 386A434C  addi r3, r10, 0x434c
	ctx.r[3].s64 = ctx.r[10].s64 + 17228;
	// 82604C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604C4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82604C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604C60: 4BE621C1  bl 0x82466e20
	ctx.lr = 0x82604C64;
	sub_82466E20(ctx, base);
	// 82604C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604C78 size=112
    let mut pc: u32 = 0x82604C78;
    'dispatch: loop {
        match pc {
            0x82604C78 => {
    //   block [0x82604C78..0x82604CE8)
	// 82604C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604C88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604C8C: 38AA434C  addi r5, r10, 0x434c
	ctx.r[5].s64 = ctx.r[10].s64 + 17228;
	// 82604C90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604C94: 390B8940  addi r8, r11, -0x76c0
	ctx.r[8].s64 = ctx.r[11].s64 + -30400;
	// 82604C98: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82604C9C: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 82604CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604CA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604CB0: 386A437C  addi r3, r10, 0x437c
	ctx.r[3].s64 = ctx.r[10].s64 + 17276;
	// 82604CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604CD4: 4BE6214D  bl 0x82466e20
	ctx.lr = 0x82604CD8;
	sub_82466E20(ctx, base);
	// 82604CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604CE8 size=108
    let mut pc: u32 = 0x82604CE8;
    'dispatch: loop {
        match pc {
            0x82604CE8 => {
    //   block [0x82604CE8..0x82604D54)
	// 82604CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604CF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604CF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604CFC: 38EB89E8  addi r7, r11, -0x7618
	ctx.r[7].s64 = ctx.r[11].s64 + -30232;
	// 82604D00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82604D04: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 82604D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604D0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604D10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604D18: 386A43AC  addi r3, r10, 0x43ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17324;
	// 82604D1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604D40: 4BE620E1  bl 0x82466e20
	ctx.lr = 0x82604D44;
	sub_82466E20(ctx, base);
	// 82604D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604D58 size=112
    let mut pc: u32 = 0x82604D58;
    'dispatch: loop {
        match pc {
            0x82604D58 => {
    //   block [0x82604D58..0x82604DC8)
	// 82604D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604D64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604D68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604D6C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604D70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604D74: 390B8A18  addi r8, r11, -0x75e8
	ctx.r[8].s64 = ctx.r[11].s64 + -30184;
	// 82604D78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604D7C: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 82604D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604D84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604D90: 386A43DC  addi r3, r10, 0x43dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17372;
	// 82604D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604DB4: 4BE6206D  bl 0x82466e20
	ctx.lr = 0x82604DB8;
	sub_82466E20(ctx, base);
	// 82604DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604DC8 size=112
    let mut pc: u32 = 0x82604DC8;
    'dispatch: loop {
        match pc {
            0x82604DC8 => {
    //   block [0x82604DC8..0x82604E38)
	// 82604DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604DD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604DD8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604DDC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82604DE4: 390B8A60  addi r8, r11, -0x75a0
	ctx.r[8].s64 = ctx.r[11].s64 + -30112;
	// 82604DE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604DEC: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 82604DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604DF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604E00: 386A440C  addi r3, r10, 0x440c
	ctx.r[3].s64 = ctx.r[10].s64 + 17420;
	// 82604E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604E24: 4BE61FFD  bl 0x82466e20
	ctx.lr = 0x82604E28;
	sub_82466E20(ctx, base);
	// 82604E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604E38 size=100
    let mut pc: u32 = 0x82604E38;
    'dispatch: loop {
        match pc {
            0x82604E38 => {
    //   block [0x82604E38..0x82604E9C)
	// 82604E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604E4C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604E50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604E58: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 82604E5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604E6C: 386A443C  addi r3, r10, 0x443c
	ctx.r[3].s64 = ctx.r[10].s64 + 17468;
	// 82604E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82604E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604E88: 4BE61F99  bl 0x82466e20
	ctx.lr = 0x82604E8C;
	sub_82466E20(ctx, base);
	// 82604E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604EA0 size=112
    let mut pc: u32 = 0x82604EA0;
    'dispatch: loop {
        match pc {
            0x82604EA0 => {
    //   block [0x82604EA0..0x82604F10)
	// 82604EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604EAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604EB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604EB4: 38AA443C  addi r5, r10, 0x443c
	ctx.r[5].s64 = ctx.r[10].s64 + 17468;
	// 82604EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604EBC: 390B8AA8  addi r8, r11, -0x7558
	ctx.r[8].s64 = ctx.r[11].s64 + -30040;
	// 82604EC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604EC4: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 82604EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604ED8: 386A446C  addi r3, r10, 0x446c
	ctx.r[3].s64 = ctx.r[10].s64 + 17516;
	// 82604EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604EFC: 4BE61F25  bl 0x82466e20
	ctx.lr = 0x82604F00;
	sub_82466E20(ctx, base);
	// 82604F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604F10 size=112
    let mut pc: u32 = 0x82604F10;
    'dispatch: loop {
        match pc {
            0x82604F10 => {
    //   block [0x82604F10..0x82604F80)
	// 82604F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604F20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604F24: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604F28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604F2C: 390B8AF0  addi r8, r11, -0x7510
	ctx.r[8].s64 = ctx.r[11].s64 + -29968;
	// 82604F30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604F34: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 82604F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604F3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604F48: 386A449C  addi r3, r10, 0x449c
	ctx.r[3].s64 = ctx.r[10].s64 + 17564;
	// 82604F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604F6C: 4BE61EB5  bl 0x82466e20
	ctx.lr = 0x82604F70;
	sub_82466E20(ctx, base);
	// 82604F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604F80 size=112
    let mut pc: u32 = 0x82604F80;
    'dispatch: loop {
        match pc {
            0x82604F80 => {
    //   block [0x82604F80..0x82604FF0)
	// 82604F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604F8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604F90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604F94: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604F98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604F9C: 390B8B08  addi r8, r11, -0x74f8
	ctx.r[8].s64 = ctx.r[11].s64 + -29944;
	// 82604FA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604FA4: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 82604FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604FAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604FB8: 386A44CC  addi r3, r10, 0x44cc
	ctx.r[3].s64 = ctx.r[10].s64 + 17612;
	// 82604FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604FCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82604FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604FDC: 4BE61E45  bl 0x82466e20
	ctx.lr = 0x82604FE0;
	sub_82466E20(ctx, base);
	// 82604FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604FF0 size=112
    let mut pc: u32 = 0x82604FF0;
    'dispatch: loop {
        match pc {
            0x82604FF0 => {
    //   block [0x82604FF0..0x82605060)
	// 82604FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604FFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605000: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605004: 38AA449C  addi r5, r10, 0x449c
	ctx.r[5].s64 = ctx.r[10].s64 + 17564;
	// 82605008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260500C: 390B8B20  addi r8, r11, -0x74e0
	ctx.r[8].s64 = ctx.r[11].s64 + -29920;
	// 82605010: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82605014: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 82605018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260501C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605028: 386A44FC  addi r3, r10, 0x44fc
	ctx.r[3].s64 = ctx.r[10].s64 + 17660;
	// 8260502C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260503C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260504C: 4BE61DD5  bl 0x82466e20
	ctx.lr = 0x82605050;
	sub_82466E20(ctx, base);
	// 82605050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260505C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605060 size=72
    let mut pc: u32 = 0x82605060;
    'dispatch: loop {
        match pc {
            0x82605060 => {
    //   block [0x82605060..0x826050A8)
	// 82605060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260506C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82605070: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82605074: 38CB7BE0  addi r6, r11, 0x7be0
	ctx.r[6].s64 = ctx.r[11].s64 + 31712;
	// 82605078: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260507C: 388B9E28  addi r4, r11, -0x61d8
	ctx.r[4].s64 = ctx.r[11].s64 + -25048;
	// 82605080: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82605084: 386B452C  addi r3, r11, 0x452c
	ctx.r[3].s64 = ctx.r[11].s64 + 17708;
	// 82605088: 4BE76A01  bl 0x8247ba88
	ctx.lr = 0x8260508C;
	sub_8247BA88(ctx, base);
	// 8260508C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82605090: 386BCD08  addi r3, r11, -0x32f8
	ctx.r[3].s64 = ctx.r[11].s64 + -13048;
	// 82605094: 4BF2DAA5  bl 0x82532b38
	ctx.lr = 0x82605098;
	sub_82532B38(ctx, base);
	// 82605098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8260509C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826050A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826050A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826050A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826050A8 size=108
    let mut pc: u32 = 0x826050A8;
    'dispatch: loop {
        match pc {
            0x826050A8 => {
    //   block [0x826050A8..0x82605114)
	// 826050A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826050AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826050B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826050B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826050B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826050BC: 38EBBAA8  addi r7, r11, -0x4558
	ctx.r[7].s64 = ctx.r[11].s64 + -17752;
	// 826050C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826050C4: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826050C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826050CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826050D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826050D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826050D8: 386A4544  addi r3, r10, 0x4544
	ctx.r[3].s64 = ctx.r[10].s64 + 17732;
	// 826050DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826050E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826050E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826050E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826050EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826050F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826050F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826050F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826050FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605100: 4BE61D21  bl 0x82466e20
	ctx.lr = 0x82605104;
	sub_82466E20(ctx, base);
	// 82605104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260510C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605118 size=24
    let mut pc: u32 = 0x82605118;
    'dispatch: loop {
        match pc {
            0x82605118 => {
    //   block [0x82605118..0x82605130)
	// 82605118: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260511C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605120: 394A2C78  addi r10, r10, 0x2c78
	ctx.r[10].s64 = ctx.r[10].s64 + 11384;
	// 82605124: 816BBB20  lwz r11, -0x44e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17632 as u32) ) } as u64;
	// 82605128: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8260512C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605130 size=112
    let mut pc: u32 = 0x82605130;
    'dispatch: loop {
        match pc {
            0x82605130 => {
    //   block [0x82605130..0x826051A0)
	// 82605130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260513C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605140: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605144: 392ACF9C  addi r9, r10, -0x3064
	ctx.r[9].s64 = ctx.r[10].s64 + -12388;
	// 82605148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260514C: 390B2C78  addi r8, r11, 0x2c78
	ctx.r[8].s64 = ctx.r[11].s64 + 11384;
	// 82605150: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82605154: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82605158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260515C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605168: 386A4574  addi r3, r10, 0x4574
	ctx.r[3].s64 = ctx.r[10].s64 + 17780;
	// 8260516C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605170: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260517C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260518C: 4BE61C95  bl 0x82466e20
	ctx.lr = 0x82605190;
	sub_82466E20(ctx, base);
	// 82605190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260519C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826051A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826051A0 size=108
    let mut pc: u32 = 0x826051A0;
    'dispatch: loop {
        match pc {
            0x826051A0 => {
    //   block [0x826051A0..0x8260520C)
	// 826051A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826051A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826051A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826051AC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826051B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826051B4: 38EBBB24  addi r7, r11, -0x44dc
	ctx.r[7].s64 = ctx.r[11].s64 + -17628;
	// 826051B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826051BC: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826051C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826051C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826051C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826051CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826051D0: 386A45A4  addi r3, r10, 0x45a4
	ctx.r[3].s64 = ctx.r[10].s64 + 17828;
	// 826051D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826051D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826051DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826051E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826051E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826051E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826051EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826051F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826051F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826051F8: 4BE61C29  bl 0x82466e20
	ctx.lr = 0x826051FC;
	sub_82466E20(ctx, base);
	// 826051FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605210 size=108
    let mut pc: u32 = 0x82605210;
    'dispatch: loop {
        match pc {
            0x82605210 => {
    //   block [0x82605210..0x8260527C)
	// 82605210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260521C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605224: 38EBBB54  addi r7, r11, -0x44ac
	ctx.r[7].s64 = ctx.r[11].s64 + -17580;
	// 82605228: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260522C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82605230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260523C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605240: 386A45D4  addi r3, r10, 0x45d4
	ctx.r[3].s64 = ctx.r[10].s64 + 17876;
	// 82605244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260524C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260525C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605268: 4BE61BB9  bl 0x82466e20
	ctx.lr = 0x8260526C;
	sub_82466E20(ctx, base);
	// 8260526C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605280 size=24
    let mut pc: u32 = 0x82605280;
    'dispatch: loop {
        match pc {
            0x82605280 => {
    //   block [0x82605280..0x82605298)
	// 82605280: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605284: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605288: 394A2CC0  addi r10, r10, 0x2cc0
	ctx.r[10].s64 = ctx.r[10].s64 + 11456;
	// 8260528C: 816BBB84  lwz r11, -0x447c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17532 as u32) ) } as u64;
	// 82605290: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82605294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605298 size=116
    let mut pc: u32 = 0x82605298;
    'dispatch: loop {
        match pc {
            0x82605298 => {
    //   block [0x82605298..0x8260530C)
	// 82605298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260529C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826052A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826052A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826052A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826052AC: 390B2CC0  addi r8, r11, 0x2cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 11456;
	// 826052B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826052B4: 392ACFD0  addi r9, r10, -0x3030
	ctx.r[9].s64 = ctx.r[10].s64 + -12336;
	// 826052B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826052BC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826052C0: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826052C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826052C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826052CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826052D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826052D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826052D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826052DC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826052E0: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826052E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826052E8: 386B4604  addi r3, r11, 0x4604
	ctx.r[3].s64 = ctx.r[11].s64 + 17924;
	// 826052EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826052F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826052F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826052F8: 4BE61B29  bl 0x82466e20
	ctx.lr = 0x826052FC;
	sub_82466E20(ctx, base);
	// 826052FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605310 size=108
    let mut pc: u32 = 0x82605310;
    'dispatch: loop {
        match pc {
            0x82605310 => {
    //   block [0x82605310..0x8260537C)
	// 82605310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260531C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605324: 38EBBB88  addi r7, r11, -0x4478
	ctx.r[7].s64 = ctx.r[11].s64 + -17528;
	// 82605328: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8260532C: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82605330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260533C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605340: 386A4634  addi r3, r10, 0x4634
	ctx.r[3].s64 = ctx.r[10].s64 + 17972;
	// 82605344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260534C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260535C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605368: 4BE61AB9  bl 0x82466e20
	ctx.lr = 0x8260536C;
	sub_82466E20(ctx, base);
	// 8260536C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605380 size=112
    let mut pc: u32 = 0x82605380;
    'dispatch: loop {
        match pc {
            0x82605380 => {
    //   block [0x82605380..0x826053F0)
	// 82605380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260538C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605390: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605394: 38AA4604  addi r5, r10, 0x4604
	ctx.r[5].s64 = ctx.r[10].s64 + 17924;
	// 82605398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260539C: 390BBC18  addi r8, r11, -0x43e8
	ctx.r[8].s64 = ctx.r[11].s64 + -17384;
	// 826053A0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826053A4: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826053A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826053AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826053B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826053B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826053B8: 386A4664  addi r3, r10, 0x4664
	ctx.r[3].s64 = ctx.r[10].s64 + 18020;
	// 826053BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826053C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826053C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826053C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826053CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826053D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826053D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826053D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826053DC: 4BE61A45  bl 0x82466e20
	ctx.lr = 0x826053E0;
	sub_82466E20(ctx, base);
	// 826053E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826053E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826053E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826053EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826053F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826053F0 size=112
    let mut pc: u32 = 0x826053F0;
    'dispatch: loop {
        match pc {
            0x826053F0 => {
    //   block [0x826053F0..0x82605460)
	// 826053F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826053F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826053F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826053FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605400: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605404: 38AA4604  addi r5, r10, 0x4604
	ctx.r[5].s64 = ctx.r[10].s64 + 17924;
	// 82605408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260540C: 390BBD38  addi r8, r11, -0x42c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17096;
	// 82605410: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82605414: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82605418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260541C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605428: 386A4694  addi r3, r10, 0x4694
	ctx.r[3].s64 = ctx.r[10].s64 + 18068;
	// 8260542C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260543C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260544C: 4BE619D5  bl 0x82466e20
	ctx.lr = 0x82605450;
	sub_82466E20(ctx, base);
	// 82605450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260545C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605460 size=108
    let mut pc: u32 = 0x82605460;
    'dispatch: loop {
        match pc {
            0x82605460 => {
    //   block [0x82605460..0x826054CC)
	// 82605460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260546C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605474: 38EBBD50  addi r7, r11, -0x42b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17072;
	// 82605478: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8260547C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82605480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260548C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605490: 386A46C4  addi r3, r10, 0x46c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18116;
	// 82605494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260549C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826054A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826054A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826054A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826054AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826054B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826054B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826054B8: 4BE61969  bl 0x82466e20
	ctx.lr = 0x826054BC;
	sub_82466E20(ctx, base);
	// 826054BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826054C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826054C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826054C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826054D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826054D0 size=112
    let mut pc: u32 = 0x826054D0;
    'dispatch: loop {
        match pc {
            0x826054D0 => {
    //   block [0x826054D0..0x82605540)
	// 826054D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826054D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826054D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826054DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826054E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826054E4: 38AA4604  addi r5, r10, 0x4604
	ctx.r[5].s64 = ctx.r[10].s64 + 17924;
	// 826054E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826054EC: 390BBDE0  addi r8, r11, -0x4220
	ctx.r[8].s64 = ctx.r[11].s64 + -16928;
	// 826054F0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826054F4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826054F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826054FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605508: 386A46F4  addi r3, r10, 0x46f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18164;
	// 8260550C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260551C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260552C: 4BE618F5  bl 0x82466e20
	ctx.lr = 0x82605530;
	sub_82466E20(ctx, base);
	// 82605530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260553C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605540 size=108
    let mut pc: u32 = 0x82605540;
    'dispatch: loop {
        match pc {
            0x82605540 => {
    //   block [0x82605540..0x826055AC)
	// 82605540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260554C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605554: 38EBBED0  addi r7, r11, -0x4130
	ctx.r[7].s64 = ctx.r[11].s64 + -16688;
	// 82605558: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260555C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82605560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260556C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605570: 386A4724  addi r3, r10, 0x4724
	ctx.r[3].s64 = ctx.r[10].s64 + 18212;
	// 82605574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260557C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260558C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605598: 4BE61889  bl 0x82466e20
	ctx.lr = 0x8260559C;
	sub_82466E20(ctx, base);
	// 8260559C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826055A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826055A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826055A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826055B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826055B0 size=108
    let mut pc: u32 = 0x826055B0;
    'dispatch: loop {
        match pc {
            0x826055B0 => {
    //   block [0x826055B0..0x8260561C)
	// 826055B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826055B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826055B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826055BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826055C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826055C4: 38EBBEE8  addi r7, r11, -0x4118
	ctx.r[7].s64 = ctx.r[11].s64 + -16664;
	// 826055C8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826055CC: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826055D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826055D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826055D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826055DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826055E0: 386A4754  addi r3, r10, 0x4754
	ctx.r[3].s64 = ctx.r[10].s64 + 18260;
	// 826055E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826055E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826055EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826055F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826055F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826055F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826055FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605608: 4BE61819  bl 0x82466e20
	ctx.lr = 0x8260560C;
	sub_82466E20(ctx, base);
	// 8260560C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605620 size=116
    let mut pc: u32 = 0x82605620;
    'dispatch: loop {
        match pc {
            0x82605620 => {
    //   block [0x82605620..0x82605694)
	// 82605620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260562C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605634: 390BBF4C  addi r8, r11, -0x40b4
	ctx.r[8].s64 = ctx.r[11].s64 + -16564;
	// 82605638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260563C: 392ACFFC  addi r9, r10, -0x3004
	ctx.r[9].s64 = ctx.r[10].s64 + -12292;
	// 82605640: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605644: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82605648: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260564C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260565C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605664: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82605668: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8260566C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605670: 386B4784  addi r3, r11, 0x4784
	ctx.r[3].s64 = ctx.r[11].s64 + 18308;
	// 82605674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260567C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605680: 4BE617A1  bl 0x82466e20
	ctx.lr = 0x82605684;
	sub_82466E20(ctx, base);
	// 82605684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260568C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605698 size=108
    let mut pc: u32 = 0x82605698;
    'dispatch: loop {
        match pc {
            0x82605698 => {
    //   block [0x82605698..0x82605704)
	// 82605698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260569C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826056A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826056A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826056A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826056AC: 38EBBF68  addi r7, r11, -0x4098
	ctx.r[7].s64 = ctx.r[11].s64 + -16536;
	// 826056B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826056B4: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826056B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826056BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826056C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826056C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826056C8: 386A47B4  addi r3, r10, 0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + 18356;
	// 826056CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826056D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826056D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826056D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826056DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826056E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826056E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826056E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826056EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826056F0: 4BE61731  bl 0x82466e20
	ctx.lr = 0x826056F4;
	sub_82466E20(ctx, base);
	// 826056F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826056F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826056FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605708 size=108
    let mut pc: u32 = 0x82605708;
    'dispatch: loop {
        match pc {
            0x82605708 => {
    //   block [0x82605708..0x82605774)
	// 82605708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260570C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605714: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260571C: 38EBBFB0  addi r7, r11, -0x4050
	ctx.r[7].s64 = ctx.r[11].s64 + -16464;
	// 82605720: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82605724: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 82605728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260572C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82605734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605738: 386A47E4  addi r3, r10, 0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + 18404;
	// 8260573C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260574C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260575C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605760: 4BE616C1  bl 0x82466e20
	ctx.lr = 0x82605764;
	sub_82466E20(ctx, base);
	// 82605764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605778 size=108
    let mut pc: u32 = 0x82605778;
    'dispatch: loop {
        match pc {
            0x82605778 => {
    //   block [0x82605778..0x826057E4)
	// 82605778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605784: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260578C: 38EBC040  addi r7, r11, -0x3fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -16320;
	// 82605790: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82605794: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82605798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260579C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826057A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826057A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826057A8: 386A4814  addi r3, r10, 0x4814
	ctx.r[3].s64 = ctx.r[10].s64 + 18452;
	// 826057AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826057B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826057B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826057B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826057BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826057C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826057C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826057C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826057CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826057D0: 4BE61651  bl 0x82466e20
	ctx.lr = 0x826057D4;
	sub_82466E20(ctx, base);
	// 826057D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826057D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826057DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826057E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826057E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826057E8 size=100
    let mut pc: u32 = 0x826057E8;
    'dispatch: loop {
        match pc {
            0x826057E8 => {
    //   block [0x826057E8..0x8260584C)
	// 826057E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826057EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826057F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826057F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826057F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826057FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605808: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8260580C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260581C: 386A4844  addi r3, r10, 0x4844
	ctx.r[3].s64 = ctx.r[10].s64 + 18500;
	// 82605820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605828: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260582C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605830: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605838: 4BE615E9  bl 0x82466e20
	ctx.lr = 0x8260583C;
	sub_82466E20(ctx, base);
	// 8260583C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605850 size=112
    let mut pc: u32 = 0x82605850;
    'dispatch: loop {
        match pc {
            0x82605850 => {
    //   block [0x82605850..0x826058C0)
	// 82605850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260585C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605860: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605864: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 82605868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260586C: 390BC0D0  addi r8, r11, -0x3f30
	ctx.r[8].s64 = ctx.r[11].s64 + -16176;
	// 82605870: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82605874: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82605878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260587C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605888: 386A4874  addi r3, r10, 0x4874
	ctx.r[3].s64 = ctx.r[10].s64 + 18548;
	// 8260588C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260589C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826058A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826058A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826058A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826058AC: 4BE61575  bl 0x82466e20
	ctx.lr = 0x826058B0;
	sub_82466E20(ctx, base);
	// 826058B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826058B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826058B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826058BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826058C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826058C0 size=108
    let mut pc: u32 = 0x826058C0;
    'dispatch: loop {
        match pc {
            0x826058C0 => {
    //   block [0x826058C0..0x8260592C)
	// 826058C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826058C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826058C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826058CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826058D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826058D4: 38EBC130  addi r7, r11, -0x3ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -16080;
	// 826058D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826058DC: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826058E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826058E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826058E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826058EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826058F0: 386A48A4  addi r3, r10, 0x48a4
	ctx.r[3].s64 = ctx.r[10].s64 + 18596;
	// 826058F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826058F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826058FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260590C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605918: 4BE61509  bl 0x82466e20
	ctx.lr = 0x8260591C;
	sub_82466E20(ctx, base);
	// 8260591C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605930 size=108
    let mut pc: u32 = 0x82605930;
    'dispatch: loop {
        match pc {
            0x82605930 => {
    //   block [0x82605930..0x8260599C)
	// 82605930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260593C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605944: 38EBC160  addi r7, r11, -0x3ea0
	ctx.r[7].s64 = ctx.r[11].s64 + -16032;
	// 82605948: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260594C: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82605950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260595C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605960: 386A48D4  addi r3, r10, 0x48d4
	ctx.r[3].s64 = ctx.r[10].s64 + 18644;
	// 82605964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260596C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260597C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605988: 4BE61499  bl 0x82466e20
	ctx.lr = 0x8260598C;
	sub_82466E20(ctx, base);
	// 8260598C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826059A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826059A0 size=108
    let mut pc: u32 = 0x826059A0;
    'dispatch: loop {
        match pc {
            0x826059A0 => {
    //   block [0x826059A0..0x82605A0C)
	// 826059A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826059A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826059A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826059AC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826059B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826059B4: 38EBC1C0  addi r7, r11, -0x3e40
	ctx.r[7].s64 = ctx.r[11].s64 + -15936;
	// 826059B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826059BC: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826059C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826059C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826059C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826059CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826059D0: 386A4904  addi r3, r10, 0x4904
	ctx.r[3].s64 = ctx.r[10].s64 + 18692;
	// 826059D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826059D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826059DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826059E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826059E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826059E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826059EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826059F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826059F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826059F8: 4BE61429  bl 0x82466e20
	ctx.lr = 0x826059FC;
	sub_82466E20(ctx, base);
	// 826059FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605A10 size=24
    let mut pc: u32 = 0x82605A10;
    'dispatch: loop {
        match pc {
            0x82605A10 => {
    //   block [0x82605A10..0x82605A28)
	// 82605A10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605A14: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605A18: 394A2D38  addi r10, r10, 0x2d38
	ctx.r[10].s64 = ctx.r[10].s64 + 11576;
	// 82605A1C: 816BBF64  lwz r11, -0x409c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16540 as u32) ) } as u64;
	// 82605A20: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 82605A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605A28 size=116
    let mut pc: u32 = 0x82605A28;
    'dispatch: loop {
        match pc {
            0x82605A28 => {
    //   block [0x82605A28..0x82605A9C)
	// 82605A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605A34: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605A38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605A3C: 390B2D38  addi r8, r11, 0x2d38
	ctx.r[8].s64 = ctx.r[11].s64 + 11576;
	// 82605A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605A44: 392AD030  addi r9, r10, -0x2fd0
	ctx.r[9].s64 = ctx.r[10].s64 + -12240;
	// 82605A48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605A4C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82605A50: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605A54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605A5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605A6C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82605A70: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 82605A74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605A78: 386B4934  addi r3, r11, 0x4934
	ctx.r[3].s64 = ctx.r[11].s64 + 18740;
	// 82605A7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605A80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605A88: 4BE61399  bl 0x82466e20
	ctx.lr = 0x82605A8C;
	sub_82466E20(ctx, base);
	// 82605A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605AA0 size=112
    let mut pc: u32 = 0x82605AA0;
    'dispatch: loop {
        match pc {
            0x82605AA0 => {
    //   block [0x82605AA0..0x82605B10)
	// 82605AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605AAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605AB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605AB4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605ABC: 390BC220  addi r8, r11, -0x3de0
	ctx.r[8].s64 = ctx.r[11].s64 + -15840;
	// 82605AC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82605AC4: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 82605AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605AD8: 386A4964  addi r3, r10, 0x4964
	ctx.r[3].s64 = ctx.r[10].s64 + 18788;
	// 82605ADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605AFC: 4BE61325  bl 0x82466e20
	ctx.lr = 0x82605B00;
	sub_82466E20(ctx, base);
	// 82605B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605B10 size=112
    let mut pc: u32 = 0x82605B10;
    'dispatch: loop {
        match pc {
            0x82605B10 => {
    //   block [0x82605B10..0x82605B80)
	// 82605B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605B1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605B20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605B24: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605B28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605B2C: 390BC268  addi r8, r11, -0x3d98
	ctx.r[8].s64 = ctx.r[11].s64 + -15768;
	// 82605B30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82605B34: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 82605B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605B3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605B40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605B48: 386A4994  addi r3, r10, 0x4994
	ctx.r[3].s64 = ctx.r[10].s64 + 18836;
	// 82605B4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605B6C: 4BE612B5  bl 0x82466e20
	ctx.lr = 0x82605B70;
	sub_82466E20(ctx, base);
	// 82605B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605B80 size=112
    let mut pc: u32 = 0x82605B80;
    'dispatch: loop {
        match pc {
            0x82605B80 => {
    //   block [0x82605B80..0x82605BF0)
	// 82605B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605B90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605B94: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82605B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605B9C: 390BC2B0  addi r8, r11, -0x3d50
	ctx.r[8].s64 = ctx.r[11].s64 + -15696;
	// 82605BA0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82605BA4: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82605BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605BB8: 386A49C4  addi r3, r10, 0x49c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18884;
	// 82605BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605BDC: 4BE61245  bl 0x82466e20
	ctx.lr = 0x82605BE0;
	sub_82466E20(ctx, base);
	// 82605BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605BF0 size=112
    let mut pc: u32 = 0x82605BF0;
    'dispatch: loop {
        match pc {
            0x82605BF0 => {
    //   block [0x82605BF0..0x82605C60)
	// 82605BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605BFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82605C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605C04: 392BD064  addi r9, r11, -0x2f9c
	ctx.r[9].s64 = ctx.r[11].s64 + -12188;
	// 82605C08: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82605C0C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82605C10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605C14: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82605C18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605C1C: 396BC390  addi r11, r11, -0x3c70
	ctx.r[11].s64 = ctx.r[11].s64 + -15472;
	// 82605C20: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82605C24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605C28: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82605C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605C30: 386A49F4  addi r3, r10, 0x49f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18932;
	// 82605C34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605C38: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82605C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605C40: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82605C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605C48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605C4C: 4BE611D5  bl 0x82466e20
	ctx.lr = 0x82605C50;
	sub_82466E20(ctx, base);
	// 82605C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605C60 size=112
    let mut pc: u32 = 0x82605C60;
    'dispatch: loop {
        match pc {
            0x82605C60 => {
    //   block [0x82605C60..0x82605CD0)
	// 82605C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605C6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605C70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605C74: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605C7C: 390BC4E0  addi r8, r11, -0x3b20
	ctx.r[8].s64 = ctx.r[11].s64 + -15136;
	// 82605C80: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82605C84: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82605C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605C8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605C98: 386A4A24  addi r3, r10, 0x4a24
	ctx.r[3].s64 = ctx.r[10].s64 + 18980;
	// 82605C9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605CBC: 4BE61165  bl 0x82466e20
	ctx.lr = 0x82605CC0;
	sub_82466E20(ctx, base);
	// 82605CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605CD0 size=112
    let mut pc: u32 = 0x82605CD0;
    'dispatch: loop {
        match pc {
            0x82605CD0 => {
    //   block [0x82605CD0..0x82605D40)
	// 82605CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605CDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605CE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605CE4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605CEC: 390BC588  addi r8, r11, -0x3a78
	ctx.r[8].s64 = ctx.r[11].s64 + -14968;
	// 82605CF0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82605CF4: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 82605CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605CFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605D08: 386A4A54  addi r3, r10, 0x4a54
	ctx.r[3].s64 = ctx.r[10].s64 + 19028;
	// 82605D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605D2C: 4BE610F5  bl 0x82466e20
	ctx.lr = 0x82605D30;
	sub_82466E20(ctx, base);
	// 82605D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605D40 size=112
    let mut pc: u32 = 0x82605D40;
    'dispatch: loop {
        match pc {
            0x82605D40 => {
    //   block [0x82605D40..0x82605DB0)
	// 82605D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605D4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605D50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605D54: 392AD0C0  addi r9, r10, -0x2f40
	ctx.r[9].s64 = ctx.r[10].s64 + -12096;
	// 82605D58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605D5C: 390BC618  addi r8, r11, -0x39e8
	ctx.r[8].s64 = ctx.r[11].s64 + -14824;
	// 82605D60: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82605D64: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82605D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605D6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605D78: 386A4A84  addi r3, r10, 0x4a84
	ctx.r[3].s64 = ctx.r[10].s64 + 19076;
	// 82605D7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605D80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605D9C: 4BE61085  bl 0x82466e20
	ctx.lr = 0x82605DA0;
	sub_82466E20(ctx, base);
	// 82605DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605DB0 size=100
    let mut pc: u32 = 0x82605DB0;
    'dispatch: loop {
        match pc {
            0x82605DB0 => {
    //   block [0x82605DB0..0x82605E14)
	// 82605DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605DBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605DC4: 38AA51D4  addi r5, r10, 0x51d4
	ctx.r[5].s64 = ctx.r[10].s64 + 20948;
	// 82605DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605DD0: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82605DD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605DE4: 386A4AB4  addi r3, r10, 0x4ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 19124;
	// 82605DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605DEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605DF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82605DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605DF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605E00: 4BE61021  bl 0x82466e20
	ctx.lr = 0x82605E04;
	sub_82466E20(ctx, base);
	// 82605E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605E18 size=112
    let mut pc: u32 = 0x82605E18;
    'dispatch: loop {
        match pc {
            0x82605E18 => {
    //   block [0x82605E18..0x82605E88)
	// 82605E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605E24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605E28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605E2C: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82605E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605E34: 390BC648  addi r8, r11, -0x39b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14776;
	// 82605E38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82605E3C: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 82605E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605E50: 386A4AE4  addi r3, r10, 0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 19172;
	// 82605E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605E74: 4BE60FAD  bl 0x82466e20
	ctx.lr = 0x82605E78;
	sub_82466E20(ctx, base);
	// 82605E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605E88 size=116
    let mut pc: u32 = 0x82605E88;
    'dispatch: loop {
        match pc {
            0x82605E88 => {
    //   block [0x82605E88..0x82605EFC)
	// 82605E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605E94: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605E98: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82605E9C: 390AC678  addi r8, r10, -0x3988
	ctx.r[8].s64 = ctx.r[10].s64 + -14728;
	// 82605EA0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605EA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82605EA8: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82605EAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605EB0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605EBC: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82605EC0: 396BD0D4  addi r11, r11, -0x2f2c
	ctx.r[11].s64 = ctx.r[11].s64 + -12076;
	// 82605EC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605EC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605ECC: 386A4B14  addi r3, r10, 0x4b14
	ctx.r[3].s64 = ctx.r[10].s64 + 19220;
	// 82605ED0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82605ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605ED8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82605EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605EE8: 4BE60F39  bl 0x82466e20
	ctx.lr = 0x82605EEC;
	sub_82466E20(ctx, base);
	// 82605EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605F00 size=100
    let mut pc: u32 = 0x82605F00;
    'dispatch: loop {
        match pc {
            0x82605F00 => {
    //   block [0x82605F00..0x82605F64)
	// 82605F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605F0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605F14: 38AA4B14  addi r5, r10, 0x4b14
	ctx.r[5].s64 = ctx.r[10].s64 + 19220;
	// 82605F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605F20: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 82605F24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605F34: 386A4B44  addi r3, r10, 0x4b44
	ctx.r[3].s64 = ctx.r[10].s64 + 19268;
	// 82605F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605F3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605F40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82605F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605F48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605F50: 4BE60ED1  bl 0x82466e20
	ctx.lr = 0x82605F54;
	sub_82466E20(ctx, base);
	// 82605F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605F68 size=24
    let mut pc: u32 = 0x82605F68;
    'dispatch: loop {
        match pc {
            0x82605F68 => {
    //   block [0x82605F68..0x82605F80)
	// 82605F68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605F6C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605F70: 394A2E70  addi r10, r10, 0x2e70
	ctx.r[10].s64 = ctx.r[10].s64 + 11888;
	// 82605F74: 816BC720  lwz r11, -0x38e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14560 as u32) ) } as u64;
	// 82605F78: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82605F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605F80 size=116
    let mut pc: u32 = 0x82605F80;
    'dispatch: loop {
        match pc {
            0x82605F80 => {
    //   block [0x82605F80..0x82605FF4)
	// 82605F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605F8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82605F90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605F94: 392BD110  addi r9, r11, -0x2ef0
	ctx.r[9].s64 = ctx.r[11].s64 + -12016;
	// 82605F98: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605F9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605FA0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82605FA4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82605FA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605FAC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82605FB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605FB4: 396B2E70  addi r11, r11, 0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + 11888;
	// 82605FB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82605FBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605FC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82605FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605FC8: 386A4B74  addi r3, r10, 0x4b74
	ctx.r[3].s64 = ctx.r[10].s64 + 19316;
	// 82605FCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605FD0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82605FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605FD8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82605FDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605FE0: 4BE60E41  bl 0x82466e20
	ctx.lr = 0x82605FE4;
	sub_82466E20(ctx, base);
	// 82605FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605FF8 size=116
    let mut pc: u32 = 0x82605FF8;
    'dispatch: loop {
        match pc {
            0x82605FF8 => {
    //   block [0x82605FF8..0x8260606C)
	// 82605FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606004: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606008: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260600C: 392BD154  addi r9, r11, -0x2eac
	ctx.r[9].s64 = ctx.r[11].s64 + -11948;
	// 82606010: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606018: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260601C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82606020: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606024: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82606028: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260602C: 396BC728  addi r11, r11, -0x38d8
	ctx.r[11].s64 = ctx.r[11].s64 + -14552;
	// 82606030: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82606034: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606038: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260603C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606040: 386A4BA4  addi r3, r10, 0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 19364;
	// 82606044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606048: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260604C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606050: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82606054: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82606058: 4BE60DC9  bl 0x82466e20
	ctx.lr = 0x8260605C;
	sub_82466E20(ctx, base);
	// 8260605C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606070 size=108
    let mut pc: u32 = 0x82606070;
    'dispatch: loop {
        match pc {
            0x82606070 => {
    //   block [0x82606070..0x826060DC)
	// 82606070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260607C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606084: 38EBC7D0  addi r7, r11, -0x3830
	ctx.r[7].s64 = ctx.r[11].s64 + -14384;
	// 82606088: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260608C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82606090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260609C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826060A0: 386A4BD4  addi r3, r10, 0x4bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 19412;
	// 826060A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826060A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826060AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826060B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826060B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826060B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826060BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826060C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826060C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826060C8: 4BE60D59  bl 0x82466e20
	ctx.lr = 0x826060CC;
	sub_82466E20(ctx, base);
	// 826060CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826060D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826060D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826060D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826060E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826060E0 size=24
    let mut pc: u32 = 0x826060E0;
    'dispatch: loop {
        match pc {
            0x826060E0 => {
    //   block [0x826060E0..0x826060F8)
	// 826060E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826060E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826060E8: 394A2EE8  addi r10, r10, 0x2ee8
	ctx.r[10].s64 = ctx.r[10].s64 + 12008;
	// 826060EC: 816BC818  lwz r11, -0x37e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14312 as u32) ) } as u64;
	// 826060F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826060F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826060F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826060F8 size=116
    let mut pc: u32 = 0x826060F8;
    'dispatch: loop {
        match pc {
            0x826060F8 => {
    //   block [0x826060F8..0x8260616C)
	// 826060F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826060FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606104: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260610C: 390B2EE8  addi r8, r11, 0x2ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 12008;
	// 82606110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606114: 392AD1C0  addi r9, r10, -0x2e40
	ctx.r[9].s64 = ctx.r[10].s64 + -11840;
	// 82606118: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260611C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82606120: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606124: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260612C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260613C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606140: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 82606144: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606148: 386B4C04  addi r3, r11, 0x4c04
	ctx.r[3].s64 = ctx.r[11].s64 + 19460;
	// 8260614C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82606150: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606158: 4BE60CC9  bl 0x82466e20
	ctx.lr = 0x8260615C;
	sub_82466E20(ctx, base);
	// 8260615C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606170 size=112
    let mut pc: u32 = 0x82606170;
    'dispatch: loop {
        match pc {
            0x82606170 => {
    //   block [0x82606170..0x826061E0)
	// 82606170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260617C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606180: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606184: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260618C: 390BC820  addi r8, r11, -0x37e0
	ctx.r[8].s64 = ctx.r[11].s64 + -14304;
	// 82606190: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82606194: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82606198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260619C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826061A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826061A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826061A8: 386A4C34  addi r3, r10, 0x4c34
	ctx.r[3].s64 = ctx.r[10].s64 + 19508;
	// 826061AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826061B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826061B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826061B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826061BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826061C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826061C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826061C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826061CC: 4BE60C55  bl 0x82466e20
	ctx.lr = 0x826061D0;
	sub_82466E20(ctx, base);
	// 826061D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826061D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826061D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826061DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826061E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826061E0 size=24
    let mut pc: u32 = 0x826061E0;
    'dispatch: loop {
        match pc {
            0x826061E0 => {
    //   block [0x826061E0..0x826061F8)
	// 826061E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826061E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826061E8: 394A3068  addi r10, r10, 0x3068
	ctx.r[10].s64 = ctx.r[10].s64 + 12392;
	// 826061EC: 816BC850  lwz r11, -0x37b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14256 as u32) ) } as u64;
	// 826061F0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826061F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826061F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826061F8 size=116
    let mut pc: u32 = 0x826061F8;
    'dispatch: loop {
        match pc {
            0x826061F8 => {
    //   block [0x826061F8..0x8260626C)
	// 826061F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826061FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606204: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260620C: 390B3068  addi r8, r11, 0x3068
	ctx.r[8].s64 = ctx.r[11].s64 + 12392;
	// 82606210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606214: 392AD1F8  addi r9, r10, -0x2e08
	ctx.r[9].s64 = ctx.r[10].s64 + -11784;
	// 82606218: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260621C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 82606220: 38AA4BA4  addi r5, r10, 0x4ba4
	ctx.r[5].s64 = ctx.r[10].s64 + 19364;
	// 82606224: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260622C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260623C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606240: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 82606244: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606248: 386B4C64  addi r3, r11, 0x4c64
	ctx.r[3].s64 = ctx.r[11].s64 + 19556;
	// 8260624C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606250: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606258: 4BE60BC9  bl 0x82466e20
	ctx.lr = 0x8260625C;
	sub_82466E20(ctx, base);
	// 8260625C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606270 size=112
    let mut pc: u32 = 0x82606270;
    'dispatch: loop {
        match pc {
            0x82606270 => {
    //   block [0x82606270..0x826062E0)
	// 82606270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260627C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606280: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606284: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260628C: 390BC854  addi r8, r11, -0x37ac
	ctx.r[8].s64 = ctx.r[11].s64 + -14252;
	// 82606290: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606294: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82606298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260629C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826062A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826062A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826062A8: 386A4C94  addi r3, r10, 0x4c94
	ctx.r[3].s64 = ctx.r[10].s64 + 19604;
	// 826062AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826062B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826062B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826062B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826062BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826062C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826062C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826062C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826062CC: 4BE60B55  bl 0x82466e20
	ctx.lr = 0x826062D0;
	sub_82466E20(ctx, base);
	// 826062D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826062D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826062D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826062DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826062E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826062E0 size=100
    let mut pc: u32 = 0x826062E0;
    'dispatch: loop {
        match pc {
            0x826062E0 => {
    //   block [0x826062E0..0x82606344)
	// 826062E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826062E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826062E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826062EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826062F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826062F4: 38AA51D4  addi r5, r10, 0x51d4
	ctx.r[5].s64 = ctx.r[10].s64 + 20948;
	// 826062F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826062FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606300: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 82606304: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260630C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606314: 386A4CC4  addi r3, r10, 0x4cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 19652;
	// 82606318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260631C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606320: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82606324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606328: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260632C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606330: 4BE60AF1  bl 0x82466e20
	ctx.lr = 0x82606334;
	sub_82466E20(ctx, base);
	// 82606334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260633C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606348 size=108
    let mut pc: u32 = 0x82606348;
    'dispatch: loop {
        match pc {
            0x82606348 => {
    //   block [0x82606348..0x826063B4)
	// 82606348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606354: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260635C: 38EBC870  addi r7, r11, -0x3790
	ctx.r[7].s64 = ctx.r[11].s64 + -14224;
	// 82606360: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82606364: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 82606368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260636C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606378: 386A4CF4  addi r3, r10, 0x4cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 19700;
	// 8260637C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260638C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260639C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826063A0: 4BE60A81  bl 0x82466e20
	ctx.lr = 0x826063A4;
	sub_82466E20(ctx, base);
	// 826063A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826063A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826063AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826063B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826063B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826063B8 size=112
    let mut pc: u32 = 0x826063B8;
    'dispatch: loop {
        match pc {
            0x826063B8 => {
    //   block [0x826063B8..0x82606428)
	// 826063B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826063BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826063C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826063C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826063C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826063CC: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826063D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826063D4: 390BC948  addi r8, r11, -0x36b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14008;
	// 826063D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826063DC: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826063E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826063E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826063E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826063EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826063F0: 386A4D24  addi r3, r10, 0x4d24
	ctx.r[3].s64 = ctx.r[10].s64 + 19748;
	// 826063F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826063F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826063FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260640C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606414: 4BE60A0D  bl 0x82466e20
	ctx.lr = 0x82606418;
	sub_82466E20(ctx, base);
	// 82606418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606428 size=108
    let mut pc: u32 = 0x82606428;
    'dispatch: loop {
        match pc {
            0x82606428 => {
    //   block [0x82606428..0x82606494)
	// 82606428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606434: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260643C: 38EBC978  addi r7, r11, -0x3688
	ctx.r[7].s64 = ctx.r[11].s64 + -13960;
	// 82606440: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82606444: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 82606448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260644C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606458: 386A4D54  addi r3, r10, 0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + 19796;
	// 8260645C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260646C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260647C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606480: 4BE609A1  bl 0x82466e20
	ctx.lr = 0x82606484;
	sub_82466E20(ctx, base);
	// 82606484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260648C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606498 size=112
    let mut pc: u32 = 0x82606498;
    'dispatch: loop {
        match pc {
            0x82606498 => {
    //   block [0x82606498..0x82606508)
	// 82606498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826064A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826064A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826064A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826064AC: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826064B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826064B4: 390BC9A8  addi r8, r11, -0x3658
	ctx.r[8].s64 = ctx.r[11].s64 + -13912;
	// 826064B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826064BC: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826064C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826064C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826064C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826064CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826064D0: 386A4D84  addi r3, r10, 0x4d84
	ctx.r[3].s64 = ctx.r[10].s64 + 19844;
	// 826064D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826064D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826064DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826064E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826064E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826064E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826064EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826064F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826064F4: 4BE6092D  bl 0x82466e20
	ctx.lr = 0x826064F8;
	sub_82466E20(ctx, base);
	// 826064F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826064FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606508 size=112
    let mut pc: u32 = 0x82606508;
    'dispatch: loop {
        match pc {
            0x82606508 => {
    //   block [0x82606508..0x82606578)
	// 82606508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606514: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606518: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8260651C: 38EAC9C0  addi r7, r10, -0x3640
	ctx.r[7].s64 = ctx.r[10].s64 + -13888;
	// 82606520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606524: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606528: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 8260652C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606530: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606534: 396BD20C  addi r11, r11, -0x2df4
	ctx.r[11].s64 = ctx.r[11].s64 + -11764;
	// 82606538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260653C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606540: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606544: 386A4DB4  addi r3, r10, 0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + 19892;
	// 82606548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260654C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606550: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606554: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82606558: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260655C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606560: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606564: 4BE608BD  bl 0x82466e20
	ctx.lr = 0x82606568;
	sub_82466E20(ctx, base);
	// 82606568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260656C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606578 size=108
    let mut pc: u32 = 0x82606578;
    'dispatch: loop {
        match pc {
            0x82606578 => {
    //   block [0x82606578..0x826065E4)
	// 82606578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260657C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606584: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260658C: 38EBCA98  addi r7, r11, -0x3568
	ctx.r[7].s64 = ctx.r[11].s64 + -13672;
	// 82606590: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82606594: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 82606598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260659C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826065A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826065A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826065A8: 386A4DE4  addi r3, r10, 0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + 19940;
	// 826065AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826065B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826065B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826065B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826065BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826065C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826065C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826065C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826065CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826065D0: 4BE60851  bl 0x82466e20
	ctx.lr = 0x826065D4;
	sub_82466E20(ctx, base);
	// 826065D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826065D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826065DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826065E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826065E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826065E8 size=108
    let mut pc: u32 = 0x826065E8;
    'dispatch: loop {
        match pc {
            0x826065E8 => {
    //   block [0x826065E8..0x82606654)
	// 826065E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826065EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826065F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826065F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826065F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826065FC: 38EBCAB0  addi r7, r11, -0x3550
	ctx.r[7].s64 = ctx.r[11].s64 + -13648;
	// 82606600: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82606604: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82606608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260660C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606618: 386A4E14  addi r3, r10, 0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + 19988;
	// 8260661C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260662C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260663C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606640: 4BE607E1  bl 0x82466e20
	ctx.lr = 0x82606644;
	sub_82466E20(ctx, base);
	// 82606644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260664C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606658 size=112
    let mut pc: u32 = 0x82606658;
    'dispatch: loop {
        match pc {
            0x82606658 => {
    //   block [0x82606658..0x826066C8)
	// 82606658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606668: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260666C: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606674: 390BCBB8  addi r8, r11, -0x3448
	ctx.r[8].s64 = ctx.r[11].s64 + -13384;
	// 82606678: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8260667C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82606680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260668C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606690: 386A4E44  addi r3, r10, 0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + 20036;
	// 82606694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260669C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826066A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826066A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826066A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826066AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826066B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826066B4: 4BE6076D  bl 0x82466e20
	ctx.lr = 0x826066B8;
	sub_82466E20(ctx, base);
	// 826066B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826066BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826066C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826066C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826066C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826066C8 size=112
    let mut pc: u32 = 0x826066C8;
    'dispatch: loop {
        match pc {
            0x826066C8 => {
    //   block [0x826066C8..0x82606738)
	// 826066C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826066CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826066D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826066D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826066D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826066DC: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826066E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826066E4: 390BCCC0  addi r8, r11, -0x3340
	ctx.r[8].s64 = ctx.r[11].s64 + -13120;
	// 826066E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826066EC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826066F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826066F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826066F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826066FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606700: 386A4E74  addi r3, r10, 0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + 20084;
	// 82606704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260670C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260671C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606724: 4BE606FD  bl 0x82466e20
	ctx.lr = 0x82606728;
	sub_82466E20(ctx, base);
	// 82606728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260672C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606738 size=116
    let mut pc: u32 = 0x82606738;
    'dispatch: loop {
        match pc {
            0x82606738 => {
    //   block [0x82606738..0x826067AC)
	// 82606738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260673C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606744: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606748: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8260674C: 390ACCD8  addi r8, r10, -0x3328
	ctx.r[8].s64 = ctx.r[10].s64 + -13096;
	// 82606750: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606754: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606758: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 8260675C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606760: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260676C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82606770: 396BD23C  addi r11, r11, -0x2dc4
	ctx.r[11].s64 = ctx.r[11].s64 + -11716;
	// 82606774: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606778: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260677C: 386A4EA4  addi r3, r10, 0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 20132;
	// 82606780: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606784: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606788: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260678C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606798: 4BE60689  bl 0x82466e20
	ctx.lr = 0x8260679C;
	sub_82466E20(ctx, base);
	// 8260679C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826067A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826067A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826067A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826067B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826067B0 size=108
    let mut pc: u32 = 0x826067B0;
    'dispatch: loop {
        match pc {
            0x826067B0 => {
    //   block [0x826067B0..0x8260681C)
	// 826067B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826067B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826067B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826067BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826067C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826067C4: 38EBCD38  addi r7, r11, -0x32c8
	ctx.r[7].s64 = ctx.r[11].s64 + -13000;
	// 826067C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826067CC: 388A3224  addi r4, r10, 0x3224
	ctx.r[4].s64 = ctx.r[10].s64 + 12836;
	// 826067D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826067D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826067D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826067DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826067E0: 386A4ED4  addi r3, r10, 0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 20180;
	// 826067E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826067E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826067EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826067F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826067F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826067F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826067FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606808: 4BE60619  bl 0x82466e20
	ctx.lr = 0x8260680C;
	sub_82466E20(ctx, base);
	// 8260680C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606820 size=108
    let mut pc: u32 = 0x82606820;
    'dispatch: loop {
        match pc {
            0x82606820 => {
    //   block [0x82606820..0x8260688C)
	// 82606820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260682C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606834: 38EBCD80  addi r7, r11, -0x3280
	ctx.r[7].s64 = ctx.r[11].s64 + -12928;
	// 82606838: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260683C: 388A3244  addi r4, r10, 0x3244
	ctx.r[4].s64 = ctx.r[10].s64 + 12868;
	// 82606840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260684C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606850: 386A4F04  addi r3, r10, 0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + 20228;
	// 82606854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260685C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260686C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606878: 4BE605A9  bl 0x82466e20
	ctx.lr = 0x8260687C;
	sub_82466E20(ctx, base);
	// 8260687C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606890 size=112
    let mut pc: u32 = 0x82606890;
    'dispatch: loop {
        match pc {
            0x82606890 => {
    //   block [0x82606890..0x82606900)
	// 82606890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260689C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826068A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826068A4: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826068A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826068AC: 390BCDC8  addi r8, r11, -0x3238
	ctx.r[8].s64 = ctx.r[11].s64 + -12856;
	// 826068B0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826068B4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826068B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826068BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826068C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826068C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826068C8: 386A4F34  addi r3, r10, 0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + 20276;
	// 826068CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826068D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826068D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826068D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826068DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826068E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826068E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826068E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826068EC: 4BE60535  bl 0x82466e20
	ctx.lr = 0x826068F0;
	sub_82466E20(ctx, base);
	// 826068F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826068F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826068F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826068FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606900 size=112
    let mut pc: u32 = 0x82606900;
    'dispatch: loop {
        match pc {
            0x82606900 => {
    //   block [0x82606900..0x82606970)
	// 82606900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260690C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606910: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606914: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260691C: 390BCED0  addi r8, r11, -0x3130
	ctx.r[8].s64 = ctx.r[11].s64 + -12592;
	// 82606920: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82606924: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82606928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260692C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606938: 386A4F64  addi r3, r10, 0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + 20324;
	// 8260693C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260694C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260695C: 4BE604C5  bl 0x82466e20
	ctx.lr = 0x82606960;
	sub_82466E20(ctx, base);
	// 82606960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260696C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606970 size=112
    let mut pc: u32 = 0x82606970;
    'dispatch: loop {
        match pc {
            0x82606970 => {
    //   block [0x82606970..0x826069E0)
	// 82606970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260697C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606980: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606984: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260698C: 390BCFD8  addi r8, r11, -0x3028
	ctx.r[8].s64 = ctx.r[11].s64 + -12328;
	// 82606990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606994: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82606998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260699C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826069A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826069A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826069A8: 386A4F94  addi r3, r10, 0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + 20372;
	// 826069AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826069B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826069B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826069B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826069BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826069C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826069C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826069C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826069CC: 4BE60455  bl 0x82466e20
	ctx.lr = 0x826069D0;
	sub_82466E20(ctx, base);
	// 826069D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826069D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826069D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826069DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826069E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826069E0 size=112
    let mut pc: u32 = 0x826069E0;
    'dispatch: loop {
        match pc {
            0x826069E0 => {
    //   block [0x826069E0..0x82606A50)
	// 826069E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826069E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826069E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826069EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826069F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826069F4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826069F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826069FC: 390BCFF0  addi r8, r11, -0x3010
	ctx.r[8].s64 = ctx.r[11].s64 + -12304;
	// 82606A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82606A04: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82606A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606A18: 386A4FC4  addi r3, r10, 0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 20420;
	// 82606A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606A3C: 4BE603E5  bl 0x82466e20
	ctx.lr = 0x82606A40;
	sub_82466E20(ctx, base);
	// 82606A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606A50 size=108
    let mut pc: u32 = 0x82606A50;
    'dispatch: loop {
        match pc {
            0x82606A50 => {
    //   block [0x82606A50..0x82606ABC)
	// 82606A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606A5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606A64: 38EBD020  addi r7, r11, -0x2fe0
	ctx.r[7].s64 = ctx.r[11].s64 + -12256;
	// 82606A68: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82606A6C: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82606A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606A80: 386A4FF4  addi r3, r10, 0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 20468;
	// 82606A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606AA8: 4BE60379  bl 0x82466e20
	ctx.lr = 0x82606AAC;
	sub_82466E20(ctx, base);
	// 82606AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82606AC0 size=24
    let mut pc: u32 = 0x82606AC0;
    'dispatch: loop {
        match pc {
            0x82606AC0 => {
    //   block [0x82606AC0..0x82606AD8)
	// 82606AC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606AC4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606AC8: 394A3158  addi r10, r10, 0x3158
	ctx.r[10].s64 = ctx.r[10].s64 + 12632;
	// 82606ACC: 816BC86C  lwz r11, -0x3794(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14228 as u32) ) } as u64;
	// 82606AD0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82606AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606AD8 size=116
    let mut pc: u32 = 0x82606AD8;
    'dispatch: loop {
        match pc {
            0x82606AD8 => {
    //   block [0x82606AD8..0x82606B4C)
	// 82606AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606AE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606AE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82606AEC: 390B3158  addi r8, r11, 0x3158
	ctx.r[8].s64 = ctx.r[11].s64 + 12632;
	// 82606AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606AF4: 392AD268  addi r9, r10, -0x2d98
	ctx.r[9].s64 = ctx.r[10].s64 + -11672;
	// 82606AF8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606AFC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82606B00: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606B04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606B0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606B1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606B20: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 82606B24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606B28: 386B5024  addi r3, r11, 0x5024
	ctx.r[3].s64 = ctx.r[11].s64 + 20516;
	// 82606B2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606B30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606B38: 4BE602E9  bl 0x82466e20
	ctx.lr = 0x82606B3C;
	sub_82466E20(ctx, base);
	// 82606B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606B50 size=112
    let mut pc: u32 = 0x82606B50;
    'dispatch: loop {
        match pc {
            0x82606B50 => {
    //   block [0x82606B50..0x82606BC0)
	// 82606B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606B5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606B60: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606B64: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606B6C: 390BD098  addi r8, r11, -0x2f68
	ctx.r[8].s64 = ctx.r[11].s64 + -12136;
	// 82606B70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82606B74: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82606B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606B7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606B88: 386A5054  addi r3, r10, 0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + 20564;
	// 82606B8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606BAC: 4BE60275  bl 0x82466e20
	ctx.lr = 0x82606BB0;
	sub_82466E20(ctx, base);
	// 82606BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606BC0 size=116
    let mut pc: u32 = 0x82606BC0;
    'dispatch: loop {
        match pc {
            0x82606BC0 => {
    //   block [0x82606BC0..0x82606C34)
	// 82606BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606BCC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606BD0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82606BD4: 390AD0C8  addi r8, r10, -0x2f38
	ctx.r[8].s64 = ctx.r[10].s64 + -12088;
	// 82606BD8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606BDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606BE0: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606BE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606BE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606BF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606BF4: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82606BF8: 396BD27C  addi r11, r11, -0x2d84
	ctx.r[11].s64 = ctx.r[11].s64 + -11652;
	// 82606BFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606C00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606C04: 386A5084  addi r3, r10, 0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + 20612;
	// 82606C08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606C10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82606C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606C20: 4BE60201  bl 0x82466e20
	ctx.lr = 0x82606C24;
	sub_82466E20(ctx, base);
	// 82606C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606C38 size=112
    let mut pc: u32 = 0x82606C38;
    'dispatch: loop {
        match pc {
            0x82606C38 => {
    //   block [0x82606C38..0x82606CA8)
	// 82606C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606C48: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606C4C: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606C54: 390BD188  addi r8, r11, -0x2e78
	ctx.r[8].s64 = ctx.r[11].s64 + -11896;
	// 82606C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606C5C: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82606C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606C64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606C70: 386A50B4  addi r3, r10, 0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + 20660;
	// 82606C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606C94: 4BE6018D  bl 0x82466e20
	ctx.lr = 0x82606C98;
	sub_82466E20(ctx, base);
	// 82606C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606CA8 size=108
    let mut pc: u32 = 0x82606CA8;
    'dispatch: loop {
        match pc {
            0x82606CA8 => {
    //   block [0x82606CA8..0x82606D14)
	// 82606CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606CB4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606CBC: 38EBD1A0  addi r7, r11, -0x2e60
	ctx.r[7].s64 = ctx.r[11].s64 + -11872;
	// 82606CC0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82606CC4: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 82606CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606CCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606CD8: 386A50E4  addi r3, r10, 0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + 20708;
	// 82606CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606D00: 4BE60121  bl 0x82466e20
	ctx.lr = 0x82606D04;
	sub_82466E20(ctx, base);
	// 82606D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606D18 size=116
    let mut pc: u32 = 0x82606D18;
    'dispatch: loop {
        match pc {
            0x82606D18 => {
    //   block [0x82606D18..0x82606D8C)
	// 82606D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606D24: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606D28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82606D2C: 390AD2D8  addi r8, r10, -0x2d28
	ctx.r[8].s64 = ctx.r[10].s64 + -11560;
	// 82606D30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606D34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606D38: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606D3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606D40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606D4C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82606D50: 396BD2A0  addi r11, r11, -0x2d60
	ctx.r[11].s64 = ctx.r[11].s64 + -11616;
	// 82606D54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606D58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606D5C: 386A5114  addi r3, r10, 0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + 20756;
	// 82606D60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606D68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82606D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606D78: 4BE600A9  bl 0x82466e20
	ctx.lr = 0x82606D7C;
	sub_82466E20(ctx, base);
	// 82606D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606D90 size=112
    let mut pc: u32 = 0x82606D90;
    'dispatch: loop {
        match pc {
            0x82606D90 => {
    //   block [0x82606D90..0x82606E00)
	// 82606D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606D9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606DA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606DA4: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606DAC: 390BD380  addi r8, r11, -0x2c80
	ctx.r[8].s64 = ctx.r[11].s64 + -11392;
	// 82606DB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606DB4: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 82606DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606DBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606DC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606DC8: 386A5144  addi r3, r10, 0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + 20804;
	// 82606DCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606DEC: 4BE60035  bl 0x82466e20
	ctx.lr = 0x82606DF0;
	sub_82466E20(ctx, base);
	// 82606DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606E00 size=112
    let mut pc: u32 = 0x82606E00;
    'dispatch: loop {
        match pc {
            0x82606E00 => {
    //   block [0x82606E00..0x82606E70)
	// 82606E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606E0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606E10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606E14: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606E1C: 390BD398  addi r8, r11, -0x2c68
	ctx.r[8].s64 = ctx.r[11].s64 + -11368;
	// 82606E20: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82606E24: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82606E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606E2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606E30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606E38: 386A5174  addi r3, r10, 0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + 20852;
	// 82606E3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606E5C: 4BE5FFC5  bl 0x82466e20
	ctx.lr = 0x82606E60;
	sub_82466E20(ctx, base);
	// 82606E60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606E70 size=112
    let mut pc: u32 = 0x82606E70;
    'dispatch: loop {
        match pc {
            0x82606E70 => {
    //   block [0x82606E70..0x82606EE0)
	// 82606E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606E7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606E80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606E84: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606E8C: 390BD4B8  addi r8, r11, -0x2b48
	ctx.r[8].s64 = ctx.r[11].s64 + -11080;
	// 82606E90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606E94: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 82606E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606E9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606EA8: 386A51A4  addi r3, r10, 0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + 20900;
	// 82606EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606ECC: 4BE5FF55  bl 0x82466e20
	ctx.lr = 0x82606ED0;
	sub_82466E20(ctx, base);
	// 82606ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606EE0 size=116
    let mut pc: u32 = 0x82606EE0;
    'dispatch: loop {
        match pc {
            0x82606EE0 => {
    //   block [0x82606EE0..0x82606F54)
	// 82606EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606EEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82606EF4: 390BD4D4  addi r8, r11, -0x2b2c
	ctx.r[8].s64 = ctx.r[11].s64 + -11052;
	// 82606EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606EFC: 392AD2D8  addi r9, r10, -0x2d28
	ctx.r[9].s64 = ctx.r[10].s64 + -11560;
	// 82606F00: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606F04: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82606F08: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82606F0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606F14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606F24: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606F28: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82606F2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606F30: 386B51D4  addi r3, r11, 0x51d4
	ctx.r[3].s64 = ctx.r[11].s64 + 20948;
	// 82606F34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606F38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606F40: 4BE5FEE1  bl 0x82466e20
	ctx.lr = 0x82606F44;
	sub_82466E20(ctx, base);
	// 82606F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606F58 size=100
    let mut pc: u32 = 0x82606F58;
    'dispatch: loop {
        match pc {
            0x82606F58 => {
    //   block [0x82606F58..0x82606FBC)
	// 82606F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606F64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606F6C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82606F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606F78: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82606F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606F8C: 386A5204  addi r3, r10, 0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + 20996;
	// 82606F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606F94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606F98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82606F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606FA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82606FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606FA8: 4BE5FE79  bl 0x82466e20
	ctx.lr = 0x82606FAC;
	sub_82466E20(ctx, base);
	// 82606FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606FC0 size=112
    let mut pc: u32 = 0x82606FC0;
    'dispatch: loop {
        match pc {
            0x82606FC0 => {
    //   block [0x82606FC0..0x82607030)
	// 82606FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606FCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606FD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606FD4: 38AA5204  addi r5, r10, 0x5204
	ctx.r[5].s64 = ctx.r[10].s64 + 20996;
	// 82606FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606FDC: 390BD504  addi r8, r11, -0x2afc
	ctx.r[8].s64 = ctx.r[11].s64 + -11004;
	// 82606FE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606FE4: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82606FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606FEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606FF8: 386A5234  addi r3, r10, 0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + 21044;
	// 82606FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260700C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260701C: 4BE5FE05  bl 0x82466e20
	ctx.lr = 0x82607020;
	sub_82466E20(ctx, base);
	// 82607020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260702C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607030 size=112
    let mut pc: u32 = 0x82607030;
    'dispatch: loop {
        match pc {
            0x82607030 => {
    //   block [0x82607030..0x826070A0)
	// 82607030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260703C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607040: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607044: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260704C: 390BD520  addi r8, r11, -0x2ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -10976;
	// 82607050: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82607054: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82607058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260705C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607068: 386A5264  addi r3, r10, 0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + 21092;
	// 8260706C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260707C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260708C: 4BE5FD95  bl 0x82466e20
	ctx.lr = 0x82607090;
	sub_82466E20(ctx, base);
	// 82607090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260709C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826070A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826070A0 size=112
    let mut pc: u32 = 0x826070A0;
    'dispatch: loop {
        match pc {
            0x826070A0 => {
    //   block [0x826070A0..0x82607110)
	// 826070A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826070A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826070A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826070AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826070B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826070B4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826070B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826070BC: 390BD580  addi r8, r11, -0x2a80
	ctx.r[8].s64 = ctx.r[11].s64 + -10880;
	// 826070C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826070C4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826070C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826070CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826070D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826070D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826070D8: 386A5294  addi r3, r10, 0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + 21140;
	// 826070DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826070E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826070E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826070E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826070EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826070F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826070F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826070F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826070FC: 4BE5FD25  bl 0x82466e20
	ctx.lr = 0x82607100;
	sub_82466E20(ctx, base);
	// 82607100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260710C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607110 size=112
    let mut pc: u32 = 0x82607110;
    'dispatch: loop {
        match pc {
            0x82607110 => {
    //   block [0x82607110..0x82607180)
	// 82607110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260711C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607120: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607124: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260712C: 390BD5C8  addi r8, r11, -0x2a38
	ctx.r[8].s64 = ctx.r[11].s64 + -10808;
	// 82607130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82607134: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82607138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260713C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607148: 386A52C4  addi r3, r10, 0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + 21188;
	// 8260714C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260715C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260716C: 4BE5FCB5  bl 0x82466e20
	ctx.lr = 0x82607170;
	sub_82466E20(ctx, base);
	// 82607170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260717C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607180 size=112
    let mut pc: u32 = 0x82607180;
    'dispatch: loop {
        match pc {
            0x82607180 => {
    //   block [0x82607180..0x826071F0)
	// 82607180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260718C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607190: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607194: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82607198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260719C: 390BD5F8  addi r8, r11, -0x2a08
	ctx.r[8].s64 = ctx.r[11].s64 + -10760;
	// 826071A0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826071A4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826071A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826071AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826071B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826071B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826071B8: 386A52F4  addi r3, r10, 0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + 21236;
	// 826071BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826071C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826071C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826071C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826071CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826071D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826071D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826071D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826071DC: 4BE5FC45  bl 0x82466e20
	ctx.lr = 0x826071E0;
	sub_82466E20(ctx, base);
	// 826071E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826071E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826071E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826071EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826071F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826071F0 size=100
    let mut pc: u32 = 0x826071F0;
    'dispatch: loop {
        match pc {
            0x826071F0 => {
    //   block [0x826071F0..0x82607254)
	// 826071F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826071F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826071F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826071FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607204: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260720C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607210: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82607214: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260721C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607224: 386A5324  addi r3, r10, 0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + 21284;
	// 82607228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260722C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607230: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82607234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607238: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260723C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607240: 4BE5FBE1  bl 0x82466e20
	ctx.lr = 0x82607244;
	sub_82466E20(ctx, base);
	// 82607244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260724C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607258 size=108
    let mut pc: u32 = 0x82607258;
    'dispatch: loop {
        match pc {
            0x82607258 => {
    //   block [0x82607258..0x826072C4)
	// 82607258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260725C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607264: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260726C: 38EBD670  addi r7, r11, -0x2990
	ctx.r[7].s64 = ctx.r[11].s64 + -10640;
	// 82607270: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607274: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82607278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260727C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607288: 386A5354  addi r3, r10, 0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + 21332;
	// 8260728C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260729C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826072A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826072A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826072A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826072AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826072B0: 4BE5FB71  bl 0x82466e20
	ctx.lr = 0x826072B4;
	sub_82466E20(ctx, base);
	// 826072B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826072B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826072BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826072C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826072C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826072C8 size=112
    let mut pc: u32 = 0x826072C8;
    'dispatch: loop {
        match pc {
            0x826072C8 => {
    //   block [0x826072C8..0x82607338)
	// 826072C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826072CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826072D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826072D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826072D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826072DC: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 826072E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826072E4: 390BD6A0  addi r8, r11, -0x2960
	ctx.r[8].s64 = ctx.r[11].s64 + -10592;
	// 826072E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826072EC: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826072F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826072F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826072F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826072FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607300: 386A5384  addi r3, r10, 0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + 21380;
	// 82607304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260730C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260731C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607324: 4BE5FAFD  bl 0x82466e20
	ctx.lr = 0x82607328;
	sub_82466E20(ctx, base);
	// 82607328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260732C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607338 size=108
    let mut pc: u32 = 0x82607338;
    'dispatch: loop {
        match pc {
            0x82607338 => {
    //   block [0x82607338..0x826073A4)
	// 82607338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260733C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607344: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260734C: 38EBD6D0  addi r7, r11, -0x2930
	ctx.r[7].s64 = ctx.r[11].s64 + -10544;
	// 82607350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607354: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82607358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260735C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607368: 386A53B4  addi r3, r10, 0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + 21428;
	// 8260736C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260737C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260738C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607390: 4BE5FA91  bl 0x82466e20
	ctx.lr = 0x82607394;
	sub_82466E20(ctx, base);
	// 82607394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260739C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826073A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826073A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826073A8 size=112
    let mut pc: u32 = 0x826073A8;
    'dispatch: loop {
        match pc {
            0x826073A8 => {
    //   block [0x826073A8..0x82607418)
	// 826073A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826073AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826073B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826073B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826073B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826073BC: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 826073C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826073C4: 390BD700  addi r8, r11, -0x2900
	ctx.r[8].s64 = ctx.r[11].s64 + -10496;
	// 826073C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826073CC: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826073D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826073D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826073D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826073DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826073E0: 386A53E4  addi r3, r10, 0x53e4
	ctx.r[3].s64 = ctx.r[10].s64 + 21476;
	// 826073E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826073E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826073EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826073F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826073F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826073F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826073FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607404: 4BE5FA1D  bl 0x82466e20
	ctx.lr = 0x82607408;
	sub_82466E20(ctx, base);
	// 82607408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260740C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607418 size=108
    let mut pc: u32 = 0x82607418;
    'dispatch: loop {
        match pc {
            0x82607418 => {
    //   block [0x82607418..0x82607484)
	// 82607418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260741C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607424: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260742C: 38EBD748  addi r7, r11, -0x28b8
	ctx.r[7].s64 = ctx.r[11].s64 + -10424;
	// 82607430: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607434: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82607438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260743C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607440: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607448: 386A5414  addi r3, r10, 0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + 21524;
	// 8260744C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260745C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260746C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607470: 4BE5F9B1  bl 0x82466e20
	ctx.lr = 0x82607474;
	sub_82466E20(ctx, base);
	// 82607474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260747C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607488 size=112
    let mut pc: u32 = 0x82607488;
    'dispatch: loop {
        match pc {
            0x82607488 => {
    //   block [0x82607488..0x826074F8)
	// 82607488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260748C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607498: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260749C: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 826074A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826074A4: 390BD778  addi r8, r11, -0x2888
	ctx.r[8].s64 = ctx.r[11].s64 + -10376;
	// 826074A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826074AC: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826074B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826074B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826074B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826074BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826074C0: 386A5444  addi r3, r10, 0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + 21572;
	// 826074C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826074C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826074CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826074D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826074D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826074D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826074DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826074E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826074E4: 4BE5F93D  bl 0x82466e20
	ctx.lr = 0x826074E8;
	sub_82466E20(ctx, base);
	// 826074E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826074EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826074F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826074F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826074F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826074F8 size=108
    let mut pc: u32 = 0x826074F8;
    'dispatch: loop {
        match pc {
            0x826074F8 => {
    //   block [0x826074F8..0x82607564)
	// 826074F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826074FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607504: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260750C: 38EBD7C0  addi r7, r11, -0x2840
	ctx.r[7].s64 = ctx.r[11].s64 + -10304;
	// 82607510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607514: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 82607518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260751C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607528: 386A5474  addi r3, r10, 0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + 21620;
	// 8260752C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260753C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260754C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607550: 4BE5F8D1  bl 0x82466e20
	ctx.lr = 0x82607554;
	sub_82466E20(ctx, base);
	// 82607554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260755C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607568 size=112
    let mut pc: u32 = 0x82607568;
    'dispatch: loop {
        match pc {
            0x82607568 => {
    //   block [0x82607568..0x826075D8)
	// 82607568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260756C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607578: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260757C: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 82607580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607584: 390BD7F0  addi r8, r11, -0x2810
	ctx.r[8].s64 = ctx.r[11].s64 + -10256;
	// 82607588: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260758C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82607590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826075A0: 386A54A4  addi r3, r10, 0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + 21668;
	// 826075A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826075A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826075AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826075B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826075B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826075B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826075BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826075C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826075C4: 4BE5F85D  bl 0x82466e20
	ctx.lr = 0x826075C8;
	sub_82466E20(ctx, base);
	// 826075C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826075CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826075D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826075D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826075D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826075D8 size=108
    let mut pc: u32 = 0x826075D8;
    'dispatch: loop {
        match pc {
            0x826075D8 => {
    //   block [0x826075D8..0x82607644)
	// 826075D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826075DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826075E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826075E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826075E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826075EC: 38EBD838  addi r7, r11, -0x27c8
	ctx.r[7].s64 = ctx.r[11].s64 + -10184;
	// 826075F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826075F4: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826075F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826075FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607608: 386A54D4  addi r3, r10, 0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + 21716;
	// 8260760C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260761C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260762C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607630: 4BE5F7F1  bl 0x82466e20
	ctx.lr = 0x82607634;
	sub_82466E20(ctx, base);
	// 82607634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260763C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607648 size=112
    let mut pc: u32 = 0x82607648;
    'dispatch: loop {
        match pc {
            0x82607648 => {
    //   block [0x82607648..0x826076B8)
	// 82607648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260764C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607658: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260765C: 392AD348  addi r9, r10, -0x2cb8
	ctx.r[9].s64 = ctx.r[10].s64 + -11448;
	// 82607660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607664: 390BD898  addi r8, r11, -0x2768
	ctx.r[8].s64 = ctx.r[11].s64 + -10088;
	// 82607668: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8260766C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82607670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260767C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607680: 386A5504  addi r3, r10, 0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + 21764;
	// 82607684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260768C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260769C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826076A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826076A4: 4BE5F77D  bl 0x82466e20
	ctx.lr = 0x826076A8;
	sub_82466E20(ctx, base);
	// 826076A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826076AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826076B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826076B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826076B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826076B8 size=108
    let mut pc: u32 = 0x826076B8;
    'dispatch: loop {
        match pc {
            0x826076B8 => {
    //   block [0x826076B8..0x82607724)
	// 826076B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826076BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826076C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826076C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826076C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826076CC: 38EBD958  addi r7, r11, -0x26a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9896;
	// 826076D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826076D4: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826076D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826076DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826076E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826076E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826076E8: 386A5534  addi r3, r10, 0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + 21812;
	// 826076EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826076F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826076F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826076F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826076FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260770C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607710: 4BE5F711  bl 0x82466e20
	ctx.lr = 0x82607714;
	sub_82466E20(ctx, base);
	// 82607714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260771C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607728 size=116
    let mut pc: u32 = 0x82607728;
    'dispatch: loop {
        match pc {
            0x82607728 => {
    //   block [0x82607728..0x8260779C)
	// 82607728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607734: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607738: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 8260773C: 390AD9D0  addi r8, r10, -0x2630
	ctx.r[8].s64 = ctx.r[10].s64 + -9776;
	// 82607740: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607744: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82607748: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 8260774C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607750: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260775C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82607760: 396BD360  addi r11, r11, -0x2ca0
	ctx.r[11].s64 = ctx.r[11].s64 + -11424;
	// 82607764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607768: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260776C: 386A5564  addi r3, r10, 0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + 21860;
	// 82607770: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82607774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607778: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260777C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607788: 4BE5F699  bl 0x82466e20
	ctx.lr = 0x8260778C;
	sub_82466E20(ctx, base);
	// 8260778C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826077A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826077A0 size=100
    let mut pc: u32 = 0x826077A0;
    'dispatch: loop {
        match pc {
            0x826077A0 => {
    //   block [0x826077A0..0x82607804)
	// 826077A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826077A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826077A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826077AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826077B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826077B4: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 826077B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826077BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826077C0: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826077C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826077C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826077CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826077D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826077D4: 386A5594  addi r3, r10, 0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + 21908;
	// 826077D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826077DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826077E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826077E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826077E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826077EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826077F0: 4BE5F631  bl 0x82466e20
	ctx.lr = 0x826077F4;
	sub_82466E20(ctx, base);
	// 826077F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826077F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826077FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607808 size=24
    let mut pc: u32 = 0x82607808;
    'dispatch: loop {
        match pc {
            0x82607808 => {
    //   block [0x82607808..0x82607820)
	// 82607808: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260780C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607810: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 82607814: 816BDB6C  lwz r11, -0x2494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9364 as u32) ) } as u64;
	// 82607818: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8260781C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607820 size=116
    let mut pc: u32 = 0x82607820;
    'dispatch: loop {
        match pc {
            0x82607820 => {
    //   block [0x82607820..0x82607894)
	// 82607820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260782C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607834: 390B3260  addi r8, r11, 0x3260
	ctx.r[8].s64 = ctx.r[11].s64 + 12896;
	// 82607838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260783C: 392AD3D8  addi r9, r10, -0x2c28
	ctx.r[9].s64 = ctx.r[10].s64 + -11304;
	// 82607840: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607844: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82607848: 38AA5594  addi r5, r10, 0x5594
	ctx.r[5].s64 = ctx.r[10].s64 + 21908;
	// 8260784C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607854: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260785C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607864: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82607868: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 8260786C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607870: 386B55C4  addi r3, r11, 0x55c4
	ctx.r[3].s64 = ctx.r[11].s64 + 21956;
	// 82607874: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82607878: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260787C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607880: 4BE5F5A1  bl 0x82466e20
	ctx.lr = 0x82607884;
	sub_82466E20(ctx, base);
	// 82607884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607898 size=108
    let mut pc: u32 = 0x82607898;
    'dispatch: loop {
        match pc {
            0x82607898 => {
    //   block [0x82607898..0x82607904)
	// 82607898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826078A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826078A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826078A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826078AC: 38EBDB70  addi r7, r11, -0x2490
	ctx.r[7].s64 = ctx.r[11].s64 + -9360;
	// 826078B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826078B4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826078B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826078BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826078C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826078C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826078C8: 386A55F4  addi r3, r10, 0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22004;
	// 826078CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826078D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826078D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826078D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826078DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826078E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826078E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826078E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826078EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826078F0: 4BE5F531  bl 0x82466e20
	ctx.lr = 0x826078F4;
	sub_82466E20(ctx, base);
	// 826078F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826078F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826078FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607908 size=24
    let mut pc: u32 = 0x82607908;
    'dispatch: loop {
        match pc {
            0x82607908 => {
    //   block [0x82607908..0x82607920)
	// 82607908: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260790C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607910: 394A3380  addi r10, r10, 0x3380
	ctx.r[10].s64 = ctx.r[10].s64 + 13184;
	// 82607914: 816BDB88  lwz r11, -0x2478(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9336 as u32) ) } as u64;
	// 82607918: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260791C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607920 size=112
    let mut pc: u32 = 0x82607920;
    'dispatch: loop {
        match pc {
            0x82607920 => {
    //   block [0x82607920..0x82607990)
	// 82607920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260792C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607930: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607934: 392AD430  addi r9, r10, -0x2bd0
	ctx.r[9].s64 = ctx.r[10].s64 + -11216;
	// 82607938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260793C: 390B3380  addi r8, r11, 0x3380
	ctx.r[8].s64 = ctx.r[11].s64 + 13184;
	// 82607940: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82607944: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82607948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260794C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607958: 386A5624  addi r3, r10, 0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + 22052;
	// 8260795C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607960: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82607964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260796C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260797C: 4BE5F4A5  bl 0x82466e20
	ctx.lr = 0x82607980;
	sub_82466E20(ctx, base);
	// 82607980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260798C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607990 size=108
    let mut pc: u32 = 0x82607990;
    'dispatch: loop {
        match pc {
            0x82607990 => {
    //   block [0x82607990..0x826079FC)
	// 82607990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260799C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826079A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826079A4: 38EBDB90  addi r7, r11, -0x2470
	ctx.r[7].s64 = ctx.r[11].s64 + -9328;
	// 826079A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826079AC: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826079B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826079B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826079B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826079BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826079C0: 386A5654  addi r3, r10, 0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + 22100;
	// 826079C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826079C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826079CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826079D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826079D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826079D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826079DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826079E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826079E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826079E8: 4BE5F439  bl 0x82466e20
	ctx.lr = 0x826079EC;
	sub_82466E20(ctx, base);
	// 826079EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826079F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826079F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826079F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607A00 size=112
    let mut pc: u32 = 0x82607A00;
    'dispatch: loop {
        match pc {
            0x82607A00 => {
    //   block [0x82607A00..0x82607A70)
	// 82607A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607A10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607A14: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607A1C: 390BDBF0  addi r8, r11, -0x2410
	ctx.r[8].s64 = ctx.r[11].s64 + -9232;
	// 82607A20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82607A24: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 82607A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607A2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607A38: 386A5684  addi r3, r10, 0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + 22148;
	// 82607A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607A5C: 4BE5F3C5  bl 0x82466e20
	ctx.lr = 0x82607A60;
	sub_82466E20(ctx, base);
	// 82607A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607A70 size=108
    let mut pc: u32 = 0x82607A70;
    'dispatch: loop {
        match pc {
            0x82607A70 => {
    //   block [0x82607A70..0x82607ADC)
	// 82607A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607A7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607A84: 38EBDC08  addi r7, r11, -0x23f8
	ctx.r[7].s64 = ctx.r[11].s64 + -9208;
	// 82607A88: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82607A8C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82607A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607A94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607AA0: 386A56B4  addi r3, r10, 0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22196;
	// 82607AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607AC8: 4BE5F359  bl 0x82466e20
	ctx.lr = 0x82607ACC;
	sub_82466E20(ctx, base);
	// 82607ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607AE0 size=108
    let mut pc: u32 = 0x82607AE0;
    'dispatch: loop {
        match pc {
            0x82607AE0 => {
    //   block [0x82607AE0..0x82607B4C)
	// 82607AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607AEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607AF4: 38EBDC68  addi r7, r11, -0x2398
	ctx.r[7].s64 = ctx.r[11].s64 + -9112;
	// 82607AF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607AFC: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82607B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607B04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607B10: 386A56E4  addi r3, r10, 0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + 22244;
	// 82607B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607B38: 4BE5F2E9  bl 0x82466e20
	ctx.lr = 0x82607B3C;
	sub_82466E20(ctx, base);
	// 82607B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607B50 size=116
    let mut pc: u32 = 0x82607B50;
    'dispatch: loop {
        match pc {
            0x82607B50 => {
    //   block [0x82607B50..0x82607BC4)
	// 82607B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607B5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607B60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607B64: 390BDC98  addi r8, r11, -0x2368
	ctx.r[8].s64 = ctx.r[11].s64 + -9064;
	// 82607B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607B6C: 392AD45C  addi r9, r10, -0x2ba4
	ctx.r[9].s64 = ctx.r[10].s64 + -11172;
	// 82607B70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607B74: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82607B78: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607B7C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607B84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607B94: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82607B98: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82607B9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607BA0: 386B5714  addi r3, r11, 0x5714
	ctx.r[3].s64 = ctx.r[11].s64 + 22292;
	// 82607BA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82607BA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607BB0: 4BE5F271  bl 0x82466e20
	ctx.lr = 0x82607BB4;
	sub_82466E20(ctx, base);
	// 82607BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607BC8 size=96
    let mut pc: u32 = 0x82607BC8;
    'dispatch: loop {
        match pc {
            0x82607BC8 => {
    //   block [0x82607BC8..0x82607C28)
	// 82607BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607BD4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607BDC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82607BE0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607BE8: 386A5744  addi r3, r10, 0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + 22340;
	// 82607BEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607BF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82607BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607C08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82607C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607C10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82607C14: 4BE5F20D  bl 0x82466e20
	ctx.lr = 0x82607C18;
	sub_82466E20(ctx, base);
	// 82607C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607C28 size=112
    let mut pc: u32 = 0x82607C28;
    'dispatch: loop {
        match pc {
            0x82607C28 => {
    //   block [0x82607C28..0x82607C98)
	// 82607C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607C38: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607C3C: 38AA5744  addi r5, r10, 0x5744
	ctx.r[5].s64 = ctx.r[10].s64 + 22340;
	// 82607C40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607C44: 390BDCB0  addi r8, r11, -0x2350
	ctx.r[8].s64 = ctx.r[11].s64 + -9040;
	// 82607C48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82607C4C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82607C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607C54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607C60: 386A5774  addi r3, r10, 0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + 22388;
	// 82607C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607C84: 4BE5F19D  bl 0x82466e20
	ctx.lr = 0x82607C88;
	sub_82466E20(ctx, base);
	// 82607C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607C98 size=112
    let mut pc: u32 = 0x82607C98;
    'dispatch: loop {
        match pc {
            0x82607C98 => {
    //   block [0x82607C98..0x82607D08)
	// 82607C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607CA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607CA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607CAC: 392AD478  addi r9, r10, -0x2b88
	ctx.r[9].s64 = ctx.r[10].s64 + -11144;
	// 82607CB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607CB4: 390BDCE8  addi r8, r11, -0x2318
	ctx.r[8].s64 = ctx.r[11].s64 + -8984;
	// 82607CB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82607CBC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82607CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607CC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607CD0: 386A57A4  addi r3, r10, 0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22436;
	// 82607CD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607CD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82607CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607CF4: 4BE5F12D  bl 0x82466e20
	ctx.lr = 0x82607CF8;
	sub_82466E20(ctx, base);
	// 82607CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607D08 size=108
    let mut pc: u32 = 0x82607D08;
    'dispatch: loop {
        match pc {
            0x82607D08 => {
    //   block [0x82607D08..0x82607D74)
	// 82607D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607D14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607D18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607D1C: 38EBDD90  addi r7, r11, -0x2270
	ctx.r[7].s64 = ctx.r[11].s64 + -8816;
	// 82607D20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607D24: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82607D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607D2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607D38: 386A57D4  addi r3, r10, 0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22484;
	// 82607D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607D60: 4BE5F0C1  bl 0x82466e20
	ctx.lr = 0x82607D64;
	sub_82466E20(ctx, base);
	// 82607D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607D78 size=108
    let mut pc: u32 = 0x82607D78;
    'dispatch: loop {
        match pc {
            0x82607D78 => {
    //   block [0x82607D78..0x82607DE4)
	// 82607D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607D84: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607D88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607D8C: 38EBDDC0  addi r7, r11, -0x2240
	ctx.r[7].s64 = ctx.r[11].s64 + -8768;
	// 82607D90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607D94: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82607D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607D9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607DA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607DA8: 386A5804  addi r3, r10, 0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + 22532;
	// 82607DAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607DCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607DD0: 4BE5F051  bl 0x82466e20
	ctx.lr = 0x82607DD4;
	sub_82466E20(ctx, base);
	// 82607DD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607DE8 size=28
    let mut pc: u32 = 0x82607DE8;
    'dispatch: loop {
        match pc {
            0x82607DE8 => {
    //   block [0x82607DE8..0x82607E04)
	// 82607DE8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607DEC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607DF0: 394A33B0  addi r10, r10, 0x33b0
	ctx.r[10].s64 = ctx.r[10].s64 + 13232;
	// 82607DF4: 816BDCE4  lwz r11, -0x231c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8988 as u32) ) } as u64;
	// 82607DF8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82607DFC: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82607E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607E08 size=112
    let mut pc: u32 = 0x82607E08;
    'dispatch: loop {
        match pc {
            0x82607E08 => {
    //   block [0x82607E08..0x82607E78)
	// 82607E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607E14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607E18: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607E1C: 392AD608  addi r9, r10, -0x29f8
	ctx.r[9].s64 = ctx.r[10].s64 + -10744;
	// 82607E20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607E24: 390B33B0  addi r8, r11, 0x33b0
	ctx.r[8].s64 = ctx.r[11].s64 + 13232;
	// 82607E28: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82607E2C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82607E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607E34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607E40: 386A5834  addi r3, r10, 0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + 22580;
	// 82607E44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607E48: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82607E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607E64: 4BE5EFBD  bl 0x82466e20
	ctx.lr = 0x82607E68;
	sub_82466E20(ctx, base);
	// 82607E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607E78 size=108
    let mut pc: u32 = 0x82607E78;
    'dispatch: loop {
        match pc {
            0x82607E78 => {
    //   block [0x82607E78..0x82607EE4)
	// 82607E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607E84: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607E88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607E8C: 38EBDDF8  addi r7, r11, -0x2208
	ctx.r[7].s64 = ctx.r[11].s64 + -8712;
	// 82607E90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607E94: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82607E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607E9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607EA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607EA8: 386A5864  addi r3, r10, 0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + 22628;
	// 82607EAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607ED0: 4BE5EF51  bl 0x82466e20
	ctx.lr = 0x82607ED4;
	sub_82466E20(ctx, base);
	// 82607ED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607EE8 size=108
    let mut pc: u32 = 0x82607EE8;
    'dispatch: loop {
        match pc {
            0x82607EE8 => {
    //   block [0x82607EE8..0x82607F54)
	// 82607EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607EF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607EF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607EFC: 38EBDE28  addi r7, r11, -0x21d8
	ctx.r[7].s64 = ctx.r[11].s64 + -8664;
	// 82607F00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607F04: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82607F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607F0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607F18: 386A5894  addi r3, r10, 0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + 22676;
	// 82607F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607F40: 4BE5EEE1  bl 0x82466e20
	ctx.lr = 0x82607F44;
	sub_82466E20(ctx, base);
	// 82607F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607F58 size=108
    let mut pc: u32 = 0x82607F58;
    'dispatch: loop {
        match pc {
            0x82607F58 => {
    //   block [0x82607F58..0x82607FC4)
	// 82607F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607F64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607F68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607F6C: 38EBDE58  addi r7, r11, -0x21a8
	ctx.r[7].s64 = ctx.r[11].s64 + -8616;
	// 82607F70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82607F74: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82607F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607F88: 386A58C4  addi r3, r10, 0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + 22724;
	// 82607F8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607FAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607FB0: 4BE5EE71  bl 0x82466e20
	ctx.lr = 0x82607FB4;
	sub_82466E20(ctx, base);
	// 82607FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607FC8 size=24
    let mut pc: u32 = 0x82607FC8;
    'dispatch: loop {
        match pc {
            0x82607FC8 => {
    //   block [0x82607FC8..0x82607FE0)
	// 82607FC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607FCC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607FD0: 394A3470  addi r10, r10, 0x3470
	ctx.r[10].s64 = ctx.r[10].s64 + 13424;
	// 82607FD4: 816BDE70  lwz r11, -0x2190(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8592 as u32) ) } as u64;
	// 82607FD8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82607FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607FE0 size=112
    let mut pc: u32 = 0x82607FE0;
    'dispatch: loop {
        match pc {
            0x82607FE0 => {
    //   block [0x82607FE0..0x82608050)
	// 82607FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607FEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607FF0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607FF4: 392AD65C  addi r9, r10, -0x29a4
	ctx.r[9].s64 = ctx.r[10].s64 + -10660;
	// 82607FF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607FFC: 390B3470  addi r8, r11, 0x3470
	ctx.r[8].s64 = ctx.r[11].s64 + 13424;
	// 82608000: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82608004: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 82608008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260800C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608018: 386A58F4  addi r3, r10, 0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22772;
	// 8260801C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608020: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260802C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260803C: 4BE5EDE5  bl 0x82466e20
	ctx.lr = 0x82608040;
	sub_82466E20(ctx, base);
	// 82608040: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260804C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608050 size=112
    let mut pc: u32 = 0x82608050;
    'dispatch: loop {
        match pc {
            0x82608050 => {
    //   block [0x82608050..0x826080C0)
	// 82608050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260805C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608060: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608064: 392AD698  addi r9, r10, -0x2968
	ctx.r[9].s64 = ctx.r[10].s64 + -10600;
	// 82608068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260806C: 390BDE80  addi r8, r11, -0x2180
	ctx.r[8].s64 = ctx.r[11].s64 + -8576;
	// 82608070: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82608074: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82608078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260807C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608088: 386A5924  addi r3, r10, 0x5924
	ctx.r[3].s64 = ctx.r[10].s64 + 22820;
	// 8260808C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608090: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82608094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260809C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826080A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826080A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826080A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826080AC: 4BE5ED75  bl 0x82466e20
	ctx.lr = 0x826080B0;
	sub_82466E20(ctx, base);
	// 826080B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826080B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826080B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826080BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826080C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826080C0 size=108
    let mut pc: u32 = 0x826080C0;
    'dispatch: loop {
        match pc {
            0x826080C0 => {
    //   block [0x826080C0..0x8260812C)
	// 826080C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826080C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826080C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826080CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826080D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826080D4: 38EBDEC8  addi r7, r11, -0x2138
	ctx.r[7].s64 = ctx.r[11].s64 + -8504;
	// 826080D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826080DC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826080E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826080E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826080E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826080EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826080F0: 386A5954  addi r3, r10, 0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + 22868;
	// 826080F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826080F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826080FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260810C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608118: 4BE5ED09  bl 0x82466e20
	ctx.lr = 0x8260811C;
	sub_82466E20(ctx, base);
	// 8260811C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608130 size=108
    let mut pc: u32 = 0x82608130;
    'dispatch: loop {
        match pc {
            0x82608130 => {
    //   block [0x82608130..0x8260819C)
	// 82608130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260813C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608140: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82608144: 38EBDEF8  addi r7, r11, -0x2108
	ctx.r[7].s64 = ctx.r[11].s64 + -8456;
	// 82608148: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260814C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82608150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608154: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608158: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260815C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608160: 386A5984  addi r3, r10, 0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + 22916;
	// 82608164: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260816C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260817C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608188: 4BE5EC99  bl 0x82466e20
	ctx.lr = 0x8260818C;
	sub_82466E20(ctx, base);
	// 8260818C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826081A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826081A0 size=112
    let mut pc: u32 = 0x826081A0;
    'dispatch: loop {
        match pc {
            0x826081A0 => {
    //   block [0x826081A0..0x82608210)
	// 826081A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826081A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826081A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826081AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826081B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826081B4: 392AD6D0  addi r9, r10, -0x2930
	ctx.r[9].s64 = ctx.r[10].s64 + -10544;
	// 826081B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826081BC: 390BDF28  addi r8, r11, -0x20d8
	ctx.r[8].s64 = ctx.r[11].s64 + -8408;
	// 826081C0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826081C4: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826081C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826081CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826081D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826081D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826081D8: 386A59B4  addi r3, r10, 0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22964;
	// 826081DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826081E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826081E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826081E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826081EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826081F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826081F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826081F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826081FC: 4BE5EC25  bl 0x82466e20
	ctx.lr = 0x82608200;
	sub_82466E20(ctx, base);
	// 82608200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260820C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608210 size=108
    let mut pc: u32 = 0x82608210;
    'dispatch: loop {
        match pc {
            0x82608210 => {
    //   block [0x82608210..0x8260827C)
	// 82608210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260821C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608220: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82608224: 38EBDF88  addi r7, r11, -0x2078
	ctx.r[7].s64 = ctx.r[11].s64 + -8312;
	// 82608228: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8260822C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82608230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260823C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608240: 386A59E4  addi r3, r10, 0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + 23012;
	// 82608244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260824C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260825C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608268: 4BE5EBB9  bl 0x82466e20
	ctx.lr = 0x8260826C;
	sub_82466E20(ctx, base);
	// 8260826C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608280 size=108
    let mut pc: u32 = 0x82608280;
    'dispatch: loop {
        match pc {
            0x82608280 => {
    //   block [0x82608280..0x826082EC)
	// 82608280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260828C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608290: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82608294: 38EBE090  addi r7, r11, -0x1f70
	ctx.r[7].s64 = ctx.r[11].s64 + -8048;
	// 82608298: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260829C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826082A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826082A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826082A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826082AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826082B0: 386A5A14  addi r3, r10, 0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + 23060;
	// 826082B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826082B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826082BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826082C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826082C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826082C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826082CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826082D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826082D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826082D8: 4BE5EB49  bl 0x82466e20
	ctx.lr = 0x826082DC;
	sub_82466E20(ctx, base);
	// 826082DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826082E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826082E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826082E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826082F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826082F0 size=108
    let mut pc: u32 = 0x826082F0;
    'dispatch: loop {
        match pc {
            0x826082F0 => {
    //   block [0x826082F0..0x8260835C)
	// 826082F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826082F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826082F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826082FC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608304: 38EBE0A8  addi r7, r11, -0x1f58
	ctx.r[7].s64 = ctx.r[11].s64 + -8024;
	// 82608308: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8260830C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82608310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608314: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260831C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608320: 386A5A44  addi r3, r10, 0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + 23108;
	// 82608324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260832C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260833C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608348: 4BE5EAD9  bl 0x82466e20
	ctx.lr = 0x8260834C;
	sub_82466E20(ctx, base);
	// 8260834C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608360 size=24
    let mut pc: u32 = 0x82608360;
    'dispatch: loop {
        match pc {
            0x82608360 => {
    //   block [0x82608360..0x82608378)
	// 82608360: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608364: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608368: 394A3548  addi r10, r10, 0x3548
	ctx.r[10].s64 = ctx.r[10].s64 + 13640;
	// 8260836C: 816BE138  lwz r11, -0x1ec8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7880 as u32) ) } as u64;
	// 82608370: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82608374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608378 size=108
    let mut pc: u32 = 0x82608378;
    'dispatch: loop {
        match pc {
            0x82608378 => {
    //   block [0x82608378..0x826083E4)
	// 82608378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608384: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260838C: 38EB3548  addi r7, r11, 0x3548
	ctx.r[7].s64 = ctx.r[11].s64 + 13640;
	// 82608390: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608394: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82608398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260839C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826083A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826083A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826083A8: 386A5A74  addi r3, r10, 0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + 23156;
	// 826083AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826083B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826083B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826083B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826083BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826083C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826083C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826083C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826083CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826083D0: 4BE5EA51  bl 0x82466e20
	ctx.lr = 0x826083D4;
	sub_82466E20(ctx, base);
	// 826083D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826083D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826083DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826083E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826083E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826083E8 size=24
    let mut pc: u32 = 0x826083E8;
    'dispatch: loop {
        match pc {
            0x826083E8 => {
    //   block [0x826083E8..0x82608400)
	// 826083E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826083EC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826083F0: 394A3578  addi r10, r10, 0x3578
	ctx.r[10].s64 = ctx.r[10].s64 + 13688;
	// 826083F4: 816BE138  lwz r11, -0x1ec8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7880 as u32) ) } as u64;
	// 826083F8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826083FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608400 size=108
    let mut pc: u32 = 0x82608400;
    'dispatch: loop {
        match pc {
            0x82608400 => {
    //   block [0x82608400..0x8260846C)
	// 82608400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260840C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608414: 38EB3578  addi r7, r11, 0x3578
	ctx.r[7].s64 = ctx.r[11].s64 + 13688;
	// 82608418: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260841C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82608420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608424: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260842C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608430: 386A5AA4  addi r3, r10, 0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 23204;
	// 82608434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260843C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260844C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608458: 4BE5E9C9  bl 0x82466e20
	ctx.lr = 0x8260845C;
	sub_82466E20(ctx, base);
	// 8260845C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608470 size=108
    let mut pc: u32 = 0x82608470;
    'dispatch: loop {
        match pc {
            0x82608470 => {
    //   block [0x82608470..0x826084DC)
	// 82608470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260847C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608484: 38EBE120  addi r7, r11, -0x1ee0
	ctx.r[7].s64 = ctx.r[11].s64 + -7904;
	// 82608488: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260848C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82608490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260849C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826084A0: 386A5AD4  addi r3, r10, 0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 23252;
	// 826084A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826084A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826084AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826084B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826084B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826084B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826084BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826084C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826084C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826084C8: 4BE5E959  bl 0x82466e20
	ctx.lr = 0x826084CC;
	sub_82466E20(ctx, base);
	// 826084CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826084D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826084D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826084D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826084E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826084E0 size=24
    let mut pc: u32 = 0x826084E0;
    'dispatch: loop {
        match pc {
            0x826084E0 => {
    //   block [0x826084E0..0x826084F8)
	// 826084E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826084E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826084E8: 394A35A8  addi r10, r10, 0x35a8
	ctx.r[10].s64 = ctx.r[10].s64 + 13736;
	// 826084EC: 816BE138  lwz r11, -0x1ec8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7880 as u32) ) } as u64;
	// 826084F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826084F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826084F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826084F8 size=108
    let mut pc: u32 = 0x826084F8;
    'dispatch: loop {
        match pc {
            0x826084F8 => {
    //   block [0x826084F8..0x82608564)
	// 826084F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826084FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608504: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260850C: 38EB35A8  addi r7, r11, 0x35a8
	ctx.r[7].s64 = ctx.r[11].s64 + 13736;
	// 82608510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608514: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82608518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260851C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608528: 386A5B04  addi r3, r10, 0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + 23300;
	// 8260852C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260853C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260854C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608550: 4BE5E8D1  bl 0x82466e20
	ctx.lr = 0x82608554;
	sub_82466E20(ctx, base);
	// 82608554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260855C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608568 size=112
    let mut pc: u32 = 0x82608568;
    'dispatch: loop {
        match pc {
            0x82608568 => {
    //   block [0x82608568..0x826085D8)
	// 82608568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260856C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608574: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608578: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260857C: 392AD714  addi r9, r10, -0x28ec
	ctx.r[9].s64 = ctx.r[10].s64 + -10476;
	// 82608580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608584: 390BE13C  addi r8, r11, -0x1ec4
	ctx.r[8].s64 = ctx.r[11].s64 + -7876;
	// 82608588: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8260858C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82608590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260859C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826085A0: 386A5B34  addi r3, r10, 0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + 23348;
	// 826085A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826085A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826085AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826085B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826085B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826085B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826085BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826085C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826085C4: 4BE5E85D  bl 0x82466e20
	ctx.lr = 0x826085C8;
	sub_82466E20(ctx, base);
	// 826085C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826085CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826085D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826085D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826085D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826085D8 size=108
    let mut pc: u32 = 0x826085D8;
    'dispatch: loop {
        match pc {
            0x826085D8 => {
    //   block [0x826085D8..0x82608644)
	// 826085D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826085DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826085E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826085E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826085E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826085EC: 38EBE16C  addi r7, r11, -0x1e94
	ctx.r[7].s64 = ctx.r[11].s64 + -7828;
	// 826085F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826085F4: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826085F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826085FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608608: 386A5B64  addi r3, r10, 0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + 23396;
	// 8260860C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260861C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260862C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608630: 4BE5E7F1  bl 0x82466e20
	ctx.lr = 0x82608634;
	sub_82466E20(ctx, base);
	// 82608634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260863C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608648 size=108
    let mut pc: u32 = 0x82608648;
    'dispatch: loop {
        match pc {
            0x82608648 => {
    //   block [0x82608648..0x826086B4)
	// 82608648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260864C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608654: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260865C: 38EBE19C  addi r7, r11, -0x1e64
	ctx.r[7].s64 = ctx.r[11].s64 + -7780;
	// 82608660: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82608664: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82608668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260866C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608678: 386A5B94  addi r3, r10, 0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + 23444;
	// 8260867C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260868C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260869C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826086A0: 4BE5E781  bl 0x82466e20
	ctx.lr = 0x826086A4;
	sub_82466E20(ctx, base);
	// 826086A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826086A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826086AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826086B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826086B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826086B8 size=108
    let mut pc: u32 = 0x826086B8;
    'dispatch: loop {
        match pc {
            0x826086B8 => {
    //   block [0x826086B8..0x82608724)
	// 826086B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826086BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826086C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826086C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826086C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826086CC: 38EBE1B4  addi r7, r11, -0x1e4c
	ctx.r[7].s64 = ctx.r[11].s64 + -7756;
	// 826086D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826086D4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826086D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826086DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826086E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826086E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826086E8: 386A5BC4  addi r3, r10, 0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 23492;
	// 826086EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826086F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826086F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826086F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826086FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260870C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608710: 4BE5E711  bl 0x82466e20
	ctx.lr = 0x82608714;
	sub_82466E20(ctx, base);
	// 82608714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260871C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608728 size=112
    let mut pc: u32 = 0x82608728;
    'dispatch: loop {
        match pc {
            0x82608728 => {
    //   block [0x82608728..0x82608798)
	// 82608728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608738: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260873C: 38AA5C24  addi r5, r10, 0x5c24
	ctx.r[5].s64 = ctx.r[10].s64 + 23588;
	// 82608740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608744: 390BE1E4  addi r8, r11, -0x1e1c
	ctx.r[8].s64 = ctx.r[11].s64 + -7708;
	// 82608748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260874C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82608750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608754: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260875C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608760: 386A5BF4  addi r3, r10, 0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 23540;
	// 82608764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82608768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260876C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260877C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608784: 4BE5E69D  bl 0x82466e20
	ctx.lr = 0x82608788;
	sub_82466E20(ctx, base);
	// 82608788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260878C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608798 size=108
    let mut pc: u32 = 0x82608798;
    'dispatch: loop {
        match pc {
            0x82608798 => {
    //   block [0x82608798..0x82608804)
	// 82608798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260879C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826087A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826087A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826087A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826087AC: 38EBE1FC  addi r7, r11, -0x1e04
	ctx.r[7].s64 = ctx.r[11].s64 + -7684;
	// 826087B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826087B4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826087B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826087BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826087C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826087C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826087C8: 386A5C24  addi r3, r10, 0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + 23588;
	// 826087CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826087D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826087D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826087D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826087DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826087E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826087E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826087E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826087EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826087F0: 4BE5E631  bl 0x82466e20
	ctx.lr = 0x826087F4;
	sub_82466E20(ctx, base);
	// 826087F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826087F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826087FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608808 size=108
    let mut pc: u32 = 0x82608808;
    'dispatch: loop {
        match pc {
            0x82608808 => {
    //   block [0x82608808..0x82608874)
	// 82608808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260880C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608814: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260881C: 38EBE22C  addi r7, r11, -0x1dd4
	ctx.r[7].s64 = ctx.r[11].s64 + -7636;
	// 82608820: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82608824: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82608828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260882C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608838: 386A5C54  addi r3, r10, 0x5c54
	ctx.r[3].s64 = ctx.r[10].s64 + 23636;
	// 8260883C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260884C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260885C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608860: 4BE5E5C1  bl 0x82466e20
	ctx.lr = 0x82608864;
	sub_82466E20(ctx, base);
	// 82608864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260886C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608878 size=108
    let mut pc: u32 = 0x82608878;
    'dispatch: loop {
        match pc {
            0x82608878 => {
    //   block [0x82608878..0x826088E4)
	// 82608878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260887C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608884: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260888C: 38EBE244  addi r7, r11, -0x1dbc
	ctx.r[7].s64 = ctx.r[11].s64 + -7612;
	// 82608890: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608894: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82608898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260889C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826088A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826088A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826088A8: 386A5C84  addi r3, r10, 0x5c84
	ctx.r[3].s64 = ctx.r[10].s64 + 23684;
	// 826088AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826088B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826088B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826088B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826088BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826088C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826088C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826088C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826088CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826088D0: 4BE5E551  bl 0x82466e20
	ctx.lr = 0x826088D4;
	sub_82466E20(ctx, base);
	// 826088D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826088D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826088DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826088E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826088E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826088E8 size=108
    let mut pc: u32 = 0x826088E8;
    'dispatch: loop {
        match pc {
            0x826088E8 => {
    //   block [0x826088E8..0x82608954)
	// 826088E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826088EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826088F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826088F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826088F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826088FC: 38EBE278  addi r7, r11, -0x1d88
	ctx.r[7].s64 = ctx.r[11].s64 + -7560;
	// 82608900: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82608904: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82608908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260890C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608918: 386A5CB4  addi r3, r10, 0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 23732;
	// 8260891C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260892C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260893C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608940: 4BE5E4E1  bl 0x82466e20
	ctx.lr = 0x82608944;
	sub_82466E20(ctx, base);
	// 82608944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260894C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608958 size=108
    let mut pc: u32 = 0x82608958;
    'dispatch: loop {
        match pc {
            0x82608958 => {
    //   block [0x82608958..0x826089C4)
	// 82608958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260895C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608964: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260896C: 38EBE320  addi r7, r11, -0x1ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -7392;
	// 82608970: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608974: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82608978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260897C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608988: 386A5CE4  addi r3, r10, 0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 23780;
	// 8260898C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826089A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826089A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826089A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826089AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826089B0: 4BE5E471  bl 0x82466e20
	ctx.lr = 0x826089B4;
	sub_82466E20(ctx, base);
	// 826089B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826089B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826089BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826089C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826089C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826089C8 size=108
    let mut pc: u32 = 0x826089C8;
    'dispatch: loop {
        match pc {
            0x826089C8 => {
    //   block [0x826089C8..0x82608A34)
	// 826089C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826089CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826089D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826089D4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826089D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826089DC: 38EBE350  addi r7, r11, -0x1cb0
	ctx.r[7].s64 = ctx.r[11].s64 + -7344;
	// 826089E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826089E4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826089E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826089EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826089F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826089F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826089F8: 386A5D14  addi r3, r10, 0x5d14
	ctx.r[3].s64 = ctx.r[10].s64 + 23828;
	// 826089FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608A20: 4BE5E401  bl 0x82466e20
	ctx.lr = 0x82608A24;
	sub_82466E20(ctx, base);
	// 82608A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608A38 size=108
    let mut pc: u32 = 0x82608A38;
    'dispatch: loop {
        match pc {
            0x82608A38 => {
    //   block [0x82608A38..0x82608AA4)
	// 82608A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608A44: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608A4C: 38EBE368  addi r7, r11, -0x1c98
	ctx.r[7].s64 = ctx.r[11].s64 + -7320;
	// 82608A50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608A54: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82608A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608A5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608A60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608A68: 386A5D44  addi r3, r10, 0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + 23876;
	// 82608A6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608A90: 4BE5E391  bl 0x82466e20
	ctx.lr = 0x82608A94;
	sub_82466E20(ctx, base);
	// 82608A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608AA8 size=112
    let mut pc: u32 = 0x82608AA8;
    'dispatch: loop {
        match pc {
            0x82608AA8 => {
    //   block [0x82608AA8..0x82608B18)
	// 82608AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608AB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608AB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608ABC: 38AA5B94  addi r5, r10, 0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + 23444;
	// 82608AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608AC4: 390BE398  addi r8, r11, -0x1c68
	ctx.r[8].s64 = ctx.r[11].s64 + -7272;
	// 82608AC8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82608ACC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82608AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608AD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608AE0: 386A5D74  addi r3, r10, 0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + 23924;
	// 82608AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82608AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608B04: 4BE5E31D  bl 0x82466e20
	ctx.lr = 0x82608B08;
	sub_82466E20(ctx, base);
	// 82608B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608B18 size=24
    let mut pc: u32 = 0x82608B18;
    'dispatch: loop {
        match pc {
            0x82608B18 => {
    //   block [0x82608B18..0x82608B30)
	// 82608B18: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608B1C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608B20: 394A35D8  addi r10, r10, 0x35d8
	ctx.r[10].s64 = ctx.r[10].s64 + 13784;
	// 82608B24: 816BE274  lwz r11, -0x1d8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7564 as u32) ) } as u64;
	// 82608B28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82608B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608B30 size=112
    let mut pc: u32 = 0x82608B30;
    'dispatch: loop {
        match pc {
            0x82608B30 => {
    //   block [0x82608B30..0x82608BA0)
	// 82608B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608B3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608B40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608B44: 392AD740  addi r9, r10, -0x28c0
	ctx.r[9].s64 = ctx.r[10].s64 + -10432;
	// 82608B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608B4C: 390B35D8  addi r8, r11, 0x35d8
	ctx.r[8].s64 = ctx.r[11].s64 + 13784;
	// 82608B50: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82608B54: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82608B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608B5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608B68: 386A5DA4  addi r3, r10, 0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + 23972;
	// 82608B6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608B70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608B8C: 4BE5E295  bl 0x82466e20
	ctx.lr = 0x82608B90;
	sub_82466E20(ctx, base);
	// 82608B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608BA0 size=108
    let mut pc: u32 = 0x82608BA0;
    'dispatch: loop {
        match pc {
            0x82608BA0 => {
    //   block [0x82608BA0..0x82608C0C)
	// 82608BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608BAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608BB4: 38EBE444  addi r7, r11, -0x1bbc
	ctx.r[7].s64 = ctx.r[11].s64 + -7100;
	// 82608BB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608BBC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82608BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608BC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608BD0: 386A5DD4  addi r3, r10, 0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 24020;
	// 82608BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608BF8: 4BE5E229  bl 0x82466e20
	ctx.lr = 0x82608BFC;
	sub_82466E20(ctx, base);
	// 82608BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608C10 size=116
    let mut pc: u32 = 0x82608C10;
    'dispatch: loop {
        match pc {
            0x82608C10 => {
    //   block [0x82608C10..0x82608C84)
	// 82608C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608C1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608C20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608C24: 390BE478  addi r8, r11, -0x1b88
	ctx.r[8].s64 = ctx.r[11].s64 + -7048;
	// 82608C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608C2C: 392AD784  addi r9, r10, -0x287c
	ctx.r[9].s64 = ctx.r[10].s64 + -10364;
	// 82608C30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608C34: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82608C38: 38AA5B94  addi r5, r10, 0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + 23444;
	// 82608C3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608C44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608C54: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82608C58: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82608C5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608C60: 386B5E04  addi r3, r11, 0x5e04
	ctx.r[3].s64 = ctx.r[11].s64 + 24068;
	// 82608C64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608C68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608C70: 4BE5E1B1  bl 0x82466e20
	ctx.lr = 0x82608C74;
	sub_82466E20(ctx, base);
	// 82608C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608C88 size=24
    let mut pc: u32 = 0x82608C88;
    'dispatch: loop {
        match pc {
            0x82608C88 => {
    //   block [0x82608C88..0x82608CA0)
	// 82608C88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608C8C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608C90: 394A3650  addi r10, r10, 0x3650
	ctx.r[10].s64 = ctx.r[10].s64 + 13904;
	// 82608C94: 816BE474  lwz r11, -0x1b8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7052 as u32) ) } as u64;
	// 82608C98: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82608C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608CA0 size=112
    let mut pc: u32 = 0x82608CA0;
    'dispatch: loop {
        match pc {
            0x82608CA0 => {
    //   block [0x82608CA0..0x82608D10)
	// 82608CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608CAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608CB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608CB4: 392AD7C0  addi r9, r10, -0x2840
	ctx.r[9].s64 = ctx.r[10].s64 + -10304;
	// 82608CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608CBC: 390B3650  addi r8, r11, 0x3650
	ctx.r[8].s64 = ctx.r[11].s64 + 13904;
	// 82608CC0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82608CC4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82608CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608CCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608CD8: 386A5E34  addi r3, r10, 0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + 24116;
	// 82608CDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608CE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608CF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608CFC: 4BE5E125  bl 0x82466e20
	ctx.lr = 0x82608D00;
	sub_82466E20(ctx, base);
	// 82608D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608D10 size=108
    let mut pc: u32 = 0x82608D10;
    'dispatch: loop {
        match pc {
            0x82608D10 => {
    //   block [0x82608D10..0x82608D7C)
	// 82608D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608D1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608D24: 38EBE538  addi r7, r11, -0x1ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -6856;
	// 82608D28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82608D2C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82608D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608D34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608D38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608D40: 386A5E64  addi r3, r10, 0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + 24164;
	// 82608D44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608D64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608D68: 4BE5E0B9  bl 0x82466e20
	ctx.lr = 0x82608D6C;
	sub_82466E20(ctx, base);
	// 82608D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608D80 size=108
    let mut pc: u32 = 0x82608D80;
    'dispatch: loop {
        match pc {
            0x82608D80 => {
    //   block [0x82608D80..0x82608DEC)
	// 82608D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608D8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608D94: 38EBE550  addi r7, r11, -0x1ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -6832;
	// 82608D98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608D9C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82608DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608DA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608DB0: 386A5E94  addi r3, r10, 0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + 24212;
	// 82608DB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608DD8: 4BE5E049  bl 0x82466e20
	ctx.lr = 0x82608DDC;
	sub_82466E20(ctx, base);
	// 82608DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608DF0 size=24
    let mut pc: u32 = 0x82608DF0;
    'dispatch: loop {
        match pc {
            0x82608DF0 => {
    //   block [0x82608DF0..0x82608E08)
	// 82608DF0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608DF4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608DF8: 394A3698  addi r10, r10, 0x3698
	ctx.r[10].s64 = ctx.r[10].s64 + 13976;
	// 82608DFC: 816BE580  lwz r11, -0x1a80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6784 as u32) ) } as u64;
	// 82608E00: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82608E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608E08 size=112
    let mut pc: u32 = 0x82608E08;
    'dispatch: loop {
        match pc {
            0x82608E08 => {
    //   block [0x82608E08..0x82608E78)
	// 82608E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608E14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608E18: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608E1C: 392AD7FC  addi r9, r10, -0x2804
	ctx.r[9].s64 = ctx.r[10].s64 + -10244;
	// 82608E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608E24: 390B3698  addi r8, r11, 0x3698
	ctx.r[8].s64 = ctx.r[11].s64 + 13976;
	// 82608E28: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82608E2C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82608E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608E34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608E40: 386A5EC4  addi r3, r10, 0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 24260;
	// 82608E44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608E48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608E64: 4BE5DFBD  bl 0x82466e20
	ctx.lr = 0x82608E68;
	sub_82466E20(ctx, base);
	// 82608E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608E78 size=112
    let mut pc: u32 = 0x82608E78;
    'dispatch: loop {
        match pc {
            0x82608E78 => {
    //   block [0x82608E78..0x82608EE8)
	// 82608E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608E84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608E88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608E8C: 38AA5B94  addi r5, r10, 0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + 23444;
	// 82608E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608E94: 390BE584  addi r8, r11, -0x1a7c
	ctx.r[8].s64 = ctx.r[11].s64 + -6780;
	// 82608E98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82608E9C: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82608EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608EA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608EB0: 386A5EF4  addi r3, r10, 0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 24308;
	// 82608EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82608EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608ED4: 4BE5DF4D  bl 0x82466e20
	ctx.lr = 0x82608ED8;
	sub_82466E20(ctx, base);
	// 82608ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608EE8 size=108
    let mut pc: u32 = 0x82608EE8;
    'dispatch: loop {
        match pc {
            0x82608EE8 => {
    //   block [0x82608EE8..0x82608F54)
	// 82608EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608EF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608EFC: 38EBE5B4  addi r7, r11, -0x1a4c
	ctx.r[7].s64 = ctx.r[11].s64 + -6732;
	// 82608F00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608F04: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82608F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608F0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608F18: 386A5F24  addi r3, r10, 0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + 24356;
	// 82608F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608F40: 4BE5DEE1  bl 0x82466e20
	ctx.lr = 0x82608F44;
	sub_82466E20(ctx, base);
	// 82608F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608F58 size=108
    let mut pc: u32 = 0x82608F58;
    'dispatch: loop {
        match pc {
            0x82608F58 => {
    //   block [0x82608F58..0x82608FC4)
	// 82608F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608F64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608F6C: 38EBE5E8  addi r7, r11, -0x1a18
	ctx.r[7].s64 = ctx.r[11].s64 + -6680;
	// 82608F70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82608F74: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82608F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608F88: 386A5F54  addi r3, r10, 0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + 24404;
	// 82608F8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608FAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608FB0: 4BE5DE71  bl 0x82466e20
	ctx.lr = 0x82608FB4;
	sub_82466E20(ctx, base);
	// 82608FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608FC8 size=108
    let mut pc: u32 = 0x82608FC8;
    'dispatch: loop {
        match pc {
            0x82608FC8 => {
    //   block [0x82608FC8..0x82609034)
	// 82608FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608FD4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608FDC: 38EBE648  addi r7, r11, -0x19b8
	ctx.r[7].s64 = ctx.r[11].s64 + -6584;
	// 82608FE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608FE4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82608FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608FEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608FF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608FF8: 386A5F84  addi r3, r10, 0x5f84
	ctx.r[3].s64 = ctx.r[10].s64 + 24452;
	// 82608FFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260900C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260901C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609020: 4BE5DE01  bl 0x82466e20
	ctx.lr = 0x82609024;
	sub_82466E20(ctx, base);
	// 82609024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260902C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609038 size=108
    let mut pc: u32 = 0x82609038;
    'dispatch: loop {
        match pc {
            0x82609038 => {
    //   block [0x82609038..0x826090A4)
	// 82609038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260903C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609044: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260904C: 38EBE678  addi r7, r11, -0x1988
	ctx.r[7].s64 = ctx.r[11].s64 + -6536;
	// 82609050: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82609054: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82609058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260905C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609060: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609068: 386A5FB4  addi r3, r10, 0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 24500;
	// 8260906C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260907C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260908C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609090: 4BE5DD91  bl 0x82466e20
	ctx.lr = 0x82609094;
	sub_82466E20(ctx, base);
	// 82609094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260909C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826090A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826090A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826090A8 size=108
    let mut pc: u32 = 0x826090A8;
    'dispatch: loop {
        match pc {
            0x826090A8 => {
    //   block [0x826090A8..0x82609114)
	// 826090A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826090AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826090B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826090B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826090B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826090BC: 38EBE798  addi r7, r11, -0x1868
	ctx.r[7].s64 = ctx.r[11].s64 + -6248;
	// 826090C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826090C4: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826090C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826090CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826090D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826090D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826090D8: 386A5FE4  addi r3, r10, 0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 24548;
	// 826090DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826090E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826090E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826090E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826090EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826090F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826090F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826090F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826090FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609100: 4BE5DD21  bl 0x82466e20
	ctx.lr = 0x82609104;
	sub_82466E20(ctx, base);
	// 82609104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260910C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609118 size=108
    let mut pc: u32 = 0x82609118;
    'dispatch: loop {
        match pc {
            0x82609118 => {
    //   block [0x82609118..0x82609184)
	// 82609118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260911C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609124: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260912C: 38EBE7B0  addi r7, r11, -0x1850
	ctx.r[7].s64 = ctx.r[11].s64 + -6224;
	// 82609130: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609134: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82609138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260913C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609148: 386A6014  addi r3, r10, 0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + 24596;
	// 8260914C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260915C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260916C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609170: 4BE5DCB1  bl 0x82466e20
	ctx.lr = 0x82609174;
	sub_82466E20(ctx, base);
	// 82609174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260917C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609188 size=108
    let mut pc: u32 = 0x82609188;
    'dispatch: loop {
        match pc {
            0x82609188 => {
    //   block [0x82609188..0x826091F4)
	// 82609188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260918C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609194: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260919C: 38EBE7C8  addi r7, r11, -0x1838
	ctx.r[7].s64 = ctx.r[11].s64 + -6200;
	// 826091A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826091A4: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826091A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826091AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826091B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826091B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826091B8: 386A6044  addi r3, r10, 0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + 24644;
	// 826091BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826091C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826091C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826091C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826091CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826091D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826091D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826091D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826091DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826091E0: 4BE5DC41  bl 0x82466e20
	ctx.lr = 0x826091E4;
	sub_82466E20(ctx, base);
	// 826091E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826091E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826091EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826091F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826091F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826091F8 size=108
    let mut pc: u32 = 0x826091F8;
    'dispatch: loop {
        match pc {
            0x826091F8 => {
    //   block [0x826091F8..0x82609264)
	// 826091F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826091FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609204: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260920C: 38EBE7E0  addi r7, r11, -0x1820
	ctx.r[7].s64 = ctx.r[11].s64 + -6176;
	// 82609210: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609214: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82609218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260921C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609228: 386A6074  addi r3, r10, 0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + 24692;
	// 8260922C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260923C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260924C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609250: 4BE5DBD1  bl 0x82466e20
	ctx.lr = 0x82609254;
	sub_82466E20(ctx, base);
	// 82609254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260925C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609268 size=108
    let mut pc: u32 = 0x82609268;
    'dispatch: loop {
        match pc {
            0x82609268 => {
    //   block [0x82609268..0x826092D4)
	// 82609268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609274: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260927C: 38EBE7F8  addi r7, r11, -0x1808
	ctx.r[7].s64 = ctx.r[11].s64 + -6152;
	// 82609280: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609284: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 82609288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260928C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609298: 386A60A4  addi r3, r10, 0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + 24740;
	// 8260929C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826092A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826092A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826092A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826092AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826092B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826092B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826092B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826092BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826092C0: 4BE5DB61  bl 0x82466e20
	ctx.lr = 0x826092C4;
	sub_82466E20(ctx, base);
	// 826092C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826092C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826092CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826092D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826092D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826092D8 size=108
    let mut pc: u32 = 0x826092D8;
    'dispatch: loop {
        match pc {
            0x826092D8 => {
    //   block [0x826092D8..0x82609344)
	// 826092D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826092DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826092E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826092E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826092E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826092EC: 38EBE810  addi r7, r11, -0x17f0
	ctx.r[7].s64 = ctx.r[11].s64 + -6128;
	// 826092F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826092F4: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826092F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826092FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609308: 386A60D4  addi r3, r10, 0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + 24788;
	// 8260930C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260931C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260932C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609330: 4BE5DAF1  bl 0x82466e20
	ctx.lr = 0x82609334;
	sub_82466E20(ctx, base);
	// 82609334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260933C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


