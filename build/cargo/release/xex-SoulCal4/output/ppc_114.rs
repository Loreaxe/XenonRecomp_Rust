pub fn sub_82690CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690CB8 size=108
    let mut pc: u32 = 0x82690CB8;
    'dispatch: loop {
        match pc {
            0x82690CB8 => {
    //   block [0x82690CB8..0x82690D24)
	// 82690CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690CC4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690CCC: 38EB8008  addi r7, r11, -0x7ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -32760;
	// 82690CD0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82690CD4: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82690CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690CE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82690CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690CE8: 386AF6A8  addi r3, r10, -0x958
	ctx.r[3].s64 = ctx.r[10].s64 + -2392;
	// 82690CEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690CF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690D0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690D10: 4BDD6111  bl 0x82466e20
	ctx.lr = 0x82690D14;
	sub_82466E20(ctx, base);
	// 82690D14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690D28 size=112
    let mut pc: u32 = 0x82690D28;
    'dispatch: loop {
        match pc {
            0x82690D28 => {
    //   block [0x82690D28..0x82690D98)
	// 82690D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690D34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690D38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690D3C: 38AAF528  addi r5, r10, -0xad8
	ctx.r[5].s64 = ctx.r[10].s64 + -2776;
	// 82690D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690D44: 390B8098  addi r8, r11, -0x7f68
	ctx.r[8].s64 = ctx.r[11].s64 + -32616;
	// 82690D48: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82690D4C: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 82690D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690D60: 386AF6D8  addi r3, r10, -0x928
	ctx.r[3].s64 = ctx.r[10].s64 + -2344;
	// 82690D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690D84: 4BDD609D  bl 0x82466e20
	ctx.lr = 0x82690D88;
	sub_82466E20(ctx, base);
	// 82690D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690D98 size=108
    let mut pc: u32 = 0x82690D98;
    'dispatch: loop {
        match pc {
            0x82690D98 => {
    //   block [0x82690D98..0x82690E04)
	// 82690D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690DA4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690DAC: 38EB8110  addi r7, r11, -0x7ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -32496;
	// 82690DB0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82690DB4: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82690DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690DC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82690DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690DC8: 386AF708  addi r3, r10, -0x8f8
	ctx.r[3].s64 = ctx.r[10].s64 + -2296;
	// 82690DCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690DF0: 4BDD6031  bl 0x82466e20
	ctx.lr = 0x82690DF4;
	sub_82466E20(ctx, base);
	// 82690DF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690E08 size=112
    let mut pc: u32 = 0x82690E08;
    'dispatch: loop {
        match pc {
            0x82690E08 => {
    //   block [0x82690E08..0x82690E78)
	// 82690E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690E14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690E18: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690E1C: 38AAF528  addi r5, r10, -0xad8
	ctx.r[5].s64 = ctx.r[10].s64 + -2776;
	// 82690E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690E24: 390B8158  addi r8, r11, -0x7ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -32424;
	// 82690E28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82690E2C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82690E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690E40: 386AF738  addi r3, r10, -0x8c8
	ctx.r[3].s64 = ctx.r[10].s64 + -2248;
	// 82690E44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690E64: 4BDD5FBD  bl 0x82466e20
	ctx.lr = 0x82690E68;
	sub_82466E20(ctx, base);
	// 82690E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690E78 size=112
    let mut pc: u32 = 0x82690E78;
    'dispatch: loop {
        match pc {
            0x82690E78 => {
    //   block [0x82690E78..0x82690EE8)
	// 82690E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690E84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690E88: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690E8C: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690E94: 390B81B8  addi r8, r11, -0x7e48
	ctx.r[8].s64 = ctx.r[11].s64 + -32328;
	// 82690E98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82690E9C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82690EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690EA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690EB0: 386AF768  addi r3, r10, -0x898
	ctx.r[3].s64 = ctx.r[10].s64 + -2200;
	// 82690EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690ED4: 4BDD5F4D  bl 0x82466e20
	ctx.lr = 0x82690ED8;
	sub_82466E20(ctx, base);
	// 82690ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690EE8 size=108
    let mut pc: u32 = 0x82690EE8;
    'dispatch: loop {
        match pc {
            0x82690EE8 => {
    //   block [0x82690EE8..0x82690F54)
	// 82690EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690EF4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690EFC: 38EB81D0  addi r7, r11, -0x7e30
	ctx.r[7].s64 = ctx.r[11].s64 + -32304;
	// 82690F00: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82690F04: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82690F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690F0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82690F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690F18: 386AF798  addi r3, r10, -0x868
	ctx.r[3].s64 = ctx.r[10].s64 + -2152;
	// 82690F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690F40: 4BDD5EE1  bl 0x82466e20
	ctx.lr = 0x82690F44;
	sub_82466E20(ctx, base);
	// 82690F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690F58 size=112
    let mut pc: u32 = 0x82690F58;
    'dispatch: loop {
        match pc {
            0x82690F58 => {
    //   block [0x82690F58..0x82690FC8)
	// 82690F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690F64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690F68: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690F6C: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690F74: 390B8248  addi r8, r11, -0x7db8
	ctx.r[8].s64 = ctx.r[11].s64 + -32184;
	// 82690F78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82690F7C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82690F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690F90: 386AF7C8  addi r3, r10, -0x838
	ctx.r[3].s64 = ctx.r[10].s64 + -2104;
	// 82690F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690FB4: 4BDD5E6D  bl 0x82466e20
	ctx.lr = 0x82690FB8;
	sub_82466E20(ctx, base);
	// 82690FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690FC8 size=104
    let mut pc: u32 = 0x82690FC8;
    'dispatch: loop {
        match pc {
            0x82690FC8 => {
    //   block [0x82690FC8..0x82691030)
	// 82690FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690FD4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82690FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690FDC: 392A6EC8  addi r9, r10, 0x6ec8
	ctx.r[9].s64 = ctx.r[10].s64 + 28360;
	// 82690FE0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690FE8: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82690FEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690FFC: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82691000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691004: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691008: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269100C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691010: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82691014: 386AF7F8  addi r3, r10, -0x808
	ctx.r[3].s64 = ctx.r[10].s64 + -2056;
	// 82691018: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269101C: 4BDD5E05  bl 0x82466e20
	ctx.lr = 0x82691020;
	sub_82466E20(ctx, base);
	// 82691020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269102C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691030 size=112
    let mut pc: u32 = 0x82691030;
    'dispatch: loop {
        match pc {
            0x82691030 => {
    //   block [0x82691030..0x826910A0)
	// 82691030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269103C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691040: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691044: 38AAF7F8  addi r5, r10, -0x808
	ctx.r[5].s64 = ctx.r[10].s64 + -2056;
	// 82691048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269104C: 390B8278  addi r8, r11, -0x7d88
	ctx.r[8].s64 = ctx.r[11].s64 + -32136;
	// 82691050: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82691054: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82691058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269105C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691068: 386AF828  addi r3, r10, -0x7d8
	ctx.r[3].s64 = ctx.r[10].s64 + -2008;
	// 8269106C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269107C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269108C: 4BDD5D95  bl 0x82466e20
	ctx.lr = 0x82691090;
	sub_82466E20(ctx, base);
	// 82691090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269109C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826910A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826910A0 size=112
    let mut pc: u32 = 0x826910A0;
    'dispatch: loop {
        match pc {
            0x826910A0 => {
    //   block [0x826910A0..0x82691110)
	// 826910A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826910A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826910A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826910AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826910B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826910B4: 38AAF828  addi r5, r10, -0x7d8
	ctx.r[5].s64 = ctx.r[10].s64 + -2008;
	// 826910B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826910BC: 390B82A8  addi r8, r11, -0x7d58
	ctx.r[8].s64 = ctx.r[11].s64 + -32088;
	// 826910C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826910C4: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826910C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826910CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826910D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826910D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826910D8: 386AF858  addi r3, r10, -0x7a8
	ctx.r[3].s64 = ctx.r[10].s64 + -1960;
	// 826910DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826910E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826910E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826910E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826910EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826910F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826910F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826910F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826910FC: 4BDD5D25  bl 0x82466e20
	ctx.lr = 0x82691100;
	sub_82466E20(ctx, base);
	// 82691100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269110C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691110 size=112
    let mut pc: u32 = 0x82691110;
    'dispatch: loop {
        match pc {
            0x82691110 => {
    //   block [0x82691110..0x82691180)
	// 82691110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269111C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691120: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691124: 38AAF828  addi r5, r10, -0x7d8
	ctx.r[5].s64 = ctx.r[10].s64 + -2008;
	// 82691128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269112C: 390B8308  addi r8, r11, -0x7cf8
	ctx.r[8].s64 = ctx.r[11].s64 + -31992;
	// 82691130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82691134: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82691138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269113C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691148: 386AF888  addi r3, r10, -0x778
	ctx.r[3].s64 = ctx.r[10].s64 + -1912;
	// 8269114C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269115C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269116C: 4BDD5CB5  bl 0x82466e20
	ctx.lr = 0x82691170;
	sub_82466E20(ctx, base);
	// 82691170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269117C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691180 size=112
    let mut pc: u32 = 0x82691180;
    'dispatch: loop {
        match pc {
            0x82691180 => {
    //   block [0x82691180..0x826911F0)
	// 82691180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269118C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691190: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691194: 38AAF828  addi r5, r10, -0x7d8
	ctx.r[5].s64 = ctx.r[10].s64 + -2008;
	// 82691198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269119C: 390B8338  addi r8, r11, -0x7cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -31944;
	// 826911A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826911A4: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826911A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826911AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826911B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826911B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826911B8: 386AF8B8  addi r3, r10, -0x748
	ctx.r[3].s64 = ctx.r[10].s64 + -1864;
	// 826911BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826911C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826911C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826911C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826911CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826911D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826911D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826911D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826911DC: 4BDD5C45  bl 0x82466e20
	ctx.lr = 0x826911E0;
	sub_82466E20(ctx, base);
	// 826911E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826911E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826911E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826911EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826911F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826911F0 size=112
    let mut pc: u32 = 0x826911F0;
    'dispatch: loop {
        match pc {
            0x826911F0 => {
    //   block [0x826911F0..0x82691260)
	// 826911F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826911F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826911F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826911FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691200: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691204: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82691208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269120C: 390B8380  addi r8, r11, -0x7c80
	ctx.r[8].s64 = ctx.r[11].s64 + -31872;
	// 82691210: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82691214: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82691218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269121C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691228: 386AF8E8  addi r3, r10, -0x718
	ctx.r[3].s64 = ctx.r[10].s64 + -1816;
	// 8269122C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269123C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269124C: 4BDD5BD5  bl 0x82466e20
	ctx.lr = 0x82691250;
	sub_82466E20(ctx, base);
	// 82691250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269125C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691260 size=116
    let mut pc: u32 = 0x82691260;
    'dispatch: loop {
        match pc {
            0x82691260 => {
    //   block [0x82691260..0x826912D4)
	// 82691260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269126C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82691270: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 82691274: 390A8410  addi r8, r10, -0x7bf0
	ctx.r[8].s64 = ctx.r[10].s64 + -31728;
	// 82691278: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269127C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82691280: 38AAFD98  addi r5, r10, -0x268
	ctx.r[5].s64 = ctx.r[10].s64 + -616;
	// 82691284: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691288: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269128C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691294: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82691298: 396B6EE0  addi r11, r11, 0x6ee0
	ctx.r[11].s64 = ctx.r[11].s64 + 28384;
	// 8269129C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826912A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826912A4: 386AF918  addi r3, r10, -0x6e8
	ctx.r[3].s64 = ctx.r[10].s64 + -1768;
	// 826912A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826912AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826912B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826912B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826912B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826912BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826912C0: 4BDD5B61  bl 0x82466e20
	ctx.lr = 0x826912C4;
	sub_82466E20(ctx, base);
	// 826912C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826912C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826912CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826912D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826912D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826912D8 size=100
    let mut pc: u32 = 0x826912D8;
    'dispatch: loop {
        match pc {
            0x826912D8 => {
    //   block [0x826912D8..0x8269133C)
	// 826912D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826912DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826912E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826912E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826912E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826912EC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826912F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826912F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826912F8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826912FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269130C: 386AF948  addi r3, r10, -0x6b8
	ctx.r[3].s64 = ctx.r[10].s64 + -1720;
	// 82691310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691314: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691318: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269131C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691320: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82691324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691328: 4BDD5AF9  bl 0x82466e20
	ctx.lr = 0x8269132C;
	sub_82466E20(ctx, base);
	// 8269132C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691340 size=100
    let mut pc: u32 = 0x82691340;
    'dispatch: loop {
        match pc {
            0x82691340 => {
    //   block [0x82691340..0x826913A4)
	// 82691340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269134C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691354: 38AAF9D8  addi r5, r10, -0x628
	ctx.r[5].s64 = ctx.r[10].s64 + -1576;
	// 82691358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269135C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691360: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82691364: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269136C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691374: 386AF978  addi r3, r10, -0x688
	ctx.r[3].s64 = ctx.r[10].s64 + -1672;
	// 82691378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269137C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691380: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82691384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691388: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269138C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691390: 4BDD5A91  bl 0x82466e20
	ctx.lr = 0x82691394;
	sub_82466E20(ctx, base);
	// 82691394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269139C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826913A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826913A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826913A8 size=100
    let mut pc: u32 = 0x826913A8;
    'dispatch: loop {
        match pc {
            0x826913A8 => {
    //   block [0x826913A8..0x8269140C)
	// 826913A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826913AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826913B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826913B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826913B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826913BC: 38AAF918  addi r5, r10, -0x6e8
	ctx.r[5].s64 = ctx.r[10].s64 + -1768;
	// 826913C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826913C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826913C8: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826913CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826913D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826913D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826913D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826913DC: 386AF9A8  addi r3, r10, -0x658
	ctx.r[3].s64 = ctx.r[10].s64 + -1624;
	// 826913E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826913E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826913E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826913EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826913F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826913F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826913F8: 4BDD5A29  bl 0x82466e20
	ctx.lr = 0x826913FC;
	sub_82466E20(ctx, base);
	// 826913FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691410 size=104
    let mut pc: u32 = 0x82691410;
    'dispatch: loop {
        match pc {
            0x82691410 => {
    //   block [0x82691410..0x82691478)
	// 82691410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269141C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82691420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691424: 392A6F4C  addi r9, r10, 0x6f4c
	ctx.r[9].s64 = ctx.r[10].s64 + 28492;
	// 82691428: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269142C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691430: 38AAF948  addi r5, r10, -0x6b8
	ctx.r[5].s64 = ctx.r[10].s64 + -1720;
	// 82691434: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269143C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691444: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82691448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269144C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691450: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82691454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269145C: 386AF9D8  addi r3, r10, -0x628
	ctx.r[3].s64 = ctx.r[10].s64 + -1576;
	// 82691460: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82691464: 4BDD59BD  bl 0x82466e20
	ctx.lr = 0x82691468;
	sub_82466E20(ctx, base);
	// 82691468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269146C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691478 size=108
    let mut pc: u32 = 0x82691478;
    'dispatch: loop {
        match pc {
            0x82691478 => {
    //   block [0x82691478..0x826914E4)
	// 82691478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691484: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269148C: 38EB85C4  addi r7, r11, -0x7a3c
	ctx.r[7].s64 = ctx.r[11].s64 + -31292;
	// 82691490: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82691494: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82691498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269149C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826914A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826914A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826914A8: 386AFA08  addi r3, r10, -0x5f8
	ctx.r[3].s64 = ctx.r[10].s64 + -1528;
	// 826914AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826914B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826914B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826914B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826914BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826914C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826914C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826914C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826914CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826914D0: 4BDD5951  bl 0x82466e20
	ctx.lr = 0x826914D4;
	sub_82466E20(ctx, base);
	// 826914D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826914D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826914DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826914E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826914E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826914E8 size=112
    let mut pc: u32 = 0x826914E8;
    'dispatch: loop {
        match pc {
            0x826914E8 => {
    //   block [0x826914E8..0x82691558)
	// 826914E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826914EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826914F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826914F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826914F8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826914FC: 38AAF9D8  addi r5, r10, -0x628
	ctx.r[5].s64 = ctx.r[10].s64 + -1576;
	// 82691500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691504: 390B85F8  addi r8, r11, -0x7a08
	ctx.r[8].s64 = ctx.r[11].s64 + -31240;
	// 82691508: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269150C: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82691510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691514: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269151C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691520: 386AFA38  addi r3, r10, -0x5c8
	ctx.r[3].s64 = ctx.r[10].s64 + -1480;
	// 82691524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269152C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269153C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691544: 4BDD58DD  bl 0x82466e20
	ctx.lr = 0x82691548;
	sub_82466E20(ctx, base);
	// 82691548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269154C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691558 size=116
    let mut pc: u32 = 0x82691558;
    'dispatch: loop {
        match pc {
            0x82691558 => {
    //   block [0x82691558..0x826915CC)
	// 82691558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691564: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691568: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8269156C: 390B86A0  addi r8, r11, -0x7960
	ctx.r[8].s64 = ctx.r[11].s64 + -31072;
	// 82691570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691574: 392A6FB0  addi r9, r10, 0x6fb0
	ctx.r[9].s64 = ctx.r[10].s64 + 28592;
	// 82691578: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269157C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82691580: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82691584: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269158C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269159C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826915A0: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826915A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826915A8: 386BFA68  addi r3, r11, -0x598
	ctx.r[3].s64 = ctx.r[11].s64 + -1432;
	// 826915AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826915B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826915B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826915B8: 4BDD5869  bl 0x82466e20
	ctx.lr = 0x826915BC;
	sub_82466E20(ctx, base);
	// 826915BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826915C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826915C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826915C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826915D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826915D0 size=112
    let mut pc: u32 = 0x826915D0;
    'dispatch: loop {
        match pc {
            0x826915D0 => {
    //   block [0x826915D0..0x82691640)
	// 826915D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826915D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826915D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826915DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826915E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826915E4: 38AAFB28  addi r5, r10, -0x4d8
	ctx.r[5].s64 = ctx.r[10].s64 + -1240;
	// 826915E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826915EC: 390B86B8  addi r8, r11, -0x7948
	ctx.r[8].s64 = ctx.r[11].s64 + -31048;
	// 826915F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826915F4: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826915F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826915FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691608: 386AFA98  addi r3, r10, -0x568
	ctx.r[3].s64 = ctx.r[10].s64 + -1384;
	// 8269160C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269161C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269162C: 4BDD57F5  bl 0x82466e20
	ctx.lr = 0x82691630;
	sub_82466E20(ctx, base);
	// 82691630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269163C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691640 size=100
    let mut pc: u32 = 0x82691640;
    'dispatch: loop {
        match pc {
            0x82691640 => {
    //   block [0x82691640..0x826916A4)
	// 82691640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269164C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691654: 38AAFAF8  addi r5, r10, -0x508
	ctx.r[5].s64 = ctx.r[10].s64 + -1288;
	// 82691658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269165C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691660: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82691664: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269166C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691674: 386AFAC8  addi r3, r10, -0x538
	ctx.r[3].s64 = ctx.r[10].s64 + -1336;
	// 82691678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269167C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691680: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82691684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691688: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691690: 4BDD5791  bl 0x82466e20
	ctx.lr = 0x82691694;
	sub_82466E20(ctx, base);
	// 82691694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269169C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826916A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826916A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826916A8 size=112
    let mut pc: u32 = 0x826916A8;
    'dispatch: loop {
        match pc {
            0x826916A8 => {
    //   block [0x826916A8..0x82691718)
	// 826916A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826916AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826916B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826916B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826916B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826916BC: 38AAFB28  addi r5, r10, -0x4d8
	ctx.r[5].s64 = ctx.r[10].s64 + -1240;
	// 826916C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826916C4: 390B86D0  addi r8, r11, -0x7930
	ctx.r[8].s64 = ctx.r[11].s64 + -31024;
	// 826916C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826916CC: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826916D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826916D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826916D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826916DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826916E0: 386AFAF8  addi r3, r10, -0x508
	ctx.r[3].s64 = ctx.r[10].s64 + -1288;
	// 826916E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826916E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826916EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826916F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826916F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826916F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826916FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691704: 4BDD571D  bl 0x82466e20
	ctx.lr = 0x82691708;
	sub_82466E20(ctx, base);
	// 82691708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269170C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691718 size=112
    let mut pc: u32 = 0x82691718;
    'dispatch: loop {
        match pc {
            0x82691718 => {
    //   block [0x82691718..0x82691788)
	// 82691718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691724: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691728: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269172C: 38AAFA68  addi r5, r10, -0x598
	ctx.r[5].s64 = ctx.r[10].s64 + -1432;
	// 82691730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82691734: 390B8700  addi r8, r11, -0x7900
	ctx.r[8].s64 = ctx.r[11].s64 + -30976;
	// 82691738: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269173C: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82691740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691744: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691750: 386AFB28  addi r3, r10, -0x4d8
	ctx.r[3].s64 = ctx.r[10].s64 + -1240;
	// 82691754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269176C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691774: 4BDD56AD  bl 0x82466e20
	ctx.lr = 0x82691778;
	sub_82466E20(ctx, base);
	// 82691778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269177C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691788 size=100
    let mut pc: u32 = 0x82691788;
    'dispatch: loop {
        match pc {
            0x82691788 => {
    //   block [0x82691788..0x826917EC)
	// 82691788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269178C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691794: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269179C: 38AAFB28  addi r5, r10, -0x4d8
	ctx.r[5].s64 = ctx.r[10].s64 + -1240;
	// 826917A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826917A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826917A8: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826917AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826917B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826917B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826917B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826917BC: 386AFB58  addi r3, r10, -0x4a8
	ctx.r[3].s64 = ctx.r[10].s64 + -1192;
	// 826917C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826917C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826917C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826917CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826917D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826917D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826917D8: 4BDD5649  bl 0x82466e20
	ctx.lr = 0x826917DC;
	sub_82466E20(ctx, base);
	// 826917DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826917E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826917E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826917E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826917F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826917F0 size=100
    let mut pc: u32 = 0x826917F0;
    'dispatch: loop {
        match pc {
            0x826917F0 => {
    //   block [0x826917F0..0x82691854)
	// 826917F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826917F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826917F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826917FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691804: 38AAFA98  addi r5, r10, -0x568
	ctx.r[5].s64 = ctx.r[10].s64 + -1384;
	// 82691808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269180C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691810: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82691814: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269181C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691824: 386AFB88  addi r3, r10, -0x478
	ctx.r[3].s64 = ctx.r[10].s64 + -1144;
	// 82691828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269182C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691830: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82691834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691838: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269183C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691840: 4BDD55E1  bl 0x82466e20
	ctx.lr = 0x82691844;
	sub_82466E20(ctx, base);
	// 82691844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269184C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691858 size=100
    let mut pc: u32 = 0x82691858;
    'dispatch: loop {
        match pc {
            0x82691858 => {
    //   block [0x82691858..0x826918BC)
	// 82691858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269185C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691864: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269186C: 38AAFB58  addi r5, r10, -0x4a8
	ctx.r[5].s64 = ctx.r[10].s64 + -1192;
	// 82691870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691878: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8269187C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269188C: 386AFBB8  addi r3, r10, -0x448
	ctx.r[3].s64 = ctx.r[10].s64 + -1096;
	// 82691890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691894: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269189C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826918A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826918A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826918A8: 4BDD5579  bl 0x82466e20
	ctx.lr = 0x826918AC;
	sub_82466E20(ctx, base);
	// 826918AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826918B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826918B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826918B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826918C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826918C0 size=100
    let mut pc: u32 = 0x826918C0;
    'dispatch: loop {
        match pc {
            0x826918C0 => {
    //   block [0x826918C0..0x82691924)
	// 826918C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826918C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826918C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826918CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826918D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826918D4: 38AAFA98  addi r5, r10, -0x568
	ctx.r[5].s64 = ctx.r[10].s64 + -1384;
	// 826918D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826918DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826918E0: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826918E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826918E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826918EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826918F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826918F4: 386AFBE8  addi r3, r10, -0x418
	ctx.r[3].s64 = ctx.r[10].s64 + -1048;
	// 826918F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826918FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691900: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82691904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691908: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269190C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691910: 4BDD5511  bl 0x82466e20
	ctx.lr = 0x82691914;
	sub_82466E20(ctx, base);
	// 82691914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269191C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691928 size=112
    let mut pc: u32 = 0x82691928;
    'dispatch: loop {
        match pc {
            0x82691928 => {
    //   block [0x82691928..0x82691998)
	// 82691928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269192C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691938: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269193C: 38AAFC78  addi r5, r10, -0x388
	ctx.r[5].s64 = ctx.r[10].s64 + -904;
	// 82691940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691944: 390B87A8  addi r8, r11, -0x7858
	ctx.r[8].s64 = ctx.r[11].s64 + -30808;
	// 82691948: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269194C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82691950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269195C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691960: 386AFC18  addi r3, r10, -0x3e8
	ctx.r[3].s64 = ctx.r[10].s64 + -1000;
	// 82691964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269196C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269197C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691984: 4BDD549D  bl 0x82466e20
	ctx.lr = 0x82691988;
	sub_82466E20(ctx, base);
	// 82691988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269198C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691998 size=112
    let mut pc: u32 = 0x82691998;
    'dispatch: loop {
        match pc {
            0x82691998 => {
    //   block [0x82691998..0x82691A08)
	// 82691998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826919A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826919A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826919A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826919AC: 38AAFCA8  addi r5, r10, -0x358
	ctx.r[5].s64 = ctx.r[10].s64 + -856;
	// 826919B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826919B4: 390B87D8  addi r8, r11, -0x7828
	ctx.r[8].s64 = ctx.r[11].s64 + -30760;
	// 826919B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826919BC: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826919C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826919C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826919C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826919CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826919D0: 386AFC48  addi r3, r10, -0x3b8
	ctx.r[3].s64 = ctx.r[10].s64 + -952;
	// 826919D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826919D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826919DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826919E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826919E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826919E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826919EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826919F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826919F4: 4BDD542D  bl 0x82466e20
	ctx.lr = 0x826919F8;
	sub_82466E20(ctx, base);
	// 826919F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826919FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691A08 size=112
    let mut pc: u32 = 0x82691A08;
    'dispatch: loop {
        match pc {
            0x82691A08 => {
    //   block [0x82691A08..0x82691A78)
	// 82691A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691A14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691A18: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691A1C: 38AAFD98  addi r5, r10, -0x268
	ctx.r[5].s64 = ctx.r[10].s64 + -616;
	// 82691A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691A24: 390B87F0  addi r8, r11, -0x7810
	ctx.r[8].s64 = ctx.r[11].s64 + -30736;
	// 82691A28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82691A2C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82691A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691A34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691A40: 386AFC78  addi r3, r10, -0x388
	ctx.r[3].s64 = ctx.r[10].s64 + -904;
	// 82691A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691A64: 4BDD53BD  bl 0x82466e20
	ctx.lr = 0x82691A68;
	sub_82466E20(ctx, base);
	// 82691A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691A78 size=112
    let mut pc: u32 = 0x82691A78;
    'dispatch: loop {
        match pc {
            0x82691A78 => {
    //   block [0x82691A78..0x82691AE8)
	// 82691A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691A84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691A88: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691A8C: 38AAFC78  addi r5, r10, -0x388
	ctx.r[5].s64 = ctx.r[10].s64 + -904;
	// 82691A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691A94: 390B8820  addi r8, r11, -0x77e0
	ctx.r[8].s64 = ctx.r[11].s64 + -30688;
	// 82691A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82691A9C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82691AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691AB0: 386AFCA8  addi r3, r10, -0x358
	ctx.r[3].s64 = ctx.r[10].s64 + -856;
	// 82691AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691AD4: 4BDD534D  bl 0x82466e20
	ctx.lr = 0x82691AD8;
	sub_82466E20(ctx, base);
	// 82691AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691AE8 size=112
    let mut pc: u32 = 0x82691AE8;
    'dispatch: loop {
        match pc {
            0x82691AE8 => {
    //   block [0x82691AE8..0x82691B58)
	// 82691AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691AF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691AF8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691AFC: 38AAFCA8  addi r5, r10, -0x358
	ctx.r[5].s64 = ctx.r[10].s64 + -856;
	// 82691B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691B04: 390B8838  addi r8, r11, -0x77c8
	ctx.r[8].s64 = ctx.r[11].s64 + -30664;
	// 82691B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82691B0C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82691B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691B14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691B20: 386AFCD8  addi r3, r10, -0x328
	ctx.r[3].s64 = ctx.r[10].s64 + -808;
	// 82691B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691B44: 4BDD52DD  bl 0x82466e20
	ctx.lr = 0x82691B48;
	sub_82466E20(ctx, base);
	// 82691B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691B58 size=112
    let mut pc: u32 = 0x82691B58;
    'dispatch: loop {
        match pc {
            0x82691B58 => {
    //   block [0x82691B58..0x82691BC8)
	// 82691B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691B64: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82691B68: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691B6C: 392A6FDC  addi r9, r10, 0x6fdc
	ctx.r[9].s64 = ctx.r[10].s64 + 28636;
	// 82691B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691B74: 390B8854  addi r8, r11, -0x77ac
	ctx.r[8].s64 = ctx.r[11].s64 + -30636;
	// 82691B78: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82691B7C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82691B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691B84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691B90: 386AFD08  addi r3, r10, -0x2f8
	ctx.r[3].s64 = ctx.r[10].s64 + -760;
	// 82691B94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82691B98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82691B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82691BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691BB4: 4BDD526D  bl 0x82466e20
	ctx.lr = 0x82691BB8;
	sub_82466E20(ctx, base);
	// 82691BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691BC8 size=112
    let mut pc: u32 = 0x82691BC8;
    'dispatch: loop {
        match pc {
            0x82691BC8 => {
    //   block [0x82691BC8..0x82691C38)
	// 82691BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691BD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691BD8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691BDC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82691BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691BE4: 390B8888  addi r8, r11, -0x7778
	ctx.r[8].s64 = ctx.r[11].s64 + -30584;
	// 82691BE8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82691BEC: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82691BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691C00: 386AFD38  addi r3, r10, -0x2c8
	ctx.r[3].s64 = ctx.r[10].s64 + -712;
	// 82691C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691C24: 4BDD51FD  bl 0x82466e20
	ctx.lr = 0x82691C28;
	sub_82466E20(ctx, base);
	// 82691C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82691C38 size=48
    let mut pc: u32 = 0x82691C38;
    'dispatch: loop {
        match pc {
            0x82691C38 => {
    //   block [0x82691C38..0x82691C68)
	// 82691C38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691C3C: 814B8920  lwz r10, -0x76e0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30432 as u32) ) } as u64;
	// 82691C40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691C44: 396BA920  addi r11, r11, -0x56e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22240;
	// 82691C48: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82691C4C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82691C50: 814A891C  lwz r10, -0x76e4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30436 as u32) ) } as u64;
	// 82691C54: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82691C58: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82691C5C: 814A8918  lwz r10, -0x76e8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30440 as u32) ) } as u64;
	// 82691C60: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 82691C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691C68 size=116
    let mut pc: u32 = 0x82691C68;
    'dispatch: loop {
        match pc {
            0x82691C68 => {
    //   block [0x82691C68..0x82691CDC)
	// 82691C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691C74: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82691C78: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691C7C: 392B70D0  addi r9, r11, 0x70d0
	ctx.r[9].s64 = ctx.r[11].s64 + 28880;
	// 82691C80: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82691C84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691C88: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82691C8C: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 82691C90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691C94: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82691C98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691C9C: 396BA920  addi r11, r11, -0x56e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22240;
	// 82691CA0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82691CA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691CA8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82691CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691CB0: 386AFD68  addi r3, r10, -0x298
	ctx.r[3].s64 = ctx.r[10].s64 + -664;
	// 82691CB4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82691CB8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82691CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691CC0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82691CC4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82691CC8: 4BDD5159  bl 0x82466e20
	ctx.lr = 0x82691CCC;
	sub_82466E20(ctx, base);
	// 82691CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691CE0 size=116
    let mut pc: u32 = 0x82691CE0;
    'dispatch: loop {
        match pc {
            0x82691CE0 => {
    //   block [0x82691CE0..0x82691D54)
	// 82691CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691CEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691CF0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82691CF4: 390B8928  addi r8, r11, -0x76d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30424;
	// 82691CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691CFC: 392A71B8  addi r9, r10, 0x71b8
	ctx.r[9].s64 = ctx.r[10].s64 + 29112;
	// 82691D00: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691D04: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82691D08: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82691D0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691D14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691D24: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82691D28: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82691D2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82691D30: 386BFD98  addi r3, r11, -0x268
	ctx.r[3].s64 = ctx.r[11].s64 + -616;
	// 82691D34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82691D38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691D40: 4BDD50E1  bl 0x82466e20
	ctx.lr = 0x82691D44;
	sub_82466E20(ctx, base);
	// 82691D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691D58 size=112
    let mut pc: u32 = 0x82691D58;
    'dispatch: loop {
        match pc {
            0x82691D58 => {
    //   block [0x82691D58..0x82691DC8)
	// 82691D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691D64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691D68: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691D6C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82691D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691D74: 390B89A0  addi r8, r11, -0x7660
	ctx.r[8].s64 = ctx.r[11].s64 + -30304;
	// 82691D78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82691D7C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82691D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691D84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691D90: 386AFDC8  addi r3, r10, -0x238
	ctx.r[3].s64 = ctx.r[10].s64 + -568;
	// 82691D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691DB4: 4BDD506D  bl 0x82466e20
	ctx.lr = 0x82691DB8;
	sub_82466E20(ctx, base);
	// 82691DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691DC8 size=112
    let mut pc: u32 = 0x82691DC8;
    'dispatch: loop {
        match pc {
            0x82691DC8 => {
    //   block [0x82691DC8..0x82691E38)
	// 82691DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691DD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691DD8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691DDC: 38AAE5F8  addi r5, r10, -0x1a08
	ctx.r[5].s64 = ctx.r[10].s64 + -6664;
	// 82691DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691DE4: 390B89B8  addi r8, r11, -0x7648
	ctx.r[8].s64 = ctx.r[11].s64 + -30280;
	// 82691DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82691DEC: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82691DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691DF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691E00: 386AFDF8  addi r3, r10, -0x208
	ctx.r[3].s64 = ctx.r[10].s64 + -520;
	// 82691E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691E24: 4BDD4FFD  bl 0x82466e20
	ctx.lr = 0x82691E28;
	sub_82466E20(ctx, base);
	// 82691E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691E38 size=108
    let mut pc: u32 = 0x82691E38;
    'dispatch: loop {
        match pc {
            0x82691E38 => {
    //   block [0x82691E38..0x82691EA4)
	// 82691E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691E44: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691E4C: 38EB89D0  addi r7, r11, -0x7630
	ctx.r[7].s64 = ctx.r[11].s64 + -30256;
	// 82691E50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82691E54: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82691E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691E5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82691E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691E68: 386AFE28  addi r3, r10, -0x1d8
	ctx.r[3].s64 = ctx.r[10].s64 + -472;
	// 82691E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82691E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82691E90: 4BDD4F91  bl 0x82466e20
	ctx.lr = 0x82691E94;
	sub_82466E20(ctx, base);
	// 82691E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691EA8 size=112
    let mut pc: u32 = 0x82691EA8;
    'dispatch: loop {
        match pc {
            0x82691EA8 => {
    //   block [0x82691EA8..0x82691F18)
	// 82691EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691EB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691EB8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691EBC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82691EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691EC4: 390B89E8  addi r8, r11, -0x7618
	ctx.r[8].s64 = ctx.r[11].s64 + -30232;
	// 82691EC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82691ECC: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82691ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691ED4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691EE0: 386AFE58  addi r3, r10, -0x1a8
	ctx.r[3].s64 = ctx.r[10].s64 + -424;
	// 82691EE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691F04: 4BDD4F1D  bl 0x82466e20
	ctx.lr = 0x82691F08;
	sub_82466E20(ctx, base);
	// 82691F08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691F18 size=108
    let mut pc: u32 = 0x82691F18;
    'dispatch: loop {
        match pc {
            0x82691F18 => {
    //   block [0x82691F18..0x82691F84)
	// 82691F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691F24: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691F2C: 38EB8A30  addi r7, r11, -0x75d0
	ctx.r[7].s64 = ctx.r[11].s64 + -30160;
	// 82691F30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82691F34: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82691F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82691F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691F48: 386AFE88  addi r3, r10, -0x178
	ctx.r[3].s64 = ctx.r[10].s64 + -376;
	// 82691F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82691F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82691F70: 4BDD4EB1  bl 0x82466e20
	ctx.lr = 0x82691F74;
	sub_82466E20(ctx, base);
	// 82691F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691F88 size=112
    let mut pc: u32 = 0x82691F88;
    'dispatch: loop {
        match pc {
            0x82691F88 => {
    //   block [0x82691F88..0x82691FF8)
	// 82691F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82691F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82691F94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691F98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82691F9C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82691FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82691FA4: 390B8A48  addi r8, r11, -0x75b8
	ctx.r[8].s64 = ctx.r[11].s64 + -30136;
	// 82691FA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82691FAC: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82691FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82691FB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82691FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82691FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82691FC0: 386AFEB8  addi r3, r10, -0x148
	ctx.r[3].s64 = ctx.r[10].s64 + -328;
	// 82691FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82691FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82691FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82691FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82691FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82691FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82691FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82691FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82691FE4: 4BDD4E3D  bl 0x82466e20
	ctx.lr = 0x82691FE8;
	sub_82466E20(ctx, base);
	// 82691FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82691FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82691FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82691FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82691FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82691FF8 size=112
    let mut pc: u32 = 0x82691FF8;
    'dispatch: loop {
        match pc {
            0x82691FF8 => {
    //   block [0x82691FF8..0x82692068)
	// 82691FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82691FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692004: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692008: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269200C: 38AAFF78  addi r5, r10, -0x88
	ctx.r[5].s64 = ctx.r[10].s64 + -136;
	// 82692010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82692014: 390B8A78  addi r8, r11, -0x7588
	ctx.r[8].s64 = ctx.r[11].s64 + -30088;
	// 82692018: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269201C: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82692020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692024: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269202C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692030: 386AFEE8  addi r3, r10, -0x118
	ctx.r[3].s64 = ctx.r[10].s64 + -280;
	// 82692034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269203C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269204C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692054: 4BDD4DCD  bl 0x82466e20
	ctx.lr = 0x82692058;
	sub_82466E20(ctx, base);
	// 82692058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692068 size=108
    let mut pc: u32 = 0x82692068;
    'dispatch: loop {
        match pc {
            0x82692068 => {
    //   block [0x82692068..0x826920D4)
	// 82692068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692074: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269207C: 38EB8AF0  addi r7, r11, -0x7510
	ctx.r[7].s64 = ctx.r[11].s64 + -29968;
	// 82692080: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82692084: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82692088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269208C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82692094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692098: 386AFF18  addi r3, r10, -0xe8
	ctx.r[3].s64 = ctx.r[10].s64 + -232;
	// 8269209C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826920A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826920A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826920A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826920AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826920B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826920B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826920B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826920BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826920C0: 4BDD4D61  bl 0x82466e20
	ctx.lr = 0x826920C4;
	sub_82466E20(ctx, base);
	// 826920C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826920C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826920CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826920D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826920D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826920D8 size=108
    let mut pc: u32 = 0x826920D8;
    'dispatch: loop {
        match pc {
            0x826920D8 => {
    //   block [0x826920D8..0x82692144)
	// 826920D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826920DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826920E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826920E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826920E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826920EC: 38EB8B38  addi r7, r11, -0x74c8
	ctx.r[7].s64 = ctx.r[11].s64 + -29896;
	// 826920F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826920F4: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 826920F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826920FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82692104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692108: 386AFF48  addi r3, r10, -0xb8
	ctx.r[3].s64 = ctx.r[10].s64 + -184;
	// 8269210C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269211C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269212C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692130: 4BDD4CF1  bl 0x82466e20
	ctx.lr = 0x82692134;
	sub_82466E20(ctx, base);
	// 82692134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269213C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692148 size=116
    let mut pc: u32 = 0x82692148;
    'dispatch: loop {
        match pc {
            0x82692148 => {
    //   block [0x82692148..0x826921BC)
	// 82692148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269214C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692154: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82692158: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8269215C: 390A8B80  addi r8, r10, -0x7480
	ctx.r[8].s64 = ctx.r[10].s64 + -29824;
	// 82692160: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692164: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82692168: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 8269216C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692170: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82692174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269217C: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82692180: 396B71CC  addi r11, r11, 0x71cc
	ctx.r[11].s64 = ctx.r[11].s64 + 29132;
	// 82692184: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692188: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269218C: 386AFF78  addi r3, r10, -0x88
	ctx.r[3].s64 = ctx.r[10].s64 + -136;
	// 82692190: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82692194: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692198: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269219C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826921A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826921A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826921A8: 4BDD4C79  bl 0x82466e20
	ctx.lr = 0x826921AC;
	sub_82466E20(ctx, base);
	// 826921AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826921B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826921B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826921B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826921C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826921C0 size=108
    let mut pc: u32 = 0x826921C0;
    'dispatch: loop {
        match pc {
            0x826921C0 => {
    //   block [0x826921C0..0x8269222C)
	// 826921C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826921C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826921C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826921CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826921D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826921D4: 38EB8C58  addi r7, r11, -0x73a8
	ctx.r[7].s64 = ctx.r[11].s64 + -29608;
	// 826921D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826921DC: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 826921E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826921E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826921E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826921EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826921F0: 386AFFA8  addi r3, r10, -0x58
	ctx.r[3].s64 = ctx.r[10].s64 + -88;
	// 826921F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826921F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826921FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269220C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692218: 4BDD4C09  bl 0x82466e20
	ctx.lr = 0x8269221C;
	sub_82466E20(ctx, base);
	// 8269221C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692230 size=108
    let mut pc: u32 = 0x82692230;
    'dispatch: loop {
        match pc {
            0x82692230 => {
    //   block [0x82692230..0x8269229C)
	// 82692230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269223C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82692244: 38EB8C88  addi r7, r11, -0x7378
	ctx.r[7].s64 = ctx.r[11].s64 + -29560;
	// 82692248: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269224C: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 82692250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692254: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269225C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692260: 386AFFD8  addi r3, r10, -0x28
	ctx.r[3].s64 = ctx.r[10].s64 + -40;
	// 82692264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269226C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269227C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692288: 4BDD4B99  bl 0x82466e20
	ctx.lr = 0x8269228C;
	sub_82466E20(ctx, base);
	// 8269228C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826922A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826922A0 size=112
    let mut pc: u32 = 0x826922A0;
    'dispatch: loop {
        match pc {
            0x826922A0 => {
    //   block [0x826922A0..0x82692310)
	// 826922A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826922A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826922A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826922AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826922B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826922B4: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 826922B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826922BC: 390B8CB8  addi r8, r11, -0x7348
	ctx.r[8].s64 = ctx.r[11].s64 + -29512;
	// 826922C0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826922C4: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826922C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826922CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826922D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826922D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826922D8: 386A0008  addi r3, r10, 8
	ctx.r[3].s64 = ctx.r[10].s64 + 8;
	// 826922DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826922E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826922E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826922E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826922EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826922F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826922F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826922F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826922FC: 4BDD4B25  bl 0x82466e20
	ctx.lr = 0x82692300;
	sub_82466E20(ctx, base);
	// 82692300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269230C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692310 size=112
    let mut pc: u32 = 0x82692310;
    'dispatch: loop {
        match pc {
            0x82692310 => {
    //   block [0x82692310..0x82692380)
	// 82692310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269231C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692320: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692324: 38AAEFB8  addi r5, r10, -0x1048
	ctx.r[5].s64 = ctx.r[10].s64 + -4168;
	// 82692328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269232C: 390B8D90  addi r8, r11, -0x7270
	ctx.r[8].s64 = ctx.r[11].s64 + -29296;
	// 82692330: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82692334: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82692338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269233C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692348: 386A0038  addi r3, r10, 0x38
	ctx.r[3].s64 = ctx.r[10].s64 + 56;
	// 8269234C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269235C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269236C: 4BDD4AB5  bl 0x82466e20
	ctx.lr = 0x82692370;
	sub_82466E20(ctx, base);
	// 82692370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269237C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692380 size=112
    let mut pc: u32 = 0x82692380;
    'dispatch: loop {
        match pc {
            0x82692380 => {
    //   block [0x82692380..0x826923F0)
	// 82692380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269238C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692390: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692394: 38AAEFB8  addi r5, r10, -0x1048
	ctx.r[5].s64 = ctx.r[10].s64 + -4168;
	// 82692398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269239C: 390B8DD8  addi r8, r11, -0x7228
	ctx.r[8].s64 = ctx.r[11].s64 + -29224;
	// 826923A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826923A4: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826923A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826923AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826923B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826923B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826923B8: 386A0068  addi r3, r10, 0x68
	ctx.r[3].s64 = ctx.r[10].s64 + 104;
	// 826923BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826923C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826923C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826923C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826923CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826923D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826923D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826923D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826923DC: 4BDD4A45  bl 0x82466e20
	ctx.lr = 0x826923E0;
	sub_82466E20(ctx, base);
	// 826923E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826923E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826923E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826923EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826923F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826923F0 size=112
    let mut pc: u32 = 0x826923F0;
    'dispatch: loop {
        match pc {
            0x826923F0 => {
    //   block [0x826923F0..0x82692460)
	// 826923F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826923F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826923F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826923FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692400: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692404: 38AAEFE8  addi r5, r10, -0x1018
	ctx.r[5].s64 = ctx.r[10].s64 + -4120;
	// 82692408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269240C: 390B8E38  addi r8, r11, -0x71c8
	ctx.r[8].s64 = ctx.r[11].s64 + -29128;
	// 82692410: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82692414: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82692418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269241C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692428: 386A0098  addi r3, r10, 0x98
	ctx.r[3].s64 = ctx.r[10].s64 + 152;
	// 8269242C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269243C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269244C: 4BDD49D5  bl 0x82466e20
	ctx.lr = 0x82692450;
	sub_82466E20(ctx, base);
	// 82692450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269245C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692460 size=112
    let mut pc: u32 = 0x82692460;
    'dispatch: loop {
        match pc {
            0x82692460 => {
    //   block [0x82692460..0x826924D0)
	// 82692460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269246C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692470: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692474: 38AAEFE8  addi r5, r10, -0x1018
	ctx.r[5].s64 = ctx.r[10].s64 + -4120;
	// 82692478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269247C: 390B8E98  addi r8, r11, -0x7168
	ctx.r[8].s64 = ctx.r[11].s64 + -29032;
	// 82692480: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82692484: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82692488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269248C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692498: 386A00C8  addi r3, r10, 0xc8
	ctx.r[3].s64 = ctx.r[10].s64 + 200;
	// 8269249C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826924A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826924A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826924A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826924AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826924B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826924B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826924B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826924BC: 4BDD4965  bl 0x82466e20
	ctx.lr = 0x826924C0;
	sub_82466E20(ctx, base);
	// 826924C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826924C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826924C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826924CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826924D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826924D0 size=112
    let mut pc: u32 = 0x826924D0;
    'dispatch: loop {
        match pc {
            0x826924D0 => {
    //   block [0x826924D0..0x82692540)
	// 826924D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826924D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826924D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826924DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826924E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826924E4: 38AAEFB8  addi r5, r10, -0x1048
	ctx.r[5].s64 = ctx.r[10].s64 + -4168;
	// 826924E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826924EC: 390B8EF8  addi r8, r11, -0x7108
	ctx.r[8].s64 = ctx.r[11].s64 + -28936;
	// 826924F0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826924F4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826924F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826924FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692508: 386A00F8  addi r3, r10, 0xf8
	ctx.r[3].s64 = ctx.r[10].s64 + 248;
	// 8269250C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269251C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269252C: 4BDD48F5  bl 0x82466e20
	ctx.lr = 0x82692530;
	sub_82466E20(ctx, base);
	// 82692530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269253C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692540 size=108
    let mut pc: u32 = 0x82692540;
    'dispatch: loop {
        match pc {
            0x82692540 => {
    //   block [0x82692540..0x826925AC)
	// 82692540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269254C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692554: 38EB8FB8  addi r7, r11, -0x7048
	ctx.r[7].s64 = ctx.r[11].s64 + -28744;
	// 82692558: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8269255C: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82692560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692564: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269256C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692570: 386A0128  addi r3, r10, 0x128
	ctx.r[3].s64 = ctx.r[10].s64 + 296;
	// 82692574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269257C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269258C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692598: 4BDD4889  bl 0x82466e20
	ctx.lr = 0x8269259C;
	sub_82466E20(ctx, base);
	// 8269259C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826925A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826925A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826925A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826925B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826925B0 size=112
    let mut pc: u32 = 0x826925B0;
    'dispatch: loop {
        match pc {
            0x826925B0 => {
    //   block [0x826925B0..0x82692620)
	// 826925B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826925B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826925B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826925BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826925C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826925C4: 38AAE6B8  addi r5, r10, -0x1948
	ctx.r[5].s64 = ctx.r[10].s64 + -6472;
	// 826925C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826925CC: 390B9150  addi r8, r11, -0x6eb0
	ctx.r[8].s64 = ctx.r[11].s64 + -28336;
	// 826925D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826925D4: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826925D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826925DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826925E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826925E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826925E8: 386A0158  addi r3, r10, 0x158
	ctx.r[3].s64 = ctx.r[10].s64 + 344;
	// 826925EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826925F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826925F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826925F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826925FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269260C: 4BDD4815  bl 0x82466e20
	ctx.lr = 0x82692610;
	sub_82466E20(ctx, base);
	// 82692610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269261C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692620 size=112
    let mut pc: u32 = 0x82692620;
    'dispatch: loop {
        match pc {
            0x82692620 => {
    //   block [0x82692620..0x82692690)
	// 82692620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269262C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692630: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692634: 38AAE6B8  addi r5, r10, -0x1948
	ctx.r[5].s64 = ctx.r[10].s64 + -6472;
	// 82692638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269263C: 390B9168  addi r8, r11, -0x6e98
	ctx.r[8].s64 = ctx.r[11].s64 + -28312;
	// 82692640: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82692644: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82692648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269264C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692658: 386A0188  addi r3, r10, 0x188
	ctx.r[3].s64 = ctx.r[10].s64 + 392;
	// 8269265C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269266C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82692670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269267C: 4BDD47A5  bl 0x82466e20
	ctx.lr = 0x82692680;
	sub_82466E20(ctx, base);
	// 82692680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269268C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692690 size=112
    let mut pc: u32 = 0x82692690;
    'dispatch: loop {
        match pc {
            0x82692690 => {
    //   block [0x82692690..0x82692700)
	// 82692690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269269C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826926A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826926A4: 38AAE6B8  addi r5, r10, -0x1948
	ctx.r[5].s64 = ctx.r[10].s64 + -6472;
	// 826926A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826926AC: 390B9180  addi r8, r11, -0x6e80
	ctx.r[8].s64 = ctx.r[11].s64 + -28288;
	// 826926B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826926B4: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826926B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826926BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826926C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826926C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826926C8: 386A01B8  addi r3, r10, 0x1b8
	ctx.r[3].s64 = ctx.r[10].s64 + 440;
	// 826926CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826926D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826926D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826926D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826926DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826926E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826926E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826926E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826926EC: 4BDD4735  bl 0x82466e20
	ctx.lr = 0x826926F0;
	sub_82466E20(ctx, base);
	// 826926F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826926F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826926F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826926FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692700 size=108
    let mut pc: u32 = 0x82692700;
    'dispatch: loop {
        match pc {
            0x82692700 => {
    //   block [0x82692700..0x8269276C)
	// 82692700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269270C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692714: 38EB91B0  addi r7, r11, -0x6e50
	ctx.r[7].s64 = ctx.r[11].s64 + -28240;
	// 82692718: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269271C: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82692720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692724: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692728: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269272C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692730: 386A01E8  addi r3, r10, 0x1e8
	ctx.r[3].s64 = ctx.r[10].s64 + 488;
	// 82692734: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269273C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269274C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692754: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692758: 4BDD46C9  bl 0x82466e20
	ctx.lr = 0x8269275C;
	sub_82466E20(ctx, base);
	// 8269275C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692770 size=112
    let mut pc: u32 = 0x82692770;
    'dispatch: loop {
        match pc {
            0x82692770 => {
    //   block [0x82692770..0x826927E0)
	// 82692770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269277C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692780: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692784: 38AAE6B8  addi r5, r10, -0x1948
	ctx.r[5].s64 = ctx.r[10].s64 + -6472;
	// 82692788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269278C: 390B91E0  addi r8, r11, -0x6e20
	ctx.r[8].s64 = ctx.r[11].s64 + -28192;
	// 82692790: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82692794: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82692798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269279C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826927A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826927A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826927A8: 386A0218  addi r3, r10, 0x218
	ctx.r[3].s64 = ctx.r[10].s64 + 536;
	// 826927AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826927B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826927B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826927B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826927BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826927C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826927C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826927C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826927CC: 4BDD4655  bl 0x82466e20
	ctx.lr = 0x826927D0;
	sub_82466E20(ctx, base);
	// 826927D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826927D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826927D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826927DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826927E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826927E0 size=108
    let mut pc: u32 = 0x826927E0;
    'dispatch: loop {
        match pc {
            0x826927E0 => {
    //   block [0x826927E0..0x8269284C)
	// 826927E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826927E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826927E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826927EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826927F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826927F4: 38EB91F8  addi r7, r11, -0x6e08
	ctx.r[7].s64 = ctx.r[11].s64 + -28168;
	// 826927F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826927FC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82692800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269280C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692810: 386A0248  addi r3, r10, 0x248
	ctx.r[3].s64 = ctx.r[10].s64 + 584;
	// 82692814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269281C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269282C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692838: 4BDD45E9  bl 0x82466e20
	ctx.lr = 0x8269283C;
	sub_82466E20(ctx, base);
	// 8269283C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692850 size=108
    let mut pc: u32 = 0x82692850;
    'dispatch: loop {
        match pc {
            0x82692850 => {
    //   block [0x82692850..0x826928BC)
	// 82692850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269285C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692864: 38EB9228  addi r7, r11, -0x6dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -28120;
	// 82692868: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269286C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82692870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269287C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692880: 386A0278  addi r3, r10, 0x278
	ctx.r[3].s64 = ctx.r[10].s64 + 632;
	// 82692884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269288C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269289C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826928A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826928A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826928A8: 4BDD4579  bl 0x82466e20
	ctx.lr = 0x826928AC;
	sub_82466E20(ctx, base);
	// 826928AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826928B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826928B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826928B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826928C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826928C0 size=112
    let mut pc: u32 = 0x826928C0;
    'dispatch: loop {
        match pc {
            0x826928C0 => {
    //   block [0x826928C0..0x82692930)
	// 826928C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826928C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826928C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826928CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826928D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826928D4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826928D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826928DC: 390B9270  addi r8, r11, -0x6d90
	ctx.r[8].s64 = ctx.r[11].s64 + -28048;
	// 826928E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826928E4: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826928E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826928EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826928F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826928F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826928F8: 386A02A8  addi r3, r10, 0x2a8
	ctx.r[3].s64 = ctx.r[10].s64 + 680;
	// 826928FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269290C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269291C: 4BDD4505  bl 0x82466e20
	ctx.lr = 0x82692920;
	sub_82466E20(ctx, base);
	// 82692920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269292C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692930 size=112
    let mut pc: u32 = 0x82692930;
    'dispatch: loop {
        match pc {
            0x82692930 => {
    //   block [0x82692930..0x826929A0)
	// 82692930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269293C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692940: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692944: 38AAEFE8  addi r5, r10, -0x1018
	ctx.r[5].s64 = ctx.r[10].s64 + -4120;
	// 82692948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269294C: 390B92B8  addi r8, r11, -0x6d48
	ctx.r[8].s64 = ctx.r[11].s64 + -27976;
	// 82692950: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82692954: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82692958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269295C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692968: 386A02D8  addi r3, r10, 0x2d8
	ctx.r[3].s64 = ctx.r[10].s64 + 728;
	// 8269296C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269297C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269298C: 4BDD4495  bl 0x82466e20
	ctx.lr = 0x82692990;
	sub_82466E20(ctx, base);
	// 82692990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269299C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826929A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826929A0 size=108
    let mut pc: u32 = 0x826929A0;
    'dispatch: loop {
        match pc {
            0x826929A0 => {
    //   block [0x826929A0..0x82692A0C)
	// 826929A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826929A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826929A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826929AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826929B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826929B4: 38EB9348  addi r7, r11, -0x6cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -27832;
	// 826929B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826929BC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826929C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826929C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826929C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826929CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826929D0: 386A0308  addi r3, r10, 0x308
	ctx.r[3].s64 = ctx.r[10].s64 + 776;
	// 826929D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826929D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826929DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826929E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826929E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826929E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826929EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826929F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826929F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826929F8: 4BDD4429  bl 0x82466e20
	ctx.lr = 0x826929FC;
	sub_82466E20(ctx, base);
	// 826929FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692A10 size=108
    let mut pc: u32 = 0x82692A10;
    'dispatch: loop {
        match pc {
            0x82692A10 => {
    //   block [0x82692A10..0x82692A7C)
	// 82692A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692A1C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692A24: 38EB9390  addi r7, r11, -0x6c70
	ctx.r[7].s64 = ctx.r[11].s64 + -27760;
	// 82692A28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82692A2C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82692A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692A34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82692A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692A40: 386A0338  addi r3, r10, 0x338
	ctx.r[3].s64 = ctx.r[10].s64 + 824;
	// 82692A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692A68: 4BDD43B9  bl 0x82466e20
	ctx.lr = 0x82692A6C;
	sub_82466E20(ctx, base);
	// 82692A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692A80 size=108
    let mut pc: u32 = 0x82692A80;
    'dispatch: loop {
        match pc {
            0x82692A80 => {
    //   block [0x82692A80..0x82692AEC)
	// 82692A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692A8C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692A94: 38EB93C0  addi r7, r11, -0x6c40
	ctx.r[7].s64 = ctx.r[11].s64 + -27712;
	// 82692A98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82692A9C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82692AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82692AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692AB0: 386A0368  addi r3, r10, 0x368
	ctx.r[3].s64 = ctx.r[10].s64 + 872;
	// 82692AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692AD8: 4BDD4349  bl 0x82466e20
	ctx.lr = 0x82692ADC;
	sub_82466E20(ctx, base);
	// 82692ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692AF0 size=112
    let mut pc: u32 = 0x82692AF0;
    'dispatch: loop {
        match pc {
            0x82692AF0 => {
    //   block [0x82692AF0..0x82692B60)
	// 82692AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692AFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692B00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692B04: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82692B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692B0C: 390B93F0  addi r8, r11, -0x6c10
	ctx.r[8].s64 = ctx.r[11].s64 + -27664;
	// 82692B10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82692B14: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82692B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692B1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692B28: 386A0398  addi r3, r10, 0x398
	ctx.r[3].s64 = ctx.r[10].s64 + 920;
	// 82692B2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692B4C: 4BDD42D5  bl 0x82466e20
	ctx.lr = 0x82692B50;
	sub_82466E20(ctx, base);
	// 82692B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692B60 size=112
    let mut pc: u32 = 0x82692B60;
    'dispatch: loop {
        match pc {
            0x82692B60 => {
    //   block [0x82692B60..0x82692BD0)
	// 82692B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692B6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692B70: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692B74: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82692B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692B7C: 390B9420  addi r8, r11, -0x6be0
	ctx.r[8].s64 = ctx.r[11].s64 + -27616;
	// 82692B80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82692B84: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82692B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692B8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692B98: 386A03C8  addi r3, r10, 0x3c8
	ctx.r[3].s64 = ctx.r[10].s64 + 968;
	// 82692B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692BBC: 4BDD4265  bl 0x82466e20
	ctx.lr = 0x82692BC0;
	sub_82466E20(ctx, base);
	// 82692BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692BD0 size=112
    let mut pc: u32 = 0x82692BD0;
    'dispatch: loop {
        match pc {
            0x82692BD0 => {
    //   block [0x82692BD0..0x82692C40)
	// 82692BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692BDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692BE0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692BE4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82692BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692BEC: 390B9438  addi r8, r11, -0x6bc8
	ctx.r[8].s64 = ctx.r[11].s64 + -27592;
	// 82692BF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82692BF4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82692BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692C00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692C08: 386A03F8  addi r3, r10, 0x3f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1016;
	// 82692C0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692C2C: 4BDD41F5  bl 0x82466e20
	ctx.lr = 0x82692C30;
	sub_82466E20(ctx, base);
	// 82692C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692C40 size=108
    let mut pc: u32 = 0x82692C40;
    'dispatch: loop {
        match pc {
            0x82692C40 => {
    //   block [0x82692C40..0x82692CAC)
	// 82692C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692C4C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692C54: 38EB9450  addi r7, r11, -0x6bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -27568;
	// 82692C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82692C5C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82692C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692C64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82692C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692C70: 386A0428  addi r3, r10, 0x428
	ctx.r[3].s64 = ctx.r[10].s64 + 1064;
	// 82692C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692C98: 4BDD4189  bl 0x82466e20
	ctx.lr = 0x82692C9C;
	sub_82466E20(ctx, base);
	// 82692C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692CB0 size=112
    let mut pc: u32 = 0x82692CB0;
    'dispatch: loop {
        match pc {
            0x82692CB0 => {
    //   block [0x82692CB0..0x82692D20)
	// 82692CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692CBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692CC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692CC4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82692CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692CCC: 390B9480  addi r8, r11, -0x6b80
	ctx.r[8].s64 = ctx.r[11].s64 + -27520;
	// 82692CD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82692CD4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82692CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692CE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692CE8: 386A0458  addi r3, r10, 0x458
	ctx.r[3].s64 = ctx.r[10].s64 + 1112;
	// 82692CEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692CF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692D0C: 4BDD4115  bl 0x82466e20
	ctx.lr = 0x82692D10;
	sub_82466E20(ctx, base);
	// 82692D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692D20 size=108
    let mut pc: u32 = 0x82692D20;
    'dispatch: loop {
        match pc {
            0x82692D20 => {
    //   block [0x82692D20..0x82692D8C)
	// 82692D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692D2C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692D34: 38EB9498  addi r7, r11, -0x6b68
	ctx.r[7].s64 = ctx.r[11].s64 + -27496;
	// 82692D38: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82692D3C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82692D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692D44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82692D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692D50: 386A0488  addi r3, r10, 0x488
	ctx.r[3].s64 = ctx.r[10].s64 + 1160;
	// 82692D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692D78: 4BDD40A9  bl 0x82466e20
	ctx.lr = 0x82692D7C;
	sub_82466E20(ctx, base);
	// 82692D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692D90 size=112
    let mut pc: u32 = 0x82692D90;
    'dispatch: loop {
        match pc {
            0x82692D90 => {
    //   block [0x82692D90..0x82692E00)
	// 82692D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692D9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692DA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692DA4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82692DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692DAC: 390B9570  addi r8, r11, -0x6a90
	ctx.r[8].s64 = ctx.r[11].s64 + -27280;
	// 82692DB0: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82692DB4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82692DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692DC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692DC8: 386A04B8  addi r3, r10, 0x4b8
	ctx.r[3].s64 = ctx.r[10].s64 + 1208;
	// 82692DCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692DEC: 4BDD4035  bl 0x82466e20
	ctx.lr = 0x82692DF0;
	sub_82466E20(ctx, base);
	// 82692DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692E00 size=108
    let mut pc: u32 = 0x82692E00;
    'dispatch: loop {
        match pc {
            0x82692E00 => {
    //   block [0x82692E00..0x82692E6C)
	// 82692E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692E0C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692E14: 38EB9720  addi r7, r11, -0x68e0
	ctx.r[7].s64 = ctx.r[11].s64 + -26848;
	// 82692E18: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82692E1C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82692E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692E24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82692E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692E30: 386A04E8  addi r3, r10, 0x4e8
	ctx.r[3].s64 = ctx.r[10].s64 + 1256;
	// 82692E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82692E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82692E58: 4BDD3FC9  bl 0x82466e20
	ctx.lr = 0x82692E5C;
	sub_82466E20(ctx, base);
	// 82692E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692E70 size=112
    let mut pc: u32 = 0x82692E70;
    'dispatch: loop {
        match pc {
            0x82692E70 => {
    //   block [0x82692E70..0x82692EE0)
	// 82692E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692E7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692E80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692E84: 38AAEFE8  addi r5, r10, -0x1018
	ctx.r[5].s64 = ctx.r[10].s64 + -4120;
	// 82692E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692E8C: 390B98B8  addi r8, r11, -0x6748
	ctx.r[8].s64 = ctx.r[11].s64 + -26440;
	// 82692E90: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 82692E94: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82692E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692EA8: 386A0518  addi r3, r10, 0x518
	ctx.r[3].s64 = ctx.r[10].s64 + 1304;
	// 82692EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692ECC: 4BDD3F55  bl 0x82466e20
	ctx.lr = 0x82692ED0;
	sub_82466E20(ctx, base);
	// 82692ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692EE0 size=100
    let mut pc: u32 = 0x82692EE0;
    'dispatch: loop {
        match pc {
            0x82692EE0 => {
    //   block [0x82692EE0..0x82692F44)
	// 82692EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692EEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692EF4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82692EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692F00: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82692F04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692F14: 386A0548  addi r3, r10, 0x548
	ctx.r[3].s64 = ctx.r[10].s64 + 1352;
	// 82692F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692F20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82692F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692F28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82692F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692F30: 4BDD3EF1  bl 0x82466e20
	ctx.lr = 0x82692F34;
	sub_82466E20(ctx, base);
	// 82692F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692F48 size=112
    let mut pc: u32 = 0x82692F48;
    'dispatch: loop {
        match pc {
            0x82692F48 => {
    //   block [0x82692F48..0x82692FB8)
	// 82692F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692F54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692F58: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82692F5C: 38AA0548  addi r5, r10, 0x548
	ctx.r[5].s64 = ctx.r[10].s64 + 1352;
	// 82692F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692F64: 390B9B28  addi r8, r11, -0x64d8
	ctx.r[8].s64 = ctx.r[11].s64 + -25816;
	// 82692F68: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82692F6C: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82692F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692F74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82692F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82692F80: 386A0578  addi r3, r10, 0x578
	ctx.r[3].s64 = ctx.r[10].s64 + 1400;
	// 82692F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82692F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82692FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692FA4: 4BDD3E7D  bl 0x82466e20
	ctx.lr = 0x82692FA8;
	sub_82466E20(ctx, base);
	// 82692FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82692FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82692FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82692FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82692FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82692FB8 size=100
    let mut pc: u32 = 0x82692FB8;
    'dispatch: loop {
        match pc {
            0x82692FB8 => {
    //   block [0x82692FB8..0x8269301C)
	// 82692FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82692FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82692FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82692FC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82692FCC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82692FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82692FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82692FD8: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82692FDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82692FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82692FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82692FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82692FEC: 386A05A8  addi r3, r10, 0x5a8
	ctx.r[3].s64 = ctx.r[10].s64 + 1448;
	// 82692FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82692FF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82692FF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82692FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693000: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82693004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693008: 4BDD3E19  bl 0x82466e20
	ctx.lr = 0x8269300C;
	sub_82466E20(ctx, base);
	// 8269300C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693020 size=108
    let mut pc: u32 = 0x82693020;
    'dispatch: loop {
        match pc {
            0x82693020 => {
    //   block [0x82693020..0x8269308C)
	// 82693020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269302C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693034: 38EB9BA0  addi r7, r11, -0x6460
	ctx.r[7].s64 = ctx.r[11].s64 + -25696;
	// 82693038: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269303C: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82693040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693044: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269304C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693050: 386A05D8  addi r3, r10, 0x5d8
	ctx.r[3].s64 = ctx.r[10].s64 + 1496;
	// 82693054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269305C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269306C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693078: 4BDD3DA9  bl 0x82466e20
	ctx.lr = 0x8269307C;
	sub_82466E20(ctx, base);
	// 8269307C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693090 size=112
    let mut pc: u32 = 0x82693090;
    'dispatch: loop {
        match pc {
            0x82693090 => {
    //   block [0x82693090..0x82693100)
	// 82693090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269309C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826930A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826930A4: 38AA05A8  addi r5, r10, 0x5a8
	ctx.r[5].s64 = ctx.r[10].s64 + 1448;
	// 826930A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826930AC: 390B9BE8  addi r8, r11, -0x6418
	ctx.r[8].s64 = ctx.r[11].s64 + -25624;
	// 826930B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826930B4: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826930B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826930BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826930C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826930C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826930C8: 386A0608  addi r3, r10, 0x608
	ctx.r[3].s64 = ctx.r[10].s64 + 1544;
	// 826930CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826930D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826930D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826930D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826930DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826930E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826930E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826930E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826930EC: 4BDD3D35  bl 0x82466e20
	ctx.lr = 0x826930F0;
	sub_82466E20(ctx, base);
	// 826930F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826930F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826930F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826930FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693100 size=100
    let mut pc: u32 = 0x82693100;
    'dispatch: loop {
        match pc {
            0x82693100 => {
    //   block [0x82693100..0x82693164)
	// 82693100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269310C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693114: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269311C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693120: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 82693124: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269312C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693134: 386A0638  addi r3, r10, 0x638
	ctx.r[3].s64 = ctx.r[10].s64 + 1592;
	// 82693138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269313C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693140: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82693144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693148: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269314C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693150: 4BDD3CD1  bl 0x82466e20
	ctx.lr = 0x82693154;
	sub_82466E20(ctx, base);
	// 82693154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269315C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693168 size=100
    let mut pc: u32 = 0x82693168;
    'dispatch: loop {
        match pc {
            0x82693168 => {
    //   block [0x82693168..0x826931CC)
	// 82693168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269316C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693174: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269317C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693188: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8269318C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269319C: 386A0668  addi r3, r10, 0x668
	ctx.r[3].s64 = ctx.r[10].s64 + 1640;
	// 826931A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826931A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826931A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826931AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826931B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826931B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826931B8: 4BDD3C69  bl 0x82466e20
	ctx.lr = 0x826931BC;
	sub_82466E20(ctx, base);
	// 826931BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826931C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826931C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826931C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826931D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826931D0 size=112
    let mut pc: u32 = 0x826931D0;
    'dispatch: loop {
        match pc {
            0x826931D0 => {
    //   block [0x826931D0..0x82693240)
	// 826931D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826931D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826931D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826931DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826931E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826931E4: 38AA0638  addi r5, r10, 0x638
	ctx.r[5].s64 = ctx.r[10].s64 + 1592;
	// 826931E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826931EC: 390B9C18  addi r8, r11, -0x63e8
	ctx.r[8].s64 = ctx.r[11].s64 + -25576;
	// 826931F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826931F4: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826931F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826931FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693208: 386A0698  addi r3, r10, 0x698
	ctx.r[3].s64 = ctx.r[10].s64 + 1688;
	// 8269320C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269321C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269322C: 4BDD3BF5  bl 0x82466e20
	ctx.lr = 0x82693230;
	sub_82466E20(ctx, base);
	// 82693230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269323C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693240 size=112
    let mut pc: u32 = 0x82693240;
    'dispatch: loop {
        match pc {
            0x82693240 => {
    //   block [0x82693240..0x826932B0)
	// 82693240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269324C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693250: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693254: 38AA0668  addi r5, r10, 0x668
	ctx.r[5].s64 = ctx.r[10].s64 + 1640;
	// 82693258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269325C: 390B9C78  addi r8, r11, -0x6388
	ctx.r[8].s64 = ctx.r[11].s64 + -25480;
	// 82693260: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82693264: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82693268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269326C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693278: 386A06C8  addi r3, r10, 0x6c8
	ctx.r[3].s64 = ctx.r[10].s64 + 1736;
	// 8269327C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269328C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269329C: 4BDD3B85  bl 0x82466e20
	ctx.lr = 0x826932A0;
	sub_82466E20(ctx, base);
	// 826932A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826932A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826932A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826932AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826932B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826932B0 size=100
    let mut pc: u32 = 0x826932B0;
    'dispatch: loop {
        match pc {
            0x826932B0 => {
    //   block [0x826932B0..0x82693314)
	// 826932B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826932B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826932B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826932BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826932C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826932C4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826932C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826932CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826932D0: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826932D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826932D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826932DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826932E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826932E4: 386A06F8  addi r3, r10, 0x6f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1784;
	// 826932E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826932EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826932F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826932F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826932F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826932FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693300: 4BDD3B21  bl 0x82466e20
	ctx.lr = 0x82693304;
	sub_82466E20(ctx, base);
	// 82693304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269330C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693318 size=112
    let mut pc: u32 = 0x82693318;
    'dispatch: loop {
        match pc {
            0x82693318 => {
    //   block [0x82693318..0x82693388)
	// 82693318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269331C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693328: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269332C: 38AA06F8  addi r5, r10, 0x6f8
	ctx.r[5].s64 = ctx.r[10].s64 + 1784;
	// 82693330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693334: 390B9CD8  addi r8, r11, -0x6328
	ctx.r[8].s64 = ctx.r[11].s64 + -25384;
	// 82693338: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8269333C: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82693340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693344: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269334C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693350: 386A0728  addi r3, r10, 0x728
	ctx.r[3].s64 = ctx.r[10].s64 + 1832;
	// 82693354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269335C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269336C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693374: 4BDD3AAD  bl 0x82466e20
	ctx.lr = 0x82693378;
	sub_82466E20(ctx, base);
	// 82693378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269337C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693388 size=108
    let mut pc: u32 = 0x82693388;
    'dispatch: loop {
        match pc {
            0x82693388 => {
    //   block [0x82693388..0x826933F4)
	// 82693388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269338C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693394: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269339C: 38EB9DC8  addi r7, r11, -0x6238
	ctx.r[7].s64 = ctx.r[11].s64 + -25144;
	// 826933A0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826933A4: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826933A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826933AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826933B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826933B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826933B8: 386A0758  addi r3, r10, 0x758
	ctx.r[3].s64 = ctx.r[10].s64 + 1880;
	// 826933BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826933C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826933C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826933C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826933CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826933D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826933D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826933D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826933DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826933E0: 4BDD3A41  bl 0x82466e20
	ctx.lr = 0x826933E4;
	sub_82466E20(ctx, base);
	// 826933E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826933E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826933EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826933F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826933F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826933F8 size=108
    let mut pc: u32 = 0x826933F8;
    'dispatch: loop {
        match pc {
            0x826933F8 => {
    //   block [0x826933F8..0x82693464)
	// 826933F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826933FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693404: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269340C: 38EB9EB8  addi r7, r11, -0x6148
	ctx.r[7].s64 = ctx.r[11].s64 + -24904;
	// 82693410: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82693414: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82693418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269341C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693428: 386A0788  addi r3, r10, 0x788
	ctx.r[3].s64 = ctx.r[10].s64 + 1928;
	// 8269342C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269343C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269344C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693450: 4BDD39D1  bl 0x82466e20
	ctx.lr = 0x82693454;
	sub_82466E20(ctx, base);
	// 82693454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269345C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693468 size=108
    let mut pc: u32 = 0x82693468;
    'dispatch: loop {
        match pc {
            0x82693468 => {
    //   block [0x82693468..0x826934D4)
	// 82693468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269346C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693474: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269347C: 38EB9F00  addi r7, r11, -0x6100
	ctx.r[7].s64 = ctx.r[11].s64 + -24832;
	// 82693480: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82693484: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82693488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269348C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693498: 386A07B8  addi r3, r10, 0x7b8
	ctx.r[3].s64 = ctx.r[10].s64 + 1976;
	// 8269349C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826934A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826934A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826934A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826934AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826934B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826934B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826934B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826934BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826934C0: 4BDD3961  bl 0x82466e20
	ctx.lr = 0x826934C4;
	sub_82466E20(ctx, base);
	// 826934C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826934C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826934CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826934D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826934D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826934D8 size=108
    let mut pc: u32 = 0x826934D8;
    'dispatch: loop {
        match pc {
            0x826934D8 => {
    //   block [0x826934D8..0x82693544)
	// 826934D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826934DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826934E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826934E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826934E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826934EC: 38EB9FD8  addi r7, r11, -0x6028
	ctx.r[7].s64 = ctx.r[11].s64 + -24616;
	// 826934F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826934F4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826934F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826934FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693508: 386A07E8  addi r3, r10, 0x7e8
	ctx.r[3].s64 = ctx.r[10].s64 + 2024;
	// 8269350C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269351C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269352C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693530: 4BDD38F1  bl 0x82466e20
	ctx.lr = 0x82693534;
	sub_82466E20(ctx, base);
	// 82693534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269353C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693548 size=100
    let mut pc: u32 = 0x82693548;
    'dispatch: loop {
        match pc {
            0x82693548 => {
    //   block [0x82693548..0x826935AC)
	// 82693548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269354C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693554: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269355C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693568: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8269356C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269357C: 386A0818  addi r3, r10, 0x818
	ctx.r[3].s64 = ctx.r[10].s64 + 2072;
	// 82693580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693584: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693588: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269358C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693590: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82693594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693598: 4BDD3889  bl 0x82466e20
	ctx.lr = 0x8269359C;
	sub_82466E20(ctx, base);
	// 8269359C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826935A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826935A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826935A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826935B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826935B0 size=112
    let mut pc: u32 = 0x826935B0;
    'dispatch: loop {
        match pc {
            0x826935B0 => {
    //   block [0x826935B0..0x82693620)
	// 826935B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826935B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826935B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826935BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826935C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826935C4: 38AA0818  addi r5, r10, 0x818
	ctx.r[5].s64 = ctx.r[10].s64 + 2072;
	// 826935C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826935CC: 390B9FF0  addi r8, r11, -0x6010
	ctx.r[8].s64 = ctx.r[11].s64 + -24592;
	// 826935D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826935D4: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826935D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826935DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826935E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826935E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826935E8: 386A0848  addi r3, r10, 0x848
	ctx.r[3].s64 = ctx.r[10].s64 + 2120;
	// 826935EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826935F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826935F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826935F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826935FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269360C: 4BDD3815  bl 0x82466e20
	ctx.lr = 0x82693610;
	sub_82466E20(ctx, base);
	// 82693610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269361C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693620 size=108
    let mut pc: u32 = 0x82693620;
    'dispatch: loop {
        match pc {
            0x82693620 => {
    //   block [0x82693620..0x8269368C)
	// 82693620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269362C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693634: 38EBA038  addi r7, r11, -0x5fc8
	ctx.r[7].s64 = ctx.r[11].s64 + -24520;
	// 82693638: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269363C: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82693640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693644: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269364C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693650: 386A0878  addi r3, r10, 0x878
	ctx.r[3].s64 = ctx.r[10].s64 + 2168;
	// 82693654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269365C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269366C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693678: 4BDD37A9  bl 0x82466e20
	ctx.lr = 0x8269367C;
	sub_82466E20(ctx, base);
	// 8269367C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693690 size=112
    let mut pc: u32 = 0x82693690;
    'dispatch: loop {
        match pc {
            0x82693690 => {
    //   block [0x82693690..0x82693700)
	// 82693690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269369C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826936A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826936A4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826936A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826936AC: 390BA080  addi r8, r11, -0x5f80
	ctx.r[8].s64 = ctx.r[11].s64 + -24448;
	// 826936B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826936B4: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826936B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826936BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826936C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826936C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826936C8: 386A08A8  addi r3, r10, 0x8a8
	ctx.r[3].s64 = ctx.r[10].s64 + 2216;
	// 826936CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826936D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826936D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826936D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826936DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826936E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826936E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826936E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826936EC: 4BDD3735  bl 0x82466e20
	ctx.lr = 0x826936F0;
	sub_82466E20(ctx, base);
	// 826936F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826936F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826936F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826936FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693700 size=108
    let mut pc: u32 = 0x82693700;
    'dispatch: loop {
        match pc {
            0x82693700 => {
    //   block [0x82693700..0x8269376C)
	// 82693700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269370C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693714: 38EBA098  addi r7, r11, -0x5f68
	ctx.r[7].s64 = ctx.r[11].s64 + -24424;
	// 82693718: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269371C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82693720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693724: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693728: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269372C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693730: 386A08D8  addi r3, r10, 0x8d8
	ctx.r[3].s64 = ctx.r[10].s64 + 2264;
	// 82693734: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269373C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269374C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693754: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693758: 4BDD36C9  bl 0x82466e20
	ctx.lr = 0x8269375C;
	sub_82466E20(ctx, base);
	// 8269375C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693770 size=112
    let mut pc: u32 = 0x82693770;
    'dispatch: loop {
        match pc {
            0x82693770 => {
    //   block [0x82693770..0x826937E0)
	// 82693770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269377C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693780: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693784: 38AA08A8  addi r5, r10, 0x8a8
	ctx.r[5].s64 = ctx.r[10].s64 + 2216;
	// 82693788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269378C: 390BA0E0  addi r8, r11, -0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + -24352;
	// 82693790: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82693794: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82693798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269379C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826937A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826937A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826937A8: 386A0908  addi r3, r10, 0x908
	ctx.r[3].s64 = ctx.r[10].s64 + 2312;
	// 826937AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826937B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826937B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826937B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826937BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826937C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826937C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826937C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826937CC: 4BDD3655  bl 0x82466e20
	ctx.lr = 0x826937D0;
	sub_82466E20(ctx, base);
	// 826937D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826937D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826937D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826937DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826937E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826937E0 size=100
    let mut pc: u32 = 0x826937E0;
    'dispatch: loop {
        match pc {
            0x826937E0 => {
    //   block [0x826937E0..0x82693844)
	// 826937E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826937E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826937E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826937EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826937F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826937F4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826937F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826937FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693800: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82693804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269380C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693814: 386A0938  addi r3, r10, 0x938
	ctx.r[3].s64 = ctx.r[10].s64 + 2360;
	// 82693818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269381C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693820: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82693824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269382C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693830: 4BDD35F1  bl 0x82466e20
	ctx.lr = 0x82693834;
	sub_82466E20(ctx, base);
	// 82693834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269383C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693848 size=112
    let mut pc: u32 = 0x82693848;
    'dispatch: loop {
        match pc {
            0x82693848 => {
    //   block [0x82693848..0x826938B8)
	// 82693848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269384C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693854: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693858: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269385C: 38AA0938  addi r5, r10, 0x938
	ctx.r[5].s64 = ctx.r[10].s64 + 2360;
	// 82693860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693864: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 82693868: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269386C: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82693870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269387C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693880: 386A0968  addi r3, r10, 0x968
	ctx.r[3].s64 = ctx.r[10].s64 + 2408;
	// 82693884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269388C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269389C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826938A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826938A4: 4BDD357D  bl 0x82466e20
	ctx.lr = 0x826938A8;
	sub_82466E20(ctx, base);
	// 826938A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826938AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826938B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826938B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826938B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826938B8 size=108
    let mut pc: u32 = 0x826938B8;
    'dispatch: loop {
        match pc {
            0x826938B8 => {
    //   block [0x826938B8..0x82693924)
	// 826938B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826938BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826938C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826938C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826938C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826938CC: 38EBA1A0  addi r7, r11, -0x5e60
	ctx.r[7].s64 = ctx.r[11].s64 + -24160;
	// 826938D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826938D4: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826938D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826938DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826938E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826938E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826938E8: 386A0998  addi r3, r10, 0x998
	ctx.r[3].s64 = ctx.r[10].s64 + 2456;
	// 826938EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826938F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826938F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826938F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826938FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269390C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693910: 4BDD3511  bl 0x82466e20
	ctx.lr = 0x82693914;
	sub_82466E20(ctx, base);
	// 82693914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269391C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693928 size=112
    let mut pc: u32 = 0x82693928;
    'dispatch: loop {
        match pc {
            0x82693928 => {
    //   block [0x82693928..0x82693998)
	// 82693928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269392C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693938: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269393C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693944: 390BA1D0  addi r8, r11, -0x5e30
	ctx.r[8].s64 = ctx.r[11].s64 + -24112;
	// 82693948: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269394C: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82693950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269395C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693960: 386A09C8  addi r3, r10, 0x9c8
	ctx.r[3].s64 = ctx.r[10].s64 + 2504;
	// 82693964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269396C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269397C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693984: 4BDD349D  bl 0x82466e20
	ctx.lr = 0x82693988;
	sub_82466E20(ctx, base);
	// 82693988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269398C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693998 size=112
    let mut pc: u32 = 0x82693998;
    'dispatch: loop {
        match pc {
            0x82693998 => {
    //   block [0x82693998..0x82693A08)
	// 82693998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269399C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826939A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826939A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826939A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826939AC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 826939B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826939B4: 390BA218  addi r8, r11, -0x5de8
	ctx.r[8].s64 = ctx.r[11].s64 + -24040;
	// 826939B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826939BC: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826939C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826939C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826939C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826939CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826939D0: 386A09F8  addi r3, r10, 0x9f8
	ctx.r[3].s64 = ctx.r[10].s64 + 2552;
	// 826939D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826939D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826939DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826939E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826939E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826939E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826939EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826939F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826939F4: 4BDD342D  bl 0x82466e20
	ctx.lr = 0x826939F8;
	sub_82466E20(ctx, base);
	// 826939F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826939FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693A08 size=100
    let mut pc: u32 = 0x82693A08;
    'dispatch: loop {
        match pc {
            0x82693A08 => {
    //   block [0x82693A08..0x82693A6C)
	// 82693A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693A14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693A1C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693A28: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82693A2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693A3C: 386A0A28  addi r3, r10, 0xa28
	ctx.r[3].s64 = ctx.r[10].s64 + 2600;
	// 82693A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693A44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693A48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82693A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693A50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82693A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693A58: 4BDD33C9  bl 0x82466e20
	ctx.lr = 0x82693A5C;
	sub_82466E20(ctx, base);
	// 82693A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693A70 size=112
    let mut pc: u32 = 0x82693A70;
    'dispatch: loop {
        match pc {
            0x82693A70 => {
    //   block [0x82693A70..0x82693AE0)
	// 82693A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693A7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693A80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693A84: 38AA0A28  addi r5, r10, 0xa28
	ctx.r[5].s64 = ctx.r[10].s64 + 2600;
	// 82693A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693A8C: 390BA260  addi r8, r11, -0x5da0
	ctx.r[8].s64 = ctx.r[11].s64 + -23968;
	// 82693A90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82693A94: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82693A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693A9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693AA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693AA8: 386A0A58  addi r3, r10, 0xa58
	ctx.r[3].s64 = ctx.r[10].s64 + 2648;
	// 82693AAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693ACC: 4BDD3355  bl 0x82466e20
	ctx.lr = 0x82693AD0;
	sub_82466E20(ctx, base);
	// 82693AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693AE0 size=112
    let mut pc: u32 = 0x82693AE0;
    'dispatch: loop {
        match pc {
            0x82693AE0 => {
    //   block [0x82693AE0..0x82693B50)
	// 82693AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693AEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693AF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693AF4: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693AFC: 390BA2A8  addi r8, r11, -0x5d58
	ctx.r[8].s64 = ctx.r[11].s64 + -23896;
	// 82693B00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82693B04: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82693B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693B0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693B10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693B18: 386A0A88  addi r3, r10, 0xa88
	ctx.r[3].s64 = ctx.r[10].s64 + 2696;
	// 82693B1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693B3C: 4BDD32E5  bl 0x82466e20
	ctx.lr = 0x82693B40;
	sub_82466E20(ctx, base);
	// 82693B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693B50 size=112
    let mut pc: u32 = 0x82693B50;
    'dispatch: loop {
        match pc {
            0x82693B50 => {
    //   block [0x82693B50..0x82693BC0)
	// 82693B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693B5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693B60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693B64: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82693B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693B6C: 390BA2C0  addi r8, r11, -0x5d40
	ctx.r[8].s64 = ctx.r[11].s64 + -23872;
	// 82693B70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82693B74: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82693B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693B7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693B88: 386A0AB8  addi r3, r10, 0xab8
	ctx.r[3].s64 = ctx.r[10].s64 + 2744;
	// 82693B8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693B9C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82693BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693BAC: 4BDD3275  bl 0x82466e20
	ctx.lr = 0x82693BB0;
	sub_82466E20(ctx, base);
	// 82693BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693BC0 size=112
    let mut pc: u32 = 0x82693BC0;
    'dispatch: loop {
        match pc {
            0x82693BC0 => {
    //   block [0x82693BC0..0x82693C30)
	// 82693BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693BCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693BD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693BD4: 38AA0A88  addi r5, r10, 0xa88
	ctx.r[5].s64 = ctx.r[10].s64 + 2696;
	// 82693BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82693BDC: 390BA2D8  addi r8, r11, -0x5d28
	ctx.r[8].s64 = ctx.r[11].s64 + -23848;
	// 82693BE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82693BE4: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82693BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693BEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693BF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693BF8: 386A0AE8  addi r3, r10, 0xae8
	ctx.r[3].s64 = ctx.r[10].s64 + 2792;
	// 82693BFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693C1C: 4BDD3205  bl 0x82466e20
	ctx.lr = 0x82693C20;
	sub_82466E20(ctx, base);
	// 82693C20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693C30 size=72
    let mut pc: u32 = 0x82693C30;
    'dispatch: loop {
        match pc {
            0x82693C30 => {
    //   block [0x82693C30..0x82693C78)
	// 82693C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693C3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82693C40: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82693C44: 38CB6850  addi r6, r11, 0x6850
	ctx.r[6].s64 = ctx.r[11].s64 + 26704;
	// 82693C48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82693C4C: 388B71F8  addi r4, r11, 0x71f8
	ctx.r[4].s64 = ctx.r[11].s64 + 29176;
	// 82693C50: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82693C54: 386B0B18  addi r3, r11, 0xb18
	ctx.r[3].s64 = ctx.r[11].s64 + 2840;
	// 82693C58: 4BDE7E31  bl 0x8247ba88
	ctx.lr = 0x82693C5C;
	sub_8247BA88(ctx, base);
	// 82693C5C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82693C60: 386BCE70  addi r3, r11, -0x3190
	ctx.r[3].s64 = ctx.r[11].s64 + -12688;
	// 82693C64: 4BE9EED5  bl 0x82532b38
	ctx.lr = 0x82693C68;
	sub_82532B38(ctx, base);
	// 82693C68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82693C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693C78 size=108
    let mut pc: u32 = 0x82693C78;
    'dispatch: loop {
        match pc {
            0x82693C78 => {
    //   block [0x82693C78..0x82693CE4)
	// 82693C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693C84: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693C88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693C8C: 38EBABC8  addi r7, r11, -0x5438
	ctx.r[7].s64 = ctx.r[11].s64 + -21560;
	// 82693C90: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82693C94: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 82693C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693C9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693CA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693CA8: 386A0B30  addi r3, r10, 0xb30
	ctx.r[3].s64 = ctx.r[10].s64 + 2864;
	// 82693CAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693CD0: 4BDD3151  bl 0x82466e20
	ctx.lr = 0x82693CD4;
	sub_82466E20(ctx, base);
	// 82693CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82693CE8 size=24
    let mut pc: u32 = 0x82693CE8;
    'dispatch: loop {
        match pc {
            0x82693CE8 => {
    //   block [0x82693CE8..0x82693D00)
	// 82693CE8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693CEC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82693CF0: 394A4830  addi r10, r10, 0x4830
	ctx.r[10].s64 = ctx.r[10].s64 + 18480;
	// 82693CF4: 816BAC40  lwz r11, -0x53c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21440 as u32) ) } as u64;
	// 82693CF8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82693CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693D00 size=112
    let mut pc: u32 = 0x82693D00;
    'dispatch: loop {
        match pc {
            0x82693D00 => {
    //   block [0x82693D00..0x82693D70)
	// 82693D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693D0C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82693D10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693D14: 392A81A4  addi r9, r10, -0x7e5c
	ctx.r[9].s64 = ctx.r[10].s64 + -32348;
	// 82693D18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693D1C: 390B4830  addi r8, r11, 0x4830
	ctx.r[8].s64 = ctx.r[11].s64 + 18480;
	// 82693D20: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82693D24: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 82693D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693D2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693D38: 386A0B60  addi r3, r10, 0xb60
	ctx.r[3].s64 = ctx.r[10].s64 + 2912;
	// 82693D3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82693D40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82693D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693D54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693D5C: 4BDD30C5  bl 0x82466e20
	ctx.lr = 0x82693D60;
	sub_82466E20(ctx, base);
	// 82693D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693D70 size=108
    let mut pc: u32 = 0x82693D70;
    'dispatch: loop {
        match pc {
            0x82693D70 => {
    //   block [0x82693D70..0x82693DDC)
	// 82693D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693D7C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693D80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693D84: 38EBAC44  addi r7, r11, -0x53bc
	ctx.r[7].s64 = ctx.r[11].s64 + -21436;
	// 82693D88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82693D8C: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 82693D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693D94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693D98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693DA0: 386A0B90  addi r3, r10, 0xb90
	ctx.r[3].s64 = ctx.r[10].s64 + 2960;
	// 82693DA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693DC8: 4BDD3059  bl 0x82466e20
	ctx.lr = 0x82693DCC;
	sub_82466E20(ctx, base);
	// 82693DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693DE0 size=108
    let mut pc: u32 = 0x82693DE0;
    'dispatch: loop {
        match pc {
            0x82693DE0 => {
    //   block [0x82693DE0..0x82693E4C)
	// 82693DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693DEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693DF4: 38EBAC74  addi r7, r11, -0x538c
	ctx.r[7].s64 = ctx.r[11].s64 + -21388;
	// 82693DF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82693DFC: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 82693E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693E04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693E08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693E10: 386A0BC0  addi r3, r10, 0xbc0
	ctx.r[3].s64 = ctx.r[10].s64 + 3008;
	// 82693E14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693E38: 4BDD2FE9  bl 0x82466e20
	ctx.lr = 0x82693E3C;
	sub_82466E20(ctx, base);
	// 82693E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82693E50 size=24
    let mut pc: u32 = 0x82693E50;
    'dispatch: loop {
        match pc {
            0x82693E50 => {
    //   block [0x82693E50..0x82693E68)
	// 82693E50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693E54: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82693E58: 394A4890  addi r10, r10, 0x4890
	ctx.r[10].s64 = ctx.r[10].s64 + 18576;
	// 82693E5C: 816BACA4  lwz r11, -0x535c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21340 as u32) ) } as u64;
	// 82693E60: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82693E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693E68 size=116
    let mut pc: u32 = 0x82693E68;
    'dispatch: loop {
        match pc {
            0x82693E68 => {
    //   block [0x82693E68..0x82693EDC)
	// 82693E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693E74: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693E78: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82693E7C: 390B4890  addi r8, r11, 0x4890
	ctx.r[8].s64 = ctx.r[11].s64 + 18576;
	// 82693E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693E84: 392A81E8  addi r9, r10, -0x7e18
	ctx.r[9].s64 = ctx.r[10].s64 + -32280;
	// 82693E88: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693E8C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82693E90: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82693E94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693E9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693EAC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82693EB0: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 82693EB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82693EB8: 386B0BF0  addi r3, r11, 0xbf0
	ctx.r[3].s64 = ctx.r[11].s64 + 3056;
	// 82693EBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82693EC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693EC8: 4BDD2F59  bl 0x82466e20
	ctx.lr = 0x82693ECC;
	sub_82466E20(ctx, base);
	// 82693ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693EE0 size=108
    let mut pc: u32 = 0x82693EE0;
    'dispatch: loop {
        match pc {
            0x82693EE0 => {
    //   block [0x82693EE0..0x82693F4C)
	// 82693EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693EEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693EF4: 38EBACA8  addi r7, r11, -0x5358
	ctx.r[7].s64 = ctx.r[11].s64 + -21336;
	// 82693EF8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82693EFC: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 82693F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693F04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82693F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693F10: 386A0C20  addi r3, r10, 0xc20
	ctx.r[3].s64 = ctx.r[10].s64 + 3104;
	// 82693F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82693F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82693F38: 4BDD2EE9  bl 0x82466e20
	ctx.lr = 0x82693F3C;
	sub_82466E20(ctx, base);
	// 82693F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693F50 size=112
    let mut pc: u32 = 0x82693F50;
    'dispatch: loop {
        match pc {
            0x82693F50 => {
    //   block [0x82693F50..0x82693FC0)
	// 82693F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693F5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693F60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693F64: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 82693F68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693F6C: 390BAD38  addi r8, r11, -0x52c8
	ctx.r[8].s64 = ctx.r[11].s64 + -21192;
	// 82693F70: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82693F74: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82693F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693F7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693F80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693F88: 386A0C50  addi r3, r10, 0xc50
	ctx.r[3].s64 = ctx.r[10].s64 + 3152;
	// 82693F8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82693F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82693F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82693F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82693F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82693FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82693FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82693FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82693FAC: 4BDD2E75  bl 0x82466e20
	ctx.lr = 0x82693FB0;
	sub_82466E20(ctx, base);
	// 82693FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82693FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82693FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82693FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82693FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82693FC0 size=112
    let mut pc: u32 = 0x82693FC0;
    'dispatch: loop {
        match pc {
            0x82693FC0 => {
    //   block [0x82693FC0..0x82694030)
	// 82693FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82693FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82693FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82693FCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693FD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82693FD4: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 82693FD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82693FDC: 390BAE58  addi r8, r11, -0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + -20904;
	// 82693FE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82693FE4: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 82693FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82693FEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82693FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82693FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82693FF8: 386A0C80  addi r3, r10, 0xc80
	ctx.r[3].s64 = ctx.r[10].s64 + 3200;
	// 82693FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269400C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269401C: 4BDD2E05  bl 0x82466e20
	ctx.lr = 0x82694020;
	sub_82466E20(ctx, base);
	// 82694020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269402C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82694030 size=44
    let mut pc: u32 = 0x82694030;
    'dispatch: loop {
        match pc {
            0x82694030 => {
    //   block [0x82694030..0x8269405C)
	// 82694030: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694034: 814BAE88  lwz r10, -0x5178(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20856 as u32) ) } as u64;
	// 82694038: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269403C: 396B4920  addi r11, r11, 0x4920
	ctx.r[11].s64 = ctx.r[11].s64 + 18720;
	// 82694040: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 82694044: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694048: 814AAE8C  lwz r10, -0x5174(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20852 as u32) ) } as u64;
	// 8269404C: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 82694050: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82694054: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82694058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694060 size=112
    let mut pc: u32 = 0x82694060;
    'dispatch: loop {
        match pc {
            0x82694060 => {
    //   block [0x82694060..0x826940D0)
	// 82694060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269406C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82694070: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694074: 392A823C  addi r9, r10, -0x7dc4
	ctx.r[9].s64 = ctx.r[10].s64 + -32196;
	// 82694078: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269407C: 390B4920  addi r8, r11, 0x4920
	ctx.r[8].s64 = ctx.r[11].s64 + 18720;
	// 82694080: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82694084: 388AB160  addi r4, r10, -0x4ea0
	ctx.r[4].s64 = ctx.r[10].s64 + -20128;
	// 82694088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269408C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694098: 386A0CB0  addi r3, r10, 0xcb0
	ctx.r[3].s64 = ctx.r[10].s64 + 3248;
	// 8269409C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826940A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826940A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826940A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826940AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826940B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826940B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826940B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826940BC: 4BDD2D65  bl 0x82466e20
	ctx.lr = 0x826940C0;
	sub_82466E20(ctx, base);
	// 826940C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826940C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826940C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826940CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826940D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826940D0 size=108
    let mut pc: u32 = 0x826940D0;
    'dispatch: loop {
        match pc {
            0x826940D0 => {
    //   block [0x826940D0..0x8269413C)
	// 826940D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826940D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826940D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826940DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826940E0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826940E4: 38EBAE90  addi r7, r11, -0x5170
	ctx.r[7].s64 = ctx.r[11].s64 + -20848;
	// 826940E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826940EC: 388AB194  addi r4, r10, -0x4e6c
	ctx.r[4].s64 = ctx.r[10].s64 + -20076;
	// 826940F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826940F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826940F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826940FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694100: 386A0CE0  addi r3, r10, 0xce0
	ctx.r[3].s64 = ctx.r[10].s64 + 3296;
	// 82694104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269410C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269411C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694128: 4BDD2CF9  bl 0x82466e20
	ctx.lr = 0x8269412C;
	sub_82466E20(ctx, base);
	// 8269412C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694140 size=112
    let mut pc: u32 = 0x82694140;
    'dispatch: loop {
        match pc {
            0x82694140 => {
    //   block [0x82694140..0x826941B0)
	// 82694140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269414C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694150: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694154: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 82694158: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269415C: 390BAEC0  addi r8, r11, -0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + -20800;
	// 82694160: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 82694164: 388AB1CC  addi r4, r10, -0x4e34
	ctx.r[4].s64 = ctx.r[10].s64 + -20020;
	// 82694168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269416C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694178: 386A0D10  addi r3, r10, 0xd10
	ctx.r[3].s64 = ctx.r[10].s64 + 3344;
	// 8269417C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269418C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269419C: 4BDD2C85  bl 0x82466e20
	ctx.lr = 0x826941A0;
	sub_82466E20(ctx, base);
	// 826941A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826941A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826941A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826941AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826941B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826941B0 size=108
    let mut pc: u32 = 0x826941B0;
    'dispatch: loop {
        match pc {
            0x826941B0 => {
    //   block [0x826941B0..0x8269421C)
	// 826941B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826941B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826941B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826941BC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826941C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826941C4: 38EBAFF8  addi r7, r11, -0x5008
	ctx.r[7].s64 = ctx.r[11].s64 + -20488;
	// 826941C8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826941CC: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826941D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826941D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826941D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826941DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826941E0: 386A0D40  addi r3, r10, 0xd40
	ctx.r[3].s64 = ctx.r[10].s64 + 3392;
	// 826941E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826941E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826941EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826941F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826941F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826941F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826941FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694208: 4BDD2C19  bl 0x82466e20
	ctx.lr = 0x8269420C;
	sub_82466E20(ctx, base);
	// 8269420C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694220 size=108
    let mut pc: u32 = 0x82694220;
    'dispatch: loop {
        match pc {
            0x82694220 => {
    //   block [0x82694220..0x8269428C)
	// 82694220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269422C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694234: 38EBB0E8  addi r7, r11, -0x4f18
	ctx.r[7].s64 = ctx.r[11].s64 + -20248;
	// 82694238: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269423C: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 82694240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269424C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694250: 386A0D70  addi r3, r10, 0xd70
	ctx.r[3].s64 = ctx.r[10].s64 + 3440;
	// 82694254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269425C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269426C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694278: 4BDD2BA9  bl 0x82466e20
	ctx.lr = 0x8269427C;
	sub_82466E20(ctx, base);
	// 8269427C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694290 size=112
    let mut pc: u32 = 0x82694290;
    'dispatch: loop {
        match pc {
            0x82694290 => {
    //   block [0x82694290..0x82694300)
	// 82694290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269429C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826942A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826942A4: 38AA0BF0  addi r5, r10, 0xbf0
	ctx.r[5].s64 = ctx.r[10].s64 + 3056;
	// 826942A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826942AC: 390BB178  addi r8, r11, -0x4e88
	ctx.r[8].s64 = ctx.r[11].s64 + -20104;
	// 826942B0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826942B4: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 826942B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826942BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826942C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826942C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826942C8: 386A0DA0  addi r3, r10, 0xda0
	ctx.r[3].s64 = ctx.r[10].s64 + 3488;
	// 826942CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826942D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826942D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826942D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826942DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826942E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826942E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826942E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826942EC: 4BDD2B35  bl 0x82466e20
	ctx.lr = 0x826942F0;
	sub_82466E20(ctx, base);
	// 826942F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826942F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826942F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826942FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694300 size=108
    let mut pc: u32 = 0x82694300;
    'dispatch: loop {
        match pc {
            0x82694300 => {
    //   block [0x82694300..0x8269436C)
	// 82694300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269430C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694314: 38EBB268  addi r7, r11, -0x4d98
	ctx.r[7].s64 = ctx.r[11].s64 + -19864;
	// 82694318: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269431C: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 82694320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269432C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694330: 386A0DD0  addi r3, r10, 0xdd0
	ctx.r[3].s64 = ctx.r[10].s64 + 3536;
	// 82694334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269433C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269434C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694358: 4BDD2AC9  bl 0x82466e20
	ctx.lr = 0x8269435C;
	sub_82466E20(ctx, base);
	// 8269435C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694370 size=108
    let mut pc: u32 = 0x82694370;
    'dispatch: loop {
        match pc {
            0x82694370 => {
    //   block [0x82694370..0x826943DC)
	// 82694370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269437C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694380: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694384: 38EBB280  addi r7, r11, -0x4d80
	ctx.r[7].s64 = ctx.r[11].s64 + -19840;
	// 82694388: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269438C: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 82694390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694394: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269439C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826943A0: 386A0E00  addi r3, r10, 0xe00
	ctx.r[3].s64 = ctx.r[10].s64 + 3584;
	// 826943A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826943A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826943AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826943B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826943B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826943B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826943BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826943C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826943C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826943C8: 4BDD2A59  bl 0x82466e20
	ctx.lr = 0x826943CC;
	sub_82466E20(ctx, base);
	// 826943CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826943D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826943D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826943D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826943E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826943E0 size=116
    let mut pc: u32 = 0x826943E0;
    'dispatch: loop {
        match pc {
            0x826943E0 => {
    //   block [0x826943E0..0x82694454)
	// 826943E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826943E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826943E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826943EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826943F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826943F4: 390BB2E4  addi r8, r11, -0x4d1c
	ctx.r[8].s64 = ctx.r[11].s64 + -19740;
	// 826943F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826943FC: 392A827C  addi r9, r10, -0x7d84
	ctx.r[9].s64 = ctx.r[10].s64 + -32132;
	// 82694400: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694404: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82694408: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269440C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694414: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269441C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694424: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82694428: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 8269442C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694430: 386B0E30  addi r3, r11, 0xe30
	ctx.r[3].s64 = ctx.r[11].s64 + 3632;
	// 82694434: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694438: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269443C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694440: 4BDD29E1  bl 0x82466e20
	ctx.lr = 0x82694444;
	sub_82466E20(ctx, base);
	// 82694444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269444C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694458 size=108
    let mut pc: u32 = 0x82694458;
    'dispatch: loop {
        match pc {
            0x82694458 => {
    //   block [0x82694458..0x826944C4)
	// 82694458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694464: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269446C: 38EBB300  addi r7, r11, -0x4d00
	ctx.r[7].s64 = ctx.r[11].s64 + -19712;
	// 82694470: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82694474: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 82694478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269447C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694488: 386A0E60  addi r3, r10, 0xe60
	ctx.r[3].s64 = ctx.r[10].s64 + 3680;
	// 8269448C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269449C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826944A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826944A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826944A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826944AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826944B0: 4BDD2971  bl 0x82466e20
	ctx.lr = 0x826944B4;
	sub_82466E20(ctx, base);
	// 826944B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826944B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826944BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826944C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826944C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826944C8 size=108
    let mut pc: u32 = 0x826944C8;
    'dispatch: loop {
        match pc {
            0x826944C8 => {
    //   block [0x826944C8..0x82694534)
	// 826944C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826944CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826944D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826944D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826944D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826944DC: 38EBB348  addi r7, r11, -0x4cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -19640;
	// 826944E0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826944E4: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826944E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826944EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826944F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826944F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826944F8: 386A0E90  addi r3, r10, 0xe90
	ctx.r[3].s64 = ctx.r[10].s64 + 3728;
	// 826944FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269450C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269451C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694520: 4BDD2901  bl 0x82466e20
	ctx.lr = 0x82694524;
	sub_82466E20(ctx, base);
	// 82694524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269452C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694538 size=108
    let mut pc: u32 = 0x82694538;
    'dispatch: loop {
        match pc {
            0x82694538 => {
    //   block [0x82694538..0x826945A4)
	// 82694538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269453C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694544: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269454C: 38EBB3D8  addi r7, r11, -0x4c28
	ctx.r[7].s64 = ctx.r[11].s64 + -19496;
	// 82694550: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82694554: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 82694558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269455C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694568: 386A0EC0  addi r3, r10, 0xec0
	ctx.r[3].s64 = ctx.r[10].s64 + 3776;
	// 8269456C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269457C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269458C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694590: 4BDD2891  bl 0x82466e20
	ctx.lr = 0x82694594;
	sub_82466E20(ctx, base);
	// 82694594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269459C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826945A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826945A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826945A8 size=100
    let mut pc: u32 = 0x826945A8;
    'dispatch: loop {
        match pc {
            0x826945A8 => {
    //   block [0x826945A8..0x8269460C)
	// 826945A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826945AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826945B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826945B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826945B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826945BC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826945C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826945C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826945C8: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826945CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826945D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826945D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826945D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826945DC: 386A0EF0  addi r3, r10, 0xef0
	ctx.r[3].s64 = ctx.r[10].s64 + 3824;
	// 826945E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826945E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826945E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826945EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826945F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826945F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826945F8: 4BDD2829  bl 0x82466e20
	ctx.lr = 0x826945FC;
	sub_82466E20(ctx, base);
	// 826945FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694610 size=112
    let mut pc: u32 = 0x82694610;
    'dispatch: loop {
        match pc {
            0x82694610 => {
    //   block [0x82694610..0x82694680)
	// 82694610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269461C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694620: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694624: 38AA0EF0  addi r5, r10, 0xef0
	ctx.r[5].s64 = ctx.r[10].s64 + 3824;
	// 82694628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269462C: 390BB468  addi r8, r11, -0x4b98
	ctx.r[8].s64 = ctx.r[11].s64 + -19352;
	// 82694630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82694634: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 82694638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269463C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694648: 386A0F20  addi r3, r10, 0xf20
	ctx.r[3].s64 = ctx.r[10].s64 + 3872;
	// 8269464C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269465C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269466C: 4BDD27B5  bl 0x82466e20
	ctx.lr = 0x82694670;
	sub_82466E20(ctx, base);
	// 82694670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269467C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694680 size=108
    let mut pc: u32 = 0x82694680;
    'dispatch: loop {
        match pc {
            0x82694680 => {
    //   block [0x82694680..0x826946EC)
	// 82694680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269468C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694694: 38EBB4C8  addi r7, r11, -0x4b38
	ctx.r[7].s64 = ctx.r[11].s64 + -19256;
	// 82694698: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269469C: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826946A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826946A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826946A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826946AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826946B0: 386A0F50  addi r3, r10, 0xf50
	ctx.r[3].s64 = ctx.r[10].s64 + 3920;
	// 826946B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826946B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826946BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826946C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826946C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826946C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826946CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826946D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826946D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826946D8: 4BDD2749  bl 0x82466e20
	ctx.lr = 0x826946DC;
	sub_82466E20(ctx, base);
	// 826946DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826946E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826946E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826946E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826946F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826946F0 size=108
    let mut pc: u32 = 0x826946F0;
    'dispatch: loop {
        match pc {
            0x826946F0 => {
    //   block [0x826946F0..0x8269475C)
	// 826946F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826946F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826946F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826946FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694704: 38EBB4F8  addi r7, r11, -0x4b08
	ctx.r[7].s64 = ctx.r[11].s64 + -19208;
	// 82694708: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269470C: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 82694710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694714: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269471C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694720: 386A0F80  addi r3, r10, 0xf80
	ctx.r[3].s64 = ctx.r[10].s64 + 3968;
	// 82694724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269472C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269473C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694748: 4BDD26D9  bl 0x82466e20
	ctx.lr = 0x8269474C;
	sub_82466E20(ctx, base);
	// 8269474C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694760 size=108
    let mut pc: u32 = 0x82694760;
    'dispatch: loop {
        match pc {
            0x82694760 => {
    //   block [0x82694760..0x826947CC)
	// 82694760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269476C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694770: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694774: 38EBB558  addi r7, r11, -0x4aa8
	ctx.r[7].s64 = ctx.r[11].s64 + -19112;
	// 82694778: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8269477C: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 82694780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694784: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269478C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694790: 386A0FB0  addi r3, r10, 0xfb0
	ctx.r[3].s64 = ctx.r[10].s64 + 4016;
	// 82694794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269479C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826947A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826947A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826947A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826947AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826947B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826947B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826947B8: 4BDD2669  bl 0x82466e20
	ctx.lr = 0x826947BC;
	sub_82466E20(ctx, base);
	// 826947BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826947C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826947C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826947C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826947D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826947D0 size=112
    let mut pc: u32 = 0x826947D0;
    'dispatch: loop {
        match pc {
            0x826947D0 => {
    //   block [0x826947D0..0x82694840)
	// 826947D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826947D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826947D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826947DC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826947E0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826947E4: 38EAB5D0  addi r7, r10, -0x4a30
	ctx.r[7].s64 = ctx.r[10].s64 + -18992;
	// 826947E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826947EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826947F0: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826947F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826947F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826947FC: 396B8290  addi r11, r11, -0x7d70
	ctx.r[11].s64 = ctx.r[11].s64 + -32112;
	// 82694800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269480C: 386A0FE0  addi r3, r10, 0xfe0
	ctx.r[3].s64 = ctx.r[10].s64 + 4064;
	// 82694810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694814: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82694818: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269481C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82694820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694824: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694828: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269482C: 4BDD25F5  bl 0x82466e20
	ctx.lr = 0x82694830;
	sub_82466E20(ctx, base);
	// 82694830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694840 size=96
    let mut pc: u32 = 0x82694840;
    'dispatch: loop {
        match pc {
            0x82694840 => {
    //   block [0x82694840..0x826948A0)
	// 82694840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269484C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694854: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 82694858: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269485C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694860: 386A1010  addi r3, r10, 0x1010
	ctx.r[3].s64 = ctx.r[10].s64 + 4112;
	// 82694864: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269486C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269487C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694880: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82694884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694888: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269488C: 4BDD2595  bl 0x82466e20
	ctx.lr = 0x82694890;
	sub_82466E20(ctx, base);
	// 82694890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269489C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826948A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826948A0 size=112
    let mut pc: u32 = 0x826948A0;
    'dispatch: loop {
        match pc {
            0x826948A0 => {
    //   block [0x826948A0..0x82694910)
	// 826948A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826948A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826948A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826948AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826948B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826948B4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826948B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826948BC: 390BB6F0  addi r8, r11, -0x4910
	ctx.r[8].s64 = ctx.r[11].s64 + -18704;
	// 826948C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826948C4: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826948C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826948CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826948D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826948D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826948D8: 386A1040  addi r3, r10, 0x1040
	ctx.r[3].s64 = ctx.r[10].s64 + 4160;
	// 826948DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826948E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826948E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826948E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826948EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826948F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826948F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826948F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826948FC: 4BDD2525  bl 0x82466e20
	ctx.lr = 0x82694900;
	sub_82466E20(ctx, base);
	// 82694900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269490C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82694910 size=24
    let mut pc: u32 = 0x82694910;
    'dispatch: loop {
        match pc {
            0x82694910 => {
    //   block [0x82694910..0x82694928)
	// 82694910: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694914: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694918: 394A4A40  addi r10, r10, 0x4a40
	ctx.r[10].s64 = ctx.r[10].s64 + 19008;
	// 8269491C: 816BB2FC  lwz r11, -0x4d04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19716 as u32) ) } as u64;
	// 82694920: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82694924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694928 size=116
    let mut pc: u32 = 0x82694928;
    'dispatch: loop {
        match pc {
            0x82694928 => {
    //   block [0x82694928..0x8269499C)
	// 82694928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269492C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694934: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694938: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269493C: 390B4A40  addi r8, r11, 0x4a40
	ctx.r[8].s64 = ctx.r[11].s64 + 19008;
	// 82694940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694944: 392A830C  addi r9, r10, -0x7cf4
	ctx.r[9].s64 = ctx.r[10].s64 + -31988;
	// 82694948: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269494C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82694950: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694954: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269495C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269496C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82694970: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 82694974: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694978: 386B1070  addi r3, r11, 0x1070
	ctx.r[3].s64 = ctx.r[11].s64 + 4208;
	// 8269497C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694980: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694988: 4BDD2499  bl 0x82466e20
	ctx.lr = 0x8269498C;
	sub_82466E20(ctx, base);
	// 8269498C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826949A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826949A0 size=24
    let mut pc: u32 = 0x826949A0;
    'dispatch: loop {
        match pc {
            0x826949A0 => {
    //   block [0x826949A0..0x826949B8)
	// 826949A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826949A4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826949A8: 394A4B18  addi r10, r10, 0x4b18
	ctx.r[10].s64 = ctx.r[10].s64 + 19224;
	// 826949AC: 816BB758  lwz r11, -0x48a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18600 as u32) ) } as u64;
	// 826949B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826949B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826949B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826949B8 size=116
    let mut pc: u32 = 0x826949B8;
    'dispatch: loop {
        match pc {
            0x826949B8 => {
    //   block [0x826949B8..0x82694A2C)
	// 826949B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826949BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826949C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826949C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826949C8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826949CC: 390B4B18  addi r8, r11, 0x4b18
	ctx.r[8].s64 = ctx.r[11].s64 + 19224;
	// 826949D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826949D4: 392A83D0  addi r9, r10, -0x7c30
	ctx.r[9].s64 = ctx.r[10].s64 + -31792;
	// 826949D8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826949DC: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826949E0: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826949E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826949E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826949EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826949F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826949F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826949F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826949FC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82694A00: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 82694A04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694A08: 386B10A0  addi r3, r11, 0x10a0
	ctx.r[3].s64 = ctx.r[11].s64 + 4256;
	// 82694A0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694A10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694A18: 4BDD2409  bl 0x82466e20
	ctx.lr = 0x82694A1C;
	sub_82466E20(ctx, base);
	// 82694A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694A30 size=112
    let mut pc: u32 = 0x82694A30;
    'dispatch: loop {
        match pc {
            0x82694A30 => {
    //   block [0x82694A30..0x82694AA0)
	// 82694A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694A3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694A40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694A44: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694A48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694A4C: 390BB760  addi r8, r11, -0x48a0
	ctx.r[8].s64 = ctx.r[11].s64 + -18592;
	// 82694A50: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82694A54: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 82694A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694A5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694A68: 386A10D0  addi r3, r10, 0x10d0
	ctx.r[3].s64 = ctx.r[10].s64 + 4304;
	// 82694A6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694A8C: 4BDD2395  bl 0x82466e20
	ctx.lr = 0x82694A90;
	sub_82466E20(ctx, base);
	// 82694A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694AA0 size=112
    let mut pc: u32 = 0x82694AA0;
    'dispatch: loop {
        match pc {
            0x82694AA0 => {
    //   block [0x82694AA0..0x82694B10)
	// 82694AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694AAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694AB0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694AB4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694ABC: 390BB7A8  addi r8, r11, -0x4858
	ctx.r[8].s64 = ctx.r[11].s64 + -18520;
	// 82694AC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82694AC4: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 82694AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694ACC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694AD8: 386A1100  addi r3, r10, 0x1100
	ctx.r[3].s64 = ctx.r[10].s64 + 4352;
	// 82694ADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694AFC: 4BDD2325  bl 0x82466e20
	ctx.lr = 0x82694B00;
	sub_82466E20(ctx, base);
	// 82694B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694B10 size=108
    let mut pc: u32 = 0x82694B10;
    'dispatch: loop {
        match pc {
            0x82694B10 => {
    //   block [0x82694B10..0x82694B7C)
	// 82694B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694B1C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694B24: 38EBB7F0  addi r7, r11, -0x4810
	ctx.r[7].s64 = ctx.r[11].s64 + -18448;
	// 82694B28: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82694B2C: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 82694B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694B40: 386A1130  addi r3, r10, 0x1130
	ctx.r[3].s64 = ctx.r[10].s64 + 4400;
	// 82694B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694B68: 4BDD22B9  bl 0x82466e20
	ctx.lr = 0x82694B6C;
	sub_82466E20(ctx, base);
	// 82694B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694B80 size=112
    let mut pc: u32 = 0x82694B80;
    'dispatch: loop {
        match pc {
            0x82694B80 => {
    //   block [0x82694B80..0x82694BF0)
	// 82694B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694B8C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82694B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694B94: 392B8404  addi r9, r11, -0x7bfc
	ctx.r[9].s64 = ctx.r[11].s64 + -31740;
	// 82694B98: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82694B9C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82694BA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694BA4: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82694BA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694BAC: 396BB898  addi r11, r11, -0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + -18280;
	// 82694BB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82694BB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694BB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82694BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694BC0: 386A1160  addi r3, r10, 0x1160
	ctx.r[3].s64 = ctx.r[10].s64 + 4448;
	// 82694BC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694BC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82694BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694BD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82694BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694BD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82694BDC: 4BDD2245  bl 0x82466e20
	ctx.lr = 0x82694BE0;
	sub_82466E20(ctx, base);
	// 82694BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694BF0 size=112
    let mut pc: u32 = 0x82694BF0;
    'dispatch: loop {
        match pc {
            0x82694BF0 => {
    //   block [0x82694BF0..0x82694C60)
	// 82694BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694C04: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694C0C: 390BB9E8  addi r8, r11, -0x4618
	ctx.r[8].s64 = ctx.r[11].s64 + -17944;
	// 82694C10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82694C14: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 82694C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694C1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694C28: 386A1190  addi r3, r10, 0x1190
	ctx.r[3].s64 = ctx.r[10].s64 + 4496;
	// 82694C2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694C4C: 4BDD21D5  bl 0x82466e20
	ctx.lr = 0x82694C50;
	sub_82466E20(ctx, base);
	// 82694C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694C60 size=112
    let mut pc: u32 = 0x82694C60;
    'dispatch: loop {
        match pc {
            0x82694C60 => {
    //   block [0x82694C60..0x82694CD0)
	// 82694C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694C6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C70: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694C74: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694C7C: 390BBA90  addi r8, r11, -0x4570
	ctx.r[8].s64 = ctx.r[11].s64 + -17776;
	// 82694C80: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82694C84: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82694C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694C8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694C98: 386A11C0  addi r3, r10, 0x11c0
	ctx.r[3].s64 = ctx.r[10].s64 + 4544;
	// 82694C9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694CBC: 4BDD2165  bl 0x82466e20
	ctx.lr = 0x82694CC0;
	sub_82466E20(ctx, base);
	// 82694CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694CD0 size=112
    let mut pc: u32 = 0x82694CD0;
    'dispatch: loop {
        match pc {
            0x82694CD0 => {
    //   block [0x82694CD0..0x82694D40)
	// 82694CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694CE0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694CE4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694CEC: 390BBB20  addi r8, r11, -0x44e0
	ctx.r[8].s64 = ctx.r[11].s64 + -17632;
	// 82694CF0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82694CF4: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82694CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694CFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694D08: 386A11F0  addi r3, r10, 0x11f0
	ctx.r[3].s64 = ctx.r[10].s64 + 4592;
	// 82694D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82694D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694D2C: 4BDD20F5  bl 0x82466e20
	ctx.lr = 0x82694D30;
	sub_82466E20(ctx, base);
	// 82694D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694D40 size=116
    let mut pc: u32 = 0x82694D40;
    'dispatch: loop {
        match pc {
            0x82694D40 => {
    //   block [0x82694D40..0x82694DB4)
	// 82694D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694D4C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694D50: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82694D54: 390ABBC8  addi r8, r10, -0x4438
	ctx.r[8].s64 = ctx.r[10].s64 + -17464;
	// 82694D58: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694D5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82694D60: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82694D64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82694D68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82694D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82694D74: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 82694D78: 396B8458  addi r11, r11, -0x7ba8
	ctx.r[11].s64 = ctx.r[11].s64 + -31656;
	// 82694D7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694D84: 386A1220  addi r3, r10, 0x1220
	ctx.r[3].s64 = ctx.r[10].s64 + 4640;
	// 82694D88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82694D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694D90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82694D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694DA0: 4BDD2081  bl 0x82466e20
	ctx.lr = 0x82694DA4;
	sub_82466E20(ctx, base);
	// 82694DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694DB8 size=108
    let mut pc: u32 = 0x82694DB8;
    'dispatch: loop {
        match pc {
            0x82694DB8 => {
    //   block [0x82694DB8..0x82694E24)
	// 82694DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694DC4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694DCC: 38EBBC58  addi r7, r11, -0x43a8
	ctx.r[7].s64 = ctx.r[11].s64 + -17320;
	// 82694DD0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82694DD4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 82694DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694DE8: 386A1250  addi r3, r10, 0x1250
	ctx.r[3].s64 = ctx.r[10].s64 + 4688;
	// 82694DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694E10: 4BDD2011  bl 0x82466e20
	ctx.lr = 0x82694E14;
	sub_82466E20(ctx, base);
	// 82694E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694E28 size=112
    let mut pc: u32 = 0x82694E28;
    'dispatch: loop {
        match pc {
            0x82694E28 => {
    //   block [0x82694E28..0x82694E98)
	// 82694E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694E34: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82694E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694E3C: 392B8518  addi r9, r11, -0x7ae8
	ctx.r[9].s64 = ctx.r[11].s64 + -31464;
	// 82694E40: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82694E44: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82694E48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694E4C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82694E50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694E54: 396BBD1C  addi r11, r11, -0x42e4
	ctx.r[11].s64 = ctx.r[11].s64 + -17124;
	// 82694E58: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82694E5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694E60: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82694E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694E68: 386A1280  addi r3, r10, 0x1280
	ctx.r[3].s64 = ctx.r[10].s64 + 4736;
	// 82694E6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82694E70: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82694E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694E78: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82694E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82694E84: 4BDD1F9D  bl 0x82466e20
	ctx.lr = 0x82694E88;
	sub_82466E20(ctx, base);
	// 82694E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694E98 size=100
    let mut pc: u32 = 0x82694E98;
    'dispatch: loop {
        match pc {
            0x82694E98 => {
    //   block [0x82694E98..0x82694EFC)
	// 82694E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694EA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694EAC: 38AA1C70  addi r5, r10, 0x1c70
	ctx.r[5].s64 = ctx.r[10].s64 + 7280;
	// 82694EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82694EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694EB8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82694EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694ECC: 386A12B0  addi r3, r10, 0x12b0
	ctx.r[3].s64 = ctx.r[10].s64 + 4784;
	// 82694ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694ED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82694EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82694EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694EE8: 4BDD1F39  bl 0x82466e20
	ctx.lr = 0x82694EEC;
	sub_82466E20(ctx, base);
	// 82694EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82694F00 size=24
    let mut pc: u32 = 0x82694F00;
    'dispatch: loop {
        match pc {
            0x82694F00 => {
    //   block [0x82694F00..0x82694F18)
	// 82694F00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694F04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82694F08: 394A4C50  addi r10, r10, 0x4c50
	ctx.r[10].s64 = ctx.r[10].s64 + 19536;
	// 82694F0C: 816BBD50  lwz r11, -0x42b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17072 as u32) ) } as u64;
	// 82694F10: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82694F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694F18 size=108
    let mut pc: u32 = 0x82694F18;
    'dispatch: loop {
        match pc {
            0x82694F18 => {
    //   block [0x82694F18..0x82694F84)
	// 82694F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694F24: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694F28: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82694F2C: 38EB4C50  addi r7, r11, 0x4c50
	ctx.r[7].s64 = ctx.r[11].s64 + 19536;
	// 82694F30: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82694F34: 388AB204  addi r4, r10, -0x4dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -19964;
	// 82694F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694F48: 386A12E0  addi r3, r10, 0x12e0
	ctx.r[3].s64 = ctx.r[10].s64 + 4832;
	// 82694F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694F70: 4BDD1EB1  bl 0x82466e20
	ctx.lr = 0x82694F74;
	sub_82466E20(ctx, base);
	// 82694F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694F88 size=108
    let mut pc: u32 = 0x82694F88;
    'dispatch: loop {
        match pc {
            0x82694F88 => {
    //   block [0x82694F88..0x82694FF4)
	// 82694F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82694F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82694F94: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82694F98: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82694F9C: 38EBBD58  addi r7, r11, -0x42a8
	ctx.r[7].s64 = ctx.r[11].s64 + -17064;
	// 82694FA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82694FA4: 388AB224  addi r4, r10, -0x4ddc
	ctx.r[4].s64 = ctx.r[10].s64 + -19932;
	// 82694FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82694FAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82694FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82694FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82694FB8: 386A1310  addi r3, r10, 0x1310
	ctx.r[3].s64 = ctx.r[10].s64 + 4880;
	// 82694FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82694FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82694FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82694FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82694FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82694FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82694FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82694FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82694FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82694FE0: 4BDD1E41  bl 0x82466e20
	ctx.lr = 0x82694FE4;
	sub_82466E20(ctx, base);
	// 82694FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82694FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82694FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82694FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82694FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82694FF8 size=108
    let mut pc: u32 = 0x82694FF8;
    'dispatch: loop {
        match pc {
            0x82694FF8 => {
    //   block [0x82694FF8..0x82695064)
	// 82694FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82694FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695004: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695008: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269500C: 38EBBDA0  addi r7, r11, -0x4260
	ctx.r[7].s64 = ctx.r[11].s64 + -16992;
	// 82695010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82695014: 388AB248  addi r4, r10, -0x4db8
	ctx.r[4].s64 = ctx.r[10].s64 + -19896;
	// 82695018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269501C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695028: 386A1340  addi r3, r10, 0x1340
	ctx.r[3].s64 = ctx.r[10].s64 + 4928;
	// 8269502C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269503C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269504C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695050: 4BDD1DD1  bl 0x82466e20
	ctx.lr = 0x82695054;
	sub_82466E20(ctx, base);
	// 82695054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269505C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695068 size=108
    let mut pc: u32 = 0x82695068;
    'dispatch: loop {
        match pc {
            0x82695068 => {
    //   block [0x82695068..0x826950D4)
	// 82695068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269506C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695074: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695078: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269507C: 38EBBDD0  addi r7, r11, -0x4230
	ctx.r[7].s64 = ctx.r[11].s64 + -16944;
	// 82695080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82695084: 388AB264  addi r4, r10, -0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -19868;
	// 82695088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269508C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695098: 386A1370  addi r3, r10, 0x1370
	ctx.r[3].s64 = ctx.r[10].s64 + 4976;
	// 8269509C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826950A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826950A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826950A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826950AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826950B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826950B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826950B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826950BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826950C0: 4BDD1D61  bl 0x82466e20
	ctx.lr = 0x826950C4;
	sub_82466E20(ctx, base);
	// 826950C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826950C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826950CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826950D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826950D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826950D8 size=100
    let mut pc: u32 = 0x826950D8;
    'dispatch: loop {
        match pc {
            0x826950D8 => {
    //   block [0x826950D8..0x8269513C)
	// 826950D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826950DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826950E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826950E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826950E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826950EC: 38AA1370  addi r5, r10, 0x1370
	ctx.r[5].s64 = ctx.r[10].s64 + 4976;
	// 826950F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826950F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826950F8: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826950FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269510C: 386A13A0  addi r3, r10, 0x13a0
	ctx.r[3].s64 = ctx.r[10].s64 + 5024;
	// 82695110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695118: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269511C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82695124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695128: 4BDD1CF9  bl 0x82466e20
	ctx.lr = 0x8269512C;
	sub_82466E20(ctx, base);
	// 8269512C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695140 size=112
    let mut pc: u32 = 0x82695140;
    'dispatch: loop {
        match pc {
            0x82695140 => {
    //   block [0x82695140..0x826951B0)
	// 82695140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269514C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82695150: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695154: 392A85A8  addi r9, r10, -0x7a58
	ctx.r[9].s64 = ctx.r[10].s64 + -31320;
	// 82695158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269515C: 390BBE00  addi r8, r11, -0x4200
	ctx.r[8].s64 = ctx.r[11].s64 + -16896;
	// 82695160: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82695164: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82695168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269516C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695178: 386A13D0  addi r3, r10, 0x13d0
	ctx.r[3].s64 = ctx.r[10].s64 + 5072;
	// 8269517C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695180: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82695184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269518C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269519C: 4BDD1C85  bl 0x82466e20
	ctx.lr = 0x826951A0;
	sub_82466E20(ctx, base);
	// 826951A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826951A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826951A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826951AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826951B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826951B0 size=112
    let mut pc: u32 = 0x826951B0;
    'dispatch: loop {
        match pc {
            0x826951B0 => {
    //   block [0x826951B0..0x82695220)
	// 826951B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826951B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826951B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826951BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826951C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826951C4: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826951C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826951CC: 390BBE48  addi r8, r11, -0x41b8
	ctx.r[8].s64 = ctx.r[11].s64 + -16824;
	// 826951D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826951D4: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826951D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826951DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826951E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826951E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826951E8: 386A1400  addi r3, r10, 0x1400
	ctx.r[3].s64 = ctx.r[10].s64 + 5120;
	// 826951EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826951F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826951F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826951F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826951FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269520C: 4BDD1C15  bl 0x82466e20
	ctx.lr = 0x82695210;
	sub_82466E20(ctx, base);
	// 82695210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269521C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695220 size=116
    let mut pc: u32 = 0x82695220;
    'dispatch: loop {
        match pc {
            0x82695220 => {
    //   block [0x82695220..0x82695294)
	// 82695220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269522C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695230: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82695234: 390ABE78  addi r8, r10, -0x4188
	ctx.r[8].s64 = ctx.r[10].s64 + -16776;
	// 82695238: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269523C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695240: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82695244: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695248: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269524C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695254: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82695258: 396B85D0  addi r11, r11, -0x7a30
	ctx.r[11].s64 = ctx.r[11].s64 + -31280;
	// 8269525C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695260: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695264: 386A1430  addi r3, r10, 0x1430
	ctx.r[3].s64 = ctx.r[10].s64 + 5168;
	// 82695268: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269526C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695270: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269527C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695280: 4BDD1BA1  bl 0x82466e20
	ctx.lr = 0x82695284;
	sub_82466E20(ctx, base);
	// 82695284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269528C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695298 size=100
    let mut pc: u32 = 0x82695298;
    'dispatch: loop {
        match pc {
            0x82695298 => {
    //   block [0x82695298..0x826952FC)
	// 82695298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269529C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826952A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826952A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826952A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826952AC: 38AA1430  addi r5, r10, 0x1430
	ctx.r[5].s64 = ctx.r[10].s64 + 5168;
	// 826952B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826952B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826952B8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826952BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826952C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826952C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826952C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826952CC: 386A1460  addi r3, r10, 0x1460
	ctx.r[3].s64 = ctx.r[10].s64 + 5216;
	// 826952D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826952D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826952D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826952DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826952E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826952E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826952E8: 4BDD1B39  bl 0x82466e20
	ctx.lr = 0x826952EC;
	sub_82466E20(ctx, base);
	// 826952EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826952F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826952F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826952F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695300 size=116
    let mut pc: u32 = 0x82695300;
    'dispatch: loop {
        match pc {
            0x82695300 => {
    //   block [0x82695300..0x82695374)
	// 82695300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269530C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695310: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695314: 392B8624  addi r9, r11, -0x79dc
	ctx.r[9].s64 = ctx.r[11].s64 + -31196;
	// 82695318: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 8269531C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695320: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82695324: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82695328: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269532C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82695330: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695334: 396BBF20  addi r11, r11, -0x40e0
	ctx.r[11].s64 = ctx.r[11].s64 + -16608;
	// 82695338: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269533C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695340: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82695344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695348: 386A1490  addi r3, r10, 0x1490
	ctx.r[3].s64 = ctx.r[10].s64 + 5264;
	// 8269534C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82695350: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82695354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695358: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269535C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82695360: 4BDD1AC1  bl 0x82466e20
	ctx.lr = 0x82695364;
	sub_82466E20(ctx, base);
	// 82695364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269536C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695378 size=24
    let mut pc: u32 = 0x82695378;
    'dispatch: loop {
        match pc {
            0x82695378 => {
    //   block [0x82695378..0x82695390)
	// 82695378: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269537C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695380: 394A4CE0  addi r10, r10, 0x4ce0
	ctx.r[10].s64 = ctx.r[10].s64 + 19680;
	// 82695384: 816BC070  lwz r11, -0x3f90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16272 as u32) ) } as u64;
	// 82695388: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269538C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695390 size=116
    let mut pc: u32 = 0x82695390;
    'dispatch: loop {
        match pc {
            0x82695390 => {
    //   block [0x82695390..0x82695404)
	// 82695390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269539C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826953A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826953A4: 392B8690  addi r9, r11, -0x7970
	ctx.r[9].s64 = ctx.r[11].s64 + -31088;
	// 826953A8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826953AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826953B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826953B4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826953B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826953BC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826953C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826953C4: 396B4CE0  addi r11, r11, 0x4ce0
	ctx.r[11].s64 = ctx.r[11].s64 + 19680;
	// 826953C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826953CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826953D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826953D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826953D8: 386A14C0  addi r3, r10, 0x14c0
	ctx.r[3].s64 = ctx.r[10].s64 + 5312;
	// 826953DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826953E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826953E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826953E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826953EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826953F0: 4BDD1A31  bl 0x82466e20
	ctx.lr = 0x826953F4;
	sub_82466E20(ctx, base);
	// 826953F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826953F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826953FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695408 size=108
    let mut pc: u32 = 0x82695408;
    'dispatch: loop {
        match pc {
            0x82695408 => {
    //   block [0x82695408..0x82695474)
	// 82695408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269540C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695414: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269541C: 38EBC078  addi r7, r11, -0x3f88
	ctx.r[7].s64 = ctx.r[11].s64 + -16264;
	// 82695420: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82695424: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82695428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269542C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695438: 386A14F0  addi r3, r10, 0x14f0
	ctx.r[3].s64 = ctx.r[10].s64 + 5360;
	// 8269543C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269544C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269545C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695460: 4BDD19C1  bl 0x82466e20
	ctx.lr = 0x82695464;
	sub_82466E20(ctx, base);
	// 82695464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269546C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695478 size=24
    let mut pc: u32 = 0x82695478;
    'dispatch: loop {
        match pc {
            0x82695478 => {
    //   block [0x82695478..0x82695490)
	// 82695478: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269547C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695480: 394A4DA0  addi r10, r10, 0x4da0
	ctx.r[10].s64 = ctx.r[10].s64 + 19872;
	// 82695484: 816BC074  lwz r11, -0x3f8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16268 as u32) ) } as u64;
	// 82695488: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269548C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695490 size=116
    let mut pc: u32 = 0x82695490;
    'dispatch: loop {
        match pc {
            0x82695490 => {
    //   block [0x82695490..0x82695504)
	// 82695490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269549C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826954A0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826954A4: 390B4DA0  addi r8, r11, 0x4da0
	ctx.r[8].s64 = ctx.r[11].s64 + 19872;
	// 826954A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826954AC: 392A8710  addi r9, r10, -0x78f0
	ctx.r[9].s64 = ctx.r[10].s64 + -30960;
	// 826954B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826954B4: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826954B8: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826954BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826954C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826954C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826954C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826954CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826954D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826954D4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826954D8: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826954DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826954E0: 386B1520  addi r3, r11, 0x1520
	ctx.r[3].s64 = ctx.r[11].s64 + 5408;
	// 826954E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826954E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826954EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826954F0: 4BDD1931  bl 0x82466e20
	ctx.lr = 0x826954F4;
	sub_82466E20(ctx, base);
	// 826954F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826954F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826954FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695508 size=112
    let mut pc: u32 = 0x82695508;
    'dispatch: loop {
        match pc {
            0x82695508 => {
    //   block [0x82695508..0x82695578)
	// 82695508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695514: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695518: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269551C: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82695520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695524: 390BC0F4  addi r8, r11, -0x3f0c
	ctx.r[8].s64 = ctx.r[11].s64 + -16140;
	// 82695528: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269552C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82695530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695534: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269553C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695540: 386A1550  addi r3, r10, 0x1550
	ctx.r[3].s64 = ctx.r[10].s64 + 5456;
	// 82695544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269554C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269555C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695564: 4BDD18BD  bl 0x82466e20
	ctx.lr = 0x82695568;
	sub_82466E20(ctx, base);
	// 82695568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269556C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695578 size=24
    let mut pc: u32 = 0x82695578;
    'dispatch: loop {
        match pc {
            0x82695578 => {
    //   block [0x82695578..0x82695590)
	// 82695578: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269557C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695580: 394A4F38  addi r10, r10, 0x4f38
	ctx.r[10].s64 = ctx.r[10].s64 + 20280;
	// 82695584: 816BC124  lwz r11, -0x3edc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16092 as u32) ) } as u64;
	// 82695588: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8269558C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695590 size=116
    let mut pc: u32 = 0x82695590;
    'dispatch: loop {
        match pc {
            0x82695590 => {
    //   block [0x82695590..0x82695604)
	// 82695590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269559C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826955A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826955A4: 392B8748  addi r9, r11, -0x78b8
	ctx.r[9].s64 = ctx.r[11].s64 + -30904;
	// 826955A8: 38AA1490  addi r5, r10, 0x1490
	ctx.r[5].s64 = ctx.r[10].s64 + 5264;
	// 826955AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826955B0: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826955B4: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826955B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826955BC: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826955C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826955C4: 396B4F38  addi r11, r11, 0x4f38
	ctx.r[11].s64 = ctx.r[11].s64 + 20280;
	// 826955C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826955CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826955D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826955D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826955D8: 386A1580  addi r3, r10, 0x1580
	ctx.r[3].s64 = ctx.r[10].s64 + 5504;
	// 826955DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826955E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826955E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826955E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826955EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826955F0: 4BDD1831  bl 0x82466e20
	ctx.lr = 0x826955F4;
	sub_82466E20(ctx, base);
	// 826955F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826955F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826955FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695608 size=116
    let mut pc: u32 = 0x82695608;
    'dispatch: loop {
        match pc {
            0x82695608 => {
    //   block [0x82695608..0x8269567C)
	// 82695608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269560C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695614: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695618: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 8269561C: 390AC128  addi r8, r10, -0x3ed8
	ctx.r[8].s64 = ctx.r[10].s64 + -16088;
	// 82695620: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695624: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695628: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 8269562C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82695630: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269563C: 388AB294  addi r4, r10, -0x4d6c
	ctx.r[4].s64 = ctx.r[10].s64 + -19820;
	// 82695640: 396B87B8  addi r11, r11, -0x7848
	ctx.r[11].s64 = ctx.r[11].s64 + -30792;
	// 82695644: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695648: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269564C: 386A15B0  addi r3, r10, 0x15b0
	ctx.r[3].s64 = ctx.r[10].s64 + 5552;
	// 82695650: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695654: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695658: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269565C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695668: 4BDD17B9  bl 0x82466e20
	ctx.lr = 0x8269566C;
	sub_82466E20(ctx, base);
	// 8269566C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695680 size=116
    let mut pc: u32 = 0x82695680;
    'dispatch: loop {
        match pc {
            0x82695680 => {
    //   block [0x82695680..0x826956F4)
	// 82695680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269568C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695690: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82695694: 390AC290  addi r8, r10, -0x3d70
	ctx.r[8].s64 = ctx.r[10].s64 + -15728;
	// 82695698: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269569C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826956A0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826956A4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826956A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826956AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826956B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826956B4: 388AB2BC  addi r4, r10, -0x4d44
	ctx.r[4].s64 = ctx.r[10].s64 + -19780;
	// 826956B8: 396B8834  addi r11, r11, -0x77cc
	ctx.r[11].s64 = ctx.r[11].s64 + -30668;
	// 826956BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826956C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826956C4: 386A15E0  addi r3, r10, 0x15e0
	ctx.r[3].s64 = ctx.r[10].s64 + 5600;
	// 826956C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826956CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826956D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826956D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826956D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826956DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826956E0: 4BDD1741  bl 0x82466e20
	ctx.lr = 0x826956E4;
	sub_82466E20(ctx, base);
	// 826956E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826956E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826956EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826956F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826956F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826956F8 size=112
    let mut pc: u32 = 0x826956F8;
    'dispatch: loop {
        match pc {
            0x826956F8 => {
    //   block [0x826956F8..0x82695768)
	// 826956F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826956FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695704: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695708: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269570C: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82695710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695714: 390BC2D8  addi r8, r11, -0x3d28
	ctx.r[8].s64 = ctx.r[11].s64 + -15656;
	// 82695718: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269571C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82695720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695724: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269572C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695730: 386A1610  addi r3, r10, 0x1610
	ctx.r[3].s64 = ctx.r[10].s64 + 5648;
	// 82695734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269573C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269574C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695754: 4BDD16CD  bl 0x82466e20
	ctx.lr = 0x82695758;
	sub_82466E20(ctx, base);
	// 82695758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269575C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695768 size=100
    let mut pc: u32 = 0x82695768;
    'dispatch: loop {
        match pc {
            0x82695768 => {
    //   block [0x82695768..0x826957CC)
	// 82695768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695774: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269577C: 38AA1C70  addi r5, r10, 0x1c70
	ctx.r[5].s64 = ctx.r[10].s64 + 7280;
	// 82695780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695788: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8269578C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269579C: 386A1640  addi r3, r10, 0x1640
	ctx.r[3].s64 = ctx.r[10].s64 + 5696;
	// 826957A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826957A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826957A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826957AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826957B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826957B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826957B8: 4BDD1669  bl 0x82466e20
	ctx.lr = 0x826957BC;
	sub_82466E20(ctx, base);
	// 826957BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826957C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826957C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826957C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826957D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826957D0 size=112
    let mut pc: u32 = 0x826957D0;
    'dispatch: loop {
        match pc {
            0x826957D0 => {
    //   block [0x826957D0..0x82695840)
	// 826957D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826957D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826957D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826957DC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826957E0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826957E4: 38EAC2F0  addi r7, r10, -0x3d10
	ctx.r[7].s64 = ctx.r[10].s64 + -15632;
	// 826957E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826957EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826957F0: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826957F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826957F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826957FC: 396B8860  addi r11, r11, -0x77a0
	ctx.r[11].s64 = ctx.r[11].s64 + -30624;
	// 82695800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269580C: 386A1670  addi r3, r10, 0x1670
	ctx.r[3].s64 = ctx.r[10].s64 + 5744;
	// 82695810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695814: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695818: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269581C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695824: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695828: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269582C: 4BDD15F5  bl 0x82466e20
	ctx.lr = 0x82695830;
	sub_82466E20(ctx, base);
	// 82695830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269583C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695840 size=112
    let mut pc: u32 = 0x82695840;
    'dispatch: loop {
        match pc {
            0x82695840 => {
    //   block [0x82695840..0x826958B0)
	// 82695840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269584C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695854: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269585C: 390BC428  addi r8, r11, -0x3bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -15320;
	// 82695860: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82695864: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82695868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269586C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695878: 386A16A0  addi r3, r10, 0x16a0
	ctx.r[3].s64 = ctx.r[10].s64 + 5792;
	// 8269587C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269588C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269589C: 4BDD1585  bl 0x82466e20
	ctx.lr = 0x826958A0;
	sub_82466E20(ctx, base);
	// 826958A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826958A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826958A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826958AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826958B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826958B0 size=112
    let mut pc: u32 = 0x826958B0;
    'dispatch: loop {
        match pc {
            0x826958B0 => {
    //   block [0x826958B0..0x82695920)
	// 826958B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826958B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826958B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826958BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826958C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826958C4: 38EAC458  addi r7, r10, -0x3ba8
	ctx.r[7].s64 = ctx.r[10].s64 + -15272;
	// 826958C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826958CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826958D0: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826958D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826958D8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826958DC: 396B88B4  addi r11, r11, -0x774c
	ctx.r[11].s64 = ctx.r[11].s64 + -30540;
	// 826958E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826958E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826958E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826958EC: 386A16D0  addi r3, r10, 0x16d0
	ctx.r[3].s64 = ctx.r[10].s64 + 5840;
	// 826958F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826958F4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826958F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826958FC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695904: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695908: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269590C: 4BDD1515  bl 0x82466e20
	ctx.lr = 0x82695910;
	sub_82466E20(ctx, base);
	// 82695910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269591C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695920 size=112
    let mut pc: u32 = 0x82695920;
    'dispatch: loop {
        match pc {
            0x82695920 => {
    //   block [0x82695920..0x82695990)
	// 82695920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269592C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695930: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695934: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269593C: 390BC488  addi r8, r11, -0x3b78
	ctx.r[8].s64 = ctx.r[11].s64 + -15224;
	// 82695940: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82695944: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82695948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269594C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695958: 386A1700  addi r3, r10, 0x1700
	ctx.r[3].s64 = ctx.r[10].s64 + 5888;
	// 8269595C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269596C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269597C: 4BDD14A5  bl 0x82466e20
	ctx.lr = 0x82695980;
	sub_82466E20(ctx, base);
	// 82695980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269598C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695990 size=108
    let mut pc: u32 = 0x82695990;
    'dispatch: loop {
        match pc {
            0x82695990 => {
    //   block [0x82695990..0x826959FC)
	// 82695990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269599C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826959A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826959A4: 38EBC4A0  addi r7, r11, -0x3b60
	ctx.r[7].s64 = ctx.r[11].s64 + -15200;
	// 826959A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826959AC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826959B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826959B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826959B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826959BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826959C0: 386A1730  addi r3, r10, 0x1730
	ctx.r[3].s64 = ctx.r[10].s64 + 5936;
	// 826959C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826959C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826959CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826959D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826959D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826959D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826959DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826959E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826959E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826959E8: 4BDD1439  bl 0x82466e20
	ctx.lr = 0x826959EC;
	sub_82466E20(ctx, base);
	// 826959EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826959F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826959F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826959F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695A00 size=112
    let mut pc: u32 = 0x82695A00;
    'dispatch: loop {
        match pc {
            0x82695A00 => {
    //   block [0x82695A00..0x82695A70)
	// 82695A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695A0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695A10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695A14: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695A1C: 390BC4B8  addi r8, r11, -0x3b48
	ctx.r[8].s64 = ctx.r[11].s64 + -15176;
	// 82695A20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82695A24: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 82695A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695A2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695A38: 386A1760  addi r3, r10, 0x1760
	ctx.r[3].s64 = ctx.r[10].s64 + 5984;
	// 82695A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695A5C: 4BDD13C5  bl 0x82466e20
	ctx.lr = 0x82695A60;
	sub_82466E20(ctx, base);
	// 82695A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695A70 size=112
    let mut pc: u32 = 0x82695A70;
    'dispatch: loop {
        match pc {
            0x82695A70 => {
    //   block [0x82695A70..0x82695AE0)
	// 82695A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695A7C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695A80: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82695A84: 38EAC4D0  addi r7, r10, -0x3b30
	ctx.r[7].s64 = ctx.r[10].s64 + -15152;
	// 82695A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695A8C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695A90: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82695A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695A98: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695A9C: 396B88C0  addi r11, r11, -0x7740
	ctx.r[11].s64 = ctx.r[11].s64 + -30528;
	// 82695AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695AA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695AAC: 386A1790  addi r3, r10, 0x1790
	ctx.r[3].s64 = ctx.r[10].s64 + 6032;
	// 82695AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695AB4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695AB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695ABC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695AC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695AC4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695AC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695ACC: 4BDD1355  bl 0x82466e20
	ctx.lr = 0x82695AD0;
	sub_82466E20(ctx, base);
	// 82695AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695AE0 size=112
    let mut pc: u32 = 0x82695AE0;
    'dispatch: loop {
        match pc {
            0x82695AE0 => {
    //   block [0x82695AE0..0x82695B50)
	// 82695AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695AEC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695AF0: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82695AF4: 38EAC5A8  addi r7, r10, -0x3a58
	ctx.r[7].s64 = ctx.r[10].s64 + -14936;
	// 82695AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695AFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695B00: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82695B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695B08: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695B0C: 396B8900  addi r11, r11, -0x7700
	ctx.r[11].s64 = ctx.r[11].s64 + -30464;
	// 82695B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695B14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695B1C: 386A17C0  addi r3, r10, 0x17c0
	ctx.r[3].s64 = ctx.r[10].s64 + 6080;
	// 82695B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695B24: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695B28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695B2C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695B30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695B34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695B38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695B3C: 4BDD12E5  bl 0x82466e20
	ctx.lr = 0x82695B40;
	sub_82466E20(ctx, base);
	// 82695B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695B50 size=108
    let mut pc: u32 = 0x82695B50;
    'dispatch: loop {
        match pc {
            0x82695B50 => {
    //   block [0x82695B50..0x82695BBC)
	// 82695B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695B5C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695B64: 38EBC710  addi r7, r11, -0x38f0
	ctx.r[7].s64 = ctx.r[11].s64 + -14576;
	// 82695B68: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82695B6C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 82695B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695B74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695B78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695B80: 386A17F0  addi r3, r10, 0x17f0
	ctx.r[3].s64 = ctx.r[10].s64 + 6128;
	// 82695B84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695BA8: 4BDD1279  bl 0x82466e20
	ctx.lr = 0x82695BAC;
	sub_82466E20(ctx, base);
	// 82695BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695BC0 size=116
    let mut pc: u32 = 0x82695BC0;
    'dispatch: loop {
        match pc {
            0x82695BC0 => {
    //   block [0x82695BC0..0x82695C34)
	// 82695BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695BCC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695BD0: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82695BD4: 390AC788  addi r8, r10, -0x3878
	ctx.r[8].s64 = ctx.r[10].s64 + -14456;
	// 82695BD8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695BDC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695BE0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695BE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695BE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695BF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695BF4: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82695BF8: 396B89A8  addi r11, r11, -0x7658
	ctx.r[11].s64 = ctx.r[11].s64 + -30296;
	// 82695BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695C00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695C04: 386A1820  addi r3, r10, 0x1820
	ctx.r[3].s64 = ctx.r[10].s64 + 6176;
	// 82695C08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695C10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695C20: 4BDD1201  bl 0x82466e20
	ctx.lr = 0x82695C24;
	sub_82466E20(ctx, base);
	// 82695C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695C38 size=116
    let mut pc: u32 = 0x82695C38;
    'dispatch: loop {
        match pc {
            0x82695C38 => {
    //   block [0x82695C38..0x82695CAC)
	// 82695C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695C44: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695C48: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82695C4C: 390AC8A8  addi r8, r10, -0x3758
	ctx.r[8].s64 = ctx.r[10].s64 + -14168;
	// 82695C50: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695C54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695C58: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695C5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695C60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695C6C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82695C70: 396B89E0  addi r11, r11, -0x7620
	ctx.r[11].s64 = ctx.r[11].s64 + -30240;
	// 82695C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695C78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695C7C: 386A1850  addi r3, r10, 0x1850
	ctx.r[3].s64 = ctx.r[10].s64 + 6224;
	// 82695C80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695C84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695C88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695C98: 4BDD1189  bl 0x82466e20
	ctx.lr = 0x82695C9C;
	sub_82466E20(ctx, base);
	// 82695C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695CB0 size=108
    let mut pc: u32 = 0x82695CB0;
    'dispatch: loop {
        match pc {
            0x82695CB0 => {
    //   block [0x82695CB0..0x82695D1C)
	// 82695CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695CBC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695CC4: 38EBC908  addi r7, r11, -0x36f8
	ctx.r[7].s64 = ctx.r[11].s64 + -14072;
	// 82695CC8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82695CCC: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 82695CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695CE0: 386A1880  addi r3, r10, 0x1880
	ctx.r[3].s64 = ctx.r[10].s64 + 6272;
	// 82695CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695D08: 4BDD1119  bl 0x82466e20
	ctx.lr = 0x82695D0C;
	sub_82466E20(ctx, base);
	// 82695D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695D20 size=112
    let mut pc: u32 = 0x82695D20;
    'dispatch: loop {
        match pc {
            0x82695D20 => {
    //   block [0x82695D20..0x82695D90)
	// 82695D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695D2C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695D30: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82695D34: 38EAC9B0  addi r7, r10, -0x3650
	ctx.r[7].s64 = ctx.r[10].s64 + -13904;
	// 82695D38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695D3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695D40: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 82695D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695D48: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82695D4C: 396B89F8  addi r11, r11, -0x7608
	ctx.r[11].s64 = ctx.r[11].s64 + -30216;
	// 82695D50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82695D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695D58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695D5C: 386A18B0  addi r3, r10, 0x18b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6320;
	// 82695D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695D64: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695D68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695D6C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695D70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695D74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695D78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82695D7C: 4BDD10A5  bl 0x82466e20
	ctx.lr = 0x82695D80;
	sub_82466E20(ctx, base);
	// 82695D80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695D90 size=112
    let mut pc: u32 = 0x82695D90;
    'dispatch: loop {
        match pc {
            0x82695D90 => {
    //   block [0x82695D90..0x82695E00)
	// 82695D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695D9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695DA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695DA4: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695DAC: 390BCA88  addi r8, r11, -0x3578
	ctx.r[8].s64 = ctx.r[11].s64 + -13688;
	// 82695DB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82695DB4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82695DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695DC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695DC8: 386A18E0  addi r3, r10, 0x18e0
	ctx.r[3].s64 = ctx.r[10].s64 + 6368;
	// 82695DCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695DEC: 4BDD1035  bl 0x82466e20
	ctx.lr = 0x82695DF0;
	sub_82466E20(ctx, base);
	// 82695DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82695E00 size=24
    let mut pc: u32 = 0x82695E00;
    'dispatch: loop {
        match pc {
            0x82695E00 => {
    //   block [0x82695E00..0x82695E18)
	// 82695E00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695E04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695E08: 394A50A0  addi r10, r10, 0x50a0
	ctx.r[10].s64 = ctx.r[10].s64 + 20640;
	// 82695E0C: 816BCAD0  lwz r11, -0x3530(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13616 as u32) ) } as u64;
	// 82695E10: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82695E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695E18 size=116
    let mut pc: u32 = 0x82695E18;
    'dispatch: loop {
        match pc {
            0x82695E18 => {
    //   block [0x82695E18..0x82695E8C)
	// 82695E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695E24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695E28: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695E2C: 392B8A68  addi r9, r11, -0x7598
	ctx.r[9].s64 = ctx.r[11].s64 + -30104;
	// 82695E30: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82695E34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695E38: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 82695E3C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82695E40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695E44: 388A7B20  addi r4, r10, 0x7b20
	ctx.r[4].s64 = ctx.r[10].s64 + 31520;
	// 82695E48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695E4C: 396B50A0  addi r11, r11, 0x50a0
	ctx.r[11].s64 = ctx.r[11].s64 + 20640;
	// 82695E50: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82695E54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695E58: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82695E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695E60: 386A1910  addi r3, r10, 0x1910
	ctx.r[3].s64 = ctx.r[10].s64 + 6416;
	// 82695E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82695E68: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82695E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695E70: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82695E74: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82695E78: 4BDD0FA9  bl 0x82466e20
	ctx.lr = 0x82695E7C;
	sub_82466E20(ctx, base);
	// 82695E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695E90 size=116
    let mut pc: u32 = 0x82695E90;
    'dispatch: loop {
        match pc {
            0x82695E90 => {
    //   block [0x82695E90..0x82695F04)
	// 82695E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695E9C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695EA0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82695EA4: 390ACAD8  addi r8, r10, -0x3528
	ctx.r[8].s64 = ctx.r[10].s64 + -13608;
	// 82695EA8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695EAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695EB0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695EB4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82695EB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695EC4: 388AB2FC  addi r4, r10, -0x4d04
	ctx.r[4].s64 = ctx.r[10].s64 + -19716;
	// 82695EC8: 396B8AC4  addi r11, r11, -0x753c
	ctx.r[11].s64 = ctx.r[11].s64 + -30012;
	// 82695ECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695ED4: 386A1940  addi r3, r10, 0x1940
	ctx.r[3].s64 = ctx.r[10].s64 + 6464;
	// 82695ED8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695EE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695EF0: 4BDD0F31  bl 0x82466e20
	ctx.lr = 0x82695EF4;
	sub_82466E20(ctx, base);
	// 82695EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695F08 size=116
    let mut pc: u32 = 0x82695F08;
    'dispatch: loop {
        match pc {
            0x82695F08 => {
    //   block [0x82695F08..0x82695F7C)
	// 82695F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695F14: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82695F18: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82695F1C: 390ACB20  addi r8, r10, -0x34e0
	ctx.r[8].s64 = ctx.r[10].s64 + -13536;
	// 82695F20: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695F24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82695F28: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695F2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82695F30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82695F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695F3C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82695F40: 396B8AD8  addi r11, r11, -0x7528
	ctx.r[11].s64 = ctx.r[11].s64 + -29992;
	// 82695F44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695F48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695F4C: 386A1970  addi r3, r10, 0x1970
	ctx.r[3].s64 = ctx.r[10].s64 + 6512;
	// 82695F50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82695F54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695F58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82695F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695F68: 4BDD0EB9  bl 0x82466e20
	ctx.lr = 0x82695F6C;
	sub_82466E20(ctx, base);
	// 82695F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695F80 size=112
    let mut pc: u32 = 0x82695F80;
    'dispatch: loop {
        match pc {
            0x82695F80 => {
    //   block [0x82695F80..0x82695FF0)
	// 82695F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695F8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695F90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82695F94: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82695F98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82695F9C: 390BCC58  addi r8, r11, -0x33a8
	ctx.r[8].s64 = ctx.r[11].s64 + -13224;
	// 82695FA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82695FA4: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 82695FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82695FAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82695FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82695FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82695FB8: 386A19A0  addi r3, r10, 0x19a0
	ctx.r[3].s64 = ctx.r[10].s64 + 6560;
	// 82695FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82695FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82695FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82695FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82695FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82695FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82695FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82695FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82695FDC: 4BDD0E45  bl 0x82466e20
	ctx.lr = 0x82695FE0;
	sub_82466E20(ctx, base);
	// 82695FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82695FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82695FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82695FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82695FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82695FF0 size=112
    let mut pc: u32 = 0x82695FF0;
    'dispatch: loop {
        match pc {
            0x82695FF0 => {
    //   block [0x82695FF0..0x82696060)
	// 82695FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82695FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82695FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82695FFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696000: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696004: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269600C: 390BCC70  addi r8, r11, -0x3390
	ctx.r[8].s64 = ctx.r[11].s64 + -13200;
	// 82696010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82696014: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82696018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269601C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696028: 386A19D0  addi r3, r10, 0x19d0
	ctx.r[3].s64 = ctx.r[10].s64 + 6608;
	// 8269602C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269603C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269604C: 4BDD0DD5  bl 0x82466e20
	ctx.lr = 0x82696050;
	sub_82466E20(ctx, base);
	// 82696050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269605C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696060 size=112
    let mut pc: u32 = 0x82696060;
    'dispatch: loop {
        match pc {
            0x82696060 => {
    //   block [0x82696060..0x826960D0)
	// 82696060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269606C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696070: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696074: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269607C: 390BCC88  addi r8, r11, -0x3378
	ctx.r[8].s64 = ctx.r[11].s64 + -13176;
	// 82696080: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82696084: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82696088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269608C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696098: 386A1A00  addi r3, r10, 0x1a00
	ctx.r[3].s64 = ctx.r[10].s64 + 6656;
	// 8269609C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826960A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826960A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826960A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826960AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826960B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826960B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826960B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826960BC: 4BDD0D65  bl 0x82466e20
	ctx.lr = 0x826960C0;
	sub_82466E20(ctx, base);
	// 826960C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826960C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826960C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826960CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826960D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826960D0 size=112
    let mut pc: u32 = 0x826960D0;
    'dispatch: loop {
        match pc {
            0x826960D0 => {
    //   block [0x826960D0..0x82696140)
	// 826960D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826960D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826960D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826960DC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826960E0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826960E4: 38EACCB8  addi r7, r10, -0x3348
	ctx.r[7].s64 = ctx.r[10].s64 + -13128;
	// 826960E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826960EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826960F0: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826960F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826960F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826960FC: 396B8B50  addi r11, r11, -0x74b0
	ctx.r[11].s64 = ctx.r[11].s64 + -29872;
	// 82696100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696104: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696108: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269610C: 386A1A30  addi r3, r10, 0x1a30
	ctx.r[3].s64 = ctx.r[10].s64 + 6704;
	// 82696110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696114: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696118: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269611C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696120: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696124: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696128: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269612C: 4BDD0CF5  bl 0x82466e20
	ctx.lr = 0x82696130;
	sub_82466E20(ctx, base);
	// 82696130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269613C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696140 size=112
    let mut pc: u32 = 0x82696140;
    'dispatch: loop {
        match pc {
            0x82696140 => {
    //   block [0x82696140..0x826961B0)
	// 82696140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269614C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696150: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696154: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269615C: 390BCD30  addi r8, r11, -0x32d0
	ctx.r[8].s64 = ctx.r[11].s64 + -13008;
	// 82696160: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82696164: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82696168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269616C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696178: 386A1A60  addi r3, r10, 0x1a60
	ctx.r[3].s64 = ctx.r[10].s64 + 6752;
	// 8269617C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269618C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269619C: 4BDD0C85  bl 0x82466e20
	ctx.lr = 0x826961A0;
	sub_82466E20(ctx, base);
	// 826961A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826961A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826961A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826961AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826961B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826961B0 size=24
    let mut pc: u32 = 0x826961B0;
    'dispatch: loop {
        match pc {
            0x826961B0 => {
    //   block [0x826961B0..0x826961C8)
	// 826961B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826961B4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826961B8: 394A51A8  addi r10, r10, 0x51a8
	ctx.r[10].s64 = ctx.r[10].s64 + 20904;
	// 826961BC: 816BCAD4  lwz r11, -0x352c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13612 as u32) ) } as u64;
	// 826961C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826961C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826961C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826961C8 size=116
    let mut pc: u32 = 0x826961C8;
    'dispatch: loop {
        match pc {
            0x826961C8 => {
    //   block [0x826961C8..0x8269623C)
	// 826961C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826961CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826961D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826961D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826961D8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826961DC: 390B51A8  addi r8, r11, 0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + 20904;
	// 826961E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826961E4: 392A8B90  addi r9, r10, -0x7470
	ctx.r[9].s64 = ctx.r[10].s64 + -29808;
	// 826961E8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826961EC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826961F0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826961F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826961F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826961FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269620C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82696210: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 82696214: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696218: 386B1A90  addi r3, r11, 0x1a90
	ctx.r[3].s64 = ctx.r[11].s64 + 6800;
	// 8269621C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82696220: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696228: 4BDD0BF9  bl 0x82466e20
	ctx.lr = 0x8269622C;
	sub_82466E20(ctx, base);
	// 8269622C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696240 size=116
    let mut pc: u32 = 0x82696240;
    'dispatch: loop {
        match pc {
            0x82696240 => {
    //   block [0x82696240..0x826962B4)
	// 82696240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269624C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696250: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82696254: 390ACD60  addi r8, r10, -0x32a0
	ctx.r[8].s64 = ctx.r[10].s64 + -12960;
	// 82696258: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269625C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696260: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696264: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696268: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269626C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696274: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82696278: 396B8BA4  addi r11, r11, -0x745c
	ctx.r[11].s64 = ctx.r[11].s64 + -29788;
	// 8269627C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696280: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696284: 386A1AC0  addi r3, r10, 0x1ac0
	ctx.r[3].s64 = ctx.r[10].s64 + 6848;
	// 82696288: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269628C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696290: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269629C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826962A0: 4BDD0B81  bl 0x82466e20
	ctx.lr = 0x826962A4;
	sub_82466E20(ctx, base);
	// 826962A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826962A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826962AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826962B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826962B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826962B8 size=112
    let mut pc: u32 = 0x826962B8;
    'dispatch: loop {
        match pc {
            0x826962B8 => {
    //   block [0x826962B8..0x82696328)
	// 826962B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826962BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826962C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826962C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826962C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826962CC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826962D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826962D4: 390BCE20  addi r8, r11, -0x31e0
	ctx.r[8].s64 = ctx.r[11].s64 + -12768;
	// 826962D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826962DC: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826962E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826962E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826962E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826962EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826962F0: 386A1AF0  addi r3, r10, 0x1af0
	ctx.r[3].s64 = ctx.r[10].s64 + 6896;
	// 826962F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826962F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826962FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269630C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696314: 4BDD0B0D  bl 0x82466e20
	ctx.lr = 0x82696318;
	sub_82466E20(ctx, base);
	// 82696318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269631C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696328 size=112
    let mut pc: u32 = 0x82696328;
    'dispatch: loop {
        match pc {
            0x82696328 => {
    //   block [0x82696328..0x82696398)
	// 82696328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696334: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269633C: 38EACE38  addi r7, r10, -0x31c8
	ctx.r[7].s64 = ctx.r[10].s64 + -12744;
	// 82696340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696344: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696348: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 8269634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696350: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696354: 396B8BC8  addi r11, r11, -0x7438
	ctx.r[11].s64 = ctx.r[11].s64 + -29752;
	// 82696358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269635C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696364: 386A1B20  addi r3, r10, 0x1b20
	ctx.r[3].s64 = ctx.r[10].s64 + 6944;
	// 82696368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269636C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696370: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696374: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696378: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269637C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696380: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696384: 4BDD0A9D  bl 0x82466e20
	ctx.lr = 0x82696388;
	sub_82466E20(ctx, base);
	// 82696388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269638C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696398 size=112
    let mut pc: u32 = 0x82696398;
    'dispatch: loop {
        match pc {
            0x82696398 => {
    //   block [0x82696398..0x82696408)
	// 82696398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826963A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826963A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826963A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826963AC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826963B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826963B4: 390BCE68  addi r8, r11, -0x3198
	ctx.r[8].s64 = ctx.r[11].s64 + -12696;
	// 826963B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826963BC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826963C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826963C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826963C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826963CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826963D0: 386A1B50  addi r3, r10, 0x1b50
	ctx.r[3].s64 = ctx.r[10].s64 + 6992;
	// 826963D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826963D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826963DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826963E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826963E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826963E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826963EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826963F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826963F4: 4BDD0A2D  bl 0x82466e20
	ctx.lr = 0x826963F8;
	sub_82466E20(ctx, base);
	// 826963F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826963FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696408 size=116
    let mut pc: u32 = 0x82696408;
    'dispatch: loop {
        match pc {
            0x82696408 => {
    //   block [0x82696408..0x8269647C)
	// 82696408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269640C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696414: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696418: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269641C: 390ACE80  addi r8, r10, -0x3180
	ctx.r[8].s64 = ctx.r[10].s64 + -12672;
	// 82696420: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696424: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696428: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 8269642C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696430: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269643C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82696440: 396B8BD4  addi r11, r11, -0x742c
	ctx.r[11].s64 = ctx.r[11].s64 + -29740;
	// 82696444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696448: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269644C: 386A1B80  addi r3, r10, 0x1b80
	ctx.r[3].s64 = ctx.r[10].s64 + 7040;
	// 82696450: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696458: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269645C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696468: 4BDD09B9  bl 0x82466e20
	ctx.lr = 0x8269646C;
	sub_82466E20(ctx, base);
	// 8269646C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696480 size=112
    let mut pc: u32 = 0x82696480;
    'dispatch: loop {
        match pc {
            0x82696480 => {
    //   block [0x82696480..0x826964F0)
	// 82696480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269648C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696490: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696494: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269649C: 390BCF28  addi r8, r11, -0x30d8
	ctx.r[8].s64 = ctx.r[11].s64 + -12504;
	// 826964A0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826964A4: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826964A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826964AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826964B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826964B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826964B8: 386A1BB0  addi r3, r10, 0x1bb0
	ctx.r[3].s64 = ctx.r[10].s64 + 7088;
	// 826964BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826964C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826964C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826964C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826964CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826964D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826964D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826964D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826964DC: 4BDD0945  bl 0x82466e20
	ctx.lr = 0x826964E0;
	sub_82466E20(ctx, base);
	// 826964E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826964E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826964E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826964EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826964F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826964F0 size=24
    let mut pc: u32 = 0x826964F0;
    'dispatch: loop {
        match pc {
            0x826964F0 => {
    //   block [0x826964F0..0x82696508)
	// 826964F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826964F4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826964F8: 394A52B0  addi r10, r10, 0x52b0
	ctx.r[10].s64 = ctx.r[10].s64 + 21168;
	// 826964FC: 816BD060  lwz r11, -0x2fa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12192 as u32) ) } as u64;
	// 82696500: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82696504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696508 size=116
    let mut pc: u32 = 0x82696508;
    'dispatch: loop {
        match pc {
            0x82696508 => {
    //   block [0x82696508..0x8269657C)
	// 82696508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696514: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696518: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269651C: 392B8C04  addi r9, r11, -0x73fc
	ctx.r[9].s64 = ctx.r[11].s64 + -29692;
	// 82696520: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696524: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696528: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269652C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82696530: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696534: 388A7B04  addi r4, r10, 0x7b04
	ctx.r[4].s64 = ctx.r[10].s64 + 31492;
	// 82696538: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269653C: 396B52B0  addi r11, r11, 0x52b0
	ctx.r[11].s64 = ctx.r[11].s64 + 21168;
	// 82696540: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82696544: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696548: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269654C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696550: 386A1BE0  addi r3, r10, 0x1be0
	ctx.r[3].s64 = ctx.r[10].s64 + 7136;
	// 82696554: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82696558: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269655C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696560: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82696564: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82696568: 4BDD08B9  bl 0x82466e20
	ctx.lr = 0x8269656C;
	sub_82466E20(ctx, base);
	// 8269656C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696580 size=112
    let mut pc: u32 = 0x82696580;
    'dispatch: loop {
        match pc {
            0x82696580 => {
    //   block [0x82696580..0x826965F0)
	// 82696580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269658C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696590: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696594: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269659C: 390BD064  addi r8, r11, -0x2f9c
	ctx.r[8].s64 = ctx.r[11].s64 + -12188;
	// 826965A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826965A4: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826965A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826965AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826965B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826965B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826965B8: 386A1C10  addi r3, r10, 0x1c10
	ctx.r[3].s64 = ctx.r[10].s64 + 7184;
	// 826965BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826965C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826965C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826965C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826965CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826965D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826965D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826965D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826965DC: 4BDD0845  bl 0x82466e20
	ctx.lr = 0x826965E0;
	sub_82466E20(ctx, base);
	// 826965E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826965E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826965E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826965EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826965F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826965F0 size=24
    let mut pc: u32 = 0x826965F0;
    'dispatch: loop {
        match pc {
            0x826965F0 => {
    //   block [0x826965F0..0x82696608)
	// 826965F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826965F4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826965F8: 394A5388  addi r10, r10, 0x5388
	ctx.r[10].s64 = ctx.r[10].s64 + 21384;
	// 826965FC: 816BD07C  lwz r11, -0x2f84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12164 as u32) ) } as u64;
	// 82696600: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82696604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696608 size=116
    let mut pc: u32 = 0x82696608;
    'dispatch: loop {
        match pc {
            0x82696608 => {
    //   block [0x82696608..0x8269667C)
	// 82696608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269660C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696614: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696618: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269661C: 392B8CA0  addi r9, r11, -0x7360
	ctx.r[9].s64 = ctx.r[11].s64 + -29536;
	// 82696620: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82696624: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82696628: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269662C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82696630: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696634: 388AB34C  addi r4, r10, -0x4cb4
	ctx.r[4].s64 = ctx.r[10].s64 + -19636;
	// 82696638: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269663C: 396B5388  addi r11, r11, 0x5388
	ctx.r[11].s64 = ctx.r[11].s64 + 21384;
	// 82696640: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82696644: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696648: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269664C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696650: 386A1C40  addi r3, r10, 0x1c40
	ctx.r[3].s64 = ctx.r[10].s64 + 7232;
	// 82696654: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82696658: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269665C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696660: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82696664: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82696668: 4BDD07B9  bl 0x82466e20
	ctx.lr = 0x8269666C;
	sub_82466E20(ctx, base);
	// 8269666C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696680 size=116
    let mut pc: u32 = 0x82696680;
    'dispatch: loop {
        match pc {
            0x82696680 => {
    //   block [0x82696680..0x826966F4)
	// 82696680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269668C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696690: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82696694: 390BD084  addi r8, r11, -0x2f7c
	ctx.r[8].s64 = ctx.r[11].s64 + -12156;
	// 82696698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269669C: 392A8CFC  addi r9, r10, -0x7304
	ctx.r[9].s64 = ctx.r[10].s64 + -29444;
	// 826966A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826966A4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826966A8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826966AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826966B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826966B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826966B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826966BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826966C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826966C4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826966C8: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826966CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826966D0: 386B1C70  addi r3, r11, 0x1c70
	ctx.r[3].s64 = ctx.r[11].s64 + 7280;
	// 826966D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826966D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826966DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826966E0: 4BDD0741  bl 0x82466e20
	ctx.lr = 0x826966E4;
	sub_82466E20(ctx, base);
	// 826966E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826966E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826966EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826966F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826966F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826966F8 size=100
    let mut pc: u32 = 0x826966F8;
    'dispatch: loop {
        match pc {
            0x826966F8 => {
    //   block [0x826966F8..0x8269675C)
	// 826966F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826966FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696704: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269670C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696718: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 8269671C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269672C: 386A1CA0  addi r3, r10, 0x1ca0
	ctx.r[3].s64 = ctx.r[10].s64 + 7328;
	// 82696730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269673C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82696744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696748: 4BDD06D9  bl 0x82466e20
	ctx.lr = 0x8269674C;
	sub_82466E20(ctx, base);
	// 8269674C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696760 size=112
    let mut pc: u32 = 0x82696760;
    'dispatch: loop {
        match pc {
            0x82696760 => {
    //   block [0x82696760..0x826967D0)
	// 82696760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269676C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696770: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696774: 38AA1CA0  addi r5, r10, 0x1ca0
	ctx.r[5].s64 = ctx.r[10].s64 + 7328;
	// 82696778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269677C: 390BD0B4  addi r8, r11, -0x2f4c
	ctx.r[8].s64 = ctx.r[11].s64 + -12108;
	// 82696780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82696784: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82696788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269678C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696798: 386A1CD0  addi r3, r10, 0x1cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 7376;
	// 8269679C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826967A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826967A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826967A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826967AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826967B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826967B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826967B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826967BC: 4BDD0665  bl 0x82466e20
	ctx.lr = 0x826967C0;
	sub_82466E20(ctx, base);
	// 826967C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826967C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826967C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826967CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826967D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826967D0 size=112
    let mut pc: u32 = 0x826967D0;
    'dispatch: loop {
        match pc {
            0x826967D0 => {
    //   block [0x826967D0..0x82696840)
	// 826967D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826967D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826967D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826967DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826967E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826967E4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826967E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826967EC: 390BD0D0  addi r8, r11, -0x2f30
	ctx.r[8].s64 = ctx.r[11].s64 + -12080;
	// 826967F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826967F4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826967F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826967FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696808: 386A1D00  addi r3, r10, 0x1d00
	ctx.r[3].s64 = ctx.r[10].s64 + 7424;
	// 8269680C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269681C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269682C: 4BDD05F5  bl 0x82466e20
	ctx.lr = 0x82696830;
	sub_82466E20(ctx, base);
	// 82696830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696840 size=112
    let mut pc: u32 = 0x82696840;
    'dispatch: loop {
        match pc {
            0x82696840 => {
    //   block [0x82696840..0x826968B0)
	// 82696840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269684C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696854: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269685C: 390BD118  addi r8, r11, -0x2ee8
	ctx.r[8].s64 = ctx.r[11].s64 + -12008;
	// 82696860: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82696864: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82696868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269686C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696878: 386A1D30  addi r3, r10, 0x1d30
	ctx.r[3].s64 = ctx.r[10].s64 + 7472;
	// 8269687C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269688C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269689C: 4BDD0585  bl 0x82466e20
	ctx.lr = 0x826968A0;
	sub_82466E20(ctx, base);
	// 826968A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826968A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826968A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826968AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826968B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826968B0 size=116
    let mut pc: u32 = 0x826968B0;
    'dispatch: loop {
        match pc {
            0x826968B0 => {
    //   block [0x826968B0..0x82696924)
	// 826968B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826968B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826968B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826968BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826968C0: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826968C4: 390AD1C0  addi r8, r10, -0x2e40
	ctx.r[8].s64 = ctx.r[10].s64 + -11840;
	// 826968C8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826968CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826968D0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826968D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826968D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826968DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826968E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826968E4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826968E8: 396B8D10  addi r11, r11, -0x72f0
	ctx.r[11].s64 = ctx.r[11].s64 + -29424;
	// 826968EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826968F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826968F4: 386A1D60  addi r3, r10, 0x1d60
	ctx.r[3].s64 = ctx.r[10].s64 + 7520;
	// 826968F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826968FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696900: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269690C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696910: 4BDD0511  bl 0x82466e20
	ctx.lr = 0x82696914;
	sub_82466E20(ctx, base);
	// 82696914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269691C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696928 size=112
    let mut pc: u32 = 0x82696928;
    'dispatch: loop {
        match pc {
            0x82696928 => {
    //   block [0x82696928..0x82696998)
	// 82696928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696938: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269693C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82696940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696944: 390BD2C8  addi r8, r11, -0x2d38
	ctx.r[8].s64 = ctx.r[11].s64 + -11576;
	// 82696948: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269694C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82696950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269695C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696960: 386A1D90  addi r3, r10, 0x1d90
	ctx.r[3].s64 = ctx.r[10].s64 + 7568;
	// 82696964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269696C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269697C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696984: 4BDD049D  bl 0x82466e20
	ctx.lr = 0x82696988;
	sub_82466E20(ctx, base);
	// 82696988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269698C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696998 size=100
    let mut pc: u32 = 0x82696998;
    'dispatch: loop {
        match pc {
            0x82696998 => {
    //   block [0x82696998..0x826969FC)
	// 82696998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826969A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826969A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826969A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826969AC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826969B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826969B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826969B8: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826969BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826969C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826969C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826969C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826969CC: 386A1DC0  addi r3, r10, 0x1dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 7616;
	// 826969D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826969D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826969D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826969DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826969E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826969E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826969E8: 4BDD0439  bl 0x82466e20
	ctx.lr = 0x826969EC;
	sub_82466E20(ctx, base);
	// 826969EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826969F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826969F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826969F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696A00 size=108
    let mut pc: u32 = 0x82696A00;
    'dispatch: loop {
        match pc {
            0x82696A00 => {
    //   block [0x82696A00..0x82696A6C)
	// 82696A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696A0C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696A14: 38EBD2F8  addi r7, r11, -0x2d08
	ctx.r[7].s64 = ctx.r[11].s64 + -11528;
	// 82696A18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696A1C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82696A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696A24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696A30: 386A1DF0  addi r3, r10, 0x1df0
	ctx.r[3].s64 = ctx.r[10].s64 + 7664;
	// 82696A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696A58: 4BDD03C9  bl 0x82466e20
	ctx.lr = 0x82696A5C;
	sub_82466E20(ctx, base);
	// 82696A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696A70 size=112
    let mut pc: u32 = 0x82696A70;
    'dispatch: loop {
        match pc {
            0x82696A70 => {
    //   block [0x82696A70..0x82696AE0)
	// 82696A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696A7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696A80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696A84: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696A8C: 390BD328  addi r8, r11, -0x2cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -11480;
	// 82696A90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82696A94: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 82696A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696A9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696AA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696AA8: 386A1E20  addi r3, r10, 0x1e20
	ctx.r[3].s64 = ctx.r[10].s64 + 7712;
	// 82696AAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82696AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696ACC: 4BDD0355  bl 0x82466e20
	ctx.lr = 0x82696AD0;
	sub_82466E20(ctx, base);
	// 82696AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696AE0 size=108
    let mut pc: u32 = 0x82696AE0;
    'dispatch: loop {
        match pc {
            0x82696AE0 => {
    //   block [0x82696AE0..0x82696B4C)
	// 82696AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696AEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696AF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696AF4: 38EBD358  addi r7, r11, -0x2ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -11432;
	// 82696AF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696AFC: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 82696B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696B04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696B10: 386A1E50  addi r3, r10, 0x1e50
	ctx.r[3].s64 = ctx.r[10].s64 + 7760;
	// 82696B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696B38: 4BDD02E9  bl 0x82466e20
	ctx.lr = 0x82696B3C;
	sub_82466E20(ctx, base);
	// 82696B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696B50 size=116
    let mut pc: u32 = 0x82696B50;
    'dispatch: loop {
        match pc {
            0x82696B50 => {
    //   block [0x82696B50..0x82696BC4)
	// 82696B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696B5C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696B60: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82696B64: 390AD388  addi r8, r10, -0x2c78
	ctx.r[8].s64 = ctx.r[10].s64 + -11384;
	// 82696B68: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696B6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696B70: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696B74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696B78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696B84: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 82696B88: 396B8D44  addi r11, r11, -0x72bc
	ctx.r[11].s64 = ctx.r[11].s64 + -29372;
	// 82696B8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696B90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696B94: 386A1E80  addi r3, r10, 0x1e80
	ctx.r[3].s64 = ctx.r[10].s64 + 7808;
	// 82696B98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696B9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696BA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696BB0: 4BDD0271  bl 0x82466e20
	ctx.lr = 0x82696BB4;
	sub_82466E20(ctx, base);
	// 82696BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696BC8 size=108
    let mut pc: u32 = 0x82696BC8;
    'dispatch: loop {
        match pc {
            0x82696BC8 => {
    //   block [0x82696BC8..0x82696C34)
	// 82696BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696BD4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696BD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696BDC: 38EBD3D0  addi r7, r11, -0x2c30
	ctx.r[7].s64 = ctx.r[11].s64 + -11312;
	// 82696BE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696BE4: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 82696BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696BEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696BF8: 386A1EB0  addi r3, r10, 0x1eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 7856;
	// 82696BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696C20: 4BDD0201  bl 0x82466e20
	ctx.lr = 0x82696C24;
	sub_82466E20(ctx, base);
	// 82696C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696C38 size=116
    let mut pc: u32 = 0x82696C38;
    'dispatch: loop {
        match pc {
            0x82696C38 => {
    //   block [0x82696C38..0x82696CAC)
	// 82696C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696C44: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696C48: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82696C4C: 390AD400  addi r8, r10, -0x2c00
	ctx.r[8].s64 = ctx.r[10].s64 + -11264;
	// 82696C50: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696C54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696C58: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696C5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696C60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696C6C: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82696C70: 396B8D54  addi r11, r11, -0x72ac
	ctx.r[11].s64 = ctx.r[11].s64 + -29356;
	// 82696C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696C78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696C7C: 386A1EE0  addi r3, r10, 0x1ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 7904;
	// 82696C80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696C84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696C88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696C98: 4BDD0189  bl 0x82466e20
	ctx.lr = 0x82696C9C;
	sub_82466E20(ctx, base);
	// 82696C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696CB0 size=108
    let mut pc: u32 = 0x82696CB0;
    'dispatch: loop {
        match pc {
            0x82696CB0 => {
    //   block [0x82696CB0..0x82696D1C)
	// 82696CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696CBC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696CC4: 38EBD448  addi r7, r11, -0x2bb8
	ctx.r[7].s64 = ctx.r[11].s64 + -11192;
	// 82696CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82696CCC: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 82696CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696CE0: 386A1F10  addi r3, r10, 0x1f10
	ctx.r[3].s64 = ctx.r[10].s64 + 7952;
	// 82696CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696D08: 4BDD0119  bl 0x82466e20
	ctx.lr = 0x82696D0C;
	sub_82466E20(ctx, base);
	// 82696D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696D20 size=116
    let mut pc: u32 = 0x82696D20;
    'dispatch: loop {
        match pc {
            0x82696D20 => {
    //   block [0x82696D20..0x82696D94)
	// 82696D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696D2C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696D30: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82696D34: 390AD478  addi r8, r10, -0x2b88
	ctx.r[8].s64 = ctx.r[10].s64 + -11144;
	// 82696D38: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696D3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82696D40: 38AA1DC0  addi r5, r10, 0x1dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 7616;
	// 82696D44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696D48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696D50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696D54: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82696D58: 396B8D64  addi r11, r11, -0x729c
	ctx.r[11].s64 = ctx.r[11].s64 + -29340;
	// 82696D5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696D60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696D64: 386A1F40  addi r3, r10, 0x1f40
	ctx.r[3].s64 = ctx.r[10].s64 + 8000;
	// 82696D68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82696D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696D70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82696D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696D80: 4BDD00A1  bl 0x82466e20
	ctx.lr = 0x82696D84;
	sub_82466E20(ctx, base);
	// 82696D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696D98 size=108
    let mut pc: u32 = 0x82696D98;
    'dispatch: loop {
        match pc {
            0x82696D98 => {
    //   block [0x82696D98..0x82696E04)
	// 82696D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696DA4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696DAC: 38EBD4C0  addi r7, r11, -0x2b40
	ctx.r[7].s64 = ctx.r[11].s64 + -11072;
	// 82696DB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82696DB4: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 82696DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696DC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696DC8: 386A1F70  addi r3, r10, 0x1f70
	ctx.r[3].s64 = ctx.r[10].s64 + 8048;
	// 82696DCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696DF0: 4BDD0031  bl 0x82466e20
	ctx.lr = 0x82696DF4;
	sub_82466E20(ctx, base);
	// 82696DF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82696E08 size=24
    let mut pc: u32 = 0x82696E08;
    'dispatch: loop {
        match pc {
            0x82696E08 => {
    //   block [0x82696E08..0x82696E20)
	// 82696E08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696E0C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696E10: 394A5418  addi r10, r10, 0x5418
	ctx.r[10].s64 = ctx.r[10].s64 + 21528;
	// 82696E14: 816BD520  lwz r11, -0x2ae0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10976 as u32) ) } as u64;
	// 82696E18: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82696E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696E20 size=112
    let mut pc: u32 = 0x82696E20;
    'dispatch: loop {
        match pc {
            0x82696E20 => {
    //   block [0x82696E20..0x82696E90)
	// 82696E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696E2C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82696E30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696E34: 392A8E30  addi r9, r10, -0x71d0
	ctx.r[9].s64 = ctx.r[10].s64 + -29136;
	// 82696E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696E3C: 390B5418  addi r8, r11, 0x5418
	ctx.r[8].s64 = ctx.r[11].s64 + 21528;
	// 82696E40: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82696E44: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82696E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696E4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82696E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696E58: 386A1FA0  addi r3, r10, 0x1fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 8096;
	// 82696E5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82696E60: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82696E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696E7C: 4BDCFFA5  bl 0x82466e20
	ctx.lr = 0x82696E80;
	sub_82466E20(ctx, base);
	// 82696E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696E90 size=108
    let mut pc: u32 = 0x82696E90;
    'dispatch: loop {
        match pc {
            0x82696E90 => {
    //   block [0x82696E90..0x82696EFC)
	// 82696E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696E9C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696EA4: 38EBD528  addi r7, r11, -0x2ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -10968;
	// 82696EA8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82696EAC: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 82696EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696EB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696EB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696EC0: 386A1FD0  addi r3, r10, 0x1fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 8144;
	// 82696EC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696EE8: 4BDCFF39  bl 0x82466e20
	ctx.lr = 0x82696EEC;
	sub_82466E20(ctx, base);
	// 82696EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696F00 size=108
    let mut pc: u32 = 0x82696F00;
    'dispatch: loop {
        match pc {
            0x82696F00 => {
    //   block [0x82696F00..0x82696F6C)
	// 82696F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696F0C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696F10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82696F14: 38EBD5A0  addi r7, r11, -0x2a60
	ctx.r[7].s64 = ctx.r[11].s64 + -10848;
	// 82696F18: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82696F1C: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 82696F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696F24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696F30: 386A2000  addi r3, r10, 0x2000
	ctx.r[3].s64 = ctx.r[10].s64 + 8192;
	// 82696F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696F58: 4BDCFEC9  bl 0x82466e20
	ctx.lr = 0x82696F5C;
	sub_82466E20(ctx, base);
	// 82696F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696F70 size=108
    let mut pc: u32 = 0x82696F70;
    'dispatch: loop {
        match pc {
            0x82696F70 => {
    //   block [0x82696F70..0x82696FDC)
	// 82696F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82696F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82696F7C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82696F84: 38EBD600  addi r7, r11, -0x2a00
	ctx.r[7].s64 = ctx.r[11].s64 + -10752;
	// 82696F88: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82696F8C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82696F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82696F94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82696F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82696F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82696FA0: 386A2030  addi r3, r10, 0x2030
	ctx.r[3].s64 = ctx.r[10].s64 + 8240;
	// 82696FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82696FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82696FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82696FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82696FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82696FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82696FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82696FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82696FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82696FC8: 4BDCFE59  bl 0x82466e20
	ctx.lr = 0x82696FCC;
	sub_82466E20(ctx, base);
	// 82696FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82696FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82696FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82696FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82696FE0 size=24
    let mut pc: u32 = 0x82696FE0;
    'dispatch: loop {
        match pc {
            0x82696FE0 => {
    //   block [0x82696FE0..0x82696FF8)
	// 82696FE0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82696FE4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82696FE8: 394A5520  addi r10, r10, 0x5520
	ctx.r[10].s64 = ctx.r[10].s64 + 21792;
	// 82696FEC: 816BD0CC  lwz r11, -0x2f34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12084 as u32) ) } as u64;
	// 82696FF0: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82696FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82696FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82696FF8 size=116
    let mut pc: u32 = 0x82696FF8;
    'dispatch: loop {
        match pc {
            0x82696FF8 => {
    //   block [0x82696FF8..0x8269706C)
	// 82696FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82696FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697004: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697008: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269700C: 392B8D94  addi r9, r11, -0x726c
	ctx.r[9].s64 = ctx.r[11].s64 + -29292;
	// 82697010: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697018: 38E900C4  addi r7, r9, 0xc4
	ctx.r[7].s64 = ctx.r[9].s64 + 196;
	// 8269701C: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82697020: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697024: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82697028: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269702C: 396B5520  addi r11, r11, 0x5520
	ctx.r[11].s64 = ctx.r[11].s64 + 21792;
	// 82697030: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82697034: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697038: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269703C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697040: 386A2060  addi r3, r10, 0x2060
	ctx.r[3].s64 = ctx.r[10].s64 + 8288;
	// 82697044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82697048: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269704C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697050: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82697054: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82697058: 4BDCFDC9  bl 0x82466e20
	ctx.lr = 0x8269705C;
	sub_82466E20(ctx, base);
	// 8269705C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697070 size=36
    let mut pc: u32 = 0x82697070;
    'dispatch: loop {
        match pc {
            0x82697070 => {
    //   block [0x82697070..0x82697094)
	// 82697070: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697074: 814BD6A8  lwz r10, -0x2958(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10584 as u32) ) } as u64;
	// 82697078: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269707C: 396B57F0  addi r11, r11, 0x57f0
	ctx.r[11].s64 = ctx.r[11].s64 + 22512;
	// 82697080: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82697084: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697088: 814AD6AC  lwz r10, -0x2954(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10580 as u32) ) } as u64;
	// 8269708C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82697090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697098 size=116
    let mut pc: u32 = 0x82697098;
    'dispatch: loop {
        match pc {
            0x82697098 => {
    //   block [0x82697098..0x8269710C)
	// 82697098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269709C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826970A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826970A4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826970A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826970AC: 390B57F0  addi r8, r11, 0x57f0
	ctx.r[8].s64 = ctx.r[11].s64 + 22512;
	// 826970B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826970B4: 392A8F18  addi r9, r10, -0x70e8
	ctx.r[9].s64 = ctx.r[10].s64 + -28904;
	// 826970B8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826970BC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826970C0: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 826970C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826970C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826970CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826970D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826970D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826970D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826970DC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826970E0: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826970E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826970E8: 386B2090  addi r3, r11, 0x2090
	ctx.r[3].s64 = ctx.r[11].s64 + 8336;
	// 826970EC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826970F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826970F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826970F8: 4BDCFD29  bl 0x82466e20
	ctx.lr = 0x826970FC;
	sub_82466E20(ctx, base);
	// 826970FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697110 size=24
    let mut pc: u32 = 0x82697110;
    'dispatch: loop {
        match pc {
            0x82697110 => {
    //   block [0x82697110..0x82697128)
	// 82697110: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697114: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697118: 394A5820  addi r10, r10, 0x5820
	ctx.r[10].s64 = ctx.r[10].s64 + 22560;
	// 8269711C: 816BD6B4  lwz r11, -0x294c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10572 as u32) ) } as u64;
	// 82697120: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82697124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697128 size=116
    let mut pc: u32 = 0x82697128;
    'dispatch: loop {
        match pc {
            0x82697128 => {
    //   block [0x82697128..0x8269719C)
	// 82697128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269712C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697134: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697138: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269713C: 390B5820  addi r8, r11, 0x5820
	ctx.r[8].s64 = ctx.r[11].s64 + 22560;
	// 82697140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697144: 392A8F70  addi r9, r10, -0x7090
	ctx.r[9].s64 = ctx.r[10].s64 + -28816;
	// 82697148: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269714C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82697150: 38AA2090  addi r5, r10, 0x2090
	ctx.r[5].s64 = ctx.r[10].s64 + 8336;
	// 82697154: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269715C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269716C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82697170: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82697174: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697178: 386B20C0  addi r3, r11, 0x20c0
	ctx.r[3].s64 = ctx.r[11].s64 + 8384;
	// 8269717C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82697180: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697188: 4BDCFC99  bl 0x82466e20
	ctx.lr = 0x8269718C;
	sub_82466E20(ctx, base);
	// 8269718C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826971A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826971A0 size=116
    let mut pc: u32 = 0x826971A0;
    'dispatch: loop {
        match pc {
            0x826971A0 => {
    //   block [0x826971A0..0x82697214)
	// 826971A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826971A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826971A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826971AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826971B0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826971B4: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826971B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826971BC: 392A8FB8  addi r9, r10, -0x7048
	ctx.r[9].s64 = ctx.r[10].s64 + -28744;
	// 826971C0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826971C4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826971C8: 38AA2090  addi r5, r10, 0x2090
	ctx.r[5].s64 = ctx.r[10].s64 + 8336;
	// 826971CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826971D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826971D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826971D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826971DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826971E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826971E4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826971E8: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826971EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826971F0: 386B20F0  addi r3, r11, 0x20f0
	ctx.r[3].s64 = ctx.r[11].s64 + 8432;
	// 826971F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826971F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826971FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697200: 4BDCFC21  bl 0x82466e20
	ctx.lr = 0x82697204;
	sub_82466E20(ctx, base);
	// 82697204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269720C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697218 size=112
    let mut pc: u32 = 0x82697218;
    'dispatch: loop {
        match pc {
            0x82697218 => {
    //   block [0x82697218..0x82697288)
	// 82697218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269721C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697224: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697228: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269722C: 38EAD7C8  addi r7, r10, -0x2838
	ctx.r[7].s64 = ctx.r[10].s64 + -10296;
	// 82697230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697234: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697238: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8269723C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697240: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697244: 396B8FCC  addi r11, r11, -0x7034
	ctx.r[11].s64 = ctx.r[11].s64 + -28724;
	// 82697248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269724C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697250: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697254: 386A2120  addi r3, r10, 0x2120
	ctx.r[3].s64 = ctx.r[10].s64 + 8480;
	// 82697258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269725C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697264: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697268: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269726C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697270: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697274: 4BDCFBAD  bl 0x82466e20
	ctx.lr = 0x82697278;
	sub_82466E20(ctx, base);
	// 82697278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269727C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697288 size=112
    let mut pc: u32 = 0x82697288;
    'dispatch: loop {
        match pc {
            0x82697288 => {
    //   block [0x82697288..0x826972F8)
	// 82697288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269728C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697298: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269729C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826972A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826972A4: 390BD858  addi r8, r11, -0x27a8
	ctx.r[8].s64 = ctx.r[11].s64 + -10152;
	// 826972A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826972AC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826972B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826972B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826972B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826972BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826972C0: 386A2150  addi r3, r10, 0x2150
	ctx.r[3].s64 = ctx.r[10].s64 + 8528;
	// 826972C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826972C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826972CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826972D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826972D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826972D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826972DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826972E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826972E4: 4BDCFB3D  bl 0x82466e20
	ctx.lr = 0x826972E8;
	sub_82466E20(ctx, base);
	// 826972E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826972EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826972F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826972F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826972F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826972F8 size=24
    let mut pc: u32 = 0x826972F8;
    'dispatch: loop {
        match pc {
            0x826972F8 => {
    //   block [0x826972F8..0x82697310)
	// 826972F8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826972FC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697300: 394A5958  addi r10, r10, 0x5958
	ctx.r[10].s64 = ctx.r[10].s64 + 22872;
	// 82697304: 816BD6BC  lwz r11, -0x2944(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10564 as u32) ) } as u64;
	// 82697308: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697310 size=112
    let mut pc: u32 = 0x82697310;
    'dispatch: loop {
        match pc {
            0x82697310 => {
    //   block [0x82697310..0x82697380)
	// 82697310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269731C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82697320: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697324: 392A9018  addi r9, r10, -0x6fe8
	ctx.r[9].s64 = ctx.r[10].s64 + -28648;
	// 82697328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269732C: 390B5958  addi r8, r11, 0x5958
	ctx.r[8].s64 = ctx.r[11].s64 + 22872;
	// 82697330: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82697334: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82697338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269733C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697348: 386A2180  addi r3, r10, 0x2180
	ctx.r[3].s64 = ctx.r[10].s64 + 8576;
	// 8269734C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697350: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82697354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269736C: 4BDCFAB5  bl 0x82466e20
	ctx.lr = 0x82697370;
	sub_82466E20(ctx, base);
	// 82697370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269737C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697380 size=108
    let mut pc: u32 = 0x82697380;
    'dispatch: loop {
        match pc {
            0x82697380 => {
    //   block [0x82697380..0x826973EC)
	// 82697380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269738C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697394: 38EBD878  addi r7, r11, -0x2788
	ctx.r[7].s64 = ctx.r[11].s64 + -10120;
	// 82697398: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269739C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826973A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826973A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826973A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826973AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826973B0: 386A21B0  addi r3, r10, 0x21b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8624;
	// 826973B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826973B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826973BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826973C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826973C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826973C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826973CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826973D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826973D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826973D8: 4BDCFA49  bl 0x82466e20
	ctx.lr = 0x826973DC;
	sub_82466E20(ctx, base);
	// 826973DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826973E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826973E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826973E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


