pub fn sub_826B3340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3340 size=116
    let mut pc: u32 = 0x826B3340;
    'dispatch: loop {
        match pc {
            0x826B3340 => {
    //   block [0x826B3340..0x826B33B4)
	// 826B3340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B334C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B3350: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3354: 392BF474  addi r9, r11, -0xb8c
	ctx.r[9].s64 = ctx.r[11].s64 + -2956;
	// 826B3358: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B335C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3360: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B3364: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826B3368: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B336C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826B3370: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3374: 396B81B8  addi r11, r11, -0x7e48
	ctx.r[11].s64 = ctx.r[11].s64 + -32328;
	// 826B3378: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B337C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3380: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B3384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3388: 386ADC44  addi r3, r10, -0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9148;
	// 826B338C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B3390: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B3394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3398: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B339C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B33A0: 4BDB3A81  bl 0x82466e20
	ctx.lr = 0x826B33A4;
	sub_82466E20(ctx, base);
	// 826B33A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B33A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B33AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B33B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B33B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B33B8 size=100
    let mut pc: u32 = 0x826B33B8;
    'dispatch: loop {
        match pc {
            0x826B33B8 => {
    //   block [0x826B33B8..0x826B341C)
	// 826B33B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B33BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B33C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B33C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B33C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B33CC: 38AADD94  addi r5, r10, -0x226c
	ctx.r[5].s64 = ctx.r[10].s64 + -8812;
	// 826B33D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B33D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B33D8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826B33DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B33E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B33E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B33E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B33EC: 386ADC74  addi r3, r10, -0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + -9100;
	// 826B33F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B33F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B33F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B33FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3400: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B3404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3408: 4BDB3A19  bl 0x82466e20
	ctx.lr = 0x826B340C;
	sub_82466E20(ctx, base);
	// 826B340C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3420 size=100
    let mut pc: u32 = 0x826B3420;
    'dispatch: loop {
        match pc {
            0x826B3420 => {
    //   block [0x826B3420..0x826B3484)
	// 826B3420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B342C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3434: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B3438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B343C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3440: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826B3444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B344C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3454: 386ADCA4  addi r3, r10, -0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + -9052;
	// 826B3458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B345C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3460: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B3464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B346C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3470: 4BDB39B1  bl 0x82466e20
	ctx.lr = 0x826B3474;
	sub_82466E20(ctx, base);
	// 826B3474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B347C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3488 size=108
    let mut pc: u32 = 0x826B3488;
    'dispatch: loop {
        match pc {
            0x826B3488 => {
    //   block [0x826B3488..0x826B34F4)
	// 826B3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3494: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B349C: 38EB8248  addi r7, r11, -0x7db8
	ctx.r[7].s64 = ctx.r[11].s64 + -32184;
	// 826B34A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B34A4: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826B34A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B34AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B34B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B34B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B34B8: 386ADCD4  addi r3, r10, -0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + -9004;
	// 826B34BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B34C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B34C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B34C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B34CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B34D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B34D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B34D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B34DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B34E0: 4BDB3941  bl 0x82466e20
	ctx.lr = 0x826B34E4;
	sub_82466E20(ctx, base);
	// 826B34E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B34E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B34EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B34F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B34F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B34F8 size=112
    let mut pc: u32 = 0x826B34F8;
    'dispatch: loop {
        match pc {
            0x826B34F8 => {
    //   block [0x826B34F8..0x826B3568)
	// 826B34F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B34FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B350C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3514: 390B8278  addi r8, r11, -0x7d88
	ctx.r[8].s64 = ctx.r[11].s64 + -32136;
	// 826B3518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B351C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826B3520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B352C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3530: 386ADD04  addi r3, r10, -0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8956;
	// 826B3534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B353C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B354C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3554: 4BDB38CD  bl 0x82466e20
	ctx.lr = 0x826B3558;
	sub_82466E20(ctx, base);
	// 826B3558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B355C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3568 size=108
    let mut pc: u32 = 0x826B3568;
    'dispatch: loop {
        match pc {
            0x826B3568 => {
    //   block [0x826B3568..0x826B35D4)
	// 826B3568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B356C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3574: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B357C: 38EB8290  addi r7, r11, -0x7d70
	ctx.r[7].s64 = ctx.r[11].s64 + -32112;
	// 826B3580: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B3584: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826B3588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B358C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3598: 386ADD34  addi r3, r10, -0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8908;
	// 826B359C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B35A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B35A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B35A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B35AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B35B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B35B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B35B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B35BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B35C0: 4BDB3861  bl 0x82466e20
	ctx.lr = 0x826B35C4;
	sub_82466E20(ctx, base);
	// 826B35C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B35C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B35CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B35D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B35D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B35D8 size=28
    let mut pc: u32 = 0x826B35D8;
    'dispatch: loop {
        match pc {
            0x826B35D8 => {
    //   block [0x826B35D8..0x826B35F4)
	// 826B35D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B35DC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B35E0: 394ABE08  addi r10, r10, -0x41f8
	ctx.r[10].s64 = ctx.r[10].s64 + -16888;
	// 826B35E4: 816B81B4  lwz r11, -0x7e4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32332 as u32) ) } as u64;
	// 826B35E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B35EC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826B35F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B35F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B35F8 size=108
    let mut pc: u32 = 0x826B35F8;
    'dispatch: loop {
        match pc {
            0x826B35F8 => {
    //   block [0x826B35F8..0x826B3664)
	// 826B35F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B35FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3604: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B360C: 38EBBE08  addi r7, r11, -0x41f8
	ctx.r[7].s64 = ctx.r[11].s64 + -16888;
	// 826B3610: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826B3614: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826B3618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B361C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3628: 386ADD64  addi r3, r10, -0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + -8860;
	// 826B362C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B363C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B364C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3650: 4BDB37D1  bl 0x82466e20
	ctx.lr = 0x826B3654;
	sub_82466E20(ctx, base);
	// 826B3654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B365C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3668 size=116
    let mut pc: u32 = 0x826B3668;
    'dispatch: loop {
        match pc {
            0x826B3668 => {
    //   block [0x826B3668..0x826B36DC)
	// 826B3668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3674: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3678: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B367C: 390B82B0  addi r8, r11, -0x7d50
	ctx.r[8].s64 = ctx.r[11].s64 + -32080;
	// 826B3680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3684: 392AF4E4  addi r9, r10, -0xb1c
	ctx.r[9].s64 = ctx.r[10].s64 + -2844;
	// 826B3688: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B368C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B3690: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3694: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B369C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B36A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B36A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B36A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B36AC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B36B0: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826B36B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B36B8: 386BDD94  addi r3, r11, -0x226c
	ctx.r[3].s64 = ctx.r[11].s64 + -8812;
	// 826B36BC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826B36C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B36C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B36C8: 4BDB3759  bl 0x82466e20
	ctx.lr = 0x826B36CC;
	sub_82466E20(ctx, base);
	// 826B36CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B36D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B36D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B36D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B36E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B36E0 size=112
    let mut pc: u32 = 0x826B36E0;
    'dispatch: loop {
        match pc {
            0x826B36E0 => {
    //   block [0x826B36E0..0x826B3750)
	// 826B36E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B36E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B36E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B36EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B36F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B36F4: 38AADA64  addi r5, r10, -0x259c
	ctx.r[5].s64 = ctx.r[10].s64 + -9628;
	// 826B36F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B36FC: 390B8328  addi r8, r11, -0x7cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -31960;
	// 826B3700: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B3704: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826B3708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B370C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3718: 386ADDC4  addi r3, r10, -0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + -8764;
	// 826B371C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B372C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B373C: 4BDB36E5  bl 0x82466e20
	ctx.lr = 0x826B3740;
	sub_82466E20(ctx, base);
	// 826B3740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B374C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3750 size=108
    let mut pc: u32 = 0x826B3750;
    'dispatch: loop {
        match pc {
            0x826B3750 => {
    //   block [0x826B3750..0x826B37BC)
	// 826B3750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B375C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3764: 38EB8340  addi r7, r11, -0x7cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -31936;
	// 826B3768: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B376C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826B3770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3778: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B377C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3780: 386ADDF4  addi r3, r10, -0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + -8716;
	// 826B3784: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B378C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B379C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B37A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B37A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B37A8: 4BDB3679  bl 0x82466e20
	ctx.lr = 0x826B37AC;
	sub_82466E20(ctx, base);
	// 826B37AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B37B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B37B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B37B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B37C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B37C0 size=112
    let mut pc: u32 = 0x826B37C0;
    'dispatch: loop {
        match pc {
            0x826B37C0 => {
    //   block [0x826B37C0..0x826B3830)
	// 826B37C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B37C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B37C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B37CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B37D0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B37D4: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B37D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B37DC: 390B8370  addi r8, r11, -0x7c90
	ctx.r[8].s64 = ctx.r[11].s64 + -31888;
	// 826B37E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B37E4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826B37E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B37EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B37F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B37F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B37F8: 386ADE24  addi r3, r10, -0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8668;
	// 826B37FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B380C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B381C: 4BDB3605  bl 0x82466e20
	ctx.lr = 0x826B3820;
	sub_82466E20(ctx, base);
	// 826B3820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B382C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3830 size=112
    let mut pc: u32 = 0x826B3830;
    'dispatch: loop {
        match pc {
            0x826B3830 => {
    //   block [0x826B3830..0x826B38A0)
	// 826B3830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B383C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3840: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3844: 38AADFA4  addi r5, r10, -0x205c
	ctx.r[5].s64 = ctx.r[10].s64 + -8284;
	// 826B3848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B384C: 390B83A0  addi r8, r11, -0x7c60
	ctx.r[8].s64 = ctx.r[11].s64 + -31840;
	// 826B3850: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3854: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826B3858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B385C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3868: 386ADE54  addi r3, r10, -0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8620;
	// 826B386C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B387C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B388C: 4BDB3595  bl 0x82466e20
	ctx.lr = 0x826B3890;
	sub_82466E20(ctx, base);
	// 826B3890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B389C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B38A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B38A0 size=100
    let mut pc: u32 = 0x826B38A0;
    'dispatch: loop {
        match pc {
            0x826B38A0 => {
    //   block [0x826B38A0..0x826B3904)
	// 826B38A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B38A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B38A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B38AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B38B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B38B4: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B38B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B38BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B38C0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826B38C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B38C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B38CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B38D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B38D4: 386ADE84  addi r3, r10, -0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + -8572;
	// 826B38D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B38DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B38E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B38E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B38E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B38EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B38F0: 4BDB3531  bl 0x82466e20
	ctx.lr = 0x826B38F4;
	sub_82466E20(ctx, base);
	// 826B38F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B38F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B38FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3908 size=112
    let mut pc: u32 = 0x826B3908;
    'dispatch: loop {
        match pc {
            0x826B3908 => {
    //   block [0x826B3908..0x826B3978)
	// 826B3908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B390C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3918: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B391C: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 826B3920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3924: 390B83D0  addi r8, r11, -0x7c30
	ctx.r[8].s64 = ctx.r[11].s64 + -31792;
	// 826B3928: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B392C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826B3930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B393C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3940: 386ADEB4  addi r3, r10, -0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + -8524;
	// 826B3944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B394C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B395C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3964: 4BDB34BD  bl 0x82466e20
	ctx.lr = 0x826B3968;
	sub_82466E20(ctx, base);
	// 826B3968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B396C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3978 size=112
    let mut pc: u32 = 0x826B3978;
    'dispatch: loop {
        match pc {
            0x826B3978 => {
    //   block [0x826B3978..0x826B39E8)
	// 826B3978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3988: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B398C: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 826B3990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3994: 390B8418  addi r8, r11, -0x7be8
	ctx.r[8].s64 = ctx.r[11].s64 + -31720;
	// 826B3998: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B399C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826B39A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B39A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B39A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B39AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B39B0: 386ADEE4  addi r3, r10, -0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + -8476;
	// 826B39B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B39B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B39BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B39C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B39C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B39C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B39CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B39D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B39D4: 4BDB344D  bl 0x82466e20
	ctx.lr = 0x826B39D8;
	sub_82466E20(ctx, base);
	// 826B39D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B39DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B39E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B39E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B39E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B39E8 size=108
    let mut pc: u32 = 0x826B39E8;
    'dispatch: loop {
        match pc {
            0x826B39E8 => {
    //   block [0x826B39E8..0x826B3A54)
	// 826B39E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B39EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B39F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B39F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B39F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B39FC: 38EB84C0  addi r7, r11, -0x7b40
	ctx.r[7].s64 = ctx.r[11].s64 + -31552;
	// 826B3A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B3A04: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826B3A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3A0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3A18: 386ADF14  addi r3, r10, -0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8428;
	// 826B3A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3A40: 4BDB33E1  bl 0x82466e20
	ctx.lr = 0x826B3A44;
	sub_82466E20(ctx, base);
	// 826B3A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3A58 size=112
    let mut pc: u32 = 0x826B3A58;
    'dispatch: loop {
        match pc {
            0x826B3A58 => {
    //   block [0x826B3A58..0x826B3AC8)
	// 826B3A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3A64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3A68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3A6C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3A74: 390B8508  addi r8, r11, -0x7af8
	ctx.r[8].s64 = ctx.r[11].s64 + -31480;
	// 826B3A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B3A7C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826B3A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3A90: 386ADF44  addi r3, r10, -0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8380;
	// 826B3A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3AB4: 4BDB336D  bl 0x82466e20
	ctx.lr = 0x826B3AB8;
	sub_82466E20(ctx, base);
	// 826B3AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3AC8 size=100
    let mut pc: u32 = 0x826B3AC8;
    'dispatch: loop {
        match pc {
            0x826B3AC8 => {
    //   block [0x826B3AC8..0x826B3B2C)
	// 826B3AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3AD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3ADC: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B3AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3AE8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826B3AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3AFC: 386ADF74  addi r3, r10, -0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + -8332;
	// 826B3B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3B08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B3B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3B10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B3B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3B18: 4BDB3309  bl 0x82466e20
	ctx.lr = 0x826B3B1C;
	sub_82466E20(ctx, base);
	// 826B3B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3B30 size=100
    let mut pc: u32 = 0x826B3B30;
    'dispatch: loop {
        match pc {
            0x826B3B30 => {
    //   block [0x826B3B30..0x826B3B94)
	// 826B3B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3B3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3B44: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B3B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3B50: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826B3B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3B64: 386ADFA4  addi r3, r10, -0x205c
	ctx.r[3].s64 = ctx.r[10].s64 + -8284;
	// 826B3B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3B70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B3B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B3B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3B80: 4BDB32A1  bl 0x82466e20
	ctx.lr = 0x826B3B84;
	sub_82466E20(ctx, base);
	// 826B3B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3B98 size=112
    let mut pc: u32 = 0x826B3B98;
    'dispatch: loop {
        match pc {
            0x826B3B98 => {
    //   block [0x826B3B98..0x826B3C08)
	// 826B3B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3BA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3BA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3BAC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B3BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3BB4: 390B8568  addi r8, r11, -0x7a98
	ctx.r[8].s64 = ctx.r[11].s64 + -31384;
	// 826B3BB8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B3BBC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826B3BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3BC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3BD0: 386ADFD4  addi r3, r10, -0x202c
	ctx.r[3].s64 = ctx.r[10].s64 + -8236;
	// 826B3BD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3BF4: 4BDB322D  bl 0x82466e20
	ctx.lr = 0x826B3BF8;
	sub_82466E20(ctx, base);
	// 826B3BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3C08 size=112
    let mut pc: u32 = 0x826B3C08;
    'dispatch: loop {
        match pc {
            0x826B3C08 => {
    //   block [0x826B3C08..0x826B3C78)
	// 826B3C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3C18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3C1C: 38AADD94  addi r5, r10, -0x226c
	ctx.r[5].s64 = ctx.r[10].s64 + -8812;
	// 826B3C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3C24: 390B85F8  addi r8, r11, -0x7a08
	ctx.r[8].s64 = ctx.r[11].s64 + -31240;
	// 826B3C28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B3C2C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826B3C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3C40: 386AE004  addi r3, r10, -0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -8188;
	// 826B3C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3C64: 4BDB31BD  bl 0x82466e20
	ctx.lr = 0x826B3C68;
	sub_82466E20(ctx, base);
	// 826B3C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3C78 size=112
    let mut pc: u32 = 0x826B3C78;
    'dispatch: loop {
        match pc {
            0x826B3C78 => {
    //   block [0x826B3C78..0x826B3CE8)
	// 826B3C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3C88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3C8C: 38AADEE4  addi r5, r10, -0x211c
	ctx.r[5].s64 = ctx.r[10].s64 + -8476;
	// 826B3C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3C94: 390B8610  addi r8, r11, -0x79f0
	ctx.r[8].s64 = ctx.r[11].s64 + -31216;
	// 826B3C98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3C9C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826B3CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3CB0: 386AE034  addi r3, r10, -0x1fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -8140;
	// 826B3CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3CD4: 4BDB314D  bl 0x82466e20
	ctx.lr = 0x826B3CD8;
	sub_82466E20(ctx, base);
	// 826B3CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3CE8 size=112
    let mut pc: u32 = 0x826B3CE8;
    'dispatch: loop {
        match pc {
            0x826B3CE8 => {
    //   block [0x826B3CE8..0x826B3D58)
	// 826B3CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3CF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3CFC: 38AAD944  addi r5, r10, -0x26bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9916;
	// 826B3D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3D04: 390B8640  addi r8, r11, -0x79c0
	ctx.r[8].s64 = ctx.r[11].s64 + -31168;
	// 826B3D08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B3D0C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826B3D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3D20: 386AE064  addi r3, r10, -0x1f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -8092;
	// 826B3D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3D44: 4BDB30DD  bl 0x82466e20
	ctx.lr = 0x826B3D48;
	sub_82466E20(ctx, base);
	// 826B3D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3D58 size=112
    let mut pc: u32 = 0x826B3D58;
    'dispatch: loop {
        match pc {
            0x826B3D58 => {
    //   block [0x826B3D58..0x826B3DC8)
	// 826B3D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3D64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3D68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3D6C: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 826B3D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3D74: 390B8688  addi r8, r11, -0x7978
	ctx.r[8].s64 = ctx.r[11].s64 + -31096;
	// 826B3D78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B3D7C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826B3D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3D84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3D90: 386AE094  addi r3, r10, -0x1f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -8044;
	// 826B3D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3DB4: 4BDB306D  bl 0x82466e20
	ctx.lr = 0x826B3DB8;
	sub_82466E20(ctx, base);
	// 826B3DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3DC8 size=112
    let mut pc: u32 = 0x826B3DC8;
    'dispatch: loop {
        match pc {
            0x826B3DC8 => {
    //   block [0x826B3DC8..0x826B3E38)
	// 826B3DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3DD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3DDC: 38AADA64  addi r5, r10, -0x259c
	ctx.r[5].s64 = ctx.r[10].s64 + -9628;
	// 826B3DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3DE4: 390B86D0  addi r8, r11, -0x7930
	ctx.r[8].s64 = ctx.r[11].s64 + -31024;
	// 826B3DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B3DEC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826B3DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3DF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3E00: 386AE0C4  addi r3, r10, -0x1f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7996;
	// 826B3E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3E24: 4BDB2FFD  bl 0x82466e20
	ctx.lr = 0x826B3E28;
	sub_82466E20(ctx, base);
	// 826B3E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3E38 size=112
    let mut pc: u32 = 0x826B3E38;
    'dispatch: loop {
        match pc {
            0x826B3E38 => {
    //   block [0x826B3E38..0x826B3EA8)
	// 826B3E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3E44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3E48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3E4C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 826B3E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3E54: 390B86E8  addi r8, r11, -0x7918
	ctx.r[8].s64 = ctx.r[11].s64 + -31000;
	// 826B3E58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3E5C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826B3E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3E70: 386AE0F4  addi r3, r10, -0x1f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -7948;
	// 826B3E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3E94: 4BDB2F8D  bl 0x82466e20
	ctx.lr = 0x826B3E98;
	sub_82466E20(ctx, base);
	// 826B3E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B3EA8 size=24
    let mut pc: u32 = 0x826B3EA8;
    'dispatch: loop {
        match pc {
            0x826B3EA8 => {
    //   block [0x826B3EA8..0x826B3EC0)
	// 826B3EA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3EAC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B3EB0: 394ABF40  addi r10, r10, -0x40c0
	ctx.r[10].s64 = ctx.r[10].s64 + -16576;
	// 826B3EB4: 816B82AC  lwz r11, -0x7d54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32084 as u32) ) } as u64;
	// 826B3EB8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B3EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3EC0 size=112
    let mut pc: u32 = 0x826B3EC0;
    'dispatch: loop {
        match pc {
            0x826B3EC0 => {
    //   block [0x826B3EC0..0x826B3F30)
	// 826B3EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3ECC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B3ED0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3ED4: 392AF608  addi r9, r10, -0x9f8
	ctx.r[9].s64 = ctx.r[10].s64 + -2552;
	// 826B3ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3EDC: 390BBF40  addi r8, r11, -0x40c0
	ctx.r[8].s64 = ctx.r[11].s64 + -16576;
	// 826B3EE0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B3EE4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826B3EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3EEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3EF8: 386AE124  addi r3, r10, -0x1edc
	ctx.r[3].s64 = ctx.r[10].s64 + -7900;
	// 826B3EFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B3F00: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826B3F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3F1C: 4BDB2F05  bl 0x82466e20
	ctx.lr = 0x826B3F20;
	sub_82466E20(ctx, base);
	// 826B3F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3F30 size=112
    let mut pc: u32 = 0x826B3F30;
    'dispatch: loop {
        match pc {
            0x826B3F30 => {
    //   block [0x826B3F30..0x826B3FA0)
	// 826B3F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3F3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3F40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3F44: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B3F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3F4C: 390B871C  addi r8, r11, -0x78e4
	ctx.r[8].s64 = ctx.r[11].s64 + -30948;
	// 826B3F50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B3F54: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826B3F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3F5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B3F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3F68: 386AE154  addi r3, r10, -0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + -7852;
	// 826B3F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B3F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3F8C: 4BDB2E95  bl 0x82466e20
	ctx.lr = 0x826B3F90;
	sub_82466E20(ctx, base);
	// 826B3F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B3F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B3F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B3F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B3FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B3FA0 size=108
    let mut pc: u32 = 0x826B3FA0;
    'dispatch: loop {
        match pc {
            0x826B3FA0 => {
    //   block [0x826B3FA0..0x826B400C)
	// 826B3FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B3FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B3FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B3FAC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B3FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B3FB4: 38EB874C  addi r7, r11, -0x78b4
	ctx.r[7].s64 = ctx.r[11].s64 + -30900;
	// 826B3FB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B3FBC: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826B3FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B3FC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B3FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B3FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B3FD0: 386AE184  addi r3, r10, -0x1e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7804;
	// 826B3FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B3FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B3FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B3FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B3FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B3FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B3FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B3FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B3FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B3FF8: 4BDB2E29  bl 0x82466e20
	ctx.lr = 0x826B3FFC;
	sub_82466E20(ctx, base);
	// 826B3FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4010 size=100
    let mut pc: u32 = 0x826B4010;
    'dispatch: loop {
        match pc {
            0x826B4010 => {
    //   block [0x826B4010..0x826B4074)
	// 826B4010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B401C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4024: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B402C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4030: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826B4034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B403C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4044: 386AE1B4  addi r3, r10, -0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7756;
	// 826B4048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B404C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4050: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B4054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B405C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4060: 4BDB2DC1  bl 0x82466e20
	ctx.lr = 0x826B4064;
	sub_82466E20(ctx, base);
	// 826B4064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B406C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4078 size=112
    let mut pc: u32 = 0x826B4078;
    'dispatch: loop {
        match pc {
            0x826B4078 => {
    //   block [0x826B4078..0x826B40E8)
	// 826B4078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B407C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4084: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4088: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B408C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4094: 390B8764  addi r8, r11, -0x789c
	ctx.r[8].s64 = ctx.r[11].s64 + -30876;
	// 826B4098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B409C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826B40A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B40A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B40A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B40AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B40B0: 386AE1E4  addi r3, r10, -0x1e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7708;
	// 826B40B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B40B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B40BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B40C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B40C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B40C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B40CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B40D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B40D4: 4BDB2D4D  bl 0x82466e20
	ctx.lr = 0x826B40D8;
	sub_82466E20(ctx, base);
	// 826B40D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B40DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B40E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B40E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B40E8 size=112
    let mut pc: u32 = 0x826B40E8;
    'dispatch: loop {
        match pc {
            0x826B40E8 => {
    //   block [0x826B40E8..0x826B4158)
	// 826B40E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B40EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B40F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B40F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B40F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B40FC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4104: 390B877C  addi r8, r11, -0x7884
	ctx.r[8].s64 = ctx.r[11].s64 + -30852;
	// 826B4108: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B410C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826B4110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B411C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4120: 386AE214  addi r3, r10, -0x1dec
	ctx.r[3].s64 = ctx.r[10].s64 + -7660;
	// 826B4124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B412C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B413C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4144: 4BDB2CDD  bl 0x82466e20
	ctx.lr = 0x826B4148;
	sub_82466E20(ctx, base);
	// 826B4148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B414C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4158 size=112
    let mut pc: u32 = 0x826B4158;
    'dispatch: loop {
        match pc {
            0x826B4158 => {
    //   block [0x826B4158..0x826B41C8)
	// 826B4158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4168: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B416C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4174: 390B87AC  addi r8, r11, -0x7854
	ctx.r[8].s64 = ctx.r[11].s64 + -30804;
	// 826B4178: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B417C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826B4180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B418C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4190: 386AE244  addi r3, r10, -0x1dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -7612;
	// 826B4194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B419C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B41A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B41A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B41A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B41AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B41B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B41B4: 4BDB2C6D  bl 0x82466e20
	ctx.lr = 0x826B41B8;
	sub_82466E20(ctx, base);
	// 826B41B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B41BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B41C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B41C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B41C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B41C8 size=112
    let mut pc: u32 = 0x826B41C8;
    'dispatch: loop {
        match pc {
            0x826B41C8 => {
    //   block [0x826B41C8..0x826B4238)
	// 826B41C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B41CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B41D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B41D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B41D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B41DC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B41E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B41E4: 390B87DC  addi r8, r11, -0x7824
	ctx.r[8].s64 = ctx.r[11].s64 + -30756;
	// 826B41E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B41EC: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826B41F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B41F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B41F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B41FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4200: 386AE274  addi r3, r10, -0x1d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7564;
	// 826B4204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B420C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B421C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4224: 4BDB2BFD  bl 0x82466e20
	ctx.lr = 0x826B4228;
	sub_82466E20(ctx, base);
	// 826B4228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B422C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4238 size=112
    let mut pc: u32 = 0x826B4238;
    'dispatch: loop {
        match pc {
            0x826B4238 => {
    //   block [0x826B4238..0x826B42A8)
	// 826B4238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B423C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4244: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4248: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B424C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4254: 390B880C  addi r8, r11, -0x77f4
	ctx.r[8].s64 = ctx.r[11].s64 + -30708;
	// 826B4258: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B425C: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826B4260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B426C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4270: 386AE2A4  addi r3, r10, -0x1d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7516;
	// 826B4274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B427C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B428C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4294: 4BDB2B8D  bl 0x82466e20
	ctx.lr = 0x826B4298;
	sub_82466E20(ctx, base);
	// 826B4298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B429C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B42A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B42A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B42A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B42A8 size=112
    let mut pc: u32 = 0x826B42A8;
    'dispatch: loop {
        match pc {
            0x826B42A8 => {
    //   block [0x826B42A8..0x826B4318)
	// 826B42A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B42AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B42B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B42B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B42B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B42BC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B42C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B42C4: 390B8824  addi r8, r11, -0x77dc
	ctx.r[8].s64 = ctx.r[11].s64 + -30684;
	// 826B42C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B42CC: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826B42D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B42D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B42D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B42DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B42E0: 386AE2D4  addi r3, r10, -0x1d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7468;
	// 826B42E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B42E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B42EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B42F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B42F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B42F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B42FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4304: 4BDB2B1D  bl 0x82466e20
	ctx.lr = 0x826B4308;
	sub_82466E20(ctx, base);
	// 826B4308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B430C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4318 size=112
    let mut pc: u32 = 0x826B4318;
    'dispatch: loop {
        match pc {
            0x826B4318 => {
    //   block [0x826B4318..0x826B4388)
	// 826B4318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4328: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B432C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4334: 390B8840  addi r8, r11, -0x77c0
	ctx.r[8].s64 = ctx.r[11].s64 + -30656;
	// 826B4338: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B433C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826B4340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B434C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4350: 386AE304  addi r3, r10, -0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -7420;
	// 826B4354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B435C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B436C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4374: 4BDB2AAD  bl 0x82466e20
	ctx.lr = 0x826B4378;
	sub_82466E20(ctx, base);
	// 826B4378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B437C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4388 size=112
    let mut pc: u32 = 0x826B4388;
    'dispatch: loop {
        match pc {
            0x826B4388 => {
    //   block [0x826B4388..0x826B43F8)
	// 826B4388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B438C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4394: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4398: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B439C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B43A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B43A4: 390B8888  addi r8, r11, -0x7778
	ctx.r[8].s64 = ctx.r[11].s64 + -30584;
	// 826B43A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B43AC: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826B43B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B43B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B43B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B43BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B43C0: 386AE334  addi r3, r10, -0x1ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -7372;
	// 826B43C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B43C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B43CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B43D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B43D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B43D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B43DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B43E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B43E4: 4BDB2A3D  bl 0x82466e20
	ctx.lr = 0x826B43E8;
	sub_82466E20(ctx, base);
	// 826B43E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B43EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B43F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B43F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B43F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B43F8 size=112
    let mut pc: u32 = 0x826B43F8;
    'dispatch: loop {
        match pc {
            0x826B43F8 => {
    //   block [0x826B43F8..0x826B4468)
	// 826B43F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B43FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4404: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4408: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B440C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4414: 390B88D0  addi r8, r11, -0x7730
	ctx.r[8].s64 = ctx.r[11].s64 + -30512;
	// 826B4418: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B441C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826B4420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B442C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4430: 386AE364  addi r3, r10, -0x1c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -7324;
	// 826B4434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B443C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B444C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4454: 4BDB29CD  bl 0x82466e20
	ctx.lr = 0x826B4458;
	sub_82466E20(ctx, base);
	// 826B4458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B445C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4468 size=112
    let mut pc: u32 = 0x826B4468;
    'dispatch: loop {
        match pc {
            0x826B4468 => {
    //   block [0x826B4468..0x826B44D8)
	// 826B4468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B446C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4474: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4478: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B447C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4484: 390B88E8  addi r8, r11, -0x7718
	ctx.r[8].s64 = ctx.r[11].s64 + -30488;
	// 826B4488: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B448C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826B4490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B449C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B44A0: 386AE394  addi r3, r10, -0x1c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -7276;
	// 826B44A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B44A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B44AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B44B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B44B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B44B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B44BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B44C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B44C4: 4BDB295D  bl 0x82466e20
	ctx.lr = 0x826B44C8;
	sub_82466E20(ctx, base);
	// 826B44C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B44CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B44D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B44D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B44D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B44D8 size=116
    let mut pc: u32 = 0x826B44D8;
    'dispatch: loop {
        match pc {
            0x826B44D8 => {
    //   block [0x826B44D8..0x826B454C)
	// 826B44D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B44DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B44E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B44E4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B44E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B44EC: 390A8918  addi r8, r10, -0x76e8
	ctx.r[8].s64 = ctx.r[10].s64 + -30440;
	// 826B44F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B44F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B44F8: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B44FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4500: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B450C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826B4510: 396BF630  addi r11, r11, -0x9d0
	ctx.r[11].s64 = ctx.r[11].s64 + -2512;
	// 826B4514: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4518: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B451C: 386AE3C4  addi r3, r10, -0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7228;
	// 826B4520: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B4524: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4528: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B452C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4538: 4BDB28E9  bl 0x82466e20
	ctx.lr = 0x826B453C;
	sub_82466E20(ctx, base);
	// 826B453C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4550 size=116
    let mut pc: u32 = 0x826B4550;
    'dispatch: loop {
        match pc {
            0x826B4550 => {
    //   block [0x826B4550..0x826B45C4)
	// 826B4550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B455C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B4560: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B4564: 390A8990  addi r8, r10, -0x7670
	ctx.r[8].s64 = ctx.r[10].s64 + -30320;
	// 826B4568: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B456C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B4570: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4574: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4578: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B457C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4584: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826B4588: 396BF648  addi r11, r11, -0x9b8
	ctx.r[11].s64 = ctx.r[11].s64 + -2488;
	// 826B458C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4590: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4594: 386AE3F4  addi r3, r10, -0x1c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -7180;
	// 826B4598: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B459C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B45A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B45A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B45A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B45AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B45B0: 4BDB2871  bl 0x82466e20
	ctx.lr = 0x826B45B4;
	sub_82466E20(ctx, base);
	// 826B45B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B45B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B45BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B45C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B45C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B45C8 size=24
    let mut pc: u32 = 0x826B45C8;
    'dispatch: loop {
        match pc {
            0x826B45C8 => {
    //   block [0x826B45C8..0x826B45E0)
	// 826B45C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B45CC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B45D0: 394ABF58  addi r10, r10, -0x40a8
	ctx.r[10].s64 = ctx.r[10].s64 + -16552;
	// 826B45D4: 816B883C  lwz r11, -0x77c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30660 as u32) ) } as u64;
	// 826B45D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B45DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B45E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B45E0 size=116
    let mut pc: u32 = 0x826B45E0;
    'dispatch: loop {
        match pc {
            0x826B45E0 => {
    //   block [0x826B45E0..0x826B4654)
	// 826B45E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B45E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B45E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B45EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B45F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B45F4: 392BF674  addi r9, r11, -0x98c
	ctx.r[9].s64 = ctx.r[11].s64 + -2444;
	// 826B45F8: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B45FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4600: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B4604: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826B4608: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B460C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826B4610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4614: 396BBF58  addi r11, r11, -0x40a8
	ctx.r[11].s64 = ctx.r[11].s64 + -16552;
	// 826B4618: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B461C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4620: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B4624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4628: 386AE424  addi r3, r10, -0x1bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -7132;
	// 826B462C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B4630: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B4634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4638: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B463C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B4640: 4BDB27E1  bl 0x82466e20
	ctx.lr = 0x826B4644;
	sub_82466E20(ctx, base);
	// 826B4644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B464C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4658 size=112
    let mut pc: u32 = 0x826B4658;
    'dispatch: loop {
        match pc {
            0x826B4658 => {
    //   block [0x826B4658..0x826B46C8)
	// 826B4658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B465C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4664: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4668: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B466C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4674: 390B8A20  addi r8, r11, -0x75e0
	ctx.r[8].s64 = ctx.r[11].s64 + -30176;
	// 826B4678: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B467C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826B4680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B468C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4690: 386AE454  addi r3, r10, -0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + -7084;
	// 826B4694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B469C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B46A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B46A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B46A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B46AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B46B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B46B4: 4BDB276D  bl 0x82466e20
	ctx.lr = 0x826B46B8;
	sub_82466E20(ctx, base);
	// 826B46B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B46BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B46C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B46C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B46C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B46C8 size=112
    let mut pc: u32 = 0x826B46C8;
    'dispatch: loop {
        match pc {
            0x826B46C8 => {
    //   block [0x826B46C8..0x826B4738)
	// 826B46C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B46CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B46D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B46D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B46D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B46DC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B46E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B46E4: 390B8A80  addi r8, r11, -0x7580
	ctx.r[8].s64 = ctx.r[11].s64 + -30080;
	// 826B46E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B46EC: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826B46F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B46F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B46F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B46FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4700: 386AE484  addi r3, r10, -0x1b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7036;
	// 826B4704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B470C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B471C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4724: 4BDB26FD  bl 0x82466e20
	ctx.lr = 0x826B4728;
	sub_82466E20(ctx, base);
	// 826B4728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B472C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4738 size=112
    let mut pc: u32 = 0x826B4738;
    'dispatch: loop {
        match pc {
            0x826B4738 => {
    //   block [0x826B4738..0x826B47A8)
	// 826B4738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B473C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4744: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4748: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B474C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4754: 390B8B28  addi r8, r11, -0x74d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29912;
	// 826B4758: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B475C: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826B4760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B476C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4770: 386AE4B4  addi r3, r10, -0x1b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -6988;
	// 826B4774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B477C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B478C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4794: 4BDB268D  bl 0x82466e20
	ctx.lr = 0x826B4798;
	sub_82466E20(ctx, base);
	// 826B4798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B479C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B47A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B47A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B47A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B47A8 size=112
    let mut pc: u32 = 0x826B47A8;
    'dispatch: loop {
        match pc {
            0x826B47A8 => {
    //   block [0x826B47A8..0x826B4818)
	// 826B47A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B47AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B47B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B47B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B47B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B47BC: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B47C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B47C4: 390B8BA0  addi r8, r11, -0x7460
	ctx.r[8].s64 = ctx.r[11].s64 + -29792;
	// 826B47C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B47CC: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826B47D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B47D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B47D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B47DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B47E0: 386AE4E4  addi r3, r10, -0x1b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -6940;
	// 826B47E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B47E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B47EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B47F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B47F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B47F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B47FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4804: 4BDB261D  bl 0x82466e20
	ctx.lr = 0x826B4808;
	sub_82466E20(ctx, base);
	// 826B4808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B480C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4818 size=112
    let mut pc: u32 = 0x826B4818;
    'dispatch: loop {
        match pc {
            0x826B4818 => {
    //   block [0x826B4818..0x826B4888)
	// 826B4818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B481C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4824: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4828: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B482C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4834: 390B8BE8  addi r8, r11, -0x7418
	ctx.r[8].s64 = ctx.r[11].s64 + -29720;
	// 826B4838: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B483C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826B4840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B484C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4850: 386AE514  addi r3, r10, -0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + -6892;
	// 826B4854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B485C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B486C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4874: 4BDB25AD  bl 0x82466e20
	ctx.lr = 0x826B4878;
	sub_82466E20(ctx, base);
	// 826B4878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B487C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4888 size=112
    let mut pc: u32 = 0x826B4888;
    'dispatch: loop {
        match pc {
            0x826B4888 => {
    //   block [0x826B4888..0x826B48F8)
	// 826B4888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B488C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4894: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4898: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B489C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B48A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B48A4: 390B8C78  addi r8, r11, -0x7388
	ctx.r[8].s64 = ctx.r[11].s64 + -29576;
	// 826B48A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B48AC: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826B48B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B48B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B48B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B48BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B48C0: 386AE544  addi r3, r10, -0x1abc
	ctx.r[3].s64 = ctx.r[10].s64 + -6844;
	// 826B48C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B48C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B48CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B48D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B48D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B48D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B48DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B48E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B48E4: 4BDB253D  bl 0x82466e20
	ctx.lr = 0x826B48E8;
	sub_82466E20(ctx, base);
	// 826B48E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B48EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B48F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B48F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B48F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B48F8 size=112
    let mut pc: u32 = 0x826B48F8;
    'dispatch: loop {
        match pc {
            0x826B48F8 => {
    //   block [0x826B48F8..0x826B4968)
	// 826B48F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B48FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4908: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B490C: 38AAE124  addi r5, r10, -0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + -7900;
	// 826B4910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4914: 390B8CD8  addi r8, r11, -0x7328
	ctx.r[8].s64 = ctx.r[11].s64 + -29480;
	// 826B4918: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B491C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826B4920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B492C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4930: 386AE574  addi r3, r10, -0x1a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -6796;
	// 826B4934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B493C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B494C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4954: 4BDB24CD  bl 0x82466e20
	ctx.lr = 0x826B4958;
	sub_82466E20(ctx, base);
	// 826B4958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B495C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4968 size=112
    let mut pc: u32 = 0x826B4968;
    'dispatch: loop {
        match pc {
            0x826B4968 => {
    //   block [0x826B4968..0x826B49D8)
	// 826B4968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B496C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4974: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4978: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B497C: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B4980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4984: 390B8D20  addi r8, r11, -0x72e0
	ctx.r[8].s64 = ctx.r[11].s64 + -29408;
	// 826B4988: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B498C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826B4990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4998: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B499C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B49A0: 386AE5A4  addi r3, r10, -0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -6748;
	// 826B49A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B49A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B49AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B49B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B49B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B49B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B49BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B49C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B49C4: 4BDB245D  bl 0x82466e20
	ctx.lr = 0x826B49C8;
	sub_82466E20(ctx, base);
	// 826B49C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B49CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B49D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B49D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B49D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B49D8 size=112
    let mut pc: u32 = 0x826B49D8;
    'dispatch: loop {
        match pc {
            0x826B49D8 => {
    //   block [0x826B49D8..0x826B4A48)
	// 826B49D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B49DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B49E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B49E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B49E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B49EC: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B49F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B49F4: 390B8D50  addi r8, r11, -0x72b0
	ctx.r[8].s64 = ctx.r[11].s64 + -29360;
	// 826B49F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B49FC: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826B4A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4A10: 386AE5D4  addi r3, r10, -0x1a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -6700;
	// 826B4A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4A34: 4BDB23ED  bl 0x82466e20
	ctx.lr = 0x826B4A38;
	sub_82466E20(ctx, base);
	// 826B4A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4A48 size=100
    let mut pc: u32 = 0x826B4A48;
    'dispatch: loop {
        match pc {
            0x826B4A48 => {
    //   block [0x826B4A48..0x826B4AAC)
	// 826B4A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4A5C: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B4A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4A68: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826B4A6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4A7C: 386AE604  addi r3, r10, -0x19fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6652;
	// 826B4A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4A88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B4A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4A90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B4A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4A98: 4BDB2389  bl 0x82466e20
	ctx.lr = 0x826B4A9C;
	sub_82466E20(ctx, base);
	// 826B4A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4AB0 size=112
    let mut pc: u32 = 0x826B4AB0;
    'dispatch: loop {
        match pc {
            0x826B4AB0 => {
    //   block [0x826B4AB0..0x826B4B20)
	// 826B4AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4ABC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4AC0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4AC4: 38AAE574  addi r5, r10, -0x1a8c
	ctx.r[5].s64 = ctx.r[10].s64 + -6796;
	// 826B4AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4ACC: 390B8D80  addi r8, r11, -0x7280
	ctx.r[8].s64 = ctx.r[11].s64 + -29312;
	// 826B4AD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B4AD4: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826B4AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4ADC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4AE8: 386AE634  addi r3, r10, -0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6604;
	// 826B4AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4B0C: 4BDB2315  bl 0x82466e20
	ctx.lr = 0x826B4B10;
	sub_82466E20(ctx, base);
	// 826B4B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4B20 size=112
    let mut pc: u32 = 0x826B4B20;
    'dispatch: loop {
        match pc {
            0x826B4B20 => {
    //   block [0x826B4B20..0x826B4B90)
	// 826B4B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4B2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4B30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4B34: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B4B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4B3C: 390B8D98  addi r8, r11, -0x7268
	ctx.r[8].s64 = ctx.r[11].s64 + -29288;
	// 826B4B40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B4B44: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826B4B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4B4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4B58: 386AE664  addi r3, r10, -0x199c
	ctx.r[3].s64 = ctx.r[10].s64 + -6556;
	// 826B4B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4B7C: 4BDB22A5  bl 0x82466e20
	ctx.lr = 0x826B4B80;
	sub_82466E20(ctx, base);
	// 826B4B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4B90 size=112
    let mut pc: u32 = 0x826B4B90;
    'dispatch: loop {
        match pc {
            0x826B4B90 => {
    //   block [0x826B4B90..0x826B4C00)
	// 826B4B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4B9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4BA0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4BA4: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B4BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4BAC: 390B8DF8  addi r8, r11, -0x7208
	ctx.r[8].s64 = ctx.r[11].s64 + -29192;
	// 826B4BB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B4BB4: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826B4BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4BBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4BC8: 386AE694  addi r3, r10, -0x196c
	ctx.r[3].s64 = ctx.r[10].s64 + -6508;
	// 826B4BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4BEC: 4BDB2235  bl 0x82466e20
	ctx.lr = 0x826B4BF0;
	sub_82466E20(ctx, base);
	// 826B4BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4C00 size=112
    let mut pc: u32 = 0x826B4C00;
    'dispatch: loop {
        match pc {
            0x826B4C00 => {
    //   block [0x826B4C00..0x826B4C70)
	// 826B4C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4C0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4C10: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4C14: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B4C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4C1C: 390B8E10  addi r8, r11, -0x71f0
	ctx.r[8].s64 = ctx.r[11].s64 + -29168;
	// 826B4C20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B4C24: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826B4C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4C2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4C38: 386AE6C4  addi r3, r10, -0x193c
	ctx.r[3].s64 = ctx.r[10].s64 + -6460;
	// 826B4C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4C5C: 4BDB21C5  bl 0x82466e20
	ctx.lr = 0x826B4C60;
	sub_82466E20(ctx, base);
	// 826B4C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4C70 size=112
    let mut pc: u32 = 0x826B4C70;
    'dispatch: loop {
        match pc {
            0x826B4C70 => {
    //   block [0x826B4C70..0x826B4CE0)
	// 826B4C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4C7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4C80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4C84: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B4C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4C8C: 390B8E40  addi r8, r11, -0x71c0
	ctx.r[8].s64 = ctx.r[11].s64 + -29120;
	// 826B4C90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B4C94: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826B4C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4C9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4CA8: 386AE6F4  addi r3, r10, -0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + -6412;
	// 826B4CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B4CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4CCC: 4BDB2155  bl 0x82466e20
	ctx.lr = 0x826B4CD0;
	sub_82466E20(ctx, base);
	// 826B4CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B4CE0 size=24
    let mut pc: u32 = 0x826B4CE0;
    'dispatch: loop {
        match pc {
            0x826B4CE0 => {
    //   block [0x826B4CE0..0x826B4CF8)
	// 826B4CE0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4CE4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B4CE8: 394AC000  addi r10, r10, -0x4000
	ctx.r[10].s64 = ctx.r[10].s64 + -16384;
	// 826B4CEC: 816B8E58  lwz r11, -0x71a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29096 as u32) ) } as u64;
	// 826B4CF0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B4CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4CF8 size=112
    let mut pc: u32 = 0x826B4CF8;
    'dispatch: loop {
        match pc {
            0x826B4CF8 => {
    //   block [0x826B4CF8..0x826B4D68)
	// 826B4CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4D04: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B4D08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4D0C: 392AF6D0  addi r9, r10, -0x930
	ctx.r[9].s64 = ctx.r[10].s64 + -2352;
	// 826B4D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4D14: 390BC000  addi r8, r11, -0x4000
	ctx.r[8].s64 = ctx.r[11].s64 + -16384;
	// 826B4D18: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B4D1C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826B4D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4D24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4D30: 386AE724  addi r3, r10, -0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6364;
	// 826B4D34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4D38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B4D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4D54: 4BDB20CD  bl 0x82466e20
	ctx.lr = 0x826B4D58;
	sub_82466E20(ctx, base);
	// 826B4D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4D68 size=108
    let mut pc: u32 = 0x826B4D68;
    'dispatch: loop {
        match pc {
            0x826B4D68 => {
    //   block [0x826B4D68..0x826B4DD4)
	// 826B4D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4D74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4D7C: 38EB8E5C  addi r7, r11, -0x71a4
	ctx.r[7].s64 = ctx.r[11].s64 + -29092;
	// 826B4D80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B4D84: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826B4D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4D90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4D98: 386AE754  addi r3, r10, -0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6316;
	// 826B4D9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4DC0: 4BDB2061  bl 0x82466e20
	ctx.lr = 0x826B4DC4;
	sub_82466E20(ctx, base);
	// 826B4DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4DD8 size=108
    let mut pc: u32 = 0x826B4DD8;
    'dispatch: loop {
        match pc {
            0x826B4DD8 => {
    //   block [0x826B4DD8..0x826B4E44)
	// 826B4DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4DE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4DEC: 38EB8E78  addi r7, r11, -0x7188
	ctx.r[7].s64 = ctx.r[11].s64 + -29064;
	// 826B4DF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B4DF4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826B4DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4E08: 386AE784  addi r3, r10, -0x187c
	ctx.r[3].s64 = ctx.r[10].s64 + -6268;
	// 826B4E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4E30: 4BDB1FF1  bl 0x82466e20
	ctx.lr = 0x826B4E34;
	sub_82466E20(ctx, base);
	// 826B4E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4E48 size=116
    let mut pc: u32 = 0x826B4E48;
    'dispatch: loop {
        match pc {
            0x826B4E48 => {
    //   block [0x826B4E48..0x826B4EBC)
	// 826B4E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4E54: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4E58: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B4E5C: 390B8EC0  addi r8, r11, -0x7140
	ctx.r[8].s64 = ctx.r[11].s64 + -28992;
	// 826B4E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4E64: 392AF788  addi r9, r10, -0x878
	ctx.r[9].s64 = ctx.r[10].s64 + -2168;
	// 826B4E68: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4E6C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B4E70: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B4E74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4E7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4E8C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B4E90: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826B4E94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4E98: 386BE7B4  addi r3, r11, -0x184c
	ctx.r[3].s64 = ctx.r[11].s64 + -6220;
	// 826B4E9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B4EA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4EA8: 4BDB1F79  bl 0x82466e20
	ctx.lr = 0x826B4EAC;
	sub_82466E20(ctx, base);
	// 826B4EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4EC0 size=108
    let mut pc: u32 = 0x826B4EC0;
    'dispatch: loop {
        match pc {
            0x826B4EC0 => {
    //   block [0x826B4EC0..0x826B4F2C)
	// 826B4EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4ECC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4ED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B4ED4: 38EB8ED8  addi r7, r11, -0x7128
	ctx.r[7].s64 = ctx.r[11].s64 + -28968;
	// 826B4ED8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B4EDC: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826B4EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4EE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4EF0: 386AE7E4  addi r3, r10, -0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + -6172;
	// 826B4EF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B4F18: 4BDB1F09  bl 0x82466e20
	ctx.lr = 0x826B4F1C;
	sub_82466E20(ctx, base);
	// 826B4F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B4F30 size=24
    let mut pc: u32 = 0x826B4F30;
    'dispatch: loop {
        match pc {
            0x826B4F30 => {
    //   block [0x826B4F30..0x826B4F48)
	// 826B4F30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4F34: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B4F38: 394AC048  addi r10, r10, -0x3fb8
	ctx.r[10].s64 = ctx.r[10].s64 + -16312;
	// 826B4F3C: 816B8F38  lwz r11, -0x70c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28872 as u32) ) } as u64;
	// 826B4F40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B4F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4F48 size=116
    let mut pc: u32 = 0x826B4F48;
    'dispatch: loop {
        match pc {
            0x826B4F48 => {
    //   block [0x826B4F48..0x826B4FBC)
	// 826B4F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4F54: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4F58: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B4F5C: 390BC048  addi r8, r11, -0x3fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -16312;
	// 826B4F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4F64: 392AF7E4  addi r9, r10, -0x81c
	ctx.r[9].s64 = ctx.r[10].s64 + -2076;
	// 826B4F68: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4F6C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826B4F70: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B4F74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B4F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B4F7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B4F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B4F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B4F8C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B4F90: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826B4F94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B4F98: 386BE814  addi r3, r11, -0x17ec
	ctx.r[3].s64 = ctx.r[11].s64 + -6124;
	// 826B4F9C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826B4FA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B4FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B4FA8: 4BDB1E79  bl 0x82466e20
	ctx.lr = 0x826B4FAC;
	sub_82466E20(ctx, base);
	// 826B4FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B4FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B4FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B4FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B4FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B4FC0 size=108
    let mut pc: u32 = 0x826B4FC0;
    'dispatch: loop {
        match pc {
            0x826B4FC0 => {
    //   block [0x826B4FC0..0x826B502C)
	// 826B4FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B4FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B4FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B4FCC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B4FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B4FD4: 38EB8F44  addi r7, r11, -0x70bc
	ctx.r[7].s64 = ctx.r[11].s64 + -28860;
	// 826B4FD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B4FDC: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826B4FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B4FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B4FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B4FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B4FF0: 386AE844  addi r3, r10, -0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6076;
	// 826B4FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B4FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B4FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B500C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5018: 4BDB1E09  bl 0x82466e20
	ctx.lr = 0x826B501C;
	sub_82466E20(ctx, base);
	// 826B501C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5030 size=112
    let mut pc: u32 = 0x826B5030;
    'dispatch: loop {
        match pc {
            0x826B5030 => {
    //   block [0x826B5030..0x826B50A0)
	// 826B5030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B503C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5040: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5044: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B504C: 390B8F74  addi r8, r11, -0x708c
	ctx.r[8].s64 = ctx.r[11].s64 + -28812;
	// 826B5050: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5054: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826B5058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B505C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5068: 386AE874  addi r3, r10, -0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + -6028;
	// 826B506C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B507C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B508C: 4BDB1D95  bl 0x82466e20
	ctx.lr = 0x826B5090;
	sub_82466E20(ctx, base);
	// 826B5090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B509C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B50A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B50A0 size=112
    let mut pc: u32 = 0x826B50A0;
    'dispatch: loop {
        match pc {
            0x826B50A0 => {
    //   block [0x826B50A0..0x826B5110)
	// 826B50A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B50A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B50A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B50AC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B50B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B50B4: 392AF828  addi r9, r10, -0x7d8
	ctx.r[9].s64 = ctx.r[10].s64 + -2008;
	// 826B50B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B50BC: 390B8F90  addi r8, r11, -0x7070
	ctx.r[8].s64 = ctx.r[11].s64 + -28784;
	// 826B50C0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B50C4: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826B50C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B50CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B50D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B50D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B50D8: 386AE8A4  addi r3, r10, -0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + -5980;
	// 826B50DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B50E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B50E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B50E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B50EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B50F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B50F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B50F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B50FC: 4BDB1D25  bl 0x82466e20
	ctx.lr = 0x826B5100;
	sub_82466E20(ctx, base);
	// 826B5100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5110 size=112
    let mut pc: u32 = 0x826B5110;
    'dispatch: loop {
        match pc {
            0x826B5110 => {
    //   block [0x826B5110..0x826B5180)
	// 826B5110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B511C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5120: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5124: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B512C: 390B8FD8  addi r8, r11, -0x7028
	ctx.r[8].s64 = ctx.r[11].s64 + -28712;
	// 826B5130: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5134: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826B5138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B513C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5148: 386AE8D4  addi r3, r10, -0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + -5932;
	// 826B514C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B515C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B516C: 4BDB1CB5  bl 0x82466e20
	ctx.lr = 0x826B5170;
	sub_82466E20(ctx, base);
	// 826B5170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B517C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5180 size=112
    let mut pc: u32 = 0x826B5180;
    'dispatch: loop {
        match pc {
            0x826B5180 => {
    //   block [0x826B5180..0x826B51F0)
	// 826B5180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B518C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5190: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5194: 392AF854  addi r9, r10, -0x7ac
	ctx.r[9].s64 = ctx.r[10].s64 + -1964;
	// 826B5198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B519C: 390B8FF8  addi r8, r11, -0x7008
	ctx.r[8].s64 = ctx.r[11].s64 + -28680;
	// 826B51A0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B51A4: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826B51A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B51AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B51B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B51B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B51B8: 386AE904  addi r3, r10, -0x16fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5884;
	// 826B51BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B51C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B51C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B51C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B51CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B51D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B51D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B51D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B51DC: 4BDB1C45  bl 0x82466e20
	ctx.lr = 0x826B51E0;
	sub_82466E20(ctx, base);
	// 826B51E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B51E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B51E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B51EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B51F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B51F0 size=112
    let mut pc: u32 = 0x826B51F0;
    'dispatch: loop {
        match pc {
            0x826B51F0 => {
    //   block [0x826B51F0..0x826B5260)
	// 826B51F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B51F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B51F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B51FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5200: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5204: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B520C: 390B9088  addi r8, r11, -0x6f78
	ctx.r[8].s64 = ctx.r[11].s64 + -28536;
	// 826B5210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5214: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826B5218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B521C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5228: 386AE934  addi r3, r10, -0x16cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5836;
	// 826B522C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B523C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B524C: 4BDB1BD5  bl 0x82466e20
	ctx.lr = 0x826B5250;
	sub_82466E20(ctx, base);
	// 826B5250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B525C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5260 size=112
    let mut pc: u32 = 0x826B5260;
    'dispatch: loop {
        match pc {
            0x826B5260 => {
    //   block [0x826B5260..0x826B52D0)
	// 826B5260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B526C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5270: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5274: 38AAE994  addi r5, r10, -0x166c
	ctx.r[5].s64 = ctx.r[10].s64 + -5740;
	// 826B5278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B527C: 390B90A0  addi r8, r11, -0x6f60
	ctx.r[8].s64 = ctx.r[11].s64 + -28512;
	// 826B5280: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B5284: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826B5288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B528C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5298: 386AE964  addi r3, r10, -0x169c
	ctx.r[3].s64 = ctx.r[10].s64 + -5788;
	// 826B529C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B52A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B52A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B52A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B52AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B52B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B52B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B52B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B52BC: 4BDB1B65  bl 0x82466e20
	ctx.lr = 0x826B52C0;
	sub_82466E20(ctx, base);
	// 826B52C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B52C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B52C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B52CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B52D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B52D0 size=100
    let mut pc: u32 = 0x826B52D0;
    'dispatch: loop {
        match pc {
            0x826B52D0 => {
    //   block [0x826B52D0..0x826B5334)
	// 826B52D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B52D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B52D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B52DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B52E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B52E4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B52E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B52EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B52F0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826B52F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B52F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B52FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5304: 386AE994  addi r3, r10, -0x166c
	ctx.r[3].s64 = ctx.r[10].s64 + -5740;
	// 826B5308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B530C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B5314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B531C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5320: 4BDB1B01  bl 0x82466e20
	ctx.lr = 0x826B5324;
	sub_82466E20(ctx, base);
	// 826B5324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B532C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B5338 size=24
    let mut pc: u32 = 0x826B5338;
    'dispatch: loop {
        match pc {
            0x826B5338 => {
    //   block [0x826B5338..0x826B5350)
	// 826B5338: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B533C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B5340: 394AC120  addi r10, r10, -0x3ee0
	ctx.r[10].s64 = ctx.r[10].s64 + -16096;
	// 826B5344: 816B8FF4  lwz r11, -0x700c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28684 as u32) ) } as u64;
	// 826B5348: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5350 size=116
    let mut pc: u32 = 0x826B5350;
    'dispatch: loop {
        match pc {
            0x826B5350 => {
    //   block [0x826B5350..0x826B53C4)
	// 826B5350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B535C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5360: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5364: 390BC120  addi r8, r11, -0x3ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -16096;
	// 826B5368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B536C: 392AF890  addi r9, r10, -0x770
	ctx.r[9].s64 = ctx.r[10].s64 + -1904;
	// 826B5370: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5374: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B5378: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B537C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B538C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5394: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B5398: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826B539C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B53A0: 386BE9C4  addi r3, r11, -0x163c
	ctx.r[3].s64 = ctx.r[11].s64 + -5692;
	// 826B53A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B53A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B53AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B53B0: 4BDB1A71  bl 0x82466e20
	ctx.lr = 0x826B53B4;
	sub_82466E20(ctx, base);
	// 826B53B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B53B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B53BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B53C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B53C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B53C8 size=108
    let mut pc: u32 = 0x826B53C8;
    'dispatch: loop {
        match pc {
            0x826B53C8 => {
    //   block [0x826B53C8..0x826B5434)
	// 826B53C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B53CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B53D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B53D4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B53D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B53DC: 38EB9118  addi r7, r11, -0x6ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -28392;
	// 826B53E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B53E4: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826B53E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B53EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B53F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B53F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B53F8: 386AE9F4  addi r3, r10, -0x160c
	ctx.r[3].s64 = ctx.r[10].s64 + -5644;
	// 826B53FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B540C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B541C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5420: 4BDB1A01  bl 0x82466e20
	ctx.lr = 0x826B5424;
	sub_82466E20(ctx, base);
	// 826B5424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B542C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5438 size=112
    let mut pc: u32 = 0x826B5438;
    'dispatch: loop {
        match pc {
            0x826B5438 => {
    //   block [0x826B5438..0x826B54A8)
	// 826B5438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5448: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B544C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5454: 390B9148  addi r8, r11, -0x6eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -28344;
	// 826B5458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B545C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826B5460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B546C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5470: 386AEA24  addi r3, r10, -0x15dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5596;
	// 826B5474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B547C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B548C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5494: 4BDB198D  bl 0x82466e20
	ctx.lr = 0x826B5498;
	sub_82466E20(ctx, base);
	// 826B5498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B549C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B54A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B54A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B54A8 size=112
    let mut pc: u32 = 0x826B54A8;
    'dispatch: loop {
        match pc {
            0x826B54A8 => {
    //   block [0x826B54A8..0x826B5518)
	// 826B54A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B54AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B54B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B54B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B54B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B54BC: 392AF8B4  addi r9, r10, -0x74c
	ctx.r[9].s64 = ctx.r[10].s64 + -1868;
	// 826B54C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B54C4: 390B9168  addi r8, r11, -0x6e98
	ctx.r[8].s64 = ctx.r[11].s64 + -28312;
	// 826B54C8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B54CC: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826B54D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B54D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B54D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B54DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B54E0: 386AEA54  addi r3, r10, -0x15ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5548;
	// 826B54E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B54E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B54EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B54F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B54F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B54F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B54FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5504: 4BDB191D  bl 0x82466e20
	ctx.lr = 0x826B5508;
	sub_82466E20(ctx, base);
	// 826B5508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B550C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5518 size=112
    let mut pc: u32 = 0x826B5518;
    'dispatch: loop {
        match pc {
            0x826B5518 => {
    //   block [0x826B5518..0x826B5588)
	// 826B5518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B551C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5528: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B552C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5534: 390B9210  addi r8, r11, -0x6df0
	ctx.r[8].s64 = ctx.r[11].s64 + -28144;
	// 826B5538: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B553C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826B5540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B554C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5550: 386AEA84  addi r3, r10, -0x157c
	ctx.r[3].s64 = ctx.r[10].s64 + -5500;
	// 826B5554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B555C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B556C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5574: 4BDB18AD  bl 0x82466e20
	ctx.lr = 0x826B5578;
	sub_82466E20(ctx, base);
	// 826B5578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B557C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5588 size=108
    let mut pc: u32 = 0x826B5588;
    'dispatch: loop {
        match pc {
            0x826B5588 => {
    //   block [0x826B5588..0x826B55F4)
	// 826B5588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5594: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B559C: 38EB9228  addi r7, r11, -0x6dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -28120;
	// 826B55A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B55A4: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826B55A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B55AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B55B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B55B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B55B8: 386AEAB4  addi r3, r10, -0x154c
	ctx.r[3].s64 = ctx.r[10].s64 + -5452;
	// 826B55BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B55C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B55C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B55C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B55CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B55D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B55D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B55D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B55DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B55E0: 4BDB1841  bl 0x82466e20
	ctx.lr = 0x826B55E4;
	sub_82466E20(ctx, base);
	// 826B55E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B55E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B55EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B55F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B55F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B55F8 size=112
    let mut pc: u32 = 0x826B55F8;
    'dispatch: loop {
        match pc {
            0x826B55F8 => {
    //   block [0x826B55F8..0x826B5668)
	// 826B55F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B55FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5608: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B560C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5614: 390B9258  addi r8, r11, -0x6da8
	ctx.r[8].s64 = ctx.r[11].s64 + -28072;
	// 826B5618: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B561C: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826B5620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5624: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B562C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5630: 386AEAE4  addi r3, r10, -0x151c
	ctx.r[3].s64 = ctx.r[10].s64 + -5404;
	// 826B5634: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B563C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B564C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5654: 4BDB17CD  bl 0x82466e20
	ctx.lr = 0x826B5658;
	sub_82466E20(ctx, base);
	// 826B5658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B565C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5668 size=112
    let mut pc: u32 = 0x826B5668;
    'dispatch: loop {
        match pc {
            0x826B5668 => {
    //   block [0x826B5668..0x826B56D8)
	// 826B5668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5674: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5678: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B567C: 392AF8E8  addi r9, r10, -0x718
	ctx.r[9].s64 = ctx.r[10].s64 + -1816;
	// 826B5680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5684: 390B9270  addi r8, r11, -0x6d90
	ctx.r[8].s64 = ctx.r[11].s64 + -28048;
	// 826B5688: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B568C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826B5690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5694: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B569C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B56A0: 386AEB14  addi r3, r10, -0x14ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5356;
	// 826B56A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B56A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B56AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B56B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B56B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B56B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B56BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B56C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B56C4: 4BDB175D  bl 0x82466e20
	ctx.lr = 0x826B56C8;
	sub_82466E20(ctx, base);
	// 826B56C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B56CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B56D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B56D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B56D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B56D8 size=112
    let mut pc: u32 = 0x826B56D8;
    'dispatch: loop {
        match pc {
            0x826B56D8 => {
    //   block [0x826B56D8..0x826B5748)
	// 826B56D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B56DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B56E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B56E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B56E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B56EC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B56F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B56F4: 390B9318  addi r8, r11, -0x6ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -27880;
	// 826B56F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B56FC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826B5700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B570C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5710: 386AEB44  addi r3, r10, -0x14bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5308;
	// 826B5714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B571C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B572C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5734: 4BDB16ED  bl 0x82466e20
	ctx.lr = 0x826B5738;
	sub_82466E20(ctx, base);
	// 826B5738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B573C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5748 size=112
    let mut pc: u32 = 0x826B5748;
    'dispatch: loop {
        match pc {
            0x826B5748 => {
    //   block [0x826B5748..0x826B57B8)
	// 826B5748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B574C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5758: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B575C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5764: 390B9360  addi r8, r11, -0x6ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -27808;
	// 826B5768: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826B576C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826B5770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B577C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5780: 386AEB74  addi r3, r10, -0x148c
	ctx.r[3].s64 = ctx.r[10].s64 + -5260;
	// 826B5784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B578C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B579C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B57A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B57A4: 4BDB167D  bl 0x82466e20
	ctx.lr = 0x826B57A8;
	sub_82466E20(ctx, base);
	// 826B57A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B57AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B57B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B57B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B57B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B57B8 size=100
    let mut pc: u32 = 0x826B57B8;
    'dispatch: loop {
        match pc {
            0x826B57B8 => {
    //   block [0x826B57B8..0x826B581C)
	// 826B57B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B57BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B57C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B57C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B57C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B57CC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B57D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B57D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B57D8: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 826B57DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B57E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B57E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B57E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B57EC: 386AEBA4  addi r3, r10, -0x145c
	ctx.r[3].s64 = ctx.r[10].s64 + -5212;
	// 826B57F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B57F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B57F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B57FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5800: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B5804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5808: 4BDB1619  bl 0x82466e20
	ctx.lr = 0x826B580C;
	sub_82466E20(ctx, base);
	// 826B580C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5820 size=112
    let mut pc: u32 = 0x826B5820;
    'dispatch: loop {
        match pc {
            0x826B5820 => {
    //   block [0x826B5820..0x826B5890)
	// 826B5820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B582C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5830: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5834: 38AAE814  addi r5, r10, -0x17ec
	ctx.r[5].s64 = ctx.r[10].s64 + -6124;
	// 826B5838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B583C: 390B9438  addi r8, r11, -0x6bc8
	ctx.r[8].s64 = ctx.r[11].s64 + -27592;
	// 826B5840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5844: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826B5848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B584C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5858: 386AEBD4  addi r3, r10, -0x142c
	ctx.r[3].s64 = ctx.r[10].s64 + -5164;
	// 826B585C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B586C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B587C: 4BDB15A5  bl 0x82466e20
	ctx.lr = 0x826B5880;
	sub_82466E20(ctx, base);
	// 826B5880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B588C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5890 size=112
    let mut pc: u32 = 0x826B5890;
    'dispatch: loop {
        match pc {
            0x826B5890 => {
    //   block [0x826B5890..0x826B5900)
	// 826B5890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B589C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B58A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B58A4: 38AAE664  addi r5, r10, -0x199c
	ctx.r[5].s64 = ctx.r[10].s64 + -6556;
	// 826B58A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B58AC: 390B9468  addi r8, r11, -0x6b98
	ctx.r[8].s64 = ctx.r[11].s64 + -27544;
	// 826B58B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B58B4: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826B58B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B58BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B58C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B58C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B58C8: 386AEC04  addi r3, r10, -0x13fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5116;
	// 826B58CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B58D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B58D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B58D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B58DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B58E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B58E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B58E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B58EC: 4BDB1535  bl 0x82466e20
	ctx.lr = 0x826B58F0;
	sub_82466E20(ctx, base);
	// 826B58F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B58F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B58F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B58FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5900 size=108
    let mut pc: u32 = 0x826B5900;
    'dispatch: loop {
        match pc {
            0x826B5900 => {
    //   block [0x826B5900..0x826B596C)
	// 826B5900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B590C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5914: 38EB9480  addi r7, r11, -0x6b80
	ctx.r[7].s64 = ctx.r[11].s64 + -27520;
	// 826B5918: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B591C: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826B5920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B592C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5930: 386AEC34  addi r3, r10, -0x13cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5068;
	// 826B5934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B593C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B594C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5958: 4BDB14C9  bl 0x82466e20
	ctx.lr = 0x826B595C;
	sub_82466E20(ctx, base);
	// 826B595C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5970 size=112
    let mut pc: u32 = 0x826B5970;
    'dispatch: loop {
        match pc {
            0x826B5970 => {
    //   block [0x826B5970..0x826B59E0)
	// 826B5970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B597C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5980: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5984: 38AAEBA4  addi r5, r10, -0x145c
	ctx.r[5].s64 = ctx.r[10].s64 + -5212;
	// 826B5988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B598C: 390B94B0  addi r8, r11, -0x6b50
	ctx.r[8].s64 = ctx.r[11].s64 + -27472;
	// 826B5990: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B5994: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826B5998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B599C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B59A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B59A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B59A8: 386AEC64  addi r3, r10, -0x139c
	ctx.r[3].s64 = ctx.r[10].s64 + -5020;
	// 826B59AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B59B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B59B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B59B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B59BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B59C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B59C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B59C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B59CC: 4BDB1455  bl 0x82466e20
	ctx.lr = 0x826B59D0;
	sub_82466E20(ctx, base);
	// 826B59D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B59D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B59D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B59DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B59E0 size=112
    let mut pc: u32 = 0x826B59E0;
    'dispatch: loop {
        match pc {
            0x826B59E0 => {
    //   block [0x826B59E0..0x826B5A50)
	// 826B59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B59E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B59E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B59EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B59F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B59F4: 392AF914  addi r9, r10, -0x6ec
	ctx.r[9].s64 = ctx.r[10].s64 + -1772;
	// 826B59F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B59FC: 390B9548  addi r8, r11, -0x6ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -27320;
	// 826B5A00: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826B5A04: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 826B5A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5A0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5A18: 386AEC94  addi r3, r10, -0x136c
	ctx.r[3].s64 = ctx.r[10].s64 + -4972;
	// 826B5A1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B5A20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B5A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5A3C: 4BDB13E5  bl 0x82466e20
	ctx.lr = 0x826B5A40;
	sub_82466E20(ctx, base);
	// 826B5A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5A50 size=112
    let mut pc: u32 = 0x826B5A50;
    'dispatch: loop {
        match pc {
            0x826B5A50 => {
    //   block [0x826B5A50..0x826B5AC0)
	// 826B5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5A5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5A60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5A64: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5A6C: 390B9590  addi r8, r11, -0x6a70
	ctx.r[8].s64 = ctx.r[11].s64 + -27248;
	// 826B5A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B5A74: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826B5A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5A7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5A88: 386AECC4  addi r3, r10, -0x133c
	ctx.r[3].s64 = ctx.r[10].s64 + -4924;
	// 826B5A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5AAC: 4BDB1375  bl 0x82466e20
	ctx.lr = 0x826B5AB0;
	sub_82466E20(ctx, base);
	// 826B5AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5AC0 size=108
    let mut pc: u32 = 0x826B5AC0;
    'dispatch: loop {
        match pc {
            0x826B5AC0 => {
    //   block [0x826B5AC0..0x826B5B2C)
	// 826B5AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5ACC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5AD4: 38EB95A8  addi r7, r11, -0x6a58
	ctx.r[7].s64 = ctx.r[11].s64 + -27224;
	// 826B5AD8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B5ADC: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 826B5AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B5AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5AF0: 386AECF4  addi r3, r10, -0x130c
	ctx.r[3].s64 = ctx.r[10].s64 + -4876;
	// 826B5AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5B18: 4BDB1309  bl 0x82466e20
	ctx.lr = 0x826B5B1C;
	sub_82466E20(ctx, base);
	// 826B5B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5B30 size=116
    let mut pc: u32 = 0x826B5B30;
    'dispatch: loop {
        match pc {
            0x826B5B30 => {
    //   block [0x826B5B30..0x826B5BA4)
	// 826B5B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5B3C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B5B40: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826B5B44: 390A9638  addi r8, r10, -0x69c8
	ctx.r[8].s64 = ctx.r[10].s64 + -27080;
	// 826B5B48: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5B4C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B5B50: 38AAEBA4  addi r5, r10, -0x145c
	ctx.r[5].s64 = ctx.r[10].s64 + -5212;
	// 826B5B54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5B58: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B5B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5B64: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826B5B68: 396BF928  addi r11, r11, -0x6d8
	ctx.r[11].s64 = ctx.r[11].s64 + -1752;
	// 826B5B6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5B70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5B74: 386AED24  addi r3, r10, -0x12dc
	ctx.r[3].s64 = ctx.r[10].s64 + -4828;
	// 826B5B78: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B5B7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5B80: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B5B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5B90: 4BDB1291  bl 0x82466e20
	ctx.lr = 0x826B5B94;
	sub_82466E20(ctx, base);
	// 826B5B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5BA8 size=108
    let mut pc: u32 = 0x826B5BA8;
    'dispatch: loop {
        match pc {
            0x826B5BA8 => {
    //   block [0x826B5BA8..0x826B5C14)
	// 826B5BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5BB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5BBC: 38EB9710  addi r7, r11, -0x68f0
	ctx.r[7].s64 = ctx.r[11].s64 + -26864;
	// 826B5BC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B5BC4: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826B5BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B5BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5BD8: 386AED54  addi r3, r10, -0x12ac
	ctx.r[3].s64 = ctx.r[10].s64 + -4780;
	// 826B5BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5C00: 4BDB1221  bl 0x82466e20
	ctx.lr = 0x826B5C04;
	sub_82466E20(ctx, base);
	// 826B5C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5C18 size=112
    let mut pc: u32 = 0x826B5C18;
    'dispatch: loop {
        match pc {
            0x826B5C18 => {
    //   block [0x826B5C18..0x826B5C88)
	// 826B5C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5C24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5C28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5C2C: 38AAEBA4  addi r5, r10, -0x145c
	ctx.r[5].s64 = ctx.r[10].s64 + -5212;
	// 826B5C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5C34: 390B9758  addi r8, r11, -0x68a8
	ctx.r[8].s64 = ctx.r[11].s64 + -26792;
	// 826B5C38: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B5C3C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826B5C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5C44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5C50: 386AED84  addi r3, r10, -0x127c
	ctx.r[3].s64 = ctx.r[10].s64 + -4732;
	// 826B5C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5C74: 4BDB11AD  bl 0x82466e20
	ctx.lr = 0x826B5C78;
	sub_82466E20(ctx, base);
	// 826B5C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5C88 size=112
    let mut pc: u32 = 0x826B5C88;
    'dispatch: loop {
        match pc {
            0x826B5C88 => {
    //   block [0x826B5C88..0x826B5CF8)
	// 826B5C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5C94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5C98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5C9C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5CA4: 390B97D0  addi r8, r11, -0x6830
	ctx.r[8].s64 = ctx.r[11].s64 + -26672;
	// 826B5CA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5CAC: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826B5CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5CB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5CC0: 386AEDB4  addi r3, r10, -0x124c
	ctx.r[3].s64 = ctx.r[10].s64 + -4684;
	// 826B5CC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5CE4: 4BDB113D  bl 0x82466e20
	ctx.lr = 0x826B5CE8;
	sub_82466E20(ctx, base);
	// 826B5CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5CF8 size=108
    let mut pc: u32 = 0x826B5CF8;
    'dispatch: loop {
        match pc {
            0x826B5CF8 => {
    //   block [0x826B5CF8..0x826B5D64)
	// 826B5CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5D04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5D0C: 38EB9800  addi r7, r11, -0x6800
	ctx.r[7].s64 = ctx.r[11].s64 + -26624;
	// 826B5D10: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B5D14: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826B5D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B5D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5D28: 386AEDE4  addi r3, r10, -0x121c
	ctx.r[3].s64 = ctx.r[10].s64 + -4636;
	// 826B5D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B5D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B5D50: 4BDB10D1  bl 0x82466e20
	ctx.lr = 0x826B5D54;
	sub_82466E20(ctx, base);
	// 826B5D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5D68 size=112
    let mut pc: u32 = 0x826B5D68;
    'dispatch: loop {
        match pc {
            0x826B5D68 => {
    //   block [0x826B5D68..0x826B5DD8)
	// 826B5D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5D74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5D78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5D7C: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B5D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5D84: 390B9878  addi r8, r11, -0x6788
	ctx.r[8].s64 = ctx.r[11].s64 + -26504;
	// 826B5D88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B5D8C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826B5D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5DA0: 386AEE14  addi r3, r10, -0x11ec
	ctx.r[3].s64 = ctx.r[10].s64 + -4588;
	// 826B5DA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5DC4: 4BDB105D  bl 0x82466e20
	ctx.lr = 0x826B5DC8;
	sub_82466E20(ctx, base);
	// 826B5DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B5DD8 size=24
    let mut pc: u32 = 0x826B5DD8;
    'dispatch: loop {
        match pc {
            0x826B5DD8 => {
    //   block [0x826B5DD8..0x826B5DF0)
	// 826B5DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5DDC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B5DE0: 394AC198  addi r10, r10, -0x3e68
	ctx.r[10].s64 = ctx.r[10].s64 + -15976;
	// 826B5DE4: 816B9544  lwz r11, -0x6abc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27324 as u32) ) } as u64;
	// 826B5DE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B5DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5DF0 size=116
    let mut pc: u32 = 0x826B5DF0;
    'dispatch: loop {
        match pc {
            0x826B5DF0 => {
    //   block [0x826B5DF0..0x826B5E64)
	// 826B5DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5DFC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5E00: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B5E04: 390BC198  addi r8, r11, -0x3e68
	ctx.r[8].s64 = ctx.r[11].s64 + -15976;
	// 826B5E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5E0C: 392AF984  addi r9, r10, -0x67c
	ctx.r[9].s64 = ctx.r[10].s64 + -1660;
	// 826B5E10: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5E14: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B5E18: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B5E1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5E24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5E34: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B5E38: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826B5E3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B5E40: 386BEE44  addi r3, r11, -0x11bc
	ctx.r[3].s64 = ctx.r[11].s64 + -4540;
	// 826B5E44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B5E48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5E50: 4BDB0FD1  bl 0x82466e20
	ctx.lr = 0x826B5E54;
	sub_82466E20(ctx, base);
	// 826B5E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5E68 size=112
    let mut pc: u32 = 0x826B5E68;
    'dispatch: loop {
        match pc {
            0x826B5E68 => {
    //   block [0x826B5E68..0x826B5ED8)
	// 826B5E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5E74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5E78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5E7C: 38AAEE44  addi r5, r10, -0x11bc
	ctx.r[5].s64 = ctx.r[10].s64 + -4540;
	// 826B5E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5E84: 390B98C0  addi r8, r11, -0x6740
	ctx.r[8].s64 = ctx.r[11].s64 + -26432;
	// 826B5E88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5E8C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 826B5E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5E94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5EA0: 386AEE74  addi r3, r10, -0x118c
	ctx.r[3].s64 = ctx.r[10].s64 + -4492;
	// 826B5EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5EC4: 4BDB0F5D  bl 0x82466e20
	ctx.lr = 0x826B5EC8;
	sub_82466E20(ctx, base);
	// 826B5EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5ED8 size=112
    let mut pc: u32 = 0x826B5ED8;
    'dispatch: loop {
        match pc {
            0x826B5ED8 => {
    //   block [0x826B5ED8..0x826B5F48)
	// 826B5ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5EE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5EEC: 38AAEE74  addi r5, r10, -0x118c
	ctx.r[5].s64 = ctx.r[10].s64 + -4492;
	// 826B5EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5EF4: 390B98F0  addi r8, r11, -0x6710
	ctx.r[8].s64 = ctx.r[11].s64 + -26384;
	// 826B5EF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B5EFC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826B5F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5F10: 386AEEA4  addi r3, r10, -0x115c
	ctx.r[3].s64 = ctx.r[10].s64 + -4444;
	// 826B5F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5F34: 4BDB0EED  bl 0x82466e20
	ctx.lr = 0x826B5F38;
	sub_82466E20(ctx, base);
	// 826B5F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5F48 size=112
    let mut pc: u32 = 0x826B5F48;
    'dispatch: loop {
        match pc {
            0x826B5F48 => {
    //   block [0x826B5F48..0x826B5FB8)
	// 826B5F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5F54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5F58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5F5C: 38AAEE74  addi r5, r10, -0x118c
	ctx.r[5].s64 = ctx.r[10].s64 + -4492;
	// 826B5F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5F64: 390B9950  addi r8, r11, -0x66b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26288;
	// 826B5F68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B5F6C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826B5F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5F74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5F80: 386AEED4  addi r3, r10, -0x112c
	ctx.r[3].s64 = ctx.r[10].s64 + -4396;
	// 826B5F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B5F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B5F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B5F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B5F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B5FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B5FA4: 4BDB0E7D  bl 0x82466e20
	ctx.lr = 0x826B5FA8;
	sub_82466E20(ctx, base);
	// 826B5FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B5FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B5FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B5FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B5FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B5FB8 size=112
    let mut pc: u32 = 0x826B5FB8;
    'dispatch: loop {
        match pc {
            0x826B5FB8 => {
    //   block [0x826B5FB8..0x826B6028)
	// 826B5FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B5FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B5FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B5FC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5FC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B5FCC: 38AAEE74  addi r5, r10, -0x118c
	ctx.r[5].s64 = ctx.r[10].s64 + -4492;
	// 826B5FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B5FD4: 390B9980  addi r8, r11, -0x6680
	ctx.r[8].s64 = ctx.r[11].s64 + -26240;
	// 826B5FD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B5FDC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826B5FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B5FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B5FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B5FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B5FF0: 386AEF04  addi r3, r10, -0x10fc
	ctx.r[3].s64 = ctx.r[10].s64 + -4348;
	// 826B5FF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B5FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B5FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B600C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6014: 4BDB0E0D  bl 0x82466e20
	ctx.lr = 0x826B6018;
	sub_82466E20(ctx, base);
	// 826B6018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B601C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6028 size=108
    let mut pc: u32 = 0x826B6028;
    'dispatch: loop {
        match pc {
            0x826B6028 => {
    //   block [0x826B6028..0x826B6094)
	// 826B6028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B602C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6034: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B603C: 38EB99C8  addi r7, r11, -0x6638
	ctx.r[7].s64 = ctx.r[11].s64 + -26168;
	// 826B6040: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B6044: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 826B6048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B604C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B6054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6058: 386AEF34  addi r3, r10, -0x10cc
	ctx.r[3].s64 = ctx.r[10].s64 + -4300;
	// 826B605C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B606C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B607C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6080: 4BDB0DA1  bl 0x82466e20
	ctx.lr = 0x826B6084;
	sub_82466E20(ctx, base);
	// 826B6084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B608C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6098 size=112
    let mut pc: u32 = 0x826B6098;
    'dispatch: loop {
        match pc {
            0x826B6098 => {
    //   block [0x826B6098..0x826B6108)
	// 826B6098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B609C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B60A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B60A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B60A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B60AC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B60B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B60B4: 390B99F8  addi r8, r11, -0x6608
	ctx.r[8].s64 = ctx.r[11].s64 + -26120;
	// 826B60B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B60BC: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826B60C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B60C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B60C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B60CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B60D0: 386AEF64  addi r3, r10, -0x109c
	ctx.r[3].s64 = ctx.r[10].s64 + -4252;
	// 826B60D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B60D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B60DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B60E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B60E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B60E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B60EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B60F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B60F4: 4BDB0D2D  bl 0x82466e20
	ctx.lr = 0x826B60F8;
	sub_82466E20(ctx, base);
	// 826B60F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B60FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6108 size=116
    let mut pc: u32 = 0x826B6108;
    'dispatch: loop {
        match pc {
            0x826B6108 => {
    //   block [0x826B6108..0x826B617C)
	// 826B6108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B610C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6114: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6118: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826B611C: 390A9A10  addi r8, r10, -0x65f0
	ctx.r[8].s64 = ctx.r[10].s64 + -26096;
	// 826B6120: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6124: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B6128: 38AAF3E4  addi r5, r10, -0xc1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3100;
	// 826B612C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6130: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B613C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826B6140: 396BF998  addi r11, r11, -0x668
	ctx.r[11].s64 = ctx.r[11].s64 + -1640;
	// 826B6144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6148: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B614C: 386AEF94  addi r3, r10, -0x106c
	ctx.r[3].s64 = ctx.r[10].s64 + -4204;
	// 826B6150: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B6154: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6158: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B615C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6168: 4BDB0CB9  bl 0x82466e20
	ctx.lr = 0x826B616C;
	sub_82466E20(ctx, base);
	// 826B616C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6180 size=100
    let mut pc: u32 = 0x826B6180;
    'dispatch: loop {
        match pc {
            0x826B6180 => {
    //   block [0x826B6180..0x826B61E4)
	// 826B6180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B618C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6194: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B619C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B61A0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826B61A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B61A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B61AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B61B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B61B4: 386AEFC4  addi r3, r10, -0x103c
	ctx.r[3].s64 = ctx.r[10].s64 + -4156;
	// 826B61B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B61BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B61C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B61C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B61C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B61CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B61D0: 4BDB0C51  bl 0x82466e20
	ctx.lr = 0x826B61D4;
	sub_82466E20(ctx, base);
	// 826B61D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B61D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B61DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B61E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B61E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B61E8 size=100
    let mut pc: u32 = 0x826B61E8;
    'dispatch: loop {
        match pc {
            0x826B61E8 => {
    //   block [0x826B61E8..0x826B624C)
	// 826B61E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B61EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B61F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B61F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B61F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B61FC: 38AAF054  addi r5, r10, -0xfac
	ctx.r[5].s64 = ctx.r[10].s64 + -4012;
	// 826B6200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6208: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826B620C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B621C: 386AEFF4  addi r3, r10, -0x100c
	ctx.r[3].s64 = ctx.r[10].s64 + -4108;
	// 826B6220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6224: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B622C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6238: 4BDB0BE9  bl 0x82466e20
	ctx.lr = 0x826B623C;
	sub_82466E20(ctx, base);
	// 826B623C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6250 size=100
    let mut pc: u32 = 0x826B6250;
    'dispatch: loop {
        match pc {
            0x826B6250 => {
    //   block [0x826B6250..0x826B62B4)
	// 826B6250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B625C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6264: 38AAEF94  addi r5, r10, -0x106c
	ctx.r[5].s64 = ctx.r[10].s64 + -4204;
	// 826B6268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B626C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6270: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826B6274: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B627C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6284: 386AF024  addi r3, r10, -0xfdc
	ctx.r[3].s64 = ctx.r[10].s64 + -4060;
	// 826B6288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B628C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6290: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B6294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B629C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B62A0: 4BDB0B81  bl 0x82466e20
	ctx.lr = 0x826B62A4;
	sub_82466E20(ctx, base);
	// 826B62A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B62A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B62AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B62B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B62B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B62B8 size=104
    let mut pc: u32 = 0x826B62B8;
    'dispatch: loop {
        match pc {
            0x826B62B8 => {
    //   block [0x826B62B8..0x826B6320)
	// 826B62B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B62BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B62C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B62C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B62C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B62CC: 392AF9FC  addi r9, r10, -0x604
	ctx.r[9].s64 = ctx.r[10].s64 + -1540;
	// 826B62D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B62D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B62D8: 38AAEFC4  addi r5, r10, -0x103c
	ctx.r[5].s64 = ctx.r[10].s64 + -4156;
	// 826B62DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B62E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B62E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B62E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B62EC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826B62F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B62F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B62F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B62FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6304: 386AF054  addi r3, r10, -0xfac
	ctx.r[3].s64 = ctx.r[10].s64 + -4012;
	// 826B6308: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B630C: 4BDB0B15  bl 0x82466e20
	ctx.lr = 0x826B6310;
	sub_82466E20(ctx, base);
	// 826B6310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B631C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6320 size=108
    let mut pc: u32 = 0x826B6320;
    'dispatch: loop {
        match pc {
            0x826B6320 => {
    //   block [0x826B6320..0x826B638C)
	// 826B6320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B632C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6334: 38EB9B94  addi r7, r11, -0x646c
	ctx.r[7].s64 = ctx.r[11].s64 + -25708;
	// 826B6338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B633C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826B6340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6350: 386AF084  addi r3, r10, -0xf7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3964;
	// 826B6354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B635C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B636C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6378: 4BDB0AA9  bl 0x82466e20
	ctx.lr = 0x826B637C;
	sub_82466E20(ctx, base);
	// 826B637C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6390 size=112
    let mut pc: u32 = 0x826B6390;
    'dispatch: loop {
        match pc {
            0x826B6390 => {
    //   block [0x826B6390..0x826B6400)
	// 826B6390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B639C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B63A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B63A4: 38AAF054  addi r5, r10, -0xfac
	ctx.r[5].s64 = ctx.r[10].s64 + -4012;
	// 826B63A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B63AC: 390B9BC8  addi r8, r11, -0x6438
	ctx.r[8].s64 = ctx.r[11].s64 + -25656;
	// 826B63B0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B63B4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826B63B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B63BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B63C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B63C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B63C8: 386AF0B4  addi r3, r10, -0xf4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3916;
	// 826B63CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B63D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B63D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B63D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B63DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B63E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B63E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B63E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B63EC: 4BDB0A35  bl 0x82466e20
	ctx.lr = 0x826B63F0;
	sub_82466E20(ctx, base);
	// 826B63F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B63F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B63F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B63FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B6400 size=24
    let mut pc: u32 = 0x826B6400;
    'dispatch: loop {
        match pc {
            0x826B6400 => {
    //   block [0x826B6400..0x826B6418)
	// 826B6400: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6404: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6408: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 826B640C: 816B9BC4  lwz r11, -0x643c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25660 as u32) ) } as u64;
	// 826B6410: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B6414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6418 size=116
    let mut pc: u32 = 0x826B6418;
    'dispatch: loop {
        match pc {
            0x826B6418 => {
    //   block [0x826B6418..0x826B648C)
	// 826B6418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B641C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6424: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6428: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B642C: 390BC1B0  addi r8, r11, -0x3e50
	ctx.r[8].s64 = ctx.r[11].s64 + -15952;
	// 826B6430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6434: 392AFA60  addi r9, r10, -0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + -1440;
	// 826B6438: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B643C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B6440: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6444: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B644C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B645C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B6460: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826B6464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6468: 386BF0E4  addi r3, r11, -0xf1c
	ctx.r[3].s64 = ctx.r[11].s64 + -3868;
	// 826B646C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6470: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6478: 4BDB09A9  bl 0x82466e20
	ctx.lr = 0x826B647C;
	sub_82466E20(ctx, base);
	// 826B647C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6490 size=100
    let mut pc: u32 = 0x826B6490;
    'dispatch: loop {
        match pc {
            0x826B6490 => {
    //   block [0x826B6490..0x826B64F4)
	// 826B6490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B649C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B64A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B64A4: 38AAF0E4  addi r5, r10, -0xf1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3868;
	// 826B64A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B64AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B64B0: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826B64B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B64B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B64BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B64C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B64C4: 386AF114  addi r3, r10, -0xeec
	ctx.r[3].s64 = ctx.r[10].s64 + -3820;
	// 826B64C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B64CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B64D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B64D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B64D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B64DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B64E0: 4BDB0941  bl 0x82466e20
	ctx.lr = 0x826B64E4;
	sub_82466E20(ctx, base);
	// 826B64E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B64E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B64EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B64F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B64F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B64F8 size=100
    let mut pc: u32 = 0x826B64F8;
    'dispatch: loop {
        match pc {
            0x826B64F8 => {
    //   block [0x826B64F8..0x826B655C)
	// 826B64F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B64FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B650C: 38AAF174  addi r5, r10, -0xe8c
	ctx.r[5].s64 = ctx.r[10].s64 + -3724;
	// 826B6510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6518: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826B651C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B652C: 386AF144  addi r3, r10, -0xebc
	ctx.r[3].s64 = ctx.r[10].s64 + -3772;
	// 826B6530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6538: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B653C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6540: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6548: 4BDB08D9  bl 0x82466e20
	ctx.lr = 0x826B654C;
	sub_82466E20(ctx, base);
	// 826B654C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6560 size=112
    let mut pc: u32 = 0x826B6560;
    'dispatch: loop {
        match pc {
            0x826B6560 => {
    //   block [0x826B6560..0x826B65D0)
	// 826B6560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B656C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6570: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6574: 38AAF0E4  addi r5, r10, -0xf1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3868;
	// 826B6578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B657C: 390B9C70  addi r8, r11, -0x6390
	ctx.r[8].s64 = ctx.r[11].s64 + -25488;
	// 826B6580: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B6584: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826B6588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B658C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6598: 386AF174  addi r3, r10, -0xe8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3724;
	// 826B659C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B65A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B65A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B65A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B65AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B65B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B65B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B65B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B65BC: 4BDB0865  bl 0x82466e20
	ctx.lr = 0x826B65C0;
	sub_82466E20(ctx, base);
	// 826B65C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B65C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B65C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B65CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B65D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B65D0 size=100
    let mut pc: u32 = 0x826B65D0;
    'dispatch: loop {
        match pc {
            0x826B65D0 => {
    //   block [0x826B65D0..0x826B6634)
	// 826B65D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B65D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B65D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B65DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B65E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B65E4: 38AAF174  addi r5, r10, -0xe8c
	ctx.r[5].s64 = ctx.r[10].s64 + -3724;
	// 826B65E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B65EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B65F0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826B65F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B65F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B65FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6604: 386AF1A4  addi r3, r10, -0xe5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3676;
	// 826B6608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B660C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6610: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B6614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6618: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B661C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6620: 4BDB0801  bl 0x82466e20
	ctx.lr = 0x826B6624;
	sub_82466E20(ctx, base);
	// 826B6624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B662C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6638 size=100
    let mut pc: u32 = 0x826B6638;
    'dispatch: loop {
        match pc {
            0x826B6638 => {
    //   block [0x826B6638..0x826B669C)
	// 826B6638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B663C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6644: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B664C: 38AAF0E4  addi r5, r10, -0xf1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3868;
	// 826B6650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6658: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826B665C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B666C: 386AF1D4  addi r3, r10, -0xe2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3628;
	// 826B6670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6678: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B667C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6680: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6688: 4BDB0799  bl 0x82466e20
	ctx.lr = 0x826B668C;
	sub_82466E20(ctx, base);
	// 826B668C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B66A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B66A0 size=100
    let mut pc: u32 = 0x826B66A0;
    'dispatch: loop {
        match pc {
            0x826B66A0 => {
    //   block [0x826B66A0..0x826B6704)
	// 826B66A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B66A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B66A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B66AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B66B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B66B4: 38AAF114  addi r5, r10, -0xeec
	ctx.r[5].s64 = ctx.r[10].s64 + -3820;
	// 826B66B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B66BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B66C0: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826B66C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B66C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B66CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B66D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B66D4: 386AF204  addi r3, r10, -0xdfc
	ctx.r[3].s64 = ctx.r[10].s64 + -3580;
	// 826B66D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B66DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B66E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B66E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B66E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B66EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B66F0: 4BDB0731  bl 0x82466e20
	ctx.lr = 0x826B66F4;
	sub_82466E20(ctx, base);
	// 826B66F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B66F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B66FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6708 size=100
    let mut pc: u32 = 0x826B6708;
    'dispatch: loop {
        match pc {
            0x826B6708 => {
    //   block [0x826B6708..0x826B676C)
	// 826B6708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B671C: 38AAF1D4  addi r5, r10, -0xe2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3628;
	// 826B6720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6728: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826B672C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B673C: 386AF234  addi r3, r10, -0xdcc
	ctx.r[3].s64 = ctx.r[10].s64 + -3532;
	// 826B6740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B674C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6758: 4BDB06C9  bl 0x82466e20
	ctx.lr = 0x826B675C;
	sub_82466E20(ctx, base);
	// 826B675C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6770 size=100
    let mut pc: u32 = 0x826B6770;
    'dispatch: loop {
        match pc {
            0x826B6770 => {
    //   block [0x826B6770..0x826B67D4)
	// 826B6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B677C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6784: 38AAF114  addi r5, r10, -0xeec
	ctx.r[5].s64 = ctx.r[10].s64 + -3820;
	// 826B6788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B678C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6790: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826B6794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B679C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B67A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B67A4: 386AF264  addi r3, r10, -0xd9c
	ctx.r[3].s64 = ctx.r[10].s64 + -3484;
	// 826B67A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B67AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B67B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B67B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B67B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B67BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B67C0: 4BDB0661  bl 0x82466e20
	ctx.lr = 0x826B67C4;
	sub_82466E20(ctx, base);
	// 826B67C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B67C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B67CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B67D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B67D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B67D8 size=112
    let mut pc: u32 = 0x826B67D8;
    'dispatch: loop {
        match pc {
            0x826B67D8 => {
    //   block [0x826B67D8..0x826B6848)
	// 826B67D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B67DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B67E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B67E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B67E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B67EC: 38AAF2F4  addi r5, r10, -0xd0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3340;
	// 826B67F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B67F4: 390B9CA0  addi r8, r11, -0x6360
	ctx.r[8].s64 = ctx.r[11].s64 + -25440;
	// 826B67F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B67FC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826B6800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B680C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6810: 386AF294  addi r3, r10, -0xd6c
	ctx.r[3].s64 = ctx.r[10].s64 + -3436;
	// 826B6814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B681C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B682C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6834: 4BDB05ED  bl 0x82466e20
	ctx.lr = 0x826B6838;
	sub_82466E20(ctx, base);
	// 826B6838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B683C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6848 size=112
    let mut pc: u32 = 0x826B6848;
    'dispatch: loop {
        match pc {
            0x826B6848 => {
    //   block [0x826B6848..0x826B68B8)
	// 826B6848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6858: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B685C: 38AAF324  addi r5, r10, -0xcdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3292;
	// 826B6860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6864: 390B9CD0  addi r8, r11, -0x6330
	ctx.r[8].s64 = ctx.r[11].s64 + -25392;
	// 826B6868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B686C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826B6870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B687C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6880: 386AF2C4  addi r3, r10, -0xd3c
	ctx.r[3].s64 = ctx.r[10].s64 + -3388;
	// 826B6884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B688C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B689C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B68A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B68A4: 4BDB057D  bl 0x82466e20
	ctx.lr = 0x826B68A8;
	sub_82466E20(ctx, base);
	// 826B68A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B68AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B68B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B68B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B68B8 size=112
    let mut pc: u32 = 0x826B68B8;
    'dispatch: loop {
        match pc {
            0x826B68B8 => {
    //   block [0x826B68B8..0x826B6928)
	// 826B68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B68C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B68C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B68C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B68CC: 38AAF3E4  addi r5, r10, -0xc1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3100;
	// 826B68D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B68D4: 390B9CE8  addi r8, r11, -0x6318
	ctx.r[8].s64 = ctx.r[11].s64 + -25368;
	// 826B68D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B68DC: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826B68E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B68E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B68E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B68EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B68F0: 386AF2F4  addi r3, r10, -0xd0c
	ctx.r[3].s64 = ctx.r[10].s64 + -3340;
	// 826B68F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B68F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B68FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B690C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6914: 4BDB050D  bl 0x82466e20
	ctx.lr = 0x826B6918;
	sub_82466E20(ctx, base);
	// 826B6918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B691C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6928 size=112
    let mut pc: u32 = 0x826B6928;
    'dispatch: loop {
        match pc {
            0x826B6928 => {
    //   block [0x826B6928..0x826B6998)
	// 826B6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6938: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B693C: 38AAF2F4  addi r5, r10, -0xd0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3340;
	// 826B6940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6944: 390B9D18  addi r8, r11, -0x62e8
	ctx.r[8].s64 = ctx.r[11].s64 + -25320;
	// 826B6948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B694C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826B6950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B695C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6960: 386AF324  addi r3, r10, -0xcdc
	ctx.r[3].s64 = ctx.r[10].s64 + -3292;
	// 826B6964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B696C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B697C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6984: 4BDB049D  bl 0x82466e20
	ctx.lr = 0x826B6988;
	sub_82466E20(ctx, base);
	// 826B6988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B698C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6998 size=112
    let mut pc: u32 = 0x826B6998;
    'dispatch: loop {
        match pc {
            0x826B6998 => {
    //   block [0x826B6998..0x826B6A08)
	// 826B6998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B69A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B69A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B69A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B69AC: 38AAF324  addi r5, r10, -0xcdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3292;
	// 826B69B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B69B4: 390B9D30  addi r8, r11, -0x62d0
	ctx.r[8].s64 = ctx.r[11].s64 + -25296;
	// 826B69B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B69BC: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826B69C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B69C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B69C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B69CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B69D0: 386AF354  addi r3, r10, -0xcac
	ctx.r[3].s64 = ctx.r[10].s64 + -3244;
	// 826B69D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B69D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B69DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B69E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B69E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B69E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B69EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B69F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B69F4: 4BDB042D  bl 0x82466e20
	ctx.lr = 0x826B69F8;
	sub_82466E20(ctx, base);
	// 826B69F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B69FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6A08 size=116
    let mut pc: u32 = 0x826B6A08;
    'dispatch: loop {
        match pc {
            0x826B6A08 => {
    //   block [0x826B6A08..0x826B6A7C)
	// 826B6A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6A14: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6A18: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B6A1C: 390A9D48  addi r8, r10, -0x62b8
	ctx.r[8].s64 = ctx.r[10].s64 + -25272;
	// 826B6A20: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6A24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B6A28: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6A2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6A30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6A3C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826B6A40: 396BFA74  addi r11, r11, -0x58c
	ctx.r[11].s64 = ctx.r[11].s64 + -1420;
	// 826B6A44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6A48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6A4C: 386AF384  addi r3, r10, -0xc7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3196;
	// 826B6A50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B6A54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6A58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B6A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6A68: 4BDB03B9  bl 0x82466e20
	ctx.lr = 0x826B6A6C;
	sub_82466E20(ctx, base);
	// 826B6A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B6A80 size=48
    let mut pc: u32 = 0x826B6A80;
    'dispatch: loop {
        match pc {
            0x826B6A80 => {
    //   block [0x826B6A80..0x826B6AB0)
	// 826B6A80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6A84: 814B9DFC  lwz r10, -0x6204(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25092 as u32) ) } as u64;
	// 826B6A88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6A8C: 396BC228  addi r11, r11, -0x3dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -15832;
	// 826B6A90: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826B6A94: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6A98: 814A9DF8  lwz r10, -0x6208(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25096 as u32) ) } as u64;
	// 826B6A9C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826B6AA0: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B6AA4: 814A9DF4  lwz r10, -0x620c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25100 as u32) ) } as u64;
	// 826B6AA8: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826B6AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6AB0 size=116
    let mut pc: u32 = 0x826B6AB0;
    'dispatch: loop {
        match pc {
            0x826B6AB0 => {
    //   block [0x826B6AB0..0x826B6B24)
	// 826B6AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6ABC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B6AC0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6AC4: 392BFB48  addi r9, r11, -0x4b8
	ctx.r[9].s64 = ctx.r[11].s64 + -1208;
	// 826B6AC8: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6ACC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6AD0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826B6AD4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826B6AD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6ADC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826B6AE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6AE4: 396BC228  addi r11, r11, -0x3dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -15832;
	// 826B6AE8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B6AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6AF0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B6AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6AF8: 386AF3B4  addi r3, r10, -0xc4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3148;
	// 826B6AFC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826B6B00: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B6B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6B08: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B6B0C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B6B10: 4BDB0311  bl 0x82466e20
	ctx.lr = 0x826B6B14;
	sub_82466E20(ctx, base);
	// 826B6B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6B28 size=116
    let mut pc: u32 = 0x826B6B28;
    'dispatch: loop {
        match pc {
            0x826B6B28 => {
    //   block [0x826B6B28..0x826B6B9C)
	// 826B6B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6B34: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6B38: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6B3C: 390B9E08  addi r8, r11, -0x61f8
	ctx.r[8].s64 = ctx.r[11].s64 + -25080;
	// 826B6B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6B44: 392AFC70  addi r9, r10, -0x390
	ctx.r[9].s64 = ctx.r[10].s64 + -912;
	// 826B6B48: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6B4C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B6B50: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6B54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6B5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6B6C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B6B70: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826B6B74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6B78: 386BF3E4  addi r3, r11, -0xc1c
	ctx.r[3].s64 = ctx.r[11].s64 + -3100;
	// 826B6B7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6B80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6B88: 4BDB0299  bl 0x82466e20
	ctx.lr = 0x826B6B8C;
	sub_82466E20(ctx, base);
	// 826B6B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6BA0 size=112
    let mut pc: u32 = 0x826B6BA0;
    'dispatch: loop {
        match pc {
            0x826B6BA0 => {
    //   block [0x826B6BA0..0x826B6C10)
	// 826B6BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6BAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6BB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6BB4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6BBC: 390B9E98  addi r8, r11, -0x6168
	ctx.r[8].s64 = ctx.r[11].s64 + -24936;
	// 826B6BC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B6BC4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826B6BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6BD8: 386AF414  addi r3, r10, -0xbec
	ctx.r[3].s64 = ctx.r[10].s64 + -3052;
	// 826B6BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6BFC: 4BDB0225  bl 0x82466e20
	ctx.lr = 0x826B6C00;
	sub_82466E20(ctx, base);
	// 826B6C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6C10 size=112
    let mut pc: u32 = 0x826B6C10;
    'dispatch: loop {
        match pc {
            0x826B6C10 => {
    //   block [0x826B6C10..0x826B6C80)
	// 826B6C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6C1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6C20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6C24: 38AAD764  addi r5, r10, -0x289c
	ctx.r[5].s64 = ctx.r[10].s64 + -10396;
	// 826B6C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6C2C: 390B9EB0  addi r8, r11, -0x6150
	ctx.r[8].s64 = ctx.r[11].s64 + -24912;
	// 826B6C30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B6C34: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826B6C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6C40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6C48: 386AF444  addi r3, r10, -0xbbc
	ctx.r[3].s64 = ctx.r[10].s64 + -3004;
	// 826B6C4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6C6C: 4BDB01B5  bl 0x82466e20
	ctx.lr = 0x826B6C70;
	sub_82466E20(ctx, base);
	// 826B6C70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6C80 size=108
    let mut pc: u32 = 0x826B6C80;
    'dispatch: loop {
        match pc {
            0x826B6C80 => {
    //   block [0x826B6C80..0x826B6CEC)
	// 826B6C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6C8C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6C94: 38EB9EC8  addi r7, r11, -0x6138
	ctx.r[7].s64 = ctx.r[11].s64 + -24888;
	// 826B6C98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B6C9C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826B6CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B6CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6CB0: 386AF474  addi r3, r10, -0xb8c
	ctx.r[3].s64 = ctx.r[10].s64 + -2956;
	// 826B6CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6CD8: 4BDB0149  bl 0x82466e20
	ctx.lr = 0x826B6CDC;
	sub_82466E20(ctx, base);
	// 826B6CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6CF0 size=112
    let mut pc: u32 = 0x826B6CF0;
    'dispatch: loop {
        match pc {
            0x826B6CF0 => {
    //   block [0x826B6CF0..0x826B6D60)
	// 826B6CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6CFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6D00: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6D04: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6D0C: 390B9EE0  addi r8, r11, -0x6120
	ctx.r[8].s64 = ctx.r[11].s64 + -24864;
	// 826B6D10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B6D14: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826B6D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6D20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6D28: 386AF4A4  addi r3, r10, -0xb5c
	ctx.r[3].s64 = ctx.r[10].s64 + -2908;
	// 826B6D2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6D4C: 4BDB00D5  bl 0x82466e20
	ctx.lr = 0x826B6D50;
	sub_82466E20(ctx, base);
	// 826B6D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6D60 size=108
    let mut pc: u32 = 0x826B6D60;
    'dispatch: loop {
        match pc {
            0x826B6D60 => {
    //   block [0x826B6D60..0x826B6DCC)
	// 826B6D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6D6C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6D74: 38EB9F28  addi r7, r11, -0x60d8
	ctx.r[7].s64 = ctx.r[11].s64 + -24792;
	// 826B6D78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B6D7C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826B6D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6D84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6D88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B6D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6D90: 386AF4D4  addi r3, r10, -0xb2c
	ctx.r[3].s64 = ctx.r[10].s64 + -2860;
	// 826B6D94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B6D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6DB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6DB8: 4BDB0069  bl 0x82466e20
	ctx.lr = 0x826B6DBC;
	sub_82466E20(ctx, base);
	// 826B6DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6DD0 size=112
    let mut pc: u32 = 0x826B6DD0;
    'dispatch: loop {
        match pc {
            0x826B6DD0 => {
    //   block [0x826B6DD0..0x826B6E40)
	// 826B6DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6DDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6DE0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6DE4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B6DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6DEC: 390B9F40  addi r8, r11, -0x60c0
	ctx.r[8].s64 = ctx.r[11].s64 + -24768;
	// 826B6DF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B6DF4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826B6DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6E08: 386AF504  addi r3, r10, -0xafc
	ctx.r[3].s64 = ctx.r[10].s64 + -2812;
	// 826B6E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6E2C: 4BDAFFF5  bl 0x82466e20
	ctx.lr = 0x826B6E30;
	sub_82466E20(ctx, base);
	// 826B6E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6E40 size=112
    let mut pc: u32 = 0x826B6E40;
    'dispatch: loop {
        match pc {
            0x826B6E40 => {
    //   block [0x826B6E40..0x826B6EB0)
	// 826B6E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6E4C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6E50: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6E54: 392AFCC8  addi r9, r10, -0x338
	ctx.r[9].s64 = ctx.r[10].s64 + -824;
	// 826B6E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6E5C: 390B9F78  addi r8, r11, -0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + -24712;
	// 826B6E60: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B6E64: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826B6E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6E78: 386AF534  addi r3, r10, -0xacc
	ctx.r[3].s64 = ctx.r[10].s64 + -2764;
	// 826B6E7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6E80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6E9C: 4BDAFF85  bl 0x82466e20
	ctx.lr = 0x826B6EA0;
	sub_82466E20(ctx, base);
	// 826B6EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6EB0 size=116
    let mut pc: u32 = 0x826B6EB0;
    'dispatch: loop {
        match pc {
            0x826B6EB0 => {
    //   block [0x826B6EB0..0x826B6F24)
	// 826B6EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6EBC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6EC0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6EC4: 390BA020  addi r8, r11, -0x5fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -24544;
	// 826B6EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6ECC: 392AFC9C  addi r9, r10, -0x364
	ctx.r[9].s64 = ctx.r[10].s64 + -868;
	// 826B6ED0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6ED4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B6ED8: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B6EDC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6EE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6EF4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B6EF8: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826B6EFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6F00: 386BF564  addi r3, r11, -0xa9c
	ctx.r[3].s64 = ctx.r[11].s64 + -2716;
	// 826B6F04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6F08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6F10: 4BDAFF11  bl 0x82466e20
	ctx.lr = 0x826B6F14;
	sub_82466E20(ctx, base);
	// 826B6F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6F28 size=112
    let mut pc: u32 = 0x826B6F28;
    'dispatch: loop {
        match pc {
            0x826B6F28 => {
    //   block [0x826B6F28..0x826B6F98)
	// 826B6F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6F34: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B6F38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6F3C: 392AFCF4  addi r9, r10, -0x30c
	ctx.r[9].s64 = ctx.r[10].s64 + -780;
	// 826B6F40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6F44: 390BA038  addi r8, r11, -0x5fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -24520;
	// 826B6F48: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826B6F4C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826B6F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6F54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6F58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6F60: 386AF594  addi r3, r10, -0xa6c
	ctx.r[3].s64 = ctx.r[10].s64 + -2668;
	// 826B6F64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B6F68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B6F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B6F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6F84: 4BDAFE9D  bl 0x82466e20
	ctx.lr = 0x826B6F88;
	sub_82466E20(ctx, base);
	// 826B6F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B6F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B6F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B6F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B6F98 size=112
    let mut pc: u32 = 0x826B6F98;
    'dispatch: loop {
        match pc {
            0x826B6F98 => {
    //   block [0x826B6F98..0x826B7008)
	// 826B6F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B6F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B6FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B6FA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6FA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B6FAC: 38AAE7B4  addi r5, r10, -0x184c
	ctx.r[5].s64 = ctx.r[10].s64 + -6220;
	// 826B6FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B6FB4: 390BA098  addi r8, r11, -0x5f68
	ctx.r[8].s64 = ctx.r[11].s64 + -24424;
	// 826B6FB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B6FBC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826B6FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B6FC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B6FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B6FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B6FD0: 386AF5C4  addi r3, r10, -0xa3c
	ctx.r[3].s64 = ctx.r[10].s64 + -2620;
	// 826B6FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B6FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B6FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B6FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B6FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B6FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B6FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B6FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B6FF4: 4BDAFE2D  bl 0x82466e20
	ctx.lr = 0x826B6FF8;
	sub_82466E20(ctx, base);
	// 826B6FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B6FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7008 size=112
    let mut pc: u32 = 0x826B7008;
    'dispatch: loop {
        match pc {
            0x826B7008 => {
    //   block [0x826B7008..0x826B7078)
	// 826B7008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7014: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7018: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B701C: 38AAE6C4  addi r5, r10, -0x193c
	ctx.r[5].s64 = ctx.r[10].s64 + -6460;
	// 826B7020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7024: 390BA0B0  addi r8, r11, -0x5f50
	ctx.r[8].s64 = ctx.r[11].s64 + -24400;
	// 826B7028: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B702C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826B7030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B703C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7040: 386AF5F4  addi r3, r10, -0xa0c
	ctx.r[3].s64 = ctx.r[10].s64 + -2572;
	// 826B7044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B704C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B705C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7064: 4BDAFDBD  bl 0x82466e20
	ctx.lr = 0x826B7068;
	sub_82466E20(ctx, base);
	// 826B7068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B706C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7078 size=112
    let mut pc: u32 = 0x826B7078;
    'dispatch: loop {
        match pc {
            0x826B7078 => {
    //   block [0x826B7078..0x826B70E8)
	// 826B7078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B707C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7084: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7088: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B708C: 38AAE6C4  addi r5, r10, -0x193c
	ctx.r[5].s64 = ctx.r[10].s64 + -6460;
	// 826B7090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7094: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 826B7098: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B709C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826B70A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B70A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B70A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B70AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B70B0: 386AF624  addi r3, r10, -0x9dc
	ctx.r[3].s64 = ctx.r[10].s64 + -2524;
	// 826B70B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B70B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B70BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B70C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B70C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B70C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B70CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B70D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B70D4: 4BDAFD4D  bl 0x82466e20
	ctx.lr = 0x826B70D8;
	sub_82466E20(ctx, base);
	// 826B70D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B70DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B70E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B70E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B70E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B70E8 size=112
    let mut pc: u32 = 0x826B70E8;
    'dispatch: loop {
        match pc {
            0x826B70E8 => {
    //   block [0x826B70E8..0x826B7158)
	// 826B70E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B70EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B70F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B70F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B70F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B70FC: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7104: 390BA158  addi r8, r11, -0x5ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -24232;
	// 826B7108: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B710C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826B7110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B711C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7120: 386AF654  addi r3, r10, -0x9ac
	ctx.r[3].s64 = ctx.r[10].s64 + -2476;
	// 826B7124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B712C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B713C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7144: 4BDAFCDD  bl 0x82466e20
	ctx.lr = 0x826B7148;
	sub_82466E20(ctx, base);
	// 826B7148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B714C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7158 size=112
    let mut pc: u32 = 0x826B7158;
    'dispatch: loop {
        match pc {
            0x826B7158 => {
    //   block [0x826B7158..0x826B71C8)
	// 826B7158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B715C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7168: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B716C: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7174: 390BA1B8  addi r8, r11, -0x5e48
	ctx.r[8].s64 = ctx.r[11].s64 + -24136;
	// 826B7178: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B717C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826B7180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B718C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7190: 386AF684  addi r3, r10, -0x97c
	ctx.r[3].s64 = ctx.r[10].s64 + -2428;
	// 826B7194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B719C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B71A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B71A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B71A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B71AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B71B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B71B4: 4BDAFC6D  bl 0x82466e20
	ctx.lr = 0x826B71B8;
	sub_82466E20(ctx, base);
	// 826B71B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B71BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B71C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B71C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B71C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B71C8 size=112
    let mut pc: u32 = 0x826B71C8;
    'dispatch: loop {
        match pc {
            0x826B71C8 => {
    //   block [0x826B71C8..0x826B7238)
	// 826B71C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B71CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B71D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B71D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B71D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B71DC: 38AAE6C4  addi r5, r10, -0x193c
	ctx.r[5].s64 = ctx.r[10].s64 + -6460;
	// 826B71E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B71E4: 390BA218  addi r8, r11, -0x5de8
	ctx.r[8].s64 = ctx.r[11].s64 + -24040;
	// 826B71E8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826B71EC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826B71F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B71F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B71F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B71FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7200: 386AF6B4  addi r3, r10, -0x94c
	ctx.r[3].s64 = ctx.r[10].s64 + -2380;
	// 826B7204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B720C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B721C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7224: 4BDAFBFD  bl 0x82466e20
	ctx.lr = 0x826B7228;
	sub_82466E20(ctx, base);
	// 826B7228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B722C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7238 size=112
    let mut pc: u32 = 0x826B7238;
    'dispatch: loop {
        match pc {
            0x826B7238 => {
    //   block [0x826B7238..0x826B72A8)
	// 826B7238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B723C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7244: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B7248: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826B724C: 38EAA2D8  addi r7, r10, -0x5d28
	ctx.r[7].s64 = ctx.r[10].s64 + -23848;
	// 826B7250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7254: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B7258: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826B725C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7260: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7264: 396BFD08  addi r11, r11, -0x2f8
	ctx.r[11].s64 = ctx.r[11].s64 + -760;
	// 826B7268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B726C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7270: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7274: 386AF6E4  addi r3, r10, -0x91c
	ctx.r[3].s64 = ctx.r[10].s64 + -2332;
	// 826B7278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B727C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B7280: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7284: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B7288: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B728C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7290: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7294: 4BDAFB8D  bl 0x82466e20
	ctx.lr = 0x826B7298;
	sub_82466E20(ctx, base);
	// 826B7298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B729C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B72A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B72A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B72A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B72A8 size=112
    let mut pc: u32 = 0x826B72A8;
    'dispatch: loop {
        match pc {
            0x826B72A8 => {
    //   block [0x826B72A8..0x826B7318)
	// 826B72A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B72AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B72B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B72B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B72B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B72BC: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B72C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B72C4: 390BA4A0  addi r8, r11, -0x5b60
	ctx.r[8].s64 = ctx.r[11].s64 + -23392;
	// 826B72C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B72CC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826B72D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B72D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B72D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B72DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B72E0: 386AF714  addi r3, r10, -0x8ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2284;
	// 826B72E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B72E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B72EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B72F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B72F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B72F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B72FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7304: 4BDAFB1D  bl 0x82466e20
	ctx.lr = 0x826B7308;
	sub_82466E20(ctx, base);
	// 826B7308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B730C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7318 size=112
    let mut pc: u32 = 0x826B7318;
    'dispatch: loop {
        match pc {
            0x826B7318 => {
    //   block [0x826B7318..0x826B7388)
	// 826B7318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7328: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B732C: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B7330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7334: 390BA4B8  addi r8, r11, -0x5b48
	ctx.r[8].s64 = ctx.r[11].s64 + -23368;
	// 826B7338: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B733C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826B7340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B734C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7350: 386AF744  addi r3, r10, -0x8bc
	ctx.r[3].s64 = ctx.r[10].s64 + -2236;
	// 826B7354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B735C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7364: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B7368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B736C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7374: 4BDAFAAD  bl 0x82466e20
	ctx.lr = 0x826B7378;
	sub_82466E20(ctx, base);
	// 826B7378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B737C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7388 size=112
    let mut pc: u32 = 0x826B7388;
    'dispatch: loop {
        match pc {
            0x826B7388 => {
    //   block [0x826B7388..0x826B73F8)
	// 826B7388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7394: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7398: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B739C: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B73A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B73A4: 390BA4D0  addi r8, r11, -0x5b30
	ctx.r[8].s64 = ctx.r[11].s64 + -23344;
	// 826B73A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B73AC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826B73B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B73B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B73B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B73BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B73C0: 386AF774  addi r3, r10, -0x88c
	ctx.r[3].s64 = ctx.r[10].s64 + -2188;
	// 826B73C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B73C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B73CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B73D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B73D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B73D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B73DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B73E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B73E4: 4BDAFA3D  bl 0x82466e20
	ctx.lr = 0x826B73E8;
	sub_82466E20(ctx, base);
	// 826B73E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B73EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B73F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B73F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B73F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B73F8 size=108
    let mut pc: u32 = 0x826B73F8;
    'dispatch: loop {
        match pc {
            0x826B73F8 => {
    //   block [0x826B73F8..0x826B7464)
	// 826B73F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B73FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7404: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B740C: 38EBA500  addi r7, r11, -0x5b00
	ctx.r[7].s64 = ctx.r[11].s64 + -23296;
	// 826B7410: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7414: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826B7418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B741C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7428: 386AF7A4  addi r3, r10, -0x85c
	ctx.r[3].s64 = ctx.r[10].s64 + -2140;
	// 826B742C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B743C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B744C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7450: 4BDAF9D1  bl 0x82466e20
	ctx.lr = 0x826B7454;
	sub_82466E20(ctx, base);
	// 826B7454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B745C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7468 size=112
    let mut pc: u32 = 0x826B7468;
    'dispatch: loop {
        match pc {
            0x826B7468 => {
    //   block [0x826B7468..0x826B74D8)
	// 826B7468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7474: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7478: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B747C: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B7480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7484: 390BA530  addi r8, r11, -0x5ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -23248;
	// 826B7488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B748C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826B7490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B749C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B74A0: 386AF7D4  addi r3, r10, -0x82c
	ctx.r[3].s64 = ctx.r[10].s64 + -2092;
	// 826B74A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B74A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B74AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B74B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B74B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B74B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B74BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B74C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B74C4: 4BDAF95D  bl 0x82466e20
	ctx.lr = 0x826B74C8;
	sub_82466E20(ctx, base);
	// 826B74C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B74CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B74D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B74D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B74D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B74D8 size=108
    let mut pc: u32 = 0x826B74D8;
    'dispatch: loop {
        match pc {
            0x826B74D8 => {
    //   block [0x826B74D8..0x826B7544)
	// 826B74D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B74DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B74E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B74E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B74E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B74EC: 38EBA548  addi r7, r11, -0x5ab8
	ctx.r[7].s64 = ctx.r[11].s64 + -23224;
	// 826B74F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B74F4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826B74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B74FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7508: 386AF804  addi r3, r10, -0x7fc
	ctx.r[3].s64 = ctx.r[10].s64 + -2044;
	// 826B750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7530: 4BDAF8F1  bl 0x82466e20
	ctx.lr = 0x826B7534;
	sub_82466E20(ctx, base);
	// 826B7534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7548 size=108
    let mut pc: u32 = 0x826B7548;
    'dispatch: loop {
        match pc {
            0x826B7548 => {
    //   block [0x826B7548..0x826B75B4)
	// 826B7548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7554: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B755C: 38EBA578  addi r7, r11, -0x5a88
	ctx.r[7].s64 = ctx.r[11].s64 + -23176;
	// 826B7560: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B7564: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826B7568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B756C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7578: 386AF834  addi r3, r10, -0x7cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1996;
	// 826B757C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B758C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B759C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B75A0: 4BDAF881  bl 0x82466e20
	ctx.lr = 0x826B75A4;
	sub_82466E20(ctx, base);
	// 826B75A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B75A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B75AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B75B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B75B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B75B8 size=112
    let mut pc: u32 = 0x826B75B8;
    'dispatch: loop {
        match pc {
            0x826B75B8 => {
    //   block [0x826B75B8..0x826B7628)
	// 826B75B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B75BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B75C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B75C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B75C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B75CC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B75D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B75D4: 390BA5C0  addi r8, r11, -0x5a40
	ctx.r[8].s64 = ctx.r[11].s64 + -23104;
	// 826B75D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B75DC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826B75E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B75E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B75E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B75EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B75F0: 386AF864  addi r3, r10, -0x79c
	ctx.r[3].s64 = ctx.r[10].s64 + -1948;
	// 826B75F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B75F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B75FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B760C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7614: 4BDAF80D  bl 0x82466e20
	ctx.lr = 0x826B7618;
	sub_82466E20(ctx, base);
	// 826B7618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B761C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7628 size=112
    let mut pc: u32 = 0x826B7628;
    'dispatch: loop {
        match pc {
            0x826B7628 => {
    //   block [0x826B7628..0x826B7698)
	// 826B7628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7634: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7638: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B763C: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7644: 390BA608  addi r8, r11, -0x59f8
	ctx.r[8].s64 = ctx.r[11].s64 + -23032;
	// 826B7648: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B764C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826B7650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B765C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7660: 386AF894  addi r3, r10, -0x76c
	ctx.r[3].s64 = ctx.r[10].s64 + -1900;
	// 826B7664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B766C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B767C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7684: 4BDAF79D  bl 0x82466e20
	ctx.lr = 0x826B7688;
	sub_82466E20(ctx, base);
	// 826B7688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B768C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7698 size=108
    let mut pc: u32 = 0x826B7698;
    'dispatch: loop {
        match pc {
            0x826B7698 => {
    //   block [0x826B7698..0x826B7704)
	// 826B7698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B76A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B76A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B76A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B76AC: 38EBA698  addi r7, r11, -0x5968
	ctx.r[7].s64 = ctx.r[11].s64 + -22888;
	// 826B76B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B76B4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826B76B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B76BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B76C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B76C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B76C8: 386AF8C4  addi r3, r10, -0x73c
	ctx.r[3].s64 = ctx.r[10].s64 + -1852;
	// 826B76CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B76D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B76D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B76D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B76DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B76E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B76E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B76E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B76EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B76F0: 4BDAF731  bl 0x82466e20
	ctx.lr = 0x826B76F4;
	sub_82466E20(ctx, base);
	// 826B76F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B76F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B76FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7708 size=108
    let mut pc: u32 = 0x826B7708;
    'dispatch: loop {
        match pc {
            0x826B7708 => {
    //   block [0x826B7708..0x826B7774)
	// 826B7708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B770C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7714: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B771C: 38EBA6E0  addi r7, r11, -0x5920
	ctx.r[7].s64 = ctx.r[11].s64 + -22816;
	// 826B7720: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7724: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826B7728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B772C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7738: 386AF8F4  addi r3, r10, -0x70c
	ctx.r[3].s64 = ctx.r[10].s64 + -1804;
	// 826B773C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B774C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B775C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7760: 4BDAF6C1  bl 0x82466e20
	ctx.lr = 0x826B7764;
	sub_82466E20(ctx, base);
	// 826B7764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B776C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7778 size=108
    let mut pc: u32 = 0x826B7778;
    'dispatch: loop {
        match pc {
            0x826B7778 => {
    //   block [0x826B7778..0x826B77E4)
	// 826B7778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7784: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B778C: 38EBA710  addi r7, r11, -0x58f0
	ctx.r[7].s64 = ctx.r[11].s64 + -22768;
	// 826B7790: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7794: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826B7798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B779C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B77A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B77A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B77A8: 386AF924  addi r3, r10, -0x6dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1756;
	// 826B77AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B77B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B77B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B77B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B77BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B77C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B77C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B77C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B77CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B77D0: 4BDAF651  bl 0x82466e20
	ctx.lr = 0x826B77D4;
	sub_82466E20(ctx, base);
	// 826B77D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B77D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B77DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B77E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B77E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B77E8 size=112
    let mut pc: u32 = 0x826B77E8;
    'dispatch: loop {
        match pc {
            0x826B77E8 => {
    //   block [0x826B77E8..0x826B7858)
	// 826B77E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B77EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B77F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B77F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B77F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B77FC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7804: 390BA740  addi r8, r11, -0x58c0
	ctx.r[8].s64 = ctx.r[11].s64 + -22720;
	// 826B7808: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B780C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826B7810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7814: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B781C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7820: 386AF954  addi r3, r10, -0x6ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1708;
	// 826B7824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B782C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B783C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7844: 4BDAF5DD  bl 0x82466e20
	ctx.lr = 0x826B7848;
	sub_82466E20(ctx, base);
	// 826B7848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B784C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7858 size=112
    let mut pc: u32 = 0x826B7858;
    'dispatch: loop {
        match pc {
            0x826B7858 => {
    //   block [0x826B7858..0x826B78C8)
	// 826B7858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B785C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7868: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B786C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7874: 390BA770  addi r8, r11, -0x5890
	ctx.r[8].s64 = ctx.r[11].s64 + -22672;
	// 826B7878: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B787C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826B7880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7884: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B788C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7890: 386AF984  addi r3, r10, -0x67c
	ctx.r[3].s64 = ctx.r[10].s64 + -1660;
	// 826B7894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B789C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B78A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B78A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B78A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B78AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B78B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B78B4: 4BDAF56D  bl 0x82466e20
	ctx.lr = 0x826B78B8;
	sub_82466E20(ctx, base);
	// 826B78B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B78BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B78C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B78C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B78C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B78C8 size=112
    let mut pc: u32 = 0x826B78C8;
    'dispatch: loop {
        match pc {
            0x826B78C8 => {
    //   block [0x826B78C8..0x826B7938)
	// 826B78C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B78CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B78D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B78D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B78D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B78DC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B78E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B78E4: 390BA788  addi r8, r11, -0x5878
	ctx.r[8].s64 = ctx.r[11].s64 + -22648;
	// 826B78E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B78EC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826B78F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B78F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B78F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B78FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7900: 386AF9B4  addi r3, r10, -0x64c
	ctx.r[3].s64 = ctx.r[10].s64 + -1612;
	// 826B7904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B790C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B791C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7924: 4BDAF4FD  bl 0x82466e20
	ctx.lr = 0x826B7928;
	sub_82466E20(ctx, base);
	// 826B7928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B792C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7938 size=108
    let mut pc: u32 = 0x826B7938;
    'dispatch: loop {
        match pc {
            0x826B7938 => {
    //   block [0x826B7938..0x826B79A4)
	// 826B7938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B793C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7944: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B794C: 38EBA7A0  addi r7, r11, -0x5860
	ctx.r[7].s64 = ctx.r[11].s64 + -22624;
	// 826B7950: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7954: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826B7958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B795C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7968: 386AF9E4  addi r3, r10, -0x61c
	ctx.r[3].s64 = ctx.r[10].s64 + -1564;
	// 826B796C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B797C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B798C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7990: 4BDAF491  bl 0x82466e20
	ctx.lr = 0x826B7994;
	sub_82466E20(ctx, base);
	// 826B7994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B799C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B79A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B79A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B79A8 size=112
    let mut pc: u32 = 0x826B79A8;
    'dispatch: loop {
        match pc {
            0x826B79A8 => {
    //   block [0x826B79A8..0x826B7A18)
	// 826B79A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B79AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B79B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B79B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B79B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B79BC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B79C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B79C4: 390BA7D0  addi r8, r11, -0x5830
	ctx.r[8].s64 = ctx.r[11].s64 + -22576;
	// 826B79C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B79CC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826B79D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B79D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B79D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B79DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B79E0: 386AFA14  addi r3, r10, -0x5ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1516;
	// 826B79E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B79E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B79EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B79F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B79F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B79F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B79FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7A04: 4BDAF41D  bl 0x82466e20
	ctx.lr = 0x826B7A08;
	sub_82466E20(ctx, base);
	// 826B7A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7A18 size=108
    let mut pc: u32 = 0x826B7A18;
    'dispatch: loop {
        match pc {
            0x826B7A18 => {
    //   block [0x826B7A18..0x826B7A84)
	// 826B7A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7A24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7A2C: 38EBA7E8  addi r7, r11, -0x5818
	ctx.r[7].s64 = ctx.r[11].s64 + -22552;
	// 826B7A30: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826B7A34: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826B7A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7A3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7A40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7A48: 386AFA44  addi r3, r10, -0x5bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1468;
	// 826B7A4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7A70: 4BDAF3B1  bl 0x82466e20
	ctx.lr = 0x826B7A74;
	sub_82466E20(ctx, base);
	// 826B7A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7A88 size=112
    let mut pc: u32 = 0x826B7A88;
    'dispatch: loop {
        match pc {
            0x826B7A88 => {
    //   block [0x826B7A88..0x826B7AF8)
	// 826B7A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7A98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7A9C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7AA4: 390BA8D8  addi r8, r11, -0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + -22312;
	// 826B7AA8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 826B7AAC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826B7AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7AB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7AC0: 386AFA74  addi r3, r10, -0x58c
	ctx.r[3].s64 = ctx.r[10].s64 + -1420;
	// 826B7AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7AE4: 4BDAF33D  bl 0x82466e20
	ctx.lr = 0x826B7AE8;
	sub_82466E20(ctx, base);
	// 826B7AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7AF8 size=108
    let mut pc: u32 = 0x826B7AF8;
    'dispatch: loop {
        match pc {
            0x826B7AF8 => {
    //   block [0x826B7AF8..0x826B7B64)
	// 826B7AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7B04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7B0C: 38EBAA88  addi r7, r11, -0x5578
	ctx.r[7].s64 = ctx.r[11].s64 + -21880;
	// 826B7B10: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826B7B14: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826B7B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7B1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7B28: 386AFAA4  addi r3, r10, -0x55c
	ctx.r[3].s64 = ctx.r[10].s64 + -1372;
	// 826B7B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7B50: 4BDAF2D1  bl 0x82466e20
	ctx.lr = 0x826B7B54;
	sub_82466E20(ctx, base);
	// 826B7B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7B68 size=112
    let mut pc: u32 = 0x826B7B68;
    'dispatch: loop {
        match pc {
            0x826B7B68 => {
    //   block [0x826B7B68..0x826B7BD8)
	// 826B7B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7B78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7B7C: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7B84: 390BAC20  addi r8, r11, -0x53e0
	ctx.r[8].s64 = ctx.r[11].s64 + -21472;
	// 826B7B88: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826B7B8C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826B7B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7B94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7BA0: 386AFAD4  addi r3, r10, -0x52c
	ctx.r[3].s64 = ctx.r[10].s64 + -1324;
	// 826B7BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7BC4: 4BDAF25D  bl 0x82466e20
	ctx.lr = 0x826B7BC8;
	sub_82466E20(ctx, base);
	// 826B7BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7BD8 size=100
    let mut pc: u32 = 0x826B7BD8;
    'dispatch: loop {
        match pc {
            0x826B7BD8 => {
    //   block [0x826B7BD8..0x826B7C3C)
	// 826B7BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7BEC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7BF8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826B7BFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7C0C: 386AFB04  addi r3, r10, -0x4fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1276;
	// 826B7C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7C14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7C18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7C20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7C28: 4BDAF1F9  bl 0x82466e20
	ctx.lr = 0x826B7C2C;
	sub_82466E20(ctx, base);
	// 826B7C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7C40 size=112
    let mut pc: u32 = 0x826B7C40;
    'dispatch: loop {
        match pc {
            0x826B7C40 => {
    //   block [0x826B7C40..0x826B7CB0)
	// 826B7C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7C4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7C50: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7C54: 38AAFB04  addi r5, r10, -0x4fc
	ctx.r[5].s64 = ctx.r[10].s64 + -1276;
	// 826B7C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7C5C: 390BAE78  addi r8, r11, -0x5188
	ctx.r[8].s64 = ctx.r[11].s64 + -20872;
	// 826B7C60: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B7C64: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826B7C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7C6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7C70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7C78: 386AFB34  addi r3, r10, -0x4cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1228;
	// 826B7C7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7C9C: 4BDAF185  bl 0x82466e20
	ctx.lr = 0x826B7CA0;
	sub_82466E20(ctx, base);
	// 826B7CA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7CB0 size=100
    let mut pc: u32 = 0x826B7CB0;
    'dispatch: loop {
        match pc {
            0x826B7CB0 => {
    //   block [0x826B7CB0..0x826B7D14)
	// 826B7CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7CBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7CC4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7CD0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826B7CD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7CE4: 386AFB64  addi r3, r10, -0x49c
	ctx.r[3].s64 = ctx.r[10].s64 + -1180;
	// 826B7CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7CEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7CF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7CF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7D00: 4BDAF121  bl 0x82466e20
	ctx.lr = 0x826B7D04;
	sub_82466E20(ctx, base);
	// 826B7D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7D18 size=108
    let mut pc: u32 = 0x826B7D18;
    'dispatch: loop {
        match pc {
            0x826B7D18 => {
    //   block [0x826B7D18..0x826B7D84)
	// 826B7D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7D24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7D2C: 38EBAEF0  addi r7, r11, -0x5110
	ctx.r[7].s64 = ctx.r[11].s64 + -20752;
	// 826B7D30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B7D34: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826B7D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7D3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7D48: 386AFB94  addi r3, r10, -0x46c
	ctx.r[3].s64 = ctx.r[10].s64 + -1132;
	// 826B7D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7D70: 4BDAF0B1  bl 0x82466e20
	ctx.lr = 0x826B7D74;
	sub_82466E20(ctx, base);
	// 826B7D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7D88 size=112
    let mut pc: u32 = 0x826B7D88;
    'dispatch: loop {
        match pc {
            0x826B7D88 => {
    //   block [0x826B7D88..0x826B7DF8)
	// 826B7D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7D98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7D9C: 38AAFB64  addi r5, r10, -0x49c
	ctx.r[5].s64 = ctx.r[10].s64 + -1180;
	// 826B7DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7DA4: 390BAF38  addi r8, r11, -0x50c8
	ctx.r[8].s64 = ctx.r[11].s64 + -20680;
	// 826B7DA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B7DAC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826B7DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7DB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7DC0: 386AFBC4  addi r3, r10, -0x43c
	ctx.r[3].s64 = ctx.r[10].s64 + -1084;
	// 826B7DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7DE4: 4BDAF03D  bl 0x82466e20
	ctx.lr = 0x826B7DE8;
	sub_82466E20(ctx, base);
	// 826B7DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7DF8 size=100
    let mut pc: u32 = 0x826B7DF8;
    'dispatch: loop {
        match pc {
            0x826B7DF8 => {
    //   block [0x826B7DF8..0x826B7E5C)
	// 826B7DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7E04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7E0C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7E18: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826B7E1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7E2C: 386AFBF4  addi r3, r10, -0x40c
	ctx.r[3].s64 = ctx.r[10].s64 + -1036;
	// 826B7E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7E34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7E38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7E40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7E48: 4BDAEFD9  bl 0x82466e20
	ctx.lr = 0x826B7E4C;
	sub_82466E20(ctx, base);
	// 826B7E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7E60 size=100
    let mut pc: u32 = 0x826B7E60;
    'dispatch: loop {
        match pc {
            0x826B7E60 => {
    //   block [0x826B7E60..0x826B7EC4)
	// 826B7E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7E74: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7E80: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826B7E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7E94: 386AFC24  addi r3, r10, -0x3dc
	ctx.r[3].s64 = ctx.r[10].s64 + -988;
	// 826B7E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7EA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7EA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7EB0: 4BDAEF71  bl 0x82466e20
	ctx.lr = 0x826B7EB4;
	sub_82466E20(ctx, base);
	// 826B7EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7EC8 size=112
    let mut pc: u32 = 0x826B7EC8;
    'dispatch: loop {
        match pc {
            0x826B7EC8 => {
    //   block [0x826B7EC8..0x826B7F38)
	// 826B7EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7ED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7ED8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7EDC: 38AAFBF4  addi r5, r10, -0x40c
	ctx.r[5].s64 = ctx.r[10].s64 + -1036;
	// 826B7EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7EE4: 390BAF68  addi r8, r11, -0x5098
	ctx.r[8].s64 = ctx.r[11].s64 + -20632;
	// 826B7EE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B7EEC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826B7EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7F00: 386AFC54  addi r3, r10, -0x3ac
	ctx.r[3].s64 = ctx.r[10].s64 + -940;
	// 826B7F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7F24: 4BDAEEFD  bl 0x82466e20
	ctx.lr = 0x826B7F28;
	sub_82466E20(ctx, base);
	// 826B7F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7F38 size=112
    let mut pc: u32 = 0x826B7F38;
    'dispatch: loop {
        match pc {
            0x826B7F38 => {
    //   block [0x826B7F38..0x826B7FA8)
	// 826B7F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7F48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7F4C: 38AAFC24  addi r5, r10, -0x3dc
	ctx.r[5].s64 = ctx.r[10].s64 + -988;
	// 826B7F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7F54: 390BAFC8  addi r8, r11, -0x5038
	ctx.r[8].s64 = ctx.r[11].s64 + -20536;
	// 826B7F58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B7F5C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826B7F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7F70: 386AFC84  addi r3, r10, -0x37c
	ctx.r[3].s64 = ctx.r[10].s64 + -892;
	// 826B7F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7F94: 4BDAEE8D  bl 0x82466e20
	ctx.lr = 0x826B7F98;
	sub_82466E20(ctx, base);
	// 826B7F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7FA8 size=100
    let mut pc: u32 = 0x826B7FA8;
    'dispatch: loop {
        match pc {
            0x826B7FA8 => {
    //   block [0x826B7FA8..0x826B800C)
	// 826B7FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7FBC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7FC8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826B7FCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7FDC: 386AFCB4  addi r3, r10, -0x34c
	ctx.r[3].s64 = ctx.r[10].s64 + -844;
	// 826B7FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7FE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7FE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7FF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7FF8: 4BDAEE29  bl 0x82466e20
	ctx.lr = 0x826B7FFC;
	sub_82466E20(ctx, base);
	// 826B7FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8010 size=112
    let mut pc: u32 = 0x826B8010;
    'dispatch: loop {
        match pc {
            0x826B8010 => {
    //   block [0x826B8010..0x826B8080)
	// 826B8010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B801C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8020: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8024: 38AAFCB4  addi r5, r10, -0x34c
	ctx.r[5].s64 = ctx.r[10].s64 + -844;
	// 826B8028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B802C: 390BB028  addi r8, r11, -0x4fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -20440;
	// 826B8030: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826B8034: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826B8038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B803C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8048: 386AFCE4  addi r3, r10, -0x31c
	ctx.r[3].s64 = ctx.r[10].s64 + -796;
	// 826B804C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B805C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B806C: 4BDAEDB5  bl 0x82466e20
	ctx.lr = 0x826B8070;
	sub_82466E20(ctx, base);
	// 826B8070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B807C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8080 size=108
    let mut pc: u32 = 0x826B8080;
    'dispatch: loop {
        match pc {
            0x826B8080 => {
    //   block [0x826B8080..0x826B80EC)
	// 826B8080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B808C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8094: 38EBB118  addi r7, r11, -0x4ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -20200;
	// 826B8098: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826B809C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826B80A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B80A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B80A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B80AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B80B0: 386AFD14  addi r3, r10, -0x2ec
	ctx.r[3].s64 = ctx.r[10].s64 + -748;
	// 826B80B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B80B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B80BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B80C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B80C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B80C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B80CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B80D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B80D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B80D8: 4BDAED49  bl 0x82466e20
	ctx.lr = 0x826B80DC;
	sub_82466E20(ctx, base);
	// 826B80DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B80E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B80E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B80E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B80F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B80F0 size=108
    let mut pc: u32 = 0x826B80F0;
    'dispatch: loop {
        match pc {
            0x826B80F0 => {
    //   block [0x826B80F0..0x826B815C)
	// 826B80F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B80F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B80F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B80FC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8104: 38EBB208  addi r7, r11, -0x4df8
	ctx.r[7].s64 = ctx.r[11].s64 + -19960;
	// 826B8108: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B810C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826B8110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B811C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8120: 386AFD44  addi r3, r10, -0x2bc
	ctx.r[3].s64 = ctx.r[10].s64 + -700;
	// 826B8124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B812C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B813C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8148: 4BDAECD9  bl 0x82466e20
	ctx.lr = 0x826B814C;
	sub_82466E20(ctx, base);
	// 826B814C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8160 size=108
    let mut pc: u32 = 0x826B8160;
    'dispatch: loop {
        match pc {
            0x826B8160 => {
    //   block [0x826B8160..0x826B81CC)
	// 826B8160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B816C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8174: 38EBB250  addi r7, r11, -0x4db0
	ctx.r[7].s64 = ctx.r[11].s64 + -19888;
	// 826B8178: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826B817C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826B8180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B818C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8190: 386AFD74  addi r3, r10, -0x28c
	ctx.r[3].s64 = ctx.r[10].s64 + -652;
	// 826B8194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B819C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B81A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B81A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B81A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B81AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B81B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B81B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B81B8: 4BDAEC69  bl 0x82466e20
	ctx.lr = 0x826B81BC;
	sub_82466E20(ctx, base);
	// 826B81BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B81C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B81C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B81C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B81D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B81D0 size=108
    let mut pc: u32 = 0x826B81D0;
    'dispatch: loop {
        match pc {
            0x826B81D0 => {
    //   block [0x826B81D0..0x826B823C)
	// 826B81D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B81D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B81D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B81DC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B81E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B81E4: 38EBB328  addi r7, r11, -0x4cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -19672;
	// 826B81E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B81EC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826B81F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B81F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B81F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B81FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8200: 386AFDA4  addi r3, r10, -0x25c
	ctx.r[3].s64 = ctx.r[10].s64 + -604;
	// 826B8204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B820C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B821C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8228: 4BDAEBF9  bl 0x82466e20
	ctx.lr = 0x826B822C;
	sub_82466E20(ctx, base);
	// 826B822C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8240 size=100
    let mut pc: u32 = 0x826B8240;
    'dispatch: loop {
        match pc {
            0x826B8240 => {
    //   block [0x826B8240..0x826B82A4)
	// 826B8240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B824C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8254: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B825C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8260: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826B8264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B826C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8274: 386AFDD4  addi r3, r10, -0x22c
	ctx.r[3].s64 = ctx.r[10].s64 + -556;
	// 826B8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B827C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8280: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B8284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8288: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B828C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8290: 4BDAEB91  bl 0x82466e20
	ctx.lr = 0x826B8294;
	sub_82466E20(ctx, base);
	// 826B8294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B829C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B82A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B82A8 size=112
    let mut pc: u32 = 0x826B82A8;
    'dispatch: loop {
        match pc {
            0x826B82A8 => {
    //   block [0x826B82A8..0x826B8318)
	// 826B82A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B82AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B82B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B82B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B82B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B82BC: 38AAFDD4  addi r5, r10, -0x22c
	ctx.r[5].s64 = ctx.r[10].s64 + -556;
	// 826B82C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B82C4: 390BB340  addi r8, r11, -0x4cc0
	ctx.r[8].s64 = ctx.r[11].s64 + -19648;
	// 826B82C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B82CC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826B82D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B82D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B82D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B82DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B82E0: 386AFE04  addi r3, r10, -0x1fc
	ctx.r[3].s64 = ctx.r[10].s64 + -508;
	// 826B82E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B82E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B82EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B82F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B82F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B82F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B82FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8304: 4BDAEB1D  bl 0x82466e20
	ctx.lr = 0x826B8308;
	sub_82466E20(ctx, base);
	// 826B8308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B830C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8318 size=108
    let mut pc: u32 = 0x826B8318;
    'dispatch: loop {
        match pc {
            0x826B8318 => {
    //   block [0x826B8318..0x826B8384)
	// 826B8318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8324: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B832C: 38EBB388  addi r7, r11, -0x4c78
	ctx.r[7].s64 = ctx.r[11].s64 + -19576;
	// 826B8330: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B8334: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826B8338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B833C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8348: 386AFE34  addi r3, r10, -0x1cc
	ctx.r[3].s64 = ctx.r[10].s64 + -460;
	// 826B834C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B835C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B836C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8370: 4BDAEAB1  bl 0x82466e20
	ctx.lr = 0x826B8374;
	sub_82466E20(ctx, base);
	// 826B8374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B837C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8388 size=112
    let mut pc: u32 = 0x826B8388;
    'dispatch: loop {
        match pc {
            0x826B8388 => {
    //   block [0x826B8388..0x826B83F8)
	// 826B8388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B838C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8394: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8398: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B839C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B83A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B83A4: 390BB3D0  addi r8, r11, -0x4c30
	ctx.r[8].s64 = ctx.r[11].s64 + -19504;
	// 826B83A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B83AC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826B83B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B83B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B83B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B83BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B83C0: 386AFE64  addi r3, r10, -0x19c
	ctx.r[3].s64 = ctx.r[10].s64 + -412;
	// 826B83C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B83C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B83CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B83D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B83D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B83D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B83DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B83E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B83E4: 4BDAEA3D  bl 0x82466e20
	ctx.lr = 0x826B83E8;
	sub_82466E20(ctx, base);
	// 826B83E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B83EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B83F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B83F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B83F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B83F8 size=108
    let mut pc: u32 = 0x826B83F8;
    'dispatch: loop {
        match pc {
            0x826B83F8 => {
    //   block [0x826B83F8..0x826B8464)
	// 826B83F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B83FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8404: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B840C: 38EBB3E8  addi r7, r11, -0x4c18
	ctx.r[7].s64 = ctx.r[11].s64 + -19480;
	// 826B8410: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B8414: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826B8418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B841C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8428: 386AFE94  addi r3, r10, -0x16c
	ctx.r[3].s64 = ctx.r[10].s64 + -364;
	// 826B842C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B843C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B844C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8450: 4BDAE9D1  bl 0x82466e20
	ctx.lr = 0x826B8454;
	sub_82466E20(ctx, base);
	// 826B8454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B845C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8468 size=112
    let mut pc: u32 = 0x826B8468;
    'dispatch: loop {
        match pc {
            0x826B8468 => {
    //   block [0x826B8468..0x826B84D8)
	// 826B8468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B846C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8474: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8478: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B847C: 38AAFE64  addi r5, r10, -0x19c
	ctx.r[5].s64 = ctx.r[10].s64 + -412;
	// 826B8480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8484: 390BB430  addi r8, r11, -0x4bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -19408;
	// 826B8488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B848C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826B8490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B849C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B84A0: 386AFEC4  addi r3, r10, -0x13c
	ctx.r[3].s64 = ctx.r[10].s64 + -316;
	// 826B84A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B84A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B84AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B84B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B84B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B84B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B84BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B84C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B84C4: 4BDAE95D  bl 0x82466e20
	ctx.lr = 0x826B84C8;
	sub_82466E20(ctx, base);
	// 826B84C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B84CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B84D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B84D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B84D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B84D8 size=100
    let mut pc: u32 = 0x826B84D8;
    'dispatch: loop {
        match pc {
            0x826B84D8 => {
    //   block [0x826B84D8..0x826B853C)
	// 826B84D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B84DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B84E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B84E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B84E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B84EC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B84F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B84F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B84F8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826B84FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B850C: 386AFEF4  addi r3, r10, -0x10c
	ctx.r[3].s64 = ctx.r[10].s64 + -268;
	// 826B8510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8514: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8518: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B851C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8520: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B8524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8528: 4BDAE8F9  bl 0x82466e20
	ctx.lr = 0x826B852C;
	sub_82466E20(ctx, base);
	// 826B852C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8540 size=112
    let mut pc: u32 = 0x826B8540;
    'dispatch: loop {
        match pc {
            0x826B8540 => {
    //   block [0x826B8540..0x826B85B0)
	// 826B8540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B854C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8550: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8554: 38AAFEF4  addi r5, r10, -0x10c
	ctx.r[5].s64 = ctx.r[10].s64 + -268;
	// 826B8558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B855C: 390BB448  addi r8, r11, -0x4bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -19384;
	// 826B8560: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B8564: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826B8568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B856C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8578: 386AFF24  addi r3, r10, -0xdc
	ctx.r[3].s64 = ctx.r[10].s64 + -220;
	// 826B857C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B858C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B859C: 4BDAE885  bl 0x82466e20
	ctx.lr = 0x826B85A0;
	sub_82466E20(ctx, base);
	// 826B85A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B85A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B85A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B85AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B85B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B85B0 size=108
    let mut pc: u32 = 0x826B85B0;
    'dispatch: loop {
        match pc {
            0x826B85B0 => {
    //   block [0x826B85B0..0x826B861C)
	// 826B85B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B85B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B85B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B85BC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B85C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B85C4: 38EBB4F0  addi r7, r11, -0x4b10
	ctx.r[7].s64 = ctx.r[11].s64 + -19216;
	// 826B85C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B85CC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826B85D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B85D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B85D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B85DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B85E0: 386AFF54  addi r3, r10, -0xac
	ctx.r[3].s64 = ctx.r[10].s64 + -172;
	// 826B85E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B85E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B85EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B85F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B85F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B85F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B85FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8608: 4BDAE819  bl 0x82466e20
	ctx.lr = 0x826B860C;
	sub_82466E20(ctx, base);
	// 826B860C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8620 size=112
    let mut pc: u32 = 0x826B8620;
    'dispatch: loop {
        match pc {
            0x826B8620 => {
    //   block [0x826B8620..0x826B8690)
	// 826B8620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B862C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8630: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8634: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B863C: 390BB520  addi r8, r11, -0x4ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -19168;
	// 826B8640: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B8644: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826B8648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B864C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8658: 386AFF84  addi r3, r10, -0x7c
	ctx.r[3].s64 = ctx.r[10].s64 + -124;
	// 826B865C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B866C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B867C: 4BDAE7A5  bl 0x82466e20
	ctx.lr = 0x826B8680;
	sub_82466E20(ctx, base);
	// 826B8680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8690 size=112
    let mut pc: u32 = 0x826B8690;
    'dispatch: loop {
        match pc {
            0x826B8690 => {
    //   block [0x826B8690..0x826B8700)
	// 826B8690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B869C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B86A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B86A4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B86A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B86AC: 390BB568  addi r8, r11, -0x4a98
	ctx.r[8].s64 = ctx.r[11].s64 + -19096;
	// 826B86B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B86B4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826B86B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B86BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B86C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B86C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B86C8: 386AFFB4  addi r3, r10, -0x4c
	ctx.r[3].s64 = ctx.r[10].s64 + -76;
	// 826B86CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B86D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B86D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B86D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B86DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B86E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B86E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B86E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B86EC: 4BDAE735  bl 0x82466e20
	ctx.lr = 0x826B86F0;
	sub_82466E20(ctx, base);
	// 826B86F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B86F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B86F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B86FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8700 size=100
    let mut pc: u32 = 0x826B8700;
    'dispatch: loop {
        match pc {
            0x826B8700 => {
    //   block [0x826B8700..0x826B8764)
	// 826B8700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B870C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8714: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B871C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8720: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826B8724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B872C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8734: 386AFFE4  addi r3, r10, -0x1c
	ctx.r[3].s64 = ctx.r[10].s64 + -28;
	// 826B8738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B873C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B8744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B874C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8750: 4BDAE6D1  bl 0x82466e20
	ctx.lr = 0x826B8754;
	sub_82466E20(ctx, base);
	// 826B8754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B875C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8768 size=112
    let mut pc: u32 = 0x826B8768;
    'dispatch: loop {
        match pc {
            0x826B8768 => {
    //   block [0x826B8768..0x826B87D8)
	// 826B8768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8778: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B877C: 38AAFFE4  addi r5, r10, -0x1c
	ctx.r[5].s64 = ctx.r[10].s64 + -28;
	// 826B8780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8784: 390BB5B0  addi r8, r11, -0x4a50
	ctx.r[8].s64 = ctx.r[11].s64 + -19024;
	// 826B8788: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B878C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826B8790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B879C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B87A0: 386A0014  addi r3, r10, 0x14
	ctx.r[3].s64 = ctx.r[10].s64 + 20;
	// 826B87A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B87A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B87AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B87B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B87B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B87B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B87BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B87C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B87C4: 4BDAE65D  bl 0x82466e20
	ctx.lr = 0x826B87C8;
	sub_82466E20(ctx, base);
	// 826B87C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B87CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B87D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B87D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B87D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B87D8 size=112
    let mut pc: u32 = 0x826B87D8;
    'dispatch: loop {
        match pc {
            0x826B87D8 => {
    //   block [0x826B87D8..0x826B8848)
	// 826B87D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B87DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B87E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B87E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B87E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B87EC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B87F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B87F4: 390BB5F8  addi r8, r11, -0x4a08
	ctx.r[8].s64 = ctx.r[11].s64 + -18952;
	// 826B87F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B87FC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826B8800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B880C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8810: 386A0044  addi r3, r10, 0x44
	ctx.r[3].s64 = ctx.r[10].s64 + 68;
	// 826B8814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B881C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B882C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8834: 4BDAE5ED  bl 0x82466e20
	ctx.lr = 0x826B8838;
	sub_82466E20(ctx, base);
	// 826B8838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B883C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8848 size=112
    let mut pc: u32 = 0x826B8848;
    'dispatch: loop {
        match pc {
            0x826B8848 => {
    //   block [0x826B8848..0x826B88B8)
	// 826B8848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B884C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8858: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B885C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8864: 390BB610  addi r8, r11, -0x49f0
	ctx.r[8].s64 = ctx.r[11].s64 + -18928;
	// 826B8868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B886C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826B8870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B887C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8880: 386A0074  addi r3, r10, 0x74
	ctx.r[3].s64 = ctx.r[10].s64 + 116;
	// 826B8884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B888C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8894: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B8898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B889C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B88A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B88A4: 4BDAE57D  bl 0x82466e20
	ctx.lr = 0x826B88A8;
	sub_82466E20(ctx, base);
	// 826B88A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B88AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B88B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B88B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B88B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B88B8 size=112
    let mut pc: u32 = 0x826B88B8;
    'dispatch: loop {
        match pc {
            0x826B88B8 => {
    //   block [0x826B88B8..0x826B8928)
	// 826B88B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B88BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B88C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B88C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B88C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B88CC: 38AA0044  addi r5, r10, 0x44
	ctx.r[5].s64 = ctx.r[10].s64 + 68;
	// 826B88D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B88D4: 390BB628  addi r8, r11, -0x49d8
	ctx.r[8].s64 = ctx.r[11].s64 + -18904;
	// 826B88D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B88DC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826B88E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B88E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B88E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B88EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B88F0: 386A00A4  addi r3, r10, 0xa4
	ctx.r[3].s64 = ctx.r[10].s64 + 164;
	// 826B88F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B88F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B88FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B890C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8914: 4BDAE50D  bl 0x82466e20
	ctx.lr = 0x826B8918;
	sub_82466E20(ctx, base);
	// 826B8918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8928 size=72
    let mut pc: u32 = 0x826B8928;
    'dispatch: loop {
        match pc {
            0x826B8928 => {
    //   block [0x826B8928..0x826B8970)
	// 826B8928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B892C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8934: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B8938: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 826B893C: 38CBEC80  addi r6, r11, -0x1380
	ctx.r[6].s64 = ctx.r[11].s64 + -4992;
	// 826B8940: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B8944: 388BFD60  addi r4, r11, -0x2a0
	ctx.r[4].s64 = ctx.r[11].s64 + -672;
	// 826B8948: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B894C: 386B00D4  addi r3, r11, 0xd4
	ctx.r[3].s64 = ctx.r[11].s64 + 212;
	// 826B8950: 4BDC3139  bl 0x8247ba88
	ctx.lr = 0x826B8954;
	sub_8247BA88(ctx, base);
	// 826B8954: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826B8958: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 826B895C: 4BE7A1DD  bl 0x82532b38
	ctx.lr = 0x826B8960;
	sub_82532B38(ctx, base);
	// 826B8960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826B8964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B896C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8970 size=108
    let mut pc: u32 = 0x826B8970;
    'dispatch: loop {
        match pc {
            0x826B8970 => {
    //   block [0x826B8970..0x826B89DC)
	// 826B8970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B897C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8984: 38EBC4B0  addi r7, r11, -0x3b50
	ctx.r[7].s64 = ctx.r[11].s64 + -15184;
	// 826B8988: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B898C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826B8990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B899C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B89A0: 386A00EC  addi r3, r10, 0xec
	ctx.r[3].s64 = ctx.r[10].s64 + 236;
	// 826B89A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B89A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B89AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B89B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B89B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B89B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B89BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B89C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B89C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B89C8: 4BDAE459  bl 0x82466e20
	ctx.lr = 0x826B89CC;
	sub_82466E20(ctx, base);
	// 826B89CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B89D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B89D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B89D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B89E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B89E0 size=24
    let mut pc: u32 = 0x826B89E0;
    'dispatch: loop {
        match pc {
            0x826B89E0 => {
    //   block [0x826B89E0..0x826B89F8)
	// 826B89E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B89E4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B89E8: 394A28A0  addi r10, r10, 0x28a0
	ctx.r[10].s64 = ctx.r[10].s64 + 10400;
	// 826B89EC: 816BC528  lwz r11, -0x3ad8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15064 as u32) ) } as u64;
	// 826B89F0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826B89F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B89F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B89F8 size=112
    let mut pc: u32 = 0x826B89F8;
    'dispatch: loop {
        match pc {
            0x826B89F8 => {
    //   block [0x826B89F8..0x826B8A68)
	// 826B89F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B89FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8A04: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B8A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8A0C: 392B02DC  addi r9, r11, 0x2dc
	ctx.r[9].s64 = ctx.r[11].s64 + 732;
	// 826B8A10: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826B8A14: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B8A18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8A1C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826B8A20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8A24: 396B28A0  addi r11, r11, 0x28a0
	ctx.r[11].s64 = ctx.r[11].s64 + 10400;
	// 826B8A28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B8A2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8A30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B8A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8A38: 386A011C  addi r3, r10, 0x11c
	ctx.r[3].s64 = ctx.r[10].s64 + 284;
	// 826B8A3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B8A40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B8A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8A48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B8A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8A50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B8A54: 4BDAE3CD  bl 0x82466e20
	ctx.lr = 0x826B8A58;
	sub_82466E20(ctx, base);
	// 826B8A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8A68 size=108
    let mut pc: u32 = 0x826B8A68;
    'dispatch: loop {
        match pc {
            0x826B8A68 => {
    //   block [0x826B8A68..0x826B8AD4)
	// 826B8A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8A74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8A7C: 38EBC52C  addi r7, r11, -0x3ad4
	ctx.r[7].s64 = ctx.r[11].s64 + -15060;
	// 826B8A80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B8A84: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826B8A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8A8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8A98: 386A014C  addi r3, r10, 0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + 332;
	// 826B8A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8AC0: 4BDAE361  bl 0x82466e20
	ctx.lr = 0x826B8AC4;
	sub_82466E20(ctx, base);
	// 826B8AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8AD8 size=108
    let mut pc: u32 = 0x826B8AD8;
    'dispatch: loop {
        match pc {
            0x826B8AD8 => {
    //   block [0x826B8AD8..0x826B8B44)
	// 826B8AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8AE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8AEC: 38EBC55C  addi r7, r11, -0x3aa4
	ctx.r[7].s64 = ctx.r[11].s64 + -15012;
	// 826B8AF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B8AF4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826B8AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8AFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8B08: 386A017C  addi r3, r10, 0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + 380;
	// 826B8B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8B30: 4BDAE2F1  bl 0x82466e20
	ctx.lr = 0x826B8B34;
	sub_82466E20(ctx, base);
	// 826B8B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8B48 size=112
    let mut pc: u32 = 0x826B8B48;
    'dispatch: loop {
        match pc {
            0x826B8B48 => {
    //   block [0x826B8B48..0x826B8BB8)
	// 826B8B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8B58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8B5C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B8B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8B64: 390BC590  addi r8, r11, -0x3a70
	ctx.r[8].s64 = ctx.r[11].s64 + -14960;
	// 826B8B68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B8B6C: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826B8B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8B80: 386A01AC  addi r3, r10, 0x1ac
	ctx.r[3].s64 = ctx.r[10].s64 + 428;
	// 826B8B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8BA4: 4BDAE27D  bl 0x82466e20
	ctx.lr = 0x826B8BA8;
	sub_82466E20(ctx, base);
	// 826B8BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8BB8 size=108
    let mut pc: u32 = 0x826B8BB8;
    'dispatch: loop {
        match pc {
            0x826B8BB8 => {
    //   block [0x826B8BB8..0x826B8C24)
	// 826B8BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8BC4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8BCC: 38EBC5F0  addi r7, r11, -0x3a10
	ctx.r[7].s64 = ctx.r[11].s64 + -14864;
	// 826B8BD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B8BD4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826B8BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8BDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8BE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8BE8: 386A01DC  addi r3, r10, 0x1dc
	ctx.r[3].s64 = ctx.r[10].s64 + 476;
	// 826B8BEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8C10: 4BDAE211  bl 0x82466e20
	ctx.lr = 0x826B8C14;
	sub_82466E20(ctx, base);
	// 826B8C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8C28 size=112
    let mut pc: u32 = 0x826B8C28;
    'dispatch: loop {
        match pc {
            0x826B8C28 => {
    //   block [0x826B8C28..0x826B8C98)
	// 826B8C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8C38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8C3C: 38AA01AC  addi r5, r10, 0x1ac
	ctx.r[5].s64 = ctx.r[10].s64 + 428;
	// 826B8C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8C44: 390BC668  addi r8, r11, -0x3998
	ctx.r[8].s64 = ctx.r[11].s64 + -14744;
	// 826B8C48: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B8C4C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826B8C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8C54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8C60: 386A020C  addi r3, r10, 0x20c
	ctx.r[3].s64 = ctx.r[10].s64 + 524;
	// 826B8C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8C84: 4BDAE19D  bl 0x82466e20
	ctx.lr = 0x826B8C88;
	sub_82466E20(ctx, base);
	// 826B8C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8C98 size=112
    let mut pc: u32 = 0x826B8C98;
    'dispatch: loop {
        match pc {
            0x826B8C98 => {
    //   block [0x826B8C98..0x826B8D08)
	// 826B8C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8CA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8CAC: 38AA01AC  addi r5, r10, 0x1ac
	ctx.r[5].s64 = ctx.r[10].s64 + 428;
	// 826B8CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8CB4: 390BC710  addi r8, r11, -0x38f0
	ctx.r[8].s64 = ctx.r[11].s64 + -14576;
	// 826B8CB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B8CBC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826B8CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8CC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8CD0: 386A023C  addi r3, r10, 0x23c
	ctx.r[3].s64 = ctx.r[10].s64 + 572;
	// 826B8CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8CF4: 4BDAE12D  bl 0x82466e20
	ctx.lr = 0x826B8CF8;
	sub_82466E20(ctx, base);
	// 826B8CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8D08 size=108
    let mut pc: u32 = 0x826B8D08;
    'dispatch: loop {
        match pc {
            0x826B8D08 => {
    //   block [0x826B8D08..0x826B8D74)
	// 826B8D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8D14: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8D1C: 38EBC728  addi r7, r11, -0x38d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14552;
	// 826B8D20: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B8D24: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826B8D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8D2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8D38: 386A026C  addi r3, r10, 0x26c
	ctx.r[3].s64 = ctx.r[10].s64 + 620;
	// 826B8D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8D60: 4BDAE0C1  bl 0x82466e20
	ctx.lr = 0x826B8D64;
	sub_82466E20(ctx, base);
	// 826B8D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8D78 size=112
    let mut pc: u32 = 0x826B8D78;
    'dispatch: loop {
        match pc {
            0x826B8D78 => {
    //   block [0x826B8D78..0x826B8DE8)
	// 826B8D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8D84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8D88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8D8C: 38AA01AC  addi r5, r10, 0x1ac
	ctx.r[5].s64 = ctx.r[10].s64 + 428;
	// 826B8D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8D94: 390BC7A0  addi r8, r11, -0x3860
	ctx.r[8].s64 = ctx.r[11].s64 + -14432;
	// 826B8D98: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B8D9C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826B8DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8DA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8DA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8DB0: 386A029C  addi r3, r10, 0x29c
	ctx.r[3].s64 = ctx.r[10].s64 + 668;
	// 826B8DB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8DD4: 4BDAE04D  bl 0x82466e20
	ctx.lr = 0x826B8DD8;
	sub_82466E20(ctx, base);
	// 826B8DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8DE8 size=108
    let mut pc: u32 = 0x826B8DE8;
    'dispatch: loop {
        match pc {
            0x826B8DE8 => {
    //   block [0x826B8DE8..0x826B8E54)
	// 826B8DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8DF4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8DFC: 38EBC848  addi r7, r11, -0x37b8
	ctx.r[7].s64 = ctx.r[11].s64 + -14264;
	// 826B8E00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B8E04: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826B8E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8E0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8E10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8E18: 386A02CC  addi r3, r10, 0x2cc
	ctx.r[3].s64 = ctx.r[10].s64 + 716;
	// 826B8E1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8E3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8E40: 4BDADFE1  bl 0x82466e20
	ctx.lr = 0x826B8E44;
	sub_82466E20(ctx, base);
	// 826B8E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8E58 size=108
    let mut pc: u32 = 0x826B8E58;
    'dispatch: loop {
        match pc {
            0x826B8E58 => {
    //   block [0x826B8E58..0x826B8EC4)
	// 826B8E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8E64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8E68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8E6C: 38EBC860  addi r7, r11, -0x37a0
	ctx.r[7].s64 = ctx.r[11].s64 + -14240;
	// 826B8E70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B8E74: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826B8E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8E80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8E88: 386A02FC  addi r3, r10, 0x2fc
	ctx.r[3].s64 = ctx.r[10].s64 + 764;
	// 826B8E8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8EAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8EB0: 4BDADF71  bl 0x82466e20
	ctx.lr = 0x826B8EB4;
	sub_82466E20(ctx, base);
	// 826B8EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8EC8 size=116
    let mut pc: u32 = 0x826B8EC8;
    'dispatch: loop {
        match pc {
            0x826B8EC8 => {
    //   block [0x826B8EC8..0x826B8F3C)
	// 826B8EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8ED4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8ED8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B8EDC: 390BC8C0  addi r8, r11, -0x3740
	ctx.r[8].s64 = ctx.r[11].s64 + -14144;
	// 826B8EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8EE4: 392A0318  addi r9, r10, 0x318
	ctx.r[9].s64 = ctx.r[10].s64 + 792;
	// 826B8EE8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8EEC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B8EF0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B8EF4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8EFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8F0C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B8F10: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826B8F14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B8F18: 386B032C  addi r3, r11, 0x32c
	ctx.r[3].s64 = ctx.r[11].s64 + 812;
	// 826B8F1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B8F20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8F28: 4BDADEF9  bl 0x82466e20
	ctx.lr = 0x826B8F2C;
	sub_82466E20(ctx, base);
	// 826B8F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8F40 size=108
    let mut pc: u32 = 0x826B8F40;
    'dispatch: loop {
        match pc {
            0x826B8F40 => {
    //   block [0x826B8F40..0x826B8FAC)
	// 826B8F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8F4C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8F54: 38EBC8D8  addi r7, r11, -0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + -14120;
	// 826B8F58: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B8F5C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826B8F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8F68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8F70: 386A035C  addi r3, r10, 0x35c
	ctx.r[3].s64 = ctx.r[10].s64 + 860;
	// 826B8F74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8F94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8F98: 4BDADE89  bl 0x82466e20
	ctx.lr = 0x826B8F9C;
	sub_82466E20(ctx, base);
	// 826B8F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8FB0 size=108
    let mut pc: u32 = 0x826B8FB0;
    'dispatch: loop {
        match pc {
            0x826B8FB0 => {
    //   block [0x826B8FB0..0x826B901C)
	// 826B8FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8FBC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8FC4: 38EBC920  addi r7, r11, -0x36e0
	ctx.r[7].s64 = ctx.r[11].s64 + -14048;
	// 826B8FC8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B8FCC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826B8FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8FD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8FD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8FE0: 386A038C  addi r3, r10, 0x38c
	ctx.r[3].s64 = ctx.r[10].s64 + 908;
	// 826B8FE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9008: 4BDADE19  bl 0x82466e20
	ctx.lr = 0x826B900C;
	sub_82466E20(ctx, base);
	// 826B900C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9020 size=108
    let mut pc: u32 = 0x826B9020;
    'dispatch: loop {
        match pc {
            0x826B9020 => {
    //   block [0x826B9020..0x826B908C)
	// 826B9020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B902C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9034: 38EBC9B0  addi r7, r11, -0x3650
	ctx.r[7].s64 = ctx.r[11].s64 + -13904;
	// 826B9038: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B903C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826B9040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B904C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9050: 386A03BC  addi r3, r10, 0x3bc
	ctx.r[3].s64 = ctx.r[10].s64 + 956;
	// 826B9054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B905C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B906C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9078: 4BDADDA9  bl 0x82466e20
	ctx.lr = 0x826B907C;
	sub_82466E20(ctx, base);
	// 826B907C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9090 size=100
    let mut pc: u32 = 0x826B9090;
    'dispatch: loop {
        match pc {
            0x826B9090 => {
    //   block [0x826B9090..0x826B90F4)
	// 826B9090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B909C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B90A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B90A4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B90A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B90AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B90B0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826B90B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B90B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B90BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B90C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B90C4: 386A03EC  addi r3, r10, 0x3ec
	ctx.r[3].s64 = ctx.r[10].s64 + 1004;
	// 826B90C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B90CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B90D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B90D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B90D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B90DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B90E0: 4BDADD41  bl 0x82466e20
	ctx.lr = 0x826B90E4;
	sub_82466E20(ctx, base);
	// 826B90E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B90E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B90EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B90F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B90F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B90F8 size=112
    let mut pc: u32 = 0x826B90F8;
    'dispatch: loop {
        match pc {
            0x826B90F8 => {
    //   block [0x826B90F8..0x826B9168)
	// 826B90F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B90FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9104: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9108: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B910C: 38AA03EC  addi r5, r10, 0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + 1004;
	// 826B9110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9114: 390BCA40  addi r8, r11, -0x35c0
	ctx.r[8].s64 = ctx.r[11].s64 + -13760;
	// 826B9118: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B911C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826B9120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9124: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B912C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9130: 386A041C  addi r3, r10, 0x41c
	ctx.r[3].s64 = ctx.r[10].s64 + 1052;
	// 826B9134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B913C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B914C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9154: 4BDADCCD  bl 0x82466e20
	ctx.lr = 0x826B9158;
	sub_82466E20(ctx, base);
	// 826B9158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B915C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9168 size=108
    let mut pc: u32 = 0x826B9168;
    'dispatch: loop {
        match pc {
            0x826B9168 => {
    //   block [0x826B9168..0x826B91D4)
	// 826B9168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B916C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9174: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B917C: 38EBCAA0  addi r7, r11, -0x3560
	ctx.r[7].s64 = ctx.r[11].s64 + -13664;
	// 826B9180: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B9184: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826B9188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B918C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9190: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9198: 386A044C  addi r3, r10, 0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + 1100;
	// 826B919C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B91A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B91A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B91A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B91AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B91B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B91B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B91B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B91BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B91C0: 4BDADC61  bl 0x82466e20
	ctx.lr = 0x826B91C4;
	sub_82466E20(ctx, base);
	// 826B91C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B91C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B91CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B91D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B91D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B91D8 size=108
    let mut pc: u32 = 0x826B91D8;
    'dispatch: loop {
        match pc {
            0x826B91D8 => {
    //   block [0x826B91D8..0x826B9244)
	// 826B91D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B91DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B91E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B91E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B91E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B91EC: 38EBCAD0  addi r7, r11, -0x3530
	ctx.r[7].s64 = ctx.r[11].s64 + -13616;
	// 826B91F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B91F4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826B91F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B91FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9208: 386A047C  addi r3, r10, 0x47c
	ctx.r[3].s64 = ctx.r[10].s64 + 1148;
	// 826B920C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B921C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B922C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9230: 4BDADBF1  bl 0x82466e20
	ctx.lr = 0x826B9234;
	sub_82466E20(ctx, base);
	// 826B9234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B923C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9248 size=108
    let mut pc: u32 = 0x826B9248;
    'dispatch: loop {
        match pc {
            0x826B9248 => {
    //   block [0x826B9248..0x826B92B4)
	// 826B9248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B924C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9254: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B925C: 38EBCB30  addi r7, r11, -0x34d0
	ctx.r[7].s64 = ctx.r[11].s64 + -13520;
	// 826B9260: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B9264: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826B9268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B926C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9278: 386A04AC  addi r3, r10, 0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1196;
	// 826B927C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B928C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B929C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B92A0: 4BDADB81  bl 0x82466e20
	ctx.lr = 0x826B92A4;
	sub_82466E20(ctx, base);
	// 826B92A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B92A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B92AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B92B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B92B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B92B8 size=108
    let mut pc: u32 = 0x826B92B8;
    'dispatch: loop {
        match pc {
            0x826B92B8 => {
    //   block [0x826B92B8..0x826B9324)
	// 826B92B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B92BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B92C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B92C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B92C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B92CC: 38EBCB90  addi r7, r11, -0x3470
	ctx.r[7].s64 = ctx.r[11].s64 + -13424;
	// 826B92D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B92D4: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826B92D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B92DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B92E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B92E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B92E8: 386A04DC  addi r3, r10, 0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + 1244;
	// 826B92EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B92F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B92F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B92F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B92FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B930C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9310: 4BDADB11  bl 0x82466e20
	ctx.lr = 0x826B9314;
	sub_82466E20(ctx, base);
	// 826B9314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B931C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9328 size=112
    let mut pc: u32 = 0x826B9328;
    'dispatch: loop {
        match pc {
            0x826B9328 => {
    //   block [0x826B9328..0x826B9398)
	// 826B9328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B932C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9334: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B9338: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B933C: 392A034C  addi r9, r10, 0x34c
	ctx.r[9].s64 = ctx.r[10].s64 + 844;
	// 826B9340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9344: 390BCC10  addi r8, r11, -0x33f0
	ctx.r[8].s64 = ctx.r[11].s64 + -13296;
	// 826B9348: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826B934C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826B9350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9354: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B935C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9360: 386A050C  addi r3, r10, 0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + 1292;
	// 826B9364: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9368: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B936C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B937C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9384: 4BDADA9D  bl 0x82466e20
	ctx.lr = 0x826B9388;
	sub_82466E20(ctx, base);
	// 826B9388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B938C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9398 size=112
    let mut pc: u32 = 0x826B9398;
    'dispatch: loop {
        match pc {
            0x826B9398 => {
    //   block [0x826B9398..0x826B9408)
	// 826B9398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B93A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B93A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B93A8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826B93AC: 38EACD18  addi r7, r10, -0x32e8
	ctx.r[7].s64 = ctx.r[10].s64 + -13032;
	// 826B93B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B93B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B93B8: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826B93BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B93C0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B93C4: 396B0360  addi r11, r11, 0x360
	ctx.r[11].s64 = ctx.r[11].s64 + 864;
	// 826B93C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B93CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B93D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B93D4: 386A053C  addi r3, r10, 0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + 1340;
	// 826B93D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B93DC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B93E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B93E4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B93E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B93EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B93F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B93F4: 4BDADA2D  bl 0x82466e20
	ctx.lr = 0x826B93F8;
	sub_82466E20(ctx, base);
	// 826B93F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B93FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9408 size=112
    let mut pc: u32 = 0x826B9408;
    'dispatch: loop {
        match pc {
            0x826B9408 => {
    //   block [0x826B9408..0x826B9478)
	// 826B9408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B940C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9414: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B9418: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B941C: 392A03A4  addi r9, r10, 0x3a4
	ctx.r[9].s64 = ctx.r[10].s64 + 932;
	// 826B9420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9424: 390BCE20  addi r8, r11, -0x31e0
	ctx.r[8].s64 = ctx.r[11].s64 + -12768;
	// 826B9428: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B942C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826B9430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9434: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B943C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9440: 386A056C  addi r3, r10, 0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + 1388;
	// 826B9444: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9448: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B944C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B945C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9464: 4BDAD9BD  bl 0x82466e20
	ctx.lr = 0x826B9468;
	sub_82466E20(ctx, base);
	// 826B9468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9478 size=100
    let mut pc: u32 = 0x826B9478;
    'dispatch: loop {
        match pc {
            0x826B9478 => {
    //   block [0x826B9478..0x826B94DC)
	// 826B9478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B947C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9484: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B948C: 38AA0B3C  addi r5, r10, 0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + 2876;
	// 826B9490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9498: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826B949C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B94A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B94A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B94A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B94AC: 386A059C  addi r3, r10, 0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + 1436;
	// 826B94B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B94B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B94B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B94BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B94C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B94C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B94C8: 4BDAD959  bl 0x82466e20
	ctx.lr = 0x826B94CC;
	sub_82466E20(ctx, base);
	// 826B94CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B94D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B94D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B94D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B94E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B94E0 size=116
    let mut pc: u32 = 0x826B94E0;
    'dispatch: loop {
        match pc {
            0x826B94E0 => {
    //   block [0x826B94E0..0x826B9554)
	// 826B94E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B94E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B94E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B94EC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B94F0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B94F4: 390ACE50  addi r8, r10, -0x31b0
	ctx.r[8].s64 = ctx.r[10].s64 + -12720;
	// 826B94F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B94FC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9500: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B9504: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9508: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B950C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9514: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826B9518: 396B03B8  addi r11, r11, 0x3b8
	ctx.r[11].s64 = ctx.r[11].s64 + 952;
	// 826B951C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9520: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9524: 386A05CC  addi r3, r10, 0x5cc
	ctx.r[3].s64 = ctx.r[10].s64 + 1484;
	// 826B9528: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B952C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9530: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B953C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9540: 4BDAD8E1  bl 0x82466e20
	ctx.lr = 0x826B9544;
	sub_82466E20(ctx, base);
	// 826B9544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B954C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9558 size=100
    let mut pc: u32 = 0x826B9558;
    'dispatch: loop {
        match pc {
            0x826B9558 => {
    //   block [0x826B9558..0x826B95BC)
	// 826B9558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B955C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9564: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B956C: 38AA05CC  addi r5, r10, 0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + 1484;
	// 826B9570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9578: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826B957C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B958C: 386A05FC  addi r3, r10, 0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + 1532;
	// 826B9590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9598: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B959C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B95A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B95A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B95A8: 4BDAD879  bl 0x82466e20
	ctx.lr = 0x826B95AC;
	sub_82466E20(ctx, base);
	// 826B95AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B95B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B95B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B95B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B95C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B95C0 size=112
    let mut pc: u32 = 0x826B95C0;
    'dispatch: loop {
        match pc {
            0x826B95C0 => {
    //   block [0x826B95C0..0x826B9630)
	// 826B95C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B95C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B95C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B95CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B95D0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B95D4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B95D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B95DC: 390BCEF8  addi r8, r11, -0x3108
	ctx.r[8].s64 = ctx.r[11].s64 + -12552;
	// 826B95E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B95E4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826B95E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B95EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B95F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B95F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B95F8: 386A062C  addi r3, r10, 0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + 1580;
	// 826B95FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B960C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B961C: 4BDAD805  bl 0x82466e20
	ctx.lr = 0x826B9620;
	sub_82466E20(ctx, base);
	// 826B9620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B962C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9630 size=116
    let mut pc: u32 = 0x826B9630;
    'dispatch: loop {
        match pc {
            0x826B9630 => {
    //   block [0x826B9630..0x826B96A4)
	// 826B9630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B963C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9640: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826B9644: 390ACF40  addi r8, r10, -0x30c0
	ctx.r[8].s64 = ctx.r[10].s64 + -12480;
	// 826B9648: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B964C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9650: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B9654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9658: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B965C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9664: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826B9668: 396B03E4  addi r11, r11, 0x3e4
	ctx.r[11].s64 = ctx.r[11].s64 + 996;
	// 826B966C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9670: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9674: 386A065C  addi r3, r10, 0x65c
	ctx.r[3].s64 = ctx.r[10].s64 + 1628;
	// 826B9678: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B967C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9680: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B968C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9690: 4BDAD791  bl 0x82466e20
	ctx.lr = 0x826B9694;
	sub_82466E20(ctx, base);
	// 826B9694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B969C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B96A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B96A8 size=108
    let mut pc: u32 = 0x826B96A8;
    'dispatch: loop {
        match pc {
            0x826B96A8 => {
    //   block [0x826B96A8..0x826B9714)
	// 826B96A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B96AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B96B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B96B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B96B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B96BC: 38EBD000  addi r7, r11, -0x3000
	ctx.r[7].s64 = ctx.r[11].s64 + -12288;
	// 826B96C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B96C4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826B96C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B96CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B96D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B96D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B96D8: 386A068C  addi r3, r10, 0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + 1676;
	// 826B96DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B96E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B96E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B96E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B96EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B96F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B96F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B96F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B96FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9700: 4BDAD721  bl 0x82466e20
	ctx.lr = 0x826B9704;
	sub_82466E20(ctx, base);
	// 826B9704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B9718 size=24
    let mut pc: u32 = 0x826B9718;
    'dispatch: loop {
        match pc {
            0x826B9718 => {
    //   block [0x826B9718..0x826B9730)
	// 826B9718: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B971C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9720: 394A28E8  addi r10, r10, 0x28e8
	ctx.r[10].s64 = ctx.r[10].s64 + 10472;
	// 826B9724: 816BD048  lwz r11, -0x2fb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12216 as u32) ) } as u64;
	// 826B9728: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9730 size=116
    let mut pc: u32 = 0x826B9730;
    'dispatch: loop {
        match pc {
            0x826B9730 => {
    //   block [0x826B9730..0x826B97A4)
	// 826B9730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B973C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9740: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9744: 392B042C  addi r9, r11, 0x42c
	ctx.r[9].s64 = ctx.r[11].s64 + 1068;
	// 826B9748: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B974C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9750: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B9754: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826B9758: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B975C: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826B9760: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9764: 396B28E8  addi r11, r11, 0x28e8
	ctx.r[11].s64 = ctx.r[11].s64 + 10472;
	// 826B9768: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B976C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9770: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B9774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9778: 386A06BC  addi r3, r10, 0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + 1724;
	// 826B977C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B9780: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B9784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9788: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B978C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B9790: 4BDAD691  bl 0x82466e20
	ctx.lr = 0x826B9794;
	sub_82466E20(ctx, base);
	// 826B9794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B979C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B97A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B97A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B97A8 size=112
    let mut pc: u32 = 0x826B97A8;
    'dispatch: loop {
        match pc {
            0x826B97A8 => {
    //   block [0x826B97A8..0x826B9818)
	// 826B97A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B97AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B97B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B97B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B97B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B97BC: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B97C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B97C4: 390BD04C  addi r8, r11, -0x2fb4
	ctx.r[8].s64 = ctx.r[11].s64 + -12212;
	// 826B97C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B97CC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826B97D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B97D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B97D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B97DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B97E0: 386A06EC  addi r3, r10, 0x6ec
	ctx.r[3].s64 = ctx.r[10].s64 + 1772;
	// 826B97E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B97E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B97EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B97F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B97F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B97F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B97FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9804: 4BDAD61D  bl 0x82466e20
	ctx.lr = 0x826B9808;
	sub_82466E20(ctx, base);
	// 826B9808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B980C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9818 size=112
    let mut pc: u32 = 0x826B9818;
    'dispatch: loop {
        match pc {
            0x826B9818 => {
    //   block [0x826B9818..0x826B9888)
	// 826B9818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B981C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9824: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9828: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B982C: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B9830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9834: 390BD07C  addi r8, r11, -0x2f84
	ctx.r[8].s64 = ctx.r[11].s64 + -12164;
	// 826B9838: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B983C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826B9840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B984C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9850: 386A071C  addi r3, r10, 0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + 1820;
	// 826B9854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B985C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B986C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9874: 4BDAD5AD  bl 0x82466e20
	ctx.lr = 0x826B9878;
	sub_82466E20(ctx, base);
	// 826B9878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B987C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9888 size=100
    let mut pc: u32 = 0x826B9888;
    'dispatch: loop {
        match pc {
            0x826B9888 => {
    //   block [0x826B9888..0x826B98EC)
	// 826B9888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9894: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B989C: 38AA0B3C  addi r5, r10, 0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + 2876;
	// 826B98A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B98A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B98A8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826B98AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B98B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B98B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B98B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B98BC: 386A074C  addi r3, r10, 0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + 1868;
	// 826B98C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B98C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B98C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B98CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B98D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B98D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B98D8: 4BDAD549  bl 0x82466e20
	ctx.lr = 0x826B98DC;
	sub_82466E20(ctx, base);
	// 826B98DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B98E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B98E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B98E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B98F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B98F0 size=112
    let mut pc: u32 = 0x826B98F0;
    'dispatch: loop {
        match pc {
            0x826B98F0 => {
    //   block [0x826B98F0..0x826B9960)
	// 826B98F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B98F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B98F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B98FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9900: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9904: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B990C: 390BD094  addi r8, r11, -0x2f6c
	ctx.r[8].s64 = ctx.r[11].s64 + -12140;
	// 826B9910: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B9914: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826B9918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B991C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9928: 386A077C  addi r3, r10, 0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + 1916;
	// 826B992C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B993C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B994C: 4BDAD4D5  bl 0x82466e20
	ctx.lr = 0x826B9950;
	sub_82466E20(ctx, base);
	// 826B9950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B995C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9960 size=108
    let mut pc: u32 = 0x826B9960;
    'dispatch: loop {
        match pc {
            0x826B9960 => {
    //   block [0x826B9960..0x826B99CC)
	// 826B9960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B996C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9974: 38EBD0C8  addi r7, r11, -0x2f38
	ctx.r[7].s64 = ctx.r[11].s64 + -12088;
	// 826B9978: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B997C: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826B9980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B998C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9990: 386A07AC  addi r3, r10, 0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1964;
	// 826B9994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B999C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B99A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B99A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B99A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B99AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B99B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B99B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B99B8: 4BDAD469  bl 0x82466e20
	ctx.lr = 0x826B99BC;
	sub_82466E20(ctx, base);
	// 826B99BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B99C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B99C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B99C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B99D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B99D0 size=108
    let mut pc: u32 = 0x826B99D0;
    'dispatch: loop {
        match pc {
            0x826B99D0 => {
    //   block [0x826B99D0..0x826B9A3C)
	// 826B99D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B99D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B99D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B99DC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B99E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B99E4: 38EBD128  addi r7, r11, -0x2ed8
	ctx.r[7].s64 = ctx.r[11].s64 + -11992;
	// 826B99E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B99EC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826B99F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B99F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B99F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B99FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9A00: 386A07DC  addi r3, r10, 0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + 2012;
	// 826B9A04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9A24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9A28: 4BDAD3F9  bl 0x82466e20
	ctx.lr = 0x826B9A2C;
	sub_82466E20(ctx, base);
	// 826B9A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9A40 size=116
    let mut pc: u32 = 0x826B9A40;
    'dispatch: loop {
        match pc {
            0x826B9A40 => {
    //   block [0x826B9A40..0x826B9AB4)
	// 826B9A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9A4C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9A50: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826B9A54: 390AD158  addi r8, r10, -0x2ea8
	ctx.r[8].s64 = ctx.r[10].s64 + -11944;
	// 826B9A58: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9A5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9A60: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9A64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9A68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9A74: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826B9A78: 396B0480  addi r11, r11, 0x480
	ctx.r[11].s64 = ctx.r[11].s64 + 1152;
	// 826B9A7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9A80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9A84: 386A080C  addi r3, r10, 0x80c
	ctx.r[3].s64 = ctx.r[10].s64 + 2060;
	// 826B9A88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9A8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9A90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9AA0: 4BDAD381  bl 0x82466e20
	ctx.lr = 0x826B9AA4;
	sub_82466E20(ctx, base);
	// 826B9AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9AB8 size=112
    let mut pc: u32 = 0x826B9AB8;
    'dispatch: loop {
        match pc {
            0x826B9AB8 => {
    //   block [0x826B9AB8..0x826B9B28)
	// 826B9AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9AC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9ACC: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9AD4: 390BD2D8  addi r8, r11, -0x2d28
	ctx.r[8].s64 = ctx.r[11].s64 + -11560;
	// 826B9AD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B9ADC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826B9AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9AF0: 386A083C  addi r3, r10, 0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + 2108;
	// 826B9AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9B14: 4BDAD30D  bl 0x82466e20
	ctx.lr = 0x826B9B18;
	sub_82466E20(ctx, base);
	// 826B9B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9B28 size=116
    let mut pc: u32 = 0x826B9B28;
    'dispatch: loop {
        match pc {
            0x826B9B28 => {
    //   block [0x826B9B28..0x826B9B9C)
	// 826B9B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9B34: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9B38: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826B9B3C: 390AD2F0  addi r8, r10, -0x2d10
	ctx.r[8].s64 = ctx.r[10].s64 + -11536;
	// 826B9B40: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9B44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9B48: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9B4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9B50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9B5C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826B9B60: 396B04CC  addi r11, r11, 0x4cc
	ctx.r[11].s64 = ctx.r[11].s64 + 1228;
	// 826B9B64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9B68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9B6C: 386A086C  addi r3, r10, 0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + 2156;
	// 826B9B70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9B74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9B78: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9B88: 4BDAD299  bl 0x82466e20
	ctx.lr = 0x826B9B8C;
	sub_82466E20(ctx, base);
	// 826B9B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9BA0 size=112
    let mut pc: u32 = 0x826B9BA0;
    'dispatch: loop {
        match pc {
            0x826B9BA0 => {
    //   block [0x826B9BA0..0x826B9C10)
	// 826B9BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9BAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9BB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9BB4: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9BBC: 390BD350  addi r8, r11, -0x2cb0
	ctx.r[8].s64 = ctx.r[11].s64 + -11440;
	// 826B9BC0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826B9BC4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826B9BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9BD8: 386A089C  addi r3, r10, 0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + 2204;
	// 826B9BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9BFC: 4BDAD225  bl 0x82466e20
	ctx.lr = 0x826B9C00;
	sub_82466E20(ctx, base);
	// 826B9C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9C10 size=112
    let mut pc: u32 = 0x826B9C10;
    'dispatch: loop {
        match pc {
            0x826B9C10 => {
    //   block [0x826B9C10..0x826B9C80)
	// 826B9C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9C1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9C20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9C24: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9C2C: 390BD428  addi r8, r11, -0x2bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -11224;
	// 826B9C30: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826B9C34: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826B9C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9C40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9C48: 386A08CC  addi r3, r10, 0x8cc
	ctx.r[3].s64 = ctx.r[10].s64 + 2252;
	// 826B9C4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9C6C: 4BDAD1B5  bl 0x82466e20
	ctx.lr = 0x826B9C70;
	sub_82466E20(ctx, base);
	// 826B9C70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


