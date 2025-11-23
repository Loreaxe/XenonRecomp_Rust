pub fn sub_8329C810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C810 size=112
    let mut pc: u32 = 0x8329C810;
    'dispatch: loop {
        match pc {
            0x8329C810 => {
    //   block [0x8329C810..0x8329C880)
	// 8329C810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C818: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C81C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C820: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8329C824: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C828: 396BA6A8  addi r11, r11, -0x5958
	ctx.r[11].s64 = ctx.r[11].s64 + -22872;
	// 8329C82C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8329C830: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8329C834: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329C838: 4BB01D69  bl 0x82d9e5a0
	ctx.lr = 0x8329C83C;
	sub_82D9E5A0(ctx, base);
	// 8329C83C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C840: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329C844: 394BA6D8  addi r10, r11, -0x5928
	ctx.r[10].s64 = ctx.r[11].s64 + -22824;
	// 8329C848: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C84C: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329C850: 396B8A80  addi r11, r11, -0x7580
	ctx.r[11].s64 = ctx.r[11].s64 + -30080;
	// 8329C854: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329C858: 39481748  addi r10, r8, 0x1748
	ctx.r[10].s64 = ctx.r[8].s64 + 5960;
	// 8329C85C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329C860: 39491730  addi r10, r9, 0x1730
	ctx.r[10].s64 = ctx.r[9].s64 + 5936;
	// 8329C864: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329C868: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329C86C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329C870: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8329C874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329C880 size=24
    let mut pc: u32 = 0x8329C880;
    'dispatch: loop {
        match pc {
            0x8329C880 => {
    //   block [0x8329C880..0x8329C898)
	// 8329C880: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C884: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329C888: 394A8BA8  addi r10, r10, -0x7458
	ctx.r[10].s64 = ctx.r[10].s64 + -29784;
	// 8329C88C: 816B8BA0  lwz r11, -0x7460(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29792 as u32) ) } as u64;
	// 8329C890: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329C894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C898 size=116
    let mut pc: u32 = 0x8329C898;
    'dispatch: loop {
        match pc {
            0x8329C898 => {
    //   block [0x8329C898..0x8329C90C)
	// 8329C898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C8A4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C8A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C8AC: 390B8BA8  addi r8, r11, -0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + -29784;
	// 8329C8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C8B4: 392AA690  addi r9, r10, -0x5970
	ctx.r[9].s64 = ctx.r[10].s64 + -22896;
	// 8329C8B8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C8BC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329C8C0: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329C8C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C8CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C8DC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C8E0: 388AA6D8  addi r4, r10, -0x5928
	ctx.r[4].s64 = ctx.r[10].s64 + -22824;
	// 8329C8E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C8E8: 386B8A90  addi r3, r11, -0x7570
	ctx.r[3].s64 = ctx.r[11].s64 + -30064;
	// 8329C8EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C8F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C8F4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329C8F8: 4BAB9349  bl 0x82d55c40
	ctx.lr = 0x8329C8FC;
	sub_82D55C40(ctx, base);
	// 8329C8FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C910 size=112
    let mut pc: u32 = 0x8329C910;
    'dispatch: loop {
        match pc {
            0x8329C910 => {
    //   block [0x8329C910..0x8329C980)
	// 8329C910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C91C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C920: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C924: 38AA8D50  addi r5, r10, -0x72b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29360;
	// 8329C928: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C92C: 390BA730  addi r8, r11, -0x58d0
	ctx.r[8].s64 = ctx.r[11].s64 + -22736;
	// 8329C930: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329C934: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 8329C938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C93C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C948: 386A8AC0  addi r3, r10, -0x7540
	ctx.r[3].s64 = ctx.r[10].s64 + -30016;
	// 8329C94C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C95C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C964: 38C00098  li r6, 0x98
	ctx.r[6].s64 = 152;
	// 8329C968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C96C: 4BAB92D5  bl 0x82d55c40
	ctx.lr = 0x8329C970;
	sub_82D55C40(ctx, base);
	// 8329C970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C980 size=108
    let mut pc: u32 = 0x8329C980;
    'dispatch: loop {
        match pc {
            0x8329C980 => {
    //   block [0x8329C980..0x8329C9EC)
	// 8329C980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C98C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C994: 38EBA770  addi r7, r11, -0x5890
	ctx.r[7].s64 = ctx.r[11].s64 + -22672;
	// 8329C998: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329C99C: 388AA7EC  addi r4, r10, -0x5814
	ctx.r[4].s64 = ctx.r[10].s64 + -22548;
	// 8329C9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C9A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C9A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C9B0: 386A8AF0  addi r3, r10, -0x7510
	ctx.r[3].s64 = ctx.r[10].s64 + -29968;
	// 8329C9B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C9CC: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8329C9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C9D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C9D8: 4BAB9269  bl 0x82d55c40
	ctx.lr = 0x8329C9DC;
	sub_82D55C40(ctx, base);
	// 8329C9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C9F0 size=112
    let mut pc: u32 = 0x8329C9F0;
    'dispatch: loop {
        match pc {
            0x8329C9F0 => {
    //   block [0x8329C9F0..0x8329CA60)
	// 8329C9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C9FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CA00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CA04: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329CA08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CA0C: 390BA7A0  addi r8, r11, -0x5860
	ctx.r[8].s64 = ctx.r[11].s64 + -22624;
	// 8329CA10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CA14: 388AA810  addi r4, r10, -0x57f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22512;
	// 8329CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CA1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CA20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CA28: 386A8B20  addi r3, r10, -0x74e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29920;
	// 8329CA2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CA44: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CA4C: 4BAB91F5  bl 0x82d55c40
	ctx.lr = 0x8329CA50;
	sub_82D55C40(ctx, base);
	// 8329CA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CA60 size=104
    let mut pc: u32 = 0x8329CA60;
    'dispatch: loop {
        match pc {
            0x8329CA60 => {
    //   block [0x8329CA60..0x8329CAC8)
	// 8329CA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CA6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CA74: 392AA8B0  addi r9, r10, -0x5750
	ctx.r[9].s64 = ctx.r[10].s64 + -22352;
	// 8329CA78: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CA80: 38AA88F0  addi r5, r10, -0x7710
	ctx.r[5].s64 = ctx.r[10].s64 + -30480;
	// 8329CA84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CA94: 388AA8C4  addi r4, r10, -0x573c
	ctx.r[4].s64 = ctx.r[10].s64 + -22332;
	// 8329CA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CA9C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CAA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329CAA4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329CAA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329CAAC: 386A8B50  addi r3, r10, -0x74b0
	ctx.r[3].s64 = ctx.r[10].s64 + -29872;
	// 8329CAB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CAB4: 4BAB918D  bl 0x82d55c40
	ctx.lr = 0x8329CAB8;
	sub_82D55C40(ctx, base);
	// 8329CAB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CAC8 size=92
    let mut pc: u32 = 0x8329CAC8;
    'dispatch: loop {
        match pc {
            0x8329CAC8 => {
    //   block [0x8329CAC8..0x8329CB24)
	// 8329CAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CAD0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CAD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329CAD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329CADC: 4BAE51F5  bl 0x82d81cd0
	ctx.lr = 0x8329CAE0;
	sub_82D81CD0(ctx, base);
	// 8329CAE0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CAE4: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329CAE8: 394BA980  addi r10, r11, -0x5680
	ctx.r[10].s64 = ctx.r[11].s64 + -22144;
	// 8329CAEC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329CAF0: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329CAF4: 396B8B80  addi r11, r11, -0x7480
	ctx.r[11].s64 = ctx.r[11].s64 + -29824;
	// 8329CAF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329CAFC: 39481C88  addi r10, r8, 0x1c88
	ctx.r[10].s64 = ctx.r[8].s64 + 7304;
	// 8329CB00: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329CB04: 39491C70  addi r10, r9, 0x1c70
	ctx.r[10].s64 = ctx.r[9].s64 + 7280;
	// 8329CB08: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329CB0C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329CB10: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329CB14: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 8329CB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CB28 size=112
    let mut pc: u32 = 0x8329CB28;
    'dispatch: loop {
        match pc {
            0x8329CB28 => {
    //   block [0x8329CB28..0x8329CB98)
	// 8329CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CB34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CB38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CB3C: 38AA8AC0  addi r5, r10, -0x7540
	ctx.r[5].s64 = ctx.r[10].s64 + -30016;
	// 8329CB40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CB44: 390BA918  addi r8, r11, -0x56e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22248;
	// 8329CB48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329CB4C: 388AA980  addi r4, r10, -0x5680
	ctx.r[4].s64 = ctx.r[10].s64 + -22144;
	// 8329CB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CB54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CB60: 386A8B90  addi r3, r10, -0x7470
	ctx.r[3].s64 = ctx.r[10].s64 + -29808;
	// 8329CB64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CB7C: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8329CB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CB84: 4BAB90BD  bl 0x82d55c40
	ctx.lr = 0x8329CB88;
	sub_82D55C40(ctx, base);
	// 8329CB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CB98 size=112
    let mut pc: u32 = 0x8329CB98;
    'dispatch: loop {
        match pc {
            0x8329CB98 => {
    //   block [0x8329CB98..0x8329CC08)
	// 8329CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CBA4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CBA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CBAC: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329CBB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CBB4: 390BA9D0  addi r8, r11, -0x5630
	ctx.r[8].s64 = ctx.r[11].s64 + -22064;
	// 8329CBB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329CBBC: 388AAA6C  addi r4, r10, -0x5594
	ctx.r[4].s64 = ctx.r[10].s64 + -21908;
	// 8329CBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CBC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CBD0: 386A8BC0  addi r3, r10, -0x7440
	ctx.r[3].s64 = ctx.r[10].s64 + -29760;
	// 8329CBD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CBEC: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8329CBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CBF4: 4BAB904D  bl 0x82d55c40
	ctx.lr = 0x8329CBF8;
	sub_82D55C40(ctx, base);
	// 8329CBF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CC08 size=112
    let mut pc: u32 = 0x8329CC08;
    'dispatch: loop {
        match pc {
            0x8329CC08 => {
    //   block [0x8329CC08..0x8329CC78)
	// 8329CC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CC14: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CC18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CC1C: 38AA8AC0  addi r5, r10, -0x7540
	ctx.r[5].s64 = ctx.r[10].s64 + -30016;
	// 8329CC20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CC24: 390BAA88  addi r8, r11, -0x5578
	ctx.r[8].s64 = ctx.r[11].s64 + -21880;
	// 8329CC28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CC2C: 388AAAA0  addi r4, r10, -0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + -21856;
	// 8329CC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CC34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CC40: 386A8BF0  addi r3, r10, -0x7410
	ctx.r[3].s64 = ctx.r[10].s64 + -29712;
	// 8329CC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CC5C: 38C00150  li r6, 0x150
	ctx.r[6].s64 = 336;
	// 8329CC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CC64: 4BAB8FDD  bl 0x82d55c40
	ctx.lr = 0x8329CC68;
	sub_82D55C40(ctx, base);
	// 8329CC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CC78 size=108
    let mut pc: u32 = 0x8329CC78;
    'dispatch: loop {
        match pc {
            0x8329CC78 => {
    //   block [0x8329CC78..0x8329CCE4)
	// 8329CC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CC84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CC88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CC8C: 38EBAAF8  addi r7, r11, -0x5508
	ctx.r[7].s64 = ctx.r[11].s64 + -21768;
	// 8329CC90: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8329CC94: 388AAC94  addi r4, r10, -0x536c
	ctx.r[4].s64 = ctx.r[10].s64 + -21356;
	// 8329CC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CC9C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CCA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329CCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CCA8: 386A8C20  addi r3, r10, -0x73e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29664;
	// 8329CCAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329CCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CCBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CCC4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329CCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CCCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329CCD0: 4BAB8F71  bl 0x82d55c40
	ctx.lr = 0x8329CCD4;
	sub_82D55C40(ctx, base);
	// 8329CCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CCE8 size=92
    let mut pc: u32 = 0x8329CCE8;
    'dispatch: loop {
        match pc {
            0x8329CCE8 => {
    //   block [0x8329CCE8..0x8329CD44)
	// 8329CCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CCF0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CCF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329CCF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329CCFC: 4BB039F5  bl 0x82da06f0
	ctx.lr = 0x8329CD00;
	sub_82DA06F0(ctx, base);
	// 8329CD00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CD04: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329CD08: 394BACB8  addi r10, r11, -0x5348
	ctx.r[10].s64 = ctx.r[11].s64 + -21320;
	// 8329CD0C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329CD10: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329CD14: 396B8C50  addi r11, r11, -0x73b0
	ctx.r[11].s64 = ctx.r[11].s64 + -29616;
	// 8329CD18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329CD1C: 39481DF0  addi r10, r8, 0x1df0
	ctx.r[10].s64 = ctx.r[8].s64 + 7664;
	// 8329CD20: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329CD24: 39491E08  addi r10, r9, 0x1e08
	ctx.r[10].s64 = ctx.r[9].s64 + 7688;
	// 8329CD28: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329CD2C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329CD30: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329CD34: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8329CD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CD48 size=112
    let mut pc: u32 = 0x8329CD48;
    'dispatch: loop {
        match pc {
            0x8329CD48 => {
    //   block [0x8329CD48..0x8329CDB8)
	// 8329CD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CD54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CD58: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CD5C: 396BAB88  addi r11, r11, -0x5478
	ctx.r[11].s64 = ctx.r[11].s64 + -21624;
	// 8329CD60: 38AA8630  addi r5, r10, -0x79d0
	ctx.r[5].s64 = ctx.r[10].s64 + -31184;
	// 8329CD64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CD68: 390B00D8  addi r8, r11, 0xd8
	ctx.r[8].s64 = ctx.r[11].s64 + 216;
	// 8329CD6C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8329CD70: 388AACB8  addi r4, r10, -0x5348
	ctx.r[4].s64 = ctx.r[10].s64 + -21320;
	// 8329CD74: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329CD78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CD7C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CD80: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329CD84: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329CD88: 386A8C60  addi r3, r10, -0x73a0
	ctx.r[3].s64 = ctx.r[10].s64 + -29600;
	// 8329CD8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CD98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329CD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CDA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329CDA4: 4BAB8E9D  bl 0x82d55c40
	ctx.lr = 0x8329CDA8;
	sub_82D55C40(ctx, base);
	// 8329CDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329CDB8 size=24
    let mut pc: u32 = 0x8329CDB8;
    'dispatch: loop {
        match pc {
            0x8329CDB8 => {
    //   block [0x8329CDB8..0x8329CDD0)
	// 8329CDB8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329CDBC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329CDC0: 394A8D90  addi r10, r10, -0x7270
	ctx.r[10].s64 = ctx.r[10].s64 + -29296;
	// 8329CDC4: 816B8D78  lwz r11, -0x7288(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29320 as u32) ) } as u64;
	// 8329CDC8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329CDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CDD0 size=112
    let mut pc: u32 = 0x8329CDD0;
    'dispatch: loop {
        match pc {
            0x8329CDD0 => {
    //   block [0x8329CDD0..0x8329CE40)
	// 8329CDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CDDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CDE0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329CDE4: 392AAD74  addi r9, r10, -0x528c
	ctx.r[9].s64 = ctx.r[10].s64 + -21132;
	// 8329CDE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CDEC: 390B8D90  addi r8, r11, -0x7270
	ctx.r[8].s64 = ctx.r[11].s64 + -29296;
	// 8329CDF0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329CDF4: 388AAD88  addi r4, r10, -0x5278
	ctx.r[4].s64 = ctx.r[10].s64 + -21112;
	// 8329CDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CDFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CE00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CE04: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329CE08: 386A8C90  addi r3, r10, -0x7370
	ctx.r[3].s64 = ctx.r[10].s64 + -29552;
	// 8329CE0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CE10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329CE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CE2C: 4BAB8E15  bl 0x82d55c40
	ctx.lr = 0x8329CE30;
	sub_82D55C40(ctx, base);
	// 8329CE30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CE40 size=112
    let mut pc: u32 = 0x8329CE40;
    'dispatch: loop {
        match pc {
            0x8329CE40 => {
    //   block [0x8329CE40..0x8329CEB0)
	// 8329CE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CE4C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CE50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CE54: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329CE58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CE5C: 390BADAC  addi r8, r11, -0x5254
	ctx.r[8].s64 = ctx.r[11].s64 + -21076;
	// 8329CE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CE64: 388AADC4  addi r4, r10, -0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + -21052;
	// 8329CE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CE6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CE78: 386A8CC0  addi r3, r10, -0x7340
	ctx.r[3].s64 = ctx.r[10].s64 + -29504;
	// 8329CE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CE94: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329CE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CE9C: 4BAB8DA5  bl 0x82d55c40
	ctx.lr = 0x8329CEA0;
	sub_82D55C40(ctx, base);
	// 8329CEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CEB0 size=108
    let mut pc: u32 = 0x8329CEB0;
    'dispatch: loop {
        match pc {
            0x8329CEB0 => {
    //   block [0x8329CEB0..0x8329CF1C)
	// 8329CEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CEBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CEC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CEC4: 392BAE30  addi r9, r11, -0x51d0
	ctx.r[9].s64 = ctx.r[11].s64 + -20944;
	// 8329CEC8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329CECC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329CED0: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 8329CED4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CED8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CEDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CEE0: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8329CEE4: 386A8CF0  addi r3, r10, -0x7310
	ctx.r[3].s64 = ctx.r[10].s64 + -29456;
	// 8329CEE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CEEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CEF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CEF4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CEF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CEFC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CF00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329CF04: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CF08: 4BAB8D39  bl 0x82d55c40
	ctx.lr = 0x8329CF0C;
	sub_82D55C40(ctx, base);
	// 8329CF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CF20 size=112
    let mut pc: u32 = 0x8329CF20;
    'dispatch: loop {
        match pc {
            0x8329CF20 => {
    //   block [0x8329CF20..0x8329CF90)
	// 8329CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CF2C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CF30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CF34: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329CF38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CF3C: 390BAEF0  addi r8, r11, -0x5110
	ctx.r[8].s64 = ctx.r[11].s64 + -20752;
	// 8329CF40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CF44: 388AAF5C  addi r4, r10, -0x50a4
	ctx.r[4].s64 = ctx.r[10].s64 + -20644;
	// 8329CF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CF4C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CF58: 386A8D20  addi r3, r10, -0x72e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29408;
	// 8329CF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CF74: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8329CF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CF7C: 4BAB8CC5  bl 0x82d55c40
	ctx.lr = 0x8329CF80;
	sub_82D55C40(ctx, base);
	// 8329CF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CF90 size=116
    let mut pc: u32 = 0x8329CF90;
    'dispatch: loop {
        match pc {
            0x8329CF90 => {
    //   block [0x8329CF90..0x8329D004)
	// 8329CF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CF9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CFA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CFA4: 390BB098  addi r8, r11, -0x4f68
	ctx.r[8].s64 = ctx.r[11].s64 + -20328;
	// 8329CFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CFAC: 392AB084  addi r9, r10, -0x4f7c
	ctx.r[9].s64 = ctx.r[10].s64 + -20348;
	// 8329CFB0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329CFB4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329CFB8: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329CFBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CFC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CFD4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329CFD8: 388AB128  addi r4, r10, -0x4ed8
	ctx.r[4].s64 = ctx.r[10].s64 + -20184;
	// 8329CFDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CFE0: 386B8D50  addi r3, r11, -0x72b0
	ctx.r[3].s64 = ctx.r[11].s64 + -29360;
	// 8329CFE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CFE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CFEC: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329CFF0: 4BAB8C51  bl 0x82d55c40
	ctx.lr = 0x8329CFF4;
	sub_82D55C40(ctx, base);
	// 8329CFF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CFF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CFFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D008 size=100
    let mut pc: u32 = 0x8329D008;
    'dispatch: loop {
        match pc {
            0x8329D008 => {
    //   block [0x8329D008..0x8329D06C)
	// 8329D008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D014: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D01C: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329D020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D028: 388AB1A0  addi r4, r10, -0x4e60
	ctx.r[4].s64 = ctx.r[10].s64 + -20064;
	// 8329D02C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D03C: 386A8D80  addi r3, r10, -0x7280
	ctx.r[3].s64 = ctx.r[10].s64 + -29312;
	// 8329D040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D048: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329D04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D050: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329D054: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329D058: 4BAB8BE9  bl 0x82d55c40
	ctx.lr = 0x8329D05C;
	sub_82D55C40(ctx, base);
	// 8329D05C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D070 size=108
    let mut pc: u32 = 0x8329D070;
    'dispatch: loop {
        match pc {
            0x8329D070 => {
    //   block [0x8329D070..0x8329D0DC)
	// 8329D070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D07C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D084: 38EBB1C8  addi r7, r11, -0x4e38
	ctx.r[7].s64 = ctx.r[11].s64 + -20024;
	// 8329D088: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329D08C: 388AB244  addi r4, r10, -0x4dbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19900;
	// 8329D090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D094: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D0A0: 386A8DB0  addi r3, r10, -0x7250
	ctx.r[3].s64 = ctx.r[10].s64 + -29264;
	// 8329D0A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D0BC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329D0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D0C8: 4BAB8B79  bl 0x82d55c40
	ctx.lr = 0x8329D0CC;
	sub_82D55C40(ctx, base);
	// 8329D0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D0E0 size=112
    let mut pc: u32 = 0x8329D0E0;
    'dispatch: loop {
        match pc {
            0x8329D0E0 => {
    //   block [0x8329D0E0..0x8329D150)
	// 8329D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D0EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D0F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D0F4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D0FC: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 8329D100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D104: 388AB264  addi r4, r10, -0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -19868;
	// 8329D108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D10C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D118: 386A8DE0  addi r3, r10, -0x7220
	ctx.r[3].s64 = ctx.r[10].s64 + -29216;
	// 8329D11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D134: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329D138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D13C: 4BAB8B05  bl 0x82d55c40
	ctx.lr = 0x8329D140;
	sub_82D55C40(ctx, base);
	// 8329D140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D150 size=92
    let mut pc: u32 = 0x8329D150;
    'dispatch: loop {
        match pc {
            0x8329D150 => {
    //   block [0x8329D150..0x8329D1AC)
	// 8329D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D158: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D15C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D164: 4BB04F15  bl 0x82da2078
	ctx.lr = 0x8329D168;
	sub_82DA2078(ctx, base);
	// 8329D168: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D16C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D170: 394BB3B8  addi r10, r11, -0x4c48
	ctx.r[10].s64 = ctx.r[11].s64 + -19528;
	// 8329D174: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D178: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329D17C: 396B8E10  addi r11, r11, -0x71f0
	ctx.r[11].s64 = ctx.r[11].s64 + -29168;
	// 8329D180: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D184: 39482030  addi r10, r8, 0x2030
	ctx.r[10].s64 = ctx.r[8].s64 + 8240;
	// 8329D188: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D18C: 39492018  addi r10, r9, 0x2018
	ctx.r[10].s64 = ctx.r[9].s64 + 8216;
	// 8329D190: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D194: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329D198: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329D19C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8329D1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D1B0 size=112
    let mut pc: u32 = 0x8329D1B0;
    'dispatch: loop {
        match pc {
            0x8329D1B0 => {
    //   block [0x8329D1B0..0x8329D220)
	// 8329D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D1BC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D1C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D1C4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D1C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D1CC: 390BB2F8  addi r8, r11, -0x4d08
	ctx.r[8].s64 = ctx.r[11].s64 + -19720;
	// 8329D1D0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8329D1D4: 388AB3B8  addi r4, r10, -0x4c48
	ctx.r[4].s64 = ctx.r[10].s64 + -19528;
	// 8329D1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D1DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D1E8: 386A8E20  addi r3, r10, -0x71e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29152;
	// 8329D1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D204: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 8329D208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D20C: 4BAB8A35  bl 0x82d55c40
	ctx.lr = 0x8329D210;
	sub_82D55C40(ctx, base);
	// 8329D210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D220 size=108
    let mut pc: u32 = 0x8329D220;
    'dispatch: loop {
        match pc {
            0x8329D220 => {
    //   block [0x8329D220..0x8329D28C)
	// 8329D220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D22C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D234: 392BB4AC  addi r9, r11, -0x4b54
	ctx.r[9].s64 = ctx.r[11].s64 + -19284;
	// 8329D238: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329D23C: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 8329D240: 388AB5B4  addi r4, r10, -0x4a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19020;
	// 8329D244: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D248: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D24C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D250: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 8329D254: 386A8E50  addi r3, r10, -0x71b0
	ctx.r[3].s64 = ctx.r[10].s64 + -29104;
	// 8329D258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D25C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D260: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D264: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D268: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D26C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D270: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D274: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D278: 4BAB89C9  bl 0x82d55c40
	ctx.lr = 0x8329D27C;
	sub_82D55C40(ctx, base);
	// 8329D27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D290 size=112
    let mut pc: u32 = 0x8329D290;
    'dispatch: loop {
        match pc {
            0x8329D290 => {
    //   block [0x8329D290..0x8329D300)
	// 8329D290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D29C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D2A0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D2A4: 392BB480  addi r9, r11, -0x4b80
	ctx.r[9].s64 = ctx.r[11].s64 + -19328;
	// 8329D2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D2AC: 390900E8  addi r8, r9, 0xe8
	ctx.r[8].s64 = ctx.r[9].s64 + 232;
	// 8329D2B0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329D2B4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D2B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D2BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D2C0: 38C00140  li r6, 0x140
	ctx.r[6].s64 = 320;
	// 8329D2C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D2C8: 388AB5D4  addi r4, r10, -0x4a2c
	ctx.r[4].s64 = ctx.r[10].s64 + -18988;
	// 8329D2CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D2D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D2D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D2D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D2DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D2E0: 386B8E80  addi r3, r11, -0x7180
	ctx.r[3].s64 = ctx.r[11].s64 + -29056;
	// 8329D2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D2E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D2EC: 4BAB8955  bl 0x82d55c40
	ctx.lr = 0x8329D2F0;
	sub_82D55C40(ctx, base);
	// 8329D2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D300 size=92
    let mut pc: u32 = 0x8329D300;
    'dispatch: loop {
        match pc {
            0x8329D300 => {
    //   block [0x8329D300..0x8329D35C)
	// 8329D300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D308: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D30C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D314: 4BAE4E9D  bl 0x82d821b0
	ctx.lr = 0x8329D318;
	sub_82D821B0(ctx, base);
	// 8329D318: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D31C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D320: 394BB61C  addi r10, r11, -0x49e4
	ctx.r[10].s64 = ctx.r[11].s64 + -18916;
	// 8329D324: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D328: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329D32C: 396B8EB0  addi r11, r11, -0x7150
	ctx.r[11].s64 = ctx.r[11].s64 + -29008;
	// 8329D330: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D334: 39482168  addi r10, r8, 0x2168
	ctx.r[10].s64 = ctx.r[8].s64 + 8552;
	// 8329D338: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D33C: 39492150  addi r10, r9, 0x2150
	ctx.r[10].s64 = ctx.r[9].s64 + 8528;
	// 8329D340: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D344: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329D348: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329D34C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 8329D350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D360 size=112
    let mut pc: u32 = 0x8329D360;
    'dispatch: loop {
        match pc {
            0x8329D360 => {
    //   block [0x8329D360..0x8329D3D0)
	// 8329D360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D36C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D370: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D374: 38AA8BF0  addi r5, r10, -0x7410
	ctx.r[5].s64 = ctx.r[10].s64 + -29712;
	// 8329D378: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D37C: 390BB604  addi r8, r11, -0x49fc
	ctx.r[8].s64 = ctx.r[11].s64 + -18940;
	// 8329D380: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D384: 388AB61C  addi r4, r10, -0x49e4
	ctx.r[4].s64 = ctx.r[10].s64 + -18916;
	// 8329D388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D38C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D398: 386A8EC0  addi r3, r10, -0x7140
	ctx.r[3].s64 = ctx.r[10].s64 + -28992;
	// 8329D39C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D3B4: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 8329D3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D3BC: 4BAB8885  bl 0x82d55c40
	ctx.lr = 0x8329D3C0;
	sub_82D55C40(ctx, base);
	// 8329D3C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D3D0 size=108
    let mut pc: u32 = 0x8329D3D0;
    'dispatch: loop {
        match pc {
            0x8329D3D0 => {
    //   block [0x8329D3D0..0x8329D43C)
	// 8329D3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D3DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D3E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D3E4: 392BB650  addi r9, r11, -0x49b0
	ctx.r[9].s64 = ctx.r[11].s64 + -18864;
	// 8329D3E8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8329D3EC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329D3F0: 388AB714  addi r4, r10, -0x48ec
	ctx.r[4].s64 = ctx.r[10].s64 + -18668;
	// 8329D3F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D3F8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D3FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D400: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329D404: 386A8EF0  addi r3, r10, -0x7110
	ctx.r[3].s64 = ctx.r[10].s64 + -28944;
	// 8329D408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D40C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D410: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D414: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D418: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D41C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D420: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D424: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D428: 4BAB8819  bl 0x82d55c40
	ctx.lr = 0x8329D42C;
	sub_82D55C40(ctx, base);
	// 8329D42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D440 size=112
    let mut pc: u32 = 0x8329D440;
    'dispatch: loop {
        match pc {
            0x8329D440 => {
    //   block [0x8329D440..0x8329D4B0)
	// 8329D440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D44C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D450: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D454: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D45C: 390BB6C8  addi r8, r11, -0x4938
	ctx.r[8].s64 = ctx.r[11].s64 + -18744;
	// 8329D460: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D464: 388AB730  addi r4, r10, -0x48d0
	ctx.r[4].s64 = ctx.r[10].s64 + -18640;
	// 8329D468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D46C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D478: 386A8F20  addi r3, r10, -0x70e0
	ctx.r[3].s64 = ctx.r[10].s64 + -28896;
	// 8329D47C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D48C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D494: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8329D498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D49C: 4BAB87A5  bl 0x82d55c40
	ctx.lr = 0x8329D4A0;
	sub_82D55C40(ctx, base);
	// 8329D4A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D4B0 size=108
    let mut pc: u32 = 0x8329D4B0;
    'dispatch: loop {
        match pc {
            0x8329D4B0 => {
    //   block [0x8329D4B0..0x8329D51C)
	// 8329D4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D4BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D4C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D4C4: 38EBB760  addi r7, r11, -0x48a0
	ctx.r[7].s64 = ctx.r[11].s64 + -18592;
	// 8329D4C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329D4CC: 388AB7C0  addi r4, r10, -0x4840
	ctx.r[4].s64 = ctx.r[10].s64 + -18496;
	// 8329D4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D4D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D4D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D4DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D4E0: 386A8F50  addi r3, r10, -0x70b0
	ctx.r[3].s64 = ctx.r[10].s64 + -28848;
	// 8329D4E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D4E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D4EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D4F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D4F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D4F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D4FC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329D500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D508: 4BAB8739  bl 0x82d55c40
	ctx.lr = 0x8329D50C;
	sub_82D55C40(ctx, base);
	// 8329D50C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D520 size=108
    let mut pc: u32 = 0x8329D520;
    'dispatch: loop {
        match pc {
            0x8329D520 => {
    //   block [0x8329D520..0x8329D58C)
	// 8329D520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D52C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D534: 38EBB778  addi r7, r11, -0x4888
	ctx.r[7].s64 = ctx.r[11].s64 + -18568;
	// 8329D538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329D53C: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 8329D540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D544: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D54C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D550: 386A8F80  addi r3, r10, -0x7080
	ctx.r[3].s64 = ctx.r[10].s64 + -28800;
	// 8329D554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D56C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329D570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D578: 4BAB86C9  bl 0x82d55c40
	ctx.lr = 0x8329D57C;
	sub_82D55C40(ctx, base);
	// 8329D57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329D590 size=24
    let mut pc: u32 = 0x8329D590;
    'dispatch: loop {
        match pc {
            0x8329D590 => {
    //   block [0x8329D590..0x8329D5A8)
	// 8329D590: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329D594: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329D598: 394A8FA0  addi r10, r10, -0x7060
	ctx.r[10].s64 = ctx.r[10].s64 + -28768;
	// 8329D59C: 816B8F88  lwz r11, -0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 8329D5A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329D5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D5A8 size=116
    let mut pc: u32 = 0x8329D5A8;
    'dispatch: loop {
        match pc {
            0x8329D5A8 => {
    //   block [0x8329D5A8..0x8329D61C)
	// 8329D5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D5B4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329D5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D5BC: 390B8FA0  addi r8, r11, -0x7060
	ctx.r[8].s64 = ctx.r[11].s64 + -28768;
	// 8329D5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D5C4: 392AB8D8  addi r9, r10, -0x4728
	ctx.r[9].s64 = ctx.r[10].s64 + -18216;
	// 8329D5C8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D5CC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329D5D0: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329D5D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D5D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D5DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D5E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D5E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D5EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D5F0: 388AB900  addi r4, r10, -0x4700
	ctx.r[4].s64 = ctx.r[10].s64 + -18176;
	// 8329D5F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D5F8: 386B8FB0  addi r3, r11, -0x7050
	ctx.r[3].s64 = ctx.r[11].s64 + -28752;
	// 8329D5FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D600: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D604: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329D608: 4BAB8639  bl 0x82d55c40
	ctx.lr = 0x8329D60C;
	sub_82D55C40(ctx, base);
	// 8329D60C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D620 size=96
    let mut pc: u32 = 0x8329D620;
    'dispatch: loop {
        match pc {
            0x8329D620 => {
    //   block [0x8329D620..0x8329D680)
	// 8329D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D628: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D62C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D630: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D634: 4BAE84FD  bl 0x82d85b30
	ctx.lr = 0x8329D638;
	sub_82D85B30(ctx, base);
	// 8329D638: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D63C: 3CE082D8  lis r7, -0x7d28
	ctx.r[7].s64 = -2099773440;
	// 8329D640: 394BB960  addi r10, r11, -0x46a0
	ctx.r[10].s64 = ctx.r[11].s64 + -18080;
	// 8329D644: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D648: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D64C: 392BB944  addi r9, r11, -0x46bc
	ctx.r[9].s64 = ctx.r[11].s64 + -18108;
	// 8329D650: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D654: 396B8FE0  addi r11, r11, -0x7020
	ctx.r[11].s64 = ctx.r[11].s64 + -28704;
	// 8329D658: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D65C: 39472360  addi r10, r7, 0x2360
	ctx.r[10].s64 = ctx.r[7].s64 + 9056;
	// 8329D660: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D664: 39482348  addi r10, r8, 0x2348
	ctx.r[10].s64 = ctx.r[8].s64 + 9032;
	// 8329D668: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D66C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8329D670: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 8329D674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D680 size=100
    let mut pc: u32 = 0x8329D680;
    'dispatch: loop {
        match pc {
            0x8329D680 => {
    //   block [0x8329D680..0x8329D6E4)
	// 8329D680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D68C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D694: 38AA8860  addi r5, r10, -0x77a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30624;
	// 8329D698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D69C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D6A0: 388AB960  addi r4, r10, -0x46a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18080;
	// 8329D6A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D6AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D6B4: 386A8FF0  addi r3, r10, -0x7010
	ctx.r[3].s64 = ctx.r[10].s64 + -28688;
	// 8329D6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D6BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D6C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329D6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D6C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329D6CC: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 8329D6D0: 4BAB8571  bl 0x82d55c40
	ctx.lr = 0x8329D6D4;
	sub_82D55C40(ctx, base);
	// 8329D6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D6E8 size=100
    let mut pc: u32 = 0x8329D6E8;
    'dispatch: loop {
        match pc {
            0x8329D6E8 => {
    //   block [0x8329D6E8..0x8329D74C)
	// 8329D6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D6F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D6FC: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329D700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D708: 388AB9D8  addi r4, r10, -0x4628
	ctx.r[4].s64 = ctx.r[10].s64 + -17960;
	// 8329D70C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D71C: 386A9020  addi r3, r10, -0x6fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -28640;
	// 8329D720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D728: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329D72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D730: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329D734: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329D738: 4BAB8509  bl 0x82d55c40
	ctx.lr = 0x8329D73C;
	sub_82D55C40(ctx, base);
	// 8329D73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D750 size=108
    let mut pc: u32 = 0x8329D750;
    'dispatch: loop {
        match pc {
            0x8329D750 => {
    //   block [0x8329D750..0x8329D7BC)
	// 8329D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D75C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D764: 38EBB9EC  addi r7, r11, -0x4614
	ctx.r[7].s64 = ctx.r[11].s64 + -17940;
	// 8329D768: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329D76C: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 8329D770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D774: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D778: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D77C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D780: 386A9050  addi r3, r10, -0x6fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -28592;
	// 8329D784: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D78C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D79C: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329D7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D7A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D7A8: 4BAB8499  bl 0x82d55c40
	ctx.lr = 0x8329D7AC;
	sub_82D55C40(ctx, base);
	// 8329D7AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D7C0 size=112
    let mut pc: u32 = 0x8329D7C0;
    'dispatch: loop {
        match pc {
            0x8329D7C0 => {
    //   block [0x8329D7C0..0x8329D830)
	// 8329D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D7CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D7D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D7D4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D7D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D7DC: 390BBA1C  addi r8, r11, -0x45e4
	ctx.r[8].s64 = ctx.r[11].s64 + -17892;
	// 8329D7E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D7E4: 388ABA8C  addi r4, r10, -0x4574
	ctx.r[4].s64 = ctx.r[10].s64 + -17780;
	// 8329D7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D7EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D7F8: 386A9080  addi r3, r10, -0x6f80
	ctx.r[3].s64 = ctx.r[10].s64 + -28544;
	// 8329D7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D814: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 8329D818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D81C: 4BAB8425  bl 0x82d55c40
	ctx.lr = 0x8329D820;
	sub_82D55C40(ctx, base);
	// 8329D820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D830 size=112
    let mut pc: u32 = 0x8329D830;
    'dispatch: loop {
        match pc {
            0x8329D830 => {
    //   block [0x8329D830..0x8329D8A0)
	// 8329D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D83C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D840: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D844: 38AA91B0  addi r5, r10, -0x6e50
	ctx.r[5].s64 = ctx.r[10].s64 + -28240;
	// 8329D848: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D84C: 390BBAC0  addi r8, r11, -0x4540
	ctx.r[8].s64 = ctx.r[11].s64 + -17728;
	// 8329D850: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329D854: 388ABB30  addi r4, r10, -0x44d0
	ctx.r[4].s64 = ctx.r[10].s64 + -17616;
	// 8329D858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D85C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D868: 386A90B0  addi r3, r10, -0x6f50
	ctx.r[3].s64 = ctx.r[10].s64 + -28496;
	// 8329D86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D884: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 8329D888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D88C: 4BAB83B5  bl 0x82d55c40
	ctx.lr = 0x8329D890;
	sub_82D55C40(ctx, base);
	// 8329D890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D8A0 size=108
    let mut pc: u32 = 0x8329D8A0;
    'dispatch: loop {
        match pc {
            0x8329D8A0 => {
    //   block [0x8329D8A0..0x8329D90C)
	// 8329D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D8AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D8B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D8B4: 38EBBB4C  addi r7, r11, -0x44b4
	ctx.r[7].s64 = ctx.r[11].s64 + -17588;
	// 8329D8B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329D8BC: 388ABB7C  addi r4, r10, -0x4484
	ctx.r[4].s64 = ctx.r[10].s64 + -17540;
	// 8329D8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D8C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D8C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D8CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D8D0: 386A90E0  addi r3, r10, -0x6f20
	ctx.r[3].s64 = ctx.r[10].s64 + -28448;
	// 8329D8D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D8EC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329D8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D8F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D8F8: 4BAB8349  bl 0x82d55c40
	ctx.lr = 0x8329D8FC;
	sub_82D55C40(ctx, base);
	// 8329D8FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D910 size=92
    let mut pc: u32 = 0x8329D910;
    'dispatch: loop {
        match pc {
            0x8329D910 => {
    //   block [0x8329D910..0x8329D96C)
	// 8329D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D918: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D91C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D924: 4BAE4D3D  bl 0x82d82660
	ctx.lr = 0x8329D928;
	sub_82D82660(ctx, base);
	// 8329D928: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D92C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D930: 394BBBA4  addi r10, r11, -0x445c
	ctx.r[10].s64 = ctx.r[11].s64 + -17500;
	// 8329D934: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D938: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329D93C: 396B9110  addi r11, r11, -0x6ef0
	ctx.r[11].s64 = ctx.r[11].s64 + -28400;
	// 8329D940: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D944: 39482618  addi r10, r8, 0x2618
	ctx.r[10].s64 = ctx.r[8].s64 + 9752;
	// 8329D948: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D94C: 39492600  addi r10, r9, 0x2600
	ctx.r[10].s64 = ctx.r[9].s64 + 9728;
	// 8329D950: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D954: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329D958: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329D95C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 8329D960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D970 size=112
    let mut pc: u32 = 0x8329D970;
    'dispatch: loop {
        match pc {
            0x8329D970 => {
    //   block [0x8329D970..0x8329D9E0)
	// 8329D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D97C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D980: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D984: 38AA8BF0  addi r5, r10, -0x7410
	ctx.r[5].s64 = ctx.r[10].s64 + -29712;
	// 8329D988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D98C: 390BBB64  addi r8, r11, -0x449c
	ctx.r[8].s64 = ctx.r[11].s64 + -17564;
	// 8329D990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D994: 388ABBA4  addi r4, r10, -0x445c
	ctx.r[4].s64 = ctx.r[10].s64 + -17500;
	// 8329D998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D99C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D9A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D9A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D9A8: 386A9120  addi r3, r10, -0x6ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -28384;
	// 8329D9AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D9B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D9B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D9C4: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 8329D9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D9CC: 4BAB8275  bl 0x82d55c40
	ctx.lr = 0x8329D9D0;
	sub_82D55C40(ctx, base);
	// 8329D9D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D9E0 size=112
    let mut pc: u32 = 0x8329D9E0;
    'dispatch: loop {
        match pc {
            0x8329D9E0 => {
    //   block [0x8329D9E0..0x8329DA50)
	// 8329D9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D9EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D9F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D9F4: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329D9F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D9FC: 390BBBD0  addi r8, r11, -0x4430
	ctx.r[8].s64 = ctx.r[11].s64 + -17456;
	// 8329DA00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329DA04: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 8329DA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DA0C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DA10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DA18: 386A9150  addi r3, r10, -0x6eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -28336;
	// 8329DA1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DA2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DA34: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329DA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DA3C: 4BAB8205  bl 0x82d55c40
	ctx.lr = 0x8329DA40;
	sub_82D55C40(ctx, base);
	// 8329DA40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329DA50 size=36
    let mut pc: u32 = 0x8329DA50;
    'dispatch: loop {
        match pc {
            0x8329DA50 => {
    //   block [0x8329DA50..0x8329DA74)
	// 8329DA50: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DA54: 814B91C8  lwz r10, -0x6e38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28216 as u32) ) } as u64;
	// 8329DA58: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DA5C: 396B91E0  addi r11, r11, -0x6e20
	ctx.r[11].s64 = ctx.r[11].s64 + -28192;
	// 8329DA60: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8329DA64: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329DA68: 814A91C0  lwz r10, -0x6e40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28224 as u32) ) } as u64;
	// 8329DA6C: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8329DA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DA78 size=116
    let mut pc: u32 = 0x8329DA78;
    'dispatch: loop {
        match pc {
            0x8329DA78 => {
    //   block [0x8329DA78..0x8329DAEC)
	// 8329DA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DA84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DA88: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329DA8C: 392ABDB8  addi r9, r10, -0x4248
	ctx.r[9].s64 = ctx.r[10].s64 + -16968;
	// 8329DA90: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329DA94: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329DA98: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329DA9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DAA0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8329DAA4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DAA8: 388ABE20  addi r4, r10, -0x41e0
	ctx.r[4].s64 = ctx.r[10].s64 + -16864;
	// 8329DAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DAB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329DAB4: 396B91E0  addi r11, r11, -0x6e20
	ctx.r[11].s64 = ctx.r[11].s64 + -28192;
	// 8329DAB8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DAC0: 386A9180  addi r3, r10, -0x6e80
	ctx.r[3].s64 = ctx.r[10].s64 + -28288;
	// 8329DAC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329DAC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329DACC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329DAD0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329DAD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DAD8: 4BAB8169  bl 0x82d55c40
	ctx.lr = 0x8329DADC;
	sub_82D55C40(ctx, base);
	// 8329DADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329DAF0 size=24
    let mut pc: u32 = 0x8329DAF0;
    'dispatch: loop {
        match pc {
            0x8329DAF0 => {
    //   block [0x8329DAF0..0x8329DB08)
	// 8329DAF0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DAF4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329DAF8: 394A92E0  addi r10, r10, -0x6d20
	ctx.r[10].s64 = ctx.r[10].s64 + -27936;
	// 8329DAFC: 816B92C0  lwz r11, -0x6d40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27968 as u32) ) } as u64;
	// 8329DB00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8329DB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DB08 size=116
    let mut pc: u32 = 0x8329DB08;
    'dispatch: loop {
        match pc {
            0x8329DB08 => {
    //   block [0x8329DB08..0x8329DB7C)
	// 8329DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DB14: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DB18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DB1C: 390B92E0  addi r8, r11, -0x6d20
	ctx.r[8].s64 = ctx.r[11].s64 + -27936;
	// 8329DB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DB24: 392AC020  addi r9, r10, -0x3fe0
	ctx.r[9].s64 = ctx.r[10].s64 + -16352;
	// 8329DB28: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329DB2C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8329DB30: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329DB34: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DB3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DB4C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329DB50: 388AC070  addi r4, r10, -0x3f90
	ctx.r[4].s64 = ctx.r[10].s64 + -16272;
	// 8329DB54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329DB58: 386B91B0  addi r3, r11, -0x6e50
	ctx.r[3].s64 = ctx.r[11].s64 + -28240;
	// 8329DB5C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8329DB60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DB64: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 8329DB68: 4BAB80D9  bl 0x82d55c40
	ctx.lr = 0x8329DB6C;
	sub_82D55C40(ctx, base);
	// 8329DB6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DB80 size=108
    let mut pc: u32 = 0x8329DB80;
    'dispatch: loop {
        match pc {
            0x8329DB80 => {
    //   block [0x8329DB80..0x8329DBEC)
	// 8329DB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DB8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DB90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DB94: 38EBC158  addi r7, r11, -0x3ea8
	ctx.r[7].s64 = ctx.r[11].s64 + -16040;
	// 8329DB98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329DB9C: 388AC24C  addi r4, r10, -0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + -15796;
	// 8329DBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DBA4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DBA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329DBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DBB0: 386A91E0  addi r3, r10, -0x6e20
	ctx.r[3].s64 = ctx.r[10].s64 + -28192;
	// 8329DBB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329DBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DBCC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329DBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DBD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329DBD8: 4BAB8069  bl 0x82d55c40
	ctx.lr = 0x8329DBDC;
	sub_82D55C40(ctx, base);
	// 8329DBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DBF0 size=112
    let mut pc: u32 = 0x8329DBF0;
    'dispatch: loop {
        match pc {
            0x8329DBF0 => {
    //   block [0x8329DBF0..0x8329DC60)
	// 8329DBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DBFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DC04: 38AA8B50  addi r5, r10, -0x74b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29872;
	// 8329DC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DC0C: 390BC188  addi r8, r11, -0x3e78
	ctx.r[8].s64 = ctx.r[11].s64 + -15992;
	// 8329DC10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8329DC14: 388AC274  addi r4, r10, -0x3d8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15756;
	// 8329DC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DC1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DC28: 386A9210  addi r3, r10, -0x6df0
	ctx.r[3].s64 = ctx.r[10].s64 + -28144;
	// 8329DC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DC44: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329DC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DC4C: 4BAB7FF5  bl 0x82d55c40
	ctx.lr = 0x8329DC50;
	sub_82D55C40(ctx, base);
	// 8329DC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DC60 size=100
    let mut pc: u32 = 0x8329DC60;
    'dispatch: loop {
        match pc {
            0x8329DC60 => {
    //   block [0x8329DC60..0x8329DCC4)
	// 8329DC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DC6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DC74: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329DC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DC80: 388AC300  addi r4, r10, -0x3d00
	ctx.r[4].s64 = ctx.r[10].s64 + -15616;
	// 8329DC84: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DC94: 386A9240  addi r3, r10, -0x6dc0
	ctx.r[3].s64 = ctx.r[10].s64 + -28096;
	// 8329DC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DC9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DCA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DCA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DCA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DCAC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329DCB0: 4BAB7F91  bl 0x82d55c40
	ctx.lr = 0x8329DCB4;
	sub_82D55C40(ctx, base);
	// 8329DCB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DCC8 size=100
    let mut pc: u32 = 0x8329DCC8;
    'dispatch: loop {
        match pc {
            0x8329DCC8 => {
    //   block [0x8329DCC8..0x8329DD2C)
	// 8329DCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DCD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DCDC: 38AA8B50  addi r5, r10, -0x74b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29872;
	// 8329DCE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DCE8: 388AC32C  addi r4, r10, -0x3cd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15572;
	// 8329DCEC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DCFC: 386A9270  addi r3, r10, -0x6d90
	ctx.r[3].s64 = ctx.r[10].s64 + -28048;
	// 8329DD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DD04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DD08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DD10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DD14: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329DD18: 4BAB7F29  bl 0x82d55c40
	ctx.lr = 0x8329DD1C;
	sub_82D55C40(ctx, base);
	// 8329DD1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DD30 size=112
    let mut pc: u32 = 0x8329DD30;
    'dispatch: loop {
        match pc {
            0x8329DD30 => {
    //   block [0x8329DD30..0x8329DDA0)
	// 8329DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DD3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DD40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DD44: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329DD48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DD4C: 390BC368  addi r8, r11, -0x3c98
	ctx.r[8].s64 = ctx.r[11].s64 + -15512;
	// 8329DD50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329DD54: 388AC3AC  addi r4, r10, -0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + -15444;
	// 8329DD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DD5C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DD68: 386A92A0  addi r3, r10, -0x6d60
	ctx.r[3].s64 = ctx.r[10].s64 + -28000;
	// 8329DD6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DD84: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329DD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DD8C: 4BAB7EB5  bl 0x82d55c40
	ctx.lr = 0x8329DD90;
	sub_82D55C40(ctx, base);
	// 8329DD90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DD94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DD98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DDA0 size=100
    let mut pc: u32 = 0x8329DDA0;
    'dispatch: loop {
        match pc {
            0x8329DDA0 => {
    //   block [0x8329DDA0..0x8329DE04)
	// 8329DDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DDAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DDB4: 38AA8D80  addi r5, r10, -0x7280
	ctx.r[5].s64 = ctx.r[10].s64 + -29312;
	// 8329DDB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DDC0: 388AC438  addi r4, r10, -0x3bc8
	ctx.r[4].s64 = ctx.r[10].s64 + -15304;
	// 8329DDC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DDC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DDD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DDD4: 386A92D0  addi r3, r10, -0x6d30
	ctx.r[3].s64 = ctx.r[10].s64 + -27952;
	// 8329DDD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DDDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DDE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DDE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DDEC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329DDF0: 4BAB7E51  bl 0x82d55c40
	ctx.lr = 0x8329DDF4;
	sub_82D55C40(ctx, base);
	// 8329DDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DE08 size=108
    let mut pc: u32 = 0x8329DE08;
    'dispatch: loop {
        match pc {
            0x8329DE08 => {
    //   block [0x8329DE08..0x8329DE74)
	// 8329DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DE14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DE18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DE1C: 38EBC460  addi r7, r11, -0x3ba0
	ctx.r[7].s64 = ctx.r[11].s64 + -15264;
	// 8329DE20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329DE24: 388AC520  addi r4, r10, -0x3ae0
	ctx.r[4].s64 = ctx.r[10].s64 + -15072;
	// 8329DE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DE2C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DE30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329DE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DE38: 386A9300  addi r3, r10, -0x6d00
	ctx.r[3].s64 = ctx.r[10].s64 + -27904;
	// 8329DE3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329DE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DE54: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329DE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DE5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329DE60: 4BAB7DE1  bl 0x82d55c40
	ctx.lr = 0x8329DE64;
	sub_82D55C40(ctx, base);
	// 8329DE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DE78 size=92
    let mut pc: u32 = 0x8329DE78;
    'dispatch: loop {
        match pc {
            0x8329DE78 => {
    //   block [0x8329DE78..0x8329DED4)
	// 8329DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DE80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DE84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329DE88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329DE8C: 4BAED8CD  bl 0x82d8b758
	ctx.lr = 0x8329DE90;
	sub_82D8B758(ctx, base);
	// 8329DE90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DE94: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329DE98: 394BC548  addi r10, r11, -0x3ab8
	ctx.r[10].s64 = ctx.r[11].s64 + -15032;
	// 8329DE9C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329DEA0: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329DEA4: 396B9330  addi r11, r11, -0x6cd0
	ctx.r[11].s64 = ctx.r[11].s64 + -27856;
	// 8329DEA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329DEAC: 39482A40  addi r10, r8, 0x2a40
	ctx.r[10].s64 = ctx.r[8].s64 + 10816;
	// 8329DEB0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329DEB4: 39492A58  addi r10, r9, 0x2a58
	ctx.r[10].s64 = ctx.r[9].s64 + 10840;
	// 8329DEB8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329DEBC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329DEC0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329DEC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8329DEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DED8 size=112
    let mut pc: u32 = 0x8329DED8;
    'dispatch: loop {
        match pc {
            0x8329DED8 => {
    //   block [0x8329DED8..0x8329DF48)
	// 8329DED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DEE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DEE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DEEC: 38AA8630  addi r5, r10, -0x79d0
	ctx.r[5].s64 = ctx.r[10].s64 + -31184;
	// 8329DEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DEF4: 390BC4A8  addi r8, r11, -0x3b58
	ctx.r[8].s64 = ctx.r[11].s64 + -15192;
	// 8329DEF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329DEFC: 388AC548  addi r4, r10, -0x3ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -15032;
	// 8329DF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DF04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DF10: 386A9340  addi r3, r10, -0x6cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -27840;
	// 8329DF14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DF2C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329DF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DF34: 4BAB7D0D  bl 0x82d55c40
	ctx.lr = 0x8329DF38;
	sub_82D55C40(ctx, base);
	// 8329DF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DF48 size=100
    let mut pc: u32 = 0x8329DF48;
    'dispatch: loop {
        match pc {
            0x8329DF48 => {
    //   block [0x8329DF48..0x8329DFAC)
	// 8329DF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DF54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DF5C: 38AA9240  addi r5, r10, -0x6dc0
	ctx.r[5].s64 = ctx.r[10].s64 + -28096;
	// 8329DF60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DF68: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 8329DF6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DF74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DF7C: 386A9370  addi r3, r10, -0x6c90
	ctx.r[3].s64 = ctx.r[10].s64 + -27792;
	// 8329DF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DF84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DF88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DF90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DF94: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329DF98: 4BAB7CA9  bl 0x82d55c40
	ctx.lr = 0x8329DF9C;
	sub_82D55C40(ctx, base);
	// 8329DF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DFB0 size=108
    let mut pc: u32 = 0x8329DFB0;
    'dispatch: loop {
        match pc {
            0x8329DFB0 => {
    //   block [0x8329DFB0..0x8329E01C)
	// 8329DFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DFBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DFC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DFC4: 38EBC5E4  addi r7, r11, -0x3a1c
	ctx.r[7].s64 = ctx.r[11].s64 + -14876;
	// 8329DFC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329DFCC: 388AC660  addi r4, r10, -0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + -14752;
	// 8329DFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DFD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329DFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DFE0: 386A93A0  addi r3, r10, -0x6c60
	ctx.r[3].s64 = ctx.r[10].s64 + -27744;
	// 8329DFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329DFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DFFC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329E000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E008: 4BAB7C39  bl 0x82d55c40
	ctx.lr = 0x8329E00C;
	sub_82D55C40(ctx, base);
	// 8329E00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E020 size=112
    let mut pc: u32 = 0x8329E020;
    'dispatch: loop {
        match pc {
            0x8329E020 => {
    //   block [0x8329E020..0x8329E090)
	// 8329E020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E02C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E030: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E034: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329E038: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E03C: 390BC614  addi r8, r11, -0x39ec
	ctx.r[8].s64 = ctx.r[11].s64 + -14828;
	// 8329E040: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E044: 388AC684  addi r4, r10, -0x397c
	ctx.r[4].s64 = ctx.r[10].s64 + -14716;
	// 8329E048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E04C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E058: 386A93D0  addi r3, r10, -0x6c30
	ctx.r[3].s64 = ctx.r[10].s64 + -27696;
	// 8329E05C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E074: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329E078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E07C: 4BAB7BC5  bl 0x82d55c40
	ctx.lr = 0x8329E080;
	sub_82D55C40(ctx, base);
	// 8329E080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E090 size=108
    let mut pc: u32 = 0x8329E090;
    'dispatch: loop {
        match pc {
            0x8329E090 => {
    //   block [0x8329E090..0x8329E0FC)
	// 8329E090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E09C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E0A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E0A4: 392BC768  addi r9, r11, -0x3898
	ctx.r[9].s64 = ctx.r[11].s64 + -14488;
	// 8329E0A8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329E0AC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329E0B0: 388AC8A4  addi r4, r10, -0x375c
	ctx.r[4].s64 = ctx.r[10].s64 + -14172;
	// 8329E0B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E0B8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E0BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329E0C0: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 8329E0C4: 386A9400  addi r3, r10, -0x6c00
	ctx.r[3].s64 = ctx.r[10].s64 + -27648;
	// 8329E0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E0CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329E0D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E0D4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E0D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E0DC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E0E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E0E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E0E8: 4BAB7B59  bl 0x82d55c40
	ctx.lr = 0x8329E0EC;
	sub_82D55C40(ctx, base);
	// 8329E0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E100 size=112
    let mut pc: u32 = 0x8329E100;
    'dispatch: loop {
        match pc {
            0x8329E100 => {
    //   block [0x8329E100..0x8329E170)
	// 8329E100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E10C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E110: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E114: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329E118: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E11C: 390BC828  addi r8, r11, -0x37d8
	ctx.r[8].s64 = ctx.r[11].s64 + -14296;
	// 8329E120: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329E124: 388AC8C0  addi r4, r10, -0x3740
	ctx.r[4].s64 = ctx.r[10].s64 + -14144;
	// 8329E128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E12C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E138: 386A9430  addi r3, r10, -0x6bd0
	ctx.r[3].s64 = ctx.r[10].s64 + -27600;
	// 8329E13C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E154: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 8329E158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E15C: 4BAB7AE5  bl 0x82d55c40
	ctx.lr = 0x8329E160;
	sub_82D55C40(ctx, base);
	// 8329E160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E170 size=100
    let mut pc: u32 = 0x8329E170;
    'dispatch: loop {
        match pc {
            0x8329E170 => {
    //   block [0x8329E170..0x8329E1D4)
	// 8329E170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E17C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E184: 38AA8D80  addi r5, r10, -0x7280
	ctx.r[5].s64 = ctx.r[10].s64 + -29312;
	// 8329E188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E190: 388AC940  addi r4, r10, -0x36c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14016;
	// 8329E194: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E1A4: 386A9460  addi r3, r10, -0x6ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -27552;
	// 8329E1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E1AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E1B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329E1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E1B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E1BC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329E1C0: 4BAB7A81  bl 0x82d55c40
	ctx.lr = 0x8329E1C4;
	sub_82D55C40(ctx, base);
	// 8329E1C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E1D8 size=112
    let mut pc: u32 = 0x8329E1D8;
    'dispatch: loop {
        match pc {
            0x8329E1D8 => {
    //   block [0x8329E1D8..0x8329E248)
	// 8329E1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E1E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E1E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E1EC: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329E1F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E1F4: 390BC990  addi r8, r11, -0x3670
	ctx.r[8].s64 = ctx.r[11].s64 + -13936;
	// 8329E1F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329E1FC: 388AC9EC  addi r4, r10, -0x3614
	ctx.r[4].s64 = ctx.r[10].s64 + -13844;
	// 8329E200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E204: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E210: 386A9490  addi r3, r10, -0x6b70
	ctx.r[3].s64 = ctx.r[10].s64 + -27504;
	// 8329E214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E22C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329E230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E234: 4BAB7A0D  bl 0x82d55c40
	ctx.lr = 0x8329E238;
	sub_82D55C40(ctx, base);
	// 8329E238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E248 size=92
    let mut pc: u32 = 0x8329E248;
    'dispatch: loop {
        match pc {
            0x8329E248 => {
    //   block [0x8329E248..0x8329E2A4)
	// 8329E248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E250: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E254: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329E258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329E25C: 4BADD70D  bl 0x82d7b968
	ctx.lr = 0x8329E260;
	sub_82D7B968(ctx, base);
	// 8329E260: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E264: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329E268: 394B8960  addi r10, r11, -0x76a0
	ctx.r[10].s64 = ctx.r[11].s64 + -30368;
	// 8329E26C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329E270: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329E274: 396B94C0  addi r11, r11, -0x6b40
	ctx.r[11].s64 = ctx.r[11].s64 + -27456;
	// 8329E278: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329E27C: 39482C28  addi r10, r8, 0x2c28
	ctx.r[10].s64 = ctx.r[8].s64 + 11304;
	// 8329E280: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329E284: 39492C40  addi r10, r9, 0x2c40
	ctx.r[10].s64 = ctx.r[9].s64 + 11328;
	// 8329E288: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329E28C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329E290: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329E294: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 8329E298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E2A8 size=116
    let mut pc: u32 = 0x8329E2A8;
    'dispatch: loop {
        match pc {
            0x8329E2A8 => {
    //   block [0x8329E2A8..0x8329E31C)
	// 8329E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E2B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E2B8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329E2BC: 392AD3CC  addi r9, r10, -0x2c34
	ctx.r[9].s64 = ctx.r[10].s64 + -11316;
	// 8329E2C0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329E2C4: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 8329E2C8: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329E2CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E2D0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8329E2D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E2D8: 388A8960  addi r4, r10, -0x76a0
	ctx.r[4].s64 = ctx.r[10].s64 + -30368;
	// 8329E2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E2E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329E2E4: 396BD3F8  addi r11, r11, -0x2c08
	ctx.r[11].s64 = ctx.r[11].s64 + -11272;
	// 8329E2E8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E2F0: 386A94D0  addi r3, r10, -0x6b30
	ctx.r[3].s64 = ctx.r[10].s64 + -27440;
	// 8329E2F4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329E2F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329E2FC: 38C00310  li r6, 0x310
	ctx.r[6].s64 = 784;
	// 8329E300: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329E304: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E308: 4BAB7939  bl 0x82d55c40
	ctx.lr = 0x8329E30C;
	sub_82D55C40(ctx, base);
	// 8329E30C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E320 size=92
    let mut pc: u32 = 0x8329E320;
    'dispatch: loop {
        match pc {
            0x8329E320 => {
    //   block [0x8329E320..0x8329E37C)
	// 8329E320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E328: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E32C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329E330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329E334: 4BAE38FD  bl 0x82d81c30
	ctx.lr = 0x8329E338;
	sub_82D81C30(ctx, base);
	// 8329E338: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E33C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329E340: 394BDF18  addi r10, r11, -0x20e8
	ctx.r[10].s64 = ctx.r[11].s64 + -8424;
	// 8329E344: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329E348: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329E34C: 396B9500  addi r11, r11, -0x6b00
	ctx.r[11].s64 = ctx.r[11].s64 + -27392;
	// 8329E350: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329E354: 39483BB8  addi r10, r8, 0x3bb8
	ctx.r[10].s64 = ctx.r[8].s64 + 15288;
	// 8329E358: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329E35C: 39493BD0  addi r10, r9, 0x3bd0
	ctx.r[10].s64 = ctx.r[9].s64 + 15312;
	// 8329E360: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329E364: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329E368: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329E36C: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8329E370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329E380 size=48
    let mut pc: u32 = 0x8329E380;
    'dispatch: loop {
        match pc {
            0x8329E380 => {
    //   block [0x8329E380..0x8329E3B0)
	// 8329E380: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E384: 814B96AC  lwz r10, -0x6954(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26964 as u32) ) } as u64;
	// 8329E388: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E38C: 396B96B0  addi r11, r11, -0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + -26960;
	// 8329E390: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8329E394: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329E398: 814A96A8  lwz r10, -0x6958(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26968 as u32) ) } as u64;
	// 8329E39C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8329E3A0: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329E3A4: 814A96A4  lwz r10, -0x695c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26972 as u32) ) } as u64;
	// 8329E3A8: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 8329E3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E3B0 size=116
    let mut pc: u32 = 0x8329E3B0;
    'dispatch: loop {
        match pc {
            0x8329E3B0 => {
    //   block [0x8329E3B0..0x8329E424)
	// 8329E3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E3BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E3C0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329E3C4: 392BDDC8  addi r9, r11, -0x2238
	ctx.r[9].s64 = ctx.r[11].s64 + -8760;
	// 8329E3C8: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329E3CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E3D0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8329E3D4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 8329E3D8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E3DC: 388ADF18  addi r4, r10, -0x20e8
	ctx.r[4].s64 = ctx.r[10].s64 + -8424;
	// 8329E3E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E3E4: 396B96B0  addi r11, r11, -0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + -26960;
	// 8329E3E8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8329E3EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E3F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329E3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E3F8: 386A9510  addi r3, r10, -0x6af0
	ctx.r[3].s64 = ctx.r[10].s64 + -27376;
	// 8329E3FC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8329E400: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8329E404: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8329E408: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329E40C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E410: 4BAB7831  bl 0x82d55c40
	ctx.lr = 0x8329E414;
	sub_82D55C40(ctx, base);
	// 8329E414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E41C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E428 size=112
    let mut pc: u32 = 0x8329E428;
    'dispatch: loop {
        match pc {
            0x8329E428 => {
    //   block [0x8329E428..0x8329E498)
	// 8329E428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E434: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329E438: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E43C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329E440: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E444: 390BE0E4  addi r8, r11, -0x1f1c
	ctx.r[8].s64 = ctx.r[11].s64 + -7964;
	// 8329E448: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E44C: 388AE0FC  addi r4, r10, -0x1f04
	ctx.r[4].s64 = ctx.r[10].s64 + -7940;
	// 8329E450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E454: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E458: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E460: 386A9540  addi r3, r10, -0x6ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -27328;
	// 8329E464: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E47C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E484: 4BAB77BD  bl 0x82d55c40
	ctx.lr = 0x8329E488;
	sub_82D55C40(ctx, base);
	// 8329E488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329E498 size=24
    let mut pc: u32 = 0x8329E498;
    'dispatch: loop {
        match pc {
            0x8329E498 => {
    //   block [0x8329E498..0x8329E4B0)
	// 8329E498: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E49C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329E4A0: 394AA10C  addi r10, r10, -0x5ef4
	ctx.r[10].s64 = ctx.r[10].s64 + -24308;
	// 8329E4A4: 816B9F30  lwz r11, -0x60d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24784 as u32) ) } as u64;
	// 8329E4A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329E4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E4B0 size=112
    let mut pc: u32 = 0x8329E4B0;
    'dispatch: loop {
        match pc {
            0x8329E4B0 => {
    //   block [0x8329E4B0..0x8329E520)
	// 8329E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E4BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E4C0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E4C4: 392AEFD0  addi r9, r10, -0x1030
	ctx.r[9].s64 = ctx.r[10].s64 + -4144;
	// 8329E4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E4CC: 390BA10C  addi r8, r11, -0x5ef4
	ctx.r[8].s64 = ctx.r[11].s64 + -24308;
	// 8329E4D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329E4D4: 388AF7E0  addi r4, r10, -0x820
	ctx.r[4].s64 = ctx.r[10].s64 + -2080;
	// 8329E4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E4DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E4E4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8329E4E8: 386A9570  addi r3, r10, -0x6a90
	ctx.r[3].s64 = ctx.r[10].s64 + -27280;
	// 8329E4EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329E4F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329E4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E50C: 4BAB7735  bl 0x82d55c40
	ctx.lr = 0x8329E510;
	sub_82D55C40(ctx, base);
	// 8329E510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E520 size=112
    let mut pc: u32 = 0x8329E520;
    'dispatch: loop {
        match pc {
            0x8329E520 => {
    //   block [0x8329E520..0x8329E590)
	// 8329E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E52C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E530: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E534: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E53C: 390BEFF8  addi r8, r11, -0x1008
	ctx.r[8].s64 = ctx.r[11].s64 + -4104;
	// 8329E540: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E544: 388AF7F4  addi r4, r10, -0x80c
	ctx.r[4].s64 = ctx.r[10].s64 + -2060;
	// 8329E548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E54C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E558: 386A95A0  addi r3, r10, -0x6a60
	ctx.r[3].s64 = ctx.r[10].s64 + -27232;
	// 8329E55C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E574: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E57C: 4BAB76C5  bl 0x82d55c40
	ctx.lr = 0x8329E580;
	sub_82D55C40(ctx, base);
	// 8329E580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E590 size=108
    let mut pc: u32 = 0x8329E590;
    'dispatch: loop {
        match pc {
            0x8329E590 => {
    //   block [0x8329E590..0x8329E5FC)
	// 8329E590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E59C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E5A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E5A4: 38EBF028  addi r7, r11, -0xfd8
	ctx.r[7].s64 = ctx.r[11].s64 + -4056;
	// 8329E5A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329E5AC: 388AF80C  addi r4, r10, -0x7f4
	ctx.r[4].s64 = ctx.r[10].s64 + -2036;
	// 8329E5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E5B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E5B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329E5BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E5C0: 386A95D0  addi r3, r10, -0x6a30
	ctx.r[3].s64 = ctx.r[10].s64 + -27184;
	// 8329E5C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329E5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E5CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E5DC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E5E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E5E8: 4BAB7659  bl 0x82d55c40
	ctx.lr = 0x8329E5EC;
	sub_82D55C40(ctx, base);
	// 8329E5EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E600 size=112
    let mut pc: u32 = 0x8329E600;
    'dispatch: loop {
        match pc {
            0x8329E600 => {
    //   block [0x8329E600..0x8329E670)
	// 8329E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E60C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E610: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E614: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E61C: 390BF040  addi r8, r11, -0xfc0
	ctx.r[8].s64 = ctx.r[11].s64 + -4032;
	// 8329E620: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8329E624: 388AF81C  addi r4, r10, -0x7e4
	ctx.r[4].s64 = ctx.r[10].s64 + -2020;
	// 8329E628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E62C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E638: 386A9600  addi r3, r10, -0x6a00
	ctx.r[3].s64 = ctx.r[10].s64 + -27136;
	// 8329E63C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E654: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329E658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E65C: 4BAB75E5  bl 0x82d55c40
	ctx.lr = 0x8329E660;
	sub_82D55C40(ctx, base);
	// 8329E660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E670 size=100
    let mut pc: u32 = 0x8329E670;
    'dispatch: loop {
        match pc {
            0x8329E670 => {
    //   block [0x8329E670..0x8329E6D4)
	// 8329E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E67C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E684: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E688: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E690: 388AF83C  addi r4, r10, -0x7c4
	ctx.r[4].s64 = ctx.r[10].s64 + -1988;
	// 8329E694: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E6A4: 386A9630  addi r3, r10, -0x69d0
	ctx.r[3].s64 = ctx.r[10].s64 + -27088;
	// 8329E6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E6AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E6B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329E6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E6B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E6BC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8329E6C0: 4BAB7581  bl 0x82d55c40
	ctx.lr = 0x8329E6C4;
	sub_82D55C40(ctx, base);
	// 8329E6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E6D8 size=112
    let mut pc: u32 = 0x8329E6D8;
    'dispatch: loop {
        match pc {
            0x8329E6D8 => {
    //   block [0x8329E6D8..0x8329E748)
	// 8329E6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E6E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E6E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E6EC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E6F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E6F4: 390BF100  addi r8, r11, -0xf00
	ctx.r[8].s64 = ctx.r[11].s64 + -3840;
	// 8329E6F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E6FC: 388AF858  addi r4, r10, -0x7a8
	ctx.r[4].s64 = ctx.r[10].s64 + -1960;
	// 8329E700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E704: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E710: 386A9660  addi r3, r10, -0x69a0
	ctx.r[3].s64 = ctx.r[10].s64 + -27040;
	// 8329E714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E72C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329E730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E734: 4BAB750D  bl 0x82d55c40
	ctx.lr = 0x8329E738;
	sub_82D55C40(ctx, base);
	// 8329E738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E748 size=112
    let mut pc: u32 = 0x8329E748;
    'dispatch: loop {
        match pc {
            0x8329E748 => {
    //   block [0x8329E748..0x8329E7B8)
	// 8329E748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E754: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E758: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E75C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E764: 390BF118  addi r8, r11, -0xee8
	ctx.r[8].s64 = ctx.r[11].s64 + -3816;
	// 8329E768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E76C: 388AF878  addi r4, r10, -0x788
	ctx.r[4].s64 = ctx.r[10].s64 + -1928;
	// 8329E770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E774: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E780: 386A9690  addi r3, r10, -0x6970
	ctx.r[3].s64 = ctx.r[10].s64 + -26992;
	// 8329E784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E79C: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 8329E7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E7A4: 4BAB749D  bl 0x82d55c40
	ctx.lr = 0x8329E7A8;
	sub_82D55C40(ctx, base);
	// 8329E7A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E7B8 size=112
    let mut pc: u32 = 0x8329E7B8;
    'dispatch: loop {
        match pc {
            0x8329E7B8 => {
    //   block [0x8329E7B8..0x8329E828)
	// 8329E7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E7C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E7C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E7CC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E7D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E7D4: 390BF148  addi r8, r11, -0xeb8
	ctx.r[8].s64 = ctx.r[11].s64 + -3768;
	// 8329E7D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E7DC: 388AF89C  addi r4, r10, -0x764
	ctx.r[4].s64 = ctx.r[10].s64 + -1892;
	// 8329E7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E7E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E7F0: 386A96C0  addi r3, r10, -0x6940
	ctx.r[3].s64 = ctx.r[10].s64 + -26944;
	// 8329E7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E80C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329E810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E814: 4BAB742D  bl 0x82d55c40
	ctx.lr = 0x8329E818;
	sub_82D55C40(ctx, base);
	// 8329E818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E828 size=112
    let mut pc: u32 = 0x8329E828;
    'dispatch: loop {
        match pc {
            0x8329E828 => {
    //   block [0x8329E828..0x8329E898)
	// 8329E828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E834: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E838: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E83C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E840: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E844: 390BF178  addi r8, r11, -0xe88
	ctx.r[8].s64 = ctx.r[11].s64 + -3720;
	// 8329E848: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E84C: 388AF8C4  addi r4, r10, -0x73c
	ctx.r[4].s64 = ctx.r[10].s64 + -1852;
	// 8329E850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E854: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E860: 386A96F0  addi r3, r10, -0x6910
	ctx.r[3].s64 = ctx.r[10].s64 + -26896;
	// 8329E864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E87C: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329E880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E884: 4BAB73BD  bl 0x82d55c40
	ctx.lr = 0x8329E888;
	sub_82D55C40(ctx, base);
	// 8329E888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E898 size=112
    let mut pc: u32 = 0x8329E898;
    'dispatch: loop {
        match pc {
            0x8329E898 => {
    //   block [0x8329E898..0x8329E908)
	// 8329E898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E8A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E8A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E8AC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E8B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E8B4: 390BF1A8  addi r8, r11, -0xe58
	ctx.r[8].s64 = ctx.r[11].s64 + -3672;
	// 8329E8B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E8BC: 388AF8E8  addi r4, r10, -0x718
	ctx.r[4].s64 = ctx.r[10].s64 + -1816;
	// 8329E8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E8C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E8D0: 386A9720  addi r3, r10, -0x68e0
	ctx.r[3].s64 = ctx.r[10].s64 + -26848;
	// 8329E8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E8EC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329E8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E8F4: 4BAB734D  bl 0x82d55c40
	ctx.lr = 0x8329E8F8;
	sub_82D55C40(ctx, base);
	// 8329E8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E908 size=112
    let mut pc: u32 = 0x8329E908;
    'dispatch: loop {
        match pc {
            0x8329E908 => {
    //   block [0x8329E908..0x8329E978)
	// 8329E908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E914: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E918: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E91C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E920: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E924: 390BF1C0  addi r8, r11, -0xe40
	ctx.r[8].s64 = ctx.r[11].s64 + -3648;
	// 8329E928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E92C: 388AF908  addi r4, r10, -0x6f8
	ctx.r[4].s64 = ctx.r[10].s64 + -1784;
	// 8329E930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E934: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E940: 386A9750  addi r3, r10, -0x68b0
	ctx.r[3].s64 = ctx.r[10].s64 + -26800;
	// 8329E944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E95C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329E960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E964: 4BAB72DD  bl 0x82d55c40
	ctx.lr = 0x8329E968;
	sub_82D55C40(ctx, base);
	// 8329E968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E978 size=112
    let mut pc: u32 = 0x8329E978;
    'dispatch: loop {
        match pc {
            0x8329E978 => {
    //   block [0x8329E978..0x8329E9E8)
	// 8329E978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E984: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E988: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E98C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E994: 390BF1D8  addi r8, r11, -0xe28
	ctx.r[8].s64 = ctx.r[11].s64 + -3624;
	// 8329E998: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329E99C: 388AF920  addi r4, r10, -0x6e0
	ctx.r[4].s64 = ctx.r[10].s64 + -1760;
	// 8329E9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E9A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E9A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E9B0: 386A9780  addi r3, r10, -0x6880
	ctx.r[3].s64 = ctx.r[10].s64 + -26752;
	// 8329E9B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E9BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E9CC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E9D4: 4BAB726D  bl 0x82d55c40
	ctx.lr = 0x8329E9D8;
	sub_82D55C40(ctx, base);
	// 8329E9D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E9E8 size=112
    let mut pc: u32 = 0x8329E9E8;
    'dispatch: loop {
        match pc {
            0x8329E9E8 => {
    //   block [0x8329E9E8..0x8329EA58)
	// 8329E9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E9F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E9F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E9FC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EA00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EA04: 390BF220  addi r8, r11, -0xde0
	ctx.r[8].s64 = ctx.r[11].s64 + -3552;
	// 8329EA08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329EA0C: 388AF93C  addi r4, r10, -0x6c4
	ctx.r[4].s64 = ctx.r[10].s64 + -1732;
	// 8329EA10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EA14: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EA18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EA20: 386A97B0  addi r3, r10, -0x6850
	ctx.r[3].s64 = ctx.r[10].s64 + -26704;
	// 8329EA24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EA28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EA30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EA38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EA3C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329EA40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EA44: 4BAB71FD  bl 0x82d55c40
	ctx.lr = 0x8329EA48;
	sub_82D55C40(ctx, base);
	// 8329EA48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EA58 size=112
    let mut pc: u32 = 0x8329EA58;
    'dispatch: loop {
        match pc {
            0x8329EA58 => {
    //   block [0x8329EA58..0x8329EAC8)
	// 8329EA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EA64: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EA68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EA6C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EA70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EA74: 390BF268  addi r8, r11, -0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + -3480;
	// 8329EA78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329EA7C: 388AF958  addi r4, r10, -0x6a8
	ctx.r[4].s64 = ctx.r[10].s64 + -1704;
	// 8329EA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EA84: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EA88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EA90: 386A97E0  addi r3, r10, -0x6820
	ctx.r[3].s64 = ctx.r[10].s64 + -26656;
	// 8329EA94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EA98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EAA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EAA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EAAC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329EAB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EAB4: 4BAB718D  bl 0x82d55c40
	ctx.lr = 0x8329EAB8;
	sub_82D55C40(ctx, base);
	// 8329EAB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EAC8 size=112
    let mut pc: u32 = 0x8329EAC8;
    'dispatch: loop {
        match pc {
            0x8329EAC8 => {
    //   block [0x8329EAC8..0x8329EB38)
	// 8329EAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EAD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EAD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EADC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EAE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EAE4: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 8329EAE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329EAEC: 388AF970  addi r4, r10, -0x690
	ctx.r[4].s64 = ctx.r[10].s64 + -1680;
	// 8329EAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EAF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EAF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EB00: 386A9810  addi r3, r10, -0x67f0
	ctx.r[3].s64 = ctx.r[10].s64 + -26608;
	// 8329EB04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EB08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EB1C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329EB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EB24: 4BAB711D  bl 0x82d55c40
	ctx.lr = 0x8329EB28;
	sub_82D55C40(ctx, base);
	// 8329EB28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EB38 size=112
    let mut pc: u32 = 0x8329EB38;
    'dispatch: loop {
        match pc {
            0x8329EB38 => {
    //   block [0x8329EB38..0x8329EBA8)
	// 8329EB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EB44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EB48: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EB4C: 396BF2B0  addi r11, r11, -0xd50
	ctx.r[11].s64 = ctx.r[11].s64 + -3408;
	// 8329EB50: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EB54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EB58: 390B0078  addi r8, r11, 0x78
	ctx.r[8].s64 = ctx.r[11].s64 + 120;
	// 8329EB5C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329EB60: 388AF988  addi r4, r10, -0x678
	ctx.r[4].s64 = ctx.r[10].s64 + -1656;
	// 8329EB64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329EB68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EB6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EB70: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329EB74: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329EB78: 386A9840  addi r3, r10, -0x67c0
	ctx.r[3].s64 = ctx.r[10].s64 + -26560;
	// 8329EB7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329EB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EB88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329EB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EB90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329EB94: 4BAB70AD  bl 0x82d55c40
	ctx.lr = 0x8329EB98;
	sub_82D55C40(ctx, base);
	// 8329EB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EBA8 size=112
    let mut pc: u32 = 0x8329EBA8;
    'dispatch: loop {
        match pc {
            0x8329EBA8 => {
    //   block [0x8329EBA8..0x8329EC18)
	// 8329EBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EBB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EBB8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EBBC: 396BF340  addi r11, r11, -0xcc0
	ctx.r[11].s64 = ctx.r[11].s64 + -3264;
	// 8329EBC0: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EBC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EBC8: 390B0090  addi r8, r11, 0x90
	ctx.r[8].s64 = ctx.r[11].s64 + 144;
	// 8329EBCC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329EBD0: 388AF9A4  addi r4, r10, -0x65c
	ctx.r[4].s64 = ctx.r[10].s64 + -1628;
	// 8329EBD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329EBD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EBDC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EBE0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329EBE4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329EBE8: 386A9870  addi r3, r10, -0x6790
	ctx.r[3].s64 = ctx.r[10].s64 + -26512;
	// 8329EBEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329EBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EBF8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329EBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EC00: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329EC04: 4BAB703D  bl 0x82d55c40
	ctx.lr = 0x8329EC08;
	sub_82D55C40(ctx, base);
	// 8329EC08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329EC18 size=24
    let mut pc: u32 = 0x8329EC18;
    'dispatch: loop {
        match pc {
            0x8329EC18 => {
    //   block [0x8329EC18..0x8329EC30)
	// 8329EC18: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329EC1C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329EC20: 394AA128  addi r10, r10, -0x5ed8
	ctx.r[10].s64 = ctx.r[10].s64 + -24280;
	// 8329EC24: 816B9F38  lwz r11, -0x60c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24776 as u32) ) } as u64;
	// 8329EC28: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329EC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EC30 size=116
    let mut pc: u32 = 0x8329EC30;
    'dispatch: loop {
        match pc {
            0x8329EC30 => {
    //   block [0x8329EC30..0x8329ECA4)
	// 8329EC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EC3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EC40: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EC44: 392BF3FC  addi r9, r11, -0xc04
	ctx.r[9].s64 = ctx.r[11].s64 + -3076;
	// 8329EC48: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EC4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EC50: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8329EC54: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329EC58: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329EC5C: 388AF9C0  addi r4, r10, -0x640
	ctx.r[4].s64 = ctx.r[10].s64 + -1600;
	// 8329EC60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EC64: 396BA128  addi r11, r11, -0x5ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -24280;
	// 8329EC68: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8329EC6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EC70: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329EC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EC78: 386A98A0  addi r3, r10, -0x6760
	ctx.r[3].s64 = ctx.r[10].s64 + -26464;
	// 8329EC7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329EC80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8329EC84: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329EC88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329EC8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329EC90: 4BAB6FB1  bl 0x82d55c40
	ctx.lr = 0x8329EC94;
	sub_82D55C40(ctx, base);
	// 8329EC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ECA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ECA8 size=112
    let mut pc: u32 = 0x8329ECA8;
    'dispatch: loop {
        match pc {
            0x8329ECA8 => {
    //   block [0x8329ECA8..0x8329ED18)
	// 8329ECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ECAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ECB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ECB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ECB8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329ECBC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329ECC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329ECC4: 390BF438  addi r8, r11, -0xbc8
	ctx.r[8].s64 = ctx.r[11].s64 + -3016;
	// 8329ECC8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329ECCC: 388AF9F4  addi r4, r10, -0x60c
	ctx.r[4].s64 = ctx.r[10].s64 + -1548;
	// 8329ECD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ECD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ECD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329ECDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ECE0: 386A98D0  addi r3, r10, -0x6730
	ctx.r[3].s64 = ctx.r[10].s64 + -26416;
	// 8329ECE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329ECE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ECEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ECF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ECF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ECF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ECFC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329ED00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ED04: 4BAB6F3D  bl 0x82d55c40
	ctx.lr = 0x8329ED08;
	sub_82D55C40(ctx, base);
	// 8329ED08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ED0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ED10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ED14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ED18 size=112
    let mut pc: u32 = 0x8329ED18;
    'dispatch: loop {
        match pc {
            0x8329ED18 => {
    //   block [0x8329ED18..0x8329ED88)
	// 8329ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ED20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ED24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ED28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329ED2C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329ED30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329ED34: 390BF498  addi r8, r11, -0xb68
	ctx.r[8].s64 = ctx.r[11].s64 + -2920;
	// 8329ED38: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8329ED3C: 388AFA14  addi r4, r10, -0x5ec
	ctx.r[4].s64 = ctx.r[10].s64 + -1516;
	// 8329ED40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ED44: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ED48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329ED4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ED50: 386A9900  addi r3, r10, -0x6700
	ctx.r[3].s64 = ctx.r[10].s64 + -26368;
	// 8329ED54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329ED58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ED5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ED60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ED64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ED68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ED6C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329ED70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ED74: 4BAB6ECD  bl 0x82d55c40
	ctx.lr = 0x8329ED78;
	sub_82D55C40(ctx, base);
	// 8329ED78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ED7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ED80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ED84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ED88 size=112
    let mut pc: u32 = 0x8329ED88;
    'dispatch: loop {
        match pc {
            0x8329ED88 => {
    //   block [0x8329ED88..0x8329EDF8)
	// 8329ED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ED8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ED90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ED94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ED98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329ED9C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EDA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EDA4: 390BF540  addi r8, r11, -0xac0
	ctx.r[8].s64 = ctx.r[11].s64 + -2752;
	// 8329EDA8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329EDAC: 388AFA30  addi r4, r10, -0x5d0
	ctx.r[4].s64 = ctx.r[10].s64 + -1488;
	// 8329EDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EDB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EDC0: 386A9930  addi r3, r10, -0x66d0
	ctx.r[3].s64 = ctx.r[10].s64 + -26320;
	// 8329EDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EDDC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329EDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EDE4: 4BAB6E5D  bl 0x82d55c40
	ctx.lr = 0x8329EDE8;
	sub_82D55C40(ctx, base);
	// 8329EDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EDF8 size=112
    let mut pc: u32 = 0x8329EDF8;
    'dispatch: loop {
        match pc {
            0x8329EDF8 => {
    //   block [0x8329EDF8..0x8329EE68)
	// 8329EDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EE04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EE0C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EE14: 390BF5B8  addi r8, r11, -0xa48
	ctx.r[8].s64 = ctx.r[11].s64 + -2632;
	// 8329EE18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329EE1C: 388AFA50  addi r4, r10, -0x5b0
	ctx.r[4].s64 = ctx.r[10].s64 + -1456;
	// 8329EE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EE24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EE30: 386A9960  addi r3, r10, -0x66a0
	ctx.r[3].s64 = ctx.r[10].s64 + -26272;
	// 8329EE34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EE4C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329EE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EE54: 4BAB6DED  bl 0x82d55c40
	ctx.lr = 0x8329EE58;
	sub_82D55C40(ctx, base);
	// 8329EE58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EE68 size=112
    let mut pc: u32 = 0x8329EE68;
    'dispatch: loop {
        match pc {
            0x8329EE68 => {
    //   block [0x8329EE68..0x8329EED8)
	// 8329EE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EE74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EE7C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EE80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EE84: 390BF600  addi r8, r11, -0xa00
	ctx.r[8].s64 = ctx.r[11].s64 + -2560;
	// 8329EE88: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329EE8C: 388AFA70  addi r4, r10, -0x590
	ctx.r[4].s64 = ctx.r[10].s64 + -1424;
	// 8329EE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EE94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EEA0: 386A9990  addi r3, r10, -0x6670
	ctx.r[3].s64 = ctx.r[10].s64 + -26224;
	// 8329EEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EEBC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329EEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EEC4: 4BAB6D7D  bl 0x82d55c40
	ctx.lr = 0x8329EEC8;
	sub_82D55C40(ctx, base);
	// 8329EEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EED8 size=112
    let mut pc: u32 = 0x8329EED8;
    'dispatch: loop {
        match pc {
            0x8329EED8 => {
    //   block [0x8329EED8..0x8329EF48)
	// 8329EED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EEE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EEE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EEEC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EEF4: 390BF690  addi r8, r11, -0x970
	ctx.r[8].s64 = ctx.r[11].s64 + -2416;
	// 8329EEF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329EEFC: 388AFA8C  addi r4, r10, -0x574
	ctx.r[4].s64 = ctx.r[10].s64 + -1396;
	// 8329EF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EF04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EF10: 386A99C0  addi r3, r10, -0x6640
	ctx.r[3].s64 = ctx.r[10].s64 + -26176;
	// 8329EF14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EF2C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329EF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EF34: 4BAB6D0D  bl 0x82d55c40
	ctx.lr = 0x8329EF38;
	sub_82D55C40(ctx, base);
	// 8329EF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EF48 size=112
    let mut pc: u32 = 0x8329EF48;
    'dispatch: loop {
        match pc {
            0x8329EF48 => {
    //   block [0x8329EF48..0x8329EFB8)
	// 8329EF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EF54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EF58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EF5C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EF60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EF64: 390BF6F0  addi r8, r11, -0x910
	ctx.r[8].s64 = ctx.r[11].s64 + -2320;
	// 8329EF68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329EF6C: 388AFAA4  addi r4, r10, -0x55c
	ctx.r[4].s64 = ctx.r[10].s64 + -1372;
	// 8329EF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EF74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EF80: 386A99F0  addi r3, r10, -0x6610
	ctx.r[3].s64 = ctx.r[10].s64 + -26128;
	// 8329EF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EF9C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329EFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EFA4: 4BAB6C9D  bl 0x82d55c40
	ctx.lr = 0x8329EFA8;
	sub_82D55C40(ctx, base);
	// 8329EFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EFB8 size=112
    let mut pc: u32 = 0x8329EFB8;
    'dispatch: loop {
        match pc {
            0x8329EFB8 => {
    //   block [0x8329EFB8..0x8329F028)
	// 8329EFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EFC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EFC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EFCC: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329EFD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EFD4: 390BF750  addi r8, r11, -0x8b0
	ctx.r[8].s64 = ctx.r[11].s64 + -2224;
	// 8329EFD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329EFDC: 388AFAC0  addi r4, r10, -0x540
	ctx.r[4].s64 = ctx.r[10].s64 + -1344;
	// 8329EFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EFE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EFF0: 386A9A20  addi r3, r10, -0x65e0
	ctx.r[3].s64 = ctx.r[10].s64 + -26080;
	// 8329EFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F00C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329F010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F014: 4BAB6C2D  bl 0x82d55c40
	ctx.lr = 0x8329F018;
	sub_82D55C40(ctx, base);
	// 8329F018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F028 size=112
    let mut pc: u32 = 0x8329F028;
    'dispatch: loop {
        match pc {
            0x8329F028 => {
    //   block [0x8329F028..0x8329F098)
	// 8329F028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F034: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F038: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F03C: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329F040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F044: 390BF780  addi r8, r11, -0x880
	ctx.r[8].s64 = ctx.r[11].s64 + -2176;
	// 8329F048: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329F04C: 388AFAE8  addi r4, r10, -0x518
	ctx.r[4].s64 = ctx.r[10].s64 + -1304;
	// 8329F050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F054: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F060: 386A9A50  addi r3, r10, -0x65b0
	ctx.r[3].s64 = ctx.r[10].s64 + -26032;
	// 8329F064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F07C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329F080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F084: 4BAB6BBD  bl 0x82d55c40
	ctx.lr = 0x8329F088;
	sub_82D55C40(ctx, base);
	// 8329F088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F098 size=100
    let mut pc: u32 = 0x8329F098;
    'dispatch: loop {
        match pc {
            0x8329F098 => {
    //   block [0x8329F098..0x8329F0FC)
	// 8329F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F0A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F0AC: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329F0B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F0B8: 388AFB10  addi r4, r10, -0x4f0
	ctx.r[4].s64 = ctx.r[10].s64 + -1264;
	// 8329F0BC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F0C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F0CC: 386A9A80  addi r3, r10, -0x6580
	ctx.r[3].s64 = ctx.r[10].s64 + -25984;
	// 8329F0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F0D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F0D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329F0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F0E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329F0E4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329F0E8: 4BAB6B59  bl 0x82d55c40
	ctx.lr = 0x8329F0EC;
	sub_82D55C40(ctx, base);
	// 8329F0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F100 size=112
    let mut pc: u32 = 0x8329F100;
    'dispatch: loop {
        match pc {
            0x8329F100 => {
    //   block [0x8329F100..0x8329F170)
	// 8329F100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F10C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F110: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F114: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329F118: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F11C: 390BF7C8  addi r8, r11, -0x838
	ctx.r[8].s64 = ctx.r[11].s64 + -2104;
	// 8329F120: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F124: 388AFB38  addi r4, r10, -0x4c8
	ctx.r[4].s64 = ctx.r[10].s64 + -1224;
	// 8329F128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F12C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F138: 386A9AB0  addi r3, r10, -0x6550
	ctx.r[3].s64 = ctx.r[10].s64 + -25936;
	// 8329F13C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F154: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329F158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F15C: 4BAB6AE5  bl 0x82d55c40
	ctx.lr = 0x8329F160;
	sub_82D55C40(ctx, base);
	// 8329F160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329F170 size=12
    let mut pc: u32 = 0x8329F170;
    'dispatch: loop {
        match pc {
            0x8329F170 => {
    //   block [0x8329F170..0x8329F17C)
	// 8329F170: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329F174: 386B7ED0  addi r3, r11, 0x7ed0
	ctx.r[3].s64 = ctx.r[11].s64 + 32464;
	// 8329F178: 4BA0ADA8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F180 size=108
    let mut pc: u32 = 0x8329F180;
    'dispatch: loop {
        match pc {
            0x8329F180 => {
    //   block [0x8329F180..0x8329F1EC)
	// 8329F180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F18C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F194: 38EBFF20  addi r7, r11, -0xe0
	ctx.r[7].s64 = ctx.r[11].s64 + -224;
	// 8329F198: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329F19C: 388AFF68  addi r4, r10, -0x98
	ctx.r[4].s64 = ctx.r[10].s64 + -152;
	// 8329F1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F1A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329F1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F1B0: 386A9B24  addi r3, r10, -0x64dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25820;
	// 8329F1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329F1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F1CC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329F1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F1D8: 4BAB6A69  bl 0x82d55c40
	ctx.lr = 0x8329F1DC;
	sub_82D55C40(ctx, base);
	// 8329F1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F1F0 size=96
    let mut pc: u32 = 0x8329F1F0;
    'dispatch: loop {
        match pc {
            0x8329F1F0 => {
    //   block [0x8329F1F0..0x8329F250)
	// 8329F1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F1FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F204: 388A0108  addi r4, r10, 0x108
	ctx.r[4].s64 = ctx.r[10].s64 + 264;
	// 8329F208: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F210: 386A9B74  addi r3, r10, -0x648c
	ctx.r[3].s64 = ctx.r[10].s64 + -25740;
	// 8329F214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F21C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329F220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F22C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329F230: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329F234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F238: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329F23C: 4BAB6A05  bl 0x82d55c40
	ctx.lr = 0x8329F240;
	sub_82D55C40(ctx, base);
	// 8329F240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F250 size=112
    let mut pc: u32 = 0x8329F250;
    'dispatch: loop {
        match pc {
            0x8329F250 => {
    //   block [0x8329F250..0x8329F2C0)
	// 8329F250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F25C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F260: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F264: 38AA9B74  addi r5, r10, -0x648c
	ctx.r[5].s64 = ctx.r[10].s64 + -25740;
	// 8329F268: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F26C: 390B00A8  addi r8, r11, 0xa8
	ctx.r[8].s64 = ctx.r[11].s64 + 168;
	// 8329F270: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F274: 388A011C  addi r4, r10, 0x11c
	ctx.r[4].s64 = ctx.r[10].s64 + 284;
	// 8329F278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F27C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F288: 386A9BA4  addi r3, r10, -0x645c
	ctx.r[3].s64 = ctx.r[10].s64 + -25692;
	// 8329F28C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F2A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F2A4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329F2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F2AC: 4BAB6995  bl 0x82d55c40
	ctx.lr = 0x8329F2B0;
	sub_82D55C40(ctx, base);
	// 8329F2B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F2C0 size=108
    let mut pc: u32 = 0x8329F2C0;
    'dispatch: loop {
        match pc {
            0x8329F2C0 => {
    //   block [0x8329F2C0..0x8329F32C)
	// 8329F2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F2CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F2D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F2D4: 38EB0178  addi r7, r11, 0x178
	ctx.r[7].s64 = ctx.r[11].s64 + 376;
	// 8329F2D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329F2DC: 388A023C  addi r4, r10, 0x23c
	ctx.r[4].s64 = ctx.r[10].s64 + 572;
	// 8329F2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F2E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329F2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F2F0: 386A9BD4  addi r3, r10, -0x642c
	ctx.r[3].s64 = ctx.r[10].s64 + -25644;
	// 8329F2F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329F2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F30C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329F310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F318: 4BAB6929  bl 0x82d55c40
	ctx.lr = 0x8329F31C;
	sub_82D55C40(ctx, base);
	// 8329F31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329F330 size=24
    let mut pc: u32 = 0x8329F330;
    'dispatch: loop {
        match pc {
            0x8329F330 => {
    //   block [0x8329F330..0x8329F348)
	// 8329F330: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329F334: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329F338: 394AA4B0  addi r10, r10, -0x5b50
	ctx.r[10].s64 = ctx.r[10].s64 + -23376;
	// 8329F33C: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 8329F340: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8329F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F348 size=116
    let mut pc: u32 = 0x8329F348;
    'dispatch: loop {
        match pc {
            0x8329F348 => {
    //   block [0x8329F348..0x8329F3BC)
	// 8329F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F354: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329F358: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329F35C: 390AA4B0  addi r8, r10, -0x5b50
	ctx.r[8].s64 = ctx.r[10].s64 + -23376;
	// 8329F360: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F364: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F368: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 8329F36C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F370: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329F374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F37C: 388A0258  addi r4, r10, 0x258
	ctx.r[4].s64 = ctx.r[10].s64 + 600;
	// 8329F380: 396B01D8  addi r11, r11, 0x1d8
	ctx.r[11].s64 = ctx.r[11].s64 + 472;
	// 8329F384: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F388: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F38C: 386A9C04  addi r3, r10, -0x63fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25596;
	// 8329F390: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329F394: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F398: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329F39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F3A4: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 8329F3A8: 4BAB6899  bl 0x82d55c40
	ctx.lr = 0x8329F3AC;
	sub_82D55C40(ctx, base);
	// 8329F3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F3C0 size=112
    let mut pc: u32 = 0x8329F3C0;
    'dispatch: loop {
        match pc {
            0x8329F3C0 => {
    //   block [0x8329F3C0..0x8329F430)
	// 8329F3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F3CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F3D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F3D4: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329F3D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F3DC: 390B026C  addi r8, r11, 0x26c
	ctx.r[8].s64 = ctx.r[11].s64 + 620;
	// 8329F3E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F3E4: 388A0284  addi r4, r10, 0x284
	ctx.r[4].s64 = ctx.r[10].s64 + 644;
	// 8329F3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F3EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F3F8: 386A9C34  addi r3, r10, -0x63cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25548;
	// 8329F3FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F414: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329F418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F41C: 4BAB6825  bl 0x82d55c40
	ctx.lr = 0x8329F420;
	sub_82D55C40(ctx, base);
	// 8329F420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F430 size=112
    let mut pc: u32 = 0x8329F430;
    'dispatch: loop {
        match pc {
            0x8329F430 => {
    //   block [0x8329F430..0x8329F4A0)
	// 8329F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F43C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F440: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F444: 38AAAD98  addi r5, r10, -0x5268
	ctx.r[5].s64 = ctx.r[10].s64 + -21096;
	// 8329F448: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F44C: 390B02C8  addi r8, r11, 0x2c8
	ctx.r[8].s64 = ctx.r[11].s64 + 712;
	// 8329F450: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329F454: 388A0328  addi r4, r10, 0x328
	ctx.r[4].s64 = ctx.r[10].s64 + 808;
	// 8329F458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F45C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F468: 386A9C64  addi r3, r10, -0x639c
	ctx.r[3].s64 = ctx.r[10].s64 + -25500;
	// 8329F46C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F484: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329F488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F48C: 4BAB67B5  bl 0x82d55c40
	ctx.lr = 0x8329F490;
	sub_82D55C40(ctx, base);
	// 8329F490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F4A0 size=92
    let mut pc: u32 = 0x8329F4A0;
    'dispatch: loop {
        match pc {
            0x8329F4A0 => {
    //   block [0x8329F4A0..0x8329F4FC)
	// 8329F4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F4A8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F4AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F4B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F4B4: 4BB23BCD  bl 0x82dc3080
	ctx.lr = 0x8329F4B8;
	sub_82DC3080(ctx, base);
	// 8329F4B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F4BC: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F4C0: 394B04D0  addi r10, r11, 0x4d0
	ctx.r[10].s64 = ctx.r[11].s64 + 1232;
	// 8329F4C4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F4C8: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F4CC: 396B9C94  addi r11, r11, -0x636c
	ctx.r[11].s64 = ctx.r[11].s64 + -25452;
	// 8329F4D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F4D4: 39481E90  addi r10, r8, 0x1e90
	ctx.r[10].s64 = ctx.r[8].s64 + 7824;
	// 8329F4D8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F4DC: 39491EA8  addi r10, r9, 0x1ea8
	ctx.r[10].s64 = ctx.r[9].s64 + 7848;
	// 8329F4E0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F4E4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F4E8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F4EC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8329F4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F500 size=112
    let mut pc: u32 = 0x8329F500;
    'dispatch: loop {
        match pc {
            0x8329F500 => {
    //   block [0x8329F500..0x8329F570)
	// 8329F500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F50C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329F510: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F514: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329F518: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F51C: 390B0398  addi r8, r11, 0x398
	ctx.r[8].s64 = ctx.r[11].s64 + 920;
	// 8329F520: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329F524: 388A04D0  addi r4, r10, 0x4d0
	ctx.r[4].s64 = ctx.r[10].s64 + 1232;
	// 8329F528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F52C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F538: 386A9CA4  addi r3, r10, -0x635c
	ctx.r[3].s64 = ctx.r[10].s64 + -25436;
	// 8329F53C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F554: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329F558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F55C: 4BAB66E5  bl 0x82d55c40
	ctx.lr = 0x8329F560;
	sub_82D55C40(ctx, base);
	// 8329F560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F570 size=92
    let mut pc: u32 = 0x8329F570;
    'dispatch: loop {
        match pc {
            0x8329F570 => {
    //   block [0x8329F570..0x8329F5CC)
	// 8329F570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F578: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F57C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F584: 4BB236E5  bl 0x82dc2c68
	ctx.lr = 0x8329F588;
	sub_82DC2C68(ctx, base);
	// 8329F588: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F58C: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F590: 394B0500  addi r10, r11, 0x500
	ctx.r[10].s64 = ctx.r[11].s64 + 1280;
	// 8329F594: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F598: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F59C: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 8329F5A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F5A4: 39481EF0  addi r10, r8, 0x1ef0
	ctx.r[10].s64 = ctx.r[8].s64 + 7920;
	// 8329F5A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F5AC: 39491F08  addi r10, r9, 0x1f08
	ctx.r[10].s64 = ctx.r[9].s64 + 7944;
	// 8329F5B0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F5B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F5B8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F5BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8329F5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F5D0 size=112
    let mut pc: u32 = 0x8329F5D0;
    'dispatch: loop {
        match pc {
            0x8329F5D0 => {
    //   block [0x8329F5D0..0x8329F640)
	// 8329F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F5DC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329F5E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F5E4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329F5E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F5EC: 390B0428  addi r8, r11, 0x428
	ctx.r[8].s64 = ctx.r[11].s64 + 1064;
	// 8329F5F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329F5F4: 388A0500  addi r4, r10, 0x500
	ctx.r[4].s64 = ctx.r[10].s64 + 1280;
	// 8329F5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F5FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F608: 386A9CE4  addi r3, r10, -0x631c
	ctx.r[3].s64 = ctx.r[10].s64 + -25372;
	// 8329F60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F624: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 8329F628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F62C: 4BAB6615  bl 0x82d55c40
	ctx.lr = 0x8329F630;
	sub_82D55C40(ctx, base);
	// 8329F630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F640 size=92
    let mut pc: u32 = 0x8329F640;
    'dispatch: loop {
        match pc {
            0x8329F640 => {
    //   block [0x8329F640..0x8329F69C)
	// 8329F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F648: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F64C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F654: 4BB2362D  bl 0x82dc2c80
	ctx.lr = 0x8329F658;
	sub_82DC2C80(ctx, base);
	// 8329F658: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F65C: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F660: 394B0530  addi r10, r11, 0x530
	ctx.r[10].s64 = ctx.r[11].s64 + 1328;
	// 8329F664: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F668: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F66C: 396B9D14  addi r11, r11, -0x62ec
	ctx.r[11].s64 = ctx.r[11].s64 + -25324;
	// 8329F670: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F674: 39481F50  addi r10, r8, 0x1f50
	ctx.r[10].s64 = ctx.r[8].s64 + 8016;
	// 8329F678: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F67C: 39491F68  addi r10, r9, 0x1f68
	ctx.r[10].s64 = ctx.r[9].s64 + 8040;
	// 8329F680: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F684: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F688: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F68C: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 8329F690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F6A0 size=112
    let mut pc: u32 = 0x8329F6A0;
    'dispatch: loop {
        match pc {
            0x8329F6A0 => {
    //   block [0x8329F6A0..0x8329F710)
	// 8329F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F6AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F6B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F6B4: 38AAA114  addi r5, r10, -0x5eec
	ctx.r[5].s64 = ctx.r[10].s64 + -24300;
	// 8329F6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F6BC: 390B0488  addi r8, r11, 0x488
	ctx.r[8].s64 = ctx.r[11].s64 + 1160;
	// 8329F6C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329F6C4: 388A0530  addi r4, r10, 0x530
	ctx.r[4].s64 = ctx.r[10].s64 + 1328;
	// 8329F6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F6CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F6D8: 386A9D24  addi r3, r10, -0x62dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25308;
	// 8329F6DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F6F4: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 8329F6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F6FC: 4BAB6545  bl 0x82d55c40
	ctx.lr = 0x8329F700;
	sub_82D55C40(ctx, base);
	// 8329F700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F710 size=100
    let mut pc: u32 = 0x8329F710;
    'dispatch: loop {
        match pc {
            0x8329F710 => {
    //   block [0x8329F710..0x8329F774)
	// 8329F710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F71C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F724: 392A065C  addi r9, r10, 0x65c
	ctx.r[9].s64 = ctx.r[10].s64 + 1628;
	// 8329F728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F730: 388A0698  addi r4, r10, 0x698
	ctx.r[4].s64 = ctx.r[10].s64 + 1688;
	// 8329F734: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F744: 386A9D54  addi r3, r10, -0x62ac
	ctx.r[3].s64 = ctx.r[10].s64 + -25260;
	// 8329F748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F74C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8329F750: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329F754: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8329F758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329F75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F760: 4BAB64E1  bl 0x82d55c40
	ctx.lr = 0x8329F764;
	sub_82D55C40(ctx, base);
	// 8329F764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F778 size=92
    let mut pc: u32 = 0x8329F778;
    'dispatch: loop {
        match pc {
            0x8329F778 => {
    //   block [0x8329F778..0x8329F7D4)
	// 8329F778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F780: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F784: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F78C: 4BB2554D  bl 0x82dc4cd8
	ctx.lr = 0x8329F790;
	sub_82DC4CD8(ctx, base);
	// 8329F790: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F794: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F798: 394B0760  addi r10, r11, 0x760
	ctx.r[10].s64 = ctx.r[11].s64 + 1888;
	// 8329F79C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F7A0: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F7A4: 396B9D84  addi r11, r11, -0x627c
	ctx.r[11].s64 = ctx.r[11].s64 + -25212;
	// 8329F7A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F7AC: 39482050  addi r10, r8, 0x2050
	ctx.r[10].s64 = ctx.r[8].s64 + 8272;
	// 8329F7B0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F7B4: 39492068  addi r10, r9, 0x2068
	ctx.r[10].s64 = ctx.r[9].s64 + 8296;
	// 8329F7B8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F7BC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F7C0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F7C4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8329F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F7D8 size=112
    let mut pc: u32 = 0x8329F7D8;
    'dispatch: loop {
        match pc {
            0x8329F7D8 => {
    //   block [0x8329F7D8..0x8329F848)
	// 8329F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F7E4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329F7E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F7EC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329F7F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F7F4: 390B06B8  addi r8, r11, 0x6b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1720;
	// 8329F7F8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329F7FC: 388A0760  addi r4, r10, 0x760
	ctx.r[4].s64 = ctx.r[10].s64 + 1888;
	// 8329F800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F804: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F810: 386A9D94  addi r3, r10, -0x626c
	ctx.r[3].s64 = ctx.r[10].s64 + -25196;
	// 8329F814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F82C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329F830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F834: 4BAB640D  bl 0x82d55c40
	ctx.lr = 0x8329F838;
	sub_82D55C40(ctx, base);
	// 8329F838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F848 size=92
    let mut pc: u32 = 0x8329F848;
    'dispatch: loop {
        match pc {
            0x8329F848 => {
    //   block [0x8329F848..0x8329F8A4)
	// 8329F848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F850: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F854: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F85C: 4BB254AD  bl 0x82dc4d08
	ctx.lr = 0x8329F860;
	sub_82DC4D08(ctx, base);
	// 8329F860: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F864: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F868: 394B0784  addi r10, r11, 0x784
	ctx.r[10].s64 = ctx.r[11].s64 + 1924;
	// 8329F86C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F870: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F874: 396B9DC4  addi r11, r11, -0x623c
	ctx.r[11].s64 = ctx.r[11].s64 + -25148;
	// 8329F878: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F87C: 394820B0  addi r10, r8, 0x20b0
	ctx.r[10].s64 = ctx.r[8].s64 + 8368;
	// 8329F880: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F884: 394920C8  addi r10, r9, 0x20c8
	ctx.r[10].s64 = ctx.r[9].s64 + 8392;
	// 8329F888: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F88C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F890: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F894: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8329F898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F8A8 size=112
    let mut pc: u32 = 0x8329F8A8;
    'dispatch: loop {
        match pc {
            0x8329F8A8 => {
    //   block [0x8329F8A8..0x8329F918)
	// 8329F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F8B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F8B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F8BC: 38AAA314  addi r5, r10, -0x5cec
	ctx.r[5].s64 = ctx.r[10].s64 + -23788;
	// 8329F8C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F8C4: 390B0748  addi r8, r11, 0x748
	ctx.r[8].s64 = ctx.r[11].s64 + 1864;
	// 8329F8C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F8CC: 388A0784  addi r4, r10, 0x784
	ctx.r[4].s64 = ctx.r[10].s64 + 1924;
	// 8329F8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F8D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F8D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F8E0: 386A9DD4  addi r3, r10, -0x622c
	ctx.r[3].s64 = ctx.r[10].s64 + -25132;
	// 8329F8E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F8FC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329F900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F904: 4BAB633D  bl 0x82d55c40
	ctx.lr = 0x8329F908;
	sub_82D55C40(ctx, base);
	// 8329F908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F918 size=112
    let mut pc: u32 = 0x8329F918;
    'dispatch: loop {
        match pc {
            0x8329F918 => {
    //   block [0x8329F918..0x8329F988)
	// 8329F918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F924: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F928: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F92C: 38AAA854  addi r5, r10, -0x57ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22444;
	// 8329F930: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F934: 390B07C0  addi r8, r11, 0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1984;
	// 8329F938: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329F93C: 388A0860  addi r4, r10, 0x860
	ctx.r[4].s64 = ctx.r[10].s64 + 2144;
	// 8329F940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F944: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F950: 386A9E04  addi r3, r10, -0x61fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25084;
	// 8329F954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F96C: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329F970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F974: 4BAB62CD  bl 0x82d55c40
	ctx.lr = 0x8329F978;
	sub_82D55C40(ctx, base);
	// 8329F978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F988 size=112
    let mut pc: u32 = 0x8329F988;
    'dispatch: loop {
        match pc {
            0x8329F988 => {
    //   block [0x8329F988..0x8329F9F8)
	// 8329F988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F994: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F998: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F99C: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 8329F9A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F9A4: 390B0890  addi r8, r11, 0x890
	ctx.r[8].s64 = ctx.r[11].s64 + 2192;
	// 8329F9A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329F9AC: 388A0900  addi r4, r10, 0x900
	ctx.r[4].s64 = ctx.r[10].s64 + 2304;
	// 8329F9B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F9B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F9B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F9C0: 386A9E34  addi r3, r10, -0x61cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25036;
	// 8329F9C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F9C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F9DC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329F9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F9E4: 4BAB625D  bl 0x82d55c40
	ctx.lr = 0x8329F9E8;
	sub_82D55C40(ctx, base);
	// 8329F9E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F9F8 size=112
    let mut pc: u32 = 0x8329F9F8;
    'dispatch: loop {
        match pc {
            0x8329F9F8 => {
    //   block [0x8329F9F8..0x8329FA68)
	// 8329F9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FA04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FA0C: 38AAA6A4  addi r5, r10, -0x595c
	ctx.r[5].s64 = ctx.r[10].s64 + -22876;
	// 8329FA10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FA14: 390B092C  addi r8, r11, 0x92c
	ctx.r[8].s64 = ctx.r[11].s64 + 2348;
	// 8329FA18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329FA1C: 388A095C  addi r4, r10, 0x95c
	ctx.r[4].s64 = ctx.r[10].s64 + 2396;
	// 8329FA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FA24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FA30: 386A9E64  addi r3, r10, -0x619c
	ctx.r[3].s64 = ctx.r[10].s64 + -24988;
	// 8329FA34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FA4C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329FA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FA54: 4BAB61ED  bl 0x82d55c40
	ctx.lr = 0x8329FA58;
	sub_82D55C40(ctx, base);
	// 8329FA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FA68 size=112
    let mut pc: u32 = 0x8329FA68;
    'dispatch: loop {
        match pc {
            0x8329FA68 => {
    //   block [0x8329FA68..0x8329FAD8)
	// 8329FA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FA74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FA7C: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 8329FA80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FA84: 390B0980  addi r8, r11, 0x980
	ctx.r[8].s64 = ctx.r[11].s64 + 2432;
	// 8329FA88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329FA8C: 388A09FC  addi r4, r10, 0x9fc
	ctx.r[4].s64 = ctx.r[10].s64 + 2556;
	// 8329FA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FA94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FAA0: 386A9E94  addi r3, r10, -0x616c
	ctx.r[3].s64 = ctx.r[10].s64 + -24940;
	// 8329FAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FABC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329FAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FAC4: 4BAB617D  bl 0x82d55c40
	ctx.lr = 0x8329FAC8;
	sub_82D55C40(ctx, base);
	// 8329FAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329FAD8 size=24
    let mut pc: u32 = 0x8329FAD8;
    'dispatch: loop {
        match pc {
            0x8329FAD8 => {
    //   block [0x8329FAD8..0x8329FAF0)
	// 8329FAD8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FADC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FAE0: 394AA720  addi r10, r10, -0x58e0
	ctx.r[10].s64 = ctx.r[10].s64 + -22752;
	// 8329FAE4: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 8329FAE8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329FAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FAF0 size=116
    let mut pc: u32 = 0x8329FAF0;
    'dispatch: loop {
        match pc {
            0x8329FAF0 => {
    //   block [0x8329FAF0..0x8329FB64)
	// 8329FAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FAFC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FB00: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329FB04: 390AA720  addi r8, r10, -0x58e0
	ctx.r[8].s64 = ctx.r[10].s64 + -22752;
	// 8329FB08: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FB0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FB10: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FB14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FB18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FB24: 388A0A40  addi r4, r10, 0xa40
	ctx.r[4].s64 = ctx.r[10].s64 + 2624;
	// 8329FB28: 396B0A20  addi r11, r11, 0xa20
	ctx.r[11].s64 = ctx.r[11].s64 + 2592;
	// 8329FB2C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FB30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FB34: 386A9EC4  addi r3, r10, -0x613c
	ctx.r[3].s64 = ctx.r[10].s64 + -24892;
	// 8329FB38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329FB3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FB40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329FB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FB4C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8329FB50: 4BAB60F1  bl 0x82d55c40
	ctx.lr = 0x8329FB54;
	sub_82D55C40(ctx, base);
	// 8329FB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FB68 size=108
    let mut pc: u32 = 0x8329FB68;
    'dispatch: loop {
        match pc {
            0x8329FB68 => {
    //   block [0x8329FB68..0x8329FBD4)
	// 8329FB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FB74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FB78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FB7C: 38EB0A90  addi r7, r11, 0xa90
	ctx.r[7].s64 = ctx.r[11].s64 + 2704;
	// 8329FB80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329FB84: 388A0AF0  addi r4, r10, 0xaf0
	ctx.r[4].s64 = ctx.r[10].s64 + 2800;
	// 8329FB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FB8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FB90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FB98: 386A9EF4  addi r3, r10, -0x610c
	ctx.r[3].s64 = ctx.r[10].s64 + -24844;
	// 8329FB9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FBB4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329FBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FBBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FBC0: 4BAB6081  bl 0x82d55c40
	ctx.lr = 0x8329FBC4;
	sub_82D55C40(ctx, base);
	// 8329FBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FBD8 size=108
    let mut pc: u32 = 0x8329FBD8;
    'dispatch: loop {
        match pc {
            0x8329FBD8 => {
    //   block [0x8329FBD8..0x8329FC44)
	// 8329FBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FBE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FBE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FBEC: 38EB0C50  addi r7, r11, 0xc50
	ctx.r[7].s64 = ctx.r[11].s64 + 3152;
	// 8329FBF0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8329FBF4: 388A0DA0  addi r4, r10, 0xda0
	ctx.r[4].s64 = ctx.r[10].s64 + 3488;
	// 8329FBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FBFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FC00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FC08: 386A9F24  addi r3, r10, -0x60dc
	ctx.r[3].s64 = ctx.r[10].s64 + -24796;
	// 8329FC0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FC24: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329FC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FC2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FC30: 4BAB6011  bl 0x82d55c40
	ctx.lr = 0x8329FC34;
	sub_82D55C40(ctx, base);
	// 8329FC34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FC48 size=116
    let mut pc: u32 = 0x8329FC48;
    'dispatch: loop {
        match pc {
            0x8329FC48 => {
    //   block [0x8329FC48..0x8329FCBC)
	// 8329FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FC54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FC58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FC5C: 390B0D10  addi r8, r11, 0xd10
	ctx.r[8].s64 = ctx.r[11].s64 + 3344;
	// 8329FC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FC64: 392A0C38  addi r9, r10, 0xc38
	ctx.r[9].s64 = ctx.r[10].s64 + 3128;
	// 8329FC68: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FC6C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329FC70: 38AA9EF4  addi r5, r10, -0x610c
	ctx.r[5].s64 = ctx.r[10].s64 + -24844;
	// 8329FC74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FC7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FC8C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329FC90: 388A0DC0  addi r4, r10, 0xdc0
	ctx.r[4].s64 = ctx.r[10].s64 + 3520;
	// 8329FC94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FC98: 386B9F54  addi r3, r11, -0x60ac
	ctx.r[3].s64 = ctx.r[11].s64 + -24748;
	// 8329FC9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329FCA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FCA4: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 8329FCA8: 4BAB5F99  bl 0x82d55c40
	ctx.lr = 0x8329FCAC;
	sub_82D55C40(ctx, base);
	// 8329FCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FCC0 size=112
    let mut pc: u32 = 0x8329FCC0;
    'dispatch: loop {
        match pc {
            0x8329FCC0 => {
    //   block [0x8329FCC0..0x8329FD30)
	// 8329FCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FCC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FCCC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FCD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FCD4: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FCD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FCDC: 390B0DDC  addi r8, r11, 0xddc
	ctx.r[8].s64 = ctx.r[11].s64 + 3548;
	// 8329FCE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329FCE4: 388A0DF4  addi r4, r10, 0xdf4
	ctx.r[4].s64 = ctx.r[10].s64 + 3572;
	// 8329FCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FCEC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FCF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FCF8: 386A9F84  addi r3, r10, -0x607c
	ctx.r[3].s64 = ctx.r[10].s64 + -24700;
	// 8329FCFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FD14: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329FD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FD1C: 4BAB5F25  bl 0x82d55c40
	ctx.lr = 0x8329FD20;
	sub_82D55C40(ctx, base);
	// 8329FD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FD30 size=112
    let mut pc: u32 = 0x8329FD30;
    'dispatch: loop {
        match pc {
            0x8329FD30 => {
    //   block [0x8329FD30..0x8329FDA0)
	// 8329FD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FD3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FD40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FD44: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 8329FD48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FD4C: 390B0E14  addi r8, r11, 0xe14
	ctx.r[8].s64 = ctx.r[11].s64 + 3604;
	// 8329FD50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329FD54: 388A0E6C  addi r4, r10, 0xe6c
	ctx.r[4].s64 = ctx.r[10].s64 + 3692;
	// 8329FD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FD5C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FD68: 386A9FB4  addi r3, r10, -0x604c
	ctx.r[3].s64 = ctx.r[10].s64 + -24652;
	// 8329FD6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FD84: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329FD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FD8C: 4BAB5EB5  bl 0x82d55c40
	ctx.lr = 0x8329FD90;
	sub_82D55C40(ctx, base);
	// 8329FD90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FD94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FD98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FDA0 size=108
    let mut pc: u32 = 0x8329FDA0;
    'dispatch: loop {
        match pc {
            0x8329FDA0 => {
    //   block [0x8329FDA0..0x8329FE0C)
	// 8329FDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FDAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FDB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FDB4: 38EB0EC0  addi r7, r11, 0xec0
	ctx.r[7].s64 = ctx.r[11].s64 + 3776;
	// 8329FDB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329FDBC: 388A0F80  addi r4, r10, 0xf80
	ctx.r[4].s64 = ctx.r[10].s64 + 3968;
	// 8329FDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FDC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FDC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FDD0: 386A9FE4  addi r3, r10, -0x601c
	ctx.r[3].s64 = ctx.r[10].s64 + -24604;
	// 8329FDD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FDEC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329FDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FDF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FDF8: 4BAB5E49  bl 0x82d55c40
	ctx.lr = 0x8329FDFC;
	sub_82D55C40(ctx, base);
	// 8329FDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FE10 size=112
    let mut pc: u32 = 0x8329FE10;
    'dispatch: loop {
        match pc {
            0x8329FE10 => {
    //   block [0x8329FE10..0x8329FE80)
	// 8329FE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FE1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FE20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FE24: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FE28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FE2C: 390B0F08  addi r8, r11, 0xf08
	ctx.r[8].s64 = ctx.r[11].s64 + 3848;
	// 8329FE30: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329FE34: 388A0FA4  addi r4, r10, 0xfa4
	ctx.r[4].s64 = ctx.r[10].s64 + 4004;
	// 8329FE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FE3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FE40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FE48: 386AA014  addi r3, r10, -0x5fec
	ctx.r[3].s64 = ctx.r[10].s64 + -24556;
	// 8329FE4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FE64: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8329FE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FE6C: 4BAB5DD5  bl 0x82d55c40
	ctx.lr = 0x8329FE70;
	sub_82D55C40(ctx, base);
	// 8329FE70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FE80 size=116
    let mut pc: u32 = 0x8329FE80;
    'dispatch: loop {
        match pc {
            0x8329FE80 => {
    //   block [0x8329FE80..0x8329FEF4)
	// 8329FE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FE8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FE90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FE94: 390B1018  addi r8, r11, 0x1018
	ctx.r[8].s64 = ctx.r[11].s64 + 4120;
	// 8329FE98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FE9C: 392A1004  addi r9, r10, 0x1004
	ctx.r[9].s64 = ctx.r[10].s64 + 4100;
	// 8329FEA0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FEA4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8329FEA8: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FEAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FEB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FEC4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329FEC8: 388A1048  addi r4, r10, 0x1048
	ctx.r[4].s64 = ctx.r[10].s64 + 4168;
	// 8329FECC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FED0: 386BA044  addi r3, r11, -0x5fbc
	ctx.r[3].s64 = ctx.r[11].s64 + -24508;
	// 8329FED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329FED8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FEDC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329FEE0: 4BAB5D61  bl 0x82d55c40
	ctx.lr = 0x8329FEE4;
	sub_82D55C40(ctx, base);
	// 8329FEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329FEF8 size=36
    let mut pc: u32 = 0x8329FEF8;
    'dispatch: loop {
        match pc {
            0x8329FEF8 => {
    //   block [0x8329FEF8..0x8329FF1C)
	// 8329FEF8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FEFC: 814BA890  lwz r10, -0x5770(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22384 as u32) ) } as u64;
	// 8329FF00: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FF04: 396BA8C8  addi r11, r11, -0x5738
	ctx.r[11].s64 = ctx.r[11].s64 + -22328;
	// 8329FF08: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329FF0C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FF10: 814AA888  lwz r10, -0x5778(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-22392 as u32) ) } as u64;
	// 8329FF14: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8329FF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FF20 size=108
    let mut pc: u32 = 0x8329FF20;
    'dispatch: loop {
        match pc {
            0x8329FF20 => {
    //   block [0x8329FF20..0x8329FF8C)
	// 8329FF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FF2C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FF30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FF34: 38EBA8C8  addi r7, r11, -0x5738
	ctx.r[7].s64 = ctx.r[11].s64 + -22328;
	// 8329FF38: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8329FF3C: 388A1380  addi r4, r10, 0x1380
	ctx.r[4].s64 = ctx.r[10].s64 + 4992;
	// 8329FF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FF44: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FF48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FF50: 386AA074  addi r3, r10, -0x5f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24460;
	// 8329FF54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FF6C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8329FF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FF74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FF78: 4BAB5CC9  bl 0x82d55c40
	ctx.lr = 0x8329FF7C;
	sub_82D55C40(ctx, base);
	// 8329FF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FF80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FF84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329FF90 size=24
    let mut pc: u32 = 0x8329FF90;
    'dispatch: loop {
        match pc {
            0x8329FF90 => {
    //   block [0x8329FF90..0x8329FFA8)
	// 8329FF90: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FF94: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FF98: 394AA9A0  addi r10, r10, -0x5660
	ctx.r[10].s64 = ctx.r[10].s64 + -22112;
	// 8329FF9C: 816BA888  lwz r11, -0x5778(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22392 as u32) ) } as u64;
	// 8329FFA0: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 8329FFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FFA8 size=116
    let mut pc: u32 = 0x8329FFA8;
    'dispatch: loop {
        match pc {
            0x8329FFA8 => {
    //   block [0x8329FFA8..0x832A001C)
	// 8329FFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FFB4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FFB8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8329FFBC: 390AA9A0  addi r8, r10, -0x5660
	ctx.r[8].s64 = ctx.r[10].s64 + -22112;
	// 8329FFC0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FFC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FFC8: 38AAA074  addi r5, r10, -0x5f8c
	ctx.r[5].s64 = ctx.r[10].s64 + -24460;
	// 8329FFCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FFD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FFD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FFDC: 388A1424  addi r4, r10, 0x1424
	ctx.r[4].s64 = ctx.r[10].s64 + 5156;
	// 8329FFE0: 396B12AC  addi r11, r11, 0x12ac
	ctx.r[11].s64 = ctx.r[11].s64 + 4780;
	// 8329FFE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FFE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FFEC: 386AA0A4  addi r3, r10, -0x5f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24412;
	// 8329FFF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329FFF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FFF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329FFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0004: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 832A0008: 4BAB5C39  bl 0x82d55c40
	ctx.lr = 0x832A000C;
	sub_82D55C40(ctx, base);
	// 832A000C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0020 size=112
    let mut pc: u32 = 0x832A0020;
    'dispatch: loop {
        match pc {
            0x832A0020 => {
    //   block [0x832A0020..0x832A0090)
	// 832A0020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A002C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0030: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0034: 38AAA074  addi r5, r10, -0x5f8c
	ctx.r[5].s64 = ctx.r[10].s64 + -24460;
	// 832A0038: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A003C: 390B12D8  addi r8, r11, 0x12d8
	ctx.r[8].s64 = ctx.r[11].s64 + 4824;
	// 832A0040: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A0044: 388A1474  addi r4, r10, 0x1474
	ctx.r[4].s64 = ctx.r[10].s64 + 5236;
	// 832A0048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A004C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0058: 386AA0D4  addi r3, r10, -0x5f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24364;
	// 832A005C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A006C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0074: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 832A0078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A007C: 4BAB5BC5  bl 0x82d55c40
	ctx.lr = 0x832A0080;
	sub_82D55C40(ctx, base);
	// 832A0080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A008C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0090 size=92
    let mut pc: u32 = 0x832A0090;
    'dispatch: loop {
        match pc {
            0x832A0090 => {
    //   block [0x832A0090..0x832A00EC)
	// 832A0090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0098: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A009C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A00A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A00A4: 4BB14E35  bl 0x82db4ed8
	ctx.lr = 0x832A00A8;
	sub_82DB4ED8(ctx, base);
	// 832A00A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A00AC: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A00B0: 394B1498  addi r10, r11, 0x1498
	ctx.r[10].s64 = ctx.r[11].s64 + 5272;
	// 832A00B4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A00B8: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 832A00BC: 396BA104  addi r11, r11, -0x5efc
	ctx.r[11].s64 = ctx.r[11].s64 + -24316;
	// 832A00C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A00C4: 39482A28  addi r10, r8, 0x2a28
	ctx.r[10].s64 = ctx.r[8].s64 + 10792;
	// 832A00C8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A00CC: 39492A40  addi r10, r9, 0x2a40
	ctx.r[10].s64 = ctx.r[9].s64 + 10816;
	// 832A00D0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A00D4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A00D8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A00DC: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 832A00E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A00E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A00E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A00F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A00F0 size=24
    let mut pc: u32 = 0x832A00F0;
    'dispatch: loop {
        match pc {
            0x832A00F0 => {
    //   block [0x832A00F0..0x832A0108)
	// 832A00F0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A00F4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A00F8: 394AAA90  addi r10, r10, -0x5570
	ctx.r[10].s64 = ctx.r[10].s64 + -21872;
	// 832A00FC: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 832A0100: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832A0104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0108 size=116
    let mut pc: u32 = 0x832A0108;
    'dispatch: loop {
        match pc {
            0x832A0108 => {
    //   block [0x832A0108..0x832A017C)
	// 832A0108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0114: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0118: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A011C: 392B1270  addi r9, r11, 0x1270
	ctx.r[9].s64 = ctx.r[11].s64 + 4720;
	// 832A0120: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 832A0124: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0128: 38E900E0  addi r7, r9, 0xe0
	ctx.r[7].s64 = ctx.r[9].s64 + 224;
	// 832A012C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 832A0130: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A0134: 388A1498  addi r4, r10, 0x1498
	ctx.r[4].s64 = ctx.r[10].s64 + 5272;
	// 832A0138: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A013C: 396BAA90  addi r11, r11, -0x5570
	ctx.r[11].s64 = ctx.r[11].s64 + -21872;
	// 832A0140: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 832A0144: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0148: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832A014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0150: 386AA114  addi r3, r10, -0x5eec
	ctx.r[3].s64 = ctx.r[10].s64 + -24300;
	// 832A0154: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A0158: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 832A015C: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 832A0160: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 832A0164: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A0168: 4BAB5AD9  bl 0x82d55c40
	ctx.lr = 0x832A016C;
	sub_82D55C40(ctx, base);
	// 832A016C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0180 size=112
    let mut pc: u32 = 0x832A0180;
    'dispatch: loop {
        match pc {
            0x832A0180 => {
    //   block [0x832A0180..0x832A01F0)
	// 832A0180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A018C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0190: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0194: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A0198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A019C: 390B14F0  addi r8, r11, 0x14f0
	ctx.r[8].s64 = ctx.r[11].s64 + 5360;
	// 832A01A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A01A4: 388A1550  addi r4, r10, 0x1550
	ctx.r[4].s64 = ctx.r[10].s64 + 5456;
	// 832A01A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A01AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A01B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A01B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A01B8: 386AA144  addi r3, r10, -0x5ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -24252;
	// 832A01BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A01C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A01C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A01C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A01CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A01D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A01D4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A01D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A01DC: 4BAB5A65  bl 0x82d55c40
	ctx.lr = 0x832A01E0;
	sub_82D55C40(ctx, base);
	// 832A01E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A01E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A01E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A01EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A01F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A01F0 size=112
    let mut pc: u32 = 0x832A01F0;
    'dispatch: loop {
        match pc {
            0x832A01F0 => {
    //   block [0x832A01F0..0x832A0260)
	// 832A01F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A01F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A01F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A01FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0200: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0204: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 832A0208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A020C: 390B15A8  addi r8, r11, 0x15a8
	ctx.r[8].s64 = ctx.r[11].s64 + 5544;
	// 832A0210: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A0214: 388A1620  addi r4, r10, 0x1620
	ctx.r[4].s64 = ctx.r[10].s64 + 5664;
	// 832A0218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A021C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0228: 386AA174  addi r3, r10, -0x5e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24204;
	// 832A022C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A023C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A0240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0244: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A0248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A024C: 4BAB59F5  bl 0x82d55c40
	ctx.lr = 0x832A0250;
	sub_82D55C40(ctx, base);
	// 832A0250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A025C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0260 size=112
    let mut pc: u32 = 0x832A0260;
    'dispatch: loop {
        match pc {
            0x832A0260 => {
    //   block [0x832A0260..0x832A02D0)
	// 832A0260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A026C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0270: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0274: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 832A0278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A027C: 390B1658  addi r8, r11, 0x1658
	ctx.r[8].s64 = ctx.r[11].s64 + 5720;
	// 832A0280: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A0284: 388A16EC  addi r4, r10, 0x16ec
	ctx.r[4].s64 = ctx.r[10].s64 + 5868;
	// 832A0288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A028C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0298: 386AA1A4  addi r3, r10, -0x5e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24156;
	// 832A029C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A02A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A02A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A02A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A02AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A02B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A02B4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A02B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A02BC: 4BAB5985  bl 0x82d55c40
	ctx.lr = 0x832A02C0;
	sub_82D55C40(ctx, base);
	// 832A02C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A02C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A02C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A02CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A02D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A02D0 size=108
    let mut pc: u32 = 0x832A02D0;
    'dispatch: loop {
        match pc {
            0x832A02D0 => {
    //   block [0x832A02D0..0x832A033C)
	// 832A02D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A02D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A02D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A02DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A02E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A02E4: 38EB1740  addi r7, r11, 0x1740
	ctx.r[7].s64 = ctx.r[11].s64 + 5952;
	// 832A02E8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A02EC: 388A1800  addi r4, r10, 0x1800
	ctx.r[4].s64 = ctx.r[10].s64 + 6144;
	// 832A02F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A02F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A02F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A02FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0300: 386AA1D4  addi r3, r10, -0x5e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24108;
	// 832A0304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A0308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A030C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A031C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A0320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A0328: 4BAB5919  bl 0x82d55c40
	ctx.lr = 0x832A032C;
	sub_82D55C40(ctx, base);
	// 832A032C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0340 size=92
    let mut pc: u32 = 0x832A0340;
    'dispatch: loop {
        match pc {
            0x832A0340 => {
    //   block [0x832A0340..0x832A039C)
	// 832A0340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0348: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A034C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A0350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A0354: 4BB2715D  bl 0x82dc74b0
	ctx.lr = 0x832A0358;
	sub_82DC74B0(ctx, base);
	// 832A0358: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A035C: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A0360: 394B1818  addi r10, r11, 0x1818
	ctx.r[10].s64 = ctx.r[11].s64 + 6168;
	// 832A0364: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A0368: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 832A036C: 396BA204  addi r11, r11, -0x5dfc
	ctx.r[11].s64 = ctx.r[11].s64 + -24060;
	// 832A0370: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A0374: 39482D98  addi r10, r8, 0x2d98
	ctx.r[10].s64 = ctx.r[8].s64 + 11672;
	// 832A0378: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A037C: 39492D80  addi r10, r9, 0x2d80
	ctx.r[10].s64 = ctx.r[9].s64 + 11648;
	// 832A0380: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A0384: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A0388: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A038C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 832A0390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A03A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A03A0 size=112
    let mut pc: u32 = 0x832A03A0;
    'dispatch: loop {
        match pc {
            0x832A03A0 => {
    //   block [0x832A03A0..0x832A0410)
	// 832A03A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A03A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A03A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A03AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A03B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A03B4: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 832A03B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A03BC: 390B17A0  addi r8, r11, 0x17a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6048;
	// 832A03C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A03C4: 388A1818  addi r4, r10, 0x1818
	ctx.r[4].s64 = ctx.r[10].s64 + 6168;
	// 832A03C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A03CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A03D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A03D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A03D8: 386AA214  addi r3, r10, -0x5dec
	ctx.r[3].s64 = ctx.r[10].s64 + -24044;
	// 832A03DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A03E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A03E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A03E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A03EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A03F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A03F4: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 832A03F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A03FC: 4BAB5845  bl 0x82d55c40
	ctx.lr = 0x832A0400;
	sub_82D55C40(ctx, base);
	// 832A0400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A040C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0410 size=112
    let mut pc: u32 = 0x832A0410;
    'dispatch: loop {
        match pc {
            0x832A0410 => {
    //   block [0x832A0410..0x832A0480)
	// 832A0410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A041C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0420: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0424: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 832A0428: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A042C: 390B1838  addi r8, r11, 0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + 6200;
	// 832A0430: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A0434: 388A1880  addi r4, r10, 0x1880
	ctx.r[4].s64 = ctx.r[10].s64 + 6272;
	// 832A0438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A043C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0448: 386AA244  addi r3, r10, -0x5dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -23996;
	// 832A044C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A045C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0464: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A0468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A046C: 4BAB57D5  bl 0x82d55c40
	ctx.lr = 0x832A0470;
	sub_82D55C40(ctx, base);
	// 832A0470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A047C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0480 size=108
    let mut pc: u32 = 0x832A0480;
    'dispatch: loop {
        match pc {
            0x832A0480 => {
    //   block [0x832A0480..0x832A04EC)
	// 832A0480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A048C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0490: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0494: 38EB1898  addi r7, r11, 0x1898
	ctx.r[7].s64 = ctx.r[11].s64 + 6296;
	// 832A0498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A049C: 388A18B0  addi r4, r10, 0x18b0
	ctx.r[4].s64 = ctx.r[10].s64 + 6320;
	// 832A04A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A04A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A04A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A04AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A04B0: 386AA274  addi r3, r10, -0x5d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23948;
	// 832A04B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A04B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A04BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A04C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A04C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A04C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A04CC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A04D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A04D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A04D8: 4BAB5769  bl 0x82d55c40
	ctx.lr = 0x832A04DC;
	sub_82D55C40(ctx, base);
	// 832A04DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A04E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A04E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A04E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A04F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A04F0 size=112
    let mut pc: u32 = 0x832A04F0;
    'dispatch: loop {
        match pc {
            0x832A04F0 => {
    //   block [0x832A04F0..0x832A0560)
	// 832A04F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A04F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A04F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A04FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0500: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0504: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 832A0508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A050C: 390B18D0  addi r8, r11, 0x18d0
	ctx.r[8].s64 = ctx.r[11].s64 + 6352;
	// 832A0510: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A0514: 388A195C  addi r4, r10, 0x195c
	ctx.r[4].s64 = ctx.r[10].s64 + 6492;
	// 832A0518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A051C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0528: 386AA2A4  addi r3, r10, -0x5d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -23900;
	// 832A052C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A053C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0544: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A0548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A054C: 4BAB56F5  bl 0x82d55c40
	ctx.lr = 0x832A0550;
	sub_82D55C40(ctx, base);
	// 832A0550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A055C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A0560 size=28
    let mut pc: u32 = 0x832A0560;
    'dispatch: loop {
        match pc {
            0x832A0560 => {
    //   block [0x832A0560..0x832A057C)
	// 832A0560: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A0564: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A0568: 394AACE8  addi r10, r10, -0x5318
	ctx.r[10].s64 = ctx.r[10].s64 + -21272;
	// 832A056C: 816BACD0  lwz r11, -0x5330(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21296 as u32) ) } as u64;
	// 832A0570: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832A0574: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 832A0578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0580 size=112
    let mut pc: u32 = 0x832A0580;
    'dispatch: loop {
        match pc {
            0x832A0580 => {
    //   block [0x832A0580..0x832A05F0)
	// 832A0580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A058C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A0590: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 832A0594: 38EAACE8  addi r7, r10, -0x5318
	ctx.r[7].s64 = ctx.r[10].s64 + -21272;
	// 832A0598: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A059C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A05A0: 388A1A48  addi r4, r10, 0x1a48
	ctx.r[4].s64 = ctx.r[10].s64 + 6728;
	// 832A05A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A05A8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A05AC: 396B19E8  addi r11, r11, 0x19e8
	ctx.r[11].s64 = ctx.r[11].s64 + 6632;
	// 832A05B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A05B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A05B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A05BC: 386AA2D4  addi r3, r10, -0x5d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -23852;
	// 832A05C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A05C4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 832A05C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A05CC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 832A05D0: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 832A05D4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A05D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A05DC: 4BAB5665  bl 0x82d55c40
	ctx.lr = 0x832A05E0;
	sub_82D55C40(ctx, base);
	// 832A05E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A05E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A05E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A05EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A05F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A05F0 size=92
    let mut pc: u32 = 0x832A05F0;
    'dispatch: loop {
        match pc {
            0x832A05F0 => {
    //   block [0x832A05F0..0x832A064C)
	// 832A05F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A05F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A05F8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A05FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A0600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A0604: 4BB1C3ED  bl 0x82dbc9f0
	ctx.lr = 0x832A0608;
	sub_82DBC9F0(ctx, base);
	// 832A0608: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A060C: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A0610: 394B1A6C  addi r10, r11, 0x1a6c
	ctx.r[10].s64 = ctx.r[11].s64 + 6764;
	// 832A0614: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A0618: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 832A061C: 396BA304  addi r11, r11, -0x5cfc
	ctx.r[11].s64 = ctx.r[11].s64 + -23804;
	// 832A0620: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A0624: 39482FB8  addi r10, r8, 0x2fb8
	ctx.r[10].s64 = ctx.r[8].s64 + 12216;
	// 832A0628: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A062C: 39492FD0  addi r10, r9, 0x2fd0
	ctx.r[10].s64 = ctx.r[9].s64 + 12240;
	// 832A0630: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A0634: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A0638: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A063C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832A0640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A0650 size=24
    let mut pc: u32 = 0x832A0650;
    'dispatch: loop {
        match pc {
            0x832A0650 => {
    //   block [0x832A0650..0x832A0668)
	// 832A0650: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A0654: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A0658: 394AAE50  addi r10, r10, -0x51b0
	ctx.r[10].s64 = ctx.r[10].s64 + -20912;
	// 832A065C: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 832A0660: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832A0664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0668 size=116
    let mut pc: u32 = 0x832A0668;
    'dispatch: loop {
        match pc {
            0x832A0668 => {
    //   block [0x832A0668..0x832A06DC)
	// 832A0668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A066C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0674: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0678: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A067C: 392B19C0  addi r9, r11, 0x19c0
	ctx.r[9].s64 = ctx.r[11].s64 + 6592;
	// 832A0680: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 832A0684: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0688: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 832A068C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 832A0690: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A0694: 388A1A6C  addi r4, r10, 0x1a6c
	ctx.r[4].s64 = ctx.r[10].s64 + 6764;
	// 832A0698: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A069C: 396BAE50  addi r11, r11, -0x51b0
	ctx.r[11].s64 = ctx.r[11].s64 + -20912;
	// 832A06A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 832A06A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A06A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832A06AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A06B0: 386AA314  addi r3, r10, -0x5cec
	ctx.r[3].s64 = ctx.r[10].s64 + -23788;
	// 832A06B4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832A06B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 832A06BC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A06C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 832A06C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A06C8: 4BAB5579  bl 0x82d55c40
	ctx.lr = 0x832A06CC;
	sub_82D55C40(ctx, base);
	// 832A06CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A06D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A06D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A06D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A06E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A06E0 size=108
    let mut pc: u32 = 0x832A06E0;
    'dispatch: loop {
        match pc {
            0x832A06E0 => {
    //   block [0x832A06E0..0x832A074C)
	// 832A06E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A06E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A06E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A06EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A06F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A06F4: 38EB1AA4  addi r7, r11, 0x1aa4
	ctx.r[7].s64 = ctx.r[11].s64 + 6820;
	// 832A06F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A06FC: 388A1B2C  addi r4, r10, 0x1b2c
	ctx.r[4].s64 = ctx.r[10].s64 + 6956;
	// 832A0700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0704: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0708: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A070C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0710: 386AA344  addi r3, r10, -0x5cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -23740;
	// 832A0714: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A0718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A071C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A072C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A0730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A0738: 4BAB5509  bl 0x82d55c40
	ctx.lr = 0x832A073C;
	sub_82D55C40(ctx, base);
	// 832A073C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0750 size=112
    let mut pc: u32 = 0x832A0750;
    'dispatch: loop {
        match pc {
            0x832A0750 => {
    //   block [0x832A0750..0x832A07C0)
	// 832A0750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A075C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0760: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0764: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 832A0768: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A076C: 390B1AD4  addi r8, r11, 0x1ad4
	ctx.r[8].s64 = ctx.r[11].s64 + 6868;
	// 832A0770: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A0774: 388A1B40  addi r4, r10, 0x1b40
	ctx.r[4].s64 = ctx.r[10].s64 + 6976;
	// 832A0778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A077C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0788: 386AA374  addi r3, r10, -0x5c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23692;
	// 832A078C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A079C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A07A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A07A4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A07A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A07AC: 4BAB5495  bl 0x82d55c40
	ctx.lr = 0x832A07B0;
	sub_82D55C40(ctx, base);
	// 832A07B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A07B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A07B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A07BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A07C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A07C0 size=92
    let mut pc: u32 = 0x832A07C0;
    'dispatch: loop {
        match pc {
            0x832A07C0 => {
    //   block [0x832A07C0..0x832A081C)
	// 832A07C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A07C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A07C8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A07CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A07D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A07D4: 4BB1D10D  bl 0x82dbd8e0
	ctx.lr = 0x832A07D8;
	sub_82DBD8E0(ctx, base);
	// 832A07D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A07DC: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A07E0: 394B1D60  addi r10, r11, 0x1d60
	ctx.r[10].s64 = ctx.r[11].s64 + 7520;
	// 832A07E4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A07E8: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 832A07EC: 396BA3A4  addi r11, r11, -0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + -23644;
	// 832A07F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A07F4: 39483120  addi r10, r8, 0x3120
	ctx.r[10].s64 = ctx.r[8].s64 + 12576;
	// 832A07F8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A07FC: 39493108  addi r10, r9, 0x3108
	ctx.r[10].s64 = ctx.r[9].s64 + 12552;
	// 832A0800: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A0804: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A0808: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A080C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832A0810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0820 size=116
    let mut pc: u32 = 0x832A0820;
    'dispatch: loop {
        match pc {
            0x832A0820 => {
    //   block [0x832A0820..0x832A0894)
	// 832A0820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A082C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0830: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0834: 392B1CA0  addi r9, r11, 0x1ca0
	ctx.r[9].s64 = ctx.r[11].s64 + 7328;
	// 832A0838: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 832A083C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0840: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 832A0844: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 832A0848: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A084C: 388A1D60  addi r4, r10, 0x1d60
	ctx.r[4].s64 = ctx.r[10].s64 + 7520;
	// 832A0850: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0854: 396B1CD0  addi r11, r11, 0x1cd0
	ctx.r[11].s64 = ctx.r[11].s64 + 7376;
	// 832A0858: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 832A085C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0860: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832A0864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0868: 386AA3B4  addi r3, r10, -0x5c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23628;
	// 832A086C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A0870: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 832A0874: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A0878: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 832A087C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A0880: 4BAB53C1  bl 0x82d55c40
	ctx.lr = 0x832A0884;
	sub_82D55C40(ctx, base);
	// 832A0884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A088C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0898 size=108
    let mut pc: u32 = 0x832A0898;
    'dispatch: loop {
        match pc {
            0x832A0898 => {
    //   block [0x832A0898..0x832A0904)
	// 832A0898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A089C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A08A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A08A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A08A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A08AC: 38EB1D88  addi r7, r11, 0x1d88
	ctx.r[7].s64 = ctx.r[11].s64 + 7560;
	// 832A08B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 832A08B4: 388A1ED4  addi r4, r10, 0x1ed4
	ctx.r[4].s64 = ctx.r[10].s64 + 7892;
	// 832A08B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A08BC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A08C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A08C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A08C8: 386AA3E4  addi r3, r10, -0x5c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -23580;
	// 832A08CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A08D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A08D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A08D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A08DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A08E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A08E4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A08E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A08EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A08F0: 4BAB5351  bl 0x82d55c40
	ctx.lr = 0x832A08F4;
	sub_82D55C40(ctx, base);
	// 832A08F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A08F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A08FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0908 size=108
    let mut pc: u32 = 0x832A0908;
    'dispatch: loop {
        match pc {
            0x832A0908 => {
    //   block [0x832A0908..0x832A0974)
	// 832A0908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0914: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0918: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A091C: 38EB1DD0  addi r7, r11, 0x1dd0
	ctx.r[7].s64 = ctx.r[11].s64 + 7632;
	// 832A0920: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A0924: 388A1EFC  addi r4, r10, 0x1efc
	ctx.r[4].s64 = ctx.r[10].s64 + 7932;
	// 832A0928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A092C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A0934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0938: 386AA414  addi r3, r10, -0x5bec
	ctx.r[3].s64 = ctx.r[10].s64 + -23532;
	// 832A093C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A0940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A094C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0954: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A0958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A095C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A0960: 4BAB52E1  bl 0x82d55c40
	ctx.lr = 0x832A0964;
	sub_82D55C40(ctx, base);
	// 832A0964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A096C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0978 size=112
    let mut pc: u32 = 0x832A0978;
    'dispatch: loop {
        match pc {
            0x832A0978 => {
    //   block [0x832A0978..0x832A09E8)
	// 832A0978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0984: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0988: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A098C: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 832A0990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0994: 390B1E30  addi r8, r11, 0x1e30
	ctx.r[8].s64 = ctx.r[11].s64 + 7728;
	// 832A0998: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A099C: 388A1F24  addi r4, r10, 0x1f24
	ctx.r[4].s64 = ctx.r[10].s64 + 7972;
	// 832A09A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A09A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A09A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A09AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A09B0: 386AA444  addi r3, r10, -0x5bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -23484;
	// 832A09B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A09B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A09BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A09C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A09C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A09C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A09CC: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 832A09D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A09D4: 4BAB526D  bl 0x82d55c40
	ctx.lr = 0x832A09D8;
	sub_82D55C40(ctx, base);
	// 832A09D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A09DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A09E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A09E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A09E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A09E8 size=100
    let mut pc: u32 = 0x832A09E8;
    'dispatch: loop {
        match pc {
            0x832A09E8 => {
    //   block [0x832A09E8..0x832A0A4C)
	// 832A09E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A09EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A09F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A09F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A09F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A09FC: 38AAA734  addi r5, r10, -0x58cc
	ctx.r[5].s64 = ctx.r[10].s64 + -22732;
	// 832A0A00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0A08: 388A1F44  addi r4, r10, 0x1f44
	ctx.r[4].s64 = ctx.r[10].s64 + 8004;
	// 832A0A0C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0A1C: 386AA474  addi r3, r10, -0x5b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23436;
	// 832A0A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0A24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0A28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A0A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0A30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A0A34: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A0A38: 4BAB5209  bl 0x82d55c40
	ctx.lr = 0x832A0A3C;
	sub_82D55C40(ctx, base);
	// 832A0A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0A50 size=108
    let mut pc: u32 = 0x832A0A50;
    'dispatch: loop {
        match pc {
            0x832A0A50 => {
    //   block [0x832A0A50..0x832A0ABC)
	// 832A0A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0A5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0A64: 38EB1F80  addi r7, r11, 0x1f80
	ctx.r[7].s64 = ctx.r[11].s64 + 8064;
	// 832A0A68: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A0A6C: 388A1FE0  addi r4, r10, 0x1fe0
	ctx.r[4].s64 = ctx.r[10].s64 + 8160;
	// 832A0A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0A74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A0A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0A80: 386AA4A4  addi r3, r10, -0x5b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -23388;
	// 832A0A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A0A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0A9C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A0AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A0AA8: 4BAB5199  bl 0x82d55c40
	ctx.lr = 0x832A0AAC;
	sub_82D55C40(ctx, base);
	// 832A0AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0AC0 size=96
    let mut pc: u32 = 0x832A0AC0;
    'dispatch: loop {
        match pc {
            0x832A0AC0 => {
    //   block [0x832A0AC0..0x832A0B20)
	// 832A0AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0AC8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0ACC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A0AD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A0AD4: 4BB1BF1D  bl 0x82dbc9f0
	ctx.lr = 0x832A0AD8;
	sub_82DBC9F0(ctx, base);
	// 832A0AD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0ADC: 3CE082DB  lis r7, -0x7d25
	ctx.r[7].s64 = -2099576832;
	// 832A0AE0: 394B2048  addi r10, r11, 0x2048
	ctx.r[10].s64 = ctx.r[11].s64 + 8264;
	// 832A0AE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0AE8: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A0AEC: 392B2020  addi r9, r11, 0x2020
	ctx.r[9].s64 = ctx.r[11].s64 + 8224;
	// 832A0AF0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A0AF4: 396BA4D4  addi r11, r11, -0x5b2c
	ctx.r[11].s64 = ctx.r[11].s64 + -23340;
	// 832A0AF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A0AFC: 39473520  addi r10, r7, 0x3520
	ctx.r[10].s64 = ctx.r[7].s64 + 13600;
	// 832A0B00: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A0B04: 39483508  addi r10, r8, 0x3508
	ctx.r[10].s64 = ctx.r[8].s64 + 13576;
	// 832A0B08: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A0B0C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A0B10: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832A0B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0B20 size=100
    let mut pc: u32 = 0x832A0B20;
    'dispatch: loop {
        match pc {
            0x832A0B20 => {
    //   block [0x832A0B20..0x832A0B84)
	// 832A0B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0B2C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0B34: 38AAA314  addi r5, r10, -0x5cec
	ctx.r[5].s64 = ctx.r[10].s64 + -23788;
	// 832A0B38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0B40: 388A2048  addi r4, r10, 0x2048
	ctx.r[4].s64 = ctx.r[10].s64 + 8264;
	// 832A0B44: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0B54: 386AA4E4  addi r3, r10, -0x5b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -23324;
	// 832A0B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0B5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A0B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A0B6C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A0B70: 4BAB50D1  bl 0x82d55c40
	ctx.lr = 0x832A0B74;
	sub_82D55C40(ctx, base);
	// 832A0B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0B88 size=112
    let mut pc: u32 = 0x832A0B88;
    'dispatch: loop {
        match pc {
            0x832A0B88 => {
    //   block [0x832A0B88..0x832A0BF8)
	// 832A0B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0B94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0B98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0B9C: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A0BA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0BA4: 390B2090  addi r8, r11, 0x2090
	ctx.r[8].s64 = ctx.r[11].s64 + 8336;
	// 832A0BA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A0BAC: 388A2124  addi r4, r10, 0x2124
	ctx.r[4].s64 = ctx.r[10].s64 + 8484;
	// 832A0BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0BB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0BC0: 386AA514  addi r3, r10, -0x5aec
	ctx.r[3].s64 = ctx.r[10].s64 + -23276;
	// 832A0BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0BDC: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 832A0BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0BE4: 4BAB505D  bl 0x82d55c40
	ctx.lr = 0x832A0BE8;
	sub_82D55C40(ctx, base);
	// 832A0BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0BF8 size=112
    let mut pc: u32 = 0x832A0BF8;
    'dispatch: loop {
        match pc {
            0x832A0BF8 => {
    //   block [0x832A0BF8..0x832A0C68)
	// 832A0BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0C04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0C08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0C0C: 38AAA6A4  addi r5, r10, -0x595c
	ctx.r[5].s64 = ctx.r[10].s64 + -22876;
	// 832A0C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0C14: 390B2168  addi r8, r11, 0x2168
	ctx.r[8].s64 = ctx.r[11].s64 + 8552;
	// 832A0C18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A0C1C: 388A222C  addi r4, r10, 0x222c
	ctx.r[4].s64 = ctx.r[10].s64 + 8748;
	// 832A0C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0C24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0C30: 386AA544  addi r3, r10, -0x5abc
	ctx.r[3].s64 = ctx.r[10].s64 + -23228;
	// 832A0C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0C4C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A0C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0C54: 4BAB4FED  bl 0x82d55c40
	ctx.lr = 0x832A0C58;
	sub_82D55C40(ctx, base);
	// 832A0C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0C68 size=112
    let mut pc: u32 = 0x832A0C68;
    'dispatch: loop {
        match pc {
            0x832A0C68 => {
    //   block [0x832A0C68..0x832A0CD8)
	// 832A0C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0C74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0C78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0C7C: 38AAA544  addi r5, r10, -0x5abc
	ctx.r[5].s64 = ctx.r[10].s64 + -23228;
	// 832A0C80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0C84: 390B21C8  addi r8, r11, 0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + 8648;
	// 832A0C88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A0C8C: 388A2244  addi r4, r10, 0x2244
	ctx.r[4].s64 = ctx.r[10].s64 + 8772;
	// 832A0C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0C94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0CA0: 386AA574  addi r3, r10, -0x5a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23180;
	// 832A0CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0CBC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A0CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0CC4: 4BAB4F7D  bl 0x82d55c40
	ctx.lr = 0x832A0CC8;
	sub_82D55C40(ctx, base);
	// 832A0CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0CD8 size=92
    let mut pc: u32 = 0x832A0CD8;
    'dispatch: loop {
        match pc {
            0x832A0CD8 => {
    //   block [0x832A0CD8..0x832A0D34)
	// 832A0CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0CE0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0CE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A0CE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A0CEC: 4BB3FDE5  bl 0x82de0ad0
	ctx.lr = 0x832A0CF0;
	sub_82DE0AD0(ctx, base);
	// 832A0CF0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0CF4: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A0CF8: 394B22A0  addi r10, r11, 0x22a0
	ctx.r[10].s64 = ctx.r[11].s64 + 8864;
	// 832A0CFC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A0D00: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 832A0D04: 396BA5A4  addi r11, r11, -0x5a5c
	ctx.r[11].s64 = ctx.r[11].s64 + -23132;
	// 832A0D08: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A0D0C: 394839E8  addi r10, r8, 0x39e8
	ctx.r[10].s64 = ctx.r[8].s64 + 14824;
	// 832A0D10: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A0D14: 394939D0  addi r10, r9, 0x39d0
	ctx.r[10].s64 = ctx.r[9].s64 + 14800;
	// 832A0D18: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A0D1C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A0D20: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A0D24: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 832A0D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A0D38 size=24
    let mut pc: u32 = 0x832A0D38;
    'dispatch: loop {
        match pc {
            0x832A0D38 => {
    //   block [0x832A0D38..0x832A0D50)
	// 832A0D38: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A0D3C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A0D40: 394AB080  addi r10, r10, -0x4f80
	ctx.r[10].s64 = ctx.r[10].s64 + -20352;
	// 832A0D44: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 832A0D48: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832A0D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0D50 size=116
    let mut pc: u32 = 0x832A0D50;
    'dispatch: loop {
        match pc {
            0x832A0D50 => {
    //   block [0x832A0D50..0x832A0DC4)
	// 832A0D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0D5C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A0D60: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832A0D64: 390AB080  addi r8, r10, -0x4f80
	ctx.r[8].s64 = ctx.r[10].s64 + -20352;
	// 832A0D68: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0D6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0D70: 38AAA544  addi r5, r10, -0x5abc
	ctx.r[5].s64 = ctx.r[10].s64 + -23228;
	// 832A0D74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0D78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A0D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0D84: 388A22A0  addi r4, r10, 0x22a0
	ctx.r[4].s64 = ctx.r[10].s64 + 8864;
	// 832A0D88: 396B2278  addi r11, r11, 0x2278
	ctx.r[11].s64 = ctx.r[11].s64 + 8824;
	// 832A0D8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0D90: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A0D94: 386AA5B4  addi r3, r10, -0x5a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23116;
	// 832A0D98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 832A0D9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0DA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 832A0DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0DAC: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 832A0DB0: 4BAB4E91  bl 0x82d55c40
	ctx.lr = 0x832A0DB4;
	sub_82D55C40(ctx, base);
	// 832A0DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0DC8 size=112
    let mut pc: u32 = 0x832A0DC8;
    'dispatch: loop {
        match pc {
            0x832A0DC8 => {
    //   block [0x832A0DC8..0x832A0E38)
	// 832A0DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0DD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0DD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0DDC: 38AAA914  addi r5, r10, -0x56ec
	ctx.r[5].s64 = ctx.r[10].s64 + -22252;
	// 832A0DE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0DE4: 390B22E0  addi r8, r11, 0x22e0
	ctx.r[8].s64 = ctx.r[11].s64 + 8928;
	// 832A0DE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A0DEC: 388A2340  addi r4, r10, 0x2340
	ctx.r[4].s64 = ctx.r[10].s64 + 9024;
	// 832A0DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0DF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0E00: 386AA5E4  addi r3, r10, -0x5a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -23068;
	// 832A0E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0E1C: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 832A0E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0E24: 4BAB4E1D  bl 0x82d55c40
	ctx.lr = 0x832A0E28;
	sub_82D55C40(ctx, base);
	// 832A0E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0E38 size=100
    let mut pc: u32 = 0x832A0E38;
    'dispatch: loop {
        match pc {
            0x832A0E38 => {
    //   block [0x832A0E38..0x832A0E9C)
	// 832A0E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0E44: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0E4C: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A0E50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0E58: 388A2354  addi r4, r10, 0x2354
	ctx.r[4].s64 = ctx.r[10].s64 + 9044;
	// 832A0E5C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0E6C: 386AA614  addi r3, r10, -0x59ec
	ctx.r[3].s64 = ctx.r[10].s64 + -23020;
	// 832A0E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A0E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A0E84: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A0E88: 4BAB4DB9  bl 0x82d55c40
	ctx.lr = 0x832A0E8C;
	sub_82D55C40(ctx, base);
	// 832A0E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0EA0 size=112
    let mut pc: u32 = 0x832A0EA0;
    'dispatch: loop {
        match pc {
            0x832A0EA0 => {
    //   block [0x832A0EA0..0x832A0F10)
	// 832A0EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0EAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0EB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0EB4: 38AAA794  addi r5, r10, -0x586c
	ctx.r[5].s64 = ctx.r[10].s64 + -22636;
	// 832A0EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0EBC: 390B2378  addi r8, r11, 0x2378
	ctx.r[8].s64 = ctx.r[11].s64 + 9080;
	// 832A0EC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A0EC4: 388A23F0  addi r4, r10, 0x23f0
	ctx.r[4].s64 = ctx.r[10].s64 + 9200;
	// 832A0EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0ECC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0ED8: 386AA644  addi r3, r10, -0x59bc
	ctx.r[3].s64 = ctx.r[10].s64 + -22972;
	// 832A0EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0EF4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A0EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0EFC: 4BAB4D45  bl 0x82d55c40
	ctx.lr = 0x832A0F00;
	sub_82D55C40(ctx, base);
	// 832A0F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0F10 size=112
    let mut pc: u32 = 0x832A0F10;
    'dispatch: loop {
        match pc {
            0x832A0F10 => {
    //   block [0x832A0F10..0x832A0F80)
	// 832A0F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0F1C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A0F20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0F24: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A0F28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0F2C: 390B2420  addi r8, r11, 0x2420
	ctx.r[8].s64 = ctx.r[11].s64 + 9248;
	// 832A0F30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A0F34: 388A2480  addi r4, r10, 0x2480
	ctx.r[4].s64 = ctx.r[10].s64 + 9344;
	// 832A0F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0F3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0F48: 386AA674  addi r3, r10, -0x598c
	ctx.r[3].s64 = ctx.r[10].s64 + -22924;
	// 832A0F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0F5C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A0F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0F64: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 832A0F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0F6C: 4BAB4CD5  bl 0x82d55c40
	ctx.lr = 0x832A0F70;
	sub_82D55C40(ctx, base);
	// 832A0F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0F80 size=100
    let mut pc: u32 = 0x832A0F80;
    'dispatch: loop {
        match pc {
            0x832A0F80 => {
    //   block [0x832A0F80..0x832A0FE4)
	// 832A0F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0F8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0F94: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 832A0F98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0FA0: 388A24A0  addi r4, r10, 0x24a0
	ctx.r[4].s64 = ctx.r[10].s64 + 9376;
	// 832A0FA4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0FB4: 386AA6A4  addi r3, r10, -0x595c
	ctx.r[3].s64 = ctx.r[10].s64 + -22876;
	// 832A0FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0FBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0FC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A0FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0FC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A0FCC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A0FD0: 4BAB4C71  bl 0x82d55c40
	ctx.lr = 0x832A0FD4;
	sub_82D55C40(ctx, base);
	// 832A0FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0FE8 size=96
    let mut pc: u32 = 0x832A0FE8;
    'dispatch: loop {
        match pc {
            0x832A0FE8 => {
    //   block [0x832A0FE8..0x832A1048)
	// 832A0FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0FF4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A0FFC: 388A24B0  addi r4, r10, 0x24b0
	ctx.r[4].s64 = ctx.r[10].s64 + 9392;
	// 832A1000: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1008: 386AA6D4  addi r3, r10, -0x592c
	ctx.r[3].s64 = ctx.r[10].s64 + -22828;
	// 832A100C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1014: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A1018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A101C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1024: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A1028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A102C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A1034: 4BAB4C0D  bl 0x82d55c40
	ctx.lr = 0x832A1038;
	sub_82D55C40(ctx, base);
	// 832A1038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A103C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A1048 size=24
    let mut pc: u32 = 0x832A1048;
    'dispatch: loop {
        match pc {
            0x832A1048 => {
    //   block [0x832A1048..0x832A1060)
	// 832A1048: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A104C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A1050: 394AB22C  addi r10, r10, -0x4dd4
	ctx.r[10].s64 = ctx.r[10].s64 + -19924;
	// 832A1054: 816BB228  lwz r11, -0x4dd8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19928 as u32) ) } as u64;
	// 832A1058: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832A105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1060 size=116
    let mut pc: u32 = 0x832A1060;
    'dispatch: loop {
        match pc {
            0x832A1060 => {
    //   block [0x832A1060..0x832A10D4)
	// 832A1060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A106C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A1070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1074: 390BB22C  addi r8, r11, -0x4dd4
	ctx.r[8].s64 = ctx.r[11].s64 + -19924;
	// 832A1078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A107C: 392A2598  addi r9, r10, 0x2598
	ctx.r[9].s64 = ctx.r[10].s64 + 9624;
	// 832A1080: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A1084: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 832A1088: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A108C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1094: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A109C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A10A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A10A4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A10A8: 388A25AC  addi r4, r10, 0x25ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9644;
	// 832A10AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A10B0: 386BA704  addi r3, r11, -0x58fc
	ctx.r[3].s64 = ctx.r[11].s64 + -22780;
	// 832A10B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A10B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A10BC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A10C0: 4BAB4B81  bl 0x82d55c40
	ctx.lr = 0x832A10C4;
	sub_82D55C40(ctx, base);
	// 832A10C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A10C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A10CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A10D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A10D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A10D8 size=104
    let mut pc: u32 = 0x832A10D8;
    'dispatch: loop {
        match pc {
            0x832A10D8 => {
    //   block [0x832A10D8..0x832A1140)
	// 832A10D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A10DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A10E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A10E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A10E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A10EC: 392A2644  addi r9, r10, 0x2644
	ctx.r[9].s64 = ctx.r[10].s64 + 9796;
	// 832A10F0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A10F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A10F8: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A10FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A110C: 388A2658  addi r4, r10, 0x2658
	ctx.r[4].s64 = ctx.r[10].s64 + 9816;
	// 832A1110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1114: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1118: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A111C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A1120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A1124: 386AA734  addi r3, r10, -0x58cc
	ctx.r[3].s64 = ctx.r[10].s64 + -22732;
	// 832A1128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A112C: 4BAB4B15  bl 0x82d55c40
	ctx.lr = 0x832A1130;
	sub_82D55C40(ctx, base);
	// 832A1130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A113C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1140 size=116
    let mut pc: u32 = 0x832A1140;
    'dispatch: loop {
        match pc {
            0x832A1140 => {
    //   block [0x832A1140..0x832A11B4)
	// 832A1140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A114C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1154: 390B2720  addi r8, r11, 0x2720
	ctx.r[8].s64 = ctx.r[11].s64 + 10016;
	// 832A1158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A115C: 392A270C  addi r9, r10, 0x270c
	ctx.r[9].s64 = ctx.r[10].s64 + 9996;
	// 832A1160: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1164: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 832A1168: 38AAA914  addi r5, r10, -0x56ec
	ctx.r[5].s64 = ctx.r[10].s64 + -22252;
	// 832A116C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1174: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A117C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1184: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A1188: 388A2738  addi r4, r10, 0x2738
	ctx.r[4].s64 = ctx.r[10].s64 + 10040;
	// 832A118C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A1190: 386BA764  addi r3, r11, -0x589c
	ctx.r[3].s64 = ctx.r[11].s64 + -22684;
	// 832A1194: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A1198: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A119C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A11A0: 4BAB4AA1  bl 0x82d55c40
	ctx.lr = 0x832A11A4;
	sub_82D55C40(ctx, base);
	// 832A11A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A11A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A11AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A11B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A11B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A11B8 size=100
    let mut pc: u32 = 0x832A11B8;
    'dispatch: loop {
        match pc {
            0x832A11B8 => {
    //   block [0x832A11B8..0x832A121C)
	// 832A11B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A11BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A11C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A11C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A11C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A11CC: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 832A11D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A11D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A11D8: 388A2748  addi r4, r10, 0x2748
	ctx.r[4].s64 = ctx.r[10].s64 + 10056;
	// 832A11DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A11E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A11E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A11E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A11EC: 386AA794  addi r3, r10, -0x586c
	ctx.r[3].s64 = ctx.r[10].s64 + -22636;
	// 832A11F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A11F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A11F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A11FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A1204: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1208: 4BAB4A39  bl 0x82d55c40
	ctx.lr = 0x832A120C;
	sub_82D55C40(ctx, base);
	// 832A120C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1220 size=100
    let mut pc: u32 = 0x832A1220;
    'dispatch: loop {
        match pc {
            0x832A1220 => {
    //   block [0x832A1220..0x832A1284)
	// 832A1220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A122C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1234: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 832A1238: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A123C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1240: 388A275C  addi r4, r10, 0x275c
	ctx.r[4].s64 = ctx.r[10].s64 + 10076;
	// 832A1244: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A124C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1254: 386AA7C4  addi r3, r10, -0x583c
	ctx.r[3].s64 = ctx.r[10].s64 + -22588;
	// 832A1258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A125C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1260: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A1264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1268: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A126C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1270: 4BAB49D1  bl 0x82d55c40
	ctx.lr = 0x832A1274;
	sub_82D55C40(ctx, base);
	// 832A1274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A127C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1288 size=96
    let mut pc: u32 = 0x832A1288;
    'dispatch: loop {
        match pc {
            0x832A1288 => {
    //   block [0x832A1288..0x832A12E8)
	// 832A1288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A128C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1294: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A129C: 388A2774  addi r4, r10, 0x2774
	ctx.r[4].s64 = ctx.r[10].s64 + 10100;
	// 832A12A0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A12A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A12A8: 386AA7F4  addi r3, r10, -0x580c
	ctx.r[3].s64 = ctx.r[10].s64 + -22540;
	// 832A12AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A12B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A12B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A12B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A12BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A12C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A12C4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A12C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A12CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A12D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A12D4: 4BAB496D  bl 0x82d55c40
	ctx.lr = 0x832A12D8;
	sub_82D55C40(ctx, base);
	// 832A12D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A12DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A12E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A12E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A12E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A12E8 size=96
    let mut pc: u32 = 0x832A12E8;
    'dispatch: loop {
        match pc {
            0x832A12E8 => {
    //   block [0x832A12E8..0x832A1348)
	// 832A12E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A12EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A12F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A12F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A12F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A12FC: 388A278C  addi r4, r10, 0x278c
	ctx.r[4].s64 = ctx.r[10].s64 + 10124;
	// 832A1300: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1308: 386AA824  addi r3, r10, -0x57dc
	ctx.r[3].s64 = ctx.r[10].s64 + -22492;
	// 832A130C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1314: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A1318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A131C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1324: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A1328: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A132C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1330: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A1334: 4BAB490D  bl 0x82d55c40
	ctx.lr = 0x832A1338;
	sub_82D55C40(ctx, base);
	// 832A1338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A133C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1348 size=112
    let mut pc: u32 = 0x832A1348;
    'dispatch: loop {
        match pc {
            0x832A1348 => {
    //   block [0x832A1348..0x832A13B8)
	// 832A1348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A134C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1354: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1358: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A135C: 38AAA794  addi r5, r10, -0x586c
	ctx.r[5].s64 = ctx.r[10].s64 + -22636;
	// 832A1360: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1364: 390B2840  addi r8, r11, 0x2840
	ctx.r[8].s64 = ctx.r[11].s64 + 10304;
	// 832A1368: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 832A136C: 388A2900  addi r4, r10, 0x2900
	ctx.r[4].s64 = ctx.r[10].s64 + 10496;
	// 832A1370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1374: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A137C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1380: 386AA854  addi r3, r10, -0x57ac
	ctx.r[3].s64 = ctx.r[10].s64 + -22444;
	// 832A1384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A138C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A139C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A13A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A13A4: 4BAB489D  bl 0x82d55c40
	ctx.lr = 0x832A13A8;
	sub_82D55C40(ctx, base);
	// 832A13A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A13AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A13B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A13B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A13B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A13B8 size=112
    let mut pc: u32 = 0x832A13B8;
    'dispatch: loop {
        match pc {
            0x832A13B8 => {
    //   block [0x832A13B8..0x832A1428)
	// 832A13B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A13BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A13C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A13C4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A13C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A13CC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A13D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A13D4: 390B291C  addi r8, r11, 0x291c
	ctx.r[8].s64 = ctx.r[11].s64 + 10524;
	// 832A13D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A13DC: 388A294C  addi r4, r10, 0x294c
	ctx.r[4].s64 = ctx.r[10].s64 + 10572;
	// 832A13E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A13E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A13E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A13EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A13F0: 386AA884  addi r3, r10, -0x577c
	ctx.r[3].s64 = ctx.r[10].s64 + -22396;
	// 832A13F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A13F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A13FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A140C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1414: 4BAB482D  bl 0x82d55c40
	ctx.lr = 0x832A1418;
	sub_82D55C40(ctx, base);
	// 832A1418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A141C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1428 size=112
    let mut pc: u32 = 0x832A1428;
    'dispatch: loop {
        match pc {
            0x832A1428 => {
    //   block [0x832A1428..0x832A1498)
	// 832A1428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A142C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1434: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1438: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A143C: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 832A1440: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1444: 390B2968  addi r8, r11, 0x2968
	ctx.r[8].s64 = ctx.r[11].s64 + 10600;
	// 832A1448: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A144C: 388A2980  addi r4, r10, 0x2980
	ctx.r[4].s64 = ctx.r[10].s64 + 10624;
	// 832A1450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1454: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1458: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A145C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1460: 386AA8B4  addi r3, r10, -0x574c
	ctx.r[3].s64 = ctx.r[10].s64 + -22348;
	// 832A1464: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A146C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1474: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A1478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A147C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A1480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1484: 4BAB47BD  bl 0x82d55c40
	ctx.lr = 0x832A1488;
	sub_82D55C40(ctx, base);
	// 832A1488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A148C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1498 size=96
    let mut pc: u32 = 0x832A1498;
    'dispatch: loop {
        match pc {
            0x832A1498 => {
    //   block [0x832A1498..0x832A14F8)
	// 832A1498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A149C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A14A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A14A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A14A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A14AC: 388A2994  addi r4, r10, 0x2994
	ctx.r[4].s64 = ctx.r[10].s64 + 10644;
	// 832A14B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A14B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A14B8: 386AA8E4  addi r3, r10, -0x571c
	ctx.r[3].s64 = ctx.r[10].s64 + -22300;
	// 832A14BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A14C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A14C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A14C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A14CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A14D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A14D4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A14D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A14DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A14E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A14E4: 4BAB475D  bl 0x82d55c40
	ctx.lr = 0x832A14E8;
	sub_82D55C40(ctx, base);
	// 832A14E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A14EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A14F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A14F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A14F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A14F8 size=100
    let mut pc: u32 = 0x832A14F8;
    'dispatch: loop {
        match pc {
            0x832A14F8 => {
    //   block [0x832A14F8..0x832A155C)
	// 832A14F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A14FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1504: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A150C: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 832A1510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1518: 388A29B0  addi r4, r10, 0x29b0
	ctx.r[4].s64 = ctx.r[10].s64 + 10672;
	// 832A151C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A152C: 386AA914  addi r3, r10, -0x56ec
	ctx.r[3].s64 = ctx.r[10].s64 + -22252;
	// 832A1530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1538: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A153C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1540: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A1544: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1548: 4BAB46F9  bl 0x82d55c40
	ctx.lr = 0x832A154C;
	sub_82D55C40(ctx, base);
	// 832A154C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A1560 size=12
    let mut pc: u32 = 0x832A1560;
    'dispatch: loop {
        match pc {
            0x832A1560 => {
    //   block [0x832A1560..0x832A156C)
	// 832A1560: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A1564: 386B7EE8  addi r3, r11, 0x7ee8
	ctx.r[3].s64 = ctx.r[11].s64 + 32488;
	// 832A1568: 4BA089B8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1570 size=108
    let mut pc: u32 = 0x832A1570;
    'dispatch: loop {
        match pc {
            0x832A1570 => {
    //   block [0x832A1570..0x832A15DC)
	// 832A1570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A157C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1584: 38EB4890  addi r7, r11, 0x4890
	ctx.r[7].s64 = ctx.r[11].s64 + 18576;
	// 832A1588: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 832A158C: 388A4908  addi r4, r10, 0x4908
	ctx.r[4].s64 = ctx.r[10].s64 + 18696;
	// 832A1590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1594: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A159C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A15A0: 386AAD68  addi r3, r10, -0x5298
	ctx.r[3].s64 = ctx.r[10].s64 + -21144;
	// 832A15A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A15A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A15AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A15B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A15B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A15B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A15BC: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 832A15C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A15C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A15C8: 4BAB4679  bl 0x82d55c40
	ctx.lr = 0x832A15CC;
	sub_82D55C40(ctx, base);
	// 832A15CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A15D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A15D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A15D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A15E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A15E0 size=108
    let mut pc: u32 = 0x832A15E0;
    'dispatch: loop {
        match pc {
            0x832A15E0 => {
    //   block [0x832A15E0..0x832A164C)
	// 832A15E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A15E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A15E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A15EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A15F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A15F4: 38EB491C  addi r7, r11, 0x491c
	ctx.r[7].s64 = ctx.r[11].s64 + 18716;
	// 832A15F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A15FC: 388A4934  addi r4, r10, 0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + 18740;
	// 832A1600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1604: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A160C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1610: 386AAD98  addi r3, r10, -0x5268
	ctx.r[3].s64 = ctx.r[10].s64 + -21096;
	// 832A1614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A161C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A162C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A1630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1638: 4BAB4609  bl 0x82d55c40
	ctx.lr = 0x832A163C;
	sub_82D55C40(ctx, base);
	// 832A163C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1650 size=112
    let mut pc: u32 = 0x832A1650;
    'dispatch: loop {
        match pc {
            0x832A1650 => {
    //   block [0x832A1650..0x832A16C0)
	// 832A1650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A165C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1660: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1664: 38AA9F54  addi r5, r10, -0x60ac
	ctx.r[5].s64 = ctx.r[10].s64 + -24748;
	// 832A1668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A166C: 390B495C  addi r8, r11, 0x495c
	ctx.r[8].s64 = ctx.r[11].s64 + 18780;
	// 832A1670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1674: 388A4974  addi r4, r10, 0x4974
	ctx.r[4].s64 = ctx.r[10].s64 + 18804;
	// 832A1678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A167C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1688: 386AADC8  addi r3, r10, -0x5238
	ctx.r[3].s64 = ctx.r[10].s64 + -21048;
	// 832A168C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A169C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A16A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A16A4: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 832A16A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A16AC: 4BAB4595  bl 0x82d55c40
	ctx.lr = 0x832A16B0;
	sub_82D55C40(ctx, base);
	// 832A16B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A16B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A16B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A16BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A16C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A16C0 size=108
    let mut pc: u32 = 0x832A16C0;
    'dispatch: loop {
        match pc {
            0x832A16C0 => {
    //   block [0x832A16C0..0x832A172C)
	// 832A16C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A16C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A16C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A16CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A16D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A16D4: 38EB49AC  addi r7, r11, 0x49ac
	ctx.r[7].s64 = ctx.r[11].s64 + 18860;
	// 832A16D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A16DC: 388A4A34  addi r4, r10, 0x4a34
	ctx.r[4].s64 = ctx.r[10].s64 + 18996;
	// 832A16E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A16E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A16E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A16EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A16F0: 386AADF8  addi r3, r10, -0x5208
	ctx.r[3].s64 = ctx.r[10].s64 + -21000;
	// 832A16F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A16F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A16FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A170C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A1710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1718: 4BAB4529  bl 0x82d55c40
	ctx.lr = 0x832A171C;
	sub_82D55C40(ctx, base);
	// 832A171C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1730 size=108
    let mut pc: u32 = 0x832A1730;
    'dispatch: loop {
        match pc {
            0x832A1730 => {
    //   block [0x832A1730..0x832A179C)
	// 832A1730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A173C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1740: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1744: 38EB49DC  addi r7, r11, 0x49dc
	ctx.r[7].s64 = ctx.r[11].s64 + 18908;
	// 832A1748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A174C: 388A4A54  addi r4, r10, 0x4a54
	ctx.r[4].s64 = ctx.r[10].s64 + 19028;
	// 832A1750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1754: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1760: 386AAE28  addi r3, r10, -0x51d8
	ctx.r[3].s64 = ctx.r[10].s64 + -20952;
	// 832A1764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A176C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A177C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1788: 4BAB44B9  bl 0x82d55c40
	ctx.lr = 0x832A178C;
	sub_82D55C40(ctx, base);
	// 832A178C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A17A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A17A0 size=112
    let mut pc: u32 = 0x832A17A0;
    'dispatch: loop {
        match pc {
            0x832A17A0 => {
    //   block [0x832A17A0..0x832A1810)
	// 832A17A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A17A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A17A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A17AC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A17B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A17B4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A17B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A17BC: 390B49F4  addi r8, r11, 0x49f4
	ctx.r[8].s64 = ctx.r[11].s64 + 18932;
	// 832A17C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A17C4: 388A4A68  addi r4, r10, 0x4a68
	ctx.r[4].s64 = ctx.r[10].s64 + 19048;
	// 832A17C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A17CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A17D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A17D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A17D8: 386AAE58  addi r3, r10, -0x51a8
	ctx.r[3].s64 = ctx.r[10].s64 + -20904;
	// 832A17DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A17E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A17E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A17E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A17EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A17F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A17F4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A17F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A17FC: 4BAB4445  bl 0x82d55c40
	ctx.lr = 0x832A1800;
	sub_82D55C40(ctx, base);
	// 832A1800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A180C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1810 size=112
    let mut pc: u32 = 0x832A1810;
    'dispatch: loop {
        match pc {
            0x832A1810 => {
    //   block [0x832A1810..0x832A1880)
	// 832A1810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A181C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A1820: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1824: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A1828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A182C: 390B4AA8  addi r8, r11, 0x4aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 19112;
	// 832A1830: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A1834: 388A4B00  addi r4, r10, 0x4b00
	ctx.r[4].s64 = ctx.r[10].s64 + 19200;
	// 832A1838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A183C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1848: 386AAE88  addi r3, r10, -0x5178
	ctx.r[3].s64 = ctx.r[10].s64 + -20856;
	// 832A184C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A185C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1864: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 832A1868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A186C: 4BAB43D5  bl 0x82d55c40
	ctx.lr = 0x832A1870;
	sub_82D55C40(ctx, base);
	// 832A1870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A187C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A1880 size=28
    let mut pc: u32 = 0x832A1880;
    'dispatch: loop {
        match pc {
            0x832A1880 => {
    //   block [0x832A1880..0x832A189C)
	// 832A1880: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A1884: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A1888: 396BC778  addi r11, r11, -0x3888
	ctx.r[11].s64 = ctx.r[11].s64 + -14472;
	// 832A188C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A1890: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A1894: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A1898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A18A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A18A0 size=28
    let mut pc: u32 = 0x832A18A0;
    'dispatch: loop {
        match pc {
            0x832A18A0 => {
    //   block [0x832A18A0..0x832A18BC)
	// 832A18A0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A18A4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A18A8: 396BCA38  addi r11, r11, -0x35c8
	ctx.r[11].s64 = ctx.r[11].s64 + -13768;
	// 832A18AC: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A18B0: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A18B4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A18B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A18C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A18C0 size=112
    let mut pc: u32 = 0x832A18C0;
    'dispatch: loop {
        match pc {
            0x832A18C0 => {
    //   block [0x832A18C0..0x832A1930)
	// 832A18C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A18C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A18C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A18CC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A18D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A18D4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A18D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A18DC: 390B5CEC  addi r8, r11, 0x5cec
	ctx.r[8].s64 = ctx.r[11].s64 + 23788;
	// 832A18E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A18E4: 388A5D2C  addi r4, r10, 0x5d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 23852;
	// 832A18E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A18EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A18F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A18F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A18F8: 386AB078  addi r3, r10, -0x4f88
	ctx.r[3].s64 = ctx.r[10].s64 + -20360;
	// 832A18FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A190C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1914: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A1918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A191C: 4BAB4325  bl 0x82d55c40
	ctx.lr = 0x832A1920;
	sub_82D55C40(ctx, base);
	// 832A1920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A192C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1930 size=112
    let mut pc: u32 = 0x832A1930;
    'dispatch: loop {
        match pc {
            0x832A1930 => {
    //   block [0x832A1930..0x832A19A0)
	// 832A1930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A193C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1940: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1944: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A1948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A194C: 390B5E4C  addi r8, r11, 0x5e4c
	ctx.r[8].s64 = ctx.r[11].s64 + 24140;
	// 832A1950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1954: 388A5EC4  addi r4, r10, 0x5ec4
	ctx.r[4].s64 = ctx.r[10].s64 + 24260;
	// 832A1958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A195C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1968: 386AB0A8  addi r3, r10, -0x4f58
	ctx.r[3].s64 = ctx.r[10].s64 + -20312;
	// 832A196C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A197C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A1980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1984: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A1988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A198C: 4BAB42B5  bl 0x82d55c40
	ctx.lr = 0x832A1990;
	sub_82D55C40(ctx, base);
	// 832A1990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A199C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A19A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A19A0 size=108
    let mut pc: u32 = 0x832A19A0;
    'dispatch: loop {
        match pc {
            0x832A19A0 => {
    //   block [0x832A19A0..0x832A1A0C)
	// 832A19A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A19A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A19A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A19AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A19B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A19B4: 38EB5F40  addi r7, r11, 0x5f40
	ctx.r[7].s64 = ctx.r[11].s64 + 24384;
	// 832A19B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A19BC: 388A6010  addi r4, r10, 0x6010
	ctx.r[4].s64 = ctx.r[10].s64 + 24592;
	// 832A19C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A19C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A19C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A19CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A19D0: 386AB138  addi r3, r10, -0x4ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -20168;
	// 832A19D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A19D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A19DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A19E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A19E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A19E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A19EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A19F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A19F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A19F8: 4BAB4249  bl 0x82d55c40
	ctx.lr = 0x832A19FC;
	sub_82D55C40(ctx, base);
	// 832A19FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1A10 size=108
    let mut pc: u32 = 0x832A1A10;
    'dispatch: loop {
        match pc {
            0x832A1A10 => {
    //   block [0x832A1A10..0x832A1A7C)
	// 832A1A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1A1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1A20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1A24: 38EB5F70  addi r7, r11, 0x5f70
	ctx.r[7].s64 = ctx.r[11].s64 + 24432;
	// 832A1A28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 832A1A2C: 388A602C  addi r4, r10, 0x602c
	ctx.r[4].s64 = ctx.r[10].s64 + 24620;
	// 832A1A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1A34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1A40: 386AB108  addi r3, r10, -0x4ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -20216;
	// 832A1A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1A5C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A1A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1A68: 4BAB41D9  bl 0x82d55c40
	ctx.lr = 0x832A1A6C;
	sub_82D55C40(ctx, base);
	// 832A1A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1A80 size=112
    let mut pc: u32 = 0x832A1A80;
    'dispatch: loop {
        match pc {
            0x832A1A80 => {
    //   block [0x832A1A80..0x832A1AF0)
	// 832A1A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1A8C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A1A90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1A94: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A1A98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1A9C: 390B5FB8  addi r8, r11, 0x5fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 24504;
	// 832A1AA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A1AA4: 388A604C  addi r4, r10, 0x604c
	ctx.r[4].s64 = ctx.r[10].s64 + 24652;
	// 832A1AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1AAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1AB8: 386AB0D8  addi r3, r10, -0x4f28
	ctx.r[3].s64 = ctx.r[10].s64 + -20264;
	// 832A1ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1AD4: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 832A1AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1ADC: 4BAB4165  bl 0x82d55c40
	ctx.lr = 0x832A1AE0;
	sub_82D55C40(ctx, base);
	// 832A1AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1AF0 size=112
    let mut pc: u32 = 0x832A1AF0;
    'dispatch: loop {
        match pc {
            0x832A1AF0 => {
    //   block [0x832A1AF0..0x832A1B60)
	// 832A1AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1AFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1B04: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A1B08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1B0C: 390B608C  addi r8, r11, 0x608c
	ctx.r[8].s64 = ctx.r[11].s64 + 24716;
	// 832A1B10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A1B14: 388A6104  addi r4, r10, 0x6104
	ctx.r[4].s64 = ctx.r[10].s64 + 24836;
	// 832A1B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1B1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1B28: 386AB168  addi r3, r10, -0x4e98
	ctx.r[3].s64 = ctx.r[10].s64 + -20120;
	// 832A1B2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1B44: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 832A1B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1B4C: 4BAB40F5  bl 0x82d55c40
	ctx.lr = 0x832A1B50;
	sub_82D55C40(ctx, base);
	// 832A1B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1B60 size=112
    let mut pc: u32 = 0x832A1B60;
    'dispatch: loop {
        match pc {
            0x832A1B60 => {
    //   block [0x832A1B60..0x832A1BD0)
	// 832A1B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1B6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1B74: 38AA8CC0  addi r5, r10, -0x7340
	ctx.r[5].s64 = ctx.r[10].s64 + -29504;
	// 832A1B78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1B7C: 390B6130  addi r8, r11, 0x6130
	ctx.r[8].s64 = ctx.r[11].s64 + 24880;
	// 832A1B80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A1B84: 388A61B4  addi r4, r10, 0x61b4
	ctx.r[4].s64 = ctx.r[10].s64 + 25012;
	// 832A1B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1B8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1B98: 386AB198  addi r3, r10, -0x4e68
	ctx.r[3].s64 = ctx.r[10].s64 + -20072;
	// 832A1B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1BB4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A1BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1BBC: 4BAB4085  bl 0x82d55c40
	ctx.lr = 0x832A1BC0;
	sub_82D55C40(ctx, base);
	// 832A1BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1BD0 size=108
    let mut pc: u32 = 0x832A1BD0;
    'dispatch: loop {
        match pc {
            0x832A1BD0 => {
    //   block [0x832A1BD0..0x832A1C3C)
	// 832A1BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1BDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1BE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1BE4: 38EB6238  addi r7, r11, 0x6238
	ctx.r[7].s64 = ctx.r[11].s64 + 25144;
	// 832A1BE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 832A1BEC: 388A62E0  addi r4, r10, 0x62e0
	ctx.r[4].s64 = ctx.r[10].s64 + 25312;
	// 832A1BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1BF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1C00: 386AB1C8  addi r3, r10, -0x4e38
	ctx.r[3].s64 = ctx.r[10].s64 + -20024;
	// 832A1C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1C1C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A1C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1C28: 4BAB4019  bl 0x82d55c40
	ctx.lr = 0x832A1C2C;
	sub_82D55C40(ctx, base);
	// 832A1C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1C40 size=108
    let mut pc: u32 = 0x832A1C40;
    'dispatch: loop {
        match pc {
            0x832A1C40 => {
    //   block [0x832A1C40..0x832A1CAC)
	// 832A1C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1C4C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1C50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1C54: 38EB6280  addi r7, r11, 0x6280
	ctx.r[7].s64 = ctx.r[11].s64 + 25216;
	// 832A1C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1C5C: 388A62FC  addi r4, r10, 0x62fc
	ctx.r[4].s64 = ctx.r[10].s64 + 25340;
	// 832A1C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1C64: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1C70: 386AB1F8  addi r3, r10, -0x4e08
	ctx.r[3].s64 = ctx.r[10].s64 + -19976;
	// 832A1C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1C8C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1C98: 4BAB3FA9  bl 0x82d55c40
	ctx.lr = 0x832A1C9C;
	sub_82D55C40(ctx, base);
	// 832A1C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1CB0 size=108
    let mut pc: u32 = 0x832A1CB0;
    'dispatch: loop {
        match pc {
            0x832A1CB0 => {
    //   block [0x832A1CB0..0x832A1D1C)
	// 832A1CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1CBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1CC4: 38EB62B0  addi r7, r11, 0x62b0
	ctx.r[7].s64 = ctx.r[11].s64 + 25264;
	// 832A1CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1CCC: 388A631C  addi r4, r10, 0x631c
	ctx.r[4].s64 = ctx.r[10].s64 + 25372;
	// 832A1CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1CD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1CE0: 386AB228  addi r3, r10, -0x4dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19928;
	// 832A1CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1CFC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A1D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1D08: 4BAB3F39  bl 0x82d55c40
	ctx.lr = 0x832A1D0C;
	sub_82D55C40(ctx, base);
	// 832A1D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1D20 size=100
    let mut pc: u32 = 0x832A1D20;
    'dispatch: loop {
        match pc {
            0x832A1D20 => {
    //   block [0x832A1D20..0x832A1D84)
	// 832A1D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1D28: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1D30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A1D34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A1D38: 4BAE9051  bl 0x82d8ad88
	ctx.lr = 0x832A1D3C;
	sub_82D8AD88(ctx, base);
	// 832A1D3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1D40: 3CE082E5  lis r7, -0x7d1b
	ctx.r[7].s64 = -2098921472;
	// 832A1D44: 394B648C  addi r10, r11, 0x648c
	ctx.r[10].s64 = ctx.r[11].s64 + 25740;
	// 832A1D48: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1D4C: 3D0082E5  lis r8, -0x7d1b
	ctx.r[8].s64 = -2098921472;
	// 832A1D50: 392B646C  addi r9, r11, 0x646c
	ctx.r[9].s64 = ctx.r[11].s64 + 25708;
	// 832A1D54: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A1D58: 396BB288  addi r11, r11, -0x4d78
	ctx.r[11].s64 = ctx.r[11].s64 + -19832;
	// 832A1D5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A1D60: 3947A5E0  addi r10, r7, -0x5a20
	ctx.r[10].s64 = ctx.r[7].s64 + -23072;
	// 832A1D64: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A1D68: 3948A5C8  addi r10, r8, -0x5a38
	ctx.r[10].s64 = ctx.r[8].s64 + -23096;
	// 832A1D6C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A1D70: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A1D74: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832A1D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1D88 size=112
    let mut pc: u32 = 0x832A1D88;
    'dispatch: loop {
        match pc {
            0x832A1D88 => {
    //   block [0x832A1D88..0x832A1DF8)
	// 832A1D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1D94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1D98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1D9C: 38AA8CC0  addi r5, r10, -0x7340
	ctx.r[5].s64 = ctx.r[10].s64 + -29504;
	// 832A1DA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1DA4: 390B63A8  addi r8, r11, 0x63a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25512;
	// 832A1DA8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 832A1DAC: 388A648C  addi r4, r10, 0x648c
	ctx.r[4].s64 = ctx.r[10].s64 + 25740;
	// 832A1DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1DB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1DC0: 386AB258  addi r3, r10, -0x4da8
	ctx.r[3].s64 = ctx.r[10].s64 + -19880;
	// 832A1DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1DDC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A1DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1DE4: 4BAB3E5D  bl 0x82d55c40
	ctx.lr = 0x832A1DE8;
	sub_82D55C40(ctx, base);
	// 832A1DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1DF8 size=108
    let mut pc: u32 = 0x832A1DF8;
    'dispatch: loop {
        match pc {
            0x832A1DF8 => {
    //   block [0x832A1DF8..0x832A1E64)
	// 832A1DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1E04: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1E0C: 38EB64B4  addi r7, r11, 0x64b4
	ctx.r[7].s64 = ctx.r[11].s64 + 25780;
	// 832A1E10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1E14: 388A650C  addi r4, r10, 0x650c
	ctx.r[4].s64 = ctx.r[10].s64 + 25868;
	// 832A1E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1E1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1E28: 386AB2C8  addi r3, r10, -0x4d38
	ctx.r[3].s64 = ctx.r[10].s64 + -19768;
	// 832A1E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1E44: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A1E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1E50: 4BAB3DF1  bl 0x82d55c40
	ctx.lr = 0x832A1E54;
	sub_82D55C40(ctx, base);
	// 832A1E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1E68 size=112
    let mut pc: u32 = 0x832A1E68;
    'dispatch: loop {
        match pc {
            0x832A1E68 => {
    //   block [0x832A1E68..0x832A1ED8)
	// 832A1E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1E74: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A1E78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1E7C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A1E80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1E84: 390B64E4  addi r8, r11, 0x64e4
	ctx.r[8].s64 = ctx.r[11].s64 + 25828;
	// 832A1E88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1E8C: 388A6544  addi r4, r10, 0x6544
	ctx.r[4].s64 = ctx.r[10].s64 + 25924;
	// 832A1E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1E94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1EA0: 386AB298  addi r3, r10, -0x4d68
	ctx.r[3].s64 = ctx.r[10].s64 + -19816;
	// 832A1EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1EBC: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A1EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1EC4: 4BAB3D7D  bl 0x82d55c40
	ctx.lr = 0x832A1EC8;
	sub_82D55C40(ctx, base);
	// 832A1EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1ED8 size=108
    let mut pc: u32 = 0x832A1ED8;
    'dispatch: loop {
        match pc {
            0x832A1ED8 => {
    //   block [0x832A1ED8..0x832A1F44)
	// 832A1ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1EE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1EE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1EEC: 38EB6578  addi r7, r11, 0x6578
	ctx.r[7].s64 = ctx.r[11].s64 + 25976;
	// 832A1EF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1EF4: 388A6624  addi r4, r10, 0x6624
	ctx.r[4].s64 = ctx.r[10].s64 + 26148;
	// 832A1EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1EFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1F00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1F08: 386AB2F8  addi r3, r10, -0x4d08
	ctx.r[3].s64 = ctx.r[10].s64 + -19720;
	// 832A1F0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1F24: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A1F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1F2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1F30: 4BAB3D11  bl 0x82d55c40
	ctx.lr = 0x832A1F34;
	sub_82D55C40(ctx, base);
	// 832A1F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1F48 size=112
    let mut pc: u32 = 0x832A1F48;
    'dispatch: loop {
        match pc {
            0x832A1F48 => {
    //   block [0x832A1F48..0x832A1FB8)
	// 832A1F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1F54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1F58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1F5C: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A1F60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1F64: 390B65A8  addi r8, r11, 0x65a8
	ctx.r[8].s64 = ctx.r[11].s64 + 26024;
	// 832A1F68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1F6C: 388A664C  addi r4, r10, 0x664c
	ctx.r[4].s64 = ctx.r[10].s64 + 26188;
	// 832A1F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1F74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1F80: 386AB328  addi r3, r10, -0x4cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19672;
	// 832A1F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1F94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A1F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1F9C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1FA4: 4BAB3C9D  bl 0x82d55c40
	ctx.lr = 0x832A1FA8;
	sub_82D55C40(ctx, base);
	// 832A1FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A1FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1FB8 size=108
    let mut pc: u32 = 0x832A1FB8;
    'dispatch: loop {
        match pc {
            0x832A1FB8 => {
    //   block [0x832A1FB8..0x832A2024)
	// 832A1FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1FC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1FCC: 38EB67A0  addi r7, r11, 0x67a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26528;
	// 832A1FD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1FD4: 388A686C  addi r4, r10, 0x686c
	ctx.r[4].s64 = ctx.r[10].s64 + 26732;
	// 832A1FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1FDC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1FE8: 386AB398  addi r3, r10, -0x4c68
	ctx.r[3].s64 = ctx.r[10].s64 + -19560;
	// 832A1FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A2004: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A2008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A200C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A2010: 4BAB3C31  bl 0x82d55c40
	ctx.lr = 0x832A2014;
	sub_82D55C40(ctx, base);
	// 832A2014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A2018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A201C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2028 size=112
    let mut pc: u32 = 0x832A2028;
    'dispatch: loop {
        match pc {
            0x832A2028 => {
    //   block [0x832A2028..0x832A2098)
	// 832A2028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2034: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2038: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A203C: 38AAB398  addi r5, r10, -0x4c68
	ctx.r[5].s64 = ctx.r[10].s64 + -19560;
	// 832A2040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2044: 390B67D0  addi r8, r11, 0x67d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26576;
	// 832A2048: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A204C: 388A6888  addi r4, r10, 0x6888
	ctx.r[4].s64 = ctx.r[10].s64 + 26760;
	// 832A2050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2054: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A205C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2060: 386AB358  addi r3, r10, -0x4ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -19624;
	// 832A2064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A206C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A207C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A2080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2084: 4BAB3BBD  bl 0x82d55c40
	ctx.lr = 0x832A2088;
	sub_82D55C40(ctx, base);
	// 832A2088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A208C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2098 size=112
    let mut pc: u32 = 0x832A2098;
    'dispatch: loop {
        match pc {
            0x832A2098 => {
    //   block [0x832A2098..0x832A2108)
	// 832A2098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A209C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A20A0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A20A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A20A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A20AC: 396B6860  addi r11, r11, 0x6860
	ctx.r[11].s64 = ctx.r[11].s64 + 26720;
	// 832A20B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A20B4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 832A20B8: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 832A20BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 832A20C0: 4BAFC681  bl 0x82d9e740
	ctx.lr = 0x832A20C4;
	sub_82D9E740(ctx, base);
	// 832A20C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A20C8: 3D0082E5  lis r8, -0x7d1b
	ctx.r[8].s64 = -2098921472;
	// 832A20CC: 394B68A4  addi r10, r11, 0x68a4
	ctx.r[10].s64 = ctx.r[11].s64 + 26788;
	// 832A20D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A20D4: 3D2082E5  lis r9, -0x7d1b
	ctx.r[9].s64 = -2098921472;
	// 832A20D8: 396BB388  addi r11, r11, -0x4c78
	ctx.r[11].s64 = ctx.r[11].s64 + -19576;
	// 832A20DC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A20E0: 3948A9A0  addi r10, r8, -0x5660
	ctx.r[10].s64 = ctx.r[8].s64 + -22112;
	// 832A20E4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A20E8: 3949A968  addi r10, r9, -0x5698
	ctx.r[10].s64 = ctx.r[9].s64 + -22168;
	// 832A20EC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A20F0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A20F4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A20F8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 832A20FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A2108 size=24
    let mut pc: u32 = 0x832A2108;
    'dispatch: loop {
        match pc {
            0x832A2108 => {
    //   block [0x832A2108..0x832A2120)
	// 832A2108: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A210C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A2110: 394ACD30  addi r10, r10, -0x32d0
	ctx.r[10].s64 = ctx.r[10].s64 + -13008;
	// 832A2114: 816BCD08  lwz r11, -0x32f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13048 as u32) ) } as u64;
	// 832A2118: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 832A211C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2120 size=116
    let mut pc: u32 = 0x832A2120;
    'dispatch: loop {
        match pc {
            0x832A2120 => {
    //   block [0x832A2120..0x832A2194)
	// 832A2120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A212C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A2130: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2134: 390BCD30  addi r8, r11, -0x32d0
	ctx.r[8].s64 = ctx.r[11].s64 + -13008;
	// 832A2138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A213C: 392A6848  addi r9, r10, 0x6848
	ctx.r[9].s64 = ctx.r[10].s64 + 26696;
	// 832A2140: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A2144: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 832A2148: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A214C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A2150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2154: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A215C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2164: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A2168: 388A68A4  addi r4, r10, 0x68a4
	ctx.r[4].s64 = ctx.r[10].s64 + 26788;
	// 832A216C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A2170: 386BB3C8  addi r3, r11, -0x4c38
	ctx.r[3].s64 = ctx.r[11].s64 + -19512;
	// 832A2174: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A2178: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A217C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 832A2180: 4BAB3AC1  bl 0x82d55c40
	ctx.lr = 0x832A2184;
	sub_82D55C40(ctx, base);
	// 832A2184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A2188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A218C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2198 size=112
    let mut pc: u32 = 0x832A2198;
    'dispatch: loop {
        match pc {
            0x832A2198 => {
    //   block [0x832A2198..0x832A2208)
	// 832A2198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A219C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A21A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A21A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A21A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A21AC: 38AA8A20  addi r5, r10, -0x75e0
	ctx.r[5].s64 = ctx.r[10].s64 + -30176;
	// 832A21B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A21B4: 390B6934  addi r8, r11, 0x6934
	ctx.r[8].s64 = ctx.r[11].s64 + 26932;
	// 832A21B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A21BC: 388A6964  addi r4, r10, 0x6964
	ctx.r[4].s64 = ctx.r[10].s64 + 26980;
	// 832A21C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A21C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A21C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A21CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A21D0: 386AB3F8  addi r3, r10, -0x4c08
	ctx.r[3].s64 = ctx.r[10].s64 + -19464;
	// 832A21D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A21D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A21DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A21E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A21E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A21E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A21EC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A21F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A21F4: 4BAB3A4D  bl 0x82d55c40
	ctx.lr = 0x832A21F8;
	sub_82D55C40(ctx, base);
	// 832A21F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A21FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2208 size=112
    let mut pc: u32 = 0x832A2208;
    'dispatch: loop {
        match pc {
            0x832A2208 => {
    //   block [0x832A2208..0x832A2278)
	// 832A2208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A220C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2214: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2218: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A221C: 38AA8CC0  addi r5, r10, -0x7340
	ctx.r[5].s64 = ctx.r[10].s64 + -29504;
	// 832A2220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2224: 390B69A0  addi r8, r11, 0x69a0
	ctx.r[8].s64 = ctx.r[11].s64 + 27040;
	// 832A2228: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A222C: 388A6A24  addi r4, r10, 0x6a24
	ctx.r[4].s64 = ctx.r[10].s64 + 27172;
	// 832A2230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2234: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A223C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2240: 386AB428  addi r3, r10, -0x4bd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19416;
	// 832A2244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A224C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A225C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A2260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2264: 4BAB39DD  bl 0x82d55c40
	ctx.lr = 0x832A2268;
	sub_82D55C40(ctx, base);
	// 832A2268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A226C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2278 size=112
    let mut pc: u32 = 0x832A2278;
    'dispatch: loop {
        match pc {
            0x832A2278 => {
    //   block [0x832A2278..0x832A22E8)
	// 832A2278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A227C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2284: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A2288: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A228C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A2290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2294: 390B6A38  addi r8, r11, 0x6a38
	ctx.r[8].s64 = ctx.r[11].s64 + 27192;
	// 832A2298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A229C: 388A6A60  addi r4, r10, 0x6a60
	ctx.r[4].s64 = ctx.r[10].s64 + 27232;
	// 832A22A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A22A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A22A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A22AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A22B0: 386AB458  addi r3, r10, -0x4ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -19368;
	// 832A22B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A22B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A22BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A22C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A22C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A22C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A22CC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A22D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A22D4: 4BAB396D  bl 0x82d55c40
	ctx.lr = 0x832A22D8;
	sub_82D55C40(ctx, base);
	// 832A22D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A22DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A22E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A22E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A22E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A22E8 size=112
    let mut pc: u32 = 0x832A22E8;
    'dispatch: loop {
        match pc {
            0x832A22E8 => {
    //   block [0x832A22E8..0x832A2358)
	// 832A22E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A22EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A22F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A22F4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A22F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A22FC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A2300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2304: 390B6A84  addi r8, r11, 0x6a84
	ctx.r[8].s64 = ctx.r[11].s64 + 27268;
	// 832A2308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A230C: 388A6AAC  addi r4, r10, 0x6aac
	ctx.r[4].s64 = ctx.r[10].s64 + 27308;
	// 832A2310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2314: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A231C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2320: 386AB488  addi r3, r10, -0x4b78
	ctx.r[3].s64 = ctx.r[10].s64 + -19320;
	// 832A2324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A232C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A233C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A2340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2344: 4BAB38FD  bl 0x82d55c40
	ctx.lr = 0x832A2348;
	sub_82D55C40(ctx, base);
	// 832A2348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A234C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2358 size=112
    let mut pc: u32 = 0x832A2358;
    'dispatch: loop {
        match pc {
            0x832A2358 => {
    //   block [0x832A2358..0x832A23C8)
	// 832A2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2364: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2368: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A236C: 38AA8540  addi r5, r10, -0x7ac0
	ctx.r[5].s64 = ctx.r[10].s64 + -31424;
	// 832A2370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2374: 390B6B20  addi r8, r11, 0x6b20
	ctx.r[8].s64 = ctx.r[11].s64 + 27424;
	// 832A2378: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 832A237C: 388A6C04  addi r4, r10, 0x6c04
	ctx.r[4].s64 = ctx.r[10].s64 + 27652;
	// 832A2380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2384: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A238C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2390: 386AB4B8  addi r3, r10, -0x4b48
	ctx.r[3].s64 = ctx.r[10].s64 + -19272;
	// 832A2394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A239C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A23A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A23A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A23A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A23AC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A23B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A23B4: 4BAB388D  bl 0x82d55c40
	ctx.lr = 0x832A23B8;
	sub_82D55C40(ctx, base);
	// 832A23B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A23BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A23C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A23C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A23C8 size=108
    let mut pc: u32 = 0x832A23C8;
    'dispatch: loop {
        match pc {
            0x832A23C8 => {
    //   block [0x832A23C8..0x832A2434)
	// 832A23C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A23CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A23D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A23D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A23D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A23DC: 396B6D60  addi r11, r11, 0x6d60
	ctx.r[11].s64 = ctx.r[11].s64 + 28000;
	// 832A23E0: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 832A23E4: 390B01C8  addi r8, r11, 0x1c8
	ctx.r[8].s64 = ctx.r[11].s64 + 456;
	// 832A23E8: 388A6F7C  addi r4, r10, 0x6f7c
	ctx.r[4].s64 = ctx.r[10].s64 + 28540;
	// 832A23EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A23F0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A23F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A23F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A23FC: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 832A2400: 386AB4E8  addi r3, r10, -0x4b18
	ctx.r[3].s64 = ctx.r[10].s64 + -19224;
	// 832A2404: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 832A2408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A240C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2410: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 832A2414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2418: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 832A241C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A2420: 4BAB3821  bl 0x82d55c40
	ctx.lr = 0x832A2424;
	sub_82D55C40(ctx, base);
	// 832A2424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A2428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A242C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2438 size=112
    let mut pc: u32 = 0x832A2438;
    'dispatch: loop {
        match pc {
            0x832A2438 => {
    //   block [0x832A2438..0x832A24A8)
	// 832A2438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2444: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A244C: 38AA8540  addi r5, r10, -0x7ac0
	ctx.r[5].s64 = ctx.r[10].s64 + -31424;
	// 832A2450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2454: 390B6F98  addi r8, r11, 0x6f98
	ctx.r[8].s64 = ctx.r[11].s64 + 28568;
	// 832A2458: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A245C: 388A7004  addi r4, r10, 0x7004
	ctx.r[4].s64 = ctx.r[10].s64 + 28676;
	// 832A2460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2464: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A246C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2470: 386AB518  addi r3, r10, -0x4ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -19176;
	// 832A2474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A247C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A248C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A2490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2494: 4BAB37AD  bl 0x82d55c40
	ctx.lr = 0x832A2498;
	sub_82D55C40(ctx, base);
	// 832A2498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A249C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A24A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A24A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A24A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A24A8 size=112
    let mut pc: u32 = 0x832A24A8;
    'dispatch: loop {
        match pc {
            0x832A24A8 => {
    //   block [0x832A24A8..0x832A2518)
	// 832A24A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A24AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A24B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A24B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A24B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A24BC: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A24C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A24C4: 390B7028  addi r8, r11, 0x7028
	ctx.r[8].s64 = ctx.r[11].s64 + 28712;
	// 832A24C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A24CC: 388A70A8  addi r4, r10, 0x70a8
	ctx.r[4].s64 = ctx.r[10].s64 + 28840;
	// 832A24D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A24D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A24D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A24DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A24E0: 386AB548  addi r3, r10, -0x4ab8
	ctx.r[3].s64 = ctx.r[10].s64 + -19128;
	// 832A24E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A24E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A24EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A24F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A24F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A24F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A24FC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A2500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2504: 4BAB373D  bl 0x82d55c40
	ctx.lr = 0x832A2508;
	sub_82D55C40(ctx, base);
	// 832A2508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A250C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2518 size=112
    let mut pc: u32 = 0x832A2518;
    'dispatch: loop {
        match pc {
            0x832A2518 => {
    //   block [0x832A2518..0x832A2588)
	// 832A2518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A251C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2524: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2528: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A252C: 38AA8540  addi r5, r10, -0x7ac0
	ctx.r[5].s64 = ctx.r[10].s64 + -31424;
	// 832A2530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2534: 390B70D0  addi r8, r11, 0x70d0
	ctx.r[8].s64 = ctx.r[11].s64 + 28880;
	// 832A2538: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A253C: 388A7154  addi r4, r10, 0x7154
	ctx.r[4].s64 = ctx.r[10].s64 + 29012;
	// 832A2540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2544: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A254C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2550: 386AB578  addi r3, r10, -0x4a88
	ctx.r[3].s64 = ctx.r[10].s64 + -19080;
	// 832A2554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A255C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A256C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A2570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2574: 4BAB36CD  bl 0x82d55c40
	ctx.lr = 0x832A2578;
	sub_82D55C40(ctx, base);
	// 832A2578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A257C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2588 size=56
    let mut pc: u32 = 0x832A2588;
    'dispatch: loop {
        match pc {
            0x832A2588 => {
    //   block [0x832A2588..0x832A25C0)
	// 832A2588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A258C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2594: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A2598: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 832A259C: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 832A25A0: 386000C8  li r3, 0xc8
	ctx.r[3].s64 = 200;
	// 832A25A4: 4BE4B6DD  bl 0x830edc80
	ctx.lr = 0x832A25A8;
	sub_830EDC80(ctx, base);
	// 832A25A8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A25AC: 906BB5C0  stw r3, -0x4a40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19008 as u32), ctx.r[3].u32 ) };
	// 832A25B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A25B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A25B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A25BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A25C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A25C0 size=56
    let mut pc: u32 = 0x832A25C0;
    'dispatch: loop {
        match pc {
            0x832A25C0 => {
    //   block [0x832A25C0..0x832A25F8)
	// 832A25C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A25C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A25C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A25CC: 38C00096  li r6, 0x96
	ctx.r[6].s64 = 150;
	// 832A25D0: 38A000C8  li r5, 0xc8
	ctx.r[5].s64 = 200;
	// 832A25D4: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 832A25D8: 386000C8  li r3, 0xc8
	ctx.r[3].s64 = 200;
	// 832A25DC: 4BE4B6A5  bl 0x830edc80
	ctx.lr = 0x832A25E0;
	sub_830EDC80(ctx, base);
	// 832A25E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A25E4: 906BB5BC  stw r3, -0x4a44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19012 as u32), ctx.r[3].u32 ) };
	// 832A25E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A25EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A25F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A25F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A25F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A25F8 size=56
    let mut pc: u32 = 0x832A25F8;
    'dispatch: loop {
        match pc {
            0x832A25F8 => {
    //   block [0x832A25F8..0x832A2630)
	// 832A25F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A25FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2604: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 832A2608: 38A000F0  li r5, 0xf0
	ctx.r[5].s64 = 240;
	// 832A260C: 388000F0  li r4, 0xf0
	ctx.r[4].s64 = 240;
	// 832A2610: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 832A2614: 4BE4B66D  bl 0x830edc80
	ctx.lr = 0x832A2618;
	sub_830EDC80(ctx, base);
	// 832A2618: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A261C: 906BB5CC  stw r3, -0x4a34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18996 as u32), ctx.r[3].u32 ) };
	// 832A2620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A262C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2630 size=56
    let mut pc: u32 = 0x832A2630;
    'dispatch: loop {
        match pc {
            0x832A2630 => {
    //   block [0x832A2630..0x832A2668)
	// 832A2630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A263C: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 832A2640: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 832A2644: 38800078  li r4, 0x78
	ctx.r[4].s64 = 120;
	// 832A2648: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 832A264C: 4BE4B635  bl 0x830edc80
	ctx.lr = 0x832A2650;
	sub_830EDC80(ctx, base);
	// 832A2650: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A2654: 906BB5D0  stw r3, -0x4a30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18992 as u32), ctx.r[3].u32 ) };
	// 832A2658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2668 size=56
    let mut pc: u32 = 0x832A2668;
    'dispatch: loop {
        match pc {
            0x832A2668 => {
    //   block [0x832A2668..0x832A26A0)
	// 832A2668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A266C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2674: 38C000C8  li r6, 0xc8
	ctx.r[6].s64 = 200;
	// 832A2678: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A267C: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 832A2680: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 832A2684: 4BE4B5FD  bl 0x830edc80
	ctx.lr = 0x832A2688;
	sub_830EDC80(ctx, base);
	// 832A2688: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A268C: 906BB5DC  stw r3, -0x4a24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18980 as u32), ctx.r[3].u32 ) };
	// 832A2690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A269C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A26A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A26A0 size=28
    let mut pc: u32 = 0x832A26A0;
    'dispatch: loop {
        match pc {
            0x832A26A0 => {
    //   block [0x832A26A0..0x832A26BC)
	// 832A26A0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A26A4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A26A8: 396BD42C  addi r11, r11, -0x2bd4
	ctx.r[11].s64 = ctx.r[11].s64 + -11220;
	// 832A26AC: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A26B0: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A26B4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A26B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A26C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A26C0 size=56
    let mut pc: u32 = 0x832A26C0;
    'dispatch: loop {
        match pc {
            0x832A26C0 => {
    //   block [0x832A26C0..0x832A26F8)
	// 832A26C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A26C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A26C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A26CC: 38C0008C  li r6, 0x8c
	ctx.r[6].s64 = 140;
	// 832A26D0: 38A0008C  li r5, 0x8c
	ctx.r[5].s64 = 140;
	// 832A26D4: 388000F0  li r4, 0xf0
	ctx.r[4].s64 = 240;
	// 832A26D8: 3860008C  li r3, 0x8c
	ctx.r[3].s64 = 140;
	// 832A26DC: 4BE4B5A5  bl 0x830edc80
	ctx.lr = 0x832A26E0;
	sub_830EDC80(ctx, base);
	// 832A26E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A26E4: 906BB5F8  stw r3, -0x4a08(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18952 as u32), ctx.r[3].u32 ) };
	// 832A26E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A26EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A26F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A26F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A26F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A26F8 size=56
    let mut pc: u32 = 0x832A26F8;
    'dispatch: loop {
        match pc {
            0x832A26F8 => {
    //   block [0x832A26F8..0x832A2730)
	// 832A26F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A26FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2704: 38C0008C  li r6, 0x8c
	ctx.r[6].s64 = 140;
	// 832A2708: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 832A270C: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 832A2710: 38600046  li r3, 0x46
	ctx.r[3].s64 = 70;
	// 832A2714: 4BE4B56D  bl 0x830edc80
	ctx.lr = 0x832A2718;
	sub_830EDC80(ctx, base);
	// 832A2718: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A271C: 906BB5FC  stw r3, -0x4a04(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18948 as u32), ctx.r[3].u32 ) };
	// 832A2720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A272C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A2730 size=28
    let mut pc: u32 = 0x832A2730;
    'dispatch: loop {
        match pc {
            0x832A2730 => {
    //   block [0x832A2730..0x832A274C)
	// 832A2730: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A2734: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A2738: 396BD69C  addi r11, r11, -0x2964
	ctx.r[11].s64 = ctx.r[11].s64 + -10596;
	// 832A273C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A2740: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A2744: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A2748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A2750 size=28
    let mut pc: u32 = 0x832A2750;
    'dispatch: loop {
        match pc {
            0x832A2750 => {
    //   block [0x832A2750..0x832A276C)
	// 832A2750: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A2754: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A2758: 396BD738  addi r11, r11, -0x28c8
	ctx.r[11].s64 = ctx.r[11].s64 + -10440;
	// 832A275C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A2760: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A2764: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A2768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2770 size=108
    let mut pc: u32 = 0x832A2770;
    'dispatch: loop {
        match pc {
            0x832A2770 => {
    //   block [0x832A2770..0x832A27DC)
	// 832A2770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A277C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A2780: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2784: 396B84A0  addi r11, r11, -0x7b60
	ctx.r[11].s64 = ctx.r[11].s64 + -31584;
	// 832A2788: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 832A278C: 390B0120  addi r8, r11, 0x120
	ctx.r[8].s64 = ctx.r[11].s64 + 288;
	// 832A2790: 388A861C  addi r4, r10, -0x79e4
	ctx.r[4].s64 = ctx.r[10].s64 + -31204;
	// 832A2794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2798: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A279C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A27A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A27A4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A27A8: 386AB618  addi r3, r10, -0x49e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18920;
	// 832A27AC: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 832A27B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A27B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A27B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 832A27BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A27C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 832A27C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A27C8: 4BAB3479  bl 0x82d55c40
	ctx.lr = 0x832A27CC;
	sub_82D55C40(ctx, base);
	// 832A27CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A27D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A27D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A27D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A27E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A27E0 size=96
    let mut pc: u32 = 0x832A27E0;
    'dispatch: loop {
        match pc {
            0x832A27E0 => {
    //   block [0x832A27E0..0x832A2840)
	// 832A27E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A27E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A27E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A27EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A27F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A27F4: 388A8644  addi r4, r10, -0x79bc
	ctx.r[4].s64 = ctx.r[10].s64 + -31164;
	// 832A27F8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A27FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2800: 386AB648  addi r3, r10, -0x49b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18872;
	// 832A2804: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A2808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A280C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A2814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A281C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 832A2820: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A2824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A2828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A282C: 4BAB3415  bl 0x82d55c40
	ctx.lr = 0x832A2830;
	sub_82D55C40(ctx, base);
	// 832A2830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A2834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A283C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2840 size=112
    let mut pc: u32 = 0x832A2840;
    'dispatch: loop {
        match pc {
            0x832A2840 => {
    //   block [0x832A2840..0x832A28B0)
	// 832A2840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A284C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A2850: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A2854: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A2858: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A285C: 390B8678  addi r8, r11, -0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + -31112;
	// 832A2860: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A2864: 388A86EC  addi r4, r10, -0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + -30996;
	// 832A2868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A286C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A2874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2878: 386AB678  addi r3, r10, -0x4988
	ctx.r[3].s64 = ctx.r[10].s64 + -18824;
	// 832A287C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A2884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A288C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A2894: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A2898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A289C: 4BAB33A5  bl 0x82d55c40
	ctx.lr = 0x832A28A0;
	sub_82D55C40(ctx, base);
	// 832A28A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A28A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A28A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A28AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A28B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A28B0 size=72
    let mut pc: u32 = 0x832A28B0;
    'dispatch: loop {
        match pc {
            0x832A28B0 => {
    //   block [0x832A28B0..0x832A28F8)
	// 832A28B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A28B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A28B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A28BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A28C0: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A28C4: 388A8770  addi r4, r10, -0x7890
	ctx.r[4].s64 = ctx.r[10].s64 + -30864;
	// 832A28C8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A28CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A28D0: 386AB6AC  addi r3, r10, -0x4954
	ctx.r[3].s64 = ctx.r[10].s64 + -18772;
	// 832A28D4: 38ABFBF0  addi r5, r11, -0x410
	ctx.r[5].s64 = ctx.r[11].s64 + -1040;
	// 832A28D8: 4BBE9E99  bl 0x82e8c770
	ctx.lr = 0x832A28DC;
	sub_82E8C770(ctx, base);
	// 832A28DC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A28E0: 386B7F00  addi r3, r11, 0x7f00
	ctx.r[3].s64 = ctx.r[11].s64 + 32512;
	// 832A28E4: 4BA0763D  bl 0x82ca9f20
	ctx.lr = 0x832A28E8;
	sub_82CA9F20(ctx, base);
	// 832A28E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A28EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A28F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A28F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A28F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A28F8 size=80
    let mut pc: u32 = 0x832A28F8;
    'dispatch: loop {
        match pc {
            0x832A28F8 => {
    //   block [0x832A28F8..0x832A2948)
	// 832A28F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A28FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2904: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2908: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A290C: 388A87F8  addi r4, r10, -0x7808
	ctx.r[4].s64 = ctx.r[10].s64 + -30728;
	// 832A2910: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2918: 386AB7C0  addi r3, r10, -0x4840
	ctx.r[3].s64 = ctx.r[10].s64 + -18496;
	// 832A291C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2920: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2924: 38AB0C10  addi r5, r11, 0xc10
	ctx.r[5].s64 = ctx.r[11].s64 + 3088;
	// 832A2928: 4BBE6D79  bl 0x82e896a0
	ctx.lr = 0x832A292C;
	sub_82E896A0(ctx, base);
	// 832A292C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2930: 386B7F18  addi r3, r11, 0x7f18
	ctx.r[3].s64 = ctx.r[11].s64 + 32536;
	// 832A2934: 4BA075ED  bl 0x82ca9f20
	ctx.lr = 0x832A2938;
	sub_82CA9F20(ctx, base);
	// 832A2938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A293C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2948 size=80
    let mut pc: u32 = 0x832A2948;
    'dispatch: loop {
        match pc {
            0x832A2948 => {
    //   block [0x832A2948..0x832A2998)
	// 832A2948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2954: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2958: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A295C: 388A88C4  addi r4, r10, -0x773c
	ctx.r[4].s64 = ctx.r[10].s64 + -30524;
	// 832A2960: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2968: 386AB8D4  addi r3, r10, -0x472c
	ctx.r[3].s64 = ctx.r[10].s64 + -18220;
	// 832A296C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2970: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2974: 38AB1578  addi r5, r11, 0x1578
	ctx.r[5].s64 = ctx.r[11].s64 + 5496;
	// 832A2978: 4BBE6D29  bl 0x82e896a0
	ctx.lr = 0x832A297C;
	sub_82E896A0(ctx, base);
	// 832A297C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2980: 386B7F30  addi r3, r11, 0x7f30
	ctx.r[3].s64 = ctx.r[11].s64 + 32560;
	// 832A2984: 4BA0759D  bl 0x82ca9f20
	ctx.lr = 0x832A2988;
	sub_82CA9F20(ctx, base);
	// 832A2988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A298C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


