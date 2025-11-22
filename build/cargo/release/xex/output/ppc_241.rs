pub fn sub_8324B480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B480 size=64
    let mut pc: u32 = 0x8324B480;
    'dispatch: loop {
        match pc {
            0x8324B480 => {
    //   block [0x8324B480..0x8324B4C0)
	// 8324B480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B48C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B490: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B494: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B498: 386A78F4  addi r3, r10, 0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30964;
	// 8324B49C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B4A0: 4AFE1A31  bl 0x8222ced0
	ctx.lr = 0x8324B4A4;
	sub_8222CED0(ctx, base);
	// 8324B4A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B4A8: 38698200  addi r3, r9, -0x7e00
	ctx.r[3].s64 = ctx.r[9].s64 + -32256;
	// 8324B4AC: 4BA5EA75  bl 0x82ca9f20
	ctx.lr = 0x8324B4B0;
	sub_82CA9F20(ctx, base);
	// 8324B4B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B4C0 size=64
    let mut pc: u32 = 0x8324B4C0;
    'dispatch: loop {
        match pc {
            0x8324B4C0 => {
    //   block [0x8324B4C0..0x8324B500)
	// 8324B4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B4CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B4D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B4D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B4D8: 386A78F8  addi r3, r10, 0x78f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30968;
	// 8324B4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B4E0: 4AFE19F1  bl 0x8222ced0
	ctx.lr = 0x8324B4E4;
	sub_8222CED0(ctx, base);
	// 8324B4E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B4E8: 38698210  addi r3, r9, -0x7df0
	ctx.r[3].s64 = ctx.r[9].s64 + -32240;
	// 8324B4EC: 4BA5EA35  bl 0x82ca9f20
	ctx.lr = 0x8324B4F0;
	sub_82CA9F20(ctx, base);
	// 8324B4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B500 size=64
    let mut pc: u32 = 0x8324B500;
    'dispatch: loop {
        match pc {
            0x8324B500 => {
    //   block [0x8324B500..0x8324B540)
	// 8324B500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B50C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B510: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B514: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B518: 386A78FC  addi r3, r10, 0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + 30972;
	// 8324B51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B520: 4AFE19B1  bl 0x8222ced0
	ctx.lr = 0x8324B524;
	sub_8222CED0(ctx, base);
	// 8324B524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B528: 38698220  addi r3, r9, -0x7de0
	ctx.r[3].s64 = ctx.r[9].s64 + -32224;
	// 8324B52C: 4BA5E9F5  bl 0x82ca9f20
	ctx.lr = 0x8324B530;
	sub_82CA9F20(ctx, base);
	// 8324B530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B540 size=64
    let mut pc: u32 = 0x8324B540;
    'dispatch: loop {
        match pc {
            0x8324B540 => {
    //   block [0x8324B540..0x8324B580)
	// 8324B540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B54C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B550: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B554: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B558: 386A7900  addi r3, r10, 0x7900
	ctx.r[3].s64 = ctx.r[10].s64 + 30976;
	// 8324B55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B560: 4AFE1971  bl 0x8222ced0
	ctx.lr = 0x8324B564;
	sub_8222CED0(ctx, base);
	// 8324B564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B568: 38698230  addi r3, r9, -0x7dd0
	ctx.r[3].s64 = ctx.r[9].s64 + -32208;
	// 8324B56C: 4BA5E9B5  bl 0x82ca9f20
	ctx.lr = 0x8324B570;
	sub_82CA9F20(ctx, base);
	// 8324B570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B580 size=96
    let mut pc: u32 = 0x8324B580;
    'dispatch: loop {
        match pc {
            0x8324B580 => {
    //   block [0x8324B580..0x8324B5E0)
	// 8324B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B58C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8324B590: 4AFD3CC9  bl 0x8221f258
	ctx.lr = 0x8324B594;
	sub_8221F258(ctx, base);
	// 8324B594: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324B598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324B59C: 419A0008  beq cr6, 0x8324b5a4
	if ctx.cr[6].eq {
	pc = 0x8324B5A4; continue 'dispatch;
	}
	// 8324B5A0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324B5A4: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324B5A8: 41820008  beq 0x8324b5b0
	if ctx.cr[0].eq {
	pc = 0x8324B5B0; continue 'dispatch;
	}
	// 8324B5AC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324B5B0: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324B5B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324B5B8: 39097904  addi r8, r9, 0x7904
	ctx.r[8].s64 = ctx.r[9].s64 + 30980;
	// 8324B5BC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324B5C0: 38678240  addi r3, r7, -0x7dc0
	ctx.r[3].s64 = ctx.r[7].s64 + -32192;
	// 8324B5C4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324B5C8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324B5CC: 4BA5E955  bl 0x82ca9f20
	ctx.lr = 0x8324B5D0;
	sub_82CA9F20(ctx, base);
	// 8324B5D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324B5E0 size=12
    let mut pc: u32 = 0x8324B5E0;
    'dispatch: loop {
        match pc {
            0x8324B5E0 => {
    //   block [0x8324B5E0..0x8324B5EC)
	// 8324B5E0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324B5E4: 386B8300  addi r3, r11, -0x7d00
	ctx.r[3].s64 = ctx.r[11].s64 + -32000;
	// 8324B5E8: 4BA5E938  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B5F0 size=64
    let mut pc: u32 = 0x8324B5F0;
    'dispatch: loop {
        match pc {
            0x8324B5F0 => {
    //   block [0x8324B5F0..0x8324B630)
	// 8324B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B5FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8324B600: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B604: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8324B608: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 8324B60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B610: 4AFE18C1  bl 0x8222ced0
	ctx.lr = 0x8324B614;
	sub_8222CED0(ctx, base);
	// 8324B614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B618: 38698358  addi r3, r9, -0x7ca8
	ctx.r[3].s64 = ctx.r[9].s64 + -31912;
	// 8324B61C: 4BA5E905  bl 0x82ca9f20
	ctx.lr = 0x8324B620;
	sub_82CA9F20(ctx, base);
	// 8324B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B630 size=64
    let mut pc: u32 = 0x8324B630;
    'dispatch: loop {
        match pc {
            0x8324B630 => {
    //   block [0x8324B630..0x8324B670)
	// 8324B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B63C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8324B640: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B644: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8324B648: 386A7928  addi r3, r10, 0x7928
	ctx.r[3].s64 = ctx.r[10].s64 + 31016;
	// 8324B64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B650: 4AFE1881  bl 0x8222ced0
	ctx.lr = 0x8324B654;
	sub_8222CED0(ctx, base);
	// 8324B654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B658: 38698368  addi r3, r9, -0x7c98
	ctx.r[3].s64 = ctx.r[9].s64 + -31896;
	// 8324B65C: 4BA5E8C5  bl 0x82ca9f20
	ctx.lr = 0x8324B660;
	sub_82CA9F20(ctx, base);
	// 8324B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B670 size=144
    let mut pc: u32 = 0x8324B670;
    'dispatch: loop {
        match pc {
            0x8324B670 => {
    //   block [0x8324B670..0x8324B700)
	// 8324B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B67C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 8324B680: 4AFD3BD9  bl 0x8221f258
	ctx.lr = 0x8324B684;
	sub_8221F258(ctx, base);
	// 8324B684: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324B688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8324B68C: 419A0008  beq cr6, 0x8324b694
	if ctx.cr[6].eq {
	pc = 0x8324B694; continue 'dispatch;
	}
	// 8324B690: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8324B694: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324B698: 41820008  beq 0x8324b6a0
	if ctx.cr[0].eq {
	pc = 0x8324B6A0; continue 'dispatch;
	}
	// 8324B69C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8324B6A0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324B6A4: 41820008  beq 0x8324b6ac
	if ctx.cr[0].eq {
	pc = 0x8324B6AC; continue 'dispatch;
	}
	// 8324B6A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8324B6AC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324B6B0: 99430051  stb r10, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 8324B6B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8324B6B8: 3909792C  addi r8, r9, 0x792c
	ctx.r[8].s64 = ctx.r[9].s64 + 31020;
	// 8324B6BC: 99630050  stb r11, 0x50(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8324B6C0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324B6C4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8324B6C8: 99630051  stb r11, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 8324B6CC: 38678378  addi r3, r7, -0x7c88
	ctx.r[3].s64 = ctx.r[7].s64 + -31880;
	// 8324B6D0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6D4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324B6D8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6DC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324B6E0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6E4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324B6E8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324B6EC: 4BA5E835  bl 0x82ca9f20
	ctx.lr = 0x8324B6F0;
	sub_82CA9F20(ctx, base);
	// 8324B6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B700 size=64
    let mut pc: u32 = 0x8324B700;
    'dispatch: loop {
        match pc {
            0x8324B700 => {
    //   block [0x8324B700..0x8324B740)
	// 8324B700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B70C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B710: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B714: 388B9988  addi r4, r11, -0x6678
	ctx.r[4].s64 = ctx.r[11].s64 + -26232;
	// 8324B718: 386A7938  addi r3, r10, 0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + 31032;
	// 8324B71C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B720: 4AFE17B1  bl 0x8222ced0
	ctx.lr = 0x8324B724;
	sub_8222CED0(ctx, base);
	// 8324B724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B728: 38698388  addi r3, r9, -0x7c78
	ctx.r[3].s64 = ctx.r[9].s64 + -31864;
	// 8324B72C: 4BA5E7F5  bl 0x82ca9f20
	ctx.lr = 0x8324B730;
	sub_82CA9F20(ctx, base);
	// 8324B730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B740 size=64
    let mut pc: u32 = 0x8324B740;
    'dispatch: loop {
        match pc {
            0x8324B740 => {
    //   block [0x8324B740..0x8324B780)
	// 8324B740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B74C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B750: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B754: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B758: 386A793C  addi r3, r10, 0x793c
	ctx.r[3].s64 = ctx.r[10].s64 + 31036;
	// 8324B75C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B760: 4AFE1771  bl 0x8222ced0
	ctx.lr = 0x8324B764;
	sub_8222CED0(ctx, base);
	// 8324B764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B768: 38698398  addi r3, r9, -0x7c68
	ctx.r[3].s64 = ctx.r[9].s64 + -31848;
	// 8324B76C: 4BA5E7B5  bl 0x82ca9f20
	ctx.lr = 0x8324B770;
	sub_82CA9F20(ctx, base);
	// 8324B770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B780 size=64
    let mut pc: u32 = 0x8324B780;
    'dispatch: loop {
        match pc {
            0x8324B780 => {
    //   block [0x8324B780..0x8324B7C0)
	// 8324B780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B78C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B790: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B794: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B798: 386A7940  addi r3, r10, 0x7940
	ctx.r[3].s64 = ctx.r[10].s64 + 31040;
	// 8324B79C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B7A0: 4AFE1731  bl 0x8222ced0
	ctx.lr = 0x8324B7A4;
	sub_8222CED0(ctx, base);
	// 8324B7A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B7A8: 386983A8  addi r3, r9, -0x7c58
	ctx.r[3].s64 = ctx.r[9].s64 + -31832;
	// 8324B7AC: 4BA5E775  bl 0x82ca9f20
	ctx.lr = 0x8324B7B0;
	sub_82CA9F20(ctx, base);
	// 8324B7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B7C0 size=64
    let mut pc: u32 = 0x8324B7C0;
    'dispatch: loop {
        match pc {
            0x8324B7C0 => {
    //   block [0x8324B7C0..0x8324B800)
	// 8324B7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B7D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B7D4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B7D8: 386A7944  addi r3, r10, 0x7944
	ctx.r[3].s64 = ctx.r[10].s64 + 31044;
	// 8324B7DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B7E0: 4AFE16F1  bl 0x8222ced0
	ctx.lr = 0x8324B7E4;
	sub_8222CED0(ctx, base);
	// 8324B7E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B7E8: 386983B8  addi r3, r9, -0x7c48
	ctx.r[3].s64 = ctx.r[9].s64 + -31816;
	// 8324B7EC: 4BA5E735  bl 0x82ca9f20
	ctx.lr = 0x8324B7F0;
	sub_82CA9F20(ctx, base);
	// 8324B7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B800 size=64
    let mut pc: u32 = 0x8324B800;
    'dispatch: loop {
        match pc {
            0x8324B800 => {
    //   block [0x8324B800..0x8324B840)
	// 8324B800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B810: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B814: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B818: 386A7948  addi r3, r10, 0x7948
	ctx.r[3].s64 = ctx.r[10].s64 + 31048;
	// 8324B81C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B820: 4AFE16B1  bl 0x8222ced0
	ctx.lr = 0x8324B824;
	sub_8222CED0(ctx, base);
	// 8324B824: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B828: 386983C8  addi r3, r9, -0x7c38
	ctx.r[3].s64 = ctx.r[9].s64 + -31800;
	// 8324B82C: 4BA5E6F5  bl 0x82ca9f20
	ctx.lr = 0x8324B830;
	sub_82CA9F20(ctx, base);
	// 8324B830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B840 size=64
    let mut pc: u32 = 0x8324B840;
    'dispatch: loop {
        match pc {
            0x8324B840 => {
    //   block [0x8324B840..0x8324B880)
	// 8324B840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B84C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B850: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B854: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B858: 386A794C  addi r3, r10, 0x794c
	ctx.r[3].s64 = ctx.r[10].s64 + 31052;
	// 8324B85C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B860: 4AFE1671  bl 0x8222ced0
	ctx.lr = 0x8324B864;
	sub_8222CED0(ctx, base);
	// 8324B864: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B868: 386983D8  addi r3, r9, -0x7c28
	ctx.r[3].s64 = ctx.r[9].s64 + -31784;
	// 8324B86C: 4BA5E6B5  bl 0x82ca9f20
	ctx.lr = 0x8324B870;
	sub_82CA9F20(ctx, base);
	// 8324B870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B880 size=64
    let mut pc: u32 = 0x8324B880;
    'dispatch: loop {
        match pc {
            0x8324B880 => {
    //   block [0x8324B880..0x8324B8C0)
	// 8324B880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B88C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B890: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B894: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B898: 386A7950  addi r3, r10, 0x7950
	ctx.r[3].s64 = ctx.r[10].s64 + 31056;
	// 8324B89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B8A0: 4AFE1631  bl 0x8222ced0
	ctx.lr = 0x8324B8A4;
	sub_8222CED0(ctx, base);
	// 8324B8A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B8A8: 386983E8  addi r3, r9, -0x7c18
	ctx.r[3].s64 = ctx.r[9].s64 + -31768;
	// 8324B8AC: 4BA5E675  bl 0x82ca9f20
	ctx.lr = 0x8324B8B0;
	sub_82CA9F20(ctx, base);
	// 8324B8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B8C0 size=64
    let mut pc: u32 = 0x8324B8C0;
    'dispatch: loop {
        match pc {
            0x8324B8C0 => {
    //   block [0x8324B8C0..0x8324B900)
	// 8324B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B8CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B8D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B8D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B8D8: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 8324B8DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B8E0: 4AFE15F1  bl 0x8222ced0
	ctx.lr = 0x8324B8E4;
	sub_8222CED0(ctx, base);
	// 8324B8E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B8E8: 386983F8  addi r3, r9, -0x7c08
	ctx.r[3].s64 = ctx.r[9].s64 + -31752;
	// 8324B8EC: 4BA5E635  bl 0x82ca9f20
	ctx.lr = 0x8324B8F0;
	sub_82CA9F20(ctx, base);
	// 8324B8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B900 size=64
    let mut pc: u32 = 0x8324B900;
    'dispatch: loop {
        match pc {
            0x8324B900 => {
    //   block [0x8324B900..0x8324B940)
	// 8324B900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B90C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B910: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B914: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B918: 386A7958  addi r3, r10, 0x7958
	ctx.r[3].s64 = ctx.r[10].s64 + 31064;
	// 8324B91C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B920: 4AFE15B1  bl 0x8222ced0
	ctx.lr = 0x8324B924;
	sub_8222CED0(ctx, base);
	// 8324B924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B928: 38698408  addi r3, r9, -0x7bf8
	ctx.r[3].s64 = ctx.r[9].s64 + -31736;
	// 8324B92C: 4BA5E5F5  bl 0x82ca9f20
	ctx.lr = 0x8324B930;
	sub_82CA9F20(ctx, base);
	// 8324B930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B940 size=64
    let mut pc: u32 = 0x8324B940;
    'dispatch: loop {
        match pc {
            0x8324B940 => {
    //   block [0x8324B940..0x8324B980)
	// 8324B940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B94C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B950: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B954: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B958: 386A795C  addi r3, r10, 0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + 31068;
	// 8324B95C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B960: 4AFE1571  bl 0x8222ced0
	ctx.lr = 0x8324B964;
	sub_8222CED0(ctx, base);
	// 8324B964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B968: 38698418  addi r3, r9, -0x7be8
	ctx.r[3].s64 = ctx.r[9].s64 + -31720;
	// 8324B96C: 4BA5E5B5  bl 0x82ca9f20
	ctx.lr = 0x8324B970;
	sub_82CA9F20(ctx, base);
	// 8324B970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B980 size=56
    let mut pc: u32 = 0x8324B980;
    'dispatch: loop {
        match pc {
            0x8324B980 => {
    //   block [0x8324B980..0x8324B9B8)
	// 8324B980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B98C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324B994: 386B9D1C  addi r3, r11, -0x62e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25316;
	// 8324B998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324B99C: 4AFA83BD  bl 0x821f3d58
	ctx.lr = 0x8324B9A0;
	sub_821F3D58(ctx, base);
	// 8324B9A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B9A4: 906A7960  stw r3, 0x7960(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31072 as u32), ctx.r[3].u32 ) };
	// 8324B9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B9B8 size=56
    let mut pc: u32 = 0x8324B9B8;
    'dispatch: loop {
        match pc {
            0x8324B9B8 => {
    //   block [0x8324B9B8..0x8324B9F0)
	// 8324B9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B9C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324B9CC: 386B9D2C  addi r3, r11, -0x62d4
	ctx.r[3].s64 = ctx.r[11].s64 + -25300;
	// 8324B9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324B9D4: 4AFA8385  bl 0x821f3d58
	ctx.lr = 0x8324B9D8;
	sub_821F3D58(ctx, base);
	// 8324B9D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B9DC: 906A7964  stw r3, 0x7964(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31076 as u32), ctx.r[3].u32 ) };
	// 8324B9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B9F0 size=96
    let mut pc: u32 = 0x8324B9F0;
    'dispatch: loop {
        match pc {
            0x8324B9F0 => {
    //   block [0x8324B9F0..0x8324BA50)
	// 8324B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B9FC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8324BA00: 4AFD3859  bl 0x8221f258
	ctx.lr = 0x8324BA04;
	sub_8221F258(ctx, base);
	// 8324BA04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BA08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BA0C: 419A0008  beq cr6, 0x8324ba14
	if ctx.cr[6].eq {
	pc = 0x8324BA14; continue 'dispatch;
	}
	// 8324BA10: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324BA14: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BA18: 41820008  beq 0x8324ba20
	if ctx.cr[0].eq {
	pc = 0x8324BA20; continue 'dispatch;
	}
	// 8324BA1C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324BA20: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BA24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BA28: 39097968  addi r8, r9, 0x7968
	ctx.r[8].s64 = ctx.r[9].s64 + 31080;
	// 8324BA2C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BA30: 38678428  addi r3, r7, -0x7bd8
	ctx.r[3].s64 = ctx.r[7].s64 + -31704;
	// 8324BA34: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BA38: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BA3C: 4BA5E4E5  bl 0x82ca9f20
	ctx.lr = 0x8324BA40;
	sub_82CA9F20(ctx, base);
	// 8324BA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BA50 size=96
    let mut pc: u32 = 0x8324BA50;
    'dispatch: loop {
        match pc {
            0x8324BA50 => {
    //   block [0x8324BA50..0x8324BAB0)
	// 8324BA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BA58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BA5C: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8324BA60: 4AFD37F9  bl 0x8221f258
	ctx.lr = 0x8324BA64;
	sub_8221F258(ctx, base);
	// 8324BA64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BA68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BA6C: 419A0008  beq cr6, 0x8324ba74
	if ctx.cr[6].eq {
	pc = 0x8324BA74; continue 'dispatch;
	}
	// 8324BA70: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324BA74: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BA78: 41820008  beq 0x8324ba80
	if ctx.cr[0].eq {
	pc = 0x8324BA80; continue 'dispatch;
	}
	// 8324BA7C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324BA80: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BA88: 39097974  addi r8, r9, 0x7974
	ctx.r[8].s64 = ctx.r[9].s64 + 31092;
	// 8324BA8C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BA90: 38678438  addi r3, r7, -0x7bc8
	ctx.r[3].s64 = ctx.r[7].s64 + -31688;
	// 8324BA94: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BA98: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BA9C: 4BA5E485  bl 0x82ca9f20
	ctx.lr = 0x8324BAA0;
	sub_82CA9F20(ctx, base);
	// 8324BAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BAB0 size=64
    let mut pc: u32 = 0x8324BAB0;
    'dispatch: loop {
        match pc {
            0x8324BAB0 => {
    //   block [0x8324BAB0..0x8324BAF0)
	// 8324BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BAC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BAC4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324BAC8: 386A7980  addi r3, r10, 0x7980
	ctx.r[3].s64 = ctx.r[10].s64 + 31104;
	// 8324BACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BAD0: 4AFE1401  bl 0x8222ced0
	ctx.lr = 0x8324BAD4;
	sub_8222CED0(ctx, base);
	// 8324BAD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BAD8: 38698448  addi r3, r9, -0x7bb8
	ctx.r[3].s64 = ctx.r[9].s64 + -31672;
	// 8324BADC: 4BA5E445  bl 0x82ca9f20
	ctx.lr = 0x8324BAE0;
	sub_82CA9F20(ctx, base);
	// 8324BAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BAF0 size=64
    let mut pc: u32 = 0x8324BAF0;
    'dispatch: loop {
        match pc {
            0x8324BAF0 => {
    //   block [0x8324BAF0..0x8324BB30)
	// 8324BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB04: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BB08: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 8324BB0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB10: 4AFE13C1  bl 0x8222ced0
	ctx.lr = 0x8324BB14;
	sub_8222CED0(ctx, base);
	// 8324BB14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB18: 38698458  addi r3, r9, -0x7ba8
	ctx.r[3].s64 = ctx.r[9].s64 + -31656;
	// 8324BB1C: 4BA5E405  bl 0x82ca9f20
	ctx.lr = 0x8324BB20;
	sub_82CA9F20(ctx, base);
	// 8324BB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BB30 size=64
    let mut pc: u32 = 0x8324BB30;
    'dispatch: loop {
        match pc {
            0x8324BB30 => {
    //   block [0x8324BB30..0x8324BB70)
	// 8324BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BB3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB44: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BB48: 386A7988  addi r3, r10, 0x7988
	ctx.r[3].s64 = ctx.r[10].s64 + 31112;
	// 8324BB4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB50: 4AFE1381  bl 0x8222ced0
	ctx.lr = 0x8324BB54;
	sub_8222CED0(ctx, base);
	// 8324BB54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB58: 38698468  addi r3, r9, -0x7b98
	ctx.r[3].s64 = ctx.r[9].s64 + -31640;
	// 8324BB5C: 4BA5E3C5  bl 0x82ca9f20
	ctx.lr = 0x8324BB60;
	sub_82CA9F20(ctx, base);
	// 8324BB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BB70 size=64
    let mut pc: u32 = 0x8324BB70;
    'dispatch: loop {
        match pc {
            0x8324BB70 => {
    //   block [0x8324BB70..0x8324BBB0)
	// 8324BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BB78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BB7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB84: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324BB88: 386A798C  addi r3, r10, 0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + 31116;
	// 8324BB8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB90: 4AFE1341  bl 0x8222ced0
	ctx.lr = 0x8324BB94;
	sub_8222CED0(ctx, base);
	// 8324BB94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB98: 38698478  addi r3, r9, -0x7b88
	ctx.r[3].s64 = ctx.r[9].s64 + -31624;
	// 8324BB9C: 4BA5E385  bl 0x82ca9f20
	ctx.lr = 0x8324BBA0;
	sub_82CA9F20(ctx, base);
	// 8324BBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BBB0 size=64
    let mut pc: u32 = 0x8324BBB0;
    'dispatch: loop {
        match pc {
            0x8324BBB0 => {
    //   block [0x8324BBB0..0x8324BBF0)
	// 8324BBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BBBC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BBC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BBC4: 388B9FE4  addi r4, r11, -0x601c
	ctx.r[4].s64 = ctx.r[11].s64 + -24604;
	// 8324BBC8: 386A7990  addi r3, r10, 0x7990
	ctx.r[3].s64 = ctx.r[10].s64 + 31120;
	// 8324BBCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BBD0: 4AFE1301  bl 0x8222ced0
	ctx.lr = 0x8324BBD4;
	sub_8222CED0(ctx, base);
	// 8324BBD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BBD8: 38698488  addi r3, r9, -0x7b78
	ctx.r[3].s64 = ctx.r[9].s64 + -31608;
	// 8324BBDC: 4BA5E345  bl 0x82ca9f20
	ctx.lr = 0x8324BBE0;
	sub_82CA9F20(ctx, base);
	// 8324BBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BBF0 size=56
    let mut pc: u32 = 0x8324BBF0;
    'dispatch: loop {
        match pc {
            0x8324BBF0 => {
    //   block [0x8324BBF0..0x8324BC28)
	// 8324BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BBFC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC04: 386B9FF0  addi r3, r11, -0x6010
	ctx.r[3].s64 = ctx.r[11].s64 + -24592;
	// 8324BC08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC0C: 4AFA814D  bl 0x821f3d58
	ctx.lr = 0x8324BC10;
	sub_821F3D58(ctx, base);
	// 8324BC10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC14: 906A7994  stw r3, 0x7994(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31124 as u32), ctx.r[3].u32 ) };
	// 8324BC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC28 size=56
    let mut pc: u32 = 0x8324BC28;
    'dispatch: loop {
        match pc {
            0x8324BC28 => {
    //   block [0x8324BC28..0x8324BC60)
	// 8324BC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BC34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC3C: 386BA01C  addi r3, r11, -0x5fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -24548;
	// 8324BC40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC44: 4AFA8115  bl 0x821f3d58
	ctx.lr = 0x8324BC48;
	sub_821F3D58(ctx, base);
	// 8324BC48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC4C: 906A7998  stw r3, 0x7998(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31128 as u32), ctx.r[3].u32 ) };
	// 8324BC50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC60 size=56
    let mut pc: u32 = 0x8324BC60;
    'dispatch: loop {
        match pc {
            0x8324BC60 => {
    //   block [0x8324BC60..0x8324BC98)
	// 8324BC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BC68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BC6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC74: 386BA048  addi r3, r11, -0x5fb8
	ctx.r[3].s64 = ctx.r[11].s64 + -24504;
	// 8324BC78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC7C: 4AFA80DD  bl 0x821f3d58
	ctx.lr = 0x8324BC80;
	sub_821F3D58(ctx, base);
	// 8324BC80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC84: 906A799C  stw r3, 0x799c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31132 as u32), ctx.r[3].u32 ) };
	// 8324BC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC98 size=56
    let mut pc: u32 = 0x8324BC98;
    'dispatch: loop {
        match pc {
            0x8324BC98 => {
    //   block [0x8324BC98..0x8324BCD0)
	// 8324BC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BCA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BCA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BCA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BCAC: 386BA074  addi r3, r11, -0x5f8c
	ctx.r[3].s64 = ctx.r[11].s64 + -24460;
	// 8324BCB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BCB4: 4AFA80A5  bl 0x821f3d58
	ctx.lr = 0x8324BCB8;
	sub_821F3D58(ctx, base);
	// 8324BCB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BCBC: 906A79A0  stw r3, 0x79a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31136 as u32), ctx.r[3].u32 ) };
	// 8324BCC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BCC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BCC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BCD0 size=56
    let mut pc: u32 = 0x8324BCD0;
    'dispatch: loop {
        match pc {
            0x8324BCD0 => {
    //   block [0x8324BCD0..0x8324BD08)
	// 8324BCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BCD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BCDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BCE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BCE4: 386BA0A0  addi r3, r11, -0x5f60
	ctx.r[3].s64 = ctx.r[11].s64 + -24416;
	// 8324BCE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BCEC: 4AFA806D  bl 0x821f3d58
	ctx.lr = 0x8324BCF0;
	sub_821F3D58(ctx, base);
	// 8324BCF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BCF4: 906A79A4  stw r3, 0x79a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31140 as u32), ctx.r[3].u32 ) };
	// 8324BCF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BD08 size=12
    let mut pc: u32 = 0x8324BD08;
    'dispatch: loop {
        match pc {
            0x8324BD08 => {
    //   block [0x8324BD08..0x8324BD14)
	// 8324BD08: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BD0C: 386B8498  addi r3, r11, -0x7b68
	ctx.r[3].s64 = ctx.r[11].s64 + -31592;
	// 8324BD10: 4BA5E210  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD18 size=64
    let mut pc: u32 = 0x8324BD18;
    'dispatch: loop {
        match pc {
            0x8324BD18 => {
    //   block [0x8324BD18..0x8324BD58)
	// 8324BD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BD20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BD24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BD28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BD2C: 388BAA40  addi r4, r11, -0x55c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21952;
	// 8324BD30: 386A79B0  addi r3, r10, 0x79b0
	ctx.r[3].s64 = ctx.r[10].s64 + 31152;
	// 8324BD34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BD38: 4AFE1199  bl 0x8222ced0
	ctx.lr = 0x8324BD3C;
	sub_8222CED0(ctx, base);
	// 8324BD3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BD40: 38698510  addi r3, r9, -0x7af0
	ctx.r[3].s64 = ctx.r[9].s64 + -31472;
	// 8324BD44: 4BA5E1DD  bl 0x82ca9f20
	ctx.lr = 0x8324BD48;
	sub_82CA9F20(ctx, base);
	// 8324BD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD58 size=64
    let mut pc: u32 = 0x8324BD58;
    'dispatch: loop {
        match pc {
            0x8324BD58 => {
    //   block [0x8324BD58..0x8324BD98)
	// 8324BD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BD68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BD6C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BD70: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 8324BD74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BD78: 4AFE1159  bl 0x8222ced0
	ctx.lr = 0x8324BD7C;
	sub_8222CED0(ctx, base);
	// 8324BD7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BD80: 38698520  addi r3, r9, -0x7ae0
	ctx.r[3].s64 = ctx.r[9].s64 + -31456;
	// 8324BD84: 4BA5E19D  bl 0x82ca9f20
	ctx.lr = 0x8324BD88;
	sub_82CA9F20(ctx, base);
	// 8324BD88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD98 size=64
    let mut pc: u32 = 0x8324BD98;
    'dispatch: loop {
        match pc {
            0x8324BD98 => {
    //   block [0x8324BD98..0x8324BDD8)
	// 8324BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BDA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BDA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BDAC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BDB0: 386A79B8  addi r3, r10, 0x79b8
	ctx.r[3].s64 = ctx.r[10].s64 + 31160;
	// 8324BDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BDB8: 4AFE1119  bl 0x8222ced0
	ctx.lr = 0x8324BDBC;
	sub_8222CED0(ctx, base);
	// 8324BDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BDC0: 38698530  addi r3, r9, -0x7ad0
	ctx.r[3].s64 = ctx.r[9].s64 + -31440;
	// 8324BDC4: 4BA5E15D  bl 0x82ca9f20
	ctx.lr = 0x8324BDC8;
	sub_82CA9F20(ctx, base);
	// 8324BDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BDD8 size=64
    let mut pc: u32 = 0x8324BDD8;
    'dispatch: loop {
        match pc {
            0x8324BDD8 => {
    //   block [0x8324BDD8..0x8324BE18)
	// 8324BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BDE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BDE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BDEC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324BDF0: 386A79BC  addi r3, r10, 0x79bc
	ctx.r[3].s64 = ctx.r[10].s64 + 31164;
	// 8324BDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BDF8: 4AFE10D9  bl 0x8222ced0
	ctx.lr = 0x8324BDFC;
	sub_8222CED0(ctx, base);
	// 8324BDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BE00: 38698540  addi r3, r9, -0x7ac0
	ctx.r[3].s64 = ctx.r[9].s64 + -31424;
	// 8324BE04: 4BA5E11D  bl 0x82ca9f20
	ctx.lr = 0x8324BE08;
	sub_82CA9F20(ctx, base);
	// 8324BE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BE18 size=64
    let mut pc: u32 = 0x8324BE18;
    'dispatch: loop {
        match pc {
            0x8324BE18 => {
    //   block [0x8324BE18..0x8324BE58)
	// 8324BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BE24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BE28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BE2C: 388BABA4  addi r4, r11, -0x545c
	ctx.r[4].s64 = ctx.r[11].s64 + -21596;
	// 8324BE30: 386A79C0  addi r3, r10, 0x79c0
	ctx.r[3].s64 = ctx.r[10].s64 + 31168;
	// 8324BE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BE38: 4AFE1099  bl 0x8222ced0
	ctx.lr = 0x8324BE3C;
	sub_8222CED0(ctx, base);
	// 8324BE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BE40: 38698550  addi r3, r9, -0x7ab0
	ctx.r[3].s64 = ctx.r[9].s64 + -31408;
	// 8324BE44: 4BA5E0DD  bl 0x82ca9f20
	ctx.lr = 0x8324BE48;
	sub_82CA9F20(ctx, base);
	// 8324BE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BE58 size=96
    let mut pc: u32 = 0x8324BE58;
    'dispatch: loop {
        match pc {
            0x8324BE58 => {
    //   block [0x8324BE58..0x8324BEB8)
	// 8324BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BE64: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8324BE68: 4AFD33F1  bl 0x8221f258
	ctx.lr = 0x8324BE6C;
	sub_8221F258(ctx, base);
	// 8324BE6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BE70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BE74: 419A0008  beq cr6, 0x8324be7c
	if ctx.cr[6].eq {
	pc = 0x8324BE7C; continue 'dispatch;
	}
	// 8324BE78: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324BE7C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BE80: 41820008  beq 0x8324be88
	if ctx.cr[0].eq {
	pc = 0x8324BE88; continue 'dispatch;
	}
	// 8324BE84: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324BE88: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BE8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BE90: 390979C4  addi r8, r9, 0x79c4
	ctx.r[8].s64 = ctx.r[9].s64 + 31172;
	// 8324BE94: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BE98: 38678560  addi r3, r7, -0x7aa0
	ctx.r[3].s64 = ctx.r[7].s64 + -31392;
	// 8324BE9C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BEA0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BEA4: 4BA5E07D  bl 0x82ca9f20
	ctx.lr = 0x8324BEA8;
	sub_82CA9F20(ctx, base);
	// 8324BEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEB8 size=12
    let mut pc: u32 = 0x8324BEB8;
    'dispatch: loop {
        match pc {
            0x8324BEB8 => {
    //   block [0x8324BEB8..0x8324BEC4)
	// 8324BEB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BEBC: 386B85A8  addi r3, r11, -0x7a58
	ctx.r[3].s64 = ctx.r[11].s64 + -31320;
	// 8324BEC0: 4BA5E060  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEC8 size=48
    let mut pc: u32 = 0x8324BEC8;
    'dispatch: loop {
        match pc {
            0x8324BEC8 => {
    //   block [0x8324BEC8..0x8324BEF8)
	// 8324BEC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BECC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 8324BED0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BED4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BED8: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BEDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BEE0: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BEE4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BEE8: 4082FFE8  bne 0x8324bed0
	if !ctx.cr[0].eq {
	pc = 0x8324BED0; continue 'dispatch;
	}
	// 8324BEEC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BEF0: 38678600  addi r3, r7, -0x7a00
	ctx.r[3].s64 = ctx.r[7].s64 + -31232;
	// 8324BEF4: 4BA5E02C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEF8 size=48
    let mut pc: u32 = 0x8324BEF8;
    'dispatch: loop {
        match pc {
            0x8324BEF8 => {
    //   block [0x8324BEF8..0x8324BF28)
	// 8324BEF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BEFC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 8324BF00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF08: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF0C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF10: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF18: 4082FFE8  bne 0x8324bf00
	if !ctx.cr[0].eq {
	pc = 0x8324BF00; continue 'dispatch;
	}
	// 8324BF1C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF20: 38678610  addi r3, r7, -0x79f0
	ctx.r[3].s64 = ctx.r[7].s64 + -31216;
	// 8324BF24: 4BA5DFFC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF28 size=12
    let mut pc: u32 = 0x8324BF28;
    'dispatch: loop {
        match pc {
            0x8324BF28 => {
    //   block [0x8324BF28..0x8324BF34)
	// 8324BF28: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BF2C: 386B8620  addi r3, r11, -0x79e0
	ctx.r[3].s64 = ctx.r[11].s64 + -31200;
	// 8324BF30: 4BA5DFF0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF38 size=48
    let mut pc: u32 = 0x8324BF38;
    'dispatch: loop {
        match pc {
            0x8324BF38 => {
    //   block [0x8324BF38..0x8324BF68)
	// 8324BF38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BF3C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 8324BF40: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF48: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF50: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF54: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF58: 4082FFE8  bne 0x8324bf40
	if !ctx.cr[0].eq {
	pc = 0x8324BF40; continue 'dispatch;
	}
	// 8324BF5C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF60: 38678630  addi r3, r7, -0x79d0
	ctx.r[3].s64 = ctx.r[7].s64 + -31184;
	// 8324BF64: 4BA5DFBC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF68 size=48
    let mut pc: u32 = 0x8324BF68;
    'dispatch: loop {
        match pc {
            0x8324BF68 => {
    //   block [0x8324BF68..0x8324BF98)
	// 8324BF68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BF6C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 8324BF70: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF74: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF78: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF80: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF84: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF88: 4082FFE8  bne 0x8324bf70
	if !ctx.cr[0].eq {
	pc = 0x8324BF70; continue 'dispatch;
	}
	// 8324BF8C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF90: 38678640  addi r3, r7, -0x79c0
	ctx.r[3].s64 = ctx.r[7].s64 + -31168;
	// 8324BF94: 4BA5DF8C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BF98 size=64
    let mut pc: u32 = 0x8324BF98;
    'dispatch: loop {
        match pc {
            0x8324BF98 => {
    //   block [0x8324BF98..0x8324BFD8)
	// 8324BF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BFA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BFA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BFAC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BFB0: 386A7A10  addi r3, r10, 0x7a10
	ctx.r[3].s64 = ctx.r[10].s64 + 31248;
	// 8324BFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BFB8: 4AFE0F19  bl 0x8222ced0
	ctx.lr = 0x8324BFBC;
	sub_8222CED0(ctx, base);
	// 8324BFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BFC0: 38698650  addi r3, r9, -0x79b0
	ctx.r[3].s64 = ctx.r[9].s64 + -31152;
	// 8324BFC4: 4BA5DF5D  bl 0x82ca9f20
	ctx.lr = 0x8324BFC8;
	sub_82CA9F20(ctx, base);
	// 8324BFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BFD8 size=64
    let mut pc: u32 = 0x8324BFD8;
    'dispatch: loop {
        match pc {
            0x8324BFD8 => {
    //   block [0x8324BFD8..0x8324C018)
	// 8324BFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BFE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BFE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BFEC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BFF0: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 8324BFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BFF8: 4AFE0ED9  bl 0x8222ced0
	ctx.lr = 0x8324BFFC;
	sub_8222CED0(ctx, base);
	// 8324BFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C000: 38698660  addi r3, r9, -0x79a0
	ctx.r[3].s64 = ctx.r[9].s64 + -31136;
	// 8324C004: 4BA5DF1D  bl 0x82ca9f20
	ctx.lr = 0x8324C008;
	sub_82CA9F20(ctx, base);
	// 8324C008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C018 size=64
    let mut pc: u32 = 0x8324C018;
    'dispatch: loop {
        match pc {
            0x8324C018 => {
    //   block [0x8324C018..0x8324C058)
	// 8324C018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C024: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C028: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C02C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324C030: 386A7A18  addi r3, r10, 0x7a18
	ctx.r[3].s64 = ctx.r[10].s64 + 31256;
	// 8324C034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C038: 4AFE0E99  bl 0x8222ced0
	ctx.lr = 0x8324C03C;
	sub_8222CED0(ctx, base);
	// 8324C03C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C040: 38698670  addi r3, r9, -0x7990
	ctx.r[3].s64 = ctx.r[9].s64 + -31120;
	// 8324C044: 4BA5DEDD  bl 0x82ca9f20
	ctx.lr = 0x8324C048;
	sub_82CA9F20(ctx, base);
	// 8324C048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C058 size=64
    let mut pc: u32 = 0x8324C058;
    'dispatch: loop {
        match pc {
            0x8324C058 => {
    //   block [0x8324C058..0x8324C098)
	// 8324C058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C064: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C068: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C06C: 388BD960  addi r4, r11, -0x26a0
	ctx.r[4].s64 = ctx.r[11].s64 + -9888;
	// 8324C070: 386A7A1C  addi r3, r10, 0x7a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 31260;
	// 8324C074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C078: 4AFE0E59  bl 0x8222ced0
	ctx.lr = 0x8324C07C;
	sub_8222CED0(ctx, base);
	// 8324C07C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C080: 386986D8  addi r3, r9, -0x7928
	ctx.r[3].s64 = ctx.r[9].s64 + -31016;
	// 8324C084: 4BA5DE9D  bl 0x82ca9f20
	ctx.lr = 0x8324C088;
	sub_82CA9F20(ctx, base);
	// 8324C088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C098 size=64
    let mut pc: u32 = 0x8324C098;
    'dispatch: loop {
        match pc {
            0x8324C098 => {
    //   block [0x8324C098..0x8324C0D8)
	// 8324C098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C0A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C0A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C0AC: 388BD978  addi r4, r11, -0x2688
	ctx.r[4].s64 = ctx.r[11].s64 + -9864;
	// 8324C0B0: 386A7A20  addi r3, r10, 0x7a20
	ctx.r[3].s64 = ctx.r[10].s64 + 31264;
	// 8324C0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C0B8: 4AFE0E19  bl 0x8222ced0
	ctx.lr = 0x8324C0BC;
	sub_8222CED0(ctx, base);
	// 8324C0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C0C0: 386986E8  addi r3, r9, -0x7918
	ctx.r[3].s64 = ctx.r[9].s64 + -31000;
	// 8324C0C4: 4BA5DE5D  bl 0x82ca9f20
	ctx.lr = 0x8324C0C8;
	sub_82CA9F20(ctx, base);
	// 8324C0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C0D8 size=64
    let mut pc: u32 = 0x8324C0D8;
    'dispatch: loop {
        match pc {
            0x8324C0D8 => {
    //   block [0x8324C0D8..0x8324C118)
	// 8324C0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C0E4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C0E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C0EC: 388BD98C  addi r4, r11, -0x2674
	ctx.r[4].s64 = ctx.r[11].s64 + -9844;
	// 8324C0F0: 386A7A24  addi r3, r10, 0x7a24
	ctx.r[3].s64 = ctx.r[10].s64 + 31268;
	// 8324C0F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C0F8: 4AFE0DD9  bl 0x8222ced0
	ctx.lr = 0x8324C0FC;
	sub_8222CED0(ctx, base);
	// 8324C0FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C100: 386986F8  addi r3, r9, -0x7908
	ctx.r[3].s64 = ctx.r[9].s64 + -30984;
	// 8324C104: 4BA5DE1D  bl 0x82ca9f20
	ctx.lr = 0x8324C108;
	sub_82CA9F20(ctx, base);
	// 8324C108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C118 size=64
    let mut pc: u32 = 0x8324C118;
    'dispatch: loop {
        match pc {
            0x8324C118 => {
    //   block [0x8324C118..0x8324C158)
	// 8324C118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C128: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C12C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324C130: 386A7A28  addi r3, r10, 0x7a28
	ctx.r[3].s64 = ctx.r[10].s64 + 31272;
	// 8324C134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C138: 4AFE0D99  bl 0x8222ced0
	ctx.lr = 0x8324C13C;
	sub_8222CED0(ctx, base);
	// 8324C13C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C140: 38698708  addi r3, r9, -0x78f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30968;
	// 8324C144: 4BA5DDDD  bl 0x82ca9f20
	ctx.lr = 0x8324C148;
	sub_82CA9F20(ctx, base);
	// 8324C148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C158 size=56
    let mut pc: u32 = 0x8324C158;
    'dispatch: loop {
        match pc {
            0x8324C158 => {
    //   block [0x8324C158..0x8324C190)
	// 8324C158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C16C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8324C170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C174: 4AFA7BE5  bl 0x821f3d58
	ctx.lr = 0x8324C178;
	sub_821F3D58(ctx, base);
	// 8324C178: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C17C: 906A7A2C  stw r3, 0x7a2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31276 as u32), ctx.r[3].u32 ) };
	// 8324C180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C190 size=56
    let mut pc: u32 = 0x8324C190;
    'dispatch: loop {
        match pc {
            0x8324C190 => {
    //   block [0x8324C190..0x8324C1C8)
	// 8324C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C1A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8324C1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C1AC: 4AFA7BAD  bl 0x821f3d58
	ctx.lr = 0x8324C1B0;
	sub_821F3D58(ctx, base);
	// 8324C1B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C1B4: 906A7A30  stw r3, 0x7a30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31280 as u32), ctx.r[3].u32 ) };
	// 8324C1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C1C8 size=56
    let mut pc: u32 = 0x8324C1C8;
    'dispatch: loop {
        match pc {
            0x8324C1C8 => {
    //   block [0x8324C1C8..0x8324C200)
	// 8324C1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C1DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8324C1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C1E4: 4AFA7B75  bl 0x821f3d58
	ctx.lr = 0x8324C1E8;
	sub_821F3D58(ctx, base);
	// 8324C1E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C1EC: 906A7A34  stw r3, 0x7a34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31284 as u32), ctx.r[3].u32 ) };
	// 8324C1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C200 size=56
    let mut pc: u32 = 0x8324C200;
    'dispatch: loop {
        match pc {
            0x8324C200 => {
    //   block [0x8324C200..0x8324C238)
	// 8324C200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C214: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8324C218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C21C: 4AFA7B3D  bl 0x821f3d58
	ctx.lr = 0x8324C220;
	sub_821F3D58(ctx, base);
	// 8324C220: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C224: 906A7A38  stw r3, 0x7a38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31288 as u32), ctx.r[3].u32 ) };
	// 8324C228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C238 size=56
    let mut pc: u32 = 0x8324C238;
    'dispatch: loop {
        match pc {
            0x8324C238 => {
    //   block [0x8324C238..0x8324C270)
	// 8324C238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C24C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8324C250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C254: 4AFA7B05  bl 0x821f3d58
	ctx.lr = 0x8324C258;
	sub_821F3D58(ctx, base);
	// 8324C258: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C25C: 906A7A3C  stw r3, 0x7a3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31292 as u32), ctx.r[3].u32 ) };
	// 8324C260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C270 size=56
    let mut pc: u32 = 0x8324C270;
    'dispatch: loop {
        match pc {
            0x8324C270 => {
    //   block [0x8324C270..0x8324C2A8)
	// 8324C270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C284: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8324C288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C28C: 4AFA7ACD  bl 0x821f3d58
	ctx.lr = 0x8324C290;
	sub_821F3D58(ctx, base);
	// 8324C290: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C294: 906A7A40  stw r3, 0x7a40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31296 as u32), ctx.r[3].u32 ) };
	// 8324C298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C2A8 size=56
    let mut pc: u32 = 0x8324C2A8;
    'dispatch: loop {
        match pc {
            0x8324C2A8 => {
    //   block [0x8324C2A8..0x8324C2E0)
	// 8324C2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C2BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8324C2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C2C4: 4AFA7A95  bl 0x821f3d58
	ctx.lr = 0x8324C2C8;
	sub_821F3D58(ctx, base);
	// 8324C2C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C2CC: 906A7A44  stw r3, 0x7a44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31300 as u32), ctx.r[3].u32 ) };
	// 8324C2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C2E0 size=56
    let mut pc: u32 = 0x8324C2E0;
    'dispatch: loop {
        match pc {
            0x8324C2E0 => {
    //   block [0x8324C2E0..0x8324C318)
	// 8324C2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C2F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8324C2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C2FC: 4AFA7A5D  bl 0x821f3d58
	ctx.lr = 0x8324C300;
	sub_821F3D58(ctx, base);
	// 8324C300: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C304: 906A7A48  stw r3, 0x7a48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31304 as u32), ctx.r[3].u32 ) };
	// 8324C308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C318 size=56
    let mut pc: u32 = 0x8324C318;
    'dispatch: loop {
        match pc {
            0x8324C318 => {
    //   block [0x8324C318..0x8324C350)
	// 8324C318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C32C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8324C330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C334: 4AFA7A25  bl 0x821f3d58
	ctx.lr = 0x8324C338;
	sub_821F3D58(ctx, base);
	// 8324C338: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C33C: 906A7A4C  stw r3, 0x7a4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31308 as u32), ctx.r[3].u32 ) };
	// 8324C340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C350 size=56
    let mut pc: u32 = 0x8324C350;
    'dispatch: loop {
        match pc {
            0x8324C350 => {
    //   block [0x8324C350..0x8324C388)
	// 8324C350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C364: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8324C368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C36C: 4AFA79ED  bl 0x821f3d58
	ctx.lr = 0x8324C370;
	sub_821F3D58(ctx, base);
	// 8324C370: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C374: 906A7A50  stw r3, 0x7a50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31312 as u32), ctx.r[3].u32 ) };
	// 8324C378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C388 size=56
    let mut pc: u32 = 0x8324C388;
    'dispatch: loop {
        match pc {
            0x8324C388 => {
    //   block [0x8324C388..0x8324C3C0)
	// 8324C388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C39C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8324C3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C3A4: 4AFA79B5  bl 0x821f3d58
	ctx.lr = 0x8324C3A8;
	sub_821F3D58(ctx, base);
	// 8324C3A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C3AC: 906A7A54  stw r3, 0x7a54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31316 as u32), ctx.r[3].u32 ) };
	// 8324C3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C3C0 size=56
    let mut pc: u32 = 0x8324C3C0;
    'dispatch: loop {
        match pc {
            0x8324C3C0 => {
    //   block [0x8324C3C0..0x8324C3F8)
	// 8324C3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C3D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8324C3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C3DC: 4AFA797D  bl 0x821f3d58
	ctx.lr = 0x8324C3E0;
	sub_821F3D58(ctx, base);
	// 8324C3E0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C3E4: 906A7A58  stw r3, 0x7a58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31320 as u32), ctx.r[3].u32 ) };
	// 8324C3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C3F8 size=56
    let mut pc: u32 = 0x8324C3F8;
    'dispatch: loop {
        match pc {
            0x8324C3F8 => {
    //   block [0x8324C3F8..0x8324C430)
	// 8324C3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C40C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8324C410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C414: 4AFA7945  bl 0x821f3d58
	ctx.lr = 0x8324C418;
	sub_821F3D58(ctx, base);
	// 8324C418: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C41C: 906A7A5C  stw r3, 0x7a5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31324 as u32), ctx.r[3].u32 ) };
	// 8324C420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C430 size=56
    let mut pc: u32 = 0x8324C430;
    'dispatch: loop {
        match pc {
            0x8324C430 => {
    //   block [0x8324C430..0x8324C468)
	// 8324C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C444: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8324C448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C44C: 4AFA790D  bl 0x821f3d58
	ctx.lr = 0x8324C450;
	sub_821F3D58(ctx, base);
	// 8324C450: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C454: 906A7A60  stw r3, 0x7a60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31328 as u32), ctx.r[3].u32 ) };
	// 8324C458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C468 size=56
    let mut pc: u32 = 0x8324C468;
    'dispatch: loop {
        match pc {
            0x8324C468 => {
    //   block [0x8324C468..0x8324C4A0)
	// 8324C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C47C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8324C480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C484: 4AFA78D5  bl 0x821f3d58
	ctx.lr = 0x8324C488;
	sub_821F3D58(ctx, base);
	// 8324C488: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C48C: 906A7A64  stw r3, 0x7a64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31332 as u32), ctx.r[3].u32 ) };
	// 8324C490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C4A0 size=56
    let mut pc: u32 = 0x8324C4A0;
    'dispatch: loop {
        match pc {
            0x8324C4A0 => {
    //   block [0x8324C4A0..0x8324C4D8)
	// 8324C4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C4B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8324C4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C4BC: 4AFA789D  bl 0x821f3d58
	ctx.lr = 0x8324C4C0;
	sub_821F3D58(ctx, base);
	// 8324C4C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C4C4: 906A7A68  stw r3, 0x7a68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31336 as u32), ctx.r[3].u32 ) };
	// 8324C4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C4D8 size=56
    let mut pc: u32 = 0x8324C4D8;
    'dispatch: loop {
        match pc {
            0x8324C4D8 => {
    //   block [0x8324C4D8..0x8324C510)
	// 8324C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C4EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8324C4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C4F4: 4AFA7865  bl 0x821f3d58
	ctx.lr = 0x8324C4F8;
	sub_821F3D58(ctx, base);
	// 8324C4F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C4FC: 906A7A6C  stw r3, 0x7a6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31340 as u32), ctx.r[3].u32 ) };
	// 8324C500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C510 size=56
    let mut pc: u32 = 0x8324C510;
    'dispatch: loop {
        match pc {
            0x8324C510 => {
    //   block [0x8324C510..0x8324C548)
	// 8324C510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C524: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8324C528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C52C: 4AFA782D  bl 0x821f3d58
	ctx.lr = 0x8324C530;
	sub_821F3D58(ctx, base);
	// 8324C530: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C534: 906A7A70  stw r3, 0x7a70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31344 as u32), ctx.r[3].u32 ) };
	// 8324C538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C548 size=56
    let mut pc: u32 = 0x8324C548;
    'dispatch: loop {
        match pc {
            0x8324C548 => {
    //   block [0x8324C548..0x8324C580)
	// 8324C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C554: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C55C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8324C560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C564: 4AFA77F5  bl 0x821f3d58
	ctx.lr = 0x8324C568;
	sub_821F3D58(ctx, base);
	// 8324C568: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C56C: 906A7A74  stw r3, 0x7a74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31348 as u32), ctx.r[3].u32 ) };
	// 8324C570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C580 size=56
    let mut pc: u32 = 0x8324C580;
    'dispatch: loop {
        match pc {
            0x8324C580 => {
    //   block [0x8324C580..0x8324C5B8)
	// 8324C580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C58C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C594: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8324C598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C59C: 4AFA77BD  bl 0x821f3d58
	ctx.lr = 0x8324C5A0;
	sub_821F3D58(ctx, base);
	// 8324C5A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C5A4: 906A7A78  stw r3, 0x7a78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31352 as u32), ctx.r[3].u32 ) };
	// 8324C5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C5B8 size=56
    let mut pc: u32 = 0x8324C5B8;
    'dispatch: loop {
        match pc {
            0x8324C5B8 => {
    //   block [0x8324C5B8..0x8324C5F0)
	// 8324C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C5C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C5C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C5CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8324C5D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C5D4: 4AFA7785  bl 0x821f3d58
	ctx.lr = 0x8324C5D8;
	sub_821F3D58(ctx, base);
	// 8324C5D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C5DC: 906A7A7C  stw r3, 0x7a7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31356 as u32), ctx.r[3].u32 ) };
	// 8324C5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C5F0 size=376
    let mut pc: u32 = 0x8324C5F0;
    'dispatch: loop {
        match pc {
            0x8324C5F0 => {
    //   block [0x8324C5F0..0x8324C768)
	// 8324C5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C5F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324C5FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C600: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324C604: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C608: 3BEB7A80  addi r31, r11, 0x7a80
	ctx.r[31].s64 = ctx.r[11].s64 + 31360;
	// 8324C60C: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 8324C610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324C614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C618: 4AFE08B9  bl 0x8222ced0
	ctx.lr = 0x8324C61C;
	sub_8222CED0(ctx, base);
	// 8324C61C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C620: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C624: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 8324C628: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324C62C: 4AFE08A5  bl 0x8222ced0
	ctx.lr = 0x8324C630;
	sub_8222CED0(ctx, base);
	// 8324C630: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C638: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 8324C63C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8324C640: 4AFE0891  bl 0x8222ced0
	ctx.lr = 0x8324C644;
	sub_8222CED0(ctx, base);
	// 8324C644: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C648: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C64C: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 8324C650: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8324C654: 4AFE087D  bl 0x8222ced0
	ctx.lr = 0x8324C658;
	sub_8222CED0(ctx, base);
	// 8324C658: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C65C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C660: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8324C664: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8324C668: 4AFE0869  bl 0x8222ced0
	ctx.lr = 0x8324C66C;
	sub_8222CED0(ctx, base);
	// 8324C66C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C670: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C674: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 8324C678: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324C67C: 4AFE0855  bl 0x8222ced0
	ctx.lr = 0x8324C680;
	sub_8222CED0(ctx, base);
	// 8324C680: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C684: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C688: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 8324C68C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8324C690: 4AFE0841  bl 0x8222ced0
	ctx.lr = 0x8324C694;
	sub_8222CED0(ctx, base);
	// 8324C694: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C698: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C69C: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 8324C6A0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8324C6A4: 4AFE082D  bl 0x8222ced0
	ctx.lr = 0x8324C6A8;
	sub_8222CED0(ctx, base);
	// 8324C6A8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C6AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6B0: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8324C6B4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8324C6B8: 4AFE0819  bl 0x8222ced0
	ctx.lr = 0x8324C6BC;
	sub_8222CED0(ctx, base);
	// 8324C6BC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C6C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6C4: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 8324C6C8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8324C6CC: 4AFE0805  bl 0x8222ced0
	ctx.lr = 0x8324C6D0;
	sub_8222CED0(ctx, base);
	// 8324C6D0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C6D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6D8: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 8324C6DC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8324C6E0: 4AFE07F1  bl 0x8222ced0
	ctx.lr = 0x8324C6E4;
	sub_8222CED0(ctx, base);
	// 8324C6E4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C6E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6EC: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 8324C6F0: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324C6F4: 4AFE07DD  bl 0x8222ced0
	ctx.lr = 0x8324C6F8;
	sub_8222CED0(ctx, base);
	// 8324C6F8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C6FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C700: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 8324C704: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8324C708: 4AFE07C9  bl 0x8222ced0
	ctx.lr = 0x8324C70C;
	sub_8222CED0(ctx, base);
	// 8324C70C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C710: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C714: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 8324C718: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8324C71C: 4AFE07B5  bl 0x8222ced0
	ctx.lr = 0x8324C720;
	sub_8222CED0(ctx, base);
	// 8324C720: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C728: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 8324C72C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8324C730: 4AFE07A1  bl 0x8222ced0
	ctx.lr = 0x8324C734;
	sub_8222CED0(ctx, base);
	// 8324C734: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C738: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C73C: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 8324C740: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8324C744: 4AFE078D  bl 0x8222ced0
	ctx.lr = 0x8324C748;
	sub_8222CED0(ctx, base);
	// 8324C748: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8324C74C: 386A8718  addi r3, r10, -0x78e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30952;
	// 8324C750: 4BA5D7D1  bl 0x82ca9f20
	ctx.lr = 0x8324C754;
	sub_82CA9F20(ctx, base);
	// 8324C754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324C764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C768 size=376
    let mut pc: u32 = 0x8324C768;
    'dispatch: loop {
        match pc {
            0x8324C768 => {
    //   block [0x8324C768..0x8324C8E0)
	// 8324C768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324C774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C778: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324C77C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C780: 3BEB7AC0  addi r31, r11, 0x7ac0
	ctx.r[31].s64 = ctx.r[11].s64 + 31424;
	// 8324C784: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 8324C788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324C78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C790: 4AFE0741  bl 0x8222ced0
	ctx.lr = 0x8324C794;
	sub_8222CED0(ctx, base);
	// 8324C794: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C798: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C79C: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 8324C7A0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324C7A4: 4AFE072D  bl 0x8222ced0
	ctx.lr = 0x8324C7A8;
	sub_8222CED0(ctx, base);
	// 8324C7A8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C7AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7B0: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8324C7B4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8324C7B8: 4AFE0719  bl 0x8222ced0
	ctx.lr = 0x8324C7BC;
	sub_8222CED0(ctx, base);
	// 8324C7BC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C7C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7C4: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 8324C7C8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8324C7CC: 4AFE0705  bl 0x8222ced0
	ctx.lr = 0x8324C7D0;
	sub_8222CED0(ctx, base);
	// 8324C7D0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C7D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7D8: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 8324C7DC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8324C7E0: 4AFE06F1  bl 0x8222ced0
	ctx.lr = 0x8324C7E4;
	sub_8222CED0(ctx, base);
	// 8324C7E4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C7E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7EC: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 8324C7F0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324C7F4: 4AFE06DD  bl 0x8222ced0
	ctx.lr = 0x8324C7F8;
	sub_8222CED0(ctx, base);
	// 8324C7F8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C7FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C800: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 8324C804: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8324C808: 4AFE06C9  bl 0x8222ced0
	ctx.lr = 0x8324C80C;
	sub_8222CED0(ctx, base);
	// 8324C80C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C810: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C814: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 8324C818: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8324C81C: 4AFE06B5  bl 0x8222ced0
	ctx.lr = 0x8324C820;
	sub_8222CED0(ctx, base);
	// 8324C820: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C828: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 8324C82C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8324C830: 4AFE06A1  bl 0x8222ced0
	ctx.lr = 0x8324C834;
	sub_8222CED0(ctx, base);
	// 8324C834: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C838: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C83C: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 8324C840: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8324C844: 4AFE068D  bl 0x8222ced0
	ctx.lr = 0x8324C848;
	sub_8222CED0(ctx, base);
	// 8324C848: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C850: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 8324C854: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8324C858: 4AFE0679  bl 0x8222ced0
	ctx.lr = 0x8324C85C;
	sub_8222CED0(ctx, base);
	// 8324C85C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C860: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C864: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 8324C868: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324C86C: 4AFE0665  bl 0x8222ced0
	ctx.lr = 0x8324C870;
	sub_8222CED0(ctx, base);
	// 8324C870: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C874: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C878: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 8324C87C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8324C880: 4AFE0651  bl 0x8222ced0
	ctx.lr = 0x8324C884;
	sub_8222CED0(ctx, base);
	// 8324C884: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C888: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C88C: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 8324C890: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8324C894: 4AFE063D  bl 0x8222ced0
	ctx.lr = 0x8324C898;
	sub_8222CED0(ctx, base);
	// 8324C898: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C8A0: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 8324C8A4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8324C8A8: 4AFE0629  bl 0x8222ced0
	ctx.lr = 0x8324C8AC;
	sub_8222CED0(ctx, base);
	// 8324C8AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C8B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C8B4: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 8324C8B8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8324C8BC: 4AFE0615  bl 0x8222ced0
	ctx.lr = 0x8324C8C0;
	sub_8222CED0(ctx, base);
	// 8324C8C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8324C8C4: 386A8780  addi r3, r10, -0x7880
	ctx.r[3].s64 = ctx.r[10].s64 + -30848;
	// 8324C8C8: 4BA5D659  bl 0x82ca9f20
	ctx.lr = 0x8324C8CC;
	sub_82CA9F20(ctx, base);
	// 8324C8CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C8D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324C8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C8E0 size=64
    let mut pc: u32 = 0x8324C8E0;
    'dispatch: loop {
        match pc {
            0x8324C8E0 => {
    //   block [0x8324C8E0..0x8324C920)
	// 8324C8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C8EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C8F0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C8F4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324C8F8: 386A7B00  addi r3, r10, 0x7b00
	ctx.r[3].s64 = ctx.r[10].s64 + 31488;
	// 8324C8FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C900: 4AFE05D1  bl 0x8222ced0
	ctx.lr = 0x8324C904;
	sub_8222CED0(ctx, base);
	// 8324C904: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C908: 386987E8  addi r3, r9, -0x7818
	ctx.r[3].s64 = ctx.r[9].s64 + -30744;
	// 8324C90C: 4BA5D615  bl 0x82ca9f20
	ctx.lr = 0x8324C910;
	sub_82CA9F20(ctx, base);
	// 8324C910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C920 size=64
    let mut pc: u32 = 0x8324C920;
    'dispatch: loop {
        match pc {
            0x8324C920 => {
    //   block [0x8324C920..0x8324C960)
	// 8324C920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C92C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C930: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C934: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324C938: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 8324C93C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C940: 4AFE0591  bl 0x8222ced0
	ctx.lr = 0x8324C944;
	sub_8222CED0(ctx, base);
	// 8324C944: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C948: 386987F8  addi r3, r9, -0x7808
	ctx.r[3].s64 = ctx.r[9].s64 + -30728;
	// 8324C94C: 4BA5D5D5  bl 0x82ca9f20
	ctx.lr = 0x8324C950;
	sub_82CA9F20(ctx, base);
	// 8324C950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C960 size=64
    let mut pc: u32 = 0x8324C960;
    'dispatch: loop {
        match pc {
            0x8324C960 => {
    //   block [0x8324C960..0x8324C9A0)
	// 8324C960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C96C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C970: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C974: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324C978: 386A7B08  addi r3, r10, 0x7b08
	ctx.r[3].s64 = ctx.r[10].s64 + 31496;
	// 8324C97C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C980: 4AFE0551  bl 0x8222ced0
	ctx.lr = 0x8324C984;
	sub_8222CED0(ctx, base);
	// 8324C984: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C988: 38698808  addi r3, r9, -0x77f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30712;
	// 8324C98C: 4BA5D595  bl 0x82ca9f20
	ctx.lr = 0x8324C990;
	sub_82CA9F20(ctx, base);
	// 8324C990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C9A0 size=56
    let mut pc: u32 = 0x8324C9A0;
    'dispatch: loop {
        match pc {
            0x8324C9A0 => {
    //   block [0x8324C9A0..0x8324C9D8)
	// 8324C9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C9A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C9AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C9B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C9B4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8324C9B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C9BC: 4AFA739D  bl 0x821f3d58
	ctx.lr = 0x8324C9C0;
	sub_821F3D58(ctx, base);
	// 8324C9C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C9C4: 906A7B0C  stw r3, 0x7b0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31500 as u32), ctx.r[3].u32 ) };
	// 8324C9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C9D8 size=56
    let mut pc: u32 = 0x8324C9D8;
    'dispatch: loop {
        match pc {
            0x8324C9D8 => {
    //   block [0x8324C9D8..0x8324CA10)
	// 8324C9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C9E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C9E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C9E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C9EC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8324C9F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C9F4: 4AFA7365  bl 0x821f3d58
	ctx.lr = 0x8324C9F8;
	sub_821F3D58(ctx, base);
	// 8324C9F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C9FC: 906A7B10  stw r3, 0x7b10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31504 as u32), ctx.r[3].u32 ) };
	// 8324CA00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CA10 size=56
    let mut pc: u32 = 0x8324CA10;
    'dispatch: loop {
        match pc {
            0x8324CA10 => {
    //   block [0x8324CA10..0x8324CA48)
	// 8324CA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CA18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CA1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CA20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CA24: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8324CA28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CA2C: 4AFA732D  bl 0x821f3d58
	ctx.lr = 0x8324CA30;
	sub_821F3D58(ctx, base);
	// 8324CA30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CA34: 906A7B14  stw r3, 0x7b14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31508 as u32), ctx.r[3].u32 ) };
	// 8324CA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CA48 size=56
    let mut pc: u32 = 0x8324CA48;
    'dispatch: loop {
        match pc {
            0x8324CA48 => {
    //   block [0x8324CA48..0x8324CA80)
	// 8324CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CA54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CA58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CA5C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8324CA60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CA64: 4AFA72F5  bl 0x821f3d58
	ctx.lr = 0x8324CA68;
	sub_821F3D58(ctx, base);
	// 8324CA68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CA6C: 906A7B18  stw r3, 0x7b18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31512 as u32), ctx.r[3].u32 ) };
	// 8324CA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CA80 size=56
    let mut pc: u32 = 0x8324CA80;
    'dispatch: loop {
        match pc {
            0x8324CA80 => {
    //   block [0x8324CA80..0x8324CAB8)
	// 8324CA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CA90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CA94: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8324CA98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CA9C: 4AFA72BD  bl 0x821f3d58
	ctx.lr = 0x8324CAA0;
	sub_821F3D58(ctx, base);
	// 8324CAA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CAA4: 906A7B1C  stw r3, 0x7b1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31516 as u32), ctx.r[3].u32 ) };
	// 8324CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CAB8 size=56
    let mut pc: u32 = 0x8324CAB8;
    'dispatch: loop {
        match pc {
            0x8324CAB8 => {
    //   block [0x8324CAB8..0x8324CAF0)
	// 8324CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CACC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8324CAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CAD4: 4AFA7285  bl 0x821f3d58
	ctx.lr = 0x8324CAD8;
	sub_821F3D58(ctx, base);
	// 8324CAD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CADC: 906A7B20  stw r3, 0x7b20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31520 as u32), ctx.r[3].u32 ) };
	// 8324CAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CAF0 size=56
    let mut pc: u32 = 0x8324CAF0;
    'dispatch: loop {
        match pc {
            0x8324CAF0 => {
    //   block [0x8324CAF0..0x8324CB28)
	// 8324CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CB04: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8324CB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CB0C: 4AFA724D  bl 0x821f3d58
	ctx.lr = 0x8324CB10;
	sub_821F3D58(ctx, base);
	// 8324CB10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CB14: 906A7B24  stw r3, 0x7b24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31524 as u32), ctx.r[3].u32 ) };
	// 8324CB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CB28 size=56
    let mut pc: u32 = 0x8324CB28;
    'dispatch: loop {
        match pc {
            0x8324CB28 => {
    //   block [0x8324CB28..0x8324CB60)
	// 8324CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CB3C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8324CB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CB44: 4AFA7215  bl 0x821f3d58
	ctx.lr = 0x8324CB48;
	sub_821F3D58(ctx, base);
	// 8324CB48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CB4C: 906A7B28  stw r3, 0x7b28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31528 as u32), ctx.r[3].u32 ) };
	// 8324CB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CB60 size=56
    let mut pc: u32 = 0x8324CB60;
    'dispatch: loop {
        match pc {
            0x8324CB60 => {
    //   block [0x8324CB60..0x8324CB98)
	// 8324CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CB74: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8324CB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CB7C: 4AFA71DD  bl 0x821f3d58
	ctx.lr = 0x8324CB80;
	sub_821F3D58(ctx, base);
	// 8324CB80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CB84: 906A7B2C  stw r3, 0x7b2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31532 as u32), ctx.r[3].u32 ) };
	// 8324CB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CB98 size=56
    let mut pc: u32 = 0x8324CB98;
    'dispatch: loop {
        match pc {
            0x8324CB98 => {
    //   block [0x8324CB98..0x8324CBD0)
	// 8324CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CBA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CBA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CBAC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8324CBB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CBB4: 4AFA71A5  bl 0x821f3d58
	ctx.lr = 0x8324CBB8;
	sub_821F3D58(ctx, base);
	// 8324CBB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CBBC: 906A7B30  stw r3, 0x7b30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31536 as u32), ctx.r[3].u32 ) };
	// 8324CBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CBD0 size=56
    let mut pc: u32 = 0x8324CBD0;
    'dispatch: loop {
        match pc {
            0x8324CBD0 => {
    //   block [0x8324CBD0..0x8324CC08)
	// 8324CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CBD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CBDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CBE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CBE4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8324CBE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CBEC: 4AFA716D  bl 0x821f3d58
	ctx.lr = 0x8324CBF0;
	sub_821F3D58(ctx, base);
	// 8324CBF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CBF4: 906A7B34  stw r3, 0x7b34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31540 as u32), ctx.r[3].u32 ) };
	// 8324CBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CC08 size=56
    let mut pc: u32 = 0x8324CC08;
    'dispatch: loop {
        match pc {
            0x8324CC08 => {
    //   block [0x8324CC08..0x8324CC40)
	// 8324CC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CC14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CC18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CC1C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8324CC20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CC24: 4AFA7135  bl 0x821f3d58
	ctx.lr = 0x8324CC28;
	sub_821F3D58(ctx, base);
	// 8324CC28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CC2C: 906A7B38  stw r3, 0x7b38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31544 as u32), ctx.r[3].u32 ) };
	// 8324CC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CC40 size=56
    let mut pc: u32 = 0x8324CC40;
    'dispatch: loop {
        match pc {
            0x8324CC40 => {
    //   block [0x8324CC40..0x8324CC78)
	// 8324CC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CC50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CC54: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8324CC58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CC5C: 4AFA70FD  bl 0x821f3d58
	ctx.lr = 0x8324CC60;
	sub_821F3D58(ctx, base);
	// 8324CC60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CC64: 906A7B3C  stw r3, 0x7b3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31548 as u32), ctx.r[3].u32 ) };
	// 8324CC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CC78 size=56
    let mut pc: u32 = 0x8324CC78;
    'dispatch: loop {
        match pc {
            0x8324CC78 => {
    //   block [0x8324CC78..0x8324CCB0)
	// 8324CC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CC88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CC8C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8324CC90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CC94: 4AFA70C5  bl 0x821f3d58
	ctx.lr = 0x8324CC98;
	sub_821F3D58(ctx, base);
	// 8324CC98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CC9C: 906A7B40  stw r3, 0x7b40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31552 as u32), ctx.r[3].u32 ) };
	// 8324CCA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CCB0 size=56
    let mut pc: u32 = 0x8324CCB0;
    'dispatch: loop {
        match pc {
            0x8324CCB0 => {
    //   block [0x8324CCB0..0x8324CCE8)
	// 8324CCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CCBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CCC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CCC4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8324CCC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CCCC: 4AFA708D  bl 0x821f3d58
	ctx.lr = 0x8324CCD0;
	sub_821F3D58(ctx, base);
	// 8324CCD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CCD4: 906A7B44  stw r3, 0x7b44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31556 as u32), ctx.r[3].u32 ) };
	// 8324CCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CCE8 size=56
    let mut pc: u32 = 0x8324CCE8;
    'dispatch: loop {
        match pc {
            0x8324CCE8 => {
    //   block [0x8324CCE8..0x8324CD20)
	// 8324CCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CCF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CCF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CCFC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8324CD00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CD04: 4AFA7055  bl 0x821f3d58
	ctx.lr = 0x8324CD08;
	sub_821F3D58(ctx, base);
	// 8324CD08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CD0C: 906A7B48  stw r3, 0x7b48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31560 as u32), ctx.r[3].u32 ) };
	// 8324CD10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CD20 size=56
    let mut pc: u32 = 0x8324CD20;
    'dispatch: loop {
        match pc {
            0x8324CD20 => {
    //   block [0x8324CD20..0x8324CD58)
	// 8324CD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CD2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CD30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CD34: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8324CD38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CD3C: 4AFA701D  bl 0x821f3d58
	ctx.lr = 0x8324CD40;
	sub_821F3D58(ctx, base);
	// 8324CD40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CD44: 906A7B4C  stw r3, 0x7b4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31564 as u32), ctx.r[3].u32 ) };
	// 8324CD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CD58 size=56
    let mut pc: u32 = 0x8324CD58;
    'dispatch: loop {
        match pc {
            0x8324CD58 => {
    //   block [0x8324CD58..0x8324CD90)
	// 8324CD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CD68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CD6C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8324CD70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CD74: 4AFA6FE5  bl 0x821f3d58
	ctx.lr = 0x8324CD78;
	sub_821F3D58(ctx, base);
	// 8324CD78: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CD7C: 906A7B50  stw r3, 0x7b50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31568 as u32), ctx.r[3].u32 ) };
	// 8324CD80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CD90 size=56
    let mut pc: u32 = 0x8324CD90;
    'dispatch: loop {
        match pc {
            0x8324CD90 => {
    //   block [0x8324CD90..0x8324CDC8)
	// 8324CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CD9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CDA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CDA4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8324CDA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CDAC: 4AFA6FAD  bl 0x821f3d58
	ctx.lr = 0x8324CDB0;
	sub_821F3D58(ctx, base);
	// 8324CDB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CDB4: 906A7B54  stw r3, 0x7b54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31572 as u32), ctx.r[3].u32 ) };
	// 8324CDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CDC8 size=56
    let mut pc: u32 = 0x8324CDC8;
    'dispatch: loop {
        match pc {
            0x8324CDC8 => {
    //   block [0x8324CDC8..0x8324CE00)
	// 8324CDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CDD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CDD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CDDC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8324CDE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CDE4: 4AFA6F75  bl 0x821f3d58
	ctx.lr = 0x8324CDE8;
	sub_821F3D58(ctx, base);
	// 8324CDE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CDEC: 906A7B58  stw r3, 0x7b58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31576 as u32), ctx.r[3].u32 ) };
	// 8324CDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CE00 size=56
    let mut pc: u32 = 0x8324CE00;
    'dispatch: loop {
        match pc {
            0x8324CE00 => {
    //   block [0x8324CE00..0x8324CE38)
	// 8324CE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CE0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CE14: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8324CE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CE1C: 4AFA6F3D  bl 0x821f3d58
	ctx.lr = 0x8324CE20;
	sub_821F3D58(ctx, base);
	// 8324CE20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CE24: 906A7B5C  stw r3, 0x7b5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31580 as u32), ctx.r[3].u32 ) };
	// 8324CE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CE38 size=64
    let mut pc: u32 = 0x8324CE38;
    'dispatch: loop {
        match pc {
            0x8324CE38 => {
    //   block [0x8324CE38..0x8324CE78)
	// 8324CE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CE44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CE48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CE4C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324CE50: 386A7B60  addi r3, r10, 0x7b60
	ctx.r[3].s64 = ctx.r[10].s64 + 31584;
	// 8324CE54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324CE58: 4AFE0079  bl 0x8222ced0
	ctx.lr = 0x8324CE5C;
	sub_8222CED0(ctx, base);
	// 8324CE5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324CE60: 38698818  addi r3, r9, -0x77e8
	ctx.r[3].s64 = ctx.r[9].s64 + -30696;
	// 8324CE64: 4BA5D0BD  bl 0x82ca9f20
	ctx.lr = 0x8324CE68;
	sub_82CA9F20(ctx, base);
	// 8324CE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CE78 size=64
    let mut pc: u32 = 0x8324CE78;
    'dispatch: loop {
        match pc {
            0x8324CE78 => {
    //   block [0x8324CE78..0x8324CEB8)
	// 8324CE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CE84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CE88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CE8C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324CE90: 386A7B64  addi r3, r10, 0x7b64
	ctx.r[3].s64 = ctx.r[10].s64 + 31588;
	// 8324CE94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324CE98: 4AFE0039  bl 0x8222ced0
	ctx.lr = 0x8324CE9C;
	sub_8222CED0(ctx, base);
	// 8324CE9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324CEA0: 38698828  addi r3, r9, -0x77d8
	ctx.r[3].s64 = ctx.r[9].s64 + -30680;
	// 8324CEA4: 4BA5D07D  bl 0x82ca9f20
	ctx.lr = 0x8324CEA8;
	sub_82CA9F20(ctx, base);
	// 8324CEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CEB8 size=64
    let mut pc: u32 = 0x8324CEB8;
    'dispatch: loop {
        match pc {
            0x8324CEB8 => {
    //   block [0x8324CEB8..0x8324CEF8)
	// 8324CEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CEC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CEC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CEC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CECC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324CED0: 386A7B68  addi r3, r10, 0x7b68
	ctx.r[3].s64 = ctx.r[10].s64 + 31592;
	// 8324CED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324CED8: 4AFDFFF9  bl 0x8222ced0
	ctx.lr = 0x8324CEDC;
	sub_8222CED0(ctx, base);
	// 8324CEDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324CEE0: 38698838  addi r3, r9, -0x77c8
	ctx.r[3].s64 = ctx.r[9].s64 + -30664;
	// 8324CEE4: 4BA5D03D  bl 0x82ca9f20
	ctx.lr = 0x8324CEE8;
	sub_82CA9F20(ctx, base);
	// 8324CEE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324CEF8 size=32
    let mut pc: u32 = 0x8324CEF8;
    'dispatch: loop {
        match pc {
            0x8324CEF8 => {
    //   block [0x8324CEF8..0x8324CF18)
	// 8324CEF8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8324CEFC: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CF00: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8324CF04: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8324CF08: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8324CF0C: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8324CF10: 916A7B6C  stw r11, 0x7b6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31596 as u32), ctx.r[11].u32 ) };
	// 8324CF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CF18 size=56
    let mut pc: u32 = 0x8324CF18;
    'dispatch: loop {
        match pc {
            0x8324CF18 => {
    //   block [0x8324CF18..0x8324CF50)
	// 8324CF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CF24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CF28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CF2C: 386BE0F0  addi r3, r11, -0x1f10
	ctx.r[3].s64 = ctx.r[11].s64 + -7952;
	// 8324CF30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CF34: 4AFA6E25  bl 0x821f3d58
	ctx.lr = 0x8324CF38;
	sub_821F3D58(ctx, base);
	// 8324CF38: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CF3C: 906A7B70  stw r3, 0x7b70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31600 as u32), ctx.r[3].u32 ) };
	// 8324CF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CF50 size=52
    let mut pc: u32 = 0x8324CF50;
    'dispatch: loop {
        match pc {
            0x8324CF50 => {
    //   block [0x8324CF50..0x8324CF84)
	// 8324CF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CF5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CF60: 386BE104  addi r3, r11, -0x1efc
	ctx.r[3].s64 = ctx.r[11].s64 + -7932;
	// 8324CF64: 4AF3C1DD  bl 0x82189140
	ctx.lr = 0x8324CF68;
	sub_82189140(ctx, base);
	// 8324CF68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CF6C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324CF70: 916A7B74  stw r11, 0x7b74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31604 as u32), ctx.r[11].u32 ) };
	// 8324CF74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CF88 size=52
    let mut pc: u32 = 0x8324CF88;
    'dispatch: loop {
        match pc {
            0x8324CF88 => {
    //   block [0x8324CF88..0x8324CFBC)
	// 8324CF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CF94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CF98: 386BE118  addi r3, r11, -0x1ee8
	ctx.r[3].s64 = ctx.r[11].s64 + -7912;
	// 8324CF9C: 4AF3C1A5  bl 0x82189140
	ctx.lr = 0x8324CFA0;
	sub_82189140(ctx, base);
	// 8324CFA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CFA4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324CFA8: 916A7B78  stw r11, 0x7b78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31608 as u32), ctx.r[11].u32 ) };
	// 8324CFAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CFB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CFB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CFC0 size=52
    let mut pc: u32 = 0x8324CFC0;
    'dispatch: loop {
        match pc {
            0x8324CFC0 => {
    //   block [0x8324CFC0..0x8324CFF4)
	// 8324CFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CFC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CFCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CFD0: 386BE124  addi r3, r11, -0x1edc
	ctx.r[3].s64 = ctx.r[11].s64 + -7900;
	// 8324CFD4: 4AF3C16D  bl 0x82189140
	ctx.lr = 0x8324CFD8;
	sub_82189140(ctx, base);
	// 8324CFD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CFDC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324CFE0: 916A7B7C  stw r11, 0x7b7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31612 as u32), ctx.r[11].u32 ) };
	// 8324CFE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CFF8 size=52
    let mut pc: u32 = 0x8324CFF8;
    'dispatch: loop {
        match pc {
            0x8324CFF8 => {
    //   block [0x8324CFF8..0x8324D02C)
	// 8324CFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D004: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D008: 386BE134  addi r3, r11, -0x1ecc
	ctx.r[3].s64 = ctx.r[11].s64 + -7884;
	// 8324D00C: 4AF3C135  bl 0x82189140
	ctx.lr = 0x8324D010;
	sub_82189140(ctx, base);
	// 8324D010: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D014: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324D018: 916A7B80  stw r11, 0x7b80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31616 as u32), ctx.r[11].u32 ) };
	// 8324D01C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D030 size=64
    let mut pc: u32 = 0x8324D030;
    'dispatch: loop {
        match pc {
            0x8324D030 => {
    //   block [0x8324D030..0x8324D070)
	// 8324D030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D03C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D040: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D044: 388BE1A0  addi r4, r11, -0x1e60
	ctx.r[4].s64 = ctx.r[11].s64 + -7776;
	// 8324D048: 386A7B84  addi r3, r10, 0x7b84
	ctx.r[3].s64 = ctx.r[10].s64 + 31620;
	// 8324D04C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D050: 4AFDFE81  bl 0x8222ced0
	ctx.lr = 0x8324D054;
	sub_8222CED0(ctx, base);
	// 8324D054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D058: 38698848  addi r3, r9, -0x77b8
	ctx.r[3].s64 = ctx.r[9].s64 + -30648;
	// 8324D05C: 4BA5CEC5  bl 0x82ca9f20
	ctx.lr = 0x8324D060;
	sub_82CA9F20(ctx, base);
	// 8324D060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D070 size=64
    let mut pc: u32 = 0x8324D070;
    'dispatch: loop {
        match pc {
            0x8324D070 => {
    //   block [0x8324D070..0x8324D0B0)
	// 8324D070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D07C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D080: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D084: 388BE1AC  addi r4, r11, -0x1e54
	ctx.r[4].s64 = ctx.r[11].s64 + -7764;
	// 8324D088: 386A7B88  addi r3, r10, 0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + 31624;
	// 8324D08C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D090: 4AFDFE41  bl 0x8222ced0
	ctx.lr = 0x8324D094;
	sub_8222CED0(ctx, base);
	// 8324D094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D098: 38698858  addi r3, r9, -0x77a8
	ctx.r[3].s64 = ctx.r[9].s64 + -30632;
	// 8324D09C: 4BA5CE85  bl 0x82ca9f20
	ctx.lr = 0x8324D0A0;
	sub_82CA9F20(ctx, base);
	// 8324D0A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D0B0 size=64
    let mut pc: u32 = 0x8324D0B0;
    'dispatch: loop {
        match pc {
            0x8324D0B0 => {
    //   block [0x8324D0B0..0x8324D0F0)
	// 8324D0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D0BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D0C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D0C4: 388BE1C4  addi r4, r11, -0x1e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -7740;
	// 8324D0C8: 386A7B8C  addi r3, r10, 0x7b8c
	ctx.r[3].s64 = ctx.r[10].s64 + 31628;
	// 8324D0CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D0D0: 4AFDFE01  bl 0x8222ced0
	ctx.lr = 0x8324D0D4;
	sub_8222CED0(ctx, base);
	// 8324D0D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D0D8: 38698868  addi r3, r9, -0x7798
	ctx.r[3].s64 = ctx.r[9].s64 + -30616;
	// 8324D0DC: 4BA5CE45  bl 0x82ca9f20
	ctx.lr = 0x8324D0E0;
	sub_82CA9F20(ctx, base);
	// 8324D0E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D0F0 size=64
    let mut pc: u32 = 0x8324D0F0;
    'dispatch: loop {
        match pc {
            0x8324D0F0 => {
    //   block [0x8324D0F0..0x8324D130)
	// 8324D0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D0F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D0FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D100: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D104: 388BE1DC  addi r4, r11, -0x1e24
	ctx.r[4].s64 = ctx.r[11].s64 + -7716;
	// 8324D108: 386A7B90  addi r3, r10, 0x7b90
	ctx.r[3].s64 = ctx.r[10].s64 + 31632;
	// 8324D10C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D110: 4AFDFDC1  bl 0x8222ced0
	ctx.lr = 0x8324D114;
	sub_8222CED0(ctx, base);
	// 8324D114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D118: 38698878  addi r3, r9, -0x7788
	ctx.r[3].s64 = ctx.r[9].s64 + -30600;
	// 8324D11C: 4BA5CE05  bl 0x82ca9f20
	ctx.lr = 0x8324D120;
	sub_82CA9F20(ctx, base);
	// 8324D120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D130 size=64
    let mut pc: u32 = 0x8324D130;
    'dispatch: loop {
        match pc {
            0x8324D130 => {
    //   block [0x8324D130..0x8324D170)
	// 8324D130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D13C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D140: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D144: 388BE1EC  addi r4, r11, -0x1e14
	ctx.r[4].s64 = ctx.r[11].s64 + -7700;
	// 8324D148: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 8324D14C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D150: 4AFDFD81  bl 0x8222ced0
	ctx.lr = 0x8324D154;
	sub_8222CED0(ctx, base);
	// 8324D154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D158: 38698888  addi r3, r9, -0x7778
	ctx.r[3].s64 = ctx.r[9].s64 + -30584;
	// 8324D15C: 4BA5CDC5  bl 0x82ca9f20
	ctx.lr = 0x8324D160;
	sub_82CA9F20(ctx, base);
	// 8324D160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D170 size=64
    let mut pc: u32 = 0x8324D170;
    'dispatch: loop {
        match pc {
            0x8324D170 => {
    //   block [0x8324D170..0x8324D1B0)
	// 8324D170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D17C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D180: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D184: 388BE208  addi r4, r11, -0x1df8
	ctx.r[4].s64 = ctx.r[11].s64 + -7672;
	// 8324D188: 386A7B98  addi r3, r10, 0x7b98
	ctx.r[3].s64 = ctx.r[10].s64 + 31640;
	// 8324D18C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D190: 4AFDFD41  bl 0x8222ced0
	ctx.lr = 0x8324D194;
	sub_8222CED0(ctx, base);
	// 8324D194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D198: 38698898  addi r3, r9, -0x7768
	ctx.r[3].s64 = ctx.r[9].s64 + -30568;
	// 8324D19C: 4BA5CD85  bl 0x82ca9f20
	ctx.lr = 0x8324D1A0;
	sub_82CA9F20(ctx, base);
	// 8324D1A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D1B0 size=64
    let mut pc: u32 = 0x8324D1B0;
    'dispatch: loop {
        match pc {
            0x8324D1B0 => {
    //   block [0x8324D1B0..0x8324D1F0)
	// 8324D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D1BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D1C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D1C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D1C8: 386A7B9C  addi r3, r10, 0x7b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 31644;
	// 8324D1CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D1D0: 4AFDFD01  bl 0x8222ced0
	ctx.lr = 0x8324D1D4;
	sub_8222CED0(ctx, base);
	// 8324D1D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D1D8: 386988A8  addi r3, r9, -0x7758
	ctx.r[3].s64 = ctx.r[9].s64 + -30552;
	// 8324D1DC: 4BA5CD45  bl 0x82ca9f20
	ctx.lr = 0x8324D1E0;
	sub_82CA9F20(ctx, base);
	// 8324D1E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D1F0 size=64
    let mut pc: u32 = 0x8324D1F0;
    'dispatch: loop {
        match pc {
            0x8324D1F0 => {
    //   block [0x8324D1F0..0x8324D230)
	// 8324D1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D1F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D1FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D200: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D204: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D208: 386A7BA0  addi r3, r10, 0x7ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 31648;
	// 8324D20C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D210: 4AFDFCC1  bl 0x8222ced0
	ctx.lr = 0x8324D214;
	sub_8222CED0(ctx, base);
	// 8324D214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D218: 386988B8  addi r3, r9, -0x7748
	ctx.r[3].s64 = ctx.r[9].s64 + -30536;
	// 8324D21C: 4BA5CD05  bl 0x82ca9f20
	ctx.lr = 0x8324D220;
	sub_82CA9F20(ctx, base);
	// 8324D220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D230 size=64
    let mut pc: u32 = 0x8324D230;
    'dispatch: loop {
        match pc {
            0x8324D230 => {
    //   block [0x8324D230..0x8324D270)
	// 8324D230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D23C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D240: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D244: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D248: 386A7BA4  addi r3, r10, 0x7ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 31652;
	// 8324D24C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D250: 4AFDFC81  bl 0x8222ced0
	ctx.lr = 0x8324D254;
	sub_8222CED0(ctx, base);
	// 8324D254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D258: 386988C8  addi r3, r9, -0x7738
	ctx.r[3].s64 = ctx.r[9].s64 + -30520;
	// 8324D25C: 4BA5CCC5  bl 0x82ca9f20
	ctx.lr = 0x8324D260;
	sub_82CA9F20(ctx, base);
	// 8324D260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D270 size=64
    let mut pc: u32 = 0x8324D270;
    'dispatch: loop {
        match pc {
            0x8324D270 => {
    //   block [0x8324D270..0x8324D2B0)
	// 8324D270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D280: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D284: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D288: 386A7BA8  addi r3, r10, 0x7ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 31656;
	// 8324D28C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D290: 4AFDFC41  bl 0x8222ced0
	ctx.lr = 0x8324D294;
	sub_8222CED0(ctx, base);
	// 8324D294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D298: 386988D8  addi r3, r9, -0x7728
	ctx.r[3].s64 = ctx.r[9].s64 + -30504;
	// 8324D29C: 4BA5CC85  bl 0x82ca9f20
	ctx.lr = 0x8324D2A0;
	sub_82CA9F20(ctx, base);
	// 8324D2A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D2B0 size=56
    let mut pc: u32 = 0x8324D2B0;
    'dispatch: loop {
        match pc {
            0x8324D2B0 => {
    //   block [0x8324D2B0..0x8324D2E8)
	// 8324D2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D2BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D2C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D2C4: 386BE224  addi r3, r11, -0x1ddc
	ctx.r[3].s64 = ctx.r[11].s64 + -7644;
	// 8324D2C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D2CC: 4AFA6A8D  bl 0x821f3d58
	ctx.lr = 0x8324D2D0;
	sub_821F3D58(ctx, base);
	// 8324D2D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D2D4: 906A7BAC  stw r3, 0x7bac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31660 as u32), ctx.r[3].u32 ) };
	// 8324D2D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D2E8 size=64
    let mut pc: u32 = 0x8324D2E8;
    'dispatch: loop {
        match pc {
            0x8324D2E8 => {
    //   block [0x8324D2E8..0x8324D328)
	// 8324D2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D2F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D2F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D2F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D2FC: 388BE22C  addi r4, r11, -0x1dd4
	ctx.r[4].s64 = ctx.r[11].s64 + -7636;
	// 8324D300: 386A7BB0  addi r3, r10, 0x7bb0
	ctx.r[3].s64 = ctx.r[10].s64 + 31664;
	// 8324D304: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D308: 4AFDFBC9  bl 0x8222ced0
	ctx.lr = 0x8324D30C;
	sub_8222CED0(ctx, base);
	// 8324D30C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D310: 386988E8  addi r3, r9, -0x7718
	ctx.r[3].s64 = ctx.r[9].s64 + -30488;
	// 8324D314: 4BA5CC0D  bl 0x82ca9f20
	ctx.lr = 0x8324D318;
	sub_82CA9F20(ctx, base);
	// 8324D318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D328 size=64
    let mut pc: u32 = 0x8324D328;
    'dispatch: loop {
        match pc {
            0x8324D328 => {
    //   block [0x8324D328..0x8324D368)
	// 8324D328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D334: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D338: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D33C: 388BE234  addi r4, r11, -0x1dcc
	ctx.r[4].s64 = ctx.r[11].s64 + -7628;
	// 8324D340: 386A7BB4  addi r3, r10, 0x7bb4
	ctx.r[3].s64 = ctx.r[10].s64 + 31668;
	// 8324D344: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D348: 4AFDFB89  bl 0x8222ced0
	ctx.lr = 0x8324D34C;
	sub_8222CED0(ctx, base);
	// 8324D34C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D350: 386988F8  addi r3, r9, -0x7708
	ctx.r[3].s64 = ctx.r[9].s64 + -30472;
	// 8324D354: 4BA5CBCD  bl 0x82ca9f20
	ctx.lr = 0x8324D358;
	sub_82CA9F20(ctx, base);
	// 8324D358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D368 size=64
    let mut pc: u32 = 0x8324D368;
    'dispatch: loop {
        match pc {
            0x8324D368 => {
    //   block [0x8324D368..0x8324D3A8)
	// 8324D368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D374: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D378: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D37C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D380: 386A7BB8  addi r3, r10, 0x7bb8
	ctx.r[3].s64 = ctx.r[10].s64 + 31672;
	// 8324D384: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D388: 4AFDFB49  bl 0x8222ced0
	ctx.lr = 0x8324D38C;
	sub_8222CED0(ctx, base);
	// 8324D38C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D390: 38698908  addi r3, r9, -0x76f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30456;
	// 8324D394: 4BA5CB8D  bl 0x82ca9f20
	ctx.lr = 0x8324D398;
	sub_82CA9F20(ctx, base);
	// 8324D398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D3A8 size=64
    let mut pc: u32 = 0x8324D3A8;
    'dispatch: loop {
        match pc {
            0x8324D3A8 => {
    //   block [0x8324D3A8..0x8324D3E8)
	// 8324D3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D3B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D3B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D3B8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D3BC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D3C0: 386A7BBC  addi r3, r10, 0x7bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 31676;
	// 8324D3C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D3C8: 4AFDFB09  bl 0x8222ced0
	ctx.lr = 0x8324D3CC;
	sub_8222CED0(ctx, base);
	// 8324D3CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D3D0: 38698918  addi r3, r9, -0x76e8
	ctx.r[3].s64 = ctx.r[9].s64 + -30440;
	// 8324D3D4: 4BA5CB4D  bl 0x82ca9f20
	ctx.lr = 0x8324D3D8;
	sub_82CA9F20(ctx, base);
	// 8324D3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D3E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D3E8 size=64
    let mut pc: u32 = 0x8324D3E8;
    'dispatch: loop {
        match pc {
            0x8324D3E8 => {
    //   block [0x8324D3E8..0x8324D428)
	// 8324D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D3F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D3F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D3F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D3FC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D400: 386A7BC0  addi r3, r10, 0x7bc0
	ctx.r[3].s64 = ctx.r[10].s64 + 31680;
	// 8324D404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D408: 4AFDFAC9  bl 0x8222ced0
	ctx.lr = 0x8324D40C;
	sub_8222CED0(ctx, base);
	// 8324D40C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D410: 38698928  addi r3, r9, -0x76d8
	ctx.r[3].s64 = ctx.r[9].s64 + -30424;
	// 8324D414: 4BA5CB0D  bl 0x82ca9f20
	ctx.lr = 0x8324D418;
	sub_82CA9F20(ctx, base);
	// 8324D418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8324D428 size=32
    let mut pc: u32 = 0x8324D428;
    'dispatch: loop {
        match pc {
            0x8324D428 => {
    //   block [0x8324D428..0x8324D448)
	// 8324D428: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8324D42C: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D430: 392B9A2C  addi r9, r11, -0x65d4
	ctx.r[9].s64 = ctx.r[11].s64 + -26068;
	// 8324D434: C1AB9A2C  lfs f13, -0x65d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8324D438: C009FFF8  lfs f0, -8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8324D43C: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8324D440: D00A7BC4  stfs f0, 0x7bc4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31684 as u32), tmp.u32 ) };
	// 8324D444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D448 size=64
    let mut pc: u32 = 0x8324D448;
    'dispatch: loop {
        match pc {
            0x8324D448 => {
    //   block [0x8324D448..0x8324D488)
	// 8324D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D458: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D45C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D460: 386A7BC8  addi r3, r10, 0x7bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 31688;
	// 8324D464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D468: 4AFDFA69  bl 0x8222ced0
	ctx.lr = 0x8324D46C;
	sub_8222CED0(ctx, base);
	// 8324D46C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D470: 38698938  addi r3, r9, -0x76c8
	ctx.r[3].s64 = ctx.r[9].s64 + -30408;
	// 8324D474: 4BA5CAAD  bl 0x82ca9f20
	ctx.lr = 0x8324D478;
	sub_82CA9F20(ctx, base);
	// 8324D478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D488 size=64
    let mut pc: u32 = 0x8324D488;
    'dispatch: loop {
        match pc {
            0x8324D488 => {
    //   block [0x8324D488..0x8324D4C8)
	// 8324D488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D498: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D49C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D4A0: 386A7BCC  addi r3, r10, 0x7bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 31692;
	// 8324D4A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D4A8: 4AFDFA29  bl 0x8222ced0
	ctx.lr = 0x8324D4AC;
	sub_8222CED0(ctx, base);
	// 8324D4AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D4B0: 38698948  addi r3, r9, -0x76b8
	ctx.r[3].s64 = ctx.r[9].s64 + -30392;
	// 8324D4B4: 4BA5CA6D  bl 0x82ca9f20
	ctx.lr = 0x8324D4B8;
	sub_82CA9F20(ctx, base);
	// 8324D4B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D4C8 size=64
    let mut pc: u32 = 0x8324D4C8;
    'dispatch: loop {
        match pc {
            0x8324D4C8 => {
    //   block [0x8324D4C8..0x8324D508)
	// 8324D4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D4D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D4D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D4D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D4DC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D4E0: 386A7BD0  addi r3, r10, 0x7bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 31696;
	// 8324D4E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D4E8: 4AFDF9E9  bl 0x8222ced0
	ctx.lr = 0x8324D4EC;
	sub_8222CED0(ctx, base);
	// 8324D4EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D4F0: 38698958  addi r3, r9, -0x76a8
	ctx.r[3].s64 = ctx.r[9].s64 + -30376;
	// 8324D4F4: 4BA5CA2D  bl 0x82ca9f20
	ctx.lr = 0x8324D4F8;
	sub_82CA9F20(ctx, base);
	// 8324D4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D508 size=64
    let mut pc: u32 = 0x8324D508;
    'dispatch: loop {
        match pc {
            0x8324D508 => {
    //   block [0x8324D508..0x8324D548)
	// 8324D508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D514: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D518: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D51C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D520: 386A7BD4  addi r3, r10, 0x7bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 31700;
	// 8324D524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D528: 4AFDF9A9  bl 0x8222ced0
	ctx.lr = 0x8324D52C;
	sub_8222CED0(ctx, base);
	// 8324D52C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D530: 38698968  addi r3, r9, -0x7698
	ctx.r[3].s64 = ctx.r[9].s64 + -30360;
	// 8324D534: 4BA5C9ED  bl 0x82ca9f20
	ctx.lr = 0x8324D538;
	sub_82CA9F20(ctx, base);
	// 8324D538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D548 size=56
    let mut pc: u32 = 0x8324D548;
    'dispatch: loop {
        match pc {
            0x8324D548 => {
    //   block [0x8324D548..0x8324D580)
	// 8324D548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D554: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D55C: 386BE3B0  addi r3, r11, -0x1c50
	ctx.r[3].s64 = ctx.r[11].s64 + -7248;
	// 8324D560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D564: 4AFA67F5  bl 0x821f3d58
	ctx.lr = 0x8324D568;
	sub_821F3D58(ctx, base);
	// 8324D568: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D56C: 906A7BD8  stw r3, 0x7bd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31704 as u32), ctx.r[3].u32 ) };
	// 8324D570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D580 size=56
    let mut pc: u32 = 0x8324D580;
    'dispatch: loop {
        match pc {
            0x8324D580 => {
    //   block [0x8324D580..0x8324D5B8)
	// 8324D580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D58C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D594: 386BE3C0  addi r3, r11, -0x1c40
	ctx.r[3].s64 = ctx.r[11].s64 + -7232;
	// 8324D598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D59C: 4AFA67BD  bl 0x821f3d58
	ctx.lr = 0x8324D5A0;
	sub_821F3D58(ctx, base);
	// 8324D5A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D5A4: 906A7BDC  stw r3, 0x7bdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31708 as u32), ctx.r[3].u32 ) };
	// 8324D5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D5B8 size=64
    let mut pc: u32 = 0x8324D5B8;
    'dispatch: loop {
        match pc {
            0x8324D5B8 => {
    //   block [0x8324D5B8..0x8324D5F8)
	// 8324D5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D5C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D5C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D5CC: 388BE3D0  addi r4, r11, -0x1c30
	ctx.r[4].s64 = ctx.r[11].s64 + -7216;
	// 8324D5D0: 386A7BE0  addi r3, r10, 0x7be0
	ctx.r[3].s64 = ctx.r[10].s64 + 31712;
	// 8324D5D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D5D8: 4AFDF8F9  bl 0x8222ced0
	ctx.lr = 0x8324D5DC;
	sub_8222CED0(ctx, base);
	// 8324D5DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D5E0: 38698978  addi r3, r9, -0x7688
	ctx.r[3].s64 = ctx.r[9].s64 + -30344;
	// 8324D5E4: 4BA5C93D  bl 0x82ca9f20
	ctx.lr = 0x8324D5E8;
	sub_82CA9F20(ctx, base);
	// 8324D5E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D5F8 size=64
    let mut pc: u32 = 0x8324D5F8;
    'dispatch: loop {
        match pc {
            0x8324D5F8 => {
    //   block [0x8324D5F8..0x8324D638)
	// 8324D5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D604: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D608: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D60C: 388BE3E0  addi r4, r11, -0x1c20
	ctx.r[4].s64 = ctx.r[11].s64 + -7200;
	// 8324D610: 386A7BE4  addi r3, r10, 0x7be4
	ctx.r[3].s64 = ctx.r[10].s64 + 31716;
	// 8324D614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D618: 4AFDF8B9  bl 0x8222ced0
	ctx.lr = 0x8324D61C;
	sub_8222CED0(ctx, base);
	// 8324D61C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D620: 38698988  addi r3, r9, -0x7678
	ctx.r[3].s64 = ctx.r[9].s64 + -30328;
	// 8324D624: 4BA5C8FD  bl 0x82ca9f20
	ctx.lr = 0x8324D628;
	sub_82CA9F20(ctx, base);
	// 8324D628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D638 size=64
    let mut pc: u32 = 0x8324D638;
    'dispatch: loop {
        match pc {
            0x8324D638 => {
    //   block [0x8324D638..0x8324D678)
	// 8324D638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D644: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D648: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D64C: 388BE41C  addi r4, r11, -0x1be4
	ctx.r[4].s64 = ctx.r[11].s64 + -7140;
	// 8324D650: 386A7BE8  addi r3, r10, 0x7be8
	ctx.r[3].s64 = ctx.r[10].s64 + 31720;
	// 8324D654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D658: 4AFDF879  bl 0x8222ced0
	ctx.lr = 0x8324D65C;
	sub_8222CED0(ctx, base);
	// 8324D65C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D660: 38698998  addi r3, r9, -0x7668
	ctx.r[3].s64 = ctx.r[9].s64 + -30312;
	// 8324D664: 4BA5C8BD  bl 0x82ca9f20
	ctx.lr = 0x8324D668;
	sub_82CA9F20(ctx, base);
	// 8324D668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D678 size=64
    let mut pc: u32 = 0x8324D678;
    'dispatch: loop {
        match pc {
            0x8324D678 => {
    //   block [0x8324D678..0x8324D6B8)
	// 8324D678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D684: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D688: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D68C: 388BE438  addi r4, r11, -0x1bc8
	ctx.r[4].s64 = ctx.r[11].s64 + -7112;
	// 8324D690: 386A7BEC  addi r3, r10, 0x7bec
	ctx.r[3].s64 = ctx.r[10].s64 + 31724;
	// 8324D694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D698: 4AFDF839  bl 0x8222ced0
	ctx.lr = 0x8324D69C;
	sub_8222CED0(ctx, base);
	// 8324D69C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D6A0: 386989A8  addi r3, r9, -0x7658
	ctx.r[3].s64 = ctx.r[9].s64 + -30296;
	// 8324D6A4: 4BA5C87D  bl 0x82ca9f20
	ctx.lr = 0x8324D6A8;
	sub_82CA9F20(ctx, base);
	// 8324D6A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D6B8 size=64
    let mut pc: u32 = 0x8324D6B8;
    'dispatch: loop {
        match pc {
            0x8324D6B8 => {
    //   block [0x8324D6B8..0x8324D6F8)
	// 8324D6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D6C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D6C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D6C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D6CC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D6D0: 386A7BF0  addi r3, r10, 0x7bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 31728;
	// 8324D6D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D6D8: 4AFDF7F9  bl 0x8222ced0
	ctx.lr = 0x8324D6DC;
	sub_8222CED0(ctx, base);
	// 8324D6DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D6E0: 386989B8  addi r3, r9, -0x7648
	ctx.r[3].s64 = ctx.r[9].s64 + -30280;
	// 8324D6E4: 4BA5C83D  bl 0x82ca9f20
	ctx.lr = 0x8324D6E8;
	sub_82CA9F20(ctx, base);
	// 8324D6E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D6F8 size=64
    let mut pc: u32 = 0x8324D6F8;
    'dispatch: loop {
        match pc {
            0x8324D6F8 => {
    //   block [0x8324D6F8..0x8324D738)
	// 8324D6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D704: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D708: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D70C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D710: 386A7BF8  addi r3, r10, 0x7bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 31736;
	// 8324D714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D718: 4AFDF7B9  bl 0x8222ced0
	ctx.lr = 0x8324D71C;
	sub_8222CED0(ctx, base);
	// 8324D71C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D720: 386989C8  addi r3, r9, -0x7638
	ctx.r[3].s64 = ctx.r[9].s64 + -30264;
	// 8324D724: 4BA5C7FD  bl 0x82ca9f20
	ctx.lr = 0x8324D728;
	sub_82CA9F20(ctx, base);
	// 8324D728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D738 size=64
    let mut pc: u32 = 0x8324D738;
    'dispatch: loop {
        match pc {
            0x8324D738 => {
    //   block [0x8324D738..0x8324D778)
	// 8324D738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D748: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D74C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D750: 386A7BFC  addi r3, r10, 0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 31740;
	// 8324D754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D758: 4AFDF779  bl 0x8222ced0
	ctx.lr = 0x8324D75C;
	sub_8222CED0(ctx, base);
	// 8324D75C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D760: 386989D8  addi r3, r9, -0x7628
	ctx.r[3].s64 = ctx.r[9].s64 + -30248;
	// 8324D764: 4BA5C7BD  bl 0x82ca9f20
	ctx.lr = 0x8324D768;
	sub_82CA9F20(ctx, base);
	// 8324D768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D778 size=64
    let mut pc: u32 = 0x8324D778;
    'dispatch: loop {
        match pc {
            0x8324D778 => {
    //   block [0x8324D778..0x8324D7B8)
	// 8324D778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D784: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D788: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D78C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D790: 386A7C00  addi r3, r10, 0x7c00
	ctx.r[3].s64 = ctx.r[10].s64 + 31744;
	// 8324D794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D798: 4AFDF739  bl 0x8222ced0
	ctx.lr = 0x8324D79C;
	sub_8222CED0(ctx, base);
	// 8324D79C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D7A0: 386989E8  addi r3, r9, -0x7618
	ctx.r[3].s64 = ctx.r[9].s64 + -30232;
	// 8324D7A4: 4BA5C77D  bl 0x82ca9f20
	ctx.lr = 0x8324D7A8;
	sub_82CA9F20(ctx, base);
	// 8324D7A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D7B8 size=64
    let mut pc: u32 = 0x8324D7B8;
    'dispatch: loop {
        match pc {
            0x8324D7B8 => {
    //   block [0x8324D7B8..0x8324D7F8)
	// 8324D7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D7C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D7C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D7C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D7CC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D7D0: 386A7C04  addi r3, r10, 0x7c04
	ctx.r[3].s64 = ctx.r[10].s64 + 31748;
	// 8324D7D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D7D8: 4AFDF6F9  bl 0x8222ced0
	ctx.lr = 0x8324D7DC;
	sub_8222CED0(ctx, base);
	// 8324D7DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D7E0: 386989F8  addi r3, r9, -0x7608
	ctx.r[3].s64 = ctx.r[9].s64 + -30216;
	// 8324D7E4: 4BA5C73D  bl 0x82ca9f20
	ctx.lr = 0x8324D7E8;
	sub_82CA9F20(ctx, base);
	// 8324D7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D7F8 size=64
    let mut pc: u32 = 0x8324D7F8;
    'dispatch: loop {
        match pc {
            0x8324D7F8 => {
    //   block [0x8324D7F8..0x8324D838)
	// 8324D7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D804: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D808: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D80C: 388B8D00  addi r4, r11, -0x7300
	ctx.r[4].s64 = ctx.r[11].s64 + -29440;
	// 8324D810: 386A7C08  addi r3, r10, 0x7c08
	ctx.r[3].s64 = ctx.r[10].s64 + 31752;
	// 8324D814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D818: 4AFDF6B9  bl 0x8222ced0
	ctx.lr = 0x8324D81C;
	sub_8222CED0(ctx, base);
	// 8324D81C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D820: 38698A08  addi r3, r9, -0x75f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30200;
	// 8324D824: 4BA5C6FD  bl 0x82ca9f20
	ctx.lr = 0x8324D828;
	sub_82CA9F20(ctx, base);
	// 8324D828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D838 size=64
    let mut pc: u32 = 0x8324D838;
    'dispatch: loop {
        match pc {
            0x8324D838 => {
    //   block [0x8324D838..0x8324D878)
	// 8324D838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D844: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D848: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D84C: 388B8D18  addi r4, r11, -0x72e8
	ctx.r[4].s64 = ctx.r[11].s64 + -29416;
	// 8324D850: 386A7C0C  addi r3, r10, 0x7c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 31756;
	// 8324D854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D858: 4AFDF679  bl 0x8222ced0
	ctx.lr = 0x8324D85C;
	sub_8222CED0(ctx, base);
	// 8324D85C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D860: 38698A18  addi r3, r9, -0x75e8
	ctx.r[3].s64 = ctx.r[9].s64 + -30184;
	// 8324D864: 4BA5C6BD  bl 0x82ca9f20
	ctx.lr = 0x8324D868;
	sub_82CA9F20(ctx, base);
	// 8324D868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D86C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D878 size=64
    let mut pc: u32 = 0x8324D878;
    'dispatch: loop {
        match pc {
            0x8324D878 => {
    //   block [0x8324D878..0x8324D8B8)
	// 8324D878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D884: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D888: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D88C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D890: 386A7C10  addi r3, r10, 0x7c10
	ctx.r[3].s64 = ctx.r[10].s64 + 31760;
	// 8324D894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D898: 4AFDF639  bl 0x8222ced0
	ctx.lr = 0x8324D89C;
	sub_8222CED0(ctx, base);
	// 8324D89C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D8A0: 38698A28  addi r3, r9, -0x75d8
	ctx.r[3].s64 = ctx.r[9].s64 + -30168;
	// 8324D8A4: 4BA5C67D  bl 0x82ca9f20
	ctx.lr = 0x8324D8A8;
	sub_82CA9F20(ctx, base);
	// 8324D8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D8B8 size=64
    let mut pc: u32 = 0x8324D8B8;
    'dispatch: loop {
        match pc {
            0x8324D8B8 => {
    //   block [0x8324D8B8..0x8324D8F8)
	// 8324D8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D8C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D8C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D8C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D8CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D8D0: 386A7C14  addi r3, r10, 0x7c14
	ctx.r[3].s64 = ctx.r[10].s64 + 31764;
	// 8324D8D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D8D8: 4AFDF5F9  bl 0x8222ced0
	ctx.lr = 0x8324D8DC;
	sub_8222CED0(ctx, base);
	// 8324D8DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D8E0: 38698A38  addi r3, r9, -0x75c8
	ctx.r[3].s64 = ctx.r[9].s64 + -30152;
	// 8324D8E4: 4BA5C63D  bl 0x82ca9f20
	ctx.lr = 0x8324D8E8;
	sub_82CA9F20(ctx, base);
	// 8324D8E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D8F8 size=64
    let mut pc: u32 = 0x8324D8F8;
    'dispatch: loop {
        match pc {
            0x8324D8F8 => {
    //   block [0x8324D8F8..0x8324D938)
	// 8324D8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D908: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D90C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D910: 386A7C18  addi r3, r10, 0x7c18
	ctx.r[3].s64 = ctx.r[10].s64 + 31768;
	// 8324D914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D918: 4AFDF5B9  bl 0x8222ced0
	ctx.lr = 0x8324D91C;
	sub_8222CED0(ctx, base);
	// 8324D91C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D920: 38698A48  addi r3, r9, -0x75b8
	ctx.r[3].s64 = ctx.r[9].s64 + -30136;
	// 8324D924: 4BA5C5FD  bl 0x82ca9f20
	ctx.lr = 0x8324D928;
	sub_82CA9F20(ctx, base);
	// 8324D928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D938 size=64
    let mut pc: u32 = 0x8324D938;
    'dispatch: loop {
        match pc {
            0x8324D938 => {
    //   block [0x8324D938..0x8324D978)
	// 8324D938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D948: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D94C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D950: 386A7C1C  addi r3, r10, 0x7c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 31772;
	// 8324D954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D958: 4AFDF579  bl 0x8222ced0
	ctx.lr = 0x8324D95C;
	sub_8222CED0(ctx, base);
	// 8324D95C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D960: 38698A58  addi r3, r9, -0x75a8
	ctx.r[3].s64 = ctx.r[9].s64 + -30120;
	// 8324D964: 4BA5C5BD  bl 0x82ca9f20
	ctx.lr = 0x8324D968;
	sub_82CA9F20(ctx, base);
	// 8324D968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D978 size=56
    let mut pc: u32 = 0x8324D978;
    'dispatch: loop {
        match pc {
            0x8324D978 => {
    //   block [0x8324D978..0x8324D9B0)
	// 8324D978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D984: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D98C: 386BE6E8  addi r3, r11, -0x1918
	ctx.r[3].s64 = ctx.r[11].s64 + -6424;
	// 8324D990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D994: 4AFA63C5  bl 0x821f3d58
	ctx.lr = 0x8324D998;
	sub_821F3D58(ctx, base);
	// 8324D998: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D99C: 906A7C20  stw r3, 0x7c20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31776 as u32), ctx.r[3].u32 ) };
	// 8324D9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D9B0 size=56
    let mut pc: u32 = 0x8324D9B0;
    'dispatch: loop {
        match pc {
            0x8324D9B0 => {
    //   block [0x8324D9B0..0x8324D9E8)
	// 8324D9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D9BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D9C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D9C4: 386BE6F8  addi r3, r11, -0x1908
	ctx.r[3].s64 = ctx.r[11].s64 + -6408;
	// 8324D9C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D9CC: 4AFA638D  bl 0x821f3d58
	ctx.lr = 0x8324D9D0;
	sub_821F3D58(ctx, base);
	// 8324D9D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D9D4: 906A7C24  stw r3, 0x7c24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31780 as u32), ctx.r[3].u32 ) };
	// 8324D9D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D9E8 size=56
    let mut pc: u32 = 0x8324D9E8;
    'dispatch: loop {
        match pc {
            0x8324D9E8 => {
    //   block [0x8324D9E8..0x8324DA20)
	// 8324D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D9F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D9F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D9F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D9FC: 386BE710  addi r3, r11, -0x18f0
	ctx.r[3].s64 = ctx.r[11].s64 + -6384;
	// 8324DA00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DA04: 4AFA6355  bl 0x821f3d58
	ctx.lr = 0x8324DA08;
	sub_821F3D58(ctx, base);
	// 8324DA08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DA0C: 906A7C28  stw r3, 0x7c28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31784 as u32), ctx.r[3].u32 ) };
	// 8324DA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DA20 size=56
    let mut pc: u32 = 0x8324DA20;
    'dispatch: loop {
        match pc {
            0x8324DA20 => {
    //   block [0x8324DA20..0x8324DA58)
	// 8324DA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DA28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DA2C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DA30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DA34: 386BE728  addi r3, r11, -0x18d8
	ctx.r[3].s64 = ctx.r[11].s64 + -6360;
	// 8324DA38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DA3C: 4AFA631D  bl 0x821f3d58
	ctx.lr = 0x8324DA40;
	sub_821F3D58(ctx, base);
	// 8324DA40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DA44: 906A7C2C  stw r3, 0x7c2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31788 as u32), ctx.r[3].u32 ) };
	// 8324DA48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DA58 size=56
    let mut pc: u32 = 0x8324DA58;
    'dispatch: loop {
        match pc {
            0x8324DA58 => {
    //   block [0x8324DA58..0x8324DA90)
	// 8324DA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DA60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DA64: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DA68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DA6C: 386BE740  addi r3, r11, -0x18c0
	ctx.r[3].s64 = ctx.r[11].s64 + -6336;
	// 8324DA70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DA74: 4AFA62E5  bl 0x821f3d58
	ctx.lr = 0x8324DA78;
	sub_821F3D58(ctx, base);
	// 8324DA78: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DA7C: 906A7C30  stw r3, 0x7c30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31792 as u32), ctx.r[3].u32 ) };
	// 8324DA80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DA90 size=56
    let mut pc: u32 = 0x8324DA90;
    'dispatch: loop {
        match pc {
            0x8324DA90 => {
    //   block [0x8324DA90..0x8324DAC8)
	// 8324DA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DA98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DA9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DAA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DAA4: 386BE758  addi r3, r11, -0x18a8
	ctx.r[3].s64 = ctx.r[11].s64 + -6312;
	// 8324DAA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DAAC: 4AFA62AD  bl 0x821f3d58
	ctx.lr = 0x8324DAB0;
	sub_821F3D58(ctx, base);
	// 8324DAB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DAB4: 906A7C34  stw r3, 0x7c34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31796 as u32), ctx.r[3].u32 ) };
	// 8324DAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DAC8 size=56
    let mut pc: u32 = 0x8324DAC8;
    'dispatch: loop {
        match pc {
            0x8324DAC8 => {
    //   block [0x8324DAC8..0x8324DB00)
	// 8324DAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DAD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DAD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DAD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DADC: 386BE770  addi r3, r11, -0x1890
	ctx.r[3].s64 = ctx.r[11].s64 + -6288;
	// 8324DAE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DAE4: 4AFA6275  bl 0x821f3d58
	ctx.lr = 0x8324DAE8;
	sub_821F3D58(ctx, base);
	// 8324DAE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DAEC: 906A7C38  stw r3, 0x7c38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31800 as u32), ctx.r[3].u32 ) };
	// 8324DAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DB00 size=56
    let mut pc: u32 = 0x8324DB00;
    'dispatch: loop {
        match pc {
            0x8324DB00 => {
    //   block [0x8324DB00..0x8324DB38)
	// 8324DB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DB0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DB10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DB14: 386BE780  addi r3, r11, -0x1880
	ctx.r[3].s64 = ctx.r[11].s64 + -6272;
	// 8324DB18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DB1C: 4AFA623D  bl 0x821f3d58
	ctx.lr = 0x8324DB20;
	sub_821F3D58(ctx, base);
	// 8324DB20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DB24: 906A7C3C  stw r3, 0x7c3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31804 as u32), ctx.r[3].u32 ) };
	// 8324DB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DB38 size=64
    let mut pc: u32 = 0x8324DB38;
    'dispatch: loop {
        match pc {
            0x8324DB38 => {
    //   block [0x8324DB38..0x8324DB78)
	// 8324DB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DB40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DB44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DB48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DB4C: 388BE790  addi r4, r11, -0x1870
	ctx.r[4].s64 = ctx.r[11].s64 + -6256;
	// 8324DB50: 386A7C40  addi r3, r10, 0x7c40
	ctx.r[3].s64 = ctx.r[10].s64 + 31808;
	// 8324DB54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DB58: 4AFDF379  bl 0x8222ced0
	ctx.lr = 0x8324DB5C;
	sub_8222CED0(ctx, base);
	// 8324DB5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DB60: 38698A68  addi r3, r9, -0x7598
	ctx.r[3].s64 = ctx.r[9].s64 + -30104;
	// 8324DB64: 4BA5C3BD  bl 0x82ca9f20
	ctx.lr = 0x8324DB68;
	sub_82CA9F20(ctx, base);
	// 8324DB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DB78 size=64
    let mut pc: u32 = 0x8324DB78;
    'dispatch: loop {
        match pc {
            0x8324DB78 => {
    //   block [0x8324DB78..0x8324DBB8)
	// 8324DB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DB84: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DB88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DB8C: 388BE79C  addi r4, r11, -0x1864
	ctx.r[4].s64 = ctx.r[11].s64 + -6244;
	// 8324DB90: 386A7C44  addi r3, r10, 0x7c44
	ctx.r[3].s64 = ctx.r[10].s64 + 31812;
	// 8324DB94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DB98: 4AFDF339  bl 0x8222ced0
	ctx.lr = 0x8324DB9C;
	sub_8222CED0(ctx, base);
	// 8324DB9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DBA0: 38698A78  addi r3, r9, -0x7588
	ctx.r[3].s64 = ctx.r[9].s64 + -30088;
	// 8324DBA4: 4BA5C37D  bl 0x82ca9f20
	ctx.lr = 0x8324DBA8;
	sub_82CA9F20(ctx, base);
	// 8324DBA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DBB8 size=64
    let mut pc: u32 = 0x8324DBB8;
    'dispatch: loop {
        match pc {
            0x8324DBB8 => {
    //   block [0x8324DBB8..0x8324DBF8)
	// 8324DBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DBC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DBC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DBC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DBCC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324DBD0: 386A7C48  addi r3, r10, 0x7c48
	ctx.r[3].s64 = ctx.r[10].s64 + 31816;
	// 8324DBD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DBD8: 4AFDF2F9  bl 0x8222ced0
	ctx.lr = 0x8324DBDC;
	sub_8222CED0(ctx, base);
	// 8324DBDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DBE0: 38698A88  addi r3, r9, -0x7578
	ctx.r[3].s64 = ctx.r[9].s64 + -30072;
	// 8324DBE4: 4BA5C33D  bl 0x82ca9f20
	ctx.lr = 0x8324DBE8;
	sub_82CA9F20(ctx, base);
	// 8324DBE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DBF8 size=64
    let mut pc: u32 = 0x8324DBF8;
    'dispatch: loop {
        match pc {
            0x8324DBF8 => {
    //   block [0x8324DBF8..0x8324DC38)
	// 8324DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DC00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DC04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DC08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DC0C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324DC10: 386A7C4C  addi r3, r10, 0x7c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 31820;
	// 8324DC14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DC18: 4AFDF2B9  bl 0x8222ced0
	ctx.lr = 0x8324DC1C;
	sub_8222CED0(ctx, base);
	// 8324DC1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DC20: 38698A98  addi r3, r9, -0x7568
	ctx.r[3].s64 = ctx.r[9].s64 + -30056;
	// 8324DC24: 4BA5C2FD  bl 0x82ca9f20
	ctx.lr = 0x8324DC28;
	sub_82CA9F20(ctx, base);
	// 8324DC28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DC38 size=64
    let mut pc: u32 = 0x8324DC38;
    'dispatch: loop {
        match pc {
            0x8324DC38 => {
    //   block [0x8324DC38..0x8324DC78)
	// 8324DC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DC40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DC44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DC48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DC4C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324DC50: 386A7C50  addi r3, r10, 0x7c50
	ctx.r[3].s64 = ctx.r[10].s64 + 31824;
	// 8324DC54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DC58: 4AFDF279  bl 0x8222ced0
	ctx.lr = 0x8324DC5C;
	sub_8222CED0(ctx, base);
	// 8324DC5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DC60: 38698AA8  addi r3, r9, -0x7558
	ctx.r[3].s64 = ctx.r[9].s64 + -30040;
	// 8324DC64: 4BA5C2BD  bl 0x82ca9f20
	ctx.lr = 0x8324DC68;
	sub_82CA9F20(ctx, base);
	// 8324DC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DC78 size=64
    let mut pc: u32 = 0x8324DC78;
    'dispatch: loop {
        match pc {
            0x8324DC78 => {
    //   block [0x8324DC78..0x8324DCB8)
	// 8324DC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DC88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DC8C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324DC90: 386A7C54  addi r3, r10, 0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + 31828;
	// 8324DC94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DC98: 4AFDF239  bl 0x8222ced0
	ctx.lr = 0x8324DC9C;
	sub_8222CED0(ctx, base);
	// 8324DC9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DCA0: 38698AB8  addi r3, r9, -0x7548
	ctx.r[3].s64 = ctx.r[9].s64 + -30024;
	// 8324DCA4: 4BA5C27D  bl 0x82ca9f20
	ctx.lr = 0x8324DCA8;
	sub_82CA9F20(ctx, base);
	// 8324DCA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DCB8 size=56
    let mut pc: u32 = 0x8324DCB8;
    'dispatch: loop {
        match pc {
            0x8324DCB8 => {
    //   block [0x8324DCB8..0x8324DCF0)
	// 8324DCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DCC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DCC4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DCC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DCCC: 386BE7F8  addi r3, r11, -0x1808
	ctx.r[3].s64 = ctx.r[11].s64 + -6152;
	// 8324DCD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DCD4: 4AFA6085  bl 0x821f3d58
	ctx.lr = 0x8324DCD8;
	sub_821F3D58(ctx, base);
	// 8324DCD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DCDC: 906A7C58  stw r3, 0x7c58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31832 as u32), ctx.r[3].u32 ) };
	// 8324DCE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DCE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DCE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DCF0 size=56
    let mut pc: u32 = 0x8324DCF0;
    'dispatch: loop {
        match pc {
            0x8324DCF0 => {
    //   block [0x8324DCF0..0x8324DD28)
	// 8324DCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DCF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DCFC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DD00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DD04: 386BE800  addi r3, r11, -0x1800
	ctx.r[3].s64 = ctx.r[11].s64 + -6144;
	// 8324DD08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DD0C: 4AFA604D  bl 0x821f3d58
	ctx.lr = 0x8324DD10;
	sub_821F3D58(ctx, base);
	// 8324DD10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DD14: 906A7C5C  stw r3, 0x7c5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31836 as u32), ctx.r[3].u32 ) };
	// 8324DD18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DD28 size=56
    let mut pc: u32 = 0x8324DD28;
    'dispatch: loop {
        match pc {
            0x8324DD28 => {
    //   block [0x8324DD28..0x8324DD60)
	// 8324DD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DD34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DD38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DD3C: 386BE80C  addi r3, r11, -0x17f4
	ctx.r[3].s64 = ctx.r[11].s64 + -6132;
	// 8324DD40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DD44: 4AFA6015  bl 0x821f3d58
	ctx.lr = 0x8324DD48;
	sub_821F3D58(ctx, base);
	// 8324DD48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DD4C: 906A7C60  stw r3, 0x7c60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31840 as u32), ctx.r[3].u32 ) };
	// 8324DD50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DD60 size=56
    let mut pc: u32 = 0x8324DD60;
    'dispatch: loop {
        match pc {
            0x8324DD60 => {
    //   block [0x8324DD60..0x8324DD98)
	// 8324DD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DD68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DD6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DD70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DD74: 386BE814  addi r3, r11, -0x17ec
	ctx.r[3].s64 = ctx.r[11].s64 + -6124;
	// 8324DD78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DD7C: 4AFA5FDD  bl 0x821f3d58
	ctx.lr = 0x8324DD80;
	sub_821F3D58(ctx, base);
	// 8324DD80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DD84: 906A7C64  stw r3, 0x7c64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31844 as u32), ctx.r[3].u32 ) };
	// 8324DD88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DD98 size=56
    let mut pc: u32 = 0x8324DD98;
    'dispatch: loop {
        match pc {
            0x8324DD98 => {
    //   block [0x8324DD98..0x8324DDD0)
	// 8324DD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DDA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DDA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DDAC: 386BE824  addi r3, r11, -0x17dc
	ctx.r[3].s64 = ctx.r[11].s64 + -6108;
	// 8324DDB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DDB4: 4AFA5FA5  bl 0x821f3d58
	ctx.lr = 0x8324DDB8;
	sub_821F3D58(ctx, base);
	// 8324DDB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DDBC: 906A7C68  stw r3, 0x7c68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31848 as u32), ctx.r[3].u32 ) };
	// 8324DDC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DDD0 size=56
    let mut pc: u32 = 0x8324DDD0;
    'dispatch: loop {
        match pc {
            0x8324DDD0 => {
    //   block [0x8324DDD0..0x8324DE08)
	// 8324DDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DDD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DDDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DDE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DDE4: 386BE834  addi r3, r11, -0x17cc
	ctx.r[3].s64 = ctx.r[11].s64 + -6092;
	// 8324DDE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DDEC: 4AFA5F6D  bl 0x821f3d58
	ctx.lr = 0x8324DDF0;
	sub_821F3D58(ctx, base);
	// 8324DDF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DDF4: 906A7C6C  stw r3, 0x7c6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31852 as u32), ctx.r[3].u32 ) };
	// 8324DDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DE08 size=56
    let mut pc: u32 = 0x8324DE08;
    'dispatch: loop {
        match pc {
            0x8324DE08 => {
    //   block [0x8324DE08..0x8324DE40)
	// 8324DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DE14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DE18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DE1C: 386BE848  addi r3, r11, -0x17b8
	ctx.r[3].s64 = ctx.r[11].s64 + -6072;
	// 8324DE20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DE24: 4AFA5F35  bl 0x821f3d58
	ctx.lr = 0x8324DE28;
	sub_821F3D58(ctx, base);
	// 8324DE28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DE2C: 906A7C70  stw r3, 0x7c70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31856 as u32), ctx.r[3].u32 ) };
	// 8324DE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DE40 size=56
    let mut pc: u32 = 0x8324DE40;
    'dispatch: loop {
        match pc {
            0x8324DE40 => {
    //   block [0x8324DE40..0x8324DE78)
	// 8324DE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DE4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DE50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DE54: 386BE85C  addi r3, r11, -0x17a4
	ctx.r[3].s64 = ctx.r[11].s64 + -6052;
	// 8324DE58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DE5C: 4AFA5EFD  bl 0x821f3d58
	ctx.lr = 0x8324DE60;
	sub_821F3D58(ctx, base);
	// 8324DE60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DE64: 906A7C74  stw r3, 0x7c74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31860 as u32), ctx.r[3].u32 ) };
	// 8324DE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DE78 size=64
    let mut pc: u32 = 0x8324DE78;
    'dispatch: loop {
        match pc {
            0x8324DE78 => {
    //   block [0x8324DE78..0x8324DEB8)
	// 8324DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DE84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DE88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DE8C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324DE90: 386A7C78  addi r3, r10, 0x7c78
	ctx.r[3].s64 = ctx.r[10].s64 + 31864;
	// 8324DE94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DE98: 4AFDF039  bl 0x8222ced0
	ctx.lr = 0x8324DE9C;
	sub_8222CED0(ctx, base);
	// 8324DE9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DEA0: 38698AC8  addi r3, r9, -0x7538
	ctx.r[3].s64 = ctx.r[9].s64 + -30008;
	// 8324DEA4: 4BA5C07D  bl 0x82ca9f20
	ctx.lr = 0x8324DEA8;
	sub_82CA9F20(ctx, base);
	// 8324DEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DEB8 size=64
    let mut pc: u32 = 0x8324DEB8;
    'dispatch: loop {
        match pc {
            0x8324DEB8 => {
    //   block [0x8324DEB8..0x8324DEF8)
	// 8324DEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DEC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DEC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DEC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DECC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324DED0: 386A7C7C  addi r3, r10, 0x7c7c
	ctx.r[3].s64 = ctx.r[10].s64 + 31868;
	// 8324DED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DED8: 4AFDEFF9  bl 0x8222ced0
	ctx.lr = 0x8324DEDC;
	sub_8222CED0(ctx, base);
	// 8324DEDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DEE0: 38698AD8  addi r3, r9, -0x7528
	ctx.r[3].s64 = ctx.r[9].s64 + -29992;
	// 8324DEE4: 4BA5C03D  bl 0x82ca9f20
	ctx.lr = 0x8324DEE8;
	sub_82CA9F20(ctx, base);
	// 8324DEE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DEF8 size=64
    let mut pc: u32 = 0x8324DEF8;
    'dispatch: loop {
        match pc {
            0x8324DEF8 => {
    //   block [0x8324DEF8..0x8324DF38)
	// 8324DEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DF04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DF08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DF0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324DF10: 386A7C80  addi r3, r10, 0x7c80
	ctx.r[3].s64 = ctx.r[10].s64 + 31872;
	// 8324DF14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DF18: 4AFDEFB9  bl 0x8222ced0
	ctx.lr = 0x8324DF1C;
	sub_8222CED0(ctx, base);
	// 8324DF1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DF20: 38698AE8  addi r3, r9, -0x7518
	ctx.r[3].s64 = ctx.r[9].s64 + -29976;
	// 8324DF24: 4BA5BFFD  bl 0x82ca9f20
	ctx.lr = 0x8324DF28;
	sub_82CA9F20(ctx, base);
	// 8324DF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DF38 size=64
    let mut pc: u32 = 0x8324DF38;
    'dispatch: loop {
        match pc {
            0x8324DF38 => {
    //   block [0x8324DF38..0x8324DF78)
	// 8324DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DF40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DF44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DF48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DF4C: 388BE874  addi r4, r11, -0x178c
	ctx.r[4].s64 = ctx.r[11].s64 + -6028;
	// 8324DF50: 386A7C84  addi r3, r10, 0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + 31876;
	// 8324DF54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DF58: 4AFDEF79  bl 0x8222ced0
	ctx.lr = 0x8324DF5C;
	sub_8222CED0(ctx, base);
	// 8324DF5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DF60: 38698AF8  addi r3, r9, -0x7508
	ctx.r[3].s64 = ctx.r[9].s64 + -29960;
	// 8324DF64: 4BA5BFBD  bl 0x82ca9f20
	ctx.lr = 0x8324DF68;
	sub_82CA9F20(ctx, base);
	// 8324DF68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DF78 size=64
    let mut pc: u32 = 0x8324DF78;
    'dispatch: loop {
        match pc {
            0x8324DF78 => {
    //   block [0x8324DF78..0x8324DFB8)
	// 8324DF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DF84: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DF88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DF8C: 388BE894  addi r4, r11, -0x176c
	ctx.r[4].s64 = ctx.r[11].s64 + -5996;
	// 8324DF90: 386A7C88  addi r3, r10, 0x7c88
	ctx.r[3].s64 = ctx.r[10].s64 + 31880;
	// 8324DF94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DF98: 4AFDEF39  bl 0x8222ced0
	ctx.lr = 0x8324DF9C;
	sub_8222CED0(ctx, base);
	// 8324DF9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DFA0: 38698B08  addi r3, r9, -0x74f8
	ctx.r[3].s64 = ctx.r[9].s64 + -29944;
	// 8324DFA4: 4BA5BF7D  bl 0x82ca9f20
	ctx.lr = 0x8324DFA8;
	sub_82CA9F20(ctx, base);
	// 8324DFA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DFB8 size=64
    let mut pc: u32 = 0x8324DFB8;
    'dispatch: loop {
        match pc {
            0x8324DFB8 => {
    //   block [0x8324DFB8..0x8324DFF8)
	// 8324DFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DFC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DFC4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DFC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DFCC: 388BE8B8  addi r4, r11, -0x1748
	ctx.r[4].s64 = ctx.r[11].s64 + -5960;
	// 8324DFD0: 386A7C8C  addi r3, r10, 0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 31884;
	// 8324DFD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DFD8: 4AFDEEF9  bl 0x8222ced0
	ctx.lr = 0x8324DFDC;
	sub_8222CED0(ctx, base);
	// 8324DFDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DFE0: 38698B18  addi r3, r9, -0x74e8
	ctx.r[3].s64 = ctx.r[9].s64 + -29928;
	// 8324DFE4: 4BA5BF3D  bl 0x82ca9f20
	ctx.lr = 0x8324DFE8;
	sub_82CA9F20(ctx, base);
	// 8324DFE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DFF8 size=64
    let mut pc: u32 = 0x8324DFF8;
    'dispatch: loop {
        match pc {
            0x8324DFF8 => {
    //   block [0x8324DFF8..0x8324E038)
	// 8324DFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E004: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E008: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E00C: 388BE8DC  addi r4, r11, -0x1724
	ctx.r[4].s64 = ctx.r[11].s64 + -5924;
	// 8324E010: 386A7C90  addi r3, r10, 0x7c90
	ctx.r[3].s64 = ctx.r[10].s64 + 31888;
	// 8324E014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E018: 4AFDEEB9  bl 0x8222ced0
	ctx.lr = 0x8324E01C;
	sub_8222CED0(ctx, base);
	// 8324E01C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E020: 38698B28  addi r3, r9, -0x74d8
	ctx.r[3].s64 = ctx.r[9].s64 + -29912;
	// 8324E024: 4BA5BEFD  bl 0x82ca9f20
	ctx.lr = 0x8324E028;
	sub_82CA9F20(ctx, base);
	// 8324E028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E038 size=64
    let mut pc: u32 = 0x8324E038;
    'dispatch: loop {
        match pc {
            0x8324E038 => {
    //   block [0x8324E038..0x8324E078)
	// 8324E038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E044: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E048: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E04C: 388BE904  addi r4, r11, -0x16fc
	ctx.r[4].s64 = ctx.r[11].s64 + -5884;
	// 8324E050: 386A7C94  addi r3, r10, 0x7c94
	ctx.r[3].s64 = ctx.r[10].s64 + 31892;
	// 8324E054: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E058: 4AFDEE79  bl 0x8222ced0
	ctx.lr = 0x8324E05C;
	sub_8222CED0(ctx, base);
	// 8324E05C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E060: 38698B38  addi r3, r9, -0x74c8
	ctx.r[3].s64 = ctx.r[9].s64 + -29896;
	// 8324E064: 4BA5BEBD  bl 0x82ca9f20
	ctx.lr = 0x8324E068;
	sub_82CA9F20(ctx, base);
	// 8324E068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E078 size=64
    let mut pc: u32 = 0x8324E078;
    'dispatch: loop {
        match pc {
            0x8324E078 => {
    //   block [0x8324E078..0x8324E0B8)
	// 8324E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E084: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E088: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E08C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E090: 386A7C98  addi r3, r10, 0x7c98
	ctx.r[3].s64 = ctx.r[10].s64 + 31896;
	// 8324E094: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E098: 4AFDEE39  bl 0x8222ced0
	ctx.lr = 0x8324E09C;
	sub_8222CED0(ctx, base);
	// 8324E09C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E0A0: 38698B48  addi r3, r9, -0x74b8
	ctx.r[3].s64 = ctx.r[9].s64 + -29880;
	// 8324E0A4: 4BA5BE7D  bl 0x82ca9f20
	ctx.lr = 0x8324E0A8;
	sub_82CA9F20(ctx, base);
	// 8324E0A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E0B8 size=64
    let mut pc: u32 = 0x8324E0B8;
    'dispatch: loop {
        match pc {
            0x8324E0B8 => {
    //   block [0x8324E0B8..0x8324E0F8)
	// 8324E0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E0C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E0C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E0C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E0CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324E0D0: 386A7C9C  addi r3, r10, 0x7c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 31900;
	// 8324E0D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E0D8: 4AFDEDF9  bl 0x8222ced0
	ctx.lr = 0x8324E0DC;
	sub_8222CED0(ctx, base);
	// 8324E0DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E0E0: 38698B58  addi r3, r9, -0x74a8
	ctx.r[3].s64 = ctx.r[9].s64 + -29864;
	// 8324E0E4: 4BA5BE3D  bl 0x82ca9f20
	ctx.lr = 0x8324E0E8;
	sub_82CA9F20(ctx, base);
	// 8324E0E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E0F8 size=64
    let mut pc: u32 = 0x8324E0F8;
    'dispatch: loop {
        match pc {
            0x8324E0F8 => {
    //   block [0x8324E0F8..0x8324E138)
	// 8324E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E108: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E10C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324E110: 386A7CA0  addi r3, r10, 0x7ca0
	ctx.r[3].s64 = ctx.r[10].s64 + 31904;
	// 8324E114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E118: 4AFDEDB9  bl 0x8222ced0
	ctx.lr = 0x8324E11C;
	sub_8222CED0(ctx, base);
	// 8324E11C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E120: 38698B68  addi r3, r9, -0x7498
	ctx.r[3].s64 = ctx.r[9].s64 + -29848;
	// 8324E124: 4BA5BDFD  bl 0x82ca9f20
	ctx.lr = 0x8324E128;
	sub_82CA9F20(ctx, base);
	// 8324E128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E138 size=64
    let mut pc: u32 = 0x8324E138;
    'dispatch: loop {
        match pc {
            0x8324E138 => {
    //   block [0x8324E138..0x8324E178)
	// 8324E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E148: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E14C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E150: 386A7CA4  addi r3, r10, 0x7ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 31908;
	// 8324E154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E158: 4AFDED79  bl 0x8222ced0
	ctx.lr = 0x8324E15C;
	sub_8222CED0(ctx, base);
	// 8324E15C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E160: 38698B78  addi r3, r9, -0x7488
	ctx.r[3].s64 = ctx.r[9].s64 + -29832;
	// 8324E164: 4BA5BDBD  bl 0x82ca9f20
	ctx.lr = 0x8324E168;
	sub_82CA9F20(ctx, base);
	// 8324E168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E178 size=56
    let mut pc: u32 = 0x8324E178;
    'dispatch: loop {
        match pc {
            0x8324E178 => {
    //   block [0x8324E178..0x8324E1B0)
	// 8324E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E184: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E18C: 386BEA10  addi r3, r11, -0x15f0
	ctx.r[3].s64 = ctx.r[11].s64 + -5616;
	// 8324E190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E194: 4AFA5BC5  bl 0x821f3d58
	ctx.lr = 0x8324E198;
	sub_821F3D58(ctx, base);
	// 8324E198: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E19C: 906A7CA8  stw r3, 0x7ca8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31912 as u32), ctx.r[3].u32 ) };
	// 8324E1A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E1B0 size=56
    let mut pc: u32 = 0x8324E1B0;
    'dispatch: loop {
        match pc {
            0x8324E1B0 => {
    //   block [0x8324E1B0..0x8324E1E8)
	// 8324E1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E1BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E1C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E1C4: 386BEA20  addi r3, r11, -0x15e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5600;
	// 8324E1C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E1CC: 4AFA5B8D  bl 0x821f3d58
	ctx.lr = 0x8324E1D0;
	sub_821F3D58(ctx, base);
	// 8324E1D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E1D4: 906A7CAC  stw r3, 0x7cac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31916 as u32), ctx.r[3].u32 ) };
	// 8324E1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E1E8 size=56
    let mut pc: u32 = 0x8324E1E8;
    'dispatch: loop {
        match pc {
            0x8324E1E8 => {
    //   block [0x8324E1E8..0x8324E220)
	// 8324E1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E1F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E1F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E1F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E1FC: 386BEA34  addi r3, r11, -0x15cc
	ctx.r[3].s64 = ctx.r[11].s64 + -5580;
	// 8324E200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E204: 4AFA5B55  bl 0x821f3d58
	ctx.lr = 0x8324E208;
	sub_821F3D58(ctx, base);
	// 8324E208: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E20C: 906A7CB0  stw r3, 0x7cb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31920 as u32), ctx.r[3].u32 ) };
	// 8324E210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E220 size=56
    let mut pc: u32 = 0x8324E220;
    'dispatch: loop {
        match pc {
            0x8324E220 => {
    //   block [0x8324E220..0x8324E258)
	// 8324E220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E22C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E234: 386BEA44  addi r3, r11, -0x15bc
	ctx.r[3].s64 = ctx.r[11].s64 + -5564;
	// 8324E238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E23C: 4AFA5B1D  bl 0x821f3d58
	ctx.lr = 0x8324E240;
	sub_821F3D58(ctx, base);
	// 8324E240: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E244: 906A7CB4  stw r3, 0x7cb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31924 as u32), ctx.r[3].u32 ) };
	// 8324E248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E258 size=56
    let mut pc: u32 = 0x8324E258;
    'dispatch: loop {
        match pc {
            0x8324E258 => {
    //   block [0x8324E258..0x8324E290)
	// 8324E258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E264: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E26C: 386BEA54  addi r3, r11, -0x15ac
	ctx.r[3].s64 = ctx.r[11].s64 + -5548;
	// 8324E270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E274: 4AFA5AE5  bl 0x821f3d58
	ctx.lr = 0x8324E278;
	sub_821F3D58(ctx, base);
	// 8324E278: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E27C: 906A7CB8  stw r3, 0x7cb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31928 as u32), ctx.r[3].u32 ) };
	// 8324E280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E290 size=56
    let mut pc: u32 = 0x8324E290;
    'dispatch: loop {
        match pc {
            0x8324E290 => {
    //   block [0x8324E290..0x8324E2C8)
	// 8324E290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E29C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E2A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E2A4: 386BEA68  addi r3, r11, -0x1598
	ctx.r[3].s64 = ctx.r[11].s64 + -5528;
	// 8324E2A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E2AC: 4AFA5AAD  bl 0x821f3d58
	ctx.lr = 0x8324E2B0;
	sub_821F3D58(ctx, base);
	// 8324E2B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E2B4: 906A7CBC  stw r3, 0x7cbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31932 as u32), ctx.r[3].u32 ) };
	// 8324E2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E2C8 size=56
    let mut pc: u32 = 0x8324E2C8;
    'dispatch: loop {
        match pc {
            0x8324E2C8 => {
    //   block [0x8324E2C8..0x8324E300)
	// 8324E2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E2D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E2D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E2D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E2DC: 386BEA78  addi r3, r11, -0x1588
	ctx.r[3].s64 = ctx.r[11].s64 + -5512;
	// 8324E2E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E2E4: 4AFA5A75  bl 0x821f3d58
	ctx.lr = 0x8324E2E8;
	sub_821F3D58(ctx, base);
	// 8324E2E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E2EC: 906A7CC0  stw r3, 0x7cc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31936 as u32), ctx.r[3].u32 ) };
	// 8324E2F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E300 size=56
    let mut pc: u32 = 0x8324E300;
    'dispatch: loop {
        match pc {
            0x8324E300 => {
    //   block [0x8324E300..0x8324E338)
	// 8324E300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E30C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E310: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E314: 386BEA88  addi r3, r11, -0x1578
	ctx.r[3].s64 = ctx.r[11].s64 + -5496;
	// 8324E318: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E31C: 4AFA5A3D  bl 0x821f3d58
	ctx.lr = 0x8324E320;
	sub_821F3D58(ctx, base);
	// 8324E320: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E324: 906A7CC4  stw r3, 0x7cc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31940 as u32), ctx.r[3].u32 ) };
	// 8324E328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E338 size=56
    let mut pc: u32 = 0x8324E338;
    'dispatch: loop {
        match pc {
            0x8324E338 => {
    //   block [0x8324E338..0x8324E370)
	// 8324E338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E344: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E34C: 386BEA98  addi r3, r11, -0x1568
	ctx.r[3].s64 = ctx.r[11].s64 + -5480;
	// 8324E350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E354: 4AFA5A05  bl 0x821f3d58
	ctx.lr = 0x8324E358;
	sub_821F3D58(ctx, base);
	// 8324E358: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E35C: 906A7CC8  stw r3, 0x7cc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31944 as u32), ctx.r[3].u32 ) };
	// 8324E360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E370 size=64
    let mut pc: u32 = 0x8324E370;
    'dispatch: loop {
        match pc {
            0x8324E370 => {
    //   block [0x8324E370..0x8324E3B0)
	// 8324E370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E37C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E380: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E384: 388BEAA8  addi r4, r11, -0x1558
	ctx.r[4].s64 = ctx.r[11].s64 + -5464;
	// 8324E388: 386A7CCC  addi r3, r10, 0x7ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 31948;
	// 8324E38C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E390: 4AFDEB41  bl 0x8222ced0
	ctx.lr = 0x8324E394;
	sub_8222CED0(ctx, base);
	// 8324E394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E398: 38698B88  addi r3, r9, -0x7478
	ctx.r[3].s64 = ctx.r[9].s64 + -29816;
	// 8324E39C: 4BA5BB85  bl 0x82ca9f20
	ctx.lr = 0x8324E3A0;
	sub_82CA9F20(ctx, base);
	// 8324E3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E3B0 size=64
    let mut pc: u32 = 0x8324E3B0;
    'dispatch: loop {
        match pc {
            0x8324E3B0 => {
    //   block [0x8324E3B0..0x8324E3F0)
	// 8324E3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E3BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E3C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E3C4: 388BEAC4  addi r4, r11, -0x153c
	ctx.r[4].s64 = ctx.r[11].s64 + -5436;
	// 8324E3C8: 386A7CD0  addi r3, r10, 0x7cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 31952;
	// 8324E3CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E3D0: 4AFDEB01  bl 0x8222ced0
	ctx.lr = 0x8324E3D4;
	sub_8222CED0(ctx, base);
	// 8324E3D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E3D8: 38698B98  addi r3, r9, -0x7468
	ctx.r[3].s64 = ctx.r[9].s64 + -29800;
	// 8324E3DC: 4BA5BB45  bl 0x82ca9f20
	ctx.lr = 0x8324E3E0;
	sub_82CA9F20(ctx, base);
	// 8324E3E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E3F0 size=64
    let mut pc: u32 = 0x8324E3F0;
    'dispatch: loop {
        match pc {
            0x8324E3F0 => {
    //   block [0x8324E3F0..0x8324E430)
	// 8324E3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E3F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E3FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E400: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E404: 388BEAE0  addi r4, r11, -0x1520
	ctx.r[4].s64 = ctx.r[11].s64 + -5408;
	// 8324E408: 386A7CD4  addi r3, r10, 0x7cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 31956;
	// 8324E40C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E410: 4AFDEAC1  bl 0x8222ced0
	ctx.lr = 0x8324E414;
	sub_8222CED0(ctx, base);
	// 8324E414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E418: 38698BA8  addi r3, r9, -0x7458
	ctx.r[3].s64 = ctx.r[9].s64 + -29784;
	// 8324E41C: 4BA5BB05  bl 0x82ca9f20
	ctx.lr = 0x8324E420;
	sub_82CA9F20(ctx, base);
	// 8324E420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E430 size=64
    let mut pc: u32 = 0x8324E430;
    'dispatch: loop {
        match pc {
            0x8324E430 => {
    //   block [0x8324E430..0x8324E470)
	// 8324E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E43C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E440: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E444: 388BEAFC  addi r4, r11, -0x1504
	ctx.r[4].s64 = ctx.r[11].s64 + -5380;
	// 8324E448: 386A7CD8  addi r3, r10, 0x7cd8
	ctx.r[3].s64 = ctx.r[10].s64 + 31960;
	// 8324E44C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E450: 4AFDEA81  bl 0x8222ced0
	ctx.lr = 0x8324E454;
	sub_8222CED0(ctx, base);
	// 8324E454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E458: 38698BB8  addi r3, r9, -0x7448
	ctx.r[3].s64 = ctx.r[9].s64 + -29768;
	// 8324E45C: 4BA5BAC5  bl 0x82ca9f20
	ctx.lr = 0x8324E460;
	sub_82CA9F20(ctx, base);
	// 8324E460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E470 size=64
    let mut pc: u32 = 0x8324E470;
    'dispatch: loop {
        match pc {
            0x8324E470 => {
    //   block [0x8324E470..0x8324E4B0)
	// 8324E470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E47C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E480: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E484: 388BE790  addi r4, r11, -0x1870
	ctx.r[4].s64 = ctx.r[11].s64 + -6256;
	// 8324E488: 386A7CDC  addi r3, r10, 0x7cdc
	ctx.r[3].s64 = ctx.r[10].s64 + 31964;
	// 8324E48C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E490: 4AFDEA41  bl 0x8222ced0
	ctx.lr = 0x8324E494;
	sub_8222CED0(ctx, base);
	// 8324E494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E498: 38698BC8  addi r3, r9, -0x7438
	ctx.r[3].s64 = ctx.r[9].s64 + -29752;
	// 8324E49C: 4BA5BA85  bl 0x82ca9f20
	ctx.lr = 0x8324E4A0;
	sub_82CA9F20(ctx, base);
	// 8324E4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E4B0 size=64
    let mut pc: u32 = 0x8324E4B0;
    'dispatch: loop {
        match pc {
            0x8324E4B0 => {
    //   block [0x8324E4B0..0x8324E4F0)
	// 8324E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E4BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E4C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E4C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E4C8: 386A7CE0  addi r3, r10, 0x7ce0
	ctx.r[3].s64 = ctx.r[10].s64 + 31968;
	// 8324E4CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E4D0: 4AFDEA01  bl 0x8222ced0
	ctx.lr = 0x8324E4D4;
	sub_8222CED0(ctx, base);
	// 8324E4D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E4D8: 38698BD8  addi r3, r9, -0x7428
	ctx.r[3].s64 = ctx.r[9].s64 + -29736;
	// 8324E4DC: 4BA5BA45  bl 0x82ca9f20
	ctx.lr = 0x8324E4E0;
	sub_82CA9F20(ctx, base);
	// 8324E4E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E4F0 size=64
    let mut pc: u32 = 0x8324E4F0;
    'dispatch: loop {
        match pc {
            0x8324E4F0 => {
    //   block [0x8324E4F0..0x8324E530)
	// 8324E4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E4FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E500: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E504: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324E508: 386A7CE4  addi r3, r10, 0x7ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 31972;
	// 8324E50C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E510: 4AFDE9C1  bl 0x8222ced0
	ctx.lr = 0x8324E514;
	sub_8222CED0(ctx, base);
	// 8324E514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E518: 38698BE8  addi r3, r9, -0x7418
	ctx.r[3].s64 = ctx.r[9].s64 + -29720;
	// 8324E51C: 4BA5BA05  bl 0x82ca9f20
	ctx.lr = 0x8324E520;
	sub_82CA9F20(ctx, base);
	// 8324E520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E530 size=64
    let mut pc: u32 = 0x8324E530;
    'dispatch: loop {
        match pc {
            0x8324E530 => {
    //   block [0x8324E530..0x8324E570)
	// 8324E530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E53C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E540: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E544: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324E548: 386A7CE8  addi r3, r10, 0x7ce8
	ctx.r[3].s64 = ctx.r[10].s64 + 31976;
	// 8324E54C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E550: 4AFDE981  bl 0x8222ced0
	ctx.lr = 0x8324E554;
	sub_8222CED0(ctx, base);
	// 8324E554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E558: 38698BF8  addi r3, r9, -0x7408
	ctx.r[3].s64 = ctx.r[9].s64 + -29704;
	// 8324E55C: 4BA5B9C5  bl 0x82ca9f20
	ctx.lr = 0x8324E560;
	sub_82CA9F20(ctx, base);
	// 8324E560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E570 size=64
    let mut pc: u32 = 0x8324E570;
    'dispatch: loop {
        match pc {
            0x8324E570 => {
    //   block [0x8324E570..0x8324E5B0)
	// 8324E570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E57C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E580: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E584: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E588: 386A7CEC  addi r3, r10, 0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + 31980;
	// 8324E58C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E590: 4AFDE941  bl 0x8222ced0
	ctx.lr = 0x8324E594;
	sub_8222CED0(ctx, base);
	// 8324E594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E598: 38698C08  addi r3, r9, -0x73f8
	ctx.r[3].s64 = ctx.r[9].s64 + -29688;
	// 8324E59C: 4BA5B985  bl 0x82ca9f20
	ctx.lr = 0x8324E5A0;
	sub_82CA9F20(ctx, base);
	// 8324E5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E5B0 size=64
    let mut pc: u32 = 0x8324E5B0;
    'dispatch: loop {
        match pc {
            0x8324E5B0 => {
    //   block [0x8324E5B0..0x8324E5F0)
	// 8324E5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E5B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E5BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E5C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E5C4: 388BEDC8  addi r4, r11, -0x1238
	ctx.r[4].s64 = ctx.r[11].s64 + -4664;
	// 8324E5C8: 386A7CF0  addi r3, r10, 0x7cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 31984;
	// 8324E5CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E5D0: 4AFDE901  bl 0x8222ced0
	ctx.lr = 0x8324E5D4;
	sub_8222CED0(ctx, base);
	// 8324E5D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E5D8: 38698C18  addi r3, r9, -0x73e8
	ctx.r[3].s64 = ctx.r[9].s64 + -29672;
	// 8324E5DC: 4BA5B945  bl 0x82ca9f20
	ctx.lr = 0x8324E5E0;
	sub_82CA9F20(ctx, base);
	// 8324E5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E5F0 size=64
    let mut pc: u32 = 0x8324E5F0;
    'dispatch: loop {
        match pc {
            0x8324E5F0 => {
    //   block [0x8324E5F0..0x8324E630)
	// 8324E5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E5FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E600: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E604: 388BEDD4  addi r4, r11, -0x122c
	ctx.r[4].s64 = ctx.r[11].s64 + -4652;
	// 8324E608: 386A7CF4  addi r3, r10, 0x7cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31988;
	// 8324E60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E610: 4AFDE8C1  bl 0x8222ced0
	ctx.lr = 0x8324E614;
	sub_8222CED0(ctx, base);
	// 8324E614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E618: 38698C28  addi r3, r9, -0x73d8
	ctx.r[3].s64 = ctx.r[9].s64 + -29656;
	// 8324E61C: 4BA5B905  bl 0x82ca9f20
	ctx.lr = 0x8324E620;
	sub_82CA9F20(ctx, base);
	// 8324E620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E630 size=64
    let mut pc: u32 = 0x8324E630;
    'dispatch: loop {
        match pc {
            0x8324E630 => {
    //   block [0x8324E630..0x8324E670)
	// 8324E630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E63C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E640: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E644: 388BEDF0  addi r4, r11, -0x1210
	ctx.r[4].s64 = ctx.r[11].s64 + -4624;
	// 8324E648: 386A7CF8  addi r3, r10, 0x7cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 31992;
	// 8324E64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E650: 4AFDE881  bl 0x8222ced0
	ctx.lr = 0x8324E654;
	sub_8222CED0(ctx, base);
	// 8324E654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E658: 38698C38  addi r3, r9, -0x73c8
	ctx.r[3].s64 = ctx.r[9].s64 + -29640;
	// 8324E65C: 4BA5B8C5  bl 0x82ca9f20
	ctx.lr = 0x8324E660;
	sub_82CA9F20(ctx, base);
	// 8324E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E670 size=64
    let mut pc: u32 = 0x8324E670;
    'dispatch: loop {
        match pc {
            0x8324E670 => {
    //   block [0x8324E670..0x8324E6B0)
	// 8324E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E67C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E680: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E684: 388BEE14  addi r4, r11, -0x11ec
	ctx.r[4].s64 = ctx.r[11].s64 + -4588;
	// 8324E688: 386A7CFC  addi r3, r10, 0x7cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 31996;
	// 8324E68C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E690: 4AFDE841  bl 0x8222ced0
	ctx.lr = 0x8324E694;
	sub_8222CED0(ctx, base);
	// 8324E694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E698: 38698C48  addi r3, r9, -0x73b8
	ctx.r[3].s64 = ctx.r[9].s64 + -29624;
	// 8324E69C: 4BA5B885  bl 0x82ca9f20
	ctx.lr = 0x8324E6A0;
	sub_82CA9F20(ctx, base);
	// 8324E6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E6B0 size=64
    let mut pc: u32 = 0x8324E6B0;
    'dispatch: loop {
        match pc {
            0x8324E6B0 => {
    //   block [0x8324E6B0..0x8324E6F0)
	// 8324E6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E6BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E6C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E6C4: 388BEE2C  addi r4, r11, -0x11d4
	ctx.r[4].s64 = ctx.r[11].s64 + -4564;
	// 8324E6C8: 386A7D00  addi r3, r10, 0x7d00
	ctx.r[3].s64 = ctx.r[10].s64 + 32000;
	// 8324E6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E6D0: 4AFDE801  bl 0x8222ced0
	ctx.lr = 0x8324E6D4;
	sub_8222CED0(ctx, base);
	// 8324E6D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E6D8: 38698C58  addi r3, r9, -0x73a8
	ctx.r[3].s64 = ctx.r[9].s64 + -29608;
	// 8324E6DC: 4BA5B845  bl 0x82ca9f20
	ctx.lr = 0x8324E6E0;
	sub_82CA9F20(ctx, base);
	// 8324E6E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E6F0 size=64
    let mut pc: u32 = 0x8324E6F0;
    'dispatch: loop {
        match pc {
            0x8324E6F0 => {
    //   block [0x8324E6F0..0x8324E730)
	// 8324E6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E6FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E700: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E704: 388BEE3C  addi r4, r11, -0x11c4
	ctx.r[4].s64 = ctx.r[11].s64 + -4548;
	// 8324E708: 386A7D04  addi r3, r10, 0x7d04
	ctx.r[3].s64 = ctx.r[10].s64 + 32004;
	// 8324E70C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E710: 4AFDE7C1  bl 0x8222ced0
	ctx.lr = 0x8324E714;
	sub_8222CED0(ctx, base);
	// 8324E714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E718: 38698C68  addi r3, r9, -0x7398
	ctx.r[3].s64 = ctx.r[9].s64 + -29592;
	// 8324E71C: 4BA5B805  bl 0x82ca9f20
	ctx.lr = 0x8324E720;
	sub_82CA9F20(ctx, base);
	// 8324E720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E730 size=64
    let mut pc: u32 = 0x8324E730;
    'dispatch: loop {
        match pc {
            0x8324E730 => {
    //   block [0x8324E730..0x8324E770)
	// 8324E730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E73C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E740: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E744: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E748: 386A7D08  addi r3, r10, 0x7d08
	ctx.r[3].s64 = ctx.r[10].s64 + 32008;
	// 8324E74C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E750: 4AFDE781  bl 0x8222ced0
	ctx.lr = 0x8324E754;
	sub_8222CED0(ctx, base);
	// 8324E754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E758: 38698C78  addi r3, r9, -0x7388
	ctx.r[3].s64 = ctx.r[9].s64 + -29576;
	// 8324E75C: 4BA5B7C5  bl 0x82ca9f20
	ctx.lr = 0x8324E760;
	sub_82CA9F20(ctx, base);
	// 8324E760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E770 size=64
    let mut pc: u32 = 0x8324E770;
    'dispatch: loop {
        match pc {
            0x8324E770 => {
    //   block [0x8324E770..0x8324E7B0)
	// 8324E770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E77C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E780: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E784: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E788: 386A7D0C  addi r3, r10, 0x7d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32012;
	// 8324E78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E790: 4AFDE741  bl 0x8222ced0
	ctx.lr = 0x8324E794;
	sub_8222CED0(ctx, base);
	// 8324E794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E798: 38698C88  addi r3, r9, -0x7378
	ctx.r[3].s64 = ctx.r[9].s64 + -29560;
	// 8324E79C: 4BA5B785  bl 0x82ca9f20
	ctx.lr = 0x8324E7A0;
	sub_82CA9F20(ctx, base);
	// 8324E7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E7B0 size=64
    let mut pc: u32 = 0x8324E7B0;
    'dispatch: loop {
        match pc {
            0x8324E7B0 => {
    //   block [0x8324E7B0..0x8324E7F0)
	// 8324E7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E7BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E7C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E7C4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324E7C8: 386A7D10  addi r3, r10, 0x7d10
	ctx.r[3].s64 = ctx.r[10].s64 + 32016;
	// 8324E7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E7D0: 4AFDE701  bl 0x8222ced0
	ctx.lr = 0x8324E7D4;
	sub_8222CED0(ctx, base);
	// 8324E7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E7D8: 38698C98  addi r3, r9, -0x7368
	ctx.r[3].s64 = ctx.r[9].s64 + -29544;
	// 8324E7DC: 4BA5B745  bl 0x82ca9f20
	ctx.lr = 0x8324E7E0;
	sub_82CA9F20(ctx, base);
	// 8324E7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E7F0 size=64
    let mut pc: u32 = 0x8324E7F0;
    'dispatch: loop {
        match pc {
            0x8324E7F0 => {
    //   block [0x8324E7F0..0x8324E830)
	// 8324E7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E7FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E800: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E804: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324E808: 386A7D14  addi r3, r10, 0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + 32020;
	// 8324E80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E810: 4AFDE6C1  bl 0x8222ced0
	ctx.lr = 0x8324E814;
	sub_8222CED0(ctx, base);
	// 8324E814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E818: 38698CA8  addi r3, r9, -0x7358
	ctx.r[3].s64 = ctx.r[9].s64 + -29528;
	// 8324E81C: 4BA5B705  bl 0x82ca9f20
	ctx.lr = 0x8324E820;
	sub_82CA9F20(ctx, base);
	// 8324E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E830 size=64
    let mut pc: u32 = 0x8324E830;
    'dispatch: loop {
        match pc {
            0x8324E830 => {
    //   block [0x8324E830..0x8324E870)
	// 8324E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E840: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E844: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E848: 386A7D18  addi r3, r10, 0x7d18
	ctx.r[3].s64 = ctx.r[10].s64 + 32024;
	// 8324E84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E850: 4AFDE681  bl 0x8222ced0
	ctx.lr = 0x8324E854;
	sub_8222CED0(ctx, base);
	// 8324E854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E858: 38698CB8  addi r3, r9, -0x7348
	ctx.r[3].s64 = ctx.r[9].s64 + -29512;
	// 8324E85C: 4BA5B6C5  bl 0x82ca9f20
	ctx.lr = 0x8324E860;
	sub_82CA9F20(ctx, base);
	// 8324E860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E870 size=56
    let mut pc: u32 = 0x8324E870;
    'dispatch: loop {
        match pc {
            0x8324E870 => {
    //   block [0x8324E870..0x8324E8A8)
	// 8324E870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E87C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E884: 386BEEF0  addi r3, r11, -0x1110
	ctx.r[3].s64 = ctx.r[11].s64 + -4368;
	// 8324E888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E88C: 4AFA54CD  bl 0x821f3d58
	ctx.lr = 0x8324E890;
	sub_821F3D58(ctx, base);
	// 8324E890: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E894: 906A7D1C  stw r3, 0x7d1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32028 as u32), ctx.r[3].u32 ) };
	// 8324E898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E8A8 size=56
    let mut pc: u32 = 0x8324E8A8;
    'dispatch: loop {
        match pc {
            0x8324E8A8 => {
    //   block [0x8324E8A8..0x8324E8E0)
	// 8324E8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E8B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E8B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E8BC: 386BEF00  addi r3, r11, -0x1100
	ctx.r[3].s64 = ctx.r[11].s64 + -4352;
	// 8324E8C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E8C4: 4AFA5495  bl 0x821f3d58
	ctx.lr = 0x8324E8C8;
	sub_821F3D58(ctx, base);
	// 8324E8C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E8CC: 906A7D20  stw r3, 0x7d20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32032 as u32), ctx.r[3].u32 ) };
	// 8324E8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E8E0 size=56
    let mut pc: u32 = 0x8324E8E0;
    'dispatch: loop {
        match pc {
            0x8324E8E0 => {
    //   block [0x8324E8E0..0x8324E918)
	// 8324E8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E8EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E8F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E8F4: 386BEF14  addi r3, r11, -0x10ec
	ctx.r[3].s64 = ctx.r[11].s64 + -4332;
	// 8324E8F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E8FC: 4AFA545D  bl 0x821f3d58
	ctx.lr = 0x8324E900;
	sub_821F3D58(ctx, base);
	// 8324E900: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E904: 906A7D24  stw r3, 0x7d24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32036 as u32), ctx.r[3].u32 ) };
	// 8324E908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E918 size=56
    let mut pc: u32 = 0x8324E918;
    'dispatch: loop {
        match pc {
            0x8324E918 => {
    //   block [0x8324E918..0x8324E950)
	// 8324E918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E924: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E92C: 386BEF28  addi r3, r11, -0x10d8
	ctx.r[3].s64 = ctx.r[11].s64 + -4312;
	// 8324E930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E934: 4AFA5425  bl 0x821f3d58
	ctx.lr = 0x8324E938;
	sub_821F3D58(ctx, base);
	// 8324E938: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E93C: 906A7D28  stw r3, 0x7d28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32040 as u32), ctx.r[3].u32 ) };
	// 8324E940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324E950 size=12
    let mut pc: u32 = 0x8324E950;
    'dispatch: loop {
        match pc {
            0x8324E950 => {
    //   block [0x8324E950..0x8324E95C)
	// 8324E950: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324E954: 386B8CC8  addi r3, r11, -0x7338
	ctx.r[3].s64 = ctx.r[11].s64 + -29496;
	// 8324E958: 4BA5B5C8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324E960 size=12
    let mut pc: u32 = 0x8324E960;
    'dispatch: loop {
        match pc {
            0x8324E960 => {
    //   block [0x8324E960..0x8324E96C)
	// 8324E960: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324E964: 386B8D30  addi r3, r11, -0x72d0
	ctx.r[3].s64 = ctx.r[11].s64 + -29392;
	// 8324E968: 4BA5B5B8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E970 size=64
    let mut pc: u32 = 0x8324E970;
    'dispatch: loop {
        match pc {
            0x8324E970 => {
    //   block [0x8324E970..0x8324E9B0)
	// 8324E970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E97C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E980: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E984: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E988: 386A7D4C  addi r3, r10, 0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32076;
	// 8324E98C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E990: 4AFDE541  bl 0x8222ced0
	ctx.lr = 0x8324E994;
	sub_8222CED0(ctx, base);
	// 8324E994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E998: 38698D98  addi r3, r9, -0x7268
	ctx.r[3].s64 = ctx.r[9].s64 + -29288;
	// 8324E99C: 4BA5B585  bl 0x82ca9f20
	ctx.lr = 0x8324E9A0;
	sub_82CA9F20(ctx, base);
	// 8324E9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E9B0 size=64
    let mut pc: u32 = 0x8324E9B0;
    'dispatch: loop {
        match pc {
            0x8324E9B0 => {
    //   block [0x8324E9B0..0x8324E9F0)
	// 8324E9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E9BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E9C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E9C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E9C8: 386A7D50  addi r3, r10, 0x7d50
	ctx.r[3].s64 = ctx.r[10].s64 + 32080;
	// 8324E9CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E9D0: 4AFDE501  bl 0x8222ced0
	ctx.lr = 0x8324E9D4;
	sub_8222CED0(ctx, base);
	// 8324E9D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E9D8: 38698DA8  addi r3, r9, -0x7258
	ctx.r[3].s64 = ctx.r[9].s64 + -29272;
	// 8324E9DC: 4BA5B545  bl 0x82ca9f20
	ctx.lr = 0x8324E9E0;
	sub_82CA9F20(ctx, base);
	// 8324E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E9F0 size=64
    let mut pc: u32 = 0x8324E9F0;
    'dispatch: loop {
        match pc {
            0x8324E9F0 => {
    //   block [0x8324E9F0..0x8324EA30)
	// 8324E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E9FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EA00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EA04: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324EA08: 386A7D54  addi r3, r10, 0x7d54
	ctx.r[3].s64 = ctx.r[10].s64 + 32084;
	// 8324EA0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EA10: 4AFDE4C1  bl 0x8222ced0
	ctx.lr = 0x8324EA14;
	sub_8222CED0(ctx, base);
	// 8324EA14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EA18: 38698DB8  addi r3, r9, -0x7248
	ctx.r[3].s64 = ctx.r[9].s64 + -29256;
	// 8324EA1C: 4BA5B505  bl 0x82ca9f20
	ctx.lr = 0x8324EA20;
	sub_82CA9F20(ctx, base);
	// 8324EA20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EA30 size=64
    let mut pc: u32 = 0x8324EA30;
    'dispatch: loop {
        match pc {
            0x8324EA30 => {
    //   block [0x8324EA30..0x8324EA70)
	// 8324EA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EA3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EA40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EA44: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324EA48: 386A7D58  addi r3, r10, 0x7d58
	ctx.r[3].s64 = ctx.r[10].s64 + 32088;
	// 8324EA4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EA50: 4AFDE481  bl 0x8222ced0
	ctx.lr = 0x8324EA54;
	sub_8222CED0(ctx, base);
	// 8324EA54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EA58: 38698DC8  addi r3, r9, -0x7238
	ctx.r[3].s64 = ctx.r[9].s64 + -29240;
	// 8324EA5C: 4BA5B4C5  bl 0x82ca9f20
	ctx.lr = 0x8324EA60;
	sub_82CA9F20(ctx, base);
	// 8324EA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EA70 size=64
    let mut pc: u32 = 0x8324EA70;
    'dispatch: loop {
        match pc {
            0x8324EA70 => {
    //   block [0x8324EA70..0x8324EAB0)
	// 8324EA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EA78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EA7C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EA80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EA84: 388BF070  addi r4, r11, -0xf90
	ctx.r[4].s64 = ctx.r[11].s64 + -3984;
	// 8324EA88: 386A7D5C  addi r3, r10, 0x7d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32092;
	// 8324EA8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EA90: 4AFDE441  bl 0x8222ced0
	ctx.lr = 0x8324EA94;
	sub_8222CED0(ctx, base);
	// 8324EA94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EA98: 38698DD8  addi r3, r9, -0x7228
	ctx.r[3].s64 = ctx.r[9].s64 + -29224;
	// 8324EA9C: 4BA5B485  bl 0x82ca9f20
	ctx.lr = 0x8324EAA0;
	sub_82CA9F20(ctx, base);
	// 8324EAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EAB0 size=64
    let mut pc: u32 = 0x8324EAB0;
    'dispatch: loop {
        match pc {
            0x8324EAB0 => {
    //   block [0x8324EAB0..0x8324EAF0)
	// 8324EAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EABC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EAC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EAC4: 388BF088  addi r4, r11, -0xf78
	ctx.r[4].s64 = ctx.r[11].s64 + -3960;
	// 8324EAC8: 386A7D60  addi r3, r10, 0x7d60
	ctx.r[3].s64 = ctx.r[10].s64 + 32096;
	// 8324EACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EAD0: 4AFDE401  bl 0x8222ced0
	ctx.lr = 0x8324EAD4;
	sub_8222CED0(ctx, base);
	// 8324EAD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EAD8: 38698DE8  addi r3, r9, -0x7218
	ctx.r[3].s64 = ctx.r[9].s64 + -29208;
	// 8324EADC: 4BA5B445  bl 0x82ca9f20
	ctx.lr = 0x8324EAE0;
	sub_82CA9F20(ctx, base);
	// 8324EAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EAF0 size=64
    let mut pc: u32 = 0x8324EAF0;
    'dispatch: loop {
        match pc {
            0x8324EAF0 => {
    //   block [0x8324EAF0..0x8324EB30)
	// 8324EAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EAFC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EB00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EB04: 388BF0A8  addi r4, r11, -0xf58
	ctx.r[4].s64 = ctx.r[11].s64 + -3928;
	// 8324EB08: 386A7D64  addi r3, r10, 0x7d64
	ctx.r[3].s64 = ctx.r[10].s64 + 32100;
	// 8324EB0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EB10: 4AFDE3C1  bl 0x8222ced0
	ctx.lr = 0x8324EB14;
	sub_8222CED0(ctx, base);
	// 8324EB14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EB18: 38698DF8  addi r3, r9, -0x7208
	ctx.r[3].s64 = ctx.r[9].s64 + -29192;
	// 8324EB1C: 4BA5B405  bl 0x82ca9f20
	ctx.lr = 0x8324EB20;
	sub_82CA9F20(ctx, base);
	// 8324EB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EB30 size=144
    let mut pc: u32 = 0x8324EB30;
    'dispatch: loop {
        match pc {
            0x8324EB30 => {
    //   block [0x8324EB30..0x8324EBC0)
	// 8324EB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EB3C: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 8324EB40: 4AFD0719  bl 0x8221f258
	ctx.lr = 0x8324EB44;
	sub_8221F258(ctx, base);
	// 8324EB44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324EB48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8324EB4C: 419A0008  beq cr6, 0x8324eb54
	if ctx.cr[6].eq {
	pc = 0x8324EB54; continue 'dispatch;
	}
	// 8324EB50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8324EB54: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324EB58: 41820008  beq 0x8324eb60
	if ctx.cr[0].eq {
	pc = 0x8324EB60; continue 'dispatch;
	}
	// 8324EB5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8324EB60: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324EB64: 41820008  beq 0x8324eb6c
	if ctx.cr[0].eq {
	pc = 0x8324EB6C; continue 'dispatch;
	}
	// 8324EB68: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8324EB6C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324EB70: 99430031  stb r10, 0x31(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(49 as u32), ctx.r[10].u8 ) };
	// 8324EB74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8324EB78: 39097D68  addi r8, r9, 0x7d68
	ctx.r[8].s64 = ctx.r[9].s64 + 32104;
	// 8324EB7C: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 8324EB80: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324EB84: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8324EB88: 99630031  stb r11, 0x31(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(49 as u32), ctx.r[11].u8 ) };
	// 8324EB8C: 38678E08  addi r3, r7, -0x71f8
	ctx.r[3].s64 = ctx.r[7].s64 + -29176;
	// 8324EB90: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324EB94: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324EB98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324EB9C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324EBA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324EBA4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324EBA8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324EBAC: 4BA5B375  bl 0x82ca9f20
	ctx.lr = 0x8324EBB0;
	sub_82CA9F20(ctx, base);
	// 8324EBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EBC0 size=64
    let mut pc: u32 = 0x8324EBC0;
    'dispatch: loop {
        match pc {
            0x8324EBC0 => {
    //   block [0x8324EBC0..0x8324EC00)
	// 8324EBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EBCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EBD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EBD4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324EBD8: 386A7D74  addi r3, r10, 0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + 32116;
	// 8324EBDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EBE0: 4AFDE2F1  bl 0x8222ced0
	ctx.lr = 0x8324EBE4;
	sub_8222CED0(ctx, base);
	// 8324EBE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EBE8: 38698E18  addi r3, r9, -0x71e8
	ctx.r[3].s64 = ctx.r[9].s64 + -29160;
	// 8324EBEC: 4BA5B335  bl 0x82ca9f20
	ctx.lr = 0x8324EBF0;
	sub_82CA9F20(ctx, base);
	// 8324EBF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EC00 size=64
    let mut pc: u32 = 0x8324EC00;
    'dispatch: loop {
        match pc {
            0x8324EC00 => {
    //   block [0x8324EC00..0x8324EC40)
	// 8324EC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EC0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EC10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EC14: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324EC18: 386A7D78  addi r3, r10, 0x7d78
	ctx.r[3].s64 = ctx.r[10].s64 + 32120;
	// 8324EC1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EC20: 4AFDE2B1  bl 0x8222ced0
	ctx.lr = 0x8324EC24;
	sub_8222CED0(ctx, base);
	// 8324EC24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EC28: 38698E28  addi r3, r9, -0x71d8
	ctx.r[3].s64 = ctx.r[9].s64 + -29144;
	// 8324EC2C: 4BA5B2F5  bl 0x82ca9f20
	ctx.lr = 0x8324EC30;
	sub_82CA9F20(ctx, base);
	// 8324EC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EC40 size=64
    let mut pc: u32 = 0x8324EC40;
    'dispatch: loop {
        match pc {
            0x8324EC40 => {
    //   block [0x8324EC40..0x8324EC80)
	// 8324EC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EC50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EC54: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324EC58: 386A7D7C  addi r3, r10, 0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 32124;
	// 8324EC5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EC60: 4AFDE271  bl 0x8222ced0
	ctx.lr = 0x8324EC64;
	sub_8222CED0(ctx, base);
	// 8324EC64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EC68: 38698E38  addi r3, r9, -0x71c8
	ctx.r[3].s64 = ctx.r[9].s64 + -29128;
	// 8324EC6C: 4BA5B2B5  bl 0x82ca9f20
	ctx.lr = 0x8324EC70;
	sub_82CA9F20(ctx, base);
	// 8324EC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EC80 size=64
    let mut pc: u32 = 0x8324EC80;
    'dispatch: loop {
        match pc {
            0x8324EC80 => {
    //   block [0x8324EC80..0x8324ECC0)
	// 8324EC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EC8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EC90: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EC94: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324EC98: 386A7D80  addi r3, r10, 0x7d80
	ctx.r[3].s64 = ctx.r[10].s64 + 32128;
	// 8324EC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ECA0: 4AFDE231  bl 0x8222ced0
	ctx.lr = 0x8324ECA4;
	sub_8222CED0(ctx, base);
	// 8324ECA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ECA8: 38698E48  addi r3, r9, -0x71b8
	ctx.r[3].s64 = ctx.r[9].s64 + -29112;
	// 8324ECAC: 4BA5B275  bl 0x82ca9f20
	ctx.lr = 0x8324ECB0;
	sub_82CA9F20(ctx, base);
	// 8324ECB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ECB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ECB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ECBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ECC0 size=64
    let mut pc: u32 = 0x8324ECC0;
    'dispatch: loop {
        match pc {
            0x8324ECC0 => {
    //   block [0x8324ECC0..0x8324ED00)
	// 8324ECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ECC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ECC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ECCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ECD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ECD4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324ECD8: 386A7D84  addi r3, r10, 0x7d84
	ctx.r[3].s64 = ctx.r[10].s64 + 32132;
	// 8324ECDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ECE0: 4AFDE1F1  bl 0x8222ced0
	ctx.lr = 0x8324ECE4;
	sub_8222CED0(ctx, base);
	// 8324ECE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ECE8: 38698E58  addi r3, r9, -0x71a8
	ctx.r[3].s64 = ctx.r[9].s64 + -29096;
	// 8324ECEC: 4BA5B235  bl 0x82ca9f20
	ctx.lr = 0x8324ECF0;
	sub_82CA9F20(ctx, base);
	// 8324ECF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ECF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ECF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ECFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ED00 size=64
    let mut pc: u32 = 0x8324ED00;
    'dispatch: loop {
        match pc {
            0x8324ED00 => {
    //   block [0x8324ED00..0x8324ED40)
	// 8324ED00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ED04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ED08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ED0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ED10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ED14: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324ED18: 386A7D88  addi r3, r10, 0x7d88
	ctx.r[3].s64 = ctx.r[10].s64 + 32136;
	// 8324ED1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ED20: 4AFDE1B1  bl 0x8222ced0
	ctx.lr = 0x8324ED24;
	sub_8222CED0(ctx, base);
	// 8324ED24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ED28: 38698E68  addi r3, r9, -0x7198
	ctx.r[3].s64 = ctx.r[9].s64 + -29080;
	// 8324ED2C: 4BA5B1F5  bl 0x82ca9f20
	ctx.lr = 0x8324ED30;
	sub_82CA9F20(ctx, base);
	// 8324ED30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ED34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ED38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ED3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ED40 size=64
    let mut pc: u32 = 0x8324ED40;
    'dispatch: loop {
        match pc {
            0x8324ED40 => {
    //   block [0x8324ED40..0x8324ED80)
	// 8324ED40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ED44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ED48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ED4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ED50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ED54: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324ED58: 386A7D8C  addi r3, r10, 0x7d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 32140;
	// 8324ED5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ED60: 4AFDE171  bl 0x8222ced0
	ctx.lr = 0x8324ED64;
	sub_8222CED0(ctx, base);
	// 8324ED64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ED68: 38698E78  addi r3, r9, -0x7188
	ctx.r[3].s64 = ctx.r[9].s64 + -29064;
	// 8324ED6C: 4BA5B1B5  bl 0x82ca9f20
	ctx.lr = 0x8324ED70;
	sub_82CA9F20(ctx, base);
	// 8324ED70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ED74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ED78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ED7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ED80 size=64
    let mut pc: u32 = 0x8324ED80;
    'dispatch: loop {
        match pc {
            0x8324ED80 => {
    //   block [0x8324ED80..0x8324EDC0)
	// 8324ED80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ED84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ED88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ED8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ED90: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ED94: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324ED98: 386A7D90  addi r3, r10, 0x7d90
	ctx.r[3].s64 = ctx.r[10].s64 + 32144;
	// 8324ED9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EDA0: 4AFDE131  bl 0x8222ced0
	ctx.lr = 0x8324EDA4;
	sub_8222CED0(ctx, base);
	// 8324EDA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EDA8: 38698E88  addi r3, r9, -0x7178
	ctx.r[3].s64 = ctx.r[9].s64 + -29048;
	// 8324EDAC: 4BA5B175  bl 0x82ca9f20
	ctx.lr = 0x8324EDB0;
	sub_82CA9F20(ctx, base);
	// 8324EDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EDC0 size=64
    let mut pc: u32 = 0x8324EDC0;
    'dispatch: loop {
        match pc {
            0x8324EDC0 => {
    //   block [0x8324EDC0..0x8324EE00)
	// 8324EDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EDCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EDD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EDD4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324EDD8: 386A7D94  addi r3, r10, 0x7d94
	ctx.r[3].s64 = ctx.r[10].s64 + 32148;
	// 8324EDDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EDE0: 4AFDE0F1  bl 0x8222ced0
	ctx.lr = 0x8324EDE4;
	sub_8222CED0(ctx, base);
	// 8324EDE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EDE8: 38698E98  addi r3, r9, -0x7168
	ctx.r[3].s64 = ctx.r[9].s64 + -29032;
	// 8324EDEC: 4BA5B135  bl 0x82ca9f20
	ctx.lr = 0x8324EDF0;
	sub_82CA9F20(ctx, base);
	// 8324EDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EE00 size=56
    let mut pc: u32 = 0x8324EE00;
    'dispatch: loop {
        match pc {
            0x8324EE00 => {
    //   block [0x8324EE00..0x8324EE38)
	// 8324EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EE0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EE14: 386B0638  addi r3, r11, 0x638
	ctx.r[3].s64 = ctx.r[11].s64 + 1592;
	// 8324EE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EE1C: 4AFA4F3D  bl 0x821f3d58
	ctx.lr = 0x8324EE20;
	sub_821F3D58(ctx, base);
	// 8324EE20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EE24: 906A7D98  stw r3, 0x7d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32152 as u32), ctx.r[3].u32 ) };
	// 8324EE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EE38 size=56
    let mut pc: u32 = 0x8324EE38;
    'dispatch: loop {
        match pc {
            0x8324EE38 => {
    //   block [0x8324EE38..0x8324EE70)
	// 8324EE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EE44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EE48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EE4C: 386B0648  addi r3, r11, 0x648
	ctx.r[3].s64 = ctx.r[11].s64 + 1608;
	// 8324EE50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EE54: 4AFA4F05  bl 0x821f3d58
	ctx.lr = 0x8324EE58;
	sub_821F3D58(ctx, base);
	// 8324EE58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EE5C: 906A7D9C  stw r3, 0x7d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32156 as u32), ctx.r[3].u32 ) };
	// 8324EE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EE70 size=56
    let mut pc: u32 = 0x8324EE70;
    'dispatch: loop {
        match pc {
            0x8324EE70 => {
    //   block [0x8324EE70..0x8324EEA8)
	// 8324EE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EE7C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EE80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EE84: 386B065C  addi r3, r11, 0x65c
	ctx.r[3].s64 = ctx.r[11].s64 + 1628;
	// 8324EE88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EE8C: 4AFA4ECD  bl 0x821f3d58
	ctx.lr = 0x8324EE90;
	sub_821F3D58(ctx, base);
	// 8324EE90: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EE94: 906A7DA0  stw r3, 0x7da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32160 as u32), ctx.r[3].u32 ) };
	// 8324EE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EEA8 size=56
    let mut pc: u32 = 0x8324EEA8;
    'dispatch: loop {
        match pc {
            0x8324EEA8 => {
    //   block [0x8324EEA8..0x8324EEE0)
	// 8324EEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EEB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EEB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EEBC: 386B0670  addi r3, r11, 0x670
	ctx.r[3].s64 = ctx.r[11].s64 + 1648;
	// 8324EEC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EEC4: 4AFA4E95  bl 0x821f3d58
	ctx.lr = 0x8324EEC8;
	sub_821F3D58(ctx, base);
	// 8324EEC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EECC: 906A7DA4  stw r3, 0x7da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32164 as u32), ctx.r[3].u32 ) };
	// 8324EED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EEE0 size=56
    let mut pc: u32 = 0x8324EEE0;
    'dispatch: loop {
        match pc {
            0x8324EEE0 => {
    //   block [0x8324EEE0..0x8324EF18)
	// 8324EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EEE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EEEC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EEF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EEF4: 386B0688  addi r3, r11, 0x688
	ctx.r[3].s64 = ctx.r[11].s64 + 1672;
	// 8324EEF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EEFC: 4AFA4E5D  bl 0x821f3d58
	ctx.lr = 0x8324EF00;
	sub_821F3D58(ctx, base);
	// 8324EF00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF04: 906A7DA8  stw r3, 0x7da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32168 as u32), ctx.r[3].u32 ) };
	// 8324EF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EF18 size=56
    let mut pc: u32 = 0x8324EF18;
    'dispatch: loop {
        match pc {
            0x8324EF18 => {
    //   block [0x8324EF18..0x8324EF50)
	// 8324EF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EF24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EF28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EF2C: 386B0698  addi r3, r11, 0x698
	ctx.r[3].s64 = ctx.r[11].s64 + 1688;
	// 8324EF30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EF34: 4AFA4E25  bl 0x821f3d58
	ctx.lr = 0x8324EF38;
	sub_821F3D58(ctx, base);
	// 8324EF38: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF3C: 906A7DAC  stw r3, 0x7dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32172 as u32), ctx.r[3].u32 ) };
	// 8324EF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EF50 size=56
    let mut pc: u32 = 0x8324EF50;
    'dispatch: loop {
        match pc {
            0x8324EF50 => {
    //   block [0x8324EF50..0x8324EF88)
	// 8324EF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EF5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EF60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EF64: 386B06A8  addi r3, r11, 0x6a8
	ctx.r[3].s64 = ctx.r[11].s64 + 1704;
	// 8324EF68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EF6C: 4AFA4DED  bl 0x821f3d58
	ctx.lr = 0x8324EF70;
	sub_821F3D58(ctx, base);
	// 8324EF70: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF74: 906A7DB0  stw r3, 0x7db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32176 as u32), ctx.r[3].u32 ) };
	// 8324EF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EF88 size=64
    let mut pc: u32 = 0x8324EF88;
    'dispatch: loop {
        match pc {
            0x8324EF88 => {
    //   block [0x8324EF88..0x8324EFC8)
	// 8324EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EF94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EF98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF9C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324EFA0: 386A7DB4  addi r3, r10, 0x7db4
	ctx.r[3].s64 = ctx.r[10].s64 + 32180;
	// 8324EFA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EFA8: 4AFDDF29  bl 0x8222ced0
	ctx.lr = 0x8324EFAC;
	sub_8222CED0(ctx, base);
	// 8324EFAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EFB0: 38698EA8  addi r3, r9, -0x7158
	ctx.r[3].s64 = ctx.r[9].s64 + -29016;
	// 8324EFB4: 4BA5AF6D  bl 0x82ca9f20
	ctx.lr = 0x8324EFB8;
	sub_82CA9F20(ctx, base);
	// 8324EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EFC8 size=64
    let mut pc: u32 = 0x8324EFC8;
    'dispatch: loop {
        match pc {
            0x8324EFC8 => {
    //   block [0x8324EFC8..0x8324F008)
	// 8324EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EFD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EFD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EFDC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324EFE0: 386A7DB8  addi r3, r10, 0x7db8
	ctx.r[3].s64 = ctx.r[10].s64 + 32184;
	// 8324EFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EFE8: 4AFDDEE9  bl 0x8222ced0
	ctx.lr = 0x8324EFEC;
	sub_8222CED0(ctx, base);
	// 8324EFEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EFF0: 38698EB8  addi r3, r9, -0x7148
	ctx.r[3].s64 = ctx.r[9].s64 + -29000;
	// 8324EFF4: 4BA5AF2D  bl 0x82ca9f20
	ctx.lr = 0x8324EFF8;
	sub_82CA9F20(ctx, base);
	// 8324EFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F008 size=64
    let mut pc: u32 = 0x8324F008;
    'dispatch: loop {
        match pc {
            0x8324F008 => {
    //   block [0x8324F008..0x8324F048)
	// 8324F008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F014: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324F018: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F01C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324F020: 386A7DBC  addi r3, r10, 0x7dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 32188;
	// 8324F024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F028: 4AFDDEA9  bl 0x8222ced0
	ctx.lr = 0x8324F02C;
	sub_8222CED0(ctx, base);
	// 8324F02C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F030: 38698EC8  addi r3, r9, -0x7138
	ctx.r[3].s64 = ctx.r[9].s64 + -28984;
	// 8324F034: 4BA5AEED  bl 0x82ca9f20
	ctx.lr = 0x8324F038;
	sub_82CA9F20(ctx, base);
	// 8324F038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F048 size=64
    let mut pc: u32 = 0x8324F048;
    'dispatch: loop {
        match pc {
            0x8324F048 => {
    //   block [0x8324F048..0x8324F088)
	// 8324F048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F054: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F058: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F05C: 388B06D4  addi r4, r11, 0x6d4
	ctx.r[4].s64 = ctx.r[11].s64 + 1748;
	// 8324F060: 386A7DC0  addi r3, r10, 0x7dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 32192;
	// 8324F064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F068: 4AFDDE69  bl 0x8222ced0
	ctx.lr = 0x8324F06C;
	sub_8222CED0(ctx, base);
	// 8324F06C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F070: 38698ED8  addi r3, r9, -0x7128
	ctx.r[3].s64 = ctx.r[9].s64 + -28968;
	// 8324F074: 4BA5AEAD  bl 0x82ca9f20
	ctx.lr = 0x8324F078;
	sub_82CA9F20(ctx, base);
	// 8324F078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F088 size=64
    let mut pc: u32 = 0x8324F088;
    'dispatch: loop {
        match pc {
            0x8324F088 => {
    //   block [0x8324F088..0x8324F0C8)
	// 8324F088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F094: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F098: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F09C: 388B06D8  addi r4, r11, 0x6d8
	ctx.r[4].s64 = ctx.r[11].s64 + 1752;
	// 8324F0A0: 386A7DC4  addi r3, r10, 0x7dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 32196;
	// 8324F0A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F0A8: 4AFDDE29  bl 0x8222ced0
	ctx.lr = 0x8324F0AC;
	sub_8222CED0(ctx, base);
	// 8324F0AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F0B0: 38698EE8  addi r3, r9, -0x7118
	ctx.r[3].s64 = ctx.r[9].s64 + -28952;
	// 8324F0B4: 4BA5AE6D  bl 0x82ca9f20
	ctx.lr = 0x8324F0B8;
	sub_82CA9F20(ctx, base);
	// 8324F0B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F0C8 size=64
    let mut pc: u32 = 0x8324F0C8;
    'dispatch: loop {
        match pc {
            0x8324F0C8 => {
    //   block [0x8324F0C8..0x8324F108)
	// 8324F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F0D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F0D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F0D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F0DC: 388B06DC  addi r4, r11, 0x6dc
	ctx.r[4].s64 = ctx.r[11].s64 + 1756;
	// 8324F0E0: 386A7DD0  addi r3, r10, 0x7dd0
	ctx.r[3].s64 = ctx.r[10].s64 + 32208;
	// 8324F0E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F0E8: 4AFDDDE9  bl 0x8222ced0
	ctx.lr = 0x8324F0EC;
	sub_8222CED0(ctx, base);
	// 8324F0EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F0F0: 38698EF8  addi r3, r9, -0x7108
	ctx.r[3].s64 = ctx.r[9].s64 + -28936;
	// 8324F0F4: 4BA5AE2D  bl 0x82ca9f20
	ctx.lr = 0x8324F0F8;
	sub_82CA9F20(ctx, base);
	// 8324F0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F108 size=64
    let mut pc: u32 = 0x8324F108;
    'dispatch: loop {
        match pc {
            0x8324F108 => {
    //   block [0x8324F108..0x8324F148)
	// 8324F108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F114: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F118: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F11C: 388B06F4  addi r4, r11, 0x6f4
	ctx.r[4].s64 = ctx.r[11].s64 + 1780;
	// 8324F120: 386A7DD4  addi r3, r10, 0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32212;
	// 8324F124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F128: 4AFDDDA9  bl 0x8222ced0
	ctx.lr = 0x8324F12C;
	sub_8222CED0(ctx, base);
	// 8324F12C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F130: 38698F08  addi r3, r9, -0x70f8
	ctx.r[3].s64 = ctx.r[9].s64 + -28920;
	// 8324F134: 4BA5ADED  bl 0x82ca9f20
	ctx.lr = 0x8324F138;
	sub_82CA9F20(ctx, base);
	// 8324F138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F148 size=64
    let mut pc: u32 = 0x8324F148;
    'dispatch: loop {
        match pc {
            0x8324F148 => {
    //   block [0x8324F148..0x8324F188)
	// 8324F148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F154: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F158: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F15C: 388B070C  addi r4, r11, 0x70c
	ctx.r[4].s64 = ctx.r[11].s64 + 1804;
	// 8324F160: 386A7DD8  addi r3, r10, 0x7dd8
	ctx.r[3].s64 = ctx.r[10].s64 + 32216;
	// 8324F164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F168: 4AFDDD69  bl 0x8222ced0
	ctx.lr = 0x8324F16C;
	sub_8222CED0(ctx, base);
	// 8324F16C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F170: 38698F18  addi r3, r9, -0x70e8
	ctx.r[3].s64 = ctx.r[9].s64 + -28904;
	// 8324F174: 4BA5ADAD  bl 0x82ca9f20
	ctx.lr = 0x8324F178;
	sub_82CA9F20(ctx, base);
	// 8324F178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F188 size=64
    let mut pc: u32 = 0x8324F188;
    'dispatch: loop {
        match pc {
            0x8324F188 => {
    //   block [0x8324F188..0x8324F1C8)
	// 8324F188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F194: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F198: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F19C: 388B0724  addi r4, r11, 0x724
	ctx.r[4].s64 = ctx.r[11].s64 + 1828;
	// 8324F1A0: 386A7DDC  addi r3, r10, 0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 32220;
	// 8324F1A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F1A8: 4AFDDD29  bl 0x8222ced0
	ctx.lr = 0x8324F1AC;
	sub_8222CED0(ctx, base);
	// 8324F1AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F1B0: 38698F28  addi r3, r9, -0x70d8
	ctx.r[3].s64 = ctx.r[9].s64 + -28888;
	// 8324F1B4: 4BA5AD6D  bl 0x82ca9f20
	ctx.lr = 0x8324F1B8;
	sub_82CA9F20(ctx, base);
	// 8324F1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F1C8 size=64
    let mut pc: u32 = 0x8324F1C8;
    'dispatch: loop {
        match pc {
            0x8324F1C8 => {
    //   block [0x8324F1C8..0x8324F208)
	// 8324F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F1D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F1D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F1DC: 388B073C  addi r4, r11, 0x73c
	ctx.r[4].s64 = ctx.r[11].s64 + 1852;
	// 8324F1E0: 386A7DE0  addi r3, r10, 0x7de0
	ctx.r[3].s64 = ctx.r[10].s64 + 32224;
	// 8324F1E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F1E8: 4AFDDCE9  bl 0x8222ced0
	ctx.lr = 0x8324F1EC;
	sub_8222CED0(ctx, base);
	// 8324F1EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F1F0: 38698F38  addi r3, r9, -0x70c8
	ctx.r[3].s64 = ctx.r[9].s64 + -28872;
	// 8324F1F4: 4BA5AD2D  bl 0x82ca9f20
	ctx.lr = 0x8324F1F8;
	sub_82CA9F20(ctx, base);
	// 8324F1F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


