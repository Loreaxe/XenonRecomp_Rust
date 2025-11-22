pub fn sub_828237B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828237B0 size=12
    let mut pc: u32 = 0x828237B0;
    'dispatch: loop {
        match pc {
            0x828237B0 => {
    //   block [0x828237B0..0x828237BC)
	// 828237B0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828237B4: 386BAE7C  addi r3, r11, -0x5184
	ctx.r[3].s64 = ctx.r[11].s64 + -20868;
	// 828237B8: 4BFF8780  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828237C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828237C0 size=12
    let mut pc: u32 = 0x828237C0;
    'dispatch: loop {
        match pc {
            0x828237C0 => {
    //   block [0x828237C0..0x828237CC)
	// 828237C0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828237C4: 386BB000  addi r3, r11, -0x5000
	ctx.r[3].s64 = ctx.r[11].s64 + -20480;
	// 828237C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828237D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828237D0 size=12
    let mut pc: u32 = 0x828237D0;
    'dispatch: loop {
        match pc {
            0x828237D0 => {
    //   block [0x828237D0..0x828237DC)
	// 828237D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828237D4: 386BAFE4  addi r3, r11, -0x501c
	ctx.r[3].s64 = ctx.r[11].s64 + -20508;
	// 828237D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828237E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828237E0 size=12
    let mut pc: u32 = 0x828237E0;
    'dispatch: loop {
        match pc {
            0x828237E0 => {
    //   block [0x828237E0..0x828237EC)
	// 828237E0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828237E4: 386BB010  addi r3, r11, -0x4ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -20464;
	// 828237E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828237F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828237F0 size=88
    let mut pc: u32 = 0x828237F0;
    'dispatch: loop {
        match pc {
            0x828237F0 => {
    //   block [0x828237F0..0x82823848)
	// 828237F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828237F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828237F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828237FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82823808: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8282380C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82823810: 394AFF20  addi r10, r10, -0xe0
	ctx.r[10].s64 = ctx.r[10].s64 + -224;
	// 82823814: 817F0304  lwz r11, 0x304(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(772 as u32) ) } as u64;
	// 82823818: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8282381C: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82823820: 4BFF5489  bl 0x82818ca8
	ctx.lr = 0x82823824;
	sub_82818CA8(ctx, base);
	// 82823824: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82823828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282382C: 4BFF8A55  bl 0x8281c280
	ctx.lr = 0x82823830;
	sub_8281C280(ctx, base);
	// 82823830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82823834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282383C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82823840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823848 size=136
    let mut pc: u32 = 0x82823848;
    'dispatch: loop {
        match pc {
            0x82823848 => {
    //   block [0x82823848..0x828238D0)
	// 82823848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282384C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82823850: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82823854: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282385C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823860: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82823864: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82823868: 409A0020  bne cr6, 0x82823888
	if !ctx.cr[6].eq {
	pc = 0x82823888; continue 'dispatch;
	}
	// 8282386C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82823870: 419A0048  beq cr6, 0x828238b8
	if ctx.cr[6].eq {
	pc = 0x828238B8; continue 'dispatch;
	}
	// 82823874: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82823878: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8282387C: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82823880: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82823884: 48000034  b 0x828238b8
	pc = 0x828238B8; continue 'dispatch;
	// 82823888: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8282388C: 419A002C  beq cr6, 0x828238b8
	if ctx.cr[6].eq {
	pc = 0x828238B8; continue 'dispatch;
	}
	// 82823890: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82823894: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823898: 388B0150  addi r4, r11, 0x150
	ctx.r[4].s64 = ctx.r[11].s64 + 336;
	// 8282389C: 4898485D  bl 0x831a80f8
	ctx.lr = 0x828238A0;
	sub_831A80F8(ctx, base);
	// 828238A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828238A4: 4182000C  beq 0x828238b0
	if ctx.cr[0].eq {
	pc = 0x828238B0; continue 'dispatch;
	}
	// 828238A8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 828238AC: 4800000C  b 0x828238b8
	pc = 0x828238B8; continue 'dispatch;
	// 828238B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828238B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828238B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828238BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828238C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828238C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828238C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828238CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828238D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828238D0 size=136
    let mut pc: u32 = 0x828238D0;
    'dispatch: loop {
        match pc {
            0x828238D0 => {
    //   block [0x828238D0..0x82823958)
	// 828238D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828238D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828238D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828238DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828238E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828238E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828238E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828238EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 828238F0: 409A0020  bne cr6, 0x82823910
	if !ctx.cr[6].eq {
	pc = 0x82823910; continue 'dispatch;
	}
	// 828238F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828238F8: 419A0048  beq cr6, 0x82823940
	if ctx.cr[6].eq {
	pc = 0x82823940; continue 'dispatch;
	}
	// 828238FC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82823900: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82823904: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82823908: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8282390C: 48000034  b 0x82823940
	pc = 0x82823940; continue 'dispatch;
	// 82823910: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82823914: 419A002C  beq cr6, 0x82823940
	if ctx.cr[6].eq {
	pc = 0x82823940; continue 'dispatch;
	}
	// 82823918: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282391C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823920: 388B0220  addi r4, r11, 0x220
	ctx.r[4].s64 = ctx.r[11].s64 + 544;
	// 82823924: 489847D5  bl 0x831a80f8
	ctx.lr = 0x82823928;
	sub_831A80F8(ctx, base);
	// 82823928: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282392C: 4182000C  beq 0x82823938
	if ctx.cr[0].eq {
	pc = 0x82823938; continue 'dispatch;
	}
	// 82823930: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82823934: 4800000C  b 0x82823940
	pc = 0x82823940; continue 'dispatch;
	// 82823938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8282393C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82823944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282394C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82823950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823958 size=132
    let mut pc: u32 = 0x82823958;
    'dispatch: loop {
        match pc {
            0x82823958 => {
    //   block [0x82823958..0x828239DC)
	// 82823958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282395C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82823960: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82823964: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282396C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82823970: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82823974: 807F0104  lwz r3, 0x104(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 82823978: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282397C: 419A0018  beq cr6, 0x82823994
	if ctx.cr[6].eq {
	pc = 0x82823994; continue 'dispatch;
	}
	// 82823980: 817F00FC  lwz r11, 0xfc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82823984: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82823988: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8282398C: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82823990: 4BFFFC29  bl 0x828235b8
	ctx.lr = 0x82823994;
	sub_828235B8(ctx, base);
	// 82823994: 807F010C  lwz r3, 0x10c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 82823998: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282399C: 419A001C  beq cr6, 0x828239b8
	if ctx.cr[6].eq {
	pc = 0x828239B8; continue 'dispatch;
	}
	// 828239A0: 817F00FC  lwz r11, 0xfc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 828239A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828239A8: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 828239AC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 828239B0: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 828239B4: 4BFFFC65  bl 0x82823618
	ctx.lr = 0x828239B8;
	sub_82823618(ctx, base);
	// 828239B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828239BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828239C0: 4BFF8749  bl 0x8281c108
	ctx.lr = 0x828239C4;
	sub_8281C108(ctx, base);
	// 828239C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828239C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828239CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828239D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828239D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828239D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828239E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828239E0 size=72
    let mut pc: u32 = 0x828239E0;
    'dispatch: loop {
        match pc {
            0x828239E0 => {
    //   block [0x828239E0..0x82823A28)
	// 828239E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828239E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828239E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828239EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828239F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828239F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828239F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828239FC: 807F02F8  lwz r3, 0x2f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) } as u64;
	// 82823A00: 481590F9  bl 0x8297caf8
	ctx.lr = 0x82823A04;
	sub_8297CAF8(ctx, base);
	// 82823A04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82823A08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823A0C: 4BFFA625  bl 0x8281e030
	ctx.lr = 0x82823A10;
	sub_8281E030(ctx, base);
	// 82823A10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82823A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82823A1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82823A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823A28 size=88
    let mut pc: u32 = 0x82823A28;
    'dispatch: loop {
        match pc {
            0x82823A28 => {
    //   block [0x82823A28..0x82823A80)
	// 82823A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823A2C: 48984741  bl 0x831a816c
	ctx.lr = 0x82823A30;
	sub_831A8130(ctx, base);
	// 82823A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823A34: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82823A38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823A3C: 38840018  addi r4, r4, 0x18
	ctx.r[4].s64 = ctx.r[4].s64 + 24;
	// 82823A40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823A44: 83EBAFE0  lwz r31, -0x5020(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20512 as u32) ) } as u64;
	// 82823A48: 485D5A09  bl 0x82df9450
	ctx.lr = 0x82823A4C;
	sub_82DF9450(ctx, base);
	// 82823A4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82823A50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82823A54: 485D59B5  bl 0x82df9408
	ctx.lr = 0x82823A58;
	sub_82DF9408(ctx, base);
	// 82823A58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82823A5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823A60: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823A64: 485CF9C5  bl 0x82df3428
	ctx.lr = 0x82823A68;
	sub_82DF3428(ctx, base);
	// 82823A68: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82823A6C: 409A000C  bne cr6, 0x82823a78
	if !ctx.cr[6].eq {
	pc = 0x82823A78; continue 'dispatch;
	}
	// 82823A70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82823A74: 997E0308  stb r11, 0x308(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(776 as u32), ctx.r[11].u8 ) };
	// 82823A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82823A7C: 48984740  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823A80 size=88
    let mut pc: u32 = 0x82823A80;
    'dispatch: loop {
        match pc {
            0x82823A80 => {
    //   block [0x82823A80..0x82823AD8)
	// 82823A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823A84: 489846E9  bl 0x831a816c
	ctx.lr = 0x82823A88;
	sub_831A8130(ctx, base);
	// 82823A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823A8C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82823A90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823A94: 38840018  addi r4, r4, 0x18
	ctx.r[4].s64 = ctx.r[4].s64 + 24;
	// 82823A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823A9C: 83EBAFE0  lwz r31, -0x5020(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20512 as u32) ) } as u64;
	// 82823AA0: 485D59B1  bl 0x82df9450
	ctx.lr = 0x82823AA4;
	sub_82DF9450(ctx, base);
	// 82823AA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82823AA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82823AAC: 485D595D  bl 0x82df9408
	ctx.lr = 0x82823AB0;
	sub_82DF9408(ctx, base);
	// 82823AB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82823AB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823AB8: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823ABC: 485CF96D  bl 0x82df3428
	ctx.lr = 0x82823AC0;
	sub_82DF3428(ctx, base);
	// 82823AC0: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82823AC4: 409A000C  bne cr6, 0x82823ad0
	if !ctx.cr[6].eq {
	pc = 0x82823AD0; continue 'dispatch;
	}
	// 82823AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823ACC: 997E0308  stb r11, 0x308(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(776 as u32), ctx.r[11].u8 ) };
	// 82823AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82823AD4: 489846E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823AD8 size=196
    let mut pc: u32 = 0x82823AD8;
    'dispatch: loop {
        match pc {
            0x82823AD8 => {
    //   block [0x82823AD8..0x82823B9C)
	// 82823AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82823AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82823AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823AEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823AF4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82823AF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82823AFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823B00: 4BA9CE39  bl 0x822c0938
	ctx.lr = 0x82823B04;
	sub_822C0938(ctx, base);
	// 82823B04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82823B08: 41820028  beq 0x82823b30
	if ctx.cr[0].eq {
	pc = 0x82823B30; continue 'dispatch;
	}
	// 82823B0C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82823B10: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82823B14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82823B18: 392BA430  addi r9, r11, -0x5bd0
	ctx.r[9].s64 = ctx.r[11].s64 + -23504;
	// 82823B1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82823B20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82823B24: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82823B28: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82823B2C: 48000008  b 0x82823b34
	pc = 0x82823B34; continue 'dispatch;
	// 82823B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823B34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823B38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82823B3C: 409A0044  bne cr6, 0x82823b80
	if !ctx.cr[6].eq {
	pc = 0x82823B80; continue 'dispatch;
	}
	// 82823B40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82823B44: 419A001C  beq cr6, 0x82823b60
	if ctx.cr[6].eq {
	pc = 0x82823B60; continue 'dispatch;
	}
	// 82823B48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823B4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82823B50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823B54: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823B58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823B5C: 4E800421  bctrl
	ctx.lr = 0x82823B60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823B60: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82823B64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82823B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823B6C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82823B70: 816BFF2C  lwz r11, -0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82823B74: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82823B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82823B7C: 4BA9C485  bl 0x822c0000
	ctx.lr = 0x82823B80;
	sub_822C0000(ctx, base);
	// 82823B80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82823B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82823B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82823B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82823B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823BA0 size=196
    let mut pc: u32 = 0x82823BA0;
    'dispatch: loop {
        match pc {
            0x82823BA0 => {
    //   block [0x82823BA0..0x82823C64)
	// 82823BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82823BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82823BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823BB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823BBC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82823BC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82823BC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823BC8: 4BA9CD71  bl 0x822c0938
	ctx.lr = 0x82823BCC;
	sub_822C0938(ctx, base);
	// 82823BCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82823BD0: 41820028  beq 0x82823bf8
	if ctx.cr[0].eq {
	pc = 0x82823BF8; continue 'dispatch;
	}
	// 82823BD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82823BD8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82823BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82823BE0: 392BA444  addi r9, r11, -0x5bbc
	ctx.r[9].s64 = ctx.r[11].s64 + -23484;
	// 82823BE4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82823BE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82823BEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82823BF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82823BF4: 48000008  b 0x82823bfc
	pc = 0x82823BFC; continue 'dispatch;
	// 82823BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823BFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82823C04: 409A0044  bne cr6, 0x82823c48
	if !ctx.cr[6].eq {
	pc = 0x82823C48; continue 'dispatch;
	}
	// 82823C08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82823C0C: 419A001C  beq cr6, 0x82823c28
	if ctx.cr[6].eq {
	pc = 0x82823C28; continue 'dispatch;
	}
	// 82823C10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823C14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82823C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823C1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823C24: 4E800421  bctrl
	ctx.lr = 0x82823C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823C28: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82823C2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82823C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823C34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82823C38: 816BFF2C  lwz r11, -0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82823C3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82823C40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82823C44: 4BA9C3BD  bl 0x822c0000
	ctx.lr = 0x82823C48;
	sub_822C0000(ctx, base);
	// 82823C48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82823C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82823C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82823C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82823C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823C68 size=196
    let mut pc: u32 = 0x82823C68;
    'dispatch: loop {
        match pc {
            0x82823C68 => {
    //   block [0x82823C68..0x82823D2C)
	// 82823C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82823C70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82823C74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823C78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823C7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823C84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82823C88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82823C8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823C90: 4BA9CCA9  bl 0x822c0938
	ctx.lr = 0x82823C94;
	sub_822C0938(ctx, base);
	// 82823C94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82823C98: 41820028  beq 0x82823cc0
	if ctx.cr[0].eq {
	pc = 0x82823CC0; continue 'dispatch;
	}
	// 82823C9C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82823CA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82823CA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82823CA8: 392BA458  addi r9, r11, -0x5ba8
	ctx.r[9].s64 = ctx.r[11].s64 + -23464;
	// 82823CAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82823CB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82823CB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82823CB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82823CBC: 48000008  b 0x82823cc4
	pc = 0x82823CC4; continue 'dispatch;
	// 82823CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823CC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82823CCC: 409A0044  bne cr6, 0x82823d10
	if !ctx.cr[6].eq {
	pc = 0x82823D10; continue 'dispatch;
	}
	// 82823CD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82823CD4: 419A001C  beq cr6, 0x82823cf0
	if ctx.cr[6].eq {
	pc = 0x82823CF0; continue 'dispatch;
	}
	// 82823CD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823CDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82823CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823CE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823CE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823CEC: 4E800421  bctrl
	ctx.lr = 0x82823CF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823CF0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82823CF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82823CF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823CFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82823D00: 816BFF2C  lwz r11, -0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82823D04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82823D08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82823D0C: 4BA9C2F5  bl 0x822c0000
	ctx.lr = 0x82823D10;
	sub_822C0000(ctx, base);
	// 82823D10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82823D14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82823D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82823D20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82823D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823D30 size=196
    let mut pc: u32 = 0x82823D30;
    'dispatch: loop {
        match pc {
            0x82823D30 => {
    //   block [0x82823D30..0x82823DF4)
	// 82823D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82823D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82823D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823D44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823D4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82823D50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82823D54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823D58: 4BA9CBE1  bl 0x822c0938
	ctx.lr = 0x82823D5C;
	sub_822C0938(ctx, base);
	// 82823D5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82823D60: 41820028  beq 0x82823d88
	if ctx.cr[0].eq {
	pc = 0x82823D88; continue 'dispatch;
	}
	// 82823D64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82823D68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82823D6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82823D70: 392BA46C  addi r9, r11, -0x5b94
	ctx.r[9].s64 = ctx.r[11].s64 + -23444;
	// 82823D74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82823D78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82823D7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82823D80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82823D84: 48000008  b 0x82823d8c
	pc = 0x82823D8C; continue 'dispatch;
	// 82823D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82823D8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823D90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82823D94: 409A0044  bne cr6, 0x82823dd8
	if !ctx.cr[6].eq {
	pc = 0x82823DD8; continue 'dispatch;
	}
	// 82823D98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82823D9C: 419A001C  beq cr6, 0x82823db8
	if ctx.cr[6].eq {
	pc = 0x82823DB8; continue 'dispatch;
	}
	// 82823DA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823DA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82823DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823DAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823DB4: 4E800421  bctrl
	ctx.lr = 0x82823DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823DB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82823DBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82823DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82823DC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82823DC8: 816BFF2C  lwz r11, -0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82823DCC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82823DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82823DD4: 4BA9C22D  bl 0x822c0000
	ctx.lr = 0x82823DD8;
	sub_822C0000(ctx, base);
	// 82823DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82823DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82823DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82823DE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82823DEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82823DF8 size=228
    let mut pc: u32 = 0x82823DF8;
    'dispatch: loop {
        match pc {
            0x82823DF8 => {
    //   block [0x82823DF8..0x82823EDC)
	// 82823DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82823E00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82823E04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823E08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82823E0C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82823E10: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82823E14: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82823E18: 396BA4E4  addi r11, r11, -0x5b1c
	ctx.r[11].s64 = ctx.r[11].s64 + -23324;
	// 82823E1C: 394AA4D0  addi r10, r10, -0x5b30
	ctx.r[10].s64 = ctx.r[10].s64 + -23344;
	// 82823E20: 3929A484  addi r9, r9, -0x5b7c
	ctx.r[9].s64 = ctx.r[9].s64 + -23420;
	// 82823E24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82823E28: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82823E2C: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82823E30: 807F0318  lwz r3, 0x318(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(792 as u32) ) } as u64;
	// 82823E34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82823E38: 419A0008  beq cr6, 0x82823e40
	if ctx.cr[6].eq {
	pc = 0x82823E40; continue 'dispatch;
	}
	// 82823E3C: 4BA9CA55  bl 0x822c0890
	ctx.lr = 0x82823E40;
	sub_822C0890(ctx, base);
	// 82823E40: 807F0310  lwz r3, 0x310(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(784 as u32) ) } as u64;
	// 82823E44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82823E48: 419A0008  beq cr6, 0x82823e50
	if ctx.cr[6].eq {
	pc = 0x82823E50; continue 'dispatch;
	}
	// 82823E4C: 4BA9CA45  bl 0x822c0890
	ctx.lr = 0x82823E50;
	sub_822C0890(ctx, base);
	// 82823E50: 807F02FC  lwz r3, 0x2fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(764 as u32) ) } as u64;
	// 82823E54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82823E58: 419A0018  beq cr6, 0x82823e70
	if ctx.cr[6].eq {
	pc = 0x82823E70; continue 'dispatch;
	}
	// 82823E5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823E60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82823E64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823E68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823E6C: 4E800421  bctrl
	ctx.lr = 0x82823E70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823E70: 807F02F8  lwz r3, 0x2f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) } as u64;
	// 82823E74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82823E78: 419A0018  beq cr6, 0x82823e90
	if ctx.cr[6].eq {
	pc = 0x82823E90; continue 'dispatch;
	}
	// 82823E7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823E80: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82823E84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823E88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823E8C: 4E800421  bctrl
	ctx.lr = 0x82823E90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823E90: 807F02F4  lwz r3, 0x2f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(756 as u32) ) } as u64;
	// 82823E94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82823E98: 419A0008  beq cr6, 0x82823ea0
	if ctx.cr[6].eq {
	pc = 0x82823EA0; continue 'dispatch;
	}
	// 82823E9C: 4BA9C9F5  bl 0x822c0890
	ctx.lr = 0x82823EA0;
	sub_822C0890(ctx, base);
	// 82823EA0: 807F02EC  lwz r3, 0x2ec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 82823EA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82823EA8: 419A0008  beq cr6, 0x82823eb0
	if ctx.cr[6].eq {
	pc = 0x82823EB0; continue 'dispatch;
	}
	// 82823EAC: 4BA9C9E5  bl 0x822c0890
	ctx.lr = 0x82823EB0;
	sub_822C0890(ctx, base);
	// 82823EB0: 807F02E4  lwz r3, 0x2e4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 82823EB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82823EB8: 419A0008  beq cr6, 0x82823ec0
	if ctx.cr[6].eq {
	pc = 0x82823EC0; continue 'dispatch;
	}
	// 82823EBC: 4BA9C9D5  bl 0x822c0890
	ctx.lr = 0x82823EC0;
	sub_822C0890(ctx, base);
	// 82823EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823EC4: 4BFFB2AD  bl 0x8281f170
	ctx.lr = 0x82823EC8;
	sub_8281F170(ctx, base);
	// 82823EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82823ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82823ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82823ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82823ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82823EE0 size=8
    let mut pc: u32 = 0x82823EE0;
    'dispatch: loop {
        match pc {
            0x82823EE0 => {
    //   block [0x82823EE0..0x82823EE8)
	// 82823EE0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82823EE4: 48000274  b 0x82824158
	sub_82824158(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82823EE8 size=8
    let mut pc: u32 = 0x82823EE8;
    'dispatch: loop {
        match pc {
            0x82823EE8 => {
    //   block [0x82823EE8..0x82823EF0)
	// 82823EE8: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82823EEC: 4800026C  b 0x82824158
	sub_82824158(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82823EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82823EF0 size=456
    let mut pc: u32 = 0x82823EF0;
    'dispatch: loop {
        match pc {
            0x82823EF0 => {
    //   block [0x82823EF0..0x828240B8)
	// 82823EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82823EF4: 48984275  bl 0x831a8168
	ctx.lr = 0x82823EF8;
	sub_831A8130(ctx, base);
	// 82823EF8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82823EFC: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82823F00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82823F04: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82823F08: 807F02F0  lwz r3, 0x2f0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(752 as u32) ) } as u64;
	// 82823F0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823F10: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82823F14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823F18: 4E800421  bctrl
	ctx.lr = 0x82823F1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823F1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823F20: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82823F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823F28: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82823F2C: C3EA0034  lfs f31, 0x34(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82823F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823F34: 4E800421  bctrl
	ctx.lr = 0x82823F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823F38: 4BFC98F9  bl 0x827ed830
	ctx.lr = 0x82823F3C;
	sub_827ED830(ctx, base);
	// 82823F3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823F40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82823F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823F48: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82823F4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823F50: 4E800421  bctrl
	ctx.lr = 0x82823F54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823F54: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82823F58: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82823F5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82823F60: 4BFCAA89  bl 0x827ee9e8
	ctx.lr = 0x82823F64;
	sub_827EE9E8(ctx, base);
	// 82823F64: C0010094  lfs f0, 0x94(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82823F68: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82823F6C: 4099001C  ble cr6, 0x82823f88
	if !ctx.cr[6].gt {
	pc = 0x82823F88; continue 'dispatch;
	}
	// 82823F70: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82823F74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823F78: 4BFF81F9  bl 0x8281c170
	ctx.lr = 0x82823F7C;
	sub_8281C170(ctx, base);
	// 82823F7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82823F80: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82823F84: 41820008  beq 0x82823f8c
	if ctx.cr[0].eq {
	pc = 0x82823F8C; continue 'dispatch;
	}
	// 82823F88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82823F8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823F90: 3B810090  addi r28, r1, 0x90
	ctx.r[28].s64 = ctx.r[1].s64 + 144;
	// 82823F94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823F98: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82823F9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823FA0: 4E800421  bctrl
	ctx.lr = 0x82823FA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823FA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82823FA8: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82823FAC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82823FB0: 4BFCA169  bl 0x827ee118
	ctx.lr = 0x82823FB4;
	sub_827EE118(ctx, base);
	// 82823FB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82823FB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82823FBC: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82823FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82823FC4: 4E800421  bctrl
	ctx.lr = 0x82823FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82823FC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82823FCC: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82823FD0: 4BFCB7E1  bl 0x827ef7b0
	ctx.lr = 0x82823FD4;
	sub_827EF7B0(ctx, base);
	// 82823FD4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82823FD8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82823FDC: 392100C0  addi r9, r1, 0xc0
	ctx.r[9].s64 = ctx.r[1].s64 + 192;
	// 82823FE0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82823FE4: 807F02F8  lwz r3, 0x2f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) } as u64;
	// 82823FE8: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82823FEC: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82823FF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82823FF4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82823FF8: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82823FFC: 13E05407  vcmpneb. (lvlx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828240B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828240B8 size=8
    let mut pc: u32 = 0x828240B8;
    'dispatch: loop {
        match pc {
            0x828240B8 => {
    //   block [0x828240B8..0x828240C0)
	// 828240B8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 828240BC: 4800053C  b 0x828245f8
	sub_828245F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828240C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828240C0 size=8
    let mut pc: u32 = 0x828240C0;
    'dispatch: loop {
        match pc {
            0x828240C0 => {
    //   block [0x828240C0..0x828240C8)
	// 828240C0: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 828240C4: 48000534  b 0x828245f8
	sub_828245F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828240C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828240C8 size=72
    let mut pc: u32 = 0x828240C8;
    'dispatch: loop {
        match pc {
            0x828240C8 => {
    //   block [0x828240C8..0x82824110)
	// 828240C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828240CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828240D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828240D4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 828240D8: 419A001C  beq cr6, 0x828240f4
	if ctx.cr[6].eq {
	pc = 0x828240F4; continue 'dispatch;
	}
	// 828240DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 828240E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 828240E4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 828240E8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828240EC: 4BFFF75D  bl 0x82823848
	ctx.lr = 0x828240F0;
	sub_82823848(ctx, base);
	// 828240F0: 48000010  b 0x82824100
	pc = 0x82824100; continue 'dispatch;
	// 828240F4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 828240F8: 396B0150  addi r11, r11, 0x150
	ctx.r[11].s64 = ctx.r[11].s64 + 336;
	// 828240FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82824104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824110 size=72
    let mut pc: u32 = 0x82824110;
    'dispatch: loop {
        match pc {
            0x82824110 => {
    //   block [0x82824110..0x82824158)
	// 82824110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282411C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82824120: 419A001C  beq cr6, 0x8282413c
	if ctx.cr[6].eq {
	pc = 0x8282413C; continue 'dispatch;
	}
	// 82824124: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82824128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8282412C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82824130: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82824134: 4BFFF79D  bl 0x828238d0
	ctx.lr = 0x82824138;
	sub_828238D0(ctx, base);
	// 82824138: 48000010  b 0x82824148
	pc = 0x82824148; continue 'dispatch;
	// 8282413C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82824140: 396B0220  addi r11, r11, 0x220
	ctx.r[11].s64 = ctx.r[11].s64 + 544;
	// 82824144: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282414C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824158 size=76
    let mut pc: u32 = 0x82824158;
    'dispatch: loop {
        match pc {
            0x82824158 => {
    //   block [0x82824158..0x828241A4)
	// 82824158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82824164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282416C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82824174: 4BFFFC85  bl 0x82823df8
	ctx.lr = 0x82824178;
	sub_82823DF8(ctx, base);
	// 82824178: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282417C: 4182000C  beq 0x82824188
	if ctx.cr[0].eq {
	pc = 0x82824188; continue 'dispatch;
	}
	// 82824180: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824184: 485CE255  bl 0x82df23d8
	ctx.lr = 0x82824188;
	sub_82DF23D8(ctx, base);
	// 82824188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282418C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82824190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824198: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282419C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828241A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828241A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828241A8 size=548
    let mut pc: u32 = 0x828241A8;
    'dispatch: loop {
        match pc {
            0x828241A8 => {
    //   block [0x828241A8..0x828243CC)
	// 828241A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828241AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828241B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828241B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828241B8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 828241BC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828241C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828241C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828241C8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 828241CC: 2B050002  cmplwi cr6, r5, 2
	ctx.cr[6].compare_u32(ctx.r[5].u32, 2 as u32, &mut ctx.xer);
	// 828241D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828241D4: 409A0094  bne cr6, 0x82824268
	if !ctx.cr[6].eq {
	pc = 0x82824268; continue 'dispatch;
	}
	// 828241D8: 485CF831  bl 0x82df3a08
	ctx.lr = 0x828241DC;
	sub_82DF3A08(ctx, base);
	// 828241DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828241E0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 828241E4: 4BFC83E5  bl 0x827ec5c8
	ctx.lr = 0x828241E8;
	sub_827EC5C8(ctx, base);
	// 828241E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 828241EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828241F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 828241F4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 828241F8: 485F0E39  bl 0x82e15030
	ctx.lr = 0x828241FC;
	sub_82E15030(ctx, base);
	// 828241FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82824200: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82824204: 4BFE468D  bl 0x82808890
	ctx.lr = 0x82824208;
	sub_82808890(ctx, base);
	// 82824208: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282420C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824210: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82824214: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82824218: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8282421C: 419A0024  beq cr6, 0x82824240
	if ctx.cr[6].eq {
	pc = 0x82824240; continue 'dispatch;
	}
	// 82824220: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82824224: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82824228: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282422C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82824230: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82824234: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82824238: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282423C: 4082FFE8  bne 0x82824224
	if !ctx.cr[0].eq {
	pc = 0x82824224; continue 'dispatch;
	}
	// 82824240: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82824244: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824248: 419A0008  beq cr6, 0x82824250
	if ctx.cr[6].eq {
	pc = 0x82824250; continue 'dispatch;
	}
	// 8282424C: 4BA9C645  bl 0x822c0890
	ctx.lr = 0x82824250;
	sub_822C0890(ctx, base);
	// 82824250: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82824254: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824258: 419A0008  beq cr6, 0x82824260
	if ctx.cr[6].eq {
	pc = 0x82824260; continue 'dispatch;
	}
	// 8282425C: 4BA9C635  bl 0x822c0890
	ctx.lr = 0x82824260;
	sub_822C0890(ctx, base);
	// 82824260: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82824264: 48000134  b 0x82824398
	pc = 0x82824398; continue 'dispatch;
	// 82824268: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8282426C: 409A00A0  bne cr6, 0x8282430c
	if !ctx.cr[6].eq {
	pc = 0x8282430C; continue 'dispatch;
	}
	// 82824270: 817E030C  lwz r11, 0x30c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(780 as u32) ) } as u64;
	// 82824274: C3EB0000  lfs f31, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82824278: 485CF791  bl 0x82df3a08
	ctx.lr = 0x8282427C;
	sub_82DF3A08(ctx, base);
	// 8282427C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82824280: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82824284: 4BFC8345  bl 0x827ec5c8
	ctx.lr = 0x82824288;
	sub_827EC5C8(ctx, base);
	// 82824288: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8282428C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82824290: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82824294: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82824298: 485F0D99  bl 0x82e15030
	ctx.lr = 0x8282429C;
	sub_82E15030(ctx, base);
	// 8282429C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828242A0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 828242A4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 828242A8: 4BFE4411  bl 0x828086b8
	ctx.lr = 0x828242AC;
	sub_828086B8(ctx, base);
	// 828242AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828242B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828242B4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828242B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828242BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 828242C0: 419A0024  beq cr6, 0x828242e4
	if ctx.cr[6].eq {
	pc = 0x828242E4; continue 'dispatch;
	}
	// 828242C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828242C8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828242CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828242D0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828242D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828242D8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828242DC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828242E0: 4082FFE8  bne 0x828242c8
	if !ctx.cr[0].eq {
	pc = 0x828242C8; continue 'dispatch;
	}
	// 828242E4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 828242E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828242EC: 419A0008  beq cr6, 0x828242f4
	if ctx.cr[6].eq {
	pc = 0x828242F4; continue 'dispatch;
	}
	// 828242F0: 4BA9C5A1  bl 0x822c0890
	ctx.lr = 0x828242F4;
	sub_822C0890(ctx, base);
	// 828242F4: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 828242F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828242FC: 419A0008  beq cr6, 0x82824304
	if ctx.cr[6].eq {
	pc = 0x82824304; continue 'dispatch;
	}
	// 82824300: 4BA9C591  bl 0x822c0890
	ctx.lr = 0x82824304;
	sub_822C0890(ctx, base);
	// 82824304: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82824308: 48000090  b 0x82824398
	pc = 0x82824398; continue 'dispatch;
	// 8282430C: 485CF6FD  bl 0x82df3a08
	ctx.lr = 0x82824310;
	sub_82DF3A08(ctx, base);
	// 82824310: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82824314: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82824318: 4BFC82B1  bl 0x827ec5c8
	ctx.lr = 0x8282431C;
	sub_827EC5C8(ctx, base);
	// 8282431C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82824320: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82824324: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82824328: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282432C: 485F0D05  bl 0x82e15030
	ctx.lr = 0x82824330;
	sub_82E15030(ctx, base);
	// 82824330: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82824334: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82824338: 4BFE4309  bl 0x82808640
	ctx.lr = 0x8282433C;
	sub_82808640(ctx, base);
	// 8282433C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82824340: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824344: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82824348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282434C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82824350: 419A0024  beq cr6, 0x82824374
	if ctx.cr[6].eq {
	pc = 0x82824374; continue 'dispatch;
	}
	// 82824354: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82824358: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282435C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824360: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82824364: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82824368: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282436C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824370: 4082FFE8  bne 0x82824358
	if !ctx.cr[0].eq {
	pc = 0x82824358; continue 'dispatch;
	}
	// 82824374: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82824378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282437C: 419A0008  beq cr6, 0x82824384
	if ctx.cr[6].eq {
	pc = 0x82824384; continue 'dispatch;
	}
	// 82824380: 4BA9C511  bl 0x822c0890
	ctx.lr = 0x82824384;
	sub_822C0890(ctx, base);
	// 82824384: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82824388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282438C: 419A0008  beq cr6, 0x82824394
	if ctx.cr[6].eq {
	pc = 0x82824394; continue 'dispatch;
	}
	// 82824390: 4BA9C501  bl 0x822c0890
	ctx.lr = 0x82824394;
	sub_822C0890(ctx, base);
	// 82824394: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82824398: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282439C: 419A0008  beq cr6, 0x828243a4
	if ctx.cr[6].eq {
	pc = 0x828243A4; continue 'dispatch;
	}
	// 828243A0: 4BA9C4F1  bl 0x822c0890
	ctx.lr = 0x828243A4;
	sub_822C0890(ctx, base);
	// 828243A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828243A8: 485CF081  bl 0x82df3428
	ctx.lr = 0x828243AC;
	sub_82DF3428(ctx, base);
	// 828243AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828243B0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 828243B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828243B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828243BC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 828243C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828243C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828243C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828243D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828243D0 size=140
    let mut pc: u32 = 0x828243D0;
    'dispatch: loop {
        match pc {
            0x828243D0 => {
    //   block [0x828243D0..0x8282445C)
	// 828243D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828243D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828243D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828243DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828243E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828243E4: 4813C17D  bl 0x82960560
	ctx.lr = 0x828243E8;
	sub_82960560(ctx, base);
	// 828243E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828243EC: 4182005C  beq 0x82824448
	if ctx.cr[0].eq {
	pc = 0x82824448; continue 'dispatch;
	}
	// 828243F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 828243F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828243F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828243FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82824400: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82824404: 419A0024  beq cr6, 0x82824428
	if ctx.cr[6].eq {
	pc = 0x82824428; continue 'dispatch;
	}
	// 82824408: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8282440C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82824410: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824414: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82824418: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282441C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82824420: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824424: 4082FFE8  bne 0x8282440c
	if !ctx.cr[0].eq {
	pc = 0x8282440C; continue 'dispatch;
	}
	// 82824428: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282442C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82824430: 388BA680  addi r4, r11, -0x5980
	ctx.r[4].s64 = ctx.r[11].s64 + -22912;
	// 82824434: 4BFF6085  bl 0x8281a4b8
	ctx.lr = 0x82824438;
	sub_8281A4B8(ctx, base);
	// 82824438: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282443C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824440: 419A0008  beq cr6, 0x82824448
	if ctx.cr[6].eq {
	pc = 0x82824448; continue 'dispatch;
	}
	// 82824444: 4BA9C44D  bl 0x822c0890
	ctx.lr = 0x82824448;
	sub_822C0890(ctx, base);
	// 82824448: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282444C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82824460 size=408
    let mut pc: u32 = 0x82824460;
    'dispatch: loop {
        match pc {
            0x82824460 => {
    //   block [0x82824460..0x828245F8)
	// 82824460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282446C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824470: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82824474: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82824478: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282447C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824484: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82824488: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282448C: 388BA6B4  addi r4, r11, -0x594c
	ctx.r[4].s64 = ctx.r[11].s64 + -22860;
	// 82824490: 485CF579  bl 0x82df3a08
	ctx.lr = 0x82824494;
	sub_82DF3A08(ctx, base);
	// 82824494: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82824498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282449C: 388BEDA0  addi r4, r11, -0x1260
	ctx.r[4].s64 = ctx.r[11].s64 + -4704;
	// 828244A0: 485CF569  bl 0x82df3a08
	ctx.lr = 0x828244A4;
	sub_82DF3A08(ctx, base);
	// 828244A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 828244A8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 828244AC: 38BF0008  addi r5, r31, 8
	ctx.r[5].s64 = ctx.r[31].s64 + 8;
	// 828244B0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 828244B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 828244B8: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 828244BC: C3CA08AC  lfs f30, 0x8ac(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2220 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 828244C0: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 828244C4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 828244C8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 828244CC: 4BD7EDDD  bl 0x825a32a8
	ctx.lr = 0x828244D0;
	sub_825A32A8(ctx, base);
	// 828244D0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 828244D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828244D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828244DC: 4BD7D295  bl 0x825a1770
	ctx.lr = 0x828244E0;
	sub_825A1770(ctx, base);
	// 828244E0: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 828244E4: 485CEF45  bl 0x82df3428
	ctx.lr = 0x828244E8;
	sub_82DF3428(ctx, base);
	// 828244E8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 828244EC: 4BAA47CD  bl 0x822c8cb8
	ctx.lr = 0x828244F0;
	sub_822C8CB8(ctx, base);
	// 828244F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828244F4: 485CEF35  bl 0x82df3428
	ctx.lr = 0x828244F8;
	sub_82DF3428(ctx, base);
	// 828244F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828244FC: 485CEF2D  bl 0x82df3428
	ctx.lr = 0x82824500;
	sub_82DF3428(ctx, base);
	// 82824500: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824504: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824508: 388BA6A0  addi r4, r11, -0x5960
	ctx.r[4].s64 = ctx.r[11].s64 + -22880;
	// 8282450C: 485CF4FD  bl 0x82df3a08
	ctx.lr = 0x82824510;
	sub_82DF3A08(ctx, base);
	// 82824510: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82824514: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82824518: 388BED88  addi r4, r11, -0x1278
	ctx.r[4].s64 = ctx.r[11].s64 + -4728;
	// 8282451C: 485CF4ED  bl 0x82df3a08
	ctx.lr = 0x82824520;
	sub_82DF3A08(ctx, base);
	// 82824520: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 82824524: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824528: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282452C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82824530: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82824534: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82824538: 4BD7ED71  bl 0x825a32a8
	ctx.lr = 0x8282453C;
	sub_825A32A8(ctx, base);
	// 8282453C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82824540: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824544: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82824548: 4BD7D229  bl 0x825a1770
	ctx.lr = 0x8282454C;
	sub_825A1770(ctx, base);
	// 8282454C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82824550: 485CEED9  bl 0x82df3428
	ctx.lr = 0x82824554;
	sub_82DF3428(ctx, base);
	// 82824554: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82824558: 4BAA4761  bl 0x822c8cb8
	ctx.lr = 0x8282455C;
	sub_822C8CB8(ctx, base);
	// 8282455C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82824560: 485CEEC9  bl 0x82df3428
	ctx.lr = 0x82824564;
	sub_82DF3428(ctx, base);
	// 82824564: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824568: 485CEEC1  bl 0x82df3428
	ctx.lr = 0x8282456C;
	sub_82DF3428(ctx, base);
	// 8282456C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824574: 388BA688  addi r4, r11, -0x5978
	ctx.r[4].s64 = ctx.r[11].s64 + -22904;
	// 82824578: 485CF491  bl 0x82df3a08
	ctx.lr = 0x8282457C;
	sub_82DF3A08(ctx, base);
	// 8282457C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82824580: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82824584: 388BED64  addi r4, r11, -0x129c
	ctx.r[4].s64 = ctx.r[11].s64 + -4764;
	// 82824588: 485CF481  bl 0x82df3a08
	ctx.lr = 0x8282458C;
	sub_82DF3A08(ctx, base);
	// 8282458C: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 82824590: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824594: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82824598: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 8282459C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 828245A0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 828245A4: 4BD7ED05  bl 0x825a32a8
	ctx.lr = 0x828245A8;
	sub_825A32A8(ctx, base);
	// 828245A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 828245AC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 828245B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828245B4: 4BD7D1BD  bl 0x825a1770
	ctx.lr = 0x828245B8;
	sub_825A1770(ctx, base);
	// 828245B8: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 828245BC: 485CEE6D  bl 0x82df3428
	ctx.lr = 0x828245C0;
	sub_82DF3428(ctx, base);
	// 828245C0: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 828245C4: 4BAA46F5  bl 0x822c8cb8
	ctx.lr = 0x828245C8;
	sub_822C8CB8(ctx, base);
	// 828245C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828245CC: 485CEE5D  bl 0x82df3428
	ctx.lr = 0x828245D0;
	sub_82DF3428(ctx, base);
	// 828245D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828245D4: 485CEE55  bl 0x82df3428
	ctx.lr = 0x828245D8;
	sub_82DF3428(ctx, base);
	// 828245D8: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 828245DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828245E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828245E4: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 828245E8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 828245EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828245F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828245F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828245F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828245F8 size=112
    let mut pc: u32 = 0x828245F8;
    'dispatch: loop {
        match pc {
            0x828245F8 => {
    //   block [0x828245F8..0x82824668)
	// 828245F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828245FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82824604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282460C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824610: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824614: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82824618: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282461C: 396BA5E4  addi r11, r11, -0x5a1c
	ctx.r[11].s64 = ctx.r[11].s64 + -23068;
	// 82824620: 394AA5D0  addi r10, r10, -0x5a30
	ctx.r[10].s64 = ctx.r[10].s64 + -23088;
	// 82824624: 3929A584  addi r9, r9, -0x5a7c
	ctx.r[9].s64 = ctx.r[9].s64 + -23164;
	// 82824628: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282462C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82824630: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82824634: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82824638: 4BFFF7C1  bl 0x82823df8
	ctx.lr = 0x8282463C;
	sub_82823DF8(ctx, base);
	// 8282463C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82824640: 4182000C  beq 0x8282464c
	if ctx.cr[0].eq {
	pc = 0x8282464C; continue 'dispatch;
	}
	// 82824644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824648: 485CDD91  bl 0x82df23d8
	ctx.lr = 0x8282464C;
	sub_82DF23D8(ctx, base);
	// 8282464C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824650: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82824654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282465C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82824660: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824668 size=176
    let mut pc: u32 = 0x82824668;
    'dispatch: loop {
        match pc {
            0x82824668 => {
    //   block [0x82824668..0x82824718)
	// 82824668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824674: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824678: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282467C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82824680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824684: 388BA3F4  addi r4, r11, -0x5c0c
	ctx.r[4].s64 = ctx.r[11].s64 + -23564;
	// 82824688: 485CF381  bl 0x82df3a08
	ctx.lr = 0x8282468C;
	sub_82DF3A08(ctx, base);
	// 8282468C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824690: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82824694: 4803F02D  bl 0x828636c0
	ctx.lr = 0x82824698;
	sub_828636C0(ctx, base);
	// 82824698: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282469C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 828246A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828246A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828246A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 828246AC: 419A0024  beq cr6, 0x828246d0
	if ctx.cr[6].eq {
	pc = 0x828246D0; continue 'dispatch;
	}
	// 828246B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828246B4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828246B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828246BC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828246C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828246C4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828246C8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828246CC: 4082FFE8  bne 0x828246b4
	if !ctx.cr[0].eq {
	pc = 0x828246B4; continue 'dispatch;
	}
	// 828246D0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 828246D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828246D8: 4BFCA599  bl 0x827eec70
	ctx.lr = 0x828246DC;
	sub_827EEC70(ctx, base);
	// 828246DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828246E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828246E4: 419A0008  beq cr6, 0x828246ec
	if ctx.cr[6].eq {
	pc = 0x828246EC; continue 'dispatch;
	}
	// 828246E8: 4BA9C1A9  bl 0x822c0890
	ctx.lr = 0x828246EC;
	sub_822C0890(ctx, base);
	// 828246EC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 828246F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828246F4: 419A0008  beq cr6, 0x828246fc
	if ctx.cr[6].eq {
	pc = 0x828246FC; continue 'dispatch;
	}
	// 828246F8: 4BA9C199  bl 0x822c0890
	ctx.lr = 0x828246FC;
	sub_822C0890(ctx, base);
	// 828246FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824700: 485CED29  bl 0x82df3428
	ctx.lr = 0x82824704;
	sub_82DF3428(ctx, base);
	// 82824704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82824708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282470C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824718 size=120
    let mut pc: u32 = 0x82824718;
    'dispatch: loop {
        match pc {
            0x82824718 => {
    //   block [0x82824718..0x82824790)
	// 82824718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282471C: 48983A51  bl 0x831a816c
	ctx.lr = 0x82824720;
	sub_831A8130(ctx, base);
	// 82824720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824724: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82824728: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282472C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82824730: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82824734: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82824738: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8282473C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82824740: 485CDCA9  bl 0x82df23e8
	ctx.lr = 0x82824744;
	sub_82DF23E8(ctx, base);
	// 82824744: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82824748: 41820014  beq 0x8282475c
	if ctx.cr[0].eq {
	pc = 0x8282475C; continue 'dispatch;
	}
	// 8282474C: 889F0000  lbz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82824750: 480F1149  bl 0x82915898
	ctx.lr = 0x82824754;
	sub_82915898(ctx, base);
	// 82824754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824758: 48000008  b 0x82824760
	pc = 0x82824760; continue 'dispatch;
	// 8282475C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82824760: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82824764: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82824768: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282476C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824770: 4BFFF431  bl 0x82823ba0
	ctx.lr = 0x82824774;
	sub_82823BA0(ctx, base);
	// 82824774: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82824778: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282477C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824780: 4BA9B881  bl 0x822c0000
	ctx.lr = 0x82824784;
	sub_822C0000(ctx, base);
	// 82824784: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82824788: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282478C: 48983A30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824790 size=120
    let mut pc: u32 = 0x82824790;
    'dispatch: loop {
        match pc {
            0x82824790 => {
    //   block [0x82824790..0x82824808)
	// 82824790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824794: 489839D9  bl 0x831a816c
	ctx.lr = 0x82824798;
	sub_831A8130(ctx, base);
	// 82824798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282479C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 828247A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828247A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828247A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828247AC: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 828247B0: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 828247B4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 828247B8: 485CDC31  bl 0x82df23e8
	ctx.lr = 0x828247BC;
	sub_82DF23E8(ctx, base);
	// 828247BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828247C0: 41820014  beq 0x828247d4
	if ctx.cr[0].eq {
	pc = 0x828247D4; continue 'dispatch;
	}
	// 828247C4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828247C8: 4832E579  bl 0x82b52d40
	ctx.lr = 0x828247CC;
	sub_82B52D40(ctx, base);
	// 828247CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828247D0: 48000008  b 0x828247d8
	pc = 0x828247D8; continue 'dispatch;
	// 828247D4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828247D8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 828247DC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 828247E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828247E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828247E8: 4BFFF481  bl 0x82823c68
	ctx.lr = 0x828247EC;
	sub_82823C68(ctx, base);
	// 828247EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828247F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828247F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828247F8: 4BA9B809  bl 0x822c0000
	ctx.lr = 0x828247FC;
	sub_822C0000(ctx, base);
	// 828247FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82824800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82824804: 489839B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824808 size=120
    let mut pc: u32 = 0x82824808;
    'dispatch: loop {
        match pc {
            0x82824808 => {
    //   block [0x82824808..0x82824880)
	// 82824808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282480C: 48983961  bl 0x831a816c
	ctx.lr = 0x82824810;
	sub_831A8130(ctx, base);
	// 82824810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824814: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82824818: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282481C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82824820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82824824: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82824828: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8282482C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82824830: 485CDBB9  bl 0x82df23e8
	ctx.lr = 0x82824834;
	sub_82DF23E8(ctx, base);
	// 82824834: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82824838: 41820014  beq 0x8282484c
	if ctx.cr[0].eq {
	pc = 0x8282484C; continue 'dispatch;
	}
	// 8282483C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82824840: 4BFE2EC9  bl 0x82807708
	ctx.lr = 0x82824844;
	sub_82807708(ctx, base);
	// 82824844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824848: 48000008  b 0x82824850
	pc = 0x82824850; continue 'dispatch;
	// 8282484C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82824850: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82824854: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82824858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282485C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824860: 4BFFF4D1  bl 0x82823d30
	ctx.lr = 0x82824864;
	sub_82823D30(ctx, base);
	// 82824864: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82824868: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282486C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824870: 4BA9B791  bl 0x822c0000
	ctx.lr = 0x82824874;
	sub_822C0000(ctx, base);
	// 82824874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82824878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282487C: 48983940  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824880 size=176
    let mut pc: u32 = 0x82824880;
    'dispatch: loop {
        match pc {
            0x82824880 => {
    //   block [0x82824880..0x82824930)
	// 82824880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282488C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824890: 4813BCD1  bl 0x82960560
	ctx.lr = 0x82824894;
	sub_82960560(ctx, base);
	// 82824894: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82824898: 41820084  beq 0x8282491c
	if ctx.cr[0].eq {
	pc = 0x8282491C; continue 'dispatch;
	}
	// 8282489C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 828248A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828248A4: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 828248A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 828248AC: 4BFFFE6D  bl 0x82824718
	ctx.lr = 0x828248B0;
	sub_82824718(ctx, base);
	// 828248B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828248B4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 828248B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828248BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828248C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 828248C4: 419A0024  beq cr6, 0x828248e8
	if ctx.cr[6].eq {
	pc = 0x828248E8; continue 'dispatch;
	}
	// 828248C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828248CC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828248D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828248D4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828248D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828248DC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828248E0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828248E4: 4082FFE8  bne 0x828248cc
	if !ctx.cr[0].eq {
	pc = 0x828248CC; continue 'dispatch;
	}
	// 828248E8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828248EC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 828248F0: 388BA680  addi r4, r11, -0x5980
	ctx.r[4].s64 = ctx.r[11].s64 + -22912;
	// 828248F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828248F8: 4BFF5BC1  bl 0x8281a4b8
	ctx.lr = 0x828248FC;
	sub_8281A4B8(ctx, base);
	// 828248FC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82824900: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824904: 419A0008  beq cr6, 0x8282490c
	if ctx.cr[6].eq {
	pc = 0x8282490C; continue 'dispatch;
	}
	// 82824908: 4BA9BF89  bl 0x822c0890
	ctx.lr = 0x8282490C;
	sub_822C0890(ctx, base);
	// 8282490C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82824910: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824914: 419A0008  beq cr6, 0x8282491c
	if ctx.cr[6].eq {
	pc = 0x8282491C; continue 'dispatch;
	}
	// 82824918: 4BA9BF79  bl 0x822c0890
	ctx.lr = 0x8282491C;
	sub_822C0890(ctx, base);
	// 8282491C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82824920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282492C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824930 size=176
    let mut pc: u32 = 0x82824930;
    'dispatch: loop {
        match pc {
            0x82824930 => {
    //   block [0x82824930..0x828249E0)
	// 82824930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282493C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824940: 4813BC21  bl 0x82960560
	ctx.lr = 0x82824944;
	sub_82960560(ctx, base);
	// 82824944: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82824948: 41820084  beq 0x828249cc
	if ctx.cr[0].eq {
	pc = 0x828249CC; continue 'dispatch;
	}
	// 8282494C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82824950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824954: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82824958: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282495C: 4BFFFDBD  bl 0x82824718
	ctx.lr = 0x82824960;
	sub_82824718(ctx, base);
	// 82824960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82824964: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82824968: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8282496C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82824970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82824974: 419A0024  beq cr6, 0x82824998
	if ctx.cr[6].eq {
	pc = 0x82824998; continue 'dispatch;
	}
	// 82824978: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8282497C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82824980: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824984: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82824988: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282498C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82824990: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824994: 4082FFE8  bne 0x8282497c
	if !ctx.cr[0].eq {
	pc = 0x8282497C; continue 'dispatch;
	}
	// 82824998: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282499C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 828249A0: 388BA680  addi r4, r11, -0x5980
	ctx.r[4].s64 = ctx.r[11].s64 + -22912;
	// 828249A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828249A8: 4BFF5B11  bl 0x8281a4b8
	ctx.lr = 0x828249AC;
	sub_8281A4B8(ctx, base);
	// 828249AC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828249B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828249B4: 419A0008  beq cr6, 0x828249bc
	if ctx.cr[6].eq {
	pc = 0x828249BC; continue 'dispatch;
	}
	// 828249B8: 4BA9BED9  bl 0x822c0890
	ctx.lr = 0x828249BC;
	sub_822C0890(ctx, base);
	// 828249BC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 828249C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828249C4: 419A0008  beq cr6, 0x828249cc
	if ctx.cr[6].eq {
	pc = 0x828249CC; continue 'dispatch;
	}
	// 828249C8: 4BA9BEC9  bl 0x822c0890
	ctx.lr = 0x828249CC;
	sub_822C0890(ctx, base);
	// 828249CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 828249D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828249D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828249D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828249DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828249E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828249E0 size=172
    let mut pc: u32 = 0x828249E0;
    'dispatch: loop {
        match pc {
            0x828249E0 => {
    //   block [0x828249E0..0x82824A8C)
	// 828249E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828249E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828249E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828249EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828249F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828249F4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 828249F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828249FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82824A00: 388B66F4  addi r4, r11, 0x66f4
	ctx.r[4].s64 = ctx.r[11].s64 + 26356;
	// 82824A04: 4BFFFD8D  bl 0x82824790
	ctx.lr = 0x82824A08;
	sub_82824790(ctx, base);
	// 82824A08: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82824A0C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82824A10: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82824A14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82824A18: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82824A1C: 419A0024  beq cr6, 0x82824a40
	if ctx.cr[6].eq {
	pc = 0x82824A40; continue 'dispatch;
	}
	// 82824A20: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82824A24: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82824A28: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824A2C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82824A30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82824A34: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82824A38: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824A3C: 4082FFE8  bne 0x82824a24
	if !ctx.cr[0].eq {
	pc = 0x82824A24; continue 'dispatch;
	}
	// 82824A40: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824A44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824A48: 4BFF5A79  bl 0x8281a4c0
	ctx.lr = 0x82824A4C;
	sub_8281A4C0(ctx, base);
	// 82824A4C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82824A50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824A54: 419A0008  beq cr6, 0x82824a5c
	if ctx.cr[6].eq {
	pc = 0x82824A5C; continue 'dispatch;
	}
	// 82824A58: 4BA9BE39  bl 0x822c0890
	ctx.lr = 0x82824A5C;
	sub_822C0890(ctx, base);
	// 82824A5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82824A60: 419A000C  beq cr6, 0x82824a6c
	if ctx.cr[6].eq {
	pc = 0x82824A6C; continue 'dispatch;
	}
	// 82824A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824A68: 4BA9BE29  bl 0x822c0890
	ctx.lr = 0x82824A6C;
	sub_822C0890(ctx, base);
	// 82824A6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824A70: 4BFF5A41  bl 0x8281a4b0
	ctx.lr = 0x82824A74;
	sub_8281A4B0(ctx, base);
	// 82824A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82824A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824A80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82824A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82824A90 size=172
    let mut pc: u32 = 0x82824A90;
    'dispatch: loop {
        match pc {
            0x82824A90 => {
    //   block [0x82824A90..0x82824B3C)
	// 82824A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82824A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824AA0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824AA4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824AAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82824AB0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82824AB4: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 82824AB8: 485CEF51  bl 0x82df3a08
	ctx.lr = 0x82824ABC;
	sub_82DF3A08(ctx, base);
	// 82824ABC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82824AC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824AC4: 388BF614  addi r4, r11, -0x9ec
	ctx.r[4].s64 = ctx.r[11].s64 + -2540;
	// 82824AC8: 485CEF41  bl 0x82df3a08
	ctx.lr = 0x82824ACC;
	sub_82DF3A08(ctx, base);
	// 82824ACC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82824AD0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82824AD4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82824AD8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82824ADC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82824AE0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82824AE4: C06B964C  lfs f3, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82824AE8: C04ADD6C  lfs f2, -0x2294(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82824AEC: C02908A4  lfs f1, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82824AF0: 4BD7E7B9  bl 0x825a32a8
	ctx.lr = 0x82824AF4;
	sub_825A32A8(ctx, base);
	// 82824AF4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82824AF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824AFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824B00: 4BD7CC71  bl 0x825a1770
	ctx.lr = 0x82824B04;
	sub_825A1770(ctx, base);
	// 82824B04: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82824B08: 485CE921  bl 0x82df3428
	ctx.lr = 0x82824B0C;
	sub_82DF3428(ctx, base);
	// 82824B0C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82824B10: 4BAA41A9  bl 0x822c8cb8
	ctx.lr = 0x82824B14;
	sub_822C8CB8(ctx, base);
	// 82824B14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824B18: 485CE911  bl 0x82df3428
	ctx.lr = 0x82824B1C;
	sub_82DF3428(ctx, base);
	// 82824B1C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82824B20: 485CE909  bl 0x82df3428
	ctx.lr = 0x82824B24;
	sub_82DF3428(ctx, base);
	// 82824B24: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82824B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824B30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82824B34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824B40 size=112
    let mut pc: u32 = 0x82824B40;
    'dispatch: loop {
        match pc {
            0x82824B40 => {
    //   block [0x82824B40..0x82824BB0)
	// 82824B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824B48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824B4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824B50: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82824B54: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82824B58: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82824B5C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82824B60: 38AA6910  addi r5, r10, 0x6910
	ctx.r[5].s64 = ctx.r[10].s64 + 26896;
	// 82824B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824B68: 388B8614  addi r4, r11, -0x79ec
	ctx.r[4].s64 = ctx.r[11].s64 + -31212;
	// 82824B6C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82824B70: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82824B74: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82824B78: 4813E4E9  bl 0x82963060
	ctx.lr = 0x82824B7C;
	sub_82963060(ctx, base);
	// 82824B7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82824B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824B84: 4BFFFC85  bl 0x82824808
	ctx.lr = 0x82824B88;
	sub_82824808(ctx, base);
	// 82824B88: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82824B8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824B90: 419A0008  beq cr6, 0x82824b98
	if ctx.cr[6].eq {
	pc = 0x82824B98; continue 'dispatch;
	}
	// 82824B94: 4BA9BCFD  bl 0x822c0890
	ctx.lr = 0x82824B98;
	sub_822C0890(ctx, base);
	// 82824B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824B9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82824BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824BA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824BB0 size=180
    let mut pc: u32 = 0x82824BB0;
    'dispatch: loop {
        match pc {
            0x82824BB0 => {
    //   block [0x82824BB0..0x82824C64)
	// 82824BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82824BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824BC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824BC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82824BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82824BCC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82824BD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82824BD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824BD8: 4BA9BD61  bl 0x822c0938
	ctx.lr = 0x82824BDC;
	sub_822C0938(ctx, base);
	// 82824BDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82824BE0: 41820028  beq 0x82824c08
	if ctx.cr[0].eq {
	pc = 0x82824C08; continue 'dispatch;
	}
	// 82824BE4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824BE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82824BEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82824BF0: 392BA408  addi r9, r11, -0x5bf8
	ctx.r[9].s64 = ctx.r[11].s64 + -23544;
	// 82824BF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82824BF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82824BFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82824C00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82824C04: 48000008  b 0x82824c0c
	pc = 0x82824C0C; continue 'dispatch;
	// 82824C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82824C0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824C10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82824C14: 409A0034  bne cr6, 0x82824c48
	if !ctx.cr[6].eq {
	pc = 0x82824C48; continue 'dispatch;
	}
	// 82824C18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82824C1C: 419A000C  beq cr6, 0x82824c28
	if ctx.cr[6].eq {
	pc = 0x82824C28; continue 'dispatch;
	}
	// 82824C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824C24: 4BA9B645  bl 0x822c0268
	ctx.lr = 0x82824C28;
	sub_822C0268(ctx, base);
	// 82824C28: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82824C2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82824C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824C34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82824C38: 816BFF2C  lwz r11, -0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82824C3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82824C40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82824C44: 4BA9B3BD  bl 0x822c0000
	ctx.lr = 0x82824C48;
	sub_822C0000(ctx, base);
	// 82824C48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82824C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82824C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82824C68 size=12
    let mut pc: u32 = 0x82824C68;
    'dispatch: loop {
        match pc {
            0x82824C68 => {
    //   block [0x82824C68..0x82824C74)
	// 82824C68: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82824C6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824C70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82824C74 size=8
    let mut pc: u32 = 0x82824C74;
    'dispatch: loop {
        match pc {
            0x82824C74 => {
    //   block [0x82824C74..0x82824C7C)
	// 82824C74: 4BA9B5F4  b 0x822c0268
	sub_822C0268(ctx, base);
	return;
	// 82824C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824C80 size=396
    let mut pc: u32 = 0x82824C80;
    'dispatch: loop {
        match pc {
            0x82824C80 => {
    //   block [0x82824C80..0x82824E0C)
	// 82824C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824C84: 489834DD  bl 0x831a8160
	ctx.lr = 0x82824C88;
	sub_831A8130(ctx, base);
	// 82824C88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824C8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82824C90: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82824C94: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82824C98: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82824C9C: 4BFF9C55  bl 0x8281e8f0
	ctx.lr = 0x82824CA0;
	sub_8281E8F0(ctx, base);
	// 82824CA0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82824CA8: 388BA6E8  addi r4, r11, -0x5918
	ctx.r[4].s64 = ctx.r[11].s64 + -22808;
	// 82824CAC: 38A0012D  li r5, 0x12d
	ctx.r[5].s64 = 301;
	// 82824CB0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82824CB4: 4BA9B725  bl 0x822c03d8
	ctx.lr = 0x82824CB8;
	sub_822C03D8(ctx, base);
	// 82824CB8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82824CBC: 41820034  beq 0x82824cf0
	if ctx.cr[0].eq {
	pc = 0x82824CF0; continue 'dispatch;
	}
	// 82824CC0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824CC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824CC8: 388BA6DC  addi r4, r11, -0x5924
	ctx.r[4].s64 = ctx.r[11].s64 + -22820;
	// 82824CCC: 485CED3D  bl 0x82df3a08
	ctx.lr = 0x82824CD0;
	sub_82DF3A08(ctx, base);
	// 82824CD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82824CD4: 38BB00FC  addi r5, r27, 0xfc
	ctx.r[5].s64 = ctx.r[27].s64 + 252;
	// 82824CD8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824CE0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82824CE4: 4BD8772D  bl 0x825ac410
	ctx.lr = 0x82824CE8;
	sub_825AC410(ctx, base);
	// 82824CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824CEC: 48000008  b 0x82824cf4
	pc = 0x82824CF4; continue 'dispatch;
	// 82824CF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82824CF4: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82824CF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82824CFC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82824D00: 4BCCE6B9  bl 0x824f33b8
	ctx.lr = 0x82824D04;
	sub_824F33B8(ctx, base);
	// 82824D04: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82824D08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82824D0C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82824D10: 4BA9B2F1  bl 0x822c0000
	ctx.lr = 0x82824D14;
	sub_822C0000(ctx, base);
	// 82824D14: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82824D18: 4182000C  beq 0x82824d24
	if ctx.cr[0].eq {
	pc = 0x82824D24; continue 'dispatch;
	}
	// 82824D1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824D20: 485CE709  bl 0x82df3428
	ctx.lr = 0x82824D24;
	sub_82DF3428(ctx, base);
	// 82824D24: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82824D28: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82824D2C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82824D30: 3BABFF14  addi r29, r11, -0xec
	ctx.r[29].s64 = ctx.r[11].s64 + -236;
	// 82824D34: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82824D38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824D3C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82824D40: 485CECC9  bl 0x82df3a08
	ctx.lr = 0x82824D44;
	sub_82DF3A08(ctx, base);
	// 82824D44: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82824D48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82824D4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82824D50: 4BD87071  bl 0x825abdc0
	ctx.lr = 0x82824D54;
	sub_825ABDC0(ctx, base);
	// 82824D54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824D58: 485CE6D1  bl 0x82df3428
	ctx.lr = 0x82824D5C;
	sub_82DF3428(ctx, base);
	// 82824D5C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82824D60: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 82824D64: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82824D68: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82824D6C: 4198FFCC  blt cr6, 0x82824d38
	if ctx.cr[6].lt {
	pc = 0x82824D38; continue 'dispatch;
	}
	// 82824D70: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824D74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824D78: 388BA6D0  addi r4, r11, -0x5930
	ctx.r[4].s64 = ctx.r[11].s64 + -22832;
	// 82824D7C: 485CEC8D  bl 0x82df3a08
	ctx.lr = 0x82824D80;
	sub_82DF3A08(ctx, base);
	// 82824D80: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82824D84: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82824D88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82824D8C: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82824D90: 419A0024  beq cr6, 0x82824db4
	if ctx.cr[6].eq {
	pc = 0x82824DB4; continue 'dispatch;
	}
	// 82824D94: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82824D98: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82824D9C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824DA0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82824DA4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82824DA8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82824DAC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82824DB0: 4082FFE8  bne 0x82824d98
	if !ctx.cr[0].eq {
	pc = 0x82824D98; continue 'dispatch;
	}
	// 82824DB4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82824DB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82824DBC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82824DC0: 4BD7D351  bl 0x825a2110
	ctx.lr = 0x82824DC4;
	sub_825A2110(ctx, base);
	// 82824DC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824DC8: 485CE661  bl 0x82df3428
	ctx.lr = 0x82824DCC;
	sub_82DF3428(ctx, base);
	// 82824DCC: 807B0104  lwz r3, 0x104(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(260 as u32) ) } as u64;
	// 82824DD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824DD4: 419A000C  beq cr6, 0x82824de0
	if ctx.cr[6].eq {
	pc = 0x82824DE0; continue 'dispatch;
	}
	// 82824DD8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82824DDC: 4BFFFCB5  bl 0x82824a90
	ctx.lr = 0x82824DE0;
	sub_82824A90(ctx, base);
	// 82824DE0: 807B010C  lwz r3, 0x10c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(268 as u32) ) } as u64;
	// 82824DE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82824DE8: 419A000C  beq cr6, 0x82824df4
	if ctx.cr[6].eq {
	pc = 0x82824DF4; continue 'dispatch;
	}
	// 82824DEC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82824DF0: 4BFFF671  bl 0x82824460
	ctx.lr = 0x82824DF4;
	sub_82824460(ctx, base);
	// 82824DF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82824DF8: 419A000C  beq cr6, 0x82824e04
	if ctx.cr[6].eq {
	pc = 0x82824E04; continue 'dispatch;
	}
	// 82824DFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824E00: 4BA9BA91  bl 0x822c0890
	ctx.lr = 0x82824E04;
	sub_822C0890(ctx, base);
	// 82824E04: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82824E08: 489833A8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824E10 size=196
    let mut pc: u32 = 0x82824E10;
    'dispatch: loop {
        match pc {
            0x82824E10 => {
    //   block [0x82824E10..0x82824ED4)
	// 82824E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824E18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82824E1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824E24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82824E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82824E2C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82824E30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82824E34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824E38: 4BA9BB01  bl 0x822c0938
	ctx.lr = 0x82824E3C;
	sub_822C0938(ctx, base);
	// 82824E3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82824E40: 41820028  beq 0x82824e68
	if ctx.cr[0].eq {
	pc = 0x82824E68; continue 'dispatch;
	}
	// 82824E44: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824E48: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82824E4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82824E50: 392BA41C  addi r9, r11, -0x5be4
	ctx.r[9].s64 = ctx.r[11].s64 + -23524;
	// 82824E54: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82824E58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82824E5C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82824E60: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82824E64: 48000008  b 0x82824e6c
	pc = 0x82824E6C; continue 'dispatch;
	// 82824E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82824E6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82824E74: 409A0044  bne cr6, 0x82824eb8
	if !ctx.cr[6].eq {
	pc = 0x82824EB8; continue 'dispatch;
	}
	// 82824E78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82824E7C: 419A001C  beq cr6, 0x82824e98
	if ctx.cr[6].eq {
	pc = 0x82824E98; continue 'dispatch;
	}
	// 82824E80: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82824E84: 4BAA3E35  bl 0x822c8cb8
	ctx.lr = 0x82824E88;
	sub_822C8CB8(ctx, base);
	// 82824E88: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82824E8C: 4BAA3E2D  bl 0x822c8cb8
	ctx.lr = 0x82824E90;
	sub_822C8CB8(ctx, base);
	// 82824E90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824E94: 4BA9B3D5  bl 0x822c0268
	ctx.lr = 0x82824E98;
	sub_822C0268(ctx, base);
	// 82824E98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82824E9C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82824EA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82824EA4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82824EA8: 816BFF2C  lwz r11, -0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-212 as u32) ) } as u64;
	// 82824EAC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82824EB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82824EB4: 4BA9B14D  bl 0x822c0000
	ctx.lr = 0x82824EB8;
	sub_822C0000(ctx, base);
	// 82824EB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82824EBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82824EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824EC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82824ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82824ED8 size=72
    let mut pc: u32 = 0x82824ED8;
    'dispatch: loop {
        match pc {
            0x82824ED8 => {
    //   block [0x82824ED8..0x82824F20)
	// 82824ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82824EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82824EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824EE8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82824EEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82824EF0: 419A001C  beq cr6, 0x82824f0c
	if ctx.cr[6].eq {
	pc = 0x82824F0C; continue 'dispatch;
	}
	// 82824EF4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82824EF8: 4BAA3DC1  bl 0x822c8cb8
	ctx.lr = 0x82824EFC;
	sub_822C8CB8(ctx, base);
	// 82824EFC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82824F00: 4BAA3DB9  bl 0x822c8cb8
	ctx.lr = 0x82824F04;
	sub_822C8CB8(ctx, base);
	// 82824F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82824F08: 4BA9B361  bl 0x822c0268
	ctx.lr = 0x82824F0C;
	sub_822C0268(ctx, base);
	// 82824F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82824F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82824F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82824F18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82824F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82824F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82824F20 size=348
    let mut pc: u32 = 0x82824F20;
    'dispatch: loop {
        match pc {
            0x82824F20 => {
    //   block [0x82824F20..0x8282507C)
	// 82824F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82824F24: 48983241  bl 0x831a8164
	ctx.lr = 0x82824F28;
	sub_831A8130(ctx, base);
	// 82824F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82824F2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82824F30: 4BFFAE19  bl 0x8281fd48
	ctx.lr = 0x82824F34;
	sub_8281FD48(ctx, base);
	// 82824F34: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824F38: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82824F3C: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82824F40: 396BA4E4  addi r11, r11, -0x5b1c
	ctx.r[11].s64 = ctx.r[11].s64 + -23324;
	// 82824F44: 394AA4D0  addi r10, r10, -0x5b30
	ctx.r[10].s64 = ctx.r[10].s64 + -23344;
	// 82824F48: 3929A484  addi r9, r9, -0x5b7c
	ctx.r[9].s64 = ctx.r[9].s64 + -23420;
	// 82824F4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82824F50: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82824F54: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82824F58: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82824F5C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824F60: 93DF02E0  stw r30, 0x2e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), ctx.r[30].u32 ) };
	// 82824F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82824F68: 93DF02E4  stw r30, 0x2e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(740 as u32), ctx.r[30].u32 ) };
	// 82824F6C: 3B6BA6E8  addi r27, r11, -0x5918
	ctx.r[27].s64 = ctx.r[11].s64 + -22808;
	// 82824F70: 93DF02E8  stw r30, 0x2e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(744 as u32), ctx.r[30].u32 ) };
	// 82824F74: 38A000B0  li r5, 0xb0
	ctx.r[5].s64 = 176;
	// 82824F78: 93DF02EC  stw r30, 0x2ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(748 as u32), ctx.r[30].u32 ) };
	// 82824F7C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82824F80: 93DF02F0  stw r30, 0x2f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[30].u32 ) };
	// 82824F84: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82824F88: 93DF02F4  stw r30, 0x2f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(756 as u32), ctx.r[30].u32 ) };
	// 82824F8C: 93DF02F8  stw r30, 0x2f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[30].u32 ) };
	// 82824F90: 93DF02FC  stw r30, 0x2fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(764 as u32), ctx.r[30].u32 ) };
	// 82824F94: 93DF0300  stw r30, 0x300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(768 as u32), ctx.r[30].u32 ) };
	// 82824F98: 93DF0304  stw r30, 0x304(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(772 as u32), ctx.r[30].u32 ) };
	// 82824F9C: 9BDF0308  stb r30, 0x308(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[30].u8 ) };
	// 82824FA0: 4BA9B439  bl 0x822c03d8
	ctx.lr = 0x82824FA4;
	sub_822C03D8(ctx, base);
	// 82824FA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82824FA8: 41820018  beq 0x82824fc0
	if ctx.cr[0].eq {
	pc = 0x82824FC0; continue 'dispatch;
	}
	// 82824FAC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82824FB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82824FB4: C00B89AC  lfs f0, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82824FB8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82824FBC: 48000008  b 0x82824fc4
	pc = 0x82824FC4; continue 'dispatch;
	// 82824FC0: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82824FC4: 93BF030C  stw r29, 0x30c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(780 as u32), ctx.r[29].u32 ) };
	// 82824FC8: 397F030C  addi r11, r31, 0x30c
	ctx.r[11].s64 = ctx.r[31].s64 + 780;
	// 82824FCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82824FD0: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 82824FD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82824FD8: 4BFFFBD9  bl 0x82824bb0
	ctx.lr = 0x82824FDC;
	sub_82824BB0(ctx, base);
	// 82824FDC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82824FE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82824FE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82824FE8: 4BA9B019  bl 0x822c0000
	ctx.lr = 0x82824FEC;
	sub_822C0000(ctx, base);
	// 82824FEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82824FF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82824FF4: 38A000AF  li r5, 0xaf
	ctx.r[5].s64 = 175;
	// 82824FF8: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 82824FFC: 4BA9B3DD  bl 0x822c03d8
	ctx.lr = 0x82825000;
	sub_822C03D8(ctx, base);
	// 82825000: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82825004: 41820040  beq 0x82825044
	if ctx.cr[0].eq {
	pc = 0x82825044; continue 'dispatch;
	}
	// 82825008: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282500C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82825010: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82825014: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82825018: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282501C: C1AB08A4  lfs f13, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82825020: C00A9524  lfs f0, -0x6adc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82825024: C189D7BC  lfs f12, -0x2844(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82825028: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8282502C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82825030: D183000C  stfs f12, 0xc(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82825034: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82825038: 93C30018  stw r30, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8282503C: 93C30038  stw r30, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82825040: 48000008  b 0x82825048
	pc = 0x82825048; continue 'dispatch;
	// 82825044: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82825048: 93BF0314  stw r29, 0x314(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(788 as u32), ctx.r[29].u32 ) };
	// 8282504C: 397F0314  addi r11, r31, 0x314
	ctx.r[11].s64 = ctx.r[31].s64 + 788;
	// 82825050: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82825054: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82825058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282505C: 4BFFFDB5  bl 0x82824e10
	ctx.lr = 0x82825060;
	sub_82824E10(ctx, base);
	// 82825060: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82825064: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82825068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282506C: 4BA9AF95  bl 0x822c0000
	ctx.lr = 0x82825070;
	sub_822C0000(ctx, base);
	// 82825070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82825078: 4898313C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82825080 size=136
    let mut pc: u32 = 0x82825080;
    'dispatch: loop {
        match pc {
            0x82825080 => {
    //   block [0x82825080..0x82825108)
	// 82825080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82825088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282508C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825090: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825094: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82825098: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282509C: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 828250A0: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 828250A4: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828250A8: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 828250AC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 828250B0: 40980044  bge cr6, 0x828250f4
	if !ctx.cr[6].lt {
	pc = 0x828250F4; continue 'dispatch;
	}
	// 828250B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828250B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828250BC: 409A0018  bne cr6, 0x828250d4
	if !ctx.cr[6].eq {
	pc = 0x828250D4; continue 'dispatch;
	}
	// 828250C0: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 828250C4: 4BC2E5BD  bl 0x82453680
	ctx.lr = 0x828250C8;
	sub_82453680(ctx, base);
	// 828250C8: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828250CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 828250D0: 4800001C  b 0x828250ec
	pc = 0x828250EC; continue 'dispatch;
	// 828250D4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 828250D8: 409A001C  bne cr6, 0x828250f4
	if !ctx.cr[6].eq {
	pc = 0x828250F4; continue 'dispatch;
	}
	// 828250DC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 828250E0: 4BC2E5A1  bl 0x82453680
	ctx.lr = 0x828250E4;
	sub_82453680(ctx, base);
	// 828250E4: C01F0010  lfs f0, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828250E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828250EC: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 828250F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828250F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 828250F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828250FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82825100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82825104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82825108 size=100
    let mut pc: u32 = 0x82825108;
    'dispatch: loop {
        match pc {
            0x82825108 => {
    //   block [0x82825108..0x8282516C)
	// 82825108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282510C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82825110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82825114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825118: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282511C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82825120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825124: C04B959C  lfs f2, -0x6a64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82825128: C02A9648  lfs f1, -0x69b8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27064 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282512C: 4BFFFDF5  bl 0x82824f20
	ctx.lr = 0x82825130;
	sub_82824F20(ctx, base);
	// 82825130: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82825134: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82825138: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282513C: 396BA5E4  addi r11, r11, -0x5a1c
	ctx.r[11].s64 = ctx.r[11].s64 + -23068;
	// 82825140: 394AA5D0  addi r10, r10, -0x5a30
	ctx.r[10].s64 = ctx.r[10].s64 + -23088;
	// 82825144: 3929A584  addi r9, r9, -0x5a7c
	ctx.r[9].s64 = ctx.r[9].s64 + -23164;
	// 82825148: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282514C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82825150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825154: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82825158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282515C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82825160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82825164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82825168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82825170 size=128
    let mut pc: u32 = 0x82825170;
    'dispatch: loop {
        match pc {
            0x82825170 => {
    //   block [0x82825170..0x828251F0)
	// 82825170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825174: 48982FF9  bl 0x831a816c
	ctx.lr = 0x82825178;
	sub_831A8130(ctx, base);
	// 82825178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282517C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82825180: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82825184: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82825188: 3BEBAFC4  addi r31, r11, -0x503c
	ctx.r[31].s64 = ctx.r[11].s64 + -20540;
	// 8282518C: 816AAFCC  lwz r11, -0x5034(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20532 as u32) ) } as u64;
	// 82825190: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82825194: 40820024  bne 0x828251b8
	if !ctx.cr[0].eq {
	pc = 0x828251B8; continue 'dispatch;
	}
	// 82825198: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8282519C: 3D008282  lis r8, -0x7d7e
	ctx.r[8].s64 = -2105409536;
	// 828251A0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 828251A4: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 828251A8: 390840C8  addi r8, r8, 0x40c8
	ctx.r[8].s64 = ctx.r[8].s64 + 16584;
	// 828251AC: 916AAFCC  stw r11, -0x5034(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20532 as u32), ctx.r[11].u32 ) };
	// 828251B0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 828251B4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 828251B8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 828251BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 828251C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828251C4: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 828251C8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 828251CC: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828251D0: 4BE2F3F1  bl 0x826545c0
	ctx.lr = 0x828251D4;
	sub_826545C0(ctx, base);
	// 828251D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828251D8: 4182000C  beq 0x828251e4
	if ctx.cr[0].eq {
	pc = 0x828251E4; continue 'dispatch;
	}
	// 828251DC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 828251E0: 48000008  b 0x828251e8
	pc = 0x828251E8; continue 'dispatch;
	// 828251E4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 828251E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 828251EC: 48982FD0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828251F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828251F0 size=128
    let mut pc: u32 = 0x828251F0;
    'dispatch: loop {
        match pc {
            0x828251F0 => {
    //   block [0x828251F0..0x82825270)
	// 828251F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828251F4: 48982F79  bl 0x831a816c
	ctx.lr = 0x828251F8;
	sub_831A8130(ctx, base);
	// 828251F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828251FC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82825200: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82825204: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82825208: 3BEBAFD0  addi r31, r11, -0x5030
	ctx.r[31].s64 = ctx.r[11].s64 + -20528;
	// 8282520C: 816AAFD8  lwz r11, -0x5028(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20520 as u32) ) } as u64;
	// 82825210: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82825214: 40820024  bne 0x82825238
	if !ctx.cr[0].eq {
	pc = 0x82825238; continue 'dispatch;
	}
	// 82825218: 3D208271  lis r9, -0x7d8f
	ctx.r[9].s64 = -2106523648;
	// 8282521C: 3D008282  lis r8, -0x7d7e
	ctx.r[8].s64 = -2105409536;
	// 82825220: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82825224: 39298E48  addi r9, r9, -0x71b8
	ctx.r[9].s64 = ctx.r[9].s64 + -29112;
	// 82825228: 39084110  addi r8, r8, 0x4110
	ctx.r[8].s64 = ctx.r[8].s64 + 16656;
	// 8282522C: 916AAFD8  stw r11, -0x5028(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20520 as u32), ctx.r[11].u32 ) };
	// 82825230: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82825234: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82825238: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8282523C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82825240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825244: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82825248: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8282524C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82825250: 4BE2F371  bl 0x826545c0
	ctx.lr = 0x82825254;
	sub_826545C0(ctx, base);
	// 82825254: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825258: 4182000C  beq 0x82825264
	if ctx.cr[0].eq {
	pc = 0x82825264; continue 'dispatch;
	}
	// 8282525C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82825260: 48000008  b 0x82825268
	pc = 0x82825268; continue 'dispatch;
	// 82825264: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82825268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8282526C: 48982F50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82825270 size=88
    let mut pc: u32 = 0x82825270;
    'dispatch: loop {
        match pc {
            0x82825270 => {
    //   block [0x82825270..0x828252C8)
	// 82825270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82825278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282527C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82825280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825288: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282528C: 4BFF99A5  bl 0x8281ec30
	ctx.lr = 0x82825290;
	sub_8281EC30(ctx, base);
	// 82825290: 807F0314  lwz r3, 0x314(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(788 as u32) ) } as u64;
	// 82825294: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82825298: 419A0018  beq cr6, 0x828252b0
	if ctx.cr[6].eq {
	pc = 0x828252B0; continue 'dispatch;
	}
	// 8282529C: 817F0304  lwz r11, 0x304(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(772 as u32) ) } as u64;
	// 828252A0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 828252A4: 409A000C  bne cr6, 0x828252b0
	if !ctx.cr[6].eq {
	pc = 0x828252B0; continue 'dispatch;
	}
	// 828252A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828252AC: 4BFFFDD5  bl 0x82825080
	ctx.lr = 0x828252B0;
	sub_82825080(ctx, base);
	// 828252B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828252B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828252B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828252BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828252C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828252C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828252C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828252C8 size=476
    let mut pc: u32 = 0x828252C8;
    'dispatch: loop {
        match pc {
            0x828252C8 => {
    //   block [0x828252C8..0x828254A4)
	// 828252C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828252CC: 48982E99  bl 0x831a8164
	ctx.lr = 0x828252D0;
	sub_831A8130(ctx, base);
	// 828252D0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 828252D4: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828252D8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828252DC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 828252E0: 3BCBA6E8  addi r30, r11, -0x5918
	ctx.r[30].s64 = ctx.r[11].s64 + -22808;
	// 828252E4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 828252E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828252EC: 38A002A8  li r5, 0x2a8
	ctx.r[5].s64 = 680;
	// 828252F0: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 828252F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828252F8: 38600320  li r3, 0x320
	ctx.r[3].s64 = 800;
	// 828252FC: 485CD0ED  bl 0x82df23e8
	ctx.lr = 0x82825300;
	sub_82DF23E8(ctx, base);
	// 82825300: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82825304: 41820010  beq 0x82825314
	if ctx.cr[0].eq {
	pc = 0x82825314; continue 'dispatch;
	}
	// 82825308: 4BFFFE01  bl 0x82825108
	ctx.lr = 0x8282530C;
	sub_82825108(ctx, base);
	// 8282530C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825310: 48000008  b 0x82825318
	pc = 0x82825318; continue 'dispatch;
	// 82825314: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82825318: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8282531C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825320: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82825324: 4BFFE7B5  bl 0x82823ad8
	ctx.lr = 0x82825328;
	sub_82823AD8(ctx, base);
	// 82825328: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282532C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825330: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82825334: 4BA9ACCD  bl 0x822c0000
	ctx.lr = 0x82825338;
	sub_822C0000(ctx, base);
	// 82825338: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282533C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82825340: 38A002A9  li r5, 0x2a9
	ctx.r[5].s64 = 681;
	// 82825344: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82825348: 4BA9B091  bl 0x822c03d8
	ctx.lr = 0x8282534C;
	sub_822C03D8(ctx, base);
	// 8282534C: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82825350: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82825354: 418200E0  beq 0x82825434
	if ctx.cr[0].eq {
	pc = 0x82825434; continue 'dispatch;
	}
	// 82825358: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282535C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82825360: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82825364: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82825368: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8282536C: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82825370: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 82825374: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82825378: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8282537C: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82825380: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82825384: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82825388: 409A0008  bne cr6, 0x82825390
	if !ctx.cr[6].eq {
	pc = 0x82825390; continue 'dispatch;
	}
	// 8282538C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82825390: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82825394: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82825398: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8282539C: 419A0024  beq cr6, 0x828253c0
	if ctx.cr[6].eq {
	pc = 0x828253C0; continue 'dispatch;
	}
	// 828253A0: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828253A4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 828253A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828253AC: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 828253B0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 828253B4: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828253B8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828253BC: 4082FFE8  bne 0x828253a4
	if !ctx.cr[0].eq {
	pc = 0x828253A4; continue 'dispatch;
	}
	// 828253C0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 828253C4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828253C8: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 828253CC: 419A0024  beq cr6, 0x828253f0
	if ctx.cr[6].eq {
	pc = 0x828253F0; continue 'dispatch;
	}
	// 828253D0: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828253D4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828253D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828253DC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828253E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828253E4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828253E8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828253EC: 4082FFE8  bne 0x828253d4
	if !ctx.cr[0].eq {
	pc = 0x828253D4; continue 'dispatch;
	}
	// 828253F0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 828253F4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 828253F8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 828253FC: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82825400: 48657841  bl 0x82e7cc40
	ctx.lr = 0x82825404;
	sub_82E7CC40(ctx, base);
	// 82825404: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82825408: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282540C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82825410: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82825414: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82825418: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8282541C: 4BF73B15  bl 0x82798f30
	ctx.lr = 0x82825420;
	sub_82798F30(ctx, base);
	// 82825420: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825428: 4BF76241  bl 0x8279b668
	ctx.lr = 0x8282542C;
	sub_8279B668(ctx, base);
	// 8282542C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825430: 48000008  b 0x82825438
	pc = 0x82825438; continue 'dispatch;
	// 82825434: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82825438: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8282543C: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 82825440: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825448: 4BD72809  bl 0x82597c50
	ctx.lr = 0x8282544C;
	sub_82597C50(ctx, base);
	// 8282544C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82825450: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825454: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825458: 4BA9ABA9  bl 0x822c0000
	ctx.lr = 0x8282545C;
	sub_822C0000(ctx, base);
	// 8282545C: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825460: 41820024  beq 0x82825484
	if ctx.cr[0].eq {
	pc = 0x82825484; continue 'dispatch;
	}
	// 82825464: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82825468: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282546C: 419A0008  beq cr6, 0x82825474
	if ctx.cr[6].eq {
	pc = 0x82825474; continue 'dispatch;
	}
	// 82825470: 4BA9B421  bl 0x822c0890
	ctx.lr = 0x82825474;
	sub_822C0890(ctx, base);
	// 82825474: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82825478: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282547C: 419A0008  beq cr6, 0x82825484
	if ctx.cr[6].eq {
	pc = 0x82825484; continue 'dispatch;
	}
	// 82825480: 4BA9B411  bl 0x822c0890
	ctx.lr = 0x82825484;
	sub_822C0890(ctx, base);
	// 82825484: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82825488: 419A000C  beq cr6, 0x82825494
	if ctx.cr[6].eq {
	pc = 0x82825494; continue 'dispatch;
	}
	// 8282548C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82825490: 4BA9B401  bl 0x822c0890
	ctx.lr = 0x82825494;
	sub_822C0890(ctx, base);
	// 82825494: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82825498: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8282549C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 828254A0: 48982D14  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828254A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828254A8 size=1716
    let mut pc: u32 = 0x828254A8;
    'dispatch: loop {
        match pc {
            0x828254A8 => {
    //   block [0x828254A8..0x82825B5C)
	// 828254A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828254AC: 48982CAD  bl 0x831a8158
	ctx.lr = 0x828254B0;
	sub_831A8130(ctx, base);
	// 828254B0: DBC1FFA8  stfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 828254B4: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 828254B8: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828254BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828254C0: 4BFFA271  bl 0x8281f730
	ctx.lr = 0x828254C4;
	sub_8281F730(ctx, base);
	// 828254C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828254C8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 828254CC: 4BFC70FD  bl 0x827ec5c8
	ctx.lr = 0x828254D0;
	sub_827EC5C8(ctx, base);
	// 828254D0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828254D4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 828254D8: 3B4BA6E8  addi r26, r11, -0x5918
	ctx.r[26].s64 = ctx.r[11].s64 + -22808;
	// 828254DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 828254E0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 828254E4: 3B2A1EBC  addi r25, r10, 0x1ebc
	ctx.r[25].s64 = ctx.r[10].s64 + 7868;
	// 828254E8: C3E908A4  lfs f31, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 828254EC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 828254F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828254F4: 419A011C  beq cr6, 0x82825610
	if ctx.cr[6].eq {
	pc = 0x82825610; continue 'dispatch;
	}
	// 828254F8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 828254FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825500: 485CE509  bl 0x82df3a08
	ctx.lr = 0x82825504;
	sub_82DF3A08(ctx, base);
	// 82825504: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82825508: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8282550C: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82825510: 485EFB21  bl 0x82e15030
	ctx.lr = 0x82825514;
	sub_82E15030(ctx, base);
	// 82825514: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82825518: 395F02F0  addi r10, r31, 0x2f0
	ctx.r[10].s64 = ctx.r[31].s64 + 752;
	// 8282551C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82825520: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82825524: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82825528: 917F02F0  stw r11, 0x2f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[11].u32 ) };
	// 8282552C: 4BA9EF35  bl 0x822c4460
	ctx.lr = 0x82825530;
	sub_822C4460(ctx, base);
	// 82825530: 806100C4  lwz r3, 0xc4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82825534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82825538: 419A0008  beq cr6, 0x82825540
	if ctx.cr[6].eq {
	pc = 0x82825540; continue 'dispatch;
	}
	// 8282553C: 4BA9B355  bl 0x822c0890
	ctx.lr = 0x82825540;
	sub_822C0890(ctx, base);
	// 82825540: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825544: 485CDEE5  bl 0x82df3428
	ctx.lr = 0x82825548;
	sub_82DF3428(ctx, base);
	// 82825548: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8282554C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82825550: 38A000C9  li r5, 0xc9
	ctx.r[5].s64 = 201;
	// 82825554: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82825558: 485CCE91  bl 0x82df23e8
	ctx.lr = 0x8282555C;
	sub_82DF23E8(ctx, base);
	// 8282555C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82825560: 41820044  beq 0x828255a4
	if ctx.cr[0].eq {
	pc = 0x828255A4; continue 'dispatch;
	}
	// 82825564: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 82825568: D3E10084  stfs f31, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8282556C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82825570: D3E1008C  stfs f31, 0x8c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82825574: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82825578: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 8282557C: C00B8EB0  lfs f0, -0x7150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29008 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82825580: C1AA2534  lfs f13, 0x2534(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9524 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82825584: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82825588: D1A10088  stfs f13, 0x88(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8282558C: 48656A4D  bl 0x82e7bfd8
	ctx.lr = 0x82825590;
	sub_82E7BFD8(ctx, base);
	// 82825590: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825598: 485EDB59  bl 0x82e130f0
	ctx.lr = 0x8282559C;
	sub_82E130F0(ctx, base);
	// 8282559C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828255A0: 48000008  b 0x828255a8
	pc = 0x828255A8; continue 'dispatch;
	// 828255A4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 828255A8: 387F02E8  addi r3, r31, 0x2e8
	ctx.r[3].s64 = ctx.r[31].s64 + 744;
	// 828255AC: 4BABC7A5  bl 0x822e1d50
	ctx.lr = 0x828255B0;
	sub_822E1D50(ctx, base);
	// 828255B0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 828255B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828255B8: 38A000CA  li r5, 0xca
	ctx.r[5].s64 = 202;
	// 828255BC: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 828255C0: 485CCE29  bl 0x82df23e8
	ctx.lr = 0x828255C4;
	sub_82DF23E8(ctx, base);
	// 828255C4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 828255C8: 4182003C  beq 0x82825604
	if ctx.cr[0].eq {
	pc = 0x82825604; continue 'dispatch;
	}
	// 828255CC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 828255D0: D3E100A0  stfs f31, 0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 828255D4: D3E100A8  stfs f31, 0xa8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 828255D8: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 828255DC: D3E100AC  stfs f31, 0xac(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 828255E0: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 828255E4: C00B31D4  lfs f0, 0x31d4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12756 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828255E8: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 828255EC: 486569ED  bl 0x82e7bfd8
	ctx.lr = 0x828255F0;
	sub_82E7BFD8(ctx, base);
	// 828255F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828255F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828255F8: 485EDAF9  bl 0x82e130f0
	ctx.lr = 0x828255FC;
	sub_82E130F0(ctx, base);
	// 828255FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825600: 48000008  b 0x82825608
	pc = 0x82825608; continue 'dispatch;
	// 82825604: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82825608: 387F02E0  addi r3, r31, 0x2e0
	ctx.r[3].s64 = ctx.r[31].s64 + 736;
	// 8282560C: 4BABC745  bl 0x822e1d50
	ctx.lr = 0x82825610;
	sub_822E1D50(ctx, base);
	// 82825610: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82825614: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82825618: 419A0008  beq cr6, 0x82825620
	if ctx.cr[6].eq {
	pc = 0x82825620; continue 'dispatch;
	}
	// 8282561C: 4BA9B275  bl 0x822c0890
	ctx.lr = 0x82825620;
	sub_822C0890(ctx, base);
	// 82825620: 83DF02E0  lwz r30, 0x2e0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(736 as u32) ) } as u64;
	// 82825624: 3BBF02E0  addi r29, r31, 0x2e0
	ctx.r[29].s64 = ctx.r[31].s64 + 736;
	// 82825628: 3B600030  li r27, 0x30
	ctx.r[27].s64 = 48;
	// 8282562C: 3F808332  lis r28, -0x7cce
	ctx.r[28].s64 = -2093875200;
	// 82825630: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82825634: 419A0100  beq cr6, 0x82825734
	if ctx.cr[6].eq {
	pc = 0x82825734; continue 'dispatch;
	}
	// 82825638: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282563C: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82825640: 482866A9  bl 0x82aabce8
	ctx.lr = 0x82825644;
	sub_82AABCE8(ctx, base);
	// 82825644: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82825648: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282564C: 485ED7E5  bl 0x82e12e30
	ctx.lr = 0x82825650;
	sub_82E12E30(ctx, base);
	// 82825650: 806100CC  lwz r3, 0xcc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 82825654: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82825658: 419A0008  beq cr6, 0x82825660
	if ctx.cr[6].eq {
	pc = 0x82825660; continue 'dispatch;
	}
	// 8282565C: 4BA9B235  bl 0x822c0890
	ctx.lr = 0x82825660;
	sub_822C0890(ctx, base);
	// 82825660: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82825664: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82825668: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8282566C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82825670: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82825674: 4867B0BD  bl 0x82ea0730
	ctx.lr = 0x82825678;
	sub_82EA0730(ctx, base);
	// 82825678: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282567C: 39410120  addi r10, r1, 0x120
	ctx.r[10].s64 = ctx.r[1].s64 + 288;
	// 82825680: B3630004  sth r27, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 82825684: 396BA770  addi r11, r11, -0x5890
	ctx.r[11].s64 = ctx.r[11].s64 + -22672;
	// 82825688: C03CF614  lfs f1, -0x9ec(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-2540 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282568C: 38810120  addi r4, r1, 0x120
	ctx.r[4].s64 = ctx.r[1].s64 + 288;
	// 82825690: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82825B60 size=156
    let mut pc: u32 = 0x82825B60;
    'dispatch: loop {
        match pc {
            0x82825B60 => {
    //   block [0x82825B60..0x82825BFC)
	// 82825B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82825B68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82825B6C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82825B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825B78: 4BFFF931  bl 0x828254a8
	ctx.lr = 0x82825B7C;
	sub_828254A8(ctx, base);
	// 82825B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825B80: 4BFC6651  bl 0x827ec1d0
	ctx.lr = 0x82825B84;
	sub_827EC1D0(ctx, base);
	// 82825B84: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82825B88: 4182005C  beq 0x82825be4
	if ctx.cr[0].eq {
	pc = 0x82825BE4; continue 'dispatch;
	}
	// 82825B8C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82825B90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825B94: 388BA3F4  addi r4, r11, -0x5c0c
	ctx.r[4].s64 = ctx.r[11].s64 + -23564;
	// 82825B98: 485CDE71  bl 0x82df3a08
	ctx.lr = 0x82825B9C;
	sub_82DF3A08(ctx, base);
	// 82825B9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82825BA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82825BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825BA8: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82825BAC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82825BB0: 4BFC4789  bl 0x827ea338
	ctx.lr = 0x82825BB4;
	sub_827EA338(ctx, base);
	// 82825BB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825BB8: 485CD871  bl 0x82df3428
	ctx.lr = 0x82825BBC;
	sub_82DF3428(ctx, base);
	// 82825BBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82825BC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825BC4: 388B2D64  addi r4, r11, 0x2d64
	ctx.r[4].s64 = ctx.r[11].s64 + 11620;
	// 82825BC8: 485CDE41  bl 0x82df3a08
	ctx.lr = 0x82825BCC;
	sub_82DF3A08(ctx, base);
	// 82825BCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82825BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825BD4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82825BD8: 4BFC4761  bl 0x827ea338
	ctx.lr = 0x82825BDC;
	sub_827EA338(ctx, base);
	// 82825BDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825BE0: 485CD849  bl 0x82df3428
	ctx.lr = 0x82825BE4;
	sub_82DF3428(ctx, base);
	// 82825BE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82825BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82825BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82825BF0: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82825BF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82825BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82825C00 size=480
    let mut pc: u32 = 0x82825C00;
    'dispatch: loop {
        match pc {
            0x82825C00 => {
    //   block [0x82825C00..0x82825DE0)
	// 82825C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825C04: 48982569  bl 0x831a816c
	ctx.lr = 0x82825C08;
	sub_831A8130(ctx, base);
	// 82825C08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825C0C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82825C10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82825C14: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82825C18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82825C1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82825C20: 808B8644  lwz r4, -0x79bc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31164 as u32) ) } as u64;
	// 82825C24: 4BB06B05  bl 0x8232c728
	ctx.lr = 0x82825C28;
	sub_8232C728(ctx, base);
	// 82825C28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82825C2C: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82825C30: E88B0008  ld r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82825C34: 4BF9473D  bl 0x827ba370
	ctx.lr = 0x82825C38;
	sub_827BA370(ctx, base);
	// 82825C38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825C3C: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82825C40: 4BF942F9  bl 0x827b9f38
	ctx.lr = 0x82825C44;
	sub_827B9F38(ctx, base);
	// 82825C44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825C48: 41820008  beq 0x82825c50
	if ctx.cr[0].eq {
	pc = 0x82825C50; continue 'dispatch;
	}
	// 82825C4C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82825C50: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82825C54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82825C58: 808B8640  lwz r4, -0x79c0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31168 as u32) ) } as u64;
	// 82825C5C: 4BB06ACD  bl 0x8232c728
	ctx.lr = 0x82825C60;
	sub_8232C728(ctx, base);
	// 82825C60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82825C64: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82825C68: E88B0008  ld r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82825C6C: 4BF94705  bl 0x827ba370
	ctx.lr = 0x82825C70;
	sub_827BA370(ctx, base);
	// 82825C70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825C74: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82825C78: 4BF942C1  bl 0x827b9f38
	ctx.lr = 0x82825C7C;
	sub_827B9F38(ctx, base);
	// 82825C7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825C80: 41820008  beq 0x82825c88
	if ctx.cr[0].eq {
	pc = 0x82825C88; continue 'dispatch;
	}
	// 82825C84: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82825C88: 897E0308  lbz r11, 0x308(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 82825C8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82825C90: 40820008  bne 0x82825c98
	if !ctx.cr[0].eq {
	pc = 0x82825C98; continue 'dispatch;
	}
	// 82825C94: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82825C98: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825C9C: 41820014  beq 0x82825cb0
	if ctx.cr[0].eq {
	pc = 0x82825CB0; continue 'dispatch;
	}
	// 82825CA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82825CA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825CA8: 4BFFA661  bl 0x82820308
	ctx.lr = 0x82825CAC;
	sub_82820308(ctx, base);
	// 82825CAC: 480000C4  b 0x82825d70
	pc = 0x82825D70; continue 'dispatch;
	// 82825CB0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82825CB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82825CB8: 38ABA780  addi r5, r11, -0x5880
	ctx.r[5].s64 = ctx.r[11].s64 + -22656;
	// 82825CBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82825CC0: 4BFC74F1  bl 0x827ed1b0
	ctx.lr = 0x82825CC4;
	sub_827ED1B0(ctx, base);
	// 82825CC4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82825CC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82825CCC: 419A0008  beq cr6, 0x82825cd4
	if ctx.cr[6].eq {
	pc = 0x82825CD4; continue 'dispatch;
	}
	// 82825CD0: 4BA9ABC1  bl 0x822c0890
	ctx.lr = 0x82825CD4;
	sub_822C0890(ctx, base);
	// 82825CD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82825CD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825CDC: 388BA3EC  addi r4, r11, -0x5c14
	ctx.r[4].s64 = ctx.r[11].s64 + -23572;
	// 82825CE0: 485CDD29  bl 0x82df3a08
	ctx.lr = 0x82825CE4;
	sub_82DF3A08(ctx, base);
	// 82825CE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82825CE8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82825CEC: 4803D9D5  bl 0x828636c0
	ctx.lr = 0x82825CF0;
	sub_828636C0(ctx, base);
	// 82825CF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82825CF4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82825CF8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82825CFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82825D00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82825D04: 419A0024  beq cr6, 0x82825d28
	if ctx.cr[6].eq {
	pc = 0x82825D28; continue 'dispatch;
	}
	// 82825D08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82825D0C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82825D10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82825D14: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82825D18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82825D1C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82825D20: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82825D24: 4082FFE8  bne 0x82825d0c
	if !ctx.cr[0].eq {
	pc = 0x82825D0C; continue 'dispatch;
	}
	// 82825D28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82825D2C: 3BE10058  addi r31, r1, 0x58
	ctx.r[31].s64 = ctx.r[1].s64 + 88;
	// 82825D30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825D34: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82825D38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82825D3C: 4E800421  bctrl
	ctx.lr = 0x82825D40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82825D40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825D44: 4BFC8F2D  bl 0x827eec70
	ctx.lr = 0x82825D48;
	sub_827EEC70(ctx, base);
	// 82825D48: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82825D4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82825D50: 419A0008  beq cr6, 0x82825d58
	if ctx.cr[6].eq {
	pc = 0x82825D58; continue 'dispatch;
	}
	// 82825D54: 4BA9AB3D  bl 0x822c0890
	ctx.lr = 0x82825D58;
	sub_822C0890(ctx, base);
	// 82825D58: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82825D5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82825D60: 419A0008  beq cr6, 0x82825d68
	if ctx.cr[6].eq {
	pc = 0x82825D68; continue 'dispatch;
	}
	// 82825D64: 4BA9AB2D  bl 0x822c0890
	ctx.lr = 0x82825D68;
	sub_822C0890(ctx, base);
	// 82825D68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825D6C: 485CD6BD  bl 0x82df3428
	ctx.lr = 0x82825D70;
	sub_82DF3428(ctx, base);
	// 82825D70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825D74: 4BFF1875  bl 0x828175e8
	ctx.lr = 0x82825D78;
	sub_828175E8(ctx, base);
	// 82825D78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825D7C: 40820024  bne 0x82825da0
	if !ctx.cr[0].eq {
	pc = 0x82825DA0; continue 'dispatch;
	}
	// 82825D80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82825D84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825D88: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82825D8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82825D90: 4E800421  bctrl
	ctx.lr = 0x82825D94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82825D94: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825D98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825D9C: 4BFFEC45  bl 0x828249e0
	ctx.lr = 0x82825DA0;
	sub_828249E0(ctx, base);
	// 82825DA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825DA4: 4BFC6425  bl 0x827ec1c8
	ctx.lr = 0x82825DA8;
	sub_827EC1C8(ctx, base);
	// 82825DA8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82825DAC: 4182002C  beq 0x82825dd8
	if ctx.cr[0].eq {
	pc = 0x82825DD8; continue 'dispatch;
	}
	// 82825DB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82825DB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825DB8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82825DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82825DC0: 4E800421  bctrl
	ctx.lr = 0x82825DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82825DC4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82825DC8: 4BFCBAF9  bl 0x827f18c0
	ctx.lr = 0x82825DCC;
	sub_827F18C0(ctx, base);
	// 82825DCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825DD4: 4BC695CD  bl 0x8248f3a0
	ctx.lr = 0x82825DD8;
	sub_8248F3A0(ctx, base);
	// 82825DD8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82825DDC: 489823E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82825DE0 size=220
    let mut pc: u32 = 0x82825DE0;
    'dispatch: loop {
        match pc {
            0x82825DE0 => {
    //   block [0x82825DE0..0x82825EBC)
	// 82825DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825DE4: 48982385  bl 0x831a8168
	ctx.lr = 0x82825DE8;
	sub_831A8130(ctx, base);
	// 82825DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825DEC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82825DF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82825DF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82825DF8: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82825DFC: 41820038  beq 0x82825e34
	if ctx.cr[0].eq {
	pc = 0x82825E34; continue 'dispatch;
	}
	// 82825E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825E04: 48983B85  bl 0x831a9988
	ctx.lr = 0x82825E08;
	sub_831A9988(ctx, base);
	// 82825E08: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82825E0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825E10: 386B3984  addi r3, r11, 0x3984
	ctx.r[3].s64 = ctx.r[11].s64 + 14724;
	// 82825E14: 489822E5  bl 0x831a80f8
	ctx.lr = 0x82825E18;
	sub_831A80F8(ctx, base);
	// 82825E18: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825E1C: 41820018  beq 0x82825e34
	if ctx.cr[0].eq {
	pc = 0x82825E34; continue 'dispatch;
	}
	// 82825E20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825E24: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82825E28: 4BFFFDD9  bl 0x82825c00
	ctx.lr = 0x82825E2C;
	sub_82825C00(ctx, base);
	// 82825E2C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82825E30: 48000084  b 0x82825eb4
	pc = 0x82825EB4; continue 'dispatch;
	// 82825E34: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82825E38: 419A006C  beq cr6, 0x82825ea4
	if ctx.cr[6].eq {
	pc = 0x82825EA4; continue 'dispatch;
	}
	// 82825E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825E40: 48983B49  bl 0x831a9988
	ctx.lr = 0x82825E44;
	sub_831A9988(ctx, base);
	// 82825E44: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82825E48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825E4C: 386B3644  addi r3, r11, 0x3644
	ctx.r[3].s64 = ctx.r[11].s64 + 13892;
	// 82825E50: 489822A9  bl 0x831a80f8
	ctx.lr = 0x82825E54;
	sub_831A80F8(ctx, base);
	// 82825E54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825E58: 41820014  beq 0x82825e6c
	if ctx.cr[0].eq {
	pc = 0x82825E6C; continue 'dispatch;
	}
	// 82825E5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825E60: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82825E64: 4BFFDBC5  bl 0x82823a28
	ctx.lr = 0x82825E68;
	sub_82823A28(ctx, base);
	// 82825E68: 4BFFFFC4  b 0x82825e2c
	pc = 0x82825E2C; continue 'dispatch;
	// 82825E6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82825E70: 419A0034  beq cr6, 0x82825ea4
	if ctx.cr[6].eq {
	pc = 0x82825EA4; continue 'dispatch;
	}
	// 82825E74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825E78: 48983B11  bl 0x831a9988
	ctx.lr = 0x82825E7C;
	sub_831A9988(ctx, base);
	// 82825E7C: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 82825E80: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825E84: 386B0B88  addi r3, r11, 0xb88
	ctx.r[3].s64 = ctx.r[11].s64 + 2952;
	// 82825E88: 48982271  bl 0x831a80f8
	ctx.lr = 0x82825E8C;
	sub_831A80F8(ctx, base);
	// 82825E8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825E90: 41820014  beq 0x82825ea4
	if ctx.cr[0].eq {
	pc = 0x82825EA4; continue 'dispatch;
	}
	// 82825E94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825E98: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82825E9C: 4BFFDBE5  bl 0x82823a80
	ctx.lr = 0x82825EA0;
	sub_82823A80(ctx, base);
	// 82825EA0: 4BFFFF8C  b 0x82825e2c
	pc = 0x82825E2C; continue 'dispatch;
	// 82825EA4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82825EA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825EAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825EB0: 4BFFA9C9  bl 0x82820878
	ctx.lr = 0x82825EB4;
	sub_82820878(ctx, base);
	// 82825EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82825EB8: 48982300  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82825EC0 size=108
    let mut pc: u32 = 0x82825EC0;
    'dispatch: loop {
        match pc {
            0x82825EC0 => {
    //   block [0x82825EC0..0x82825F2C)
	// 82825EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825EC4: 489822A9  bl 0x831a816c
	ctx.lr = 0x82825EC8;
	sub_831A8130(ctx, base);
	// 82825EC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825ECC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82825ED0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82825ED4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82825ED8: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825EDC: 41820038  beq 0x82825f14
	if ctx.cr[0].eq {
	pc = 0x82825F14; continue 'dispatch;
	}
	// 82825EE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825EE4: 48983AA5  bl 0x831a9988
	ctx.lr = 0x82825EE8;
	sub_831A9988(ctx, base);
	// 82825EE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82825EEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82825EF0: 386BDBD4  addi r3, r11, -0x242c
	ctx.r[3].s64 = ctx.r[11].s64 + -9260;
	// 82825EF4: 48982205  bl 0x831a80f8
	ctx.lr = 0x82825EF8;
	sub_831A80F8(ctx, base);
	// 82825EF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82825EFC: 41820018  beq 0x82825f14
	if ctx.cr[0].eq {
	pc = 0x82825F14; continue 'dispatch;
	}
	// 82825F00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825F04: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 82825F08: 4BFFD8E9  bl 0x828237f0
	ctx.lr = 0x82825F0C;
	sub_828237F0(ctx, base);
	// 82825F0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82825F10: 48000014  b 0x82825f24
	pc = 0x82825F24; continue 'dispatch;
	// 82825F14: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82825F18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82825F1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82825F20: 4BFFFEC1  bl 0x82825de0
	ctx.lr = 0x82825F24;
	sub_82825DE0(ctx, base);
	// 82825F24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82825F28: 48982294  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82825F30 size=84
    let mut pc: u32 = 0x82825F30;
    'dispatch: loop {
        match pc {
            0x82825F30 => {
    //   block [0x82825F30..0x82825F84)
	// 82825F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82825F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82825F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825F40: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82825F44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825F48: 388BB04C  addi r4, r11, -0x4fb4
	ctx.r[4].s64 = ctx.r[11].s64 + -20404;
	// 82825F4C: 4BFCA8DD  bl 0x827f0828
	ctx.lr = 0x82825F50;
	sub_827F0828(ctx, base);
	// 82825F50: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82825F54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825F58: 388BB068  addi r4, r11, -0x4f98
	ctx.r[4].s64 = ctx.r[11].s64 + -20376;
	// 82825F5C: 4BFD0195  bl 0x827f60f0
	ctx.lr = 0x82825F60;
	sub_827F60F0(ctx, base);
	// 82825F60: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82825F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825F68: 388BB078  addi r4, r11, -0x4f88
	ctx.r[4].s64 = ctx.r[11].s64 + -20360;
	// 82825F6C: 4BFCAB5D  bl 0x827f0ac8
	ctx.lr = 0x82825F70;
	sub_827F0AC8(ctx, base);
	// 82825F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82825F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82825F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82825F7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82825F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82825F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82825F88 size=128
    let mut pc: u32 = 0x82825F88;
    'dispatch: loop {
        match pc {
            0x82825F88 => {
    //   block [0x82825F88..0x82826008)
	// 82825F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82825F8C: 489821E1  bl 0x831a816c
	ctx.lr = 0x82825F90;
	sub_831A8130(ctx, base);
	// 82825F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82825F94: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82825F98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82825F9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82825FA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825FA4: 388BA814  addi r4, r11, -0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + -22508;
	// 82825FA8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82825FAC: 485CDA5D  bl 0x82df3a08
	ctx.lr = 0x82825FB0;
	sub_82DF3A08(ctx, base);
	// 82825FB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82825FB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825FB8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82825FBC: 4BFC5B75  bl 0x827ebb30
	ctx.lr = 0x82825FC0;
	sub_827EBB30(ctx, base);
	// 82825FC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825FC4: 485CD465  bl 0x82df3428
	ctx.lr = 0x82825FC8;
	sub_82DF3428(ctx, base);
	// 82825FC8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82825FCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825FD0: 388BA808  addi r4, r11, -0x57f8
	ctx.r[4].s64 = ctx.r[11].s64 + -22520;
	// 82825FD4: 485CDA35  bl 0x82df3a08
	ctx.lr = 0x82825FD8;
	sub_82DF3A08(ctx, base);
	// 82825FD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82825FDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82825FE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82825FE4: 4BFC5B4D  bl 0x827ebb30
	ctx.lr = 0x82825FE8;
	sub_827EBB30(ctx, base);
	// 82825FE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82825FEC: 485CD43D  bl 0x82df3428
	ctx.lr = 0x82825FF0;
	sub_82DF3428(ctx, base);
	// 82825FF0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82825FF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82825FF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82825FFC: 4BFFD52D  bl 0x82823528
	ctx.lr = 0x82826000;
	sub_82823528(ctx, base);
	// 82826000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82826004: 489821B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826008 size=12
    let mut pc: u32 = 0x82826008;
    'dispatch: loop {
        match pc {
            0x82826008 => {
    //   block [0x82826008..0x82826014)
	// 82826008: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282600C: 386BAE84  addi r3, r11, -0x517c
	ctx.r[3].s64 = ctx.r[11].s64 + -20860;
	// 82826010: 4BFF5F20  b 0x8281bf30
	sub_8281BF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826018 size=12
    let mut pc: u32 = 0x82826018;
    'dispatch: loop {
        match pc {
            0x82826018 => {
    //   block [0x82826018..0x82826024)
	// 82826018: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282601C: 386BAE84  addi r3, r11, -0x517c
	ctx.r[3].s64 = ctx.r[11].s64 + -20860;
	// 82826020: 4BFF5F18  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826028 size=12
    let mut pc: u32 = 0x82826028;
    'dispatch: loop {
        match pc {
            0x82826028 => {
    //   block [0x82826028..0x82826034)
	// 82826028: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282602C: 386BB068  addi r3, r11, -0x4f98
	ctx.r[3].s64 = ctx.r[11].s64 + -20376;
	// 82826030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826038 size=12
    let mut pc: u32 = 0x82826038;
    'dispatch: loop {
        match pc {
            0x82826038 => {
    //   block [0x82826038..0x82826044)
	// 82826038: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282603C: 386BB04C  addi r3, r11, -0x4fb4
	ctx.r[3].s64 = ctx.r[11].s64 + -20404;
	// 82826040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826048 size=12
    let mut pc: u32 = 0x82826048;
    'dispatch: loop {
        match pc {
            0x82826048 => {
    //   block [0x82826048..0x82826054)
	// 82826048: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282604C: 386BB078  addi r3, r11, -0x4f88
	ctx.r[3].s64 = ctx.r[11].s64 + -20360;
	// 82826050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826058 size=4
    let mut pc: u32 = 0x82826058;
    'dispatch: loop {
        match pc {
            0x82826058 => {
    //   block [0x82826058..0x8282605C)
	// 82826058: 4BFFEC28  b 0x82824c80
	sub_82824C80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826060 size=4
    let mut pc: u32 = 0x82826060;
    'dispatch: loop {
        match pc {
            0x82826060 => {
    //   block [0x82826060..0x82826064)
	// 82826060: 4BFFCDC0  b 0x82822e20
	sub_82822E20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826068 size=4
    let mut pc: u32 = 0x82826068;
    'dispatch: loop {
        match pc {
            0x82826068 => {
    //   block [0x82826068..0x8282606C)
	// 82826068: 4BFFF208  b 0x82825270
	sub_82825270(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826070 size=92
    let mut pc: u32 = 0x82826070;
    'dispatch: loop {
        match pc {
            0x82826070 => {
    //   block [0x82826070..0x828260CC)
	// 82826070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82826074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282607C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82826080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82826088: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282608C: 4BFFD525  bl 0x828235b0
	ctx.lr = 0x82826090;
	sub_828235B0(ctx, base);
	// 82826090: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82826094: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82826098: 396B0314  addi r11, r11, 0x314
	ctx.r[11].s64 = ctx.r[11].s64 + 788;
	// 8282609C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828260A0: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 828260A4: 4BFF2C05  bl 0x82818ca8
	ctx.lr = 0x828260A8;
	sub_82818CA8(ctx, base);
	// 828260A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828260AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828260B0: 4BFF61D1  bl 0x8281c280
	ctx.lr = 0x828260B4;
	sub_8281C280(ctx, base);
	// 828260B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828260B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828260BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828260C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828260C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828260C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828260D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828260D0 size=136
    let mut pc: u32 = 0x828260D0;
    'dispatch: loop {
        match pc {
            0x828260D0 => {
    //   block [0x828260D0..0x82826158)
	// 828260D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828260D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828260D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828260DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828260E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828260E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828260E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828260EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 828260F0: 409A0020  bne cr6, 0x82826110
	if !ctx.cr[6].eq {
	pc = 0x82826110; continue 'dispatch;
	}
	// 828260F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828260F8: 419A0048  beq cr6, 0x82826140
	if ctx.cr[6].eq {
	pc = 0x82826140; continue 'dispatch;
	}
	// 828260FC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82826100: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82826104: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82826108: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8282610C: 48000034  b 0x82826140
	pc = 0x82826140; continue 'dispatch;
	// 82826110: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82826114: 419A002C  beq cr6, 0x82826140
	if ctx.cr[6].eq {
	pc = 0x82826140; continue 'dispatch;
	}
	// 82826118: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282611C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826120: 388B0328  addi r4, r11, 0x328
	ctx.r[4].s64 = ctx.r[11].s64 + 808;
	// 82826124: 48981FD5  bl 0x831a80f8
	ctx.lr = 0x82826128;
	sub_831A80F8(ctx, base);
	// 82826128: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282612C: 4182000C  beq 0x82826138
	if ctx.cr[0].eq {
	pc = 0x82826138; continue 'dispatch;
	}
	// 82826130: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82826134: 4800000C  b 0x82826140
	pc = 0x82826140; continue 'dispatch;
	// 82826138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8282613C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82826140: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82826144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82826148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282614C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82826150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826158 size=136
    let mut pc: u32 = 0x82826158;
    'dispatch: loop {
        match pc {
            0x82826158 => {
    //   block [0x82826158..0x828261E0)
	// 82826158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282615C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82826164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82826168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282616C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82826170: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82826174: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82826178: 409A0020  bne cr6, 0x82826198
	if !ctx.cr[6].eq {
	pc = 0x82826198; continue 'dispatch;
	}
	// 8282617C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82826180: 419A0048  beq cr6, 0x828261c8
	if ctx.cr[6].eq {
	pc = 0x828261C8; continue 'dispatch;
	}
	// 82826184: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82826188: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8282618C: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82826190: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82826194: 48000034  b 0x828261c8
	pc = 0x828261C8; continue 'dispatch;
	// 82826198: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8282619C: 419A002C  beq cr6, 0x828261c8
	if ctx.cr[6].eq {
	pc = 0x828261C8; continue 'dispatch;
	}
	// 828261A0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 828261A4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828261A8: 388B03C0  addi r4, r11, 0x3c0
	ctx.r[4].s64 = ctx.r[11].s64 + 960;
	// 828261AC: 48981F4D  bl 0x831a80f8
	ctx.lr = 0x828261B0;
	sub_831A80F8(ctx, base);
	// 828261B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828261B4: 4182000C  beq 0x828261c0
	if ctx.cr[0].eq {
	pc = 0x828261C0; continue 'dispatch;
	}
	// 828261B8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 828261BC: 4800000C  b 0x828261c8
	pc = 0x828261C8; continue 'dispatch;
	// 828261C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828261C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828261C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828261CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828261D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828261D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828261D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828261DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828261E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828261E0 size=108
    let mut pc: u32 = 0x828261E0;
    'dispatch: loop {
        match pc {
            0x828261E0 => {
    //   block [0x828261E0..0x8282624C)
	// 828261E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828261E4: 48981F89  bl 0x831a816c
	ctx.lr = 0x828261E8;
	sub_831A8130(ctx, base);
	// 828261E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828261EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 828261F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828261F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828261F8: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828261FC: 41820038  beq 0x82826234
	if ctx.cr[0].eq {
	pc = 0x82826234; continue 'dispatch;
	}
	// 82826200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826204: 48983785  bl 0x831a9988
	ctx.lr = 0x82826208;
	sub_831A9988(ctx, base);
	// 82826208: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282620C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82826210: 386BDBD4  addi r3, r11, -0x242c
	ctx.r[3].s64 = ctx.r[11].s64 + -9260;
	// 82826214: 48981EE5  bl 0x831a80f8
	ctx.lr = 0x82826218;
	sub_831A80F8(ctx, base);
	// 82826218: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282621C: 41820018  beq 0x82826234
	if ctx.cr[0].eq {
	pc = 0x82826234; continue 'dispatch;
	}
	// 82826220: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82826224: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 82826228: 4BFFFE49  bl 0x82826070
	ctx.lr = 0x8282622C;
	sub_82826070(ctx, base);
	// 8282622C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82826230: 48000014  b 0x82826244
	pc = 0x82826244; continue 'dispatch;
	// 82826234: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82826238: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282623C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826240: 4BFFFBA1  bl 0x82825de0
	ctx.lr = 0x82826244;
	sub_82825DE0(ctx, base);
	// 82826244: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82826248: 48981F74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826250 size=72
    let mut pc: u32 = 0x82826250;
    'dispatch: loop {
        match pc {
            0x82826250 => {
    //   block [0x82826250..0x82826298)
	// 82826250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82826254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282625C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82826260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82826268: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282626C: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 82826270: 48156889  bl 0x8297caf8
	ctx.lr = 0x82826274;
	sub_8297CAF8(ctx, base);
	// 82826274: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82826278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282627C: 4BFFD765  bl 0x828239e0
	ctx.lr = 0x82826280;
	sub_828239E0(ctx, base);
	// 82826280: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82826284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82826288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282628C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82826290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82826298 size=108
    let mut pc: u32 = 0x82826298;
    'dispatch: loop {
        match pc {
            0x82826298 => {
    //   block [0x82826298..0x82826304)
	// 82826298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282629C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828262A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828262A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828262A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 828262AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 828262B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828262B4: C04B959C  lfs f2, -0x6a64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 828262B8: C02AD7BC  lfs f1, -0x2844(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828262BC: 4BFFEC65  bl 0x82824f20
	ctx.lr = 0x828262C0;
	sub_82824F20(ctx, base);
	// 828262C0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828262C4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 828262C8: 396BA89C  addi r11, r11, -0x5764
	ctx.r[11].s64 = ctx.r[11].s64 + -22372;
	// 828262CC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 828262D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828262D4: 394AA888  addi r10, r10, -0x5778
	ctx.r[10].s64 = ctx.r[10].s64 + -22392;
	// 828262D8: 3929A83C  addi r9, r9, -0x57c4
	ctx.r[9].s64 = ctx.r[9].s64 + -22468;
	// 828262DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828262E0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 828262E4: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 828262E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828262EC: 917F0320  stw r11, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[11].u32 ) };
	// 828262F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 828262F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828262F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828262FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826308 size=8
    let mut pc: u32 = 0x82826308;
    'dispatch: loop {
        match pc {
            0x82826308 => {
    //   block [0x82826308..0x82826310)
	// 82826308: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 8282630C: 480001DC  b 0x828264e8
	sub_828264E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82826310 size=8
    let mut pc: u32 = 0x82826310;
    'dispatch: loop {
        match pc {
            0x82826310 => {
    //   block [0x82826310..0x82826318)
	// 82826310: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82826314: 480001D4  b 0x828264e8
	sub_828264E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826318 size=116
    let mut pc: u32 = 0x82826318;
    'dispatch: loop {
        match pc {
            0x82826318 => {
    //   block [0x82826318..0x8282638C)
	// 82826318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282631C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82826324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282632C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826330: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82826334: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82826338: 396BA89C  addi r11, r11, -0x5764
	ctx.r[11].s64 = ctx.r[11].s64 + -22372;
	// 8282633C: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 82826340: 394AA888  addi r10, r10, -0x5778
	ctx.r[10].s64 = ctx.r[10].s64 + -22392;
	// 82826344: 3929A83C  addi r9, r9, -0x57c4
	ctx.r[9].s64 = ctx.r[9].s64 + -22468;
	// 82826348: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282634C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82826350: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82826354: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82826358: 419A0018  beq cr6, 0x82826370
	if ctx.cr[6].eq {
	pc = 0x82826370; continue 'dispatch;
	}
	// 8282635C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826360: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82826364: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282636C: 4E800421  bctrl
	ctx.lr = 0x82826370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82826370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826374: 4BFFDA85  bl 0x82823df8
	ctx.lr = 0x82826378;
	sub_82823DF8(ctx, base);
	// 82826378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282637C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82826380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82826384: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826390 size=196
    let mut pc: u32 = 0x82826390;
    'dispatch: loop {
        match pc {
            0x82826390 => {
    //   block [0x82826390..0x82826454)
	// 82826390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82826394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282639C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828263A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828263A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828263A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828263AC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 828263B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828263B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828263B8: 4BA9A581  bl 0x822c0938
	ctx.lr = 0x828263BC;
	sub_822C0938(ctx, base);
	// 828263BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828263C0: 41820028  beq 0x828263e8
	if ctx.cr[0].eq {
	pc = 0x828263E8; continue 'dispatch;
	}
	// 828263C4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828263C8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 828263CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 828263D0: 392BA824  addi r9, r11, -0x57dc
	ctx.r[9].s64 = ctx.r[11].s64 + -22492;
	// 828263D4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 828263D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 828263DC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 828263E0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 828263E4: 48000008  b 0x828263ec
	pc = 0x828263EC; continue 'dispatch;
	// 828263E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828263EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828263F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828263F4: 409A0044  bne cr6, 0x82826438
	if !ctx.cr[6].eq {
	pc = 0x82826438; continue 'dispatch;
	}
	// 828263F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828263FC: 419A001C  beq cr6, 0x82826418
	if ctx.cr[6].eq {
	pc = 0x82826418; continue 'dispatch;
	}
	// 82826400: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826404: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82826408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282640C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826410: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82826414: 4E800421  bctrl
	ctx.lr = 0x82826418;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82826418: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282641C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82826420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826424: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82826428: 816B0320  lwz r11, 0x320(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(800 as u32) ) } as u64;
	// 8282642C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82826430: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82826434: 4BA99BCD  bl 0x822c0000
	ctx.lr = 0x82826438;
	sub_822C0000(ctx, base);
	// 82826438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282643C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82826440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82826444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82826448: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282644C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826458 size=72
    let mut pc: u32 = 0x82826458;
    'dispatch: loop {
        match pc {
            0x82826458 => {
    //   block [0x82826458..0x828264A0)
	// 82826458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282645C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826464: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82826468: 419A001C  beq cr6, 0x82826484
	if ctx.cr[6].eq {
	pc = 0x82826484; continue 'dispatch;
	}
	// 8282646C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82826470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82826474: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82826478: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8282647C: 4BFFFC55  bl 0x828260d0
	ctx.lr = 0x82826480;
	sub_828260D0(ctx, base);
	// 82826480: 48000010  b 0x82826490
	pc = 0x82826490; continue 'dispatch;
	// 82826484: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82826488: 396B0328  addi r11, r11, 0x328
	ctx.r[11].s64 = ctx.r[11].s64 + 808;
	// 8282648C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82826490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82826494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82826498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282649C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828264A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828264A0 size=72
    let mut pc: u32 = 0x828264A0;
    'dispatch: loop {
        match pc {
            0x828264A0 => {
    //   block [0x828264A0..0x828264E8)
	// 828264A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828264A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828264A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828264AC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 828264B0: 419A001C  beq cr6, 0x828264cc
	if ctx.cr[6].eq {
	pc = 0x828264CC; continue 'dispatch;
	}
	// 828264B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 828264B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 828264BC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 828264C0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828264C4: 4BFFFC95  bl 0x82826158
	ctx.lr = 0x828264C8;
	sub_82826158(ctx, base);
	// 828264C8: 48000010  b 0x828264d8
	pc = 0x828264D8; continue 'dispatch;
	// 828264CC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 828264D0: 396B03C0  addi r11, r11, 0x3c0
	ctx.r[11].s64 = ctx.r[11].s64 + 960;
	// 828264D4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828264D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 828264DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828264E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828264E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828264E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828264E8 size=76
    let mut pc: u32 = 0x828264E8;
    'dispatch: loop {
        match pc {
            0x828264E8 => {
    //   block [0x828264E8..0x82826534)
	// 828264E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828264EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828264F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828264F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828264F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828264FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82826500: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82826504: 4BFFFE15  bl 0x82826318
	ctx.lr = 0x82826508;
	sub_82826318(ctx, base);
	// 82826508: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282650C: 4182000C  beq 0x82826518
	if ctx.cr[0].eq {
	pc = 0x82826518; continue 'dispatch;
	}
	// 82826510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826514: 485CBEC5  bl 0x82df23d8
	ctx.lr = 0x82826518;
	sub_82DF23D8(ctx, base);
	// 82826518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282651C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82826520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82826524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82826528: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282652C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82826538 size=304
    let mut pc: u32 = 0x82826538;
    'dispatch: loop {
        match pc {
            0x82826538 => {
    //   block [0x82826538..0x82826668)
	// 82826538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282653C: 48981C2D  bl 0x831a8168
	ctx.lr = 0x82826540;
	sub_831A8130(ctx, base);
	// 82826540: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82826544: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826548: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282654C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82826550: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82826554: 4BFC5C7D  bl 0x827ec1d0
	ctx.lr = 0x82826558;
	sub_827EC1D0(ctx, base);
	// 82826558: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282655C: 41820088  beq 0x828265e4
	if ctx.cr[0].eq {
	pc = 0x828265E4; continue 'dispatch;
	}
	// 82826560: 4BFC3D49  bl 0x827ea2a8
	ctx.lr = 0x82826564;
	sub_827EA2A8(ctx, base);
	// 82826564: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826568: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282656C: 3BCBA808  addi r30, r11, -0x57f8
	ctx.r[30].s64 = ctx.r[11].s64 + -22520;
	// 82826570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82826574: 485CD255  bl 0x82df37c8
	ctx.lr = 0x82826578;
	sub_82DF37C8(ctx, base);
	// 82826578: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282657C: 41820068  beq 0x828265e4
	if ctx.cr[0].eq {
	pc = 0x828265E4; continue 'dispatch;
	}
	// 82826580: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82826584: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826588: 485CD481  bl 0x82df3a08
	ctx.lr = 0x8282658C;
	sub_82DF3A08(ctx, base);
	// 8282658C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82826590: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82826594: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82826598: 4838C631  bl 0x82bb2bc8
	ctx.lr = 0x8282659C;
	sub_82BB2BC8(ctx, base);
	// 8282659C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828265A0: 48390499  bl 0x82bb6a38
	ctx.lr = 0x828265A4;
	sub_82BB6A38(ctx, base);
	// 828265A4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828265A8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 828265AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828265B0: 419A0008  beq cr6, 0x828265b8
	if ctx.cr[6].eq {
	pc = 0x828265B8; continue 'dispatch;
	}
	// 828265B4: 4BA9A2DD  bl 0x822c0890
	ctx.lr = 0x828265B8;
	sub_822C0890(ctx, base);
	// 828265B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828265BC: 485CCE6D  bl 0x82df3428
	ctx.lr = 0x828265C0;
	sub_82DF3428(ctx, base);
	// 828265C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 828265C4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828265C8: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 828265CC: 40990018  ble cr6, 0x828265e4
	if !ctx.cr[6].gt {
	pc = 0x828265E4; continue 'dispatch;
	}
	// 828265D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 828265D4: C00B9590  lfs f0, -0x6a70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828265D8: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 828265DC: 40980008  bge cr6, 0x828265e4
	if !ctx.cr[6].lt {
	pc = 0x828265E4; continue 'dispatch;
	}
	// 828265E0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 828265E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828265E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828265EC: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 828265F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828265F4: 4E800421  bctrl
	ctx.lr = 0x828265F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828265F8: 4BFC7239  bl 0x827ed830
	ctx.lr = 0x828265FC;
	sub_827ED830(ctx, base);
	// 828265FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826600: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82826604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826608: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8282660C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82826610: 4E800421  bctrl
	ctx.lr = 0x82826614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82826614: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82826618: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282661C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82826620: 4BFC83C9  bl 0x827ee9e8
	ctx.lr = 0x82826624;
	sub_827EE9E8(ctx, base);
	// 82826624: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826628: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 8282662C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826630: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82826634: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82826638: 4E800421  bctrl
	ctx.lr = 0x8282663C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282663C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82826640: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82826644: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82826648: 4BFC7AD1  bl 0x827ee118
	ctx.lr = 0x8282664C;
	sub_827EE118(ctx, base);
	// 8282664C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82826650: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82826654: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 82826658: 48156B79  bl 0x8297d1d0
	ctx.lr = 0x8282665C;
	sub_8297D1D0(ctx, base);
	// 8282665C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82826660: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82826664: 48981B54  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826668 size=320
    let mut pc: u32 = 0x82826668;
    'dispatch: loop {
        match pc {
            0x82826668 => {
    //   block [0x82826668..0x828267A8)
	// 82826668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82826674: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826678: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282667C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826680: 4BFC8561  bl 0x827eebe0
	ctx.lr = 0x82826684;
	sub_827EEBE0(ctx, base);
	// 82826684: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282668C: 388BA808  addi r4, r11, -0x57f8
	ctx.r[4].s64 = ctx.r[11].s64 + -22520;
	// 82826690: 485CD379  bl 0x82df3a08
	ctx.lr = 0x82826694;
	sub_82DF3A08(ctx, base);
	// 82826694: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826698: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8282669C: 4803D025  bl 0x828636c0
	ctx.lr = 0x828266A0;
	sub_828636C0(ctx, base);
	// 828266A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828266A4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 828266A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828266AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828266B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 828266B4: 419A0024  beq cr6, 0x828266d8
	if ctx.cr[6].eq {
	pc = 0x828266D8; continue 'dispatch;
	}
	// 828266B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828266BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828266C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828266C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828266C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828266CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828266D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828266D4: 4082FFE8  bne 0x828266bc
	if !ctx.cr[0].eq {
	pc = 0x828266BC; continue 'dispatch;
	}
	// 828266D8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 828266DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828266E0: 4BFC8FC1  bl 0x827ef6a0
	ctx.lr = 0x828266E4;
	sub_827EF6A0(ctx, base);
	// 828266E4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828266E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828266EC: 419A0008  beq cr6, 0x828266f4
	if ctx.cr[6].eq {
	pc = 0x828266F4; continue 'dispatch;
	}
	// 828266F0: 4BA9A1A1  bl 0x822c0890
	ctx.lr = 0x828266F4;
	sub_822C0890(ctx, base);
	// 828266F4: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 828266F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828266FC: 419A0008  beq cr6, 0x82826704
	if ctx.cr[6].eq {
	pc = 0x82826704; continue 'dispatch;
	}
	// 82826700: 4BA9A191  bl 0x822c0890
	ctx.lr = 0x82826704;
	sub_822C0890(ctx, base);
	// 82826704: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826708: 485CCD21  bl 0x82df3428
	ctx.lr = 0x8282670C;
	sub_82DF3428(ctx, base);
	// 8282670C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826710: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826714: 388BA814  addi r4, r11, -0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + -22508;
	// 82826718: 485CD2F1  bl 0x82df3a08
	ctx.lr = 0x8282671C;
	sub_82DF3A08(ctx, base);
	// 8282671C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826720: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82826724: 4803CF9D  bl 0x828636c0
	ctx.lr = 0x82826728;
	sub_828636C0(ctx, base);
	// 82826728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282672C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82826730: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82826734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82826738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8282673C: 419A0024  beq cr6, 0x82826760
	if ctx.cr[6].eq {
	pc = 0x82826760; continue 'dispatch;
	}
	// 82826740: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82826744: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82826748: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282674C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82826750: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82826754: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82826758: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282675C: 4082FFE8  bne 0x82826744
	if !ctx.cr[0].eq {
	pc = 0x82826744; continue 'dispatch;
	}
	// 82826760: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82826764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826768: 4BFC8F39  bl 0x827ef6a0
	ctx.lr = 0x8282676C;
	sub_827EF6A0(ctx, base);
	// 8282676C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82826770: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82826774: 419A0008  beq cr6, 0x8282677c
	if ctx.cr[6].eq {
	pc = 0x8282677C; continue 'dispatch;
	}
	// 82826778: 4BA9A119  bl 0x822c0890
	ctx.lr = 0x8282677C;
	sub_822C0890(ctx, base);
	// 8282677C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82826780: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82826784: 419A0008  beq cr6, 0x8282678c
	if ctx.cr[6].eq {
	pc = 0x8282678C; continue 'dispatch;
	}
	// 82826788: 4BA9A109  bl 0x822c0890
	ctx.lr = 0x8282678C;
	sub_822C0890(ctx, base);
	// 8282678C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826790: 485CCC99  bl 0x82df3428
	ctx.lr = 0x82826794;
	sub_82DF3428(ctx, base);
	// 82826794: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82826798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282679C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828267A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828267A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828267A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828267A8 size=176
    let mut pc: u32 = 0x828267A8;
    'dispatch: loop {
        match pc {
            0x828267A8 => {
    //   block [0x828267A8..0x82826858)
	// 828267A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828267AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828267B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828267B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828267B8: 48139DA9  bl 0x82960560
	ctx.lr = 0x828267BC;
	sub_82960560(ctx, base);
	// 828267BC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 828267C0: 41820084  beq 0x82826844
	if ctx.cr[0].eq {
	pc = 0x82826844; continue 'dispatch;
	}
	// 828267C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 828267C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828267CC: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 828267D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 828267D4: 4BFFDF45  bl 0x82824718
	ctx.lr = 0x828267D8;
	sub_82824718(ctx, base);
	// 828267D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828267DC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 828267E0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828267E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828267E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 828267EC: 419A0024  beq cr6, 0x82826810
	if ctx.cr[6].eq {
	pc = 0x82826810; continue 'dispatch;
	}
	// 828267F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828267F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828267F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828267FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82826800: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82826804: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82826808: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282680C: 4082FFE8  bne 0x828267f4
	if !ctx.cr[0].eq {
	pc = 0x828267F4; continue 'dispatch;
	}
	// 82826810: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826814: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82826818: 388BA938  addi r4, r11, -0x56c8
	ctx.r[4].s64 = ctx.r[11].s64 + -22216;
	// 8282681C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826820: 4BFF3C99  bl 0x8281a4b8
	ctx.lr = 0x82826824;
	sub_8281A4B8(ctx, base);
	// 82826824: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82826828: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282682C: 419A0008  beq cr6, 0x82826834
	if ctx.cr[6].eq {
	pc = 0x82826834; continue 'dispatch;
	}
	// 82826830: 4BA9A061  bl 0x822c0890
	ctx.lr = 0x82826834;
	sub_822C0890(ctx, base);
	// 82826834: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82826838: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282683C: 419A0008  beq cr6, 0x82826844
	if ctx.cr[6].eq {
	pc = 0x82826844; continue 'dispatch;
	}
	// 82826840: 4BA9A051  bl 0x822c0890
	ctx.lr = 0x82826844;
	sub_822C0890(ctx, base);
	// 82826844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82826848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282684C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82826850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826858 size=176
    let mut pc: u32 = 0x82826858;
    'dispatch: loop {
        match pc {
            0x82826858 => {
    //   block [0x82826858..0x82826908)
	// 82826858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282685C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82826860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82826864: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826868: 48139CF9  bl 0x82960560
	ctx.lr = 0x8282686C;
	sub_82960560(ctx, base);
	// 8282686C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82826870: 41820084  beq 0x828268f4
	if ctx.cr[0].eq {
	pc = 0x828268F4; continue 'dispatch;
	}
	// 82826874: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82826878: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282687C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82826880: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82826884: 4BFFDE95  bl 0x82824718
	ctx.lr = 0x82826888;
	sub_82824718(ctx, base);
	// 82826888: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282688C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82826890: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82826894: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82826898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8282689C: 419A0024  beq cr6, 0x828268c0
	if ctx.cr[6].eq {
	pc = 0x828268C0; continue 'dispatch;
	}
	// 828268A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828268A4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828268A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828268AC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828268B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828268B4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828268B8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828268BC: 4082FFE8  bne 0x828268a4
	if !ctx.cr[0].eq {
	pc = 0x828268A4; continue 'dispatch;
	}
	// 828268C0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828268C4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 828268C8: 388BA938  addi r4, r11, -0x56c8
	ctx.r[4].s64 = ctx.r[11].s64 + -22216;
	// 828268CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828268D0: 4BFF3BE9  bl 0x8281a4b8
	ctx.lr = 0x828268D4;
	sub_8281A4B8(ctx, base);
	// 828268D4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828268D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828268DC: 419A0008  beq cr6, 0x828268e4
	if ctx.cr[6].eq {
	pc = 0x828268E4; continue 'dispatch;
	}
	// 828268E0: 4BA99FB1  bl 0x822c0890
	ctx.lr = 0x828268E4;
	sub_822C0890(ctx, base);
	// 828268E4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 828268E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828268EC: 419A0008  beq cr6, 0x828268f4
	if ctx.cr[6].eq {
	pc = 0x828268F4; continue 'dispatch;
	}
	// 828268F0: 4BA99FA1  bl 0x822c0890
	ctx.lr = 0x828268F4;
	sub_822C0890(ctx, base);
	// 828268F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 828268F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828268FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82826900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82826904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82826908 size=476
    let mut pc: u32 = 0x82826908;
    'dispatch: loop {
        match pc {
            0x82826908 => {
    //   block [0x82826908..0x82826AE4)
	// 82826908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282690C: 48981859  bl 0x831a8164
	ctx.lr = 0x82826910;
	sub_831A8130(ctx, base);
	// 82826910: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82826914: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826918: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282691C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82826920: 3BCBA940  addi r30, r11, -0x56c0
	ctx.r[30].s64 = ctx.r[11].s64 + -22208;
	// 82826924: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82826928: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282692C: 38A00048  li r5, 0x48
	ctx.r[5].s64 = 72;
	// 82826930: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82826934: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82826938: 38600330  li r3, 0x330
	ctx.r[3].s64 = 816;
	// 8282693C: 485CBAAD  bl 0x82df23e8
	ctx.lr = 0x82826940;
	sub_82DF23E8(ctx, base);
	// 82826940: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82826944: 41820010  beq 0x82826954
	if ctx.cr[0].eq {
	pc = 0x82826954; continue 'dispatch;
	}
	// 82826948: 4BFFF951  bl 0x82826298
	ctx.lr = 0x8282694C;
	sub_82826298(ctx, base);
	// 8282694C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82826950: 48000008  b 0x82826958
	pc = 0x82826958; continue 'dispatch;
	// 82826954: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82826958: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8282695C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82826960: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82826964: 4BFFFA2D  bl 0x82826390
	ctx.lr = 0x82826968;
	sub_82826390(ctx, base);
	// 82826968: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282696C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82826970: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82826974: 4BA9968D  bl 0x822c0000
	ctx.lr = 0x82826978;
	sub_822C0000(ctx, base);
	// 82826978: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282697C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82826980: 38A00049  li r5, 0x49
	ctx.r[5].s64 = 73;
	// 82826984: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82826988: 4BA99A51  bl 0x822c03d8
	ctx.lr = 0x8282698C;
	sub_822C03D8(ctx, base);
	// 8282698C: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82826990: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82826994: 418200E0  beq 0x82826a74
	if ctx.cr[0].eq {
	pc = 0x82826A74; continue 'dispatch;
	}
	// 82826998: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282699C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828269A0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 828269A4: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 828269A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 828269AC: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 828269B0: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 828269B4: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828269B8: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 828269BC: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 828269C0: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 828269C4: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 828269C8: 409A0008  bne cr6, 0x828269d0
	if !ctx.cr[6].eq {
	pc = 0x828269D0; continue 'dispatch;
	}
	// 828269CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828269D0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 828269D4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828269D8: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 828269DC: 419A0024  beq cr6, 0x82826a00
	if ctx.cr[6].eq {
	pc = 0x82826A00; continue 'dispatch;
	}
	// 828269E0: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828269E4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 828269E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828269EC: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 828269F0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 828269F4: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828269F8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828269FC: 4082FFE8  bne 0x828269e4
	if !ctx.cr[0].eq {
	pc = 0x828269E4; continue 'dispatch;
	}
	// 82826A00: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82826A04: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82826A08: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82826A0C: 419A0024  beq cr6, 0x82826a30
	if ctx.cr[6].eq {
	pc = 0x82826A30; continue 'dispatch;
	}
	// 82826A10: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82826A14: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82826A18: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82826A1C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82826A20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82826A24: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82826A28: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82826A2C: 4082FFE8  bne 0x82826a14
	if !ctx.cr[0].eq {
	pc = 0x82826A14; continue 'dispatch;
	}
	// 82826A30: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82826A34: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82826A38: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82826A3C: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82826A40: 48656201  bl 0x82e7cc40
	ctx.lr = 0x82826A44;
	sub_82E7CC40(ctx, base);
	// 82826A44: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82826A48: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82826A4C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82826A50: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82826A54: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82826A58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82826A5C: 4BF724D5  bl 0x82798f30
	ctx.lr = 0x82826A60;
	sub_82798F30(ctx, base);
	// 82826A60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82826A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826A68: 4BF74C01  bl 0x8279b668
	ctx.lr = 0x82826A6C;
	sub_8279B668(ctx, base);
	// 82826A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82826A70: 48000008  b 0x82826a78
	pc = 0x82826A78; continue 'dispatch;
	// 82826A74: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82826A78: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82826A7C: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 82826A80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82826A84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82826A88: 4BD711C9  bl 0x82597c50
	ctx.lr = 0x82826A8C;
	sub_82597C50(ctx, base);
	// 82826A8C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82826A90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82826A94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82826A98: 4BA99569  bl 0x822c0000
	ctx.lr = 0x82826A9C;
	sub_822C0000(ctx, base);
	// 82826A9C: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82826AA0: 41820024  beq 0x82826ac4
	if ctx.cr[0].eq {
	pc = 0x82826AC4; continue 'dispatch;
	}
	// 82826AA4: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82826AA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82826AAC: 419A0008  beq cr6, 0x82826ab4
	if ctx.cr[6].eq {
	pc = 0x82826AB4; continue 'dispatch;
	}
	// 82826AB0: 4BA99DE1  bl 0x822c0890
	ctx.lr = 0x82826AB4;
	sub_822C0890(ctx, base);
	// 82826AB4: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82826AB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82826ABC: 419A0008  beq cr6, 0x82826ac4
	if ctx.cr[6].eq {
	pc = 0x82826AC4; continue 'dispatch;
	}
	// 82826AC0: 4BA99DD1  bl 0x822c0890
	ctx.lr = 0x82826AC4;
	sub_822C0890(ctx, base);
	// 82826AC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82826AC8: 419A000C  beq cr6, 0x82826ad4
	if ctx.cr[6].eq {
	pc = 0x82826AD4; continue 'dispatch;
	}
	// 82826ACC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826AD0: 4BA99DC1  bl 0x822c0890
	ctx.lr = 0x82826AD4;
	sub_822C0890(ctx, base);
	// 82826AD4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82826AD8: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82826ADC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82826AE0: 489816D4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826AE8 size=128
    let mut pc: u32 = 0x82826AE8;
    'dispatch: loop {
        match pc {
            0x82826AE8 => {
    //   block [0x82826AE8..0x82826B68)
	// 82826AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82826AEC: 48981681  bl 0x831a816c
	ctx.lr = 0x82826AF0;
	sub_831A8130(ctx, base);
	// 82826AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826AF4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82826AF8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82826AFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82826B00: 3BEBB030  addi r31, r11, -0x4fd0
	ctx.r[31].s64 = ctx.r[11].s64 + -20432;
	// 82826B04: 816AB038  lwz r11, -0x4fc8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20424 as u32) ) } as u64;
	// 82826B08: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82826B0C: 40820024  bne 0x82826b30
	if !ctx.cr[0].eq {
	pc = 0x82826B30; continue 'dispatch;
	}
	// 82826B10: 3D208271  lis r9, -0x7d8f
	ctx.r[9].s64 = -2106523648;
	// 82826B14: 3D008282  lis r8, -0x7d7e
	ctx.r[8].s64 = -2105409536;
	// 82826B18: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82826B1C: 39298E48  addi r9, r9, -0x71b8
	ctx.r[9].s64 = ctx.r[9].s64 + -29112;
	// 82826B20: 39086458  addi r8, r8, 0x6458
	ctx.r[8].s64 = ctx.r[8].s64 + 25688;
	// 82826B24: 916AB038  stw r11, -0x4fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20424 as u32), ctx.r[11].u32 ) };
	// 82826B28: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82826B2C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82826B30: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82826B34: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82826B38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826B3C: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82826B40: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82826B44: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82826B48: 4BE2DA79  bl 0x826545c0
	ctx.lr = 0x82826B4C;
	sub_826545C0(ctx, base);
	// 82826B4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82826B50: 4182000C  beq 0x82826b5c
	if ctx.cr[0].eq {
	pc = 0x82826B5C; continue 'dispatch;
	}
	// 82826B54: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82826B58: 48000008  b 0x82826b60
	pc = 0x82826B60; continue 'dispatch;
	// 82826B5C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82826B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82826B64: 48981658  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82826B68 size=128
    let mut pc: u32 = 0x82826B68;
    'dispatch: loop {
        match pc {
            0x82826B68 => {
    //   block [0x82826B68..0x82826BE8)
	// 82826B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82826B6C: 48981601  bl 0x831a816c
	ctx.lr = 0x82826B70;
	sub_831A8130(ctx, base);
	// 82826B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826B74: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82826B78: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82826B7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82826B80: 3BEBB03C  addi r31, r11, -0x4fc4
	ctx.r[31].s64 = ctx.r[11].s64 + -20420;
	// 82826B84: 816AB044  lwz r11, -0x4fbc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20412 as u32) ) } as u64;
	// 82826B88: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82826B8C: 40820024  bne 0x82826bb0
	if !ctx.cr[0].eq {
	pc = 0x82826BB0; continue 'dispatch;
	}
	// 82826B90: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 82826B94: 3D008282  lis r8, -0x7d7e
	ctx.r[8].s64 = -2105409536;
	// 82826B98: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82826B9C: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 82826BA0: 390864A0  addi r8, r8, 0x64a0
	ctx.r[8].s64 = ctx.r[8].s64 + 25760;
	// 82826BA4: 916AB044  stw r11, -0x4fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20412 as u32), ctx.r[11].u32 ) };
	// 82826BA8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82826BAC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82826BB0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82826BB4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82826BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826BBC: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82826BC0: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82826BC4: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82826BC8: 4BE2D9F9  bl 0x826545c0
	ctx.lr = 0x82826BCC;
	sub_826545C0(ctx, base);
	// 82826BCC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82826BD0: 4182000C  beq 0x82826bdc
	if ctx.cr[0].eq {
	pc = 0x82826BDC; continue 'dispatch;
	}
	// 82826BD4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82826BD8: 48000008  b 0x82826be0
	pc = 0x82826BE0; continue 'dispatch;
	// 82826BDC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82826BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82826BE4: 489815D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82826BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82826BE8 size=1088
    let mut pc: u32 = 0x82826BE8;
    'dispatch: loop {
        match pc {
            0x82826BE8 => {
    //   block [0x82826BE8..0x82827028)
	// 82826BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82826BEC: 48981579  bl 0x831a8164
	ctx.lr = 0x82826BF0;
	sub_831A8130(ctx, base);
	// 82826BF0: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82826BF4: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82826BF8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82826BFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82826C00: 4BFFE8A9  bl 0x828254a8
	ctx.lr = 0x82826C04;
	sub_828254A8(ctx, base);
	// 82826C04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82826C08: 4BFC55C9  bl 0x827ec1d0
	ctx.lr = 0x82826C0C;
	sub_827EC1D0(ctx, base);
	// 82826C0C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82826C10: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826C14: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82826C18: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82826C1C: C3C9964C  lfs f30, -0x69b4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82826C20: 3B8BA808  addi r28, r11, -0x57f8
	ctx.r[28].s64 = ctx.r[11].s64 + -22520;
	// 82826C24: 3B6AA814  addi r27, r10, -0x57ec
	ctx.r[27].s64 = ctx.r[10].s64 + -22508;
	// 82826C28: 4182004C  beq 0x82826c74
	if ctx.cr[0].eq {
	pc = 0x82826C74; continue 'dispatch;
	}
	// 82826C2C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82826C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826C34: 485CCDD5  bl 0x82df3a08
	ctx.lr = 0x82826C38;
	sub_82DF3A08(ctx, base);
	// 82826C38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826C3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826C40: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82826C44: 4BFC36F5  bl 0x827ea338
	ctx.lr = 0x82826C48;
	sub_827EA338(ctx, base);
	// 82826C48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826C4C: 485CC7DD  bl 0x82df3428
	ctx.lr = 0x82826C50;
	sub_82DF3428(ctx, base);
	// 82826C50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82826C54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826C58: 485CCDB1  bl 0x82df3a08
	ctx.lr = 0x82826C5C;
	sub_82DF3A08(ctx, base);
	// 82826C5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826C60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826C64: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82826C68: 4BFC36D1  bl 0x827ea338
	ctx.lr = 0x82826C6C;
	sub_827EA338(ctx, base);
	// 82826C6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826C70: 485CC7B9  bl 0x82df3428
	ctx.lr = 0x82826C74;
	sub_82DF3428(ctx, base);
	// 82826C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82826C78: 4BFC5559  bl 0x827ec1d0
	ctx.lr = 0x82826C7C;
	sub_827EC1D0(ctx, base);
	// 82826C7C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82826C80: 418202CC  beq 0x82826f4c
	if ctx.cr[0].eq {
	pc = 0x82826F4C; continue 'dispatch;
	}
	// 82826C84: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82826C88: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82826C8C: 396B67A8  addi r11, r11, 0x67a8
	ctx.r[11].s64 = ctx.r[11].s64 + 26536;
	// 82826C90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82826C94: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82826C98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826C9C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82826CA0: 485CCD69  bl 0x82df3a08
	ctx.lr = 0x82826CA4;
	sub_82DF3A08(ctx, base);
	// 82826CA4: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82826CA8: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82826CAC: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82826CB0: E8A100A8  ld r5, 0xa8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) };
	// 82826CB4: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82826CB8: 4BFFFE31  bl 0x82826ae8
	ctx.lr = 0x82826CBC;
	sub_82826AE8(ctx, base);
	// 82826CBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82826CC0: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82826CC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826CC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826CCC: C02B0944  lfs f1, 0x944(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2372 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82826CD0: 4BFC3AE1  bl 0x827ea7b0
	ctx.lr = 0x82826CD4;
	sub_827EA7B0(ctx, base);
	// 82826CD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826CD8: 485CC751  bl 0x82df3428
	ctx.lr = 0x82826CDC;
	sub_82DF3428(ctx, base);
	// 82826CDC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82826CE0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82826CE4: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82826CE8: 396B6858  addi r11, r11, 0x6858
	ctx.r[11].s64 = ctx.r[11].s64 + 26712;
	// 82826CEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826CF0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82826CF4: 485CCD15  bl 0x82df3a08
	ctx.lr = 0x82826CF8;
	sub_82DF3A08(ctx, base);
	// 82826CF8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82826CFC: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82826D00: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82826D04: E8A100A8  ld r5, 0xa8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) };
	// 82826D08: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82826D0C: 4BFFFDDD  bl 0x82826ae8
	ctx.lr = 0x82826D10;
	sub_82826AE8(ctx, base);
	// 82826D10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82826D14: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82826D18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826D1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826D20: C3EBE47C  lfs f31, -0x1b84(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7044 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82826D24: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82826D28: 4BFC3A89  bl 0x827ea7b0
	ctx.lr = 0x82826D2C;
	sub_827EA7B0(ctx, base);
	// 82826D2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826D30: 485CC6F9  bl 0x82df3428
	ctx.lr = 0x82826D34;
	sub_82DF3428(ctx, base);
	// 82826D34: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82826D38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82826D3C: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82826D40: 396B67A8  addi r11, r11, 0x67a8
	ctx.r[11].s64 = ctx.r[11].s64 + 26536;
	// 82826D44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826D48: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82826D4C: 485CCCBD  bl 0x82df3a08
	ctx.lr = 0x82826D50;
	sub_82DF3A08(ctx, base);
	// 82826D50: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82826D54: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82826D58: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82826D5C: E8A100A8  ld r5, 0xa8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) };
	// 82826D60: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82826D64: 4BFFFD85  bl 0x82826ae8
	ctx.lr = 0x82826D68;
	sub_82826AE8(ctx, base);
	// 82826D68: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82826D6C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82826D70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826D74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826D78: 4BFC3A39  bl 0x827ea7b0
	ctx.lr = 0x82826D7C;
	sub_827EA7B0(ctx, base);
	// 82826D7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826D80: 485CC6A9  bl 0x82df3428
	ctx.lr = 0x82826D84;
	sub_82DF3428(ctx, base);
	// 82826D84: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82826D88: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82826D8C: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82826D90: 396B6858  addi r11, r11, 0x6858
	ctx.r[11].s64 = ctx.r[11].s64 + 26712;
	// 82826D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826D98: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82826D9C: 485CCC6D  bl 0x82df3a08
	ctx.lr = 0x82826DA0;
	sub_82DF3A08(ctx, base);
	// 82826DA0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82826DA4: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82826DA8: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82826DAC: E8A100A8  ld r5, 0xa8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) };
	// 82826DB0: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82826DB4: 4BFFFD35  bl 0x82826ae8
	ctx.lr = 0x82826DB8;
	sub_82826AE8(ctx, base);
	// 82826DB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82826DBC: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82826DC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826DC4: C02BDF2C  lfs f1, -0x20d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8404 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82826DC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826DCC: 4BFC39E5  bl 0x827ea7b0
	ctx.lr = 0x82826DD0;
	sub_827EA7B0(ctx, base);
	// 82826DD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826DD4: 485CC655  bl 0x82df3428
	ctx.lr = 0x82826DD8;
	sub_82DF3428(ctx, base);
	// 82826DD8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82826DDC: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82826DE0: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82826DE4: 396B6538  addi r11, r11, 0x6538
	ctx.r[11].s64 = ctx.r[11].s64 + 25912;
	// 82826DE8: 388AA9A4  addi r4, r10, -0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + -22108;
	// 82826DEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826DF0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82826DF4: 485CCC15  bl 0x82df3a08
	ctx.lr = 0x82826DF8;
	sub_82DF3A08(ctx, base);
	// 82826DF8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82826DFC: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82826E00: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82826E04: E8A100A8  ld r5, 0xa8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) };
	// 82826E08: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82826E0C: 4BFFFD5D  bl 0x82826b68
	ctx.lr = 0x82826E10;
	sub_82826B68(ctx, base);
	// 82826E10: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 82826E14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826E18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826E1C: 4BFC39ED  bl 0x827ea808
	ctx.lr = 0x82826E20;
	sub_827EA808(ctx, base);
	// 82826E20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826E24: 485CC605  bl 0x82df3428
	ctx.lr = 0x82826E28;
	sub_82DF3428(ctx, base);
	// 82826E28: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826E2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826E30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826E34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82826E38: 4E800421  bctrl
	ctx.lr = 0x82826E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82826E3C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82826E40: 4182010C  beq 0x82826f4c
	if ctx.cr[0].eq {
	pc = 0x82826F4C; continue 'dispatch;
	}
	// 82826E44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82826E48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826E4C: 3B8B1EDC  addi r28, r11, 0x1edc
	ctx.r[28].s64 = ctx.r[11].s64 + 7900;
	// 82826E50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82826E54: 485CCBB5  bl 0x82df3a08
	ctx.lr = 0x82826E58;
	sub_82DF3A08(ctx, base);
	// 82826E58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826E5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826E60: 48386E01  bl 0x82badc60
	ctx.lr = 0x82826E64;
	sub_82BADC60(ctx, base);
	// 82826E64: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82826E68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826E6C: 485CC5BD  bl 0x82df3428
	ctx.lr = 0x82826E70;
	sub_82DF3428(ctx, base);
	// 82826E70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82826E74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826E78: 485CCB91  bl 0x82df3a08
	ctx.lr = 0x82826E7C;
	sub_82DF3A08(ctx, base);
	// 82826E7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82826E80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82826E84: 48386DDD  bl 0x82badc60
	ctx.lr = 0x82826E88;
	sub_82BADC60(ctx, base);
	// 82826E88: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82826E8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826E90: 485CC599  bl 0x82df3428
	ctx.lr = 0x82826E94;
	sub_82DF3428(ctx, base);
	// 82826E94: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82826E98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82826E9C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82826EA0: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82826EA4: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82826EA8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82826EAC: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82826EB0: C1AA9534  lfs f13, -0x6acc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82826EB4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82826EB8: C18908A8  lfs f12, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82826EBC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82826EC0: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82826EC4: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82826EC8: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82826ECC: D1A10088  stfs f13, 0x88(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82826ED0: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82826ED4: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82826ED8: D1810094  stfs f12, 0x94(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82826EDC: D0010098  stfs f0, 0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82826EE0: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82826EE4: C026D8B0  lfs f1, -0x2750(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-10064 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82826EE8: 48156309  bl 0x8297d1f0
	ctx.lr = 0x82826EEC;
	sub_8297D1F0(ctx, base);
	// 82826EEC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826EF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82826EF4: 388BA940  addi r4, r11, -0x56c0
	ctx.r[4].s64 = ctx.r[11].s64 + -22208;
	// 82826EF8: 38A00083  li r5, 0x83
	ctx.r[5].s64 = 131;
	// 82826EFC: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82826F00: 4BA994D9  bl 0x822c03d8
	ctx.lr = 0x82826F04;
	sub_822C03D8(ctx, base);
	// 82826F04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82826F08: 41820018  beq 0x82826f20
	if ctx.cr[0].eq {
	pc = 0x82826F20; continue 'dispatch;
	}
	// 82826F0C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82826F10: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82826F14: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82826F18: 48156321  bl 0x8297d238
	ctx.lr = 0x82826F1C;
	sub_8297D238(ctx, base);
	// 82826F1C: 48000008  b 0x82826f24
	pc = 0x82826F24; continue 'dispatch;
	// 82826F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82826F24: 817E0320  lwz r11, 0x320(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(800 as u32) ) } as u64;
	// 82826F28: 907E0320  stw r3, 0x320(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(800 as u32), ctx.r[3].u32 ) };
	// 82826F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82826F30: 419A001C  beq cr6, 0x82826f4c
	if ctx.cr[6].eq {
	pc = 0x82826F4C; continue 'dispatch;
	}
	// 82826F34: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826F38: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82826F3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82826F40: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826F44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82826F48: 4E800421  bctrl
	ctx.lr = 0x82826F4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82826F4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82826F50: 48139611  bl 0x82960560
	ctx.lr = 0x82826F54;
	sub_82960560(ctx, base);
	// 82826F54: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82826F58: 418200C0  beq 0x82827018
	if ctx.cr[0].eq {
	pc = 0x82827018; continue 'dispatch;
	}
	// 82826F5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82826F60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826F64: 388BF820  addi r4, r11, -0x7e0
	ctx.r[4].s64 = ctx.r[11].s64 + -2016;
	// 82826F68: 485CCAA1  bl 0x82df3a08
	ctx.lr = 0x82826F6C;
	sub_82DF3A08(ctx, base);
	// 82826F6C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82826F70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82826F74: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82826F78: 4BFC78A9  bl 0x827ee820
	ctx.lr = 0x82826F7C;
	sub_827EE820(ctx, base);
	// 82826F7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82826F80: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82826F84: 4BFE58DD  bl 0x8280c860
	ctx.lr = 0x82826F88;
	sub_8280C860(ctx, base);
	// 82826F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82826F8C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82826F90: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82826F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82826F98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82826F9C: 419A0024  beq cr6, 0x82826fc0
	if ctx.cr[6].eq {
	pc = 0x82826FC0; continue 'dispatch;
	}
	// 82826FA0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82826FA4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82826FA8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82826FAC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82826FB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82826FB4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82826FB8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82826FBC: 4082FFE8  bne 0x82826fa4
	if !ctx.cr[0].eq {
	pc = 0x82826FA4; continue 'dispatch;
	}
	// 82826FC0: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82826FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82826FC8: 419A0008  beq cr6, 0x82826fd0
	if ctx.cr[6].eq {
	pc = 0x82826FD0; continue 'dispatch;
	}
	// 82826FCC: 4BA998C5  bl 0x822c0890
	ctx.lr = 0x82826FD0;
	sub_822C0890(ctx, base);
	// 82826FD0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82826FD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82826FD8: 419A0008  beq cr6, 0x82826fe0
	if ctx.cr[6].eq {
	pc = 0x82826FE0; continue 'dispatch;
	}
	// 82826FDC: 4BA998B5  bl 0x822c0890
	ctx.lr = 0x82826FE0;
	sub_822C0890(ctx, base);
	// 82826FE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82826FE4: 485CC445  bl 0x82df3428
	ctx.lr = 0x82826FE8;
	sub_82DF3428(ctx, base);
	// 82826FE8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82826FEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82826FF0: 419A0018  beq cr6, 0x82827008
	if ctx.cr[6].eq {
	pc = 0x82827008; continue 'dispatch;
	}
	// 82826FF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82826FF8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82826FFC: 388BA938  addi r4, r11, -0x56c8
	ctx.r[4].s64 = ctx.r[11].s64 + -22216;
	// 82827000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827004: 4BFF345D  bl 0x8281a460
	ctx.lr = 0x82827008;
	sub_8281A460(ctx, base);
	// 82827008: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282700C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827010: 419A0008  beq cr6, 0x82827018
	if ctx.cr[6].eq {
	pc = 0x82827018; continue 'dispatch;
	}
	// 82827014: 4BA9987D  bl 0x822c0890
	ctx.lr = 0x82827018;
	sub_822C0890(ctx, base);
	// 82827018: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8282701C: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82827020: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82827024: 48981190  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827028 size=4
    let mut pc: u32 = 0x82827028;
    'dispatch: loop {
        match pc {
            0x82827028 => {
    //   block [0x82827028..0x8282702C)
	// 82827028: 4BFFBEB0  b 0x82822ed8
	sub_82822ED8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827030 size=84
    let mut pc: u32 = 0x82827030;
    'dispatch: loop {
        match pc {
            0x82827030 => {
    //   block [0x82827030..0x82827084)
	// 82827030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282703C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827044: 4BFFBC35  bl 0x82822c78
	ctx.lr = 0x82827048;
	sub_82822C78(ctx, base);
	// 82827048: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282704C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82827050: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82827054: 396BAA14  addi r11, r11, -0x55ec
	ctx.r[11].s64 = ctx.r[11].s64 + -21996;
	// 82827058: 394AAA00  addi r10, r10, -0x5600
	ctx.r[10].s64 = ctx.r[10].s64 + -22016;
	// 8282705C: 3929A9B4  addi r9, r9, -0x564c
	ctx.r[9].s64 = ctx.r[9].s64 + -22092;
	// 82827060: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827064: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82827068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282706C: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82827070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82827074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282707C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827088 size=12
    let mut pc: u32 = 0x82827088;
    'dispatch: loop {
        match pc {
            0x82827088 => {
    //   block [0x82827088..0x82827094)
	// 82827088: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282708C: 386BAE94  addi r3, r11, -0x516c
	ctx.r[3].s64 = ctx.r[11].s64 + -20844;
	// 82827090: 4BFF4EA0  b 0x8281bf30
	sub_8281BF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827098 size=12
    let mut pc: u32 = 0x82827098;
    'dispatch: loop {
        match pc {
            0x82827098 => {
    //   block [0x82827098..0x828270A4)
	// 82827098: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282709C: 386BAE94  addi r3, r11, -0x516c
	ctx.r[3].s64 = ctx.r[11].s64 + -20844;
	// 828270A0: 4BFF4E98  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828270A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828270A8 size=8
    let mut pc: u32 = 0x828270A8;
    'dispatch: loop {
        match pc {
            0x828270A8 => {
    //   block [0x828270A8..0x828270B0)
	// 828270A8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 828270AC: 4800000C  b 0x828270b8
	sub_828270B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828270B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828270B0 size=8
    let mut pc: u32 = 0x828270B0;
    'dispatch: loop {
        match pc {
            0x828270B0 => {
    //   block [0x828270B0..0x828270B8)
	// 828270B0: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 828270B4: 48000004  b 0x828270b8
	sub_828270B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828270B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828270B8 size=112
    let mut pc: u32 = 0x828270B8;
    'dispatch: loop {
        match pc {
            0x828270B8 => {
    //   block [0x828270B8..0x82827128)
	// 828270B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828270BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828270C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828270C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828270C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828270CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828270D0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828270D4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 828270D8: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 828270DC: 396BAA14  addi r11, r11, -0x55ec
	ctx.r[11].s64 = ctx.r[11].s64 + -21996;
	// 828270E0: 394AAA00  addi r10, r10, -0x5600
	ctx.r[10].s64 = ctx.r[10].s64 + -22016;
	// 828270E4: 3929A9B4  addi r9, r9, -0x564c
	ctx.r[9].s64 = ctx.r[9].s64 + -22092;
	// 828270E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828270EC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 828270F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828270F4: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 828270F8: 4BFFBBE9  bl 0x82822ce0
	ctx.lr = 0x828270FC;
	sub_82822CE0(ctx, base);
	// 828270FC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82827100: 4182000C  beq 0x8282710c
	if ctx.cr[0].eq {
	pc = 0x8282710C; continue 'dispatch;
	}
	// 82827104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827108: 485CB2D1  bl 0x82df23d8
	ctx.lr = 0x8282710C;
	sub_82DF23D8(ctx, base);
	// 8282710C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827110: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282711C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827128 size=196
    let mut pc: u32 = 0x82827128;
    'dispatch: loop {
        match pc {
            0x82827128 => {
    //   block [0x82827128..0x828271EC)
	// 82827128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282712C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82827134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282713C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82827140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827144: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82827148: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282714C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827150: 4BA997E9  bl 0x822c0938
	ctx.lr = 0x82827154;
	sub_822C0938(ctx, base);
	// 82827154: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82827158: 41820028  beq 0x82827180
	if ctx.cr[0].eq {
	pc = 0x82827180; continue 'dispatch;
	}
	// 8282715C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827160: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82827164: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82827168: 392BAAB4  addi r9, r11, -0x554c
	ctx.r[9].s64 = ctx.r[11].s64 + -21836;
	// 8282716C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82827170: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82827174: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82827178: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8282717C: 48000008  b 0x82827184
	pc = 0x82827184; continue 'dispatch;
	// 82827180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827184: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282718C: 409A0044  bne cr6, 0x828271d0
	if !ctx.cr[6].eq {
	pc = 0x828271D0; continue 'dispatch;
	}
	// 82827190: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82827194: 419A001C  beq cr6, 0x828271b0
	if ctx.cr[6].eq {
	pc = 0x828271B0; continue 'dispatch;
	}
	// 82827198: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282719C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 828271A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828271A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 828271A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828271AC: 4E800421  bctrl
	ctx.lr = 0x828271B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828271B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 828271B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 828271B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828271BC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 828271C0: 816B04FC  lwz r11, 0x4fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1276 as u32) ) } as u64;
	// 828271C4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 828271C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 828271CC: 4BA98E35  bl 0x822c0000
	ctx.lr = 0x828271D0;
	sub_822C0000(ctx, base);
	// 828271D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828271D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828271D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828271DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828271E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828271E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828271E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828271F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828271F0 size=476
    let mut pc: u32 = 0x828271F0;
    'dispatch: loop {
        match pc {
            0x828271F0 => {
    //   block [0x828271F0..0x828273CC)
	// 828271F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828271F4: 48980F71  bl 0x831a8164
	ctx.lr = 0x828271F8;
	sub_831A8130(ctx, base);
	// 828271F8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 828271FC: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827200: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827204: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82827208: 3BCBAAC8  addi r30, r11, -0x5538
	ctx.r[30].s64 = ctx.r[11].s64 + -21816;
	// 8282720C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82827210: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82827214: 38A0002B  li r5, 0x2b
	ctx.r[5].s64 = 43;
	// 82827218: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8282721C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82827220: 386002E0  li r3, 0x2e0
	ctx.r[3].s64 = 736;
	// 82827224: 485CB1C5  bl 0x82df23e8
	ctx.lr = 0x82827228;
	sub_82DF23E8(ctx, base);
	// 82827228: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282722C: 41820010  beq 0x8282723c
	if ctx.cr[0].eq {
	pc = 0x8282723C; continue 'dispatch;
	}
	// 82827230: 4BFFFE01  bl 0x82827030
	ctx.lr = 0x82827234;
	sub_82827030(ctx, base);
	// 82827234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827238: 48000008  b 0x82827240
	pc = 0x82827240; continue 'dispatch;
	// 8282723C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82827240: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82827244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82827248: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282724C: 4BFFFEDD  bl 0x82827128
	ctx.lr = 0x82827250;
	sub_82827128(ctx, base);
	// 82827250: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82827254: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82827258: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282725C: 4BA98DA5  bl 0x822c0000
	ctx.lr = 0x82827260;
	sub_822C0000(ctx, base);
	// 82827260: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82827264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82827268: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 8282726C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82827270: 4BA99169  bl 0x822c03d8
	ctx.lr = 0x82827274;
	sub_822C03D8(ctx, base);
	// 82827274: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82827278: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282727C: 418200E0  beq 0x8282735c
	if ctx.cr[0].eq {
	pc = 0x8282735C; continue 'dispatch;
	}
	// 82827280: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82827284: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82827288: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282728C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82827290: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82827294: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82827298: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 8282729C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828272A0: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 828272A4: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 828272A8: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 828272AC: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 828272B0: 409A0008  bne cr6, 0x828272b8
	if !ctx.cr[6].eq {
	pc = 0x828272B8; continue 'dispatch;
	}
	// 828272B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828272B8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 828272BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828272C0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 828272C4: 419A0024  beq cr6, 0x828272e8
	if ctx.cr[6].eq {
	pc = 0x828272E8; continue 'dispatch;
	}
	// 828272C8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828272CC: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 828272D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828272D4: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 828272D8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 828272DC: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828272E0: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828272E4: 4082FFE8  bne 0x828272cc
	if !ctx.cr[0].eq {
	pc = 0x828272CC; continue 'dispatch;
	}
	// 828272E8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 828272EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828272F0: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 828272F4: 419A0024  beq cr6, 0x82827318
	if ctx.cr[6].eq {
	pc = 0x82827318; continue 'dispatch;
	}
	// 828272F8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828272FC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82827300: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82827304: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82827308: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282730C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82827310: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82827314: 4082FFE8  bne 0x828272fc
	if !ctx.cr[0].eq {
	pc = 0x828272FC; continue 'dispatch;
	}
	// 82827318: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8282731C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82827320: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82827324: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82827328: 48655919  bl 0x82e7cc40
	ctx.lr = 0x8282732C;
	sub_82E7CC40(ctx, base);
	// 8282732C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82827330: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82827334: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82827338: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8282733C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82827340: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82827344: 4BF71BED  bl 0x82798f30
	ctx.lr = 0x82827348;
	sub_82798F30(ctx, base);
	// 82827348: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282734C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827350: 4BF74319  bl 0x8279b668
	ctx.lr = 0x82827354;
	sub_8279B668(ctx, base);
	// 82827354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827358: 48000008  b 0x82827360
	pc = 0x82827360; continue 'dispatch;
	// 8282735C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82827360: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82827364: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 82827368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282736C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82827370: 4BD708E1  bl 0x82597c50
	ctx.lr = 0x82827374;
	sub_82597C50(ctx, base);
	// 82827374: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82827378: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282737C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82827380: 4BA98C81  bl 0x822c0000
	ctx.lr = 0x82827384;
	sub_822C0000(ctx, base);
	// 82827384: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82827388: 41820024  beq 0x828273ac
	if ctx.cr[0].eq {
	pc = 0x828273AC; continue 'dispatch;
	}
	// 8282738C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82827390: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827394: 419A0008  beq cr6, 0x8282739c
	if ctx.cr[6].eq {
	pc = 0x8282739C; continue 'dispatch;
	}
	// 82827398: 4BA994F9  bl 0x822c0890
	ctx.lr = 0x8282739C;
	sub_822C0890(ctx, base);
	// 8282739C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 828273A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828273A4: 419A0008  beq cr6, 0x828273ac
	if ctx.cr[6].eq {
	pc = 0x828273AC; continue 'dispatch;
	}
	// 828273A8: 4BA994E9  bl 0x822c0890
	ctx.lr = 0x828273AC;
	sub_822C0890(ctx, base);
	// 828273AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828273B0: 419A000C  beq cr6, 0x828273bc
	if ctx.cr[6].eq {
	pc = 0x828273BC; continue 'dispatch;
	}
	// 828273B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 828273B8: 4BA994D9  bl 0x822c0890
	ctx.lr = 0x828273BC;
	sub_822C0890(ctx, base);
	// 828273BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 828273C0: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 828273C4: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 828273C8: 48980DEC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828273D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x828273D0 size=12
    let mut pc: u32 = 0x828273D0;
    'dispatch: loop {
        match pc {
            0x828273D0 => {
    //   block [0x828273D0..0x828273DC)
	// 828273D0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828273D4: C02BAB2C  lfs f1, -0x54d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21716 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828273D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828273E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828273E0 size=252
    let mut pc: u32 = 0x828273E0;
    'dispatch: loop {
        match pc {
            0x828273E0 => {
    //   block [0x828273E0..0x828274DC)
	// 828273E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828273E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828273E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828273EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828273F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828273F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 828273F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828273FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827400: 388B0380  addi r4, r11, 0x380
	ctx.r[4].s64 = ctx.r[11].s64 + 896;
	// 82827404: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82827408: 485CC601  bl 0x82df3a08
	ctx.lr = 0x8282740C;
	sub_82DF3A08(ctx, base);
	// 8282740C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82827410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827414: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82827418: 4BFC4719  bl 0x827ebb30
	ctx.lr = 0x8282741C;
	sub_827EBB30(ctx, base);
	// 8282741C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827420: 485CC009  bl 0x82df3428
	ctx.lr = 0x82827424;
	sub_82DF3428(ctx, base);
	// 82827424: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282742C: 388BAB54  addi r4, r11, -0x54ac
	ctx.r[4].s64 = ctx.r[11].s64 + -21676;
	// 82827430: 485CC5D9  bl 0x82df3a08
	ctx.lr = 0x82827434;
	sub_82DF3A08(ctx, base);
	// 82827434: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82827438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282743C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82827440: 4BFC46F1  bl 0x827ebb30
	ctx.lr = 0x82827444;
	sub_827EBB30(ctx, base);
	// 82827444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827448: 485CBFE1  bl 0x82df3428
	ctx.lr = 0x8282744C;
	sub_82DF3428(ctx, base);
	// 8282744C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827450: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827454: 388BAB48  addi r4, r11, -0x54b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21688;
	// 82827458: 485CC5B1  bl 0x82df3a08
	ctx.lr = 0x8282745C;
	sub_82DF3A08(ctx, base);
	// 8282745C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82827460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827464: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82827468: 4BFC46C9  bl 0x827ebb30
	ctx.lr = 0x8282746C;
	sub_827EBB30(ctx, base);
	// 8282746C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827470: 485CBFB9  bl 0x82df3428
	ctx.lr = 0x82827474;
	sub_82DF3428(ctx, base);
	// 82827474: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282747C: 388BAB3C  addi r4, r11, -0x54c4
	ctx.r[4].s64 = ctx.r[11].s64 + -21700;
	// 82827480: 485CC589  bl 0x82df3a08
	ctx.lr = 0x82827484;
	sub_82DF3A08(ctx, base);
	// 82827484: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82827488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282748C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82827490: 4BFC4769  bl 0x827ebbf8
	ctx.lr = 0x82827494;
	sub_827EBBF8(ctx, base);
	// 82827494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827498: 485CBF91  bl 0x82df3428
	ctx.lr = 0x8282749C;
	sub_82DF3428(ctx, base);
	// 8282749C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828274A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828274A4: 388BAB30  addi r4, r11, -0x54d0
	ctx.r[4].s64 = ctx.r[11].s64 + -21712;
	// 828274A8: 485CC561  bl 0x82df3a08
	ctx.lr = 0x828274AC;
	sub_82DF3A08(ctx, base);
	// 828274AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828274B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828274B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828274B8: 4BFC4741  bl 0x827ebbf8
	ctx.lr = 0x828274BC;
	sub_827EBBF8(ctx, base);
	// 828274BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828274C0: 485CBF69  bl 0x82df3428
	ctx.lr = 0x828274C4;
	sub_82DF3428(ctx, base);
	// 828274C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828274C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828274CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828274D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828274D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828274D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828274E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828274E0 size=120
    let mut pc: u32 = 0x828274E0;
    'dispatch: loop {
        match pc {
            0x828274E0 => {
    //   block [0x828274E0..0x82827558)
	// 828274E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828274E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828274E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828274EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828274F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828274F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828274F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828274FC: 817F027C  lwz r11, 0x27c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(636 as u32) ) } as u64;
	// 82827500: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82827504: 419A0010  beq cr6, 0x82827514
	if ctx.cr[6].eq {
	pc = 0x82827514; continue 'dispatch;
	}
	// 82827508: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282750C: 388BAB60  addi r4, r11, -0x54a0
	ctx.r[4].s64 = ctx.r[11].s64 + -21664;
	// 82827510: 4800000C  b 0x8282751c
	pc = 0x8282751C; continue 'dispatch;
	// 82827514: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82827518: 388BCFB0  addi r4, r11, -0x3050
	ctx.r[4].s64 = ctx.r[11].s64 + -12368;
	// 8282751C: 485CC4ED  bl 0x82df3a08
	ctx.lr = 0x82827520;
	sub_82DF3A08(ctx, base);
	// 82827520: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827524: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82827528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282752C: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82827530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82827534: 4E800421  bctrl
	ctx.lr = 0x82827538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82827538: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282753C: 4BFC817D  bl 0x827ef6b8
	ctx.lr = 0x82827540;
	sub_827EF6B8(ctx, base);
	// 82827540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282754C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827558 size=12
    let mut pc: u32 = 0x82827558;
    'dispatch: loop {
        match pc {
            0x82827558 => {
    //   block [0x82827558..0x82827564)
	// 82827558: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8282755C: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82827560: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82827564 size=24
    let mut pc: u32 = 0x82827564;
    'dispatch: loop {
        match pc {
            0x82827564 => {
    //   block [0x82827564..0x8282757C)
	// 82827564: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82827568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8282756C: 91430278  stw r10, 0x278(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(632 as u32), ctx.r[10].u32 ) };
	// 82827570: C00B9534  lfs f0, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82827574: D0030274  stfs f0, 0x274(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(628 as u32), tmp.u32 ) };
	// 82827578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82827580 size=152
    let mut pc: u32 = 0x82827580;
    'dispatch: loop {
        match pc {
            0x82827580 => {
    //   block [0x82827580..0x82827618)
	// 82827580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282758C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282759C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828275A0: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 828275A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828275A8: 4E800421  bctrl
	ctx.lr = 0x828275AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828275AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828275B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828275B4: 4BFF16F5  bl 0x82818ca8
	ctx.lr = 0x828275B8;
	sub_82818CA8(ctx, base);
	// 828275B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828275BC: 481EC0C5  bl 0x82a13680
	ctx.lr = 0x828275C0;
	sub_82A13680(ctx, base);
	// 828275C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 828275C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828275C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828275CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828275D0: C02B6218  lfs f1, 0x6218(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828275D4: 4BFF1795  bl 0x82818d68
	ctx.lr = 0x828275D8;
	sub_82818D68(ctx, base);
	// 828275D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828275DC: 481EC0A5  bl 0x82a13680
	ctx.lr = 0x828275E0;
	sub_82A13680(ctx, base);
	// 828275E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 828275E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828275E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828275EC: C02BA9F0  lfs f1, -0x5610(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22032 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828275F0: 4BFF1A41  bl 0x82819030
	ctx.lr = 0x828275F4;
	sub_82819030(ctx, base);
	// 828275F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828275F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828275FC: 4BFF212D  bl 0x82819728
	ctx.lr = 0x82827600;
	sub_82819728(ctx, base);
	// 82827600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282760C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827618 size=84
    let mut pc: u32 = 0x82827618;
    'dispatch: loop {
        match pc {
            0x82827618 => {
    //   block [0x82827618..0x8282766C)
	// 82827618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282761C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827620: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827624: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827628: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282762C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827630: 388BB098  addi r4, r11, -0x4f68
	ctx.r[4].s64 = ctx.r[11].s64 + -20328;
	// 82827634: 4BFC91F5  bl 0x827f0828
	ctx.lr = 0x82827638;
	sub_827F0828(ctx, base);
	// 82827638: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282763C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827640: 388BB0B4  addi r4, r11, -0x4f4c
	ctx.r[4].s64 = ctx.r[11].s64 + -20300;
	// 82827644: 4BFCEAAD  bl 0x827f60f0
	ctx.lr = 0x82827648;
	sub_827F60F0(ctx, base);
	// 82827648: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282764C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827650: 388BB0C4  addi r4, r11, -0x4f3c
	ctx.r[4].s64 = ctx.r[11].s64 + -20284;
	// 82827654: 4BFC9475  bl 0x827f0ac8
	ctx.lr = 0x82827658;
	sub_827F0AC8(ctx, base);
	// 82827658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282765C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827670 size=12
    let mut pc: u32 = 0x82827670;
    'dispatch: loop {
        match pc {
            0x82827670 => {
    //   block [0x82827670..0x8282767C)
	// 82827670: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827674: 386BAED4  addi r3, r11, -0x512c
	ctx.r[3].s64 = ctx.r[11].s64 + -20780;
	// 82827678: 4BFF48B8  b 0x8281bf30
	sub_8281BF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827680 size=12
    let mut pc: u32 = 0x82827680;
    'dispatch: loop {
        match pc {
            0x82827680 => {
    //   block [0x82827680..0x8282768C)
	// 82827680: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827684: 386BAED4  addi r3, r11, -0x512c
	ctx.r[3].s64 = ctx.r[11].s64 + -20780;
	// 82827688: 4BFF48B0  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827690 size=12
    let mut pc: u32 = 0x82827690;
    'dispatch: loop {
        match pc {
            0x82827690 => {
    //   block [0x82827690..0x8282769C)
	// 82827690: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827694: 386BAB6C  addi r3, r11, -0x5494
	ctx.r[3].s64 = ctx.r[11].s64 + -21652;
	// 82827698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828276A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828276A0 size=12
    let mut pc: u32 = 0x828276A0;
    'dispatch: loop {
        match pc {
            0x828276A0 => {
    //   block [0x828276A0..0x828276AC)
	// 828276A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 828276A4: 386B24D4  addi r3, r11, 0x24d4
	ctx.r[3].s64 = ctx.r[11].s64 + 9428;
	// 828276A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828276B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828276B0 size=12
    let mut pc: u32 = 0x828276B0;
    'dispatch: loop {
        match pc {
            0x828276B0 => {
    //   block [0x828276B0..0x828276BC)
	// 828276B0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828276B4: 386BAB7C  addi r3, r11, -0x5484
	ctx.r[3].s64 = ctx.r[11].s64 + -21636;
	// 828276B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828276C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828276C0 size=12
    let mut pc: u32 = 0x828276C0;
    'dispatch: loop {
        match pc {
            0x828276C0 => {
    //   block [0x828276C0..0x828276CC)
	// 828276C0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828276C4: 386BB0B4  addi r3, r11, -0x4f4c
	ctx.r[3].s64 = ctx.r[11].s64 + -20300;
	// 828276C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828276D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828276D0 size=12
    let mut pc: u32 = 0x828276D0;
    'dispatch: loop {
        match pc {
            0x828276D0 => {
    //   block [0x828276D0..0x828276DC)
	// 828276D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828276D4: 386BB098  addi r3, r11, -0x4f68
	ctx.r[3].s64 = ctx.r[11].s64 + -20328;
	// 828276D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828276E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828276E0 size=12
    let mut pc: u32 = 0x828276E0;
    'dispatch: loop {
        match pc {
            0x828276E0 => {
    //   block [0x828276E0..0x828276EC)
	// 828276E0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828276E4: 386BB0C4  addi r3, r11, -0x4f3c
	ctx.r[3].s64 = ctx.r[11].s64 + -20284;
	// 828276E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828276F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828276F0 size=84
    let mut pc: u32 = 0x828276F0;
    'dispatch: loop {
        match pc {
            0x828276F0 => {
    //   block [0x828276F0..0x82827744)
	// 828276F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828276F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828276F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828276FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827700: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827708: 388BB0E4  addi r4, r11, -0x4f1c
	ctx.r[4].s64 = ctx.r[11].s64 + -20252;
	// 8282770C: 4BFC911D  bl 0x827f0828
	ctx.lr = 0x82827710;
	sub_827F0828(ctx, base);
	// 82827710: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827718: 388BB100  addi r4, r11, -0x4f00
	ctx.r[4].s64 = ctx.r[11].s64 + -20224;
	// 8282771C: 4BFCE9D5  bl 0x827f60f0
	ctx.lr = 0x82827720;
	sub_827F60F0(ctx, base);
	// 82827720: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827728: 388BB110  addi r4, r11, -0x4ef0
	ctx.r[4].s64 = ctx.r[11].s64 + -20208;
	// 8282772C: 4BFC939D  bl 0x827f0ac8
	ctx.lr = 0x82827730;
	sub_827F0AC8(ctx, base);
	// 82827730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82827734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282773C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827748 size=12
    let mut pc: u32 = 0x82827748;
    'dispatch: loop {
        match pc {
            0x82827748 => {
    //   block [0x82827748..0x82827754)
	// 82827748: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282774C: 386BAEE4  addi r3, r11, -0x511c
	ctx.r[3].s64 = ctx.r[11].s64 + -20764;
	// 82827750: 4BFF47E0  b 0x8281bf30
	sub_8281BF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827758 size=12
    let mut pc: u32 = 0x82827758;
    'dispatch: loop {
        match pc {
            0x82827758 => {
    //   block [0x82827758..0x82827764)
	// 82827758: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282775C: 386BAEE4  addi r3, r11, -0x511c
	ctx.r[3].s64 = ctx.r[11].s64 + -20764;
	// 82827760: 4BFF47D8  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827768 size=12
    let mut pc: u32 = 0x82827768;
    'dispatch: loop {
        match pc {
            0x82827768 => {
    //   block [0x82827768..0x82827774)
	// 82827768: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282776C: 386BAB94  addi r3, r11, -0x546c
	ctx.r[3].s64 = ctx.r[11].s64 + -21612;
	// 82827770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827778 size=12
    let mut pc: u32 = 0x82827778;
    'dispatch: loop {
        match pc {
            0x82827778 => {
    //   block [0x82827778..0x82827784)
	// 82827778: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282777C: 386BABA4  addi r3, r11, -0x545c
	ctx.r[3].s64 = ctx.r[11].s64 + -21596;
	// 82827780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827788 size=12
    let mut pc: u32 = 0x82827788;
    'dispatch: loop {
        match pc {
            0x82827788 => {
    //   block [0x82827788..0x82827794)
	// 82827788: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282778C: 386BABB4  addi r3, r11, -0x544c
	ctx.r[3].s64 = ctx.r[11].s64 + -21580;
	// 82827790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827798 size=12
    let mut pc: u32 = 0x82827798;
    'dispatch: loop {
        match pc {
            0x82827798 => {
    //   block [0x82827798..0x828277A4)
	// 82827798: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282779C: 386BB100  addi r3, r11, -0x4f00
	ctx.r[3].s64 = ctx.r[11].s64 + -20224;
	// 828277A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828277A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828277A8 size=12
    let mut pc: u32 = 0x828277A8;
    'dispatch: loop {
        match pc {
            0x828277A8 => {
    //   block [0x828277A8..0x828277B4)
	// 828277A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828277AC: 386BB0E4  addi r3, r11, -0x4f1c
	ctx.r[3].s64 = ctx.r[11].s64 + -20252;
	// 828277B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828277B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828277B8 size=12
    let mut pc: u32 = 0x828277B8;
    'dispatch: loop {
        match pc {
            0x828277B8 => {
    //   block [0x828277B8..0x828277C4)
	// 828277B8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828277BC: 386BB110  addi r3, r11, -0x4ef0
	ctx.r[3].s64 = ctx.r[11].s64 + -20208;
	// 828277C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828277C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828277C8 size=84
    let mut pc: u32 = 0x828277C8;
    'dispatch: loop {
        match pc {
            0x828277C8 => {
    //   block [0x828277C8..0x8282781C)
	// 828277C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828277CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828277D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828277D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828277D8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828277DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828277E0: 388BB130  addi r4, r11, -0x4ed0
	ctx.r[4].s64 = ctx.r[11].s64 + -20176;
	// 828277E4: 4BFC9045  bl 0x827f0828
	ctx.lr = 0x828277E8;
	sub_827F0828(ctx, base);
	// 828277E8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828277EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828277F0: 388BB14C  addi r4, r11, -0x4eb4
	ctx.r[4].s64 = ctx.r[11].s64 + -20148;
	// 828277F4: 4BFCE8FD  bl 0x827f60f0
	ctx.lr = 0x828277F8;
	sub_827F60F0(ctx, base);
	// 828277F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828277FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827800: 388BB15C  addi r4, r11, -0x4ea4
	ctx.r[4].s64 = ctx.r[11].s64 + -20132;
	// 82827804: 4BFC92C5  bl 0x827f0ac8
	ctx.lr = 0x82827808;
	sub_827F0AC8(ctx, base);
	// 82827808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282780C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827820 size=12
    let mut pc: u32 = 0x82827820;
    'dispatch: loop {
        match pc {
            0x82827820 => {
    //   block [0x82827820..0x8282782C)
	// 82827820: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827824: 386BAEDC  addi r3, r11, -0x5124
	ctx.r[3].s64 = ctx.r[11].s64 + -20772;
	// 82827828: 4BFF4708  b 0x8281bf30
	sub_8281BF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827830 size=12
    let mut pc: u32 = 0x82827830;
    'dispatch: loop {
        match pc {
            0x82827830 => {
    //   block [0x82827830..0x8282783C)
	// 82827830: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827834: 386BAEDC  addi r3, r11, -0x5124
	ctx.r[3].s64 = ctx.r[11].s64 + -20772;
	// 82827838: 4BFF4700  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827840 size=12
    let mut pc: u32 = 0x82827840;
    'dispatch: loop {
        match pc {
            0x82827840 => {
    //   block [0x82827840..0x8282784C)
	// 82827840: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827844: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 82827848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827850 size=12
    let mut pc: u32 = 0x82827850;
    'dispatch: loop {
        match pc {
            0x82827850 => {
    //   block [0x82827850..0x8282785C)
	// 82827850: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827854: 386BABE0  addi r3, r11, -0x5420
	ctx.r[3].s64 = ctx.r[11].s64 + -21536;
	// 82827858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827860 size=12
    let mut pc: u32 = 0x82827860;
    'dispatch: loop {
        match pc {
            0x82827860 => {
    //   block [0x82827860..0x8282786C)
	// 82827860: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827864: 386BABF0  addi r3, r11, -0x5410
	ctx.r[3].s64 = ctx.r[11].s64 + -21520;
	// 82827868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827870 size=12
    let mut pc: u32 = 0x82827870;
    'dispatch: loop {
        match pc {
            0x82827870 => {
    //   block [0x82827870..0x8282787C)
	// 82827870: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827874: 386BB14C  addi r3, r11, -0x4eb4
	ctx.r[3].s64 = ctx.r[11].s64 + -20148;
	// 82827878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827880 size=12
    let mut pc: u32 = 0x82827880;
    'dispatch: loop {
        match pc {
            0x82827880 => {
    //   block [0x82827880..0x8282788C)
	// 82827880: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827884: 386BB130  addi r3, r11, -0x4ed0
	ctx.r[3].s64 = ctx.r[11].s64 + -20176;
	// 82827888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827890 size=12
    let mut pc: u32 = 0x82827890;
    'dispatch: loop {
        match pc {
            0x82827890 => {
    //   block [0x82827890..0x8282789C)
	// 82827890: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82827894: 386BB15C  addi r3, r11, -0x4ea4
	ctx.r[3].s64 = ctx.r[11].s64 + -20132;
	// 82827898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828278A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x828278A0 size=24
    let mut pc: u32 = 0x828278A0;
    'dispatch: loop {
        match pc {
            0x828278A0 => {
    //   block [0x828278A0..0x828278B8)
	// 828278A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 828278A4: C1A30080  lfs f13, 0x80(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 828278A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 828278AC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828278B0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 828278B4: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828278B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828278B8 size=8
    let mut pc: u32 = 0x828278B8;
    'dispatch: loop {
        match pc {
            0x828278B8 => {
    //   block [0x828278B8..0x828278C0)
	// 828278B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 828278BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828278C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828278C0 size=100
    let mut pc: u32 = 0x828278C0;
    'dispatch: loop {
        match pc {
            0x828278C0 => {
    //   block [0x828278C0..0x82827924)
	// 828278C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828278C4: 489808A9  bl 0x831a816c
	ctx.lr = 0x828278C8;
	sub_831A8130(ctx, base);
	// 828278C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828278CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828278D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828278D4: 3BFD026C  addi r31, r29, 0x26c
	ctx.r[31].s64 = ctx.r[29].s64 + 620;
	// 828278D8: 817D026C  lwz r11, 0x26c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(620 as u32) ) } as u64;
	// 828278DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828278E0: 419A0030  beq cr6, 0x82827910
	if ctx.cr[6].eq {
	pc = 0x82827910; continue 'dispatch;
	}
	// 828278E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828278E8: 4BCE3069  bl 0x8250a950
	ctx.lr = 0x828278EC;
	sub_8250A950(ctx, base);
	// 828278EC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828278F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828278F4: 386BFF40  addi r3, r11, -0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + -192;
	// 828278F8: 409A0008  bne cr6, 0x82827900
	if !ctx.cr[6].eq {
	pc = 0x82827900; continue 'dispatch;
	}
	// 828278FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82827900: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82827904: 4BFAC0FD  bl 0x827d3a00
	ctx.lr = 0x82827908;
	sub_827D3A00(ctx, base);
	// 82827908: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282790C: 485CA385  bl 0x82df1c90
	ctx.lr = 0x82827910;
	sub_82DF1C90(ctx, base);
	// 82827910: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82827914: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82827918: 4BFF2149  bl 0x82819a60
	ctx.lr = 0x8282791C;
	sub_82819A60(ctx, base);
	// 8282791C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82827920: 4898089C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82827928 size=116
    let mut pc: u32 = 0x82827928;
    'dispatch: loop {
        match pc {
            0x82827928 => {
    //   block [0x82827928..0x8282799C)
	// 82827928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282792C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827934: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827938: 8063025C  lwz r3, 0x25c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(604 as u32) ) } as u64;
	// 8282793C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82827940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827944: 419A0044  beq cr6, 0x82827988
	if ctx.cr[6].eq {
	pc = 0x82827988; continue 'dispatch;
	}
	// 82827948: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282794C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82827950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82827954: 4E800421  bctrl
	ctx.lr = 0x82827958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82827958: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8282795C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82827960: C0030030  lfs f0, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82827964: C1A30034  lfs f13, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82827968: C1830038  lfs f12, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8282796C: C163003C  lfs f11, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82827970: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82827974: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82827978: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8282797C: D161005C  stfs f11, 0x5c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82827980: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828279A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828279A0 size=156
    let mut pc: u32 = 0x828279A0;
    'dispatch: loop {
        match pc {
            0x828279A0 => {
    //   block [0x828279A0..0x82827A3C)
	// 828279A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828279A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828279A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828279AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828279B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828279B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 828279B8: C01F0080  lfs f0, 0x80(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828279BC: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 828279C0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 828279C4: 4099000C  ble cr6, 0x828279d0
	if !ctx.cr[6].gt {
	pc = 0x828279D0; continue 'dispatch;
	}
	// 828279C8: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 828279CC: D01F0080  stfs f0, 0x80(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 828279D0: C01F007C  lfs f0, 0x7c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828279D4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828279D8: C1BF0080  lfs f13, 0x80(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 828279DC: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 828279E0: C00B6150  lfs f0, 0x6150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828279E4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 828279E8: 40990040  ble cr6, 0x82827a28
	if !ctx.cr[6].gt {
	pc = 0x82827A28; continue 'dispatch;
	}
	// 828279EC: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 828279F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 828279F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 828279F8: 419A0030  beq cr6, 0x82827a28
	if ctx.cr[6].eq {
	pc = 0x82827A28; continue 'dispatch;
	}
	// 828279FC: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82827A00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827A04: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82827A08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82827A0C: 4E800421  bctrl
	ctx.lr = 0x82827A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82827A10: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82827A14: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82827A18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82827A1C: 38AA6910  addi r5, r10, 0x6910
	ctx.r[5].s64 = ctx.r[10].s64 + 26896;
	// 82827A20: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827A24: 48137DFD  bl 0x8295f820
	ctx.lr = 0x82827A28;
	sub_8295F820(ctx, base);
	// 82827A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82827A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827A34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82827A40 size=128
    let mut pc: u32 = 0x82827A40;
    'dispatch: loop {
        match pc {
            0x82827A40 => {
    //   block [0x82827A40..0x82827AC0)
	// 82827A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827A48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827A4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827A50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827A54: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82827A58: 4BFC7D29  bl 0x827ef780
	ctx.lr = 0x82827A5C;
	sub_827EF780(ctx, base);
	// 82827A5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82827A60: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82827A64: 4BFC284D  bl 0x827ea2b0
	ctx.lr = 0x82827A68;
	sub_827EA2B0(ctx, base);
	// 82827A68: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82827A6C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827A70: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82827A74: 419A000C  beq cr6, 0x82827a80
	if ctx.cr[6].eq {
	pc = 0x82827A80; continue 'dispatch;
	}
	// 82827A78: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82827A7C: 48137AD5  bl 0x8295f550
	ctx.lr = 0x82827A80;
	sub_8295F550(ctx, base);
	// 82827A80: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82827A84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827A88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82827A8C: 419A0010  beq cr6, 0x82827a9c
	if ctx.cr[6].eq {
	pc = 0x82827A9C; continue 'dispatch;
	}
	// 82827A90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82827A94: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82827A98: 48659CA9  bl 0x82e81740
	ctx.lr = 0x82827A9C;
	sub_82E81740(ctx, base);
	// 82827A9C: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82827AA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827AA4: 419A0008  beq cr6, 0x82827aac
	if ctx.cr[6].eq {
	pc = 0x82827AAC; continue 'dispatch;
	}
	// 82827AA8: 4831EC21  bl 0x82b466c8
	ctx.lr = 0x82827AAC;
	sub_82B466C8(ctx, base);
	// 82827AAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82827AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827AC0 size=196
    let mut pc: u32 = 0x82827AC0;
    'dispatch: loop {
        match pc {
            0x82827AC0 => {
    //   block [0x82827AC0..0x82827B84)
	// 82827AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827AC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82827ACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827AD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827AD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82827AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827ADC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82827AE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82827AE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827AE8: 4BA98E51  bl 0x822c0938
	ctx.lr = 0x82827AEC;
	sub_822C0938(ctx, base);
	// 82827AEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82827AF0: 41820028  beq 0x82827b18
	if ctx.cr[0].eq {
	pc = 0x82827B18; continue 'dispatch;
	}
	// 82827AF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827AF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82827AFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82827B00: 392BAC10  addi r9, r11, -0x53f0
	ctx.r[9].s64 = ctx.r[11].s64 + -21488;
	// 82827B04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82827B08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82827B0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82827B10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82827B14: 48000008  b 0x82827b1c
	pc = 0x82827B1C; continue 'dispatch;
	// 82827B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827B1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827B20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82827B24: 409A0044  bne cr6, 0x82827b68
	if !ctx.cr[6].eq {
	pc = 0x82827B68; continue 'dispatch;
	}
	// 82827B28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82827B2C: 419A001C  beq cr6, 0x82827b48
	if ctx.cr[6].eq {
	pc = 0x82827B48; continue 'dispatch;
	}
	// 82827B30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827B34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82827B38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827B3C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82827B40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82827B44: 4E800421  bctrl
	ctx.lr = 0x82827B48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82827B48: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82827B4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82827B50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827B54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82827B58: 816B0580  lwz r11, 0x580(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82827B5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82827B60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82827B64: 4BA9849D  bl 0x822c0000
	ctx.lr = 0x82827B68;
	sub_822C0000(ctx, base);
	// 82827B68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82827B6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827B78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827B88 size=196
    let mut pc: u32 = 0x82827B88;
    'dispatch: loop {
        match pc {
            0x82827B88 => {
    //   block [0x82827B88..0x82827C4C)
	// 82827B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827B90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82827B94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827B9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82827BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827BA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82827BA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82827BAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827BB0: 4BA98D89  bl 0x822c0938
	ctx.lr = 0x82827BB4;
	sub_822C0938(ctx, base);
	// 82827BB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82827BB8: 41820028  beq 0x82827be0
	if ctx.cr[0].eq {
	pc = 0x82827BE0; continue 'dispatch;
	}
	// 82827BBC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827BC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82827BC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82827BC8: 392BAC24  addi r9, r11, -0x53dc
	ctx.r[9].s64 = ctx.r[11].s64 + -21468;
	// 82827BCC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82827BD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82827BD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82827BD8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82827BDC: 48000008  b 0x82827be4
	pc = 0x82827BE4; continue 'dispatch;
	// 82827BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827BE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827BE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82827BEC: 409A0044  bne cr6, 0x82827c30
	if !ctx.cr[6].eq {
	pc = 0x82827C30; continue 'dispatch;
	}
	// 82827BF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82827BF4: 419A001C  beq cr6, 0x82827c10
	if ctx.cr[6].eq {
	pc = 0x82827C10; continue 'dispatch;
	}
	// 82827BF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827BFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82827C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827C04: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827C08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82827C0C: 4E800421  bctrl
	ctx.lr = 0x82827C10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82827C10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82827C14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82827C18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827C1C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82827C20: 816B0580  lwz r11, 0x580(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82827C24: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82827C28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82827C2C: 4BA983D5  bl 0x822c0000
	ctx.lr = 0x82827C30;
	sub_822C0000(ctx, base);
	// 82827C30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82827C34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827C40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827C50 size=196
    let mut pc: u32 = 0x82827C50;
    'dispatch: loop {
        match pc {
            0x82827C50 => {
    //   block [0x82827C50..0x82827D14)
	// 82827C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82827C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827C64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82827C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827C6C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82827C70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82827C74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827C78: 4BA98CC1  bl 0x822c0938
	ctx.lr = 0x82827C7C;
	sub_822C0938(ctx, base);
	// 82827C7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82827C80: 41820028  beq 0x82827ca8
	if ctx.cr[0].eq {
	pc = 0x82827CA8; continue 'dispatch;
	}
	// 82827C84: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827C88: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82827C8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82827C90: 392BAC38  addi r9, r11, -0x53c8
	ctx.r[9].s64 = ctx.r[11].s64 + -21448;
	// 82827C94: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82827C98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82827C9C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82827CA0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82827CA4: 48000008  b 0x82827cac
	pc = 0x82827CAC; continue 'dispatch;
	// 82827CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827CAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82827CB4: 409A0044  bne cr6, 0x82827cf8
	if !ctx.cr[6].eq {
	pc = 0x82827CF8; continue 'dispatch;
	}
	// 82827CB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82827CBC: 419A001C  beq cr6, 0x82827cd8
	if ctx.cr[6].eq {
	pc = 0x82827CD8; continue 'dispatch;
	}
	// 82827CC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827CC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82827CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827CCC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827CD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82827CD4: 4E800421  bctrl
	ctx.lr = 0x82827CD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82827CD8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82827CDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82827CE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827CE4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82827CE8: 816B0580  lwz r11, 0x580(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82827CEC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82827CF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82827CF4: 4BA9830D  bl 0x822c0000
	ctx.lr = 0x82827CF8;
	sub_822C0000(ctx, base);
	// 82827CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82827CFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827D08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827D0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827D18 size=196
    let mut pc: u32 = 0x82827D18;
    'dispatch: loop {
        match pc {
            0x82827D18 => {
    //   block [0x82827D18..0x82827DDC)
	// 82827D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827D20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82827D24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827D28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827D2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82827D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827D34: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82827D38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82827D3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827D40: 4BA98BF9  bl 0x822c0938
	ctx.lr = 0x82827D44;
	sub_822C0938(ctx, base);
	// 82827D44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82827D48: 41820028  beq 0x82827d70
	if ctx.cr[0].eq {
	pc = 0x82827D70; continue 'dispatch;
	}
	// 82827D4C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827D50: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82827D54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82827D58: 392BAC4C  addi r9, r11, -0x53b4
	ctx.r[9].s64 = ctx.r[11].s64 + -21428;
	// 82827D5C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82827D60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82827D64: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82827D68: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82827D6C: 48000008  b 0x82827d74
	pc = 0x82827D74; continue 'dispatch;
	// 82827D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827D74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82827D7C: 409A0044  bne cr6, 0x82827dc0
	if !ctx.cr[6].eq {
	pc = 0x82827DC0; continue 'dispatch;
	}
	// 82827D80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82827D84: 419A001C  beq cr6, 0x82827da0
	if ctx.cr[6].eq {
	pc = 0x82827DA0; continue 'dispatch;
	}
	// 82827D88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827D8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82827D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827D94: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82827D98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82827D9C: 4E800421  bctrl
	ctx.lr = 0x82827DA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82827DA0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82827DA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82827DA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82827DAC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82827DB0: 816B0580  lwz r11, 0x580(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82827DB4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82827DB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82827DBC: 4BA98245  bl 0x822c0000
	ctx.lr = 0x82827DC0;
	sub_822C0000(ctx, base);
	// 82827DC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82827DC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827DD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827DD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827DE0 size=148
    let mut pc: u32 = 0x82827DE0;
    'dispatch: loop {
        match pc {
            0x82827DE0 => {
    //   block [0x82827DE0..0x82827E74)
	// 82827DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827DF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827DF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827DF8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82827DFC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82827E00: 396BACC4  addi r11, r11, -0x533c
	ctx.r[11].s64 = ctx.r[11].s64 + -21308;
	// 82827E04: 394AACB0  addi r10, r10, -0x5350
	ctx.r[10].s64 = ctx.r[10].s64 + -21328;
	// 82827E08: 3929AC64  addi r9, r9, -0x539c
	ctx.r[9].s64 = ctx.r[9].s64 + -21404;
	// 82827E0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827E10: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82827E14: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82827E18: 807F0270  lwz r3, 0x270(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82827E1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827E20: 419A0008  beq cr6, 0x82827e28
	if ctx.cr[6].eq {
	pc = 0x82827E28; continue 'dispatch;
	}
	// 82827E24: 4BA98A6D  bl 0x822c0890
	ctx.lr = 0x82827E28;
	sub_822C0890(ctx, base);
	// 82827E28: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82827E2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827E30: 419A0008  beq cr6, 0x82827e38
	if ctx.cr[6].eq {
	pc = 0x82827E38; continue 'dispatch;
	}
	// 82827E34: 4BA98A5D  bl 0x822c0890
	ctx.lr = 0x82827E38;
	sub_822C0890(ctx, base);
	// 82827E38: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 82827E3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827E40: 419A0008  beq cr6, 0x82827e48
	if ctx.cr[6].eq {
	pc = 0x82827E48; continue 'dispatch;
	}
	// 82827E44: 4BA98A4D  bl 0x822c0890
	ctx.lr = 0x82827E48;
	sub_822C0890(ctx, base);
	// 82827E48: 807F0258  lwz r3, 0x258(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(600 as u32) ) } as u64;
	// 82827E4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827E50: 419A0008  beq cr6, 0x82827e58
	if ctx.cr[6].eq {
	pc = 0x82827E58; continue 'dispatch;
	}
	// 82827E54: 4BA98A3D  bl 0x822c0890
	ctx.lr = 0x82827E58;
	sub_822C0890(ctx, base);
	// 82827E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827E5C: 4BFF001D  bl 0x82817e78
	ctx.lr = 0x82827E60;
	sub_82817E78(ctx, base);
	// 82827E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82827E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827E78 size=8
    let mut pc: u32 = 0x82827E78;
    'dispatch: loop {
        match pc {
            0x82827E78 => {
    //   block [0x82827E78..0x82827E80)
	// 82827E78: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82827E7C: 4800013C  b 0x82827fb8
	sub_82827FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827E80 size=8
    let mut pc: u32 = 0x82827E80;
    'dispatch: loop {
        match pc {
            0x82827E80 => {
    //   block [0x82827E80..0x82827E88)
	// 82827E80: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82827E84: 48000134  b 0x82827fb8
	sub_82827FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827E88 size=8
    let mut pc: u32 = 0x82827E88;
    'dispatch: loop {
        match pc {
            0x82827E88 => {
    //   block [0x82827E88..0x82827E90)
	// 82827E88: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82827E8C: 480001D4  b 0x82828060
	sub_82828060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827E90 size=8
    let mut pc: u32 = 0x82827E90;
    'dispatch: loop {
        match pc {
            0x82827E90 => {
    //   block [0x82827E90..0x82827E98)
	// 82827E90: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82827E94: 480001CC  b 0x82828060
	sub_82828060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827E98 size=8
    let mut pc: u32 = 0x82827E98;
    'dispatch: loop {
        match pc {
            0x82827E98 => {
    //   block [0x82827E98..0x82827EA0)
	// 82827E98: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82827E9C: 4800028C  b 0x82828128
	sub_82828128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827EA0 size=8
    let mut pc: u32 = 0x82827EA0;
    'dispatch: loop {
        match pc {
            0x82827EA0 => {
    //   block [0x82827EA0..0x82827EA8)
	// 82827EA0: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82827EA4: 48000284  b 0x82828128
	sub_82828128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827EA8 size=8
    let mut pc: u32 = 0x82827EA8;
    'dispatch: loop {
        match pc {
            0x82827EA8 => {
    //   block [0x82827EA8..0x82827EB0)
	// 82827EA8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82827EAC: 48000344  b 0x828281f0
	sub_828281F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82827EB0 size=8
    let mut pc: u32 = 0x82827EB0;
    'dispatch: loop {
        match pc {
            0x82827EB0 => {
    //   block [0x82827EB0..0x82827EB8)
	// 82827EB0: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82827EB4: 4800033C  b 0x828281f0
	sub_828281F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827EB8 size=84
    let mut pc: u32 = 0x82827EB8;
    'dispatch: loop {
        match pc {
            0x82827EB8 => {
    //   block [0x82827EB8..0x82827F0C)
	// 82827EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827EC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827ECC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827ED0: 396BB044  addi r11, r11, -0x4fbc
	ctx.r[11].s64 = ctx.r[11].s64 + -20412;
	// 82827ED4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827ED8: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82827EDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82827EE0: 419A0008  beq cr6, 0x82827ee8
	if ctx.cr[6].eq {
	pc = 0x82827EE8; continue 'dispatch;
	}
	// 82827EE4: 4BA989AD  bl 0x822c0890
	ctx.lr = 0x82827EE8;
	sub_822C0890(ctx, base);
	// 82827EE8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82827EEC: 485CB53D  bl 0x82df3428
	ctx.lr = 0x82827EF0;
	sub_82DF3428(ctx, base);
	// 82827EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827EF4: 48634695  bl 0x82e5c588
	ctx.lr = 0x82827EF8;
	sub_82E5C588(ctx, base);
	// 82827EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82827EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827F04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82827F10 size=168
    let mut pc: u32 = 0x82827F10;
    'dispatch: loop {
        match pc {
            0x82827F10 => {
    //   block [0x82827F10..0x82827FB8)
	// 82827F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827F1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827F24: 4BFF00C5  bl 0x82817fe8
	ctx.lr = 0x82827F28;
	sub_82817FE8(ctx, base);
	// 82827F28: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82827F2C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82827F30: 396BACC4  addi r11, r11, -0x533c
	ctx.r[11].s64 = ctx.r[11].s64 + -21308;
	// 82827F34: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82827F38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82827F3C: 394AACB0  addi r10, r10, -0x5350
	ctx.r[10].s64 = ctx.r[10].s64 + -21328;
	// 82827F40: 3929AC64  addi r9, r9, -0x539c
	ctx.r[9].s64 = ctx.r[9].s64 + -21404;
	// 82827F44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82827F48: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82827F4C: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82827F50: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82827F54: 917F0254  stw r11, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[11].u32 ) };
	// 82827F58: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82827F5C: 917F0258  stw r11, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[11].u32 ) };
	// 82827F60: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82827F64: 917F025C  stw r11, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[11].u32 ) };
	// 82827F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827F6C: 917F0260  stw r11, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[11].u32 ) };
	// 82827F70: 917F0264  stw r11, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[11].u32 ) };
	// 82827F74: C1AA08A4  lfs f13, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82827F78: 917F0268  stw r11, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[11].u32 ) };
	// 82827F7C: C0099524  lfs f0, -0x6adc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82827F80: 917F026C  stw r11, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[11].u32 ) };
	// 82827F84: C188D7BC  lfs f12, -0x2844(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82827F88: 917F0270  stw r11, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[11].u32 ) };
	// 82827F8C: D1BF0274  stfs f13, 0x274(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), tmp.u32 ) };
	// 82827F90: D01F0280  stfs f0, 0x280(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(640 as u32), tmp.u32 ) };
	// 82827F94: 917F0278  stw r11, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[11].u32 ) };
	// 82827F98: D19F0284  stfs f12, 0x284(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), tmp.u32 ) };
	// 82827F9C: 917F027C  stw r11, 0x27c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(636 as u32), ctx.r[11].u32 ) };
	// 82827FA0: D01F0288  stfs f0, 0x288(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), tmp.u32 ) };
	// 82827FA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82827FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82827FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82827FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82827FB8 size=76
    let mut pc: u32 = 0x82827FB8;
    'dispatch: loop {
        match pc {
            0x82827FB8 => {
    //   block [0x82827FB8..0x82828004)
	// 82827FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82827FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82827FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82827FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82827FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82827FCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82827FD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82827FD4: 4BFFFE0D  bl 0x82827de0
	ctx.lr = 0x82827FD8;
	sub_82827DE0(ctx, base);
	// 82827FD8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82827FDC: 4182000C  beq 0x82827fe8
	if ctx.cr[0].eq {
	pc = 0x82827FE8; continue 'dispatch;
	}
	// 82827FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827FE4: 485CA3F5  bl 0x82df23d8
	ctx.lr = 0x82827FE8;
	sub_82DF23D8(ctx, base);
	// 82827FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82827FEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82827FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82827FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82827FF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82827FFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82828000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82828008 size=84
    let mut pc: u32 = 0x82828008;
    'dispatch: loop {
        match pc {
            0x82828008 => {
    //   block [0x82828008..0x8282805C)
	// 82828008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282800C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82828010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82828014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282801C: 4BFFFEF5  bl 0x82827f10
	ctx.lr = 0x82828020;
	sub_82827F10(ctx, base);
	// 82828020: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82828024: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82828028: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282802C: 396BADBC  addi r11, r11, -0x5244
	ctx.r[11].s64 = ctx.r[11].s64 + -21060;
	// 82828030: 394AADA8  addi r10, r10, -0x5258
	ctx.r[10].s64 = ctx.r[10].s64 + -21080;
	// 82828034: 3929AD5C  addi r9, r9, -0x52a4
	ctx.r[9].s64 = ctx.r[9].s64 + -21156;
	// 82828038: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282803C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82828040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828044: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82828048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82828050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82828054: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82828058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82828060 size=112
    let mut pc: u32 = 0x82828060;
    'dispatch: loop {
        match pc {
            0x82828060 => {
    //   block [0x82828060..0x828280D0)
	// 82828060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82828064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82828068: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282806C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82828070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82828078: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282807C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82828080: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82828084: 396BADBC  addi r11, r11, -0x5244
	ctx.r[11].s64 = ctx.r[11].s64 + -21060;
	// 82828088: 394AADA8  addi r10, r10, -0x5258
	ctx.r[10].s64 = ctx.r[10].s64 + -21080;
	// 8282808C: 3929AD5C  addi r9, r9, -0x52a4
	ctx.r[9].s64 = ctx.r[9].s64 + -21156;
	// 82828090: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82828094: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82828098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282809C: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 828280A0: 4BFFFD41  bl 0x82827de0
	ctx.lr = 0x828280A4;
	sub_82827DE0(ctx, base);
	// 828280A4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828280A8: 4182000C  beq 0x828280b4
	if ctx.cr[0].eq {
	pc = 0x828280B4; continue 'dispatch;
	}
	// 828280AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828280B0: 485CA329  bl 0x82df23d8
	ctx.lr = 0x828280B4;
	sub_82DF23D8(ctx, base);
	// 828280B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828280B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828280BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828280C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828280C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828280C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828280CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828280D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828280D0 size=84
    let mut pc: u32 = 0x828280D0;
    'dispatch: loop {
        match pc {
            0x828280D0 => {
    //   block [0x828280D0..0x82828124)
	// 828280D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828280D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828280D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828280DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828280E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828280E4: 4BFFFE2D  bl 0x82827f10
	ctx.lr = 0x828280E8;
	sub_82827F10(ctx, base);
	// 828280E8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828280EC: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 828280F0: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 828280F4: 396BAEB4  addi r11, r11, -0x514c
	ctx.r[11].s64 = ctx.r[11].s64 + -20812;
	// 828280F8: 394AAEA0  addi r10, r10, -0x5160
	ctx.r[10].s64 = ctx.r[10].s64 + -20832;
	// 828280FC: 3929AE54  addi r9, r9, -0x51ac
	ctx.r[9].s64 = ctx.r[9].s64 + -20908;
	// 82828100: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82828104: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82828108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282810C: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82828110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82828114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82828118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282811C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82828120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82828128 size=112
    let mut pc: u32 = 0x82828128;
    'dispatch: loop {
        match pc {
            0x82828128 => {
    //   block [0x82828128..0x82828198)
	// 82828128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282812C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82828130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82828134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82828138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282813C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82828140: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82828144: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82828148: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282814C: 396BAEB4  addi r11, r11, -0x514c
	ctx.r[11].s64 = ctx.r[11].s64 + -20812;
	// 82828150: 394AAEA0  addi r10, r10, -0x5160
	ctx.r[10].s64 = ctx.r[10].s64 + -20832;
	// 82828154: 3929AE54  addi r9, r9, -0x51ac
	ctx.r[9].s64 = ctx.r[9].s64 + -20908;
	// 82828158: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282815C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82828160: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82828164: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82828168: 4BFFFC79  bl 0x82827de0
	ctx.lr = 0x8282816C;
	sub_82827DE0(ctx, base);
	// 8282816C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82828170: 4182000C  beq 0x8282817c
	if ctx.cr[0].eq {
	pc = 0x8282817C; continue 'dispatch;
	}
	// 82828174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828178: 485CA261  bl 0x82df23d8
	ctx.lr = 0x8282817C;
	sub_82DF23D8(ctx, base);
	// 8282817C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82828184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82828188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282818C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82828190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82828194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82828198 size=84
    let mut pc: u32 = 0x82828198;
    'dispatch: loop {
        match pc {
            0x82828198 => {
    //   block [0x82828198..0x828281EC)
	// 82828198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282819C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828281A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828281A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828281A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828281AC: 4BFFFD65  bl 0x82827f10
	ctx.lr = 0x828281B0;
	sub_82827F10(ctx, base);
	// 828281B0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828281B4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 828281B8: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 828281BC: 396BAFAC  addi r11, r11, -0x5054
	ctx.r[11].s64 = ctx.r[11].s64 + -20564;
	// 828281C0: 394AAF98  addi r10, r10, -0x5068
	ctx.r[10].s64 = ctx.r[10].s64 + -20584;
	// 828281C4: 3929AF4C  addi r9, r9, -0x50b4
	ctx.r[9].s64 = ctx.r[9].s64 + -20660;
	// 828281C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828281CC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 828281D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828281D4: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 828281D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 828281DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828281E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828281E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828281E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828281F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828281F0 size=112
    let mut pc: u32 = 0x828281F0;
    'dispatch: loop {
        match pc {
            0x828281F0 => {
    //   block [0x828281F0..0x82828260)
	// 828281F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828281F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828281F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828281FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82828200: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82828208: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282820C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82828210: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82828214: 396BAFAC  addi r11, r11, -0x5054
	ctx.r[11].s64 = ctx.r[11].s64 + -20564;
	// 82828218: 394AAF98  addi r10, r10, -0x5068
	ctx.r[10].s64 = ctx.r[10].s64 + -20584;
	// 8282821C: 3929AF4C  addi r9, r9, -0x50b4
	ctx.r[9].s64 = ctx.r[9].s64 + -20660;
	// 82828220: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82828224: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82828228: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282822C: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82828230: 4BFFFBB1  bl 0x82827de0
	ctx.lr = 0x82828234;
	sub_82827DE0(ctx, base);
	// 82828234: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82828238: 4182000C  beq 0x82828244
	if ctx.cr[0].eq {
	pc = 0x82828244; continue 'dispatch;
	}
	// 8282823C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828240: 485CA199  bl 0x82df23d8
	ctx.lr = 0x82828244;
	sub_82DF23D8(ctx, base);
	// 82828244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828248: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282824C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82828250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82828254: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82828258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282825C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82828260 size=116
    let mut pc: u32 = 0x82828260;
    'dispatch: loop {
        match pc {
            0x82828260 => {
    //   block [0x82828260..0x828282D4)
	// 82828260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82828264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82828268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282826C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82828270: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82828274: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282827C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82828280: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82828284: 4803A895  bl 0x82862b18
	ctx.lr = 0x82828288;
	sub_82862B18(ctx, base);
	// 82828288: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282828C: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 82828290: 396BB044  addi r11, r11, -0x4fbc
	ctx.r[11].s64 = ctx.r[11].s64 + -20412;
	// 82828294: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82828298: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282829C: 4BD30335  bl 0x825585d0
	ctx.lr = 0x828282A0;
	sub_825585D0(ctx, base);
	// 828282A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828282A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828282A8: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 828282AC: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 828282B0: D3FF007C  stfs f31, 0x7c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 828282B4: D3FF0080  stfs f31, 0x80(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 828282B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828282BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828282C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828282C4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 828282C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828282CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828282D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828282D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828282D8 size=76
    let mut pc: u32 = 0x828282D8;
    'dispatch: loop {
        match pc {
            0x828282D8 => {
    //   block [0x828282D8..0x82828324)
	// 828282D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828282DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828282E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828282E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828282E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828282EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828282F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828282F4: 4BFFFBC5  bl 0x82827eb8
	ctx.lr = 0x828282F8;
	sub_82827EB8(ctx, base);
	// 828282F8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828282FC: 4182000C  beq 0x82828308
	if ctx.cr[0].eq {
	pc = 0x82828308; continue 'dispatch;
	}
	// 82828300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828304: 485CA0D5  bl 0x82df23d8
	ctx.lr = 0x82828308;
	sub_82DF23D8(ctx, base);
	// 82828308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282830C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82828310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82828314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82828318: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282831C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82828320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82828328 size=196
    let mut pc: u32 = 0x82828328;
    'dispatch: loop {
        match pc {
            0x82828328 => {
    //   block [0x82828328..0x828283EC)
	// 82828328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282832C: 4897FE41  bl 0x831a816c
	ctx.lr = 0x82828330;
	sub_831A8130(ctx, base);
	// 82828330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828334: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82828338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282833C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82828340: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82828344: 388BAB3C  addi r4, r11, -0x54c4
	ctx.r[4].s64 = ctx.r[11].s64 + -21700;
	// 82828348: 485CB6C1  bl 0x82df3a08
	ctx.lr = 0x8282834C;
	sub_82DF3A08(ctx, base);
	// 8282834C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82828350: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82828354: 4BFC742D  bl 0x827ef780
	ctx.lr = 0x82828358;
	sub_827EF780(ctx, base);
	// 82828358: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8282835C: 4BFC2185  bl 0x827ea4e0
	ctx.lr = 0x82828360;
	sub_827EA4E0(ctx, base);
	// 82828360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82828364: 485CB0C5  bl 0x82df3428
	ctx.lr = 0x82828368;
	sub_82DF3428(ctx, base);
	// 82828368: 3BBF0070  addi r29, r31, 0x70
	ctx.r[29].s64 = ctx.r[31].s64 + 112;
	// 8282836C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82828370: 485CB839  bl 0x82df3ba8
	ctx.lr = 0x82828374;
	sub_82DF3BA8(ctx, base);
	// 82828374: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82828378: 40820048  bne 0x828283c0
	if !ctx.cr[0].eq {
	pc = 0x828283C0; continue 'dispatch;
	}
	// 8282837C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82828380: 485CAE31  bl 0x82df31b0
	ctx.lr = 0x82828384;
	sub_82DF31B0(ctx, base);
	// 82828384: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82828388: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282838C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82828390: 4BFC7939  bl 0x827efcc8
	ctx.lr = 0x82828394;
	sub_827EFCC8(ctx, base);
	// 82828394: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82828398: 395F0074  addi r10, r31, 0x74
	ctx.r[10].s64 = ctx.r[31].s64 + 116;
	// 8282839C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 828283A0: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 828283A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 828283A8: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 828283AC: 4BA9C0B5  bl 0x822c4460
	ctx.lr = 0x828283B0;
	sub_822C4460(ctx, base);
	// 828283B0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828283B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828283B8: 419A0008  beq cr6, 0x828283c0
	if ctx.cr[6].eq {
	pc = 0x828283C0; continue 'dispatch;
	}
	// 828283BC: 4BA984D5  bl 0x822c0890
	ctx.lr = 0x828283C0;
	sub_822C0890(ctx, base);
	// 828283C0: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 828283C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 828283C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 828283CC: 419A0018  beq cr6, 0x828283e4
	if ctx.cr[6].eq {
	pc = 0x828283E4; continue 'dispatch;
	}
	// 828283D0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 828283D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828283D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 828283DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828283E0: 4E800421  bctrl
	ctx.lr = 0x828283E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828283E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 828283E8: 4897FDD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828283F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828283F0 size=416
    let mut pc: u32 = 0x828283F0;
    'dispatch: loop {
        match pc {
            0x828283F0 => {
    //   block [0x828283F0..0x82828590)
	// 828283F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828283F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828283F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828283FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82828400: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82828404: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82828408: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282840C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82828410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82828414: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82828418: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282841C: 388B4A98  addi r4, r11, 0x4a98
	ctx.r[4].s64 = ctx.r[11].s64 + 19096;
	// 82828420: 485CB5E9  bl 0x82df3a08
	ctx.lr = 0x82828424;
	sub_82DF3A08(ctx, base);
	// 82828424: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82828428: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8282842C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82828430: 389F0074  addi r4, r31, 0x74
	ctx.r[4].s64 = ctx.r[31].s64 + 116;
	// 82828434: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82828438: 4BD7AF71  bl 0x825a33a8
	ctx.lr = 0x8282843C;
	sub_825A33A8(ctx, base);
	// 8282843C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82828440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82828444: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82828448: 4BD79A91  bl 0x825a1ed8
	ctx.lr = 0x8282844C;
	sub_825A1ED8(ctx, base);
	// 8282844C: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 82828450: 485CAFD9  bl 0x82df3428
	ctx.lr = 0x82828454;
	sub_82DF3428(ctx, base);
	// 82828454: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 82828458: 4BAA0861  bl 0x822c8cb8
	ctx.lr = 0x8282845C;
	sub_822C8CB8(ctx, base);
	// 8282845C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82828460: 485CAFC9  bl 0x82df3428
	ctx.lr = 0x82828464;
	sub_82DF3428(ctx, base);
	// 82828464: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82828468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282846C: 388BEDA0  addi r4, r11, -0x1260
	ctx.r[4].s64 = ctx.r[11].s64 + -4704;
	// 82828470: 485CB599  bl 0x82df3a08
	ctx.lr = 0x82828474;
	sub_82DF3A08(ctx, base);
	// 82828474: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82828478: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8282847C: 389F0078  addi r4, r31, 0x78
	ctx.r[4].s64 = ctx.r[31].s64 + 120;
	// 82828480: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82828484: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82828488: C3CA08AC  lfs f30, 0x8ac(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2220 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8282848C: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82828490: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82828494: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82828498: 4BD7AD19  bl 0x825a31b0
	ctx.lr = 0x8282849C;
	sub_825A31B0(ctx, base);
	// 8282849C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 828284A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828284A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828284A8: 4BD792C9  bl 0x825a1770
	ctx.lr = 0x828284AC;
	sub_825A1770(ctx, base);
	// 828284AC: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 828284B0: 485CAF79  bl 0x82df3428
	ctx.lr = 0x828284B4;
	sub_82DF3428(ctx, base);
	// 828284B4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 828284B8: 4BAA0801  bl 0x822c8cb8
	ctx.lr = 0x828284BC;
	sub_822C8CB8(ctx, base);
	// 828284BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828284C0: 485CAF69  bl 0x82df3428
	ctx.lr = 0x828284C4;
	sub_82DF3428(ctx, base);
	// 828284C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 828284C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828284CC: 388BED88  addi r4, r11, -0x1278
	ctx.r[4].s64 = ctx.r[11].s64 + -4728;
	// 828284D0: 485CB539  bl 0x82df3a08
	ctx.lr = 0x828284D4;
	sub_82DF3A08(ctx, base);
	// 828284D4: 389F007C  addi r4, r31, 0x7c
	ctx.r[4].s64 = ctx.r[31].s64 + 124;
	// 828284D8: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 828284DC: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 828284E0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 828284E4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 828284E8: 4BD7ACC9  bl 0x825a31b0
	ctx.lr = 0x828284EC;
	sub_825A31B0(ctx, base);
	// 828284EC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 828284F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828284F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828284F8: 4BD79279  bl 0x825a1770
	ctx.lr = 0x828284FC;
	sub_825A1770(ctx, base);
	// 828284FC: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82828500: 485CAF29  bl 0x82df3428
	ctx.lr = 0x82828504;
	sub_82DF3428(ctx, base);
	// 82828504: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82828508: 4BAA07B1  bl 0x822c8cb8
	ctx.lr = 0x8282850C;
	sub_822C8CB8(ctx, base);
	// 8282850C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82828510: 485CAF19  bl 0x82df3428
	ctx.lr = 0x82828514;
	sub_82DF3428(ctx, base);
	// 82828514: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82828518: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282851C: 388BED64  addi r4, r11, -0x129c
	ctx.r[4].s64 = ctx.r[11].s64 + -4764;
	// 82828520: 485CB4E9  bl 0x82df3a08
	ctx.lr = 0x82828524;
	sub_82DF3A08(ctx, base);
	// 82828524: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 82828528: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 8282852C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82828530: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82828534: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82828538: 4BD7AC79  bl 0x825a31b0
	ctx.lr = 0x8282853C;
	sub_825A31B0(ctx, base);
	// 8282853C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82828540: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82828544: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82828548: 4BD79229  bl 0x825a1770
	ctx.lr = 0x8282854C;
	sub_825A1770(ctx, base);
	// 8282854C: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 82828550: 485CAED9  bl 0x82df3428
	ctx.lr = 0x82828554;
	sub_82DF3428(ctx, base);
	// 82828554: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 82828558: 4BAA0761  bl 0x822c8cb8
	ctx.lr = 0x8282855C;
	sub_822C8CB8(ctx, base);
	// 8282855C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82828560: 485CAEC9  bl 0x82df3428
	ctx.lr = 0x82828564;
	sub_82DF3428(ctx, base);
	// 82828564: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82828568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282856C: 4BFF03FD  bl 0x82818968
	ctx.lr = 0x82828570;
	sub_82818968(ctx, base);
	// 82828570: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 82828574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82828578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282857C: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82828580: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82828584: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82828588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282858C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82828590 size=680
    let mut pc: u32 = 0x82828590;
    'dispatch: loop {
        match pc {
            0x82828590 => {
    //   block [0x82828590..0x82828838)
	// 82828590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82828594: 4897FBD9  bl 0x831a816c
	ctx.lr = 0x82828598;
	sub_831A8130(ctx, base);
	// 82828598: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282859C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 828285A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828285A4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828285A8: 419A0288  beq cr6, 0x82828830
	if ctx.cr[6].eq {
	pc = 0x82828830; continue 'dispatch;
	}
	// 828285AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 828285B0: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 828285B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828285B8: 4E800421  bctrl
	ctx.lr = 0x828285BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828285BC: 397E0254  addi r11, r30, 0x254
	ctx.r[11].s64 = ctx.r[30].s64 + 596;
	// 828285C0: 395E0264  addi r10, r30, 0x264
	ctx.r[10].s64 = ctx.r[30].s64 + 612;
	// 828285C4: 393E026C  addi r9, r30, 0x26c
	ctx.r[9].s64 = ctx.r[30].s64 + 620;
	// 828285C8: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 828285CC: 91410094  stw r10, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 828285D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828285D4: 91210098  stw r9, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[9].u32 ) };
	// 828285D8: 3861009C  addi r3, r1, 0x9c
	ctx.r[3].s64 = ctx.r[1].s64 + 156;
	// 828285DC: 485CB42D  bl 0x82df3a08
	ctx.lr = 0x828285E0;
	sub_82DF3A08(ctx, base);
	// 828285E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 828285E4: 4BFC7185  bl 0x827ef768
	ctx.lr = 0x828285E8;
	sub_827EF768(ctx, base);
	// 828285E8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828285EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828285F0: 388BAB54  addi r4, r11, -0x54ac
	ctx.r[4].s64 = ctx.r[11].s64 + -21676;
	// 828285F4: 485CB415  bl 0x82df3a08
	ctx.lr = 0x828285F8;
	sub_82DF3A08(ctx, base);
	// 828285F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828285FC: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82828600: 4803B0C1  bl 0x828636c0
	ctx.lr = 0x82828604;
	sub_828636C0(ctx, base);
	// 82828604: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82828608: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282860C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82828610: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82828614: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82828618: 419A0024  beq cr6, 0x8282863c
	if ctx.cr[6].eq {
	pc = 0x8282863C; continue 'dispatch;
	}
	// 8282861C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82828620: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82828624: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82828628: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282862C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82828630: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82828634: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82828638: 4082FFE8  bne 0x82828620
	if !ctx.cr[0].eq {
	pc = 0x82828620; continue 'dispatch;
	}
	// 8282863C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82828640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82828644: 4BFC705D  bl 0x827ef6a0
	ctx.lr = 0x82828648;
	sub_827EF6A0(ctx, base);
	// 82828648: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282864C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828650: 419A0008  beq cr6, 0x82828658
	if ctx.cr[6].eq {
	pc = 0x82828658; continue 'dispatch;
	}
	// 82828654: 4BA9823D  bl 0x822c0890
	ctx.lr = 0x82828658;
	sub_822C0890(ctx, base);
	// 82828658: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8282865C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828660: 419A0008  beq cr6, 0x82828668
	if ctx.cr[6].eq {
	pc = 0x82828668; continue 'dispatch;
	}
	// 82828664: 4BA9822D  bl 0x822c0890
	ctx.lr = 0x82828668;
	sub_822C0890(ctx, base);
	// 82828668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282866C: 485CADBD  bl 0x82df3428
	ctx.lr = 0x82828670;
	sub_82DF3428(ctx, base);
	// 82828670: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82828674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82828678: 388BB080  addi r4, r11, -0x4f80
	ctx.r[4].s64 = ctx.r[11].s64 + -20352;
	// 8282867C: 38A00118  li r5, 0x118
	ctx.r[5].s64 = 280;
	// 82828680: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 82828684: 485C9D65  bl 0x82df23e8
	ctx.lr = 0x82828688;
	sub_82DF23E8(ctx, base);
	// 82828688: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282868C: 41820018  beq 0x828286a4
	if ctx.cr[0].eq {
	pc = 0x828286A4; continue 'dispatch;
	}
	// 82828690: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82828694: C03E0284  lfs f1, 0x284(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(644 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82828698: 4BFFFBC9  bl 0x82828260
	ctx.lr = 0x8282869C;
	sub_82828260(ctx, base);
	// 8282869C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828286A0: 48000008  b 0x828286a8
	pc = 0x828286A8; continue 'dispatch;
	// 828286A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828286A8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 828286AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828286B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828286B4: 4BFFF40D  bl 0x82827ac0
	ctx.lr = 0x828286B8;
	sub_82827AC0(ctx, base);
	// 828286B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828286BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828286C0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828286C4: 4BA9793D  bl 0x822c0000
	ctx.lr = 0x828286C8;
	sub_822C0000(ctx, base);
	// 828286C8: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 828286CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828286D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828286D4: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 828286D8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 828286DC: 419A0024  beq cr6, 0x82828700
	if ctx.cr[6].eq {
	pc = 0x82828700; continue 'dispatch;
	}
	// 828286E0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 828286E4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828286E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828286EC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828286F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828286F4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828286F8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828286FC: 4082FFE8  bne 0x828286e4
	if !ctx.cr[0].eq {
	pc = 0x828286E4; continue 'dispatch;
	}
	// 82828700: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82828704: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82828708: 4BFC6F99  bl 0x827ef6a0
	ctx.lr = 0x8282870C;
	sub_827EF6A0(ctx, base);
	// 8282870C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82828710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828714: 419A0008  beq cr6, 0x8282871c
	if ctx.cr[6].eq {
	pc = 0x8282871C; continue 'dispatch;
	}
	// 82828718: 4BA98179  bl 0x822c0890
	ctx.lr = 0x8282871C;
	sub_822C0890(ctx, base);
	// 8282871C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82828720: 419A000C  beq cr6, 0x8282872c
	if ctx.cr[6].eq {
	pc = 0x8282872C; continue 'dispatch;
	}
	// 82828724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828728: 4BA98169  bl 0x822c0890
	ctx.lr = 0x8282872C;
	sub_822C0890(ctx, base);
	// 8282872C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82828730: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82828734: 388BAB48  addi r4, r11, -0x54b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21688;
	// 82828738: 485CB2D1  bl 0x82df3a08
	ctx.lr = 0x8282873C;
	sub_82DF3A08(ctx, base);
	// 8282873C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82828740: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82828744: 4803AF7D  bl 0x828636c0
	ctx.lr = 0x82828748;
	sub_828636C0(ctx, base);
	// 82828748: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282874C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82828750: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82828754: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82828758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8282875C: 419A0024  beq cr6, 0x82828780
	if ctx.cr[6].eq {
	pc = 0x82828780; continue 'dispatch;
	}
	// 82828760: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82828764: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82828768: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282876C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82828770: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82828774: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82828778: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282877C: 4082FFE8  bne 0x82828764
	if !ctx.cr[0].eq {
	pc = 0x82828764; continue 'dispatch;
	}
	// 82828780: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82828784: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82828788: 4BFC6F19  bl 0x827ef6a0
	ctx.lr = 0x8282878C;
	sub_827EF6A0(ctx, base);
	// 8282878C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82828790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828794: 419A0008  beq cr6, 0x8282879c
	if ctx.cr[6].eq {
	pc = 0x8282879C; continue 'dispatch;
	}
	// 82828798: 4BA980F9  bl 0x822c0890
	ctx.lr = 0x8282879C;
	sub_822C0890(ctx, base);
	// 8282879C: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 828287A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828287A4: 419A0008  beq cr6, 0x828287ac
	if ctx.cr[6].eq {
	pc = 0x828287AC; continue 'dispatch;
	}
	// 828287A8: 4BA980E9  bl 0x822c0890
	ctx.lr = 0x828287AC;
	sub_822C0890(ctx, base);
	// 828287AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828287B0: 485CAC79  bl 0x82df3428
	ctx.lr = 0x828287B4;
	sub_82DF3428(ctx, base);
	// 828287B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 828287B8: 389E0278  addi r4, r30, 0x278
	ctx.r[4].s64 = ctx.r[30].s64 + 632;
	// 828287BC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 828287C0: 4803B351  bl 0x82863b10
	ctx.lr = 0x828287C4;
	sub_82863B10(ctx, base);
	// 828287C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828287C8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 828287CC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828287D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828287D4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 828287D8: 419A0024  beq cr6, 0x828287fc
	if ctx.cr[6].eq {
	pc = 0x828287FC; continue 'dispatch;
	}
	// 828287DC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828287E0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828287E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828287E8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828287EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828287F0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828287F4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828287F8: 4082FFE8  bne 0x828287e0
	if !ctx.cr[0].eq {
	pc = 0x828287E0; continue 'dispatch;
	}
	// 828287FC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82828800: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82828804: 4BFC6E9D  bl 0x827ef6a0
	ctx.lr = 0x82828808;
	sub_827EF6A0(ctx, base);
	// 82828808: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8282880C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828810: 419A0008  beq cr6, 0x82828818
	if ctx.cr[6].eq {
	pc = 0x82828818; continue 'dispatch;
	}
	// 82828814: 4BA9807D  bl 0x822c0890
	ctx.lr = 0x82828818;
	sub_822C0890(ctx, base);
	// 82828818: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8282881C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828820: 419A0008  beq cr6, 0x82828828
	if ctx.cr[6].eq {
	pc = 0x82828828; continue 'dispatch;
	}
	// 82828824: 4BA9806D  bl 0x822c0890
	ctx.lr = 0x82828828;
	sub_822C0890(ctx, base);
	// 82828828: 3861009C  addi r3, r1, 0x9c
	ctx.r[3].s64 = ctx.r[1].s64 + 156;
	// 8282882C: 485CABFD  bl 0x82df3428
	ctx.lr = 0x82828830;
	sub_82DF3428(ctx, base);
	// 82828830: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82828834: 4897F988  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82828838 size=1320
    let mut pc: u32 = 0x82828838;
    'dispatch: loop {
        match pc {
            0x82828838 => {
    //   block [0x82828838..0x82828D60)
	// 82828838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282883C: 4897F915  bl 0x831a8150
	ctx.lr = 0x82828840;
	sub_831A8130(ctx, base);
	// 82828840: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82828844: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282884C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82828850: 4BFF19E9  bl 0x8281a238
	ctx.lr = 0x82828854;
	sub_8281A238(ctx, base);
	// 82828854: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82828858: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8282885C: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82828860: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82828864: 7C79D02E  lwzx r3, r25, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82828868: 48677EC9  bl 0x82ea0730
	ctx.lr = 0x8282886C;
	sub_82EA0730(ctx, base);
	// 8282886C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82828870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82828874: 3B6BBC40  addi r27, r11, -0x43c0
	ctx.r[27].s64 = ctx.r[11].s64 + -17344;
	// 82828878: 396A2900  addi r11, r10, 0x2900
	ctx.r[11].s64 = ctx.r[10].s64 + 10496;
	// 8282887C: 394100F0  addi r10, r1, 0xf0
	ctx.r[10].s64 = ctx.r[1].s64 + 240;
	// 82828880: 39210100  addi r9, r1, 0x100
	ctx.r[9].s64 = ctx.r[1].s64 + 256;
	// 82828884: 3AE00060  li r23, 0x60
	ctx.r[23].s64 = 96;
	// 82828888: 13E0D8C7  vcmpequd (lvx128) v31, v0, v27
	tmp.u32 = ctx.r[27].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282888C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82828890: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82828894: B2E30004  sth r23, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[23].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82828D60 size=136
    let mut pc: u32 = 0x82828D60;
    'dispatch: loop {
        match pc {
            0x82828D60 => {
    //   block [0x82828D60..0x82828DE8)
	// 82828D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82828D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82828D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82828D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82828D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82828D78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82828D7C: 4BFEE90D  bl 0x82817688
	ctx.lr = 0x82828D80;
	sub_82817688(ctx, base);
	// 82828D80: 817F0278  lwz r11, 0x278(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 82828D84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82828D88: 409A0048  bne cr6, 0x82828dd0
	if !ctx.cr[6].eq {
	pc = 0x82828DD0; continue 'dispatch;
	}
	// 82828D8C: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82828D90: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82828D94: C1BF0274  lfs f13, 0x274(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(628 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82828D98: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82828D9C: D1BF0274  stfs f13, 0x274(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), tmp.u32 ) };
	// 82828DA0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82828DA4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82828DA8: 40980028  bge cr6, 0x82828dd0
	if !ctx.cr[6].lt {
	pc = 0x82828DD0; continue 'dispatch;
	}
	// 82828DAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82828DB0: C01F0288  lfs f0, 0x288(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(648 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82828DB4: D01F0274  stfs f0, 0x274(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), tmp.u32 ) };
	// 82828DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828DBC: 917F0278  stw r11, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[11].u32 ) };
	// 82828DC0: 481377A1  bl 0x82960560
	ctx.lr = 0x82828DC4;
	sub_82960560(ctx, base);
	// 82828DC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82828DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828DCC: 4BFFF7C5  bl 0x82828590
	ctx.lr = 0x82828DD0;
	sub_82828590(ctx, base);
	// 82828DD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82828DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82828DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82828DDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82828DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82828DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82828DE8 size=4
    let mut pc: u32 = 0x82828DE8;
    'dispatch: loop {
        match pc {
            0x82828DE8 => {
    //   block [0x82828DE8..0x82828DEC)
	// 82828DE8: 4BFFFA50  b 0x82828838
	sub_82828838(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82828DF0 size=476
    let mut pc: u32 = 0x82828DF0;
    'dispatch: loop {
        match pc {
            0x82828DF0 => {
    //   block [0x82828DF0..0x82828FCC)
	// 82828DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82828DF4: 4897F371  bl 0x831a8164
	ctx.lr = 0x82828DF8;
	sub_831A8130(ctx, base);
	// 82828DF8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82828DFC: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828E00: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82828E04: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82828E08: 3BCBB080  addi r30, r11, -0x4f80
	ctx.r[30].s64 = ctx.r[11].s64 + -20352;
	// 82828E0C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82828E10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82828E14: 38A00185  li r5, 0x185
	ctx.r[5].s64 = 389;
	// 82828E18: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82828E1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82828E20: 3860028C  li r3, 0x28c
	ctx.r[3].s64 = 652;
	// 82828E24: 485C95C5  bl 0x82df23e8
	ctx.lr = 0x82828E28;
	sub_82DF23E8(ctx, base);
	// 82828E28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82828E2C: 41820010  beq 0x82828e3c
	if ctx.cr[0].eq {
	pc = 0x82828E3C; continue 'dispatch;
	}
	// 82828E30: 4BFFF1D9  bl 0x82828008
	ctx.lr = 0x82828E34;
	sub_82828008(ctx, base);
	// 82828E34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82828E38: 48000008  b 0x82828e40
	pc = 0x82828E40; continue 'dispatch;
	// 82828E3C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82828E40: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82828E44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82828E48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82828E4C: 4BFFED3D  bl 0x82827b88
	ctx.lr = 0x82828E50;
	sub_82827B88(ctx, base);
	// 82828E50: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82828E54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82828E58: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82828E5C: 4BA971A5  bl 0x822c0000
	ctx.lr = 0x82828E60;
	sub_822C0000(ctx, base);
	// 82828E60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82828E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82828E68: 38A00186  li r5, 0x186
	ctx.r[5].s64 = 390;
	// 82828E6C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82828E70: 4BA97569  bl 0x822c03d8
	ctx.lr = 0x82828E74;
	sub_822C03D8(ctx, base);
	// 82828E74: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82828E78: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82828E7C: 418200E0  beq 0x82828f5c
	if ctx.cr[0].eq {
	pc = 0x82828F5C; continue 'dispatch;
	}
	// 82828E80: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82828E84: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82828E88: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82828E8C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82828E90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82828E94: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82828E98: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 82828E9C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82828EA0: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82828EA4: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82828EA8: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82828EAC: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82828EB0: 409A0008  bne cr6, 0x82828eb8
	if !ctx.cr[6].eq {
	pc = 0x82828EB8; continue 'dispatch;
	}
	// 82828EB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82828EB8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82828EBC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82828EC0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82828EC4: 419A0024  beq cr6, 0x82828ee8
	if ctx.cr[6].eq {
	pc = 0x82828EE8; continue 'dispatch;
	}
	// 82828EC8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82828ECC: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82828ED0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82828ED4: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82828ED8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82828EDC: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82828EE0: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82828EE4: 4082FFE8  bne 0x82828ecc
	if !ctx.cr[0].eq {
	pc = 0x82828ECC; continue 'dispatch;
	}
	// 82828EE8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82828EEC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82828EF0: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82828EF4: 419A0024  beq cr6, 0x82828f18
	if ctx.cr[6].eq {
	pc = 0x82828F18; continue 'dispatch;
	}
	// 82828EF8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82828EFC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82828F00: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82828F04: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82828F08: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82828F0C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82828F10: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82828F14: 4082FFE8  bne 0x82828efc
	if !ctx.cr[0].eq {
	pc = 0x82828EFC; continue 'dispatch;
	}
	// 82828F18: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82828F1C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82828F20: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82828F24: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82828F28: 48653D19  bl 0x82e7cc40
	ctx.lr = 0x82828F2C;
	sub_82E7CC40(ctx, base);
	// 82828F2C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82828F30: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82828F34: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82828F38: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82828F3C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82828F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82828F44: 4BF6FFED  bl 0x82798f30
	ctx.lr = 0x82828F48;
	sub_82798F30(ctx, base);
	// 82828F48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82828F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82828F50: 4BF72719  bl 0x8279b668
	ctx.lr = 0x82828F54;
	sub_8279B668(ctx, base);
	// 82828F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82828F58: 48000008  b 0x82828f60
	pc = 0x82828F60; continue 'dispatch;
	// 82828F5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82828F60: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82828F64: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 82828F68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82828F6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82828F70: 4BD6ECE1  bl 0x82597c50
	ctx.lr = 0x82828F74;
	sub_82597C50(ctx, base);
	// 82828F74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82828F78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82828F7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82828F80: 4BA97081  bl 0x822c0000
	ctx.lr = 0x82828F84;
	sub_822C0000(ctx, base);
	// 82828F84: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82828F88: 41820024  beq 0x82828fac
	if ctx.cr[0].eq {
	pc = 0x82828FAC; continue 'dispatch;
	}
	// 82828F8C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82828F90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828F94: 419A0008  beq cr6, 0x82828f9c
	if ctx.cr[6].eq {
	pc = 0x82828F9C; continue 'dispatch;
	}
	// 82828F98: 4BA978F9  bl 0x822c0890
	ctx.lr = 0x82828F9C;
	sub_822C0890(ctx, base);
	// 82828F9C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82828FA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82828FA4: 419A0008  beq cr6, 0x82828fac
	if ctx.cr[6].eq {
	pc = 0x82828FAC; continue 'dispatch;
	}
	// 82828FA8: 4BA978E9  bl 0x822c0890
	ctx.lr = 0x82828FAC;
	sub_822C0890(ctx, base);
	// 82828FAC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82828FB0: 419A000C  beq cr6, 0x82828fbc
	if ctx.cr[6].eq {
	pc = 0x82828FBC; continue 'dispatch;
	}
	// 82828FB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82828FB8: 4BA978D9  bl 0x822c0890
	ctx.lr = 0x82828FBC;
	sub_822C0890(ctx, base);
	// 82828FBC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82828FC0: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82828FC4: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82828FC8: 4897F1EC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82828FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82828FD0 size=476
    let mut pc: u32 = 0x82828FD0;
    'dispatch: loop {
        match pc {
            0x82828FD0 => {
    //   block [0x82828FD0..0x828291AC)
	// 82828FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82828FD4: 4897F191  bl 0x831a8164
	ctx.lr = 0x82828FD8;
	sub_831A8130(ctx, base);
	// 82828FD8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82828FDC: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82828FE0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82828FE4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82828FE8: 3BCBB080  addi r30, r11, -0x4f80
	ctx.r[30].s64 = ctx.r[11].s64 + -20352;
	// 82828FEC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82828FF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82828FF4: 38A00204  li r5, 0x204
	ctx.r[5].s64 = 516;
	// 82828FF8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82828FFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829000: 3860028C  li r3, 0x28c
	ctx.r[3].s64 = 652;
	// 82829004: 485C93E5  bl 0x82df23e8
	ctx.lr = 0x82829008;
	sub_82DF23E8(ctx, base);
	// 82829008: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282900C: 41820010  beq 0x8282901c
	if ctx.cr[0].eq {
	pc = 0x8282901C; continue 'dispatch;
	}
	// 82829010: 4BFFF0C1  bl 0x828280d0
	ctx.lr = 0x82829014;
	sub_828280D0(ctx, base);
	// 82829014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82829018: 48000008  b 0x82829020
	pc = 0x82829020; continue 'dispatch;
	// 8282901C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82829020: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82829024: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82829028: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282902C: 4BFFEC25  bl 0x82827c50
	ctx.lr = 0x82829030;
	sub_82827C50(ctx, base);
	// 82829030: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82829034: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82829038: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282903C: 4BA96FC5  bl 0x822c0000
	ctx.lr = 0x82829040;
	sub_822C0000(ctx, base);
	// 82829040: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82829048: 38A00205  li r5, 0x205
	ctx.r[5].s64 = 517;
	// 8282904C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82829050: 4BA97389  bl 0x822c03d8
	ctx.lr = 0x82829054;
	sub_822C03D8(ctx, base);
	// 82829054: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82829058: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282905C: 418200E0  beq 0x8282913c
	if ctx.cr[0].eq {
	pc = 0x8282913C; continue 'dispatch;
	}
	// 82829060: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82829064: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82829068: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282906C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82829070: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82829074: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82829078: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 8282907C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82829080: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82829084: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82829088: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8282908C: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82829090: 409A0008  bne cr6, 0x82829098
	if !ctx.cr[6].eq {
	pc = 0x82829098; continue 'dispatch;
	}
	// 82829094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829098: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282909C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828290A0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 828290A4: 419A0024  beq cr6, 0x828290c8
	if ctx.cr[6].eq {
	pc = 0x828290C8; continue 'dispatch;
	}
	// 828290A8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828290AC: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 828290B0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828290B4: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 828290B8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 828290BC: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828290C0: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828290C4: 4082FFE8  bne 0x828290ac
	if !ctx.cr[0].eq {
	pc = 0x828290AC; continue 'dispatch;
	}
	// 828290C8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 828290CC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828290D0: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 828290D4: 419A0024  beq cr6, 0x828290f8
	if ctx.cr[6].eq {
	pc = 0x828290F8; continue 'dispatch;
	}
	// 828290D8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828290DC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828290E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828290E4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828290E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828290EC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828290F0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828290F4: 4082FFE8  bne 0x828290dc
	if !ctx.cr[0].eq {
	pc = 0x828290DC; continue 'dispatch;
	}
	// 828290F8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 828290FC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82829100: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82829104: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82829108: 48653B39  bl 0x82e7cc40
	ctx.lr = 0x8282910C;
	sub_82E7CC40(ctx, base);
	// 8282910C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82829110: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82829114: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82829118: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8282911C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82829120: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82829124: 4BF6FE0D  bl 0x82798f30
	ctx.lr = 0x82829128;
	sub_82798F30(ctx, base);
	// 82829128: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282912C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829130: 4BF72539  bl 0x8279b668
	ctx.lr = 0x82829134;
	sub_8279B668(ctx, base);
	// 82829134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82829138: 48000008  b 0x82829140
	pc = 0x82829140; continue 'dispatch;
	// 8282913C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82829140: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82829144: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 82829148: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282914C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829150: 4BD6EB01  bl 0x82597c50
	ctx.lr = 0x82829154;
	sub_82597C50(ctx, base);
	// 82829154: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82829158: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282915C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829160: 4BA96EA1  bl 0x822c0000
	ctx.lr = 0x82829164;
	sub_822C0000(ctx, base);
	// 82829164: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82829168: 41820024  beq 0x8282918c
	if ctx.cr[0].eq {
	pc = 0x8282918C; continue 'dispatch;
	}
	// 8282916C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82829170: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82829174: 419A0008  beq cr6, 0x8282917c
	if ctx.cr[6].eq {
	pc = 0x8282917C; continue 'dispatch;
	}
	// 82829178: 4BA97719  bl 0x822c0890
	ctx.lr = 0x8282917C;
	sub_822C0890(ctx, base);
	// 8282917C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82829180: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82829184: 419A0008  beq cr6, 0x8282918c
	if ctx.cr[6].eq {
	pc = 0x8282918C; continue 'dispatch;
	}
	// 82829188: 4BA97709  bl 0x822c0890
	ctx.lr = 0x8282918C;
	sub_822C0890(ctx, base);
	// 8282918C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82829190: 419A000C  beq cr6, 0x8282919c
	if ctx.cr[6].eq {
	pc = 0x8282919C; continue 'dispatch;
	}
	// 82829194: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82829198: 4BA976F9  bl 0x822c0890
	ctx.lr = 0x8282919C;
	sub_822C0890(ctx, base);
	// 8282919C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 828291A0: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 828291A4: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 828291A8: 4897F00C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828291B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828291B0 size=476
    let mut pc: u32 = 0x828291B0;
    'dispatch: loop {
        match pc {
            0x828291B0 => {
    //   block [0x828291B0..0x8282938C)
	// 828291B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828291B4: 4897EFB1  bl 0x831a8164
	ctx.lr = 0x828291B8;
	sub_831A8130(ctx, base);
	// 828291B8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 828291BC: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828291C0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828291C4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 828291C8: 3BCBB080  addi r30, r11, -0x4f80
	ctx.r[30].s64 = ctx.r[11].s64 + -20352;
	// 828291CC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 828291D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828291D4: 38A00283  li r5, 0x283
	ctx.r[5].s64 = 643;
	// 828291D8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 828291DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828291E0: 3860028C  li r3, 0x28c
	ctx.r[3].s64 = 652;
	// 828291E4: 485C9205  bl 0x82df23e8
	ctx.lr = 0x828291E8;
	sub_82DF23E8(ctx, base);
	// 828291E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828291EC: 41820010  beq 0x828291fc
	if ctx.cr[0].eq {
	pc = 0x828291FC; continue 'dispatch;
	}
	// 828291F0: 4BFFEFA9  bl 0x82828198
	ctx.lr = 0x828291F4;
	sub_82828198(ctx, base);
	// 828291F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828291F8: 48000008  b 0x82829200
	pc = 0x82829200; continue 'dispatch;
	// 828291FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82829200: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82829204: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82829208: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282920C: 4BFFEB0D  bl 0x82827d18
	ctx.lr = 0x82829210;
	sub_82827D18(ctx, base);
	// 82829210: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82829214: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82829218: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282921C: 4BA96DE5  bl 0x822c0000
	ctx.lr = 0x82829220;
	sub_822C0000(ctx, base);
	// 82829220: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82829228: 38A00284  li r5, 0x284
	ctx.r[5].s64 = 644;
	// 8282922C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82829230: 4BA971A9  bl 0x822c03d8
	ctx.lr = 0x82829234;
	sub_822C03D8(ctx, base);
	// 82829234: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82829238: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282923C: 418200E0  beq 0x8282931c
	if ctx.cr[0].eq {
	pc = 0x8282931C; continue 'dispatch;
	}
	// 82829240: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82829244: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82829248: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282924C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82829250: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82829254: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82829258: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 8282925C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82829260: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82829264: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82829268: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8282926C: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82829270: 409A0008  bne cr6, 0x82829278
	if !ctx.cr[6].eq {
	pc = 0x82829278; continue 'dispatch;
	}
	// 82829274: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829278: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282927C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82829280: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82829284: 419A0024  beq cr6, 0x828292a8
	if ctx.cr[6].eq {
	pc = 0x828292A8; continue 'dispatch;
	}
	// 82829288: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8282928C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82829290: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82829294: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82829298: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8282929C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828292A0: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828292A4: 4082FFE8  bne 0x8282928c
	if !ctx.cr[0].eq {
	pc = 0x8282928C; continue 'dispatch;
	}
	// 828292A8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 828292AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828292B0: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 828292B4: 419A0024  beq cr6, 0x828292d8
	if ctx.cr[6].eq {
	pc = 0x828292D8; continue 'dispatch;
	}
	// 828292B8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 828292BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828292C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828292C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828292C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828292CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828292D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828292D4: 4082FFE8  bne 0x828292bc
	if !ctx.cr[0].eq {
	pc = 0x828292BC; continue 'dispatch;
	}
	// 828292D8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 828292DC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 828292E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 828292E4: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828292E8: 48653959  bl 0x82e7cc40
	ctx.lr = 0x828292EC;
	sub_82E7CC40(ctx, base);
	// 828292EC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 828292F0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 828292F4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 828292F8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 828292FC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82829300: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82829304: 4BF6FC2D  bl 0x82798f30
	ctx.lr = 0x82829308;
	sub_82798F30(ctx, base);
	// 82829308: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282930C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829310: 4BF72359  bl 0x8279b668
	ctx.lr = 0x82829314;
	sub_8279B668(ctx, base);
	// 82829314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82829318: 48000008  b 0x82829320
	pc = 0x82829320; continue 'dispatch;
	// 8282931C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82829320: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82829324: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 82829328: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282932C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829330: 4BD6E921  bl 0x82597c50
	ctx.lr = 0x82829334;
	sub_82597C50(ctx, base);
	// 82829334: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82829338: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282933C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829340: 4BA96CC1  bl 0x822c0000
	ctx.lr = 0x82829344;
	sub_822C0000(ctx, base);
	// 82829344: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82829348: 41820024  beq 0x8282936c
	if ctx.cr[0].eq {
	pc = 0x8282936C; continue 'dispatch;
	}
	// 8282934C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82829350: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82829354: 419A0008  beq cr6, 0x8282935c
	if ctx.cr[6].eq {
	pc = 0x8282935C; continue 'dispatch;
	}
	// 82829358: 4BA97539  bl 0x822c0890
	ctx.lr = 0x8282935C;
	sub_822C0890(ctx, base);
	// 8282935C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82829360: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82829364: 419A0008  beq cr6, 0x8282936c
	if ctx.cr[6].eq {
	pc = 0x8282936C; continue 'dispatch;
	}
	// 82829368: 4BA97529  bl 0x822c0890
	ctx.lr = 0x8282936C;
	sub_822C0890(ctx, base);
	// 8282936C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82829370: 419A000C  beq cr6, 0x8282937c
	if ctx.cr[6].eq {
	pc = 0x8282937C; continue 'dispatch;
	}
	// 82829374: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82829378: 4BA97519  bl 0x822c0890
	ctx.lr = 0x8282937C;
	sub_822C0890(ctx, base);
	// 8282937C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82829380: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82829384: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82829388: 4897EE2C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829390 size=68
    let mut pc: u32 = 0x82829390;
    'dispatch: loop {
        match pc {
            0x82829390 => {
    //   block [0x82829390..0x828293D4)
	// 82829390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282939C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828293A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828293A4: 808B8638  lwz r4, -0x79c8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31176 as u32) ) } as u64;
	// 828293A8: 4BB03381  bl 0x8232c728
	ctx.lr = 0x828293AC;
	sub_8232C728(ctx, base);
	// 828293AC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 828293B0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828293B4: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 828293B8: 806B8614  lwz r3, -0x79ec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31212 as u32) ) } as u64;
	// 828293BC: E8AA0008  ld r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 828293C0: 4BF90FA9  bl 0x827ba368
	ctx.lr = 0x828293C4;
	sub_827BA368(ctx, base);
	// 828293C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828293C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828293CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828293D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828293D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828293D8 size=72
    let mut pc: u32 = 0x828293D8;
    'dispatch: loop {
        match pc {
            0x828293D8 => {
    //   block [0x828293D8..0x82829420)
	// 828293D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828293DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828293E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828293E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828293E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828293EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828293F0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828293F4: 4BFFFF9D  bl 0x82829390
	ctx.lr = 0x828293F8;
	sub_82829390(ctx, base);
	// 828293F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 828293FC: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82829400: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829404: 916A0060  stw r11, 0x60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82829408: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282940C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829414: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82829418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282941C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829420 size=72
    let mut pc: u32 = 0x82829420;
    'dispatch: loop {
        match pc {
            0x82829420 => {
    //   block [0x82829420..0x82829468)
	// 82829420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282942C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82829438: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282943C: 4BFFFF55  bl 0x82829390
	ctx.lr = 0x82829440;
	sub_82829390(ctx, base);
	// 82829440: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82829444: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82829448: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282944C: 916A0060  stw r11, 0x60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82829450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82829454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282945C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82829460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829468 size=48
    let mut pc: u32 = 0x82829468;
    'dispatch: loop {
        match pc {
            0x82829468 => {
    //   block [0x82829468..0x82829498)
	// 82829468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282946C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829478: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282947C: 4BFFFF15  bl 0x82829390
	ctx.lr = 0x82829480;
	sub_82829390(ctx, base);
	// 82829480: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82829484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82829488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282948C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829498 size=276
    let mut pc: u32 = 0x82829498;
    'dispatch: loop {
        match pc {
            0x82829498 => {
    //   block [0x82829498..0x828295AC)
	// 82829498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282949C: 4897ECCD  bl 0x831a8168
	ctx.lr = 0x828294A0;
	sub_831A8130(ctx, base);
	// 828294A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828294A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 828294A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828294AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828294B0: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 828294B4: 41820038  beq 0x828294ec
	if ctx.cr[0].eq {
	pc = 0x828294EC; continue 'dispatch;
	}
	// 828294B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828294BC: 489804CD  bl 0x831a9988
	ctx.lr = 0x828294C0;
	sub_831A9988(ctx, base);
	// 828294C0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 828294C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828294C8: 386BCAE8  addi r3, r11, -0x3518
	ctx.r[3].s64 = ctx.r[11].s64 + -13592;
	// 828294CC: 4897EC2D  bl 0x831a80f8
	ctx.lr = 0x828294D0;
	sub_831A80F8(ctx, base);
	// 828294D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828294D4: 41820018  beq 0x828294ec
	if ctx.cr[0].eq {
	pc = 0x828294EC; continue 'dispatch;
	}
	// 828294D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828294DC: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 828294E0: 4BFFE449  bl 0x82827928
	ctx.lr = 0x828294E4;
	sub_82827928(ctx, base);
	// 828294E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 828294E8: 480000BC  b 0x828295a4
	pc = 0x828295A4; continue 'dispatch;
	// 828294EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 828294F0: 419A00A4  beq cr6, 0x82829594
	if ctx.cr[6].eq {
	pc = 0x82829594; continue 'dispatch;
	}
	// 828294F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828294F8: 48980491  bl 0x831a9988
	ctx.lr = 0x828294FC;
	sub_831A9988(ctx, base);
	// 828294FC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82829500: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82829504: 386B6C0C  addi r3, r11, 0x6c0c
	ctx.r[3].s64 = ctx.r[11].s64 + 27660;
	// 82829508: 4897EBF1  bl 0x831a80f8
	ctx.lr = 0x8282950C;
	sub_831A80F8(ctx, base);
	// 8282950C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82829510: 41820014  beq 0x82829524
	if ctx.cr[0].eq {
	pc = 0x82829524; continue 'dispatch;
	}
	// 82829514: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82829518: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8282951C: 48041A05  bl 0x8286af20
	ctx.lr = 0x82829520;
	sub_8286AF20(ctx, base);
	// 82829520: 4BFFFFC4  b 0x828294e4
	pc = 0x828294E4; continue 'dispatch;
	// 82829524: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82829528: 419A006C  beq cr6, 0x82829594
	if ctx.cr[6].eq {
	pc = 0x82829594; continue 'dispatch;
	}
	// 8282952C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829530: 48980459  bl 0x831a9988
	ctx.lr = 0x82829534;
	sub_831A9988(ctx, base);
	// 82829534: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82829538: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282953C: 386B6BDC  addi r3, r11, 0x6bdc
	ctx.r[3].s64 = ctx.r[11].s64 + 27612;
	// 82829540: 4897EBB9  bl 0x831a80f8
	ctx.lr = 0x82829544;
	sub_831A80F8(ctx, base);
	// 82829544: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82829548: 41820014  beq 0x8282955c
	if ctx.cr[0].eq {
	pc = 0x8282955C; continue 'dispatch;
	}
	// 8282954C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82829550: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82829554: 4BFFE005  bl 0x82827558
	ctx.lr = 0x82829558;
	sub_82827558(ctx, base);
	// 82829558: 4BFFFF8C  b 0x828294e4
	pc = 0x828294E4; continue 'dispatch;
	// 8282955C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82829560: 419A0034  beq cr6, 0x82829594
	if ctx.cr[6].eq {
	pc = 0x82829594; continue 'dispatch;
	}
	// 82829564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829568: 48980421  bl 0x831a9988
	ctx.lr = 0x8282956C;
	sub_831A9988(ctx, base);
	// 8282956C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82829570: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82829574: 386BDBD4  addi r3, r11, -0x242c
	ctx.r[3].s64 = ctx.r[11].s64 + -9260;
	// 82829578: 4897EB81  bl 0x831a80f8
	ctx.lr = 0x8282957C;
	sub_831A80F8(ctx, base);
	// 8282957C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82829580: 41820014  beq 0x82829594
	if ctx.cr[0].eq {
	pc = 0x82829594; continue 'dispatch;
	}
	// 82829584: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82829588: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8282958C: 4BFFDFF5  bl 0x82827580
	ctx.lr = 0x82829590;
	sub_82827580(ctx, base);
	// 82829590: 4BFFFF54  b 0x828294e4
	pc = 0x828294E4; continue 'dispatch;
	// 82829594: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82829598: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282959C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828295A0: 4BFF0291  bl 0x82819830
	ctx.lr = 0x828295A4;
	sub_82819830(ctx, base);
	// 828295A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 828295A8: 4897EC10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828295B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828295B0 size=4
    let mut pc: u32 = 0x828295B0;
    'dispatch: loop {
        match pc {
            0x828295B0 => {
    //   block [0x828295B0..0x828295B4)
	// 828295B0: 4BFFFEE8  b 0x82829498
	sub_82829498(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828295B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828295B8 size=12
    let mut pc: u32 = 0x828295B8;
    'dispatch: loop {
        match pc {
            0x828295B8 => {
    //   block [0x828295B8..0x828295C4)
	// 828295B8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828295BC: 388BB194  addi r4, r11, -0x4e6c
	ctx.r[4].s64 = ctx.r[11].s64 + -20076;
	// 828295C0: 4BFF0E28  b 0x8281a3e8
	sub_8281A3E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828295C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828295C8 size=4
    let mut pc: u32 = 0x828295C8;
    'dispatch: loop {
        match pc {
            0x828295C8 => {
    //   block [0x828295C8..0x828295CC)
	// 828295C8: 4BFF0E38  b 0x8281a400
	sub_8281A400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828295D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828295D0 size=220
    let mut pc: u32 = 0x828295D0;
    'dispatch: loop {
        match pc {
            0x828295D0 => {
    //   block [0x828295D0..0x828296AC)
	// 828295D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828295D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828295D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828295DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828295E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828295E4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828295E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828295EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828295F0: 388BB1C0  addi r4, r11, -0x4e40
	ctx.r[4].s64 = ctx.r[11].s64 + -20032;
	// 828295F4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 828295F8: 485CA411  bl 0x82df3a08
	ctx.lr = 0x828295FC;
	sub_82DF3A08(ctx, base);
	// 828295FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82829600: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829608: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282960C: C02B9450  lfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82829610: 4BFC2771  bl 0x827ebd80
	ctx.lr = 0x82829614;
	sub_827EBD80(ctx, base);
	// 82829614: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829618: 485C9E11  bl 0x82df3428
	ctx.lr = 0x8282961C;
	sub_82DF3428(ctx, base);
	// 8282961C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829620: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829624: 388BB1B8  addi r4, r11, -0x4e48
	ctx.r[4].s64 = ctx.r[11].s64 + -20040;
	// 82829628: 485CA3E1  bl 0x82df3a08
	ctx.lr = 0x8282962C;
	sub_82DF3A08(ctx, base);
	// 8282962C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829634: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82829638: 4BFC25C1  bl 0x827ebbf8
	ctx.lr = 0x8282963C;
	sub_827EBBF8(ctx, base);
	// 8282963C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829640: 485C9DE9  bl 0x82df3428
	ctx.lr = 0x82829644;
	sub_82DF3428(ctx, base);
	// 82829644: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282964C: 388BB1B0  addi r4, r11, -0x4e50
	ctx.r[4].s64 = ctx.r[11].s64 + -20048;
	// 82829650: 485CA3B9  bl 0x82df3a08
	ctx.lr = 0x82829654;
	sub_82DF3A08(ctx, base);
	// 82829654: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282965C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82829660: 4BFC24D1  bl 0x827ebb30
	ctx.lr = 0x82829664;
	sub_827EBB30(ctx, base);
	// 82829664: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829668: 485C9DC1  bl 0x82df3428
	ctx.lr = 0x8282966C;
	sub_82DF3428(ctx, base);
	// 8282966C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829674: 388BB1A8  addi r4, r11, -0x4e58
	ctx.r[4].s64 = ctx.r[11].s64 + -20056;
	// 82829678: 485CA391  bl 0x82df3a08
	ctx.lr = 0x8282967C;
	sub_82DF3A08(ctx, base);
	// 8282967C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82829680: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829688: 4BFC24A9  bl 0x827ebb30
	ctx.lr = 0x8282968C;
	sub_827EBB30(ctx, base);
	// 8282968C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829690: 485C9D99  bl 0x82df3428
	ctx.lr = 0x82829694;
	sub_82DF3428(ctx, base);
	// 82829694: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82829698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282969C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828296A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828296A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828296A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828296B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828296B0 size=8
    let mut pc: u32 = 0x828296B0;
    'dispatch: loop {
        match pc {
            0x828296B0 => {
    //   block [0x828296B0..0x828296B8)
	// 828296B0: 3863027C  addi r3, r3, 0x27c
	ctx.r[3].s64 = ctx.r[3].s64 + 636;
	// 828296B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828296B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828296B8 size=96
    let mut pc: u32 = 0x828296B8;
    'dispatch: loop {
        match pc {
            0x828296B8 => {
    //   block [0x828296B8..0x82829718)
	// 828296B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828296BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828296C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828296C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828296C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828296CC: 817F026C  lwz r11, 0x26c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 828296D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828296D4: 419A0018  beq cr6, 0x828296ec
	if ctx.cr[6].eq {
	pc = 0x828296EC; continue 'dispatch;
	}
	// 828296D8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 828296DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828296E0: 4E800421  bctrl
	ctx.lr = 0x828296E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828296E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828296E8: 917F026C  stw r11, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[11].u32 ) };
	// 828296EC: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 828296F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828296F4: 409A0010  bne cr6, 0x82829704
	if !ctx.cr[6].eq {
	pc = 0x82829704; continue 'dispatch;
	}
	// 828296F8: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 828296FC: 396B95C8  addi r11, r11, -0x6a38
	ctx.r[11].s64 = ctx.r[11].s64 + -27192;
	// 82829700: 917F0270  stw r11, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[11].u32 ) };
	// 82829704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82829708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829718 size=96
    let mut pc: u32 = 0x82829718;
    'dispatch: loop {
        match pc {
            0x82829718 => {
    //   block [0x82829718..0x82829778)
	// 82829718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282972C: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82829730: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829734: 419A0018  beq cr6, 0x8282974c
	if ctx.cr[6].eq {
	pc = 0x8282974C; continue 'dispatch;
	}
	// 82829738: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8282973C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82829740: 4E800421  bctrl
	ctx.lr = 0x82829744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82829744: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829748: 917F0270  stw r11, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[11].u32 ) };
	// 8282974C: 817F026C  lwz r11, 0x26c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 82829750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829754: 409A0010  bne cr6, 0x82829764
	if !ctx.cr[6].eq {
	pc = 0x82829764; continue 'dispatch;
	}
	// 82829758: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8282975C: 396B95B8  addi r11, r11, -0x6a48
	ctx.r[11].s64 = ctx.r[11].s64 + -27208;
	// 82829760: 917F026C  stw r11, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[11].u32 ) };
	// 82829764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82829768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282976C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82829778 size=120
    let mut pc: u32 = 0x82829778;
    'dispatch: loop {
        match pc {
            0x82829778 => {
    //   block [0x82829778..0x828297F0)
	// 82829778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282977C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82829784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282978C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82829790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82829794: 481E9EED  bl 0x82a13680
	ctx.lr = 0x82829798;
	sub_82A13680(ctx, base);
	// 82829798: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8282979C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828297A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828297A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828297A8: C02B6218  lfs f1, 0x6218(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828297AC: 4BFEF5BD  bl 0x82818d68
	ctx.lr = 0x828297B0;
	sub_82818D68(ctx, base);
	// 828297B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828297B4: 481E9ECD  bl 0x82a13680
	ctx.lr = 0x828297B8;
	sub_82A13680(ctx, base);
	// 828297B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 828297BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828297C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828297C4: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828297C8: 4BFEF869  bl 0x82819030
	ctx.lr = 0x828297CC;
	sub_82819030(ctx, base);
	// 828297CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828297D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828297D4: 4BFEFF55  bl 0x82819728
	ctx.lr = 0x828297D8;
	sub_82819728(ctx, base);
	// 828297D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828297DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828297E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828297E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828297E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828297EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828297F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828297F0 size=84
    let mut pc: u32 = 0x828297F0;
    'dispatch: loop {
        match pc {
            0x828297F0 => {
    //   block [0x828297F0..0x82829844)
	// 828297F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828297F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828297F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828297FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829800: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82829808: 388BB1C4  addi r4, r11, -0x4e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -20028;
	// 8282980C: 4BFC701D  bl 0x827f0828
	ctx.lr = 0x82829810;
	sub_827F0828(ctx, base);
	// 82829810: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829818: 388BB1E0  addi r4, r11, -0x4e20
	ctx.r[4].s64 = ctx.r[11].s64 + -20000;
	// 8282981C: 4BFCC8D5  bl 0x827f60f0
	ctx.lr = 0x82829820;
	sub_827F60F0(ctx, base);
	// 82829820: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829828: 388BB1A4  addi r4, r11, -0x4e5c
	ctx.r[4].s64 = ctx.r[11].s64 + -20060;
	// 8282982C: 4BFC729D  bl 0x827f0ac8
	ctx.lr = 0x82829830;
	sub_827F0AC8(ctx, base);
	// 82829830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82829834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282983C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829848 size=12
    let mut pc: u32 = 0x82829848;
    'dispatch: loop {
        match pc {
            0x82829848 => {
    //   block [0x82829848..0x82829854)
	// 82829848: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282984C: 386BAE54  addi r3, r11, -0x51ac
	ctx.r[3].s64 = ctx.r[11].s64 + -20908;
	// 82829850: 4BFF26E0  b 0x8281bf30
	sub_8281BF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829858 size=12
    let mut pc: u32 = 0x82829858;
    'dispatch: loop {
        match pc {
            0x82829858 => {
    //   block [0x82829858..0x82829864)
	// 82829858: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282985C: 386BAE54  addi r3, r11, -0x51ac
	ctx.r[3].s64 = ctx.r[11].s64 + -20908;
	// 82829860: 4BFF26D8  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829868 size=12
    let mut pc: u32 = 0x82829868;
    'dispatch: loop {
        match pc {
            0x82829868 => {
    //   block [0x82829868..0x82829874)
	// 82829868: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282986C: 386BB1E0  addi r3, r11, -0x4e20
	ctx.r[3].s64 = ctx.r[11].s64 + -20000;
	// 82829870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829878 size=12
    let mut pc: u32 = 0x82829878;
    'dispatch: loop {
        match pc {
            0x82829878 => {
    //   block [0x82829878..0x82829884)
	// 82829878: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282987C: 386BB1C4  addi r3, r11, -0x4e3c
	ctx.r[3].s64 = ctx.r[11].s64 + -20028;
	// 82829880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829888 size=76
    let mut pc: u32 = 0x82829888;
    'dispatch: loop {
        match pc {
            0x82829888 => {
    //   block [0x82829888..0x828298D4)
	// 82829888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82829894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282989C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828298A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828298A4: 388BB1C8  addi r4, r11, -0x4e38
	ctx.r[4].s64 = ctx.r[11].s64 + -20024;
	// 828298A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828298AC: 4BFEF3FD  bl 0x82818ca8
	ctx.lr = 0x828298B0;
	sub_82818CA8(ctx, base);
	// 828298B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828298B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828298B8: 4BFFFEC1  bl 0x82829778
	ctx.lr = 0x828298BC;
	sub_82829778(ctx, base);
	// 828298BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828298C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828298C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828298C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828298CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828298D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828298D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828298D8 size=84
    let mut pc: u32 = 0x828298D8;
    'dispatch: loop {
        match pc {
            0x828298D8 => {
    //   block [0x828298D8..0x8282992C)
	// 828298D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828298DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828298E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828298E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828298E8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828298EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828298F0: 388BB1F0  addi r4, r11, -0x4e10
	ctx.r[4].s64 = ctx.r[11].s64 + -19984;
	// 828298F4: 4BFC6F35  bl 0x827f0828
	ctx.lr = 0x828298F8;
	sub_827F0828(ctx, base);
	// 828298F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828298FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829900: 388BB20C  addi r4, r11, -0x4df4
	ctx.r[4].s64 = ctx.r[11].s64 + -19956;
	// 82829904: 4BFCC7ED  bl 0x827f60f0
	ctx.lr = 0x82829908;
	sub_827F60F0(ctx, base);
	// 82829908: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282990C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829910: 388BB1A4  addi r4, r11, -0x4e5c
	ctx.r[4].s64 = ctx.r[11].s64 + -20060;
	// 82829914: 4BFC71B5  bl 0x827f0ac8
	ctx.lr = 0x82829918;
	sub_827F0AC8(ctx, base);
	// 82829918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282991C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829930 size=12
    let mut pc: u32 = 0x82829930;
    'dispatch: loop {
        match pc {
            0x82829930 => {
    //   block [0x82829930..0x8282993C)
	// 82829930: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829934: 386BAE5C  addi r3, r11, -0x51a4
	ctx.r[3].s64 = ctx.r[11].s64 + -20900;
	// 82829938: 4BFF25F8  b 0x8281bf30
	sub_8281BF30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829940 size=12
    let mut pc: u32 = 0x82829940;
    'dispatch: loop {
        match pc {
            0x82829940 => {
    //   block [0x82829940..0x8282994C)
	// 82829940: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829944: 386BAE5C  addi r3, r11, -0x51a4
	ctx.r[3].s64 = ctx.r[11].s64 + -20900;
	// 82829948: 4BFF25F0  b 0x8281bf38
	sub_8281BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829950 size=12
    let mut pc: u32 = 0x82829950;
    'dispatch: loop {
        match pc {
            0x82829950 => {
    //   block [0x82829950..0x8282995C)
	// 82829950: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829954: 386BB20C  addi r3, r11, -0x4df4
	ctx.r[3].s64 = ctx.r[11].s64 + -19956;
	// 82829958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829960 size=12
    let mut pc: u32 = 0x82829960;
    'dispatch: loop {
        match pc {
            0x82829960 => {
    //   block [0x82829960..0x8282996C)
	// 82829960: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829964: 386BB1F0  addi r3, r11, -0x4e10
	ctx.r[3].s64 = ctx.r[11].s64 + -19984;
	// 82829968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829970 size=76
    let mut pc: u32 = 0x82829970;
    'dispatch: loop {
        match pc {
            0x82829970 => {
    //   block [0x82829970..0x828299BC)
	// 82829970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282997C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829984: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829988: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282998C: 388BB1E0  addi r4, r11, -0x4e20
	ctx.r[4].s64 = ctx.r[11].s64 + -20000;
	// 82829990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82829994: 4BFEF315  bl 0x82818ca8
	ctx.lr = 0x82829998;
	sub_82818CA8(ctx, base);
	// 82829998: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282999C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828299A0: 4BFFFDD9  bl 0x82829778
	ctx.lr = 0x828299A4;
	sub_82829778(ctx, base);
	// 828299A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828299A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828299AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828299B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828299B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828299B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828299C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828299C0 size=136
    let mut pc: u32 = 0x828299C0;
    'dispatch: loop {
        match pc {
            0x828299C0 => {
    //   block [0x828299C0..0x82829A48)
	// 828299C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828299C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828299C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828299CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828299D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828299D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828299D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828299DC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 828299E0: 409A0020  bne cr6, 0x82829a00
	if !ctx.cr[6].eq {
	pc = 0x82829A00; continue 'dispatch;
	}
	// 828299E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828299E8: 419A0048  beq cr6, 0x82829a30
	if ctx.cr[6].eq {
	pc = 0x82829A30; continue 'dispatch;
	}
	// 828299EC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 828299F0: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 828299F4: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 828299F8: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 828299FC: 48000034  b 0x82829a30
	pc = 0x82829A30; continue 'dispatch;
	// 82829A00: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82829A04: 419A002C  beq cr6, 0x82829a30
	if ctx.cr[6].eq {
	pc = 0x82829A30; continue 'dispatch;
	}
	// 82829A08: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82829A0C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829A10: 388B0900  addi r4, r11, 0x900
	ctx.r[4].s64 = ctx.r[11].s64 + 2304;
	// 82829A14: 4897E6E5  bl 0x831a80f8
	ctx.lr = 0x82829A18;
	sub_831A80F8(ctx, base);
	// 82829A18: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82829A1C: 4182000C  beq 0x82829a28
	if ctx.cr[0].eq {
	pc = 0x82829A28; continue 'dispatch;
	}
	// 82829A20: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82829A24: 4800000C  b 0x82829a30
	pc = 0x82829A30; continue 'dispatch;
	// 82829A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829A2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829A30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82829A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829A3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82829A40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82829A48 size=344
    let mut pc: u32 = 0x82829A48;
    'dispatch: loop {
        match pc {
            0x82829A48 => {
    //   block [0x82829A48..0x82829BA0)
	// 82829A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829A4C: 4897E721  bl 0x831a816c
	ctx.lr = 0x82829A50;
	sub_831A8130(ctx, base);
	// 82829A50: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82829A54: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82829A58: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82829A5C: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829A60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82829A64: 4BCE80F5  bl 0x82511b58
	ctx.lr = 0x82829A68;
	sub_82511B58(ctx, base);
	// 82829A68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82829A6C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82829A70: 48652429  bl 0x82e7be98
	ctx.lr = 0x82829A74;
	sub_82E7BE98(ctx, base);
	// 82829A74: 3BE000FF  li r31, 0xff
	ctx.r[31].s64 = 255;
	// 82829A78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82829A7C: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82829A80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829A84: 9BA10051  stb r29, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[29].u8 ) };
	// 82829A88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82829A8C: 9BA10052  stb r29, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[29].u8 ) };
	// 82829A90: 9BE10053  stb r31, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[31].u8 ) };
	// 82829A94: 4BCE5A35  bl 0x8250f4c8
	ctx.lr = 0x82829A98;
	sub_8250F4C8(ctx, base);
	// 82829A98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829A9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829AA0: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82829AA4: 409A0008  bne cr6, 0x82829aac
	if !ctx.cr[6].eq {
	pc = 0x82829AAC; continue 'dispatch;
	}
	// 82829AA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82829AAC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82829AB0: 4BCDEF01  bl 0x825089b0
	ctx.lr = 0x82829AB4;
	sub_825089B0(ctx, base);
	// 82829AB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829ABC: 386BFF34  addi r3, r11, -0xcc
	ctx.r[3].s64 = ctx.r[11].s64 + -204;
	// 82829AC0: 409A0008  bne cr6, 0x82829ac8
	if !ctx.cr[6].eq {
	pc = 0x82829AC8; continue 'dispatch;
	}
	// 82829AC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82829AC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82829ACC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82829AD0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82829AD4: 38EABA80  addi r7, r10, -0x4580
	ctx.r[7].s64 = ctx.r[10].s64 + -17792;
	// 82829AD8: 389E00F0  addi r4, r30, 0xf0
	ctx.r[4].s64 = ctx.r[30].s64 + 240;
	// 82829ADC: C02B9450  lfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82829AE0: 4BA9DF89  bl 0x822c7a68
	ctx.lr = 0x82829AE4;
	sub_822C7A68(ctx, base);
	// 82829AE4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82829AE8: 485C81A9  bl 0x82df1c90
	ctx.lr = 0x82829AEC;
	sub_82DF1C90(ctx, base);
	// 82829AEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82829AF0: 485C81A1  bl 0x82df1c90
	ctx.lr = 0x82829AF4;
	sub_82DF1C90(ctx, base);
	// 82829AF4: 9BE10054  stb r31, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u8 ) };
	// 82829AF8: 9BE10055  stb r31, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[31].u8 ) };
	// 82829AFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82829B00: 9BE10056  stb r31, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[31].u8 ) };
	// 82829B04: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82829B08: 9BE10057  stb r31, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[31].u8 ) };
	// 82829B0C: 4BCE59BD  bl 0x8250f4c8
	ctx.lr = 0x82829B10;
	sub_8250F4C8(ctx, base);
	// 82829B10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829B14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829B18: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82829B1C: 409A0008  bne cr6, 0x82829b24
	if !ctx.cr[6].eq {
	pc = 0x82829B24; continue 'dispatch;
	}
	// 82829B20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82829B24: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82829B28: 4BCDEE89  bl 0x825089b0
	ctx.lr = 0x82829B2C;
	sub_825089B0(ctx, base);
	// 82829B2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829B34: 3BEBFF34  addi r31, r11, -0xcc
	ctx.r[31].s64 = ctx.r[11].s64 + -204;
	// 82829B38: 409A0008  bne cr6, 0x82829b40
	if !ctx.cr[6].eq {
	pc = 0x82829B40; continue 'dispatch;
	}
	// 82829B3C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82829B40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829B44: C3FE0108  lfs f31, 0x108(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82829B48: C3DE0104  lfs f30, 0x104(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(260 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82829B4C: 3BA10054  addi r29, r1, 0x54
	ctx.r[29].s64 = ctx.r[1].s64 + 84;
	// 82829B50: C3BE0100  lfs f29, 0x100(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82829B54: 3BC10080  addi r30, r1, 0x80
	ctx.r[30].s64 = ctx.r[1].s64 + 128;
	// 82829B58: 4BCE7F91  bl 0x82511ae8
	ctx.lr = 0x82829B5C;
	sub_82511AE8(ctx, base);
	// 82829B5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82829B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829B64: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82829B68: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82829B6C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82829B70: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82829B74: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82829B78: 4BA9DFD1  bl 0x822c7b48
	ctx.lr = 0x82829B7C;
	sub_822C7B48(ctx, base);
	// 82829B7C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82829B80: 485C8111  bl 0x82df1c90
	ctx.lr = 0x82829B84;
	sub_82DF1C90(ctx, base);
	// 82829B84: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82829B88: 485C8109  bl 0x82df1c90
	ctx.lr = 0x82829B8C;
	sub_82DF1C90(ctx, base);
	// 82829B8C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82829B90: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82829B94: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82829B98: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82829B9C: 4897E620  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829BA0 size=196
    let mut pc: u32 = 0x82829BA0;
    'dispatch: loop {
        match pc {
            0x82829BA0 => {
    //   block [0x82829BA0..0x82829C64)
	// 82829BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82829BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829BB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82829BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829BBC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82829BC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82829BC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829BC8: 4BA96D71  bl 0x822c0938
	ctx.lr = 0x82829BCC;
	sub_822C0938(ctx, base);
	// 82829BCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82829BD0: 41820028  beq 0x82829bf8
	if ctx.cr[0].eq {
	pc = 0x82829BF8; continue 'dispatch;
	}
	// 82829BD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829BD8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82829BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82829BE0: 392BB200  addi r9, r11, -0x4e00
	ctx.r[9].s64 = ctx.r[11].s64 + -19968;
	// 82829BE4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82829BE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82829BEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82829BF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82829BF4: 48000008  b 0x82829bfc
	pc = 0x82829BFC; continue 'dispatch;
	// 82829BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829BFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829C04: 409A0044  bne cr6, 0x82829c48
	if !ctx.cr[6].eq {
	pc = 0x82829C48; continue 'dispatch;
	}
	// 82829C08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82829C0C: 419A001C  beq cr6, 0x82829c28
	if ctx.cr[6].eq {
	pc = 0x82829C28; continue 'dispatch;
	}
	// 82829C10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829C14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82829C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829C1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82829C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82829C24: 4E800421  bctrl
	ctx.lr = 0x82829C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82829C28: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82829C2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82829C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829C34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82829C38: 816B0780  lwz r11, 0x780(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1920 as u32) ) } as u64;
	// 82829C3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82829C40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82829C44: 4BA963BD  bl 0x822c0000
	ctx.lr = 0x82829C48;
	sub_822C0000(ctx, base);
	// 82829C48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82829C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82829C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829C68 size=196
    let mut pc: u32 = 0x82829C68;
    'dispatch: loop {
        match pc {
            0x82829C68 => {
    //   block [0x82829C68..0x82829D2C)
	// 82829C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829C70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82829C74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829C78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829C7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82829C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829C84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82829C88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82829C8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829C90: 4BA96CA9  bl 0x822c0938
	ctx.lr = 0x82829C94;
	sub_822C0938(ctx, base);
	// 82829C94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82829C98: 41820028  beq 0x82829cc0
	if ctx.cr[0].eq {
	pc = 0x82829CC0; continue 'dispatch;
	}
	// 82829C9C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829CA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82829CA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82829CA8: 392BB214  addi r9, r11, -0x4dec
	ctx.r[9].s64 = ctx.r[11].s64 + -19948;
	// 82829CAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82829CB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82829CB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82829CB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82829CBC: 48000008  b 0x82829cc4
	pc = 0x82829CC4; continue 'dispatch;
	// 82829CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829CC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829CCC: 409A0044  bne cr6, 0x82829d10
	if !ctx.cr[6].eq {
	pc = 0x82829D10; continue 'dispatch;
	}
	// 82829CD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82829CD4: 419A001C  beq cr6, 0x82829cf0
	if ctx.cr[6].eq {
	pc = 0x82829CF0; continue 'dispatch;
	}
	// 82829CD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829CDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82829CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829CE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829CE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82829CEC: 4E800421  bctrl
	ctx.lr = 0x82829CF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82829CF0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82829CF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82829CF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829CFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82829D00: 816B0780  lwz r11, 0x780(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1920 as u32) ) } as u64;
	// 82829D04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82829D08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82829D0C: 4BA962F5  bl 0x822c0000
	ctx.lr = 0x82829D10;
	sub_822C0000(ctx, base);
	// 82829D10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829D14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82829D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829D20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82829D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829D30 size=196
    let mut pc: u32 = 0x82829D30;
    'dispatch: loop {
        match pc {
            0x82829D30 => {
    //   block [0x82829D30..0x82829DF4)
	// 82829D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82829D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829D44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82829D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829D4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82829D50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82829D54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829D58: 4BA96BE1  bl 0x822c0938
	ctx.lr = 0x82829D5C;
	sub_822C0938(ctx, base);
	// 82829D5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82829D60: 41820028  beq 0x82829d88
	if ctx.cr[0].eq {
	pc = 0x82829D88; continue 'dispatch;
	}
	// 82829D64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829D68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82829D6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82829D70: 392BB228  addi r9, r11, -0x4dd8
	ctx.r[9].s64 = ctx.r[11].s64 + -19928;
	// 82829D74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82829D78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82829D7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82829D80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82829D84: 48000008  b 0x82829d8c
	pc = 0x82829D8C; continue 'dispatch;
	// 82829D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829D8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829D90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829D94: 409A0044  bne cr6, 0x82829dd8
	if !ctx.cr[6].eq {
	pc = 0x82829DD8; continue 'dispatch;
	}
	// 82829D98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82829D9C: 419A001C  beq cr6, 0x82829db8
	if ctx.cr[6].eq {
	pc = 0x82829DB8; continue 'dispatch;
	}
	// 82829DA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829DA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82829DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829DAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82829DB4: 4E800421  bctrl
	ctx.lr = 0x82829DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82829DB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82829DBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82829DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829DC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82829DC8: 816B0780  lwz r11, 0x780(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1920 as u32) ) } as u64;
	// 82829DCC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82829DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82829DD4: 4BA9622D  bl 0x822c0000
	ctx.lr = 0x82829DD8;
	sub_822C0000(ctx, base);
	// 82829DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82829DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829DE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82829DEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829DF8 size=196
    let mut pc: u32 = 0x82829DF8;
    'dispatch: loop {
        match pc {
            0x82829DF8 => {
    //   block [0x82829DF8..0x82829EBC)
	// 82829DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829E00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82829E04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829E0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82829E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829E14: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82829E18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82829E1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829E20: 4BA96B19  bl 0x822c0938
	ctx.lr = 0x82829E24;
	sub_822C0938(ctx, base);
	// 82829E24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82829E28: 41820028  beq 0x82829e50
	if ctx.cr[0].eq {
	pc = 0x82829E50; continue 'dispatch;
	}
	// 82829E2C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829E30: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82829E34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82829E38: 392BB23C  addi r9, r11, -0x4dc4
	ctx.r[9].s64 = ctx.r[11].s64 + -19908;
	// 82829E3C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82829E40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82829E44: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82829E48: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82829E4C: 48000008  b 0x82829e54
	pc = 0x82829E54; continue 'dispatch;
	// 82829E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82829E54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82829E5C: 409A0044  bne cr6, 0x82829ea0
	if !ctx.cr[6].eq {
	pc = 0x82829EA0; continue 'dispatch;
	}
	// 82829E60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82829E64: 419A001C  beq cr6, 0x82829e80
	if ctx.cr[6].eq {
	pc = 0x82829E80; continue 'dispatch;
	}
	// 82829E68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829E6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82829E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829E74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829E78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82829E7C: 4E800421  bctrl
	ctx.lr = 0x82829E80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82829E80: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82829E84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82829E88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82829E8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82829E90: 816B0780  lwz r11, 0x780(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1920 as u32) ) } as u64;
	// 82829E94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82829E98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82829E9C: 4BA96165  bl 0x822c0000
	ctx.lr = 0x82829EA0;
	sub_822C0000(ctx, base);
	// 82829EA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829EA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82829EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829EB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82829EB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82829EC0 size=132
    let mut pc: u32 = 0x82829EC0;
    'dispatch: loop {
        match pc {
            0x82829EC0 => {
    //   block [0x82829EC0..0x82829F44)
	// 82829EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82829EC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82829ECC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829ED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82829ED4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82829ED8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82829EDC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 82829EE0: 396BB2B4  addi r11, r11, -0x4d4c
	ctx.r[11].s64 = ctx.r[11].s64 + -19788;
	// 82829EE4: 394AB2A0  addi r10, r10, -0x4d60
	ctx.r[10].s64 = ctx.r[10].s64 + -19808;
	// 82829EE8: 3929B254  addi r9, r9, -0x4dac
	ctx.r[9].s64 = ctx.r[9].s64 + -19884;
	// 82829EEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82829EF0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82829EF4: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82829EF8: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82829EFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82829F00: 419A0008  beq cr6, 0x82829f08
	if ctx.cr[6].eq {
	pc = 0x82829F08; continue 'dispatch;
	}
	// 82829F04: 4BA9698D  bl 0x822c0890
	ctx.lr = 0x82829F08;
	sub_822C0890(ctx, base);
	// 82829F08: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 82829F0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82829F10: 419A0008  beq cr6, 0x82829f18
	if ctx.cr[6].eq {
	pc = 0x82829F18; continue 'dispatch;
	}
	// 82829F14: 4BA9697D  bl 0x822c0890
	ctx.lr = 0x82829F18;
	sub_822C0890(ctx, base);
	// 82829F18: 807F0258  lwz r3, 0x258(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(600 as u32) ) } as u64;
	// 82829F1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82829F20: 419A0008  beq cr6, 0x82829f28
	if ctx.cr[6].eq {
	pc = 0x82829F28; continue 'dispatch;
	}
	// 82829F24: 4BA9696D  bl 0x822c0890
	ctx.lr = 0x82829F28;
	sub_822C0890(ctx, base);
	// 82829F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82829F2C: 4BFEDF4D  bl 0x82817e78
	ctx.lr = 0x82829F30;
	sub_82817E78(ctx, base);
	// 82829F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82829F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82829F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82829F3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82829F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829F48 size=8
    let mut pc: u32 = 0x82829F48;
    'dispatch: loop {
        match pc {
            0x82829F48 => {
    //   block [0x82829F48..0x82829F50)
	// 82829F48: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82829F4C: 480003B4  b 0x8282a300
	sub_8282A300(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82829F50 size=8
    let mut pc: u32 = 0x82829F50;
    'dispatch: loop {
        match pc {
            0x82829F50 => {
    //   block [0x82829F50..0x82829F58)
	// 82829F50: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82829F54: 480003AC  b 0x8282a300
	sub_8282A300(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82829F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82829F58 size=300
    let mut pc: u32 = 0x82829F58;
    'dispatch: loop {
        match pc {
            0x82829F58 => {
    //   block [0x82829F58..0x8282A084)
	// 82829F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82829F5C: 4897E20D  bl 0x831a8168
	ctx.lr = 0x82829F60;
	sub_831A8130(ctx, base);
	// 82829F60: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82829F64: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82829F68: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82829F6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82829F70: 3BEBB180  addi r31, r11, -0x4e80
	ctx.r[31].s64 = ctx.r[11].s64 + -20096;
	// 82829F74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82829F78: 816AB190  lwz r11, -0x4e70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20080 as u32) ) } as u64;
	// 82829F7C: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82829F80: 4082002C  bne 0x82829fac
	if !ctx.cr[0].eq {
	pc = 0x82829FAC; continue 'dispatch;
	}
	// 82829F84: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82829F88: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82829F8C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82829F90: 916AB190  stw r11, -0x4e70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20080 as u32), ctx.r[11].u32 ) };
	// 82829F94: C00908A8  lfs f0, 0x8a8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82829F98: C1A808A4  lfs f13, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82829F9C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82829FA0: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82829FA4: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82829FA8: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82829FAC: 807E025C  lwz r3, 0x25c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(604 as u32) ) } as u64;
	// 82829FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82829FB4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82829FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82829FBC: 4E800421  bctrl
	ctx.lr = 0x82829FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82829FC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82829FC4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82829FC8: 48652E41  bl 0x82e7ce08
	ctx.lr = 0x82829FCC;
	sub_82E7CE08(ctx, base);
	// 82829FCC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82829FD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82829FD4: 481E96B5  bl 0x82a13688
	ctx.lr = 0x82829FD8;
	sub_82A13688(ctx, base);
	// 82829FD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82829FDC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82829FE0: 48652D01  bl 0x82e7cce0
	ctx.lr = 0x82829FE4;
	sub_82E7CCE0(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282A088 size=76
    let mut pc: u32 = 0x8282A088;
    'dispatch: loop {
        match pc {
            0x8282A088 => {
    //   block [0x8282A088..0x8282A0D4)
	// 8282A088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282A08C: 4897E0E1  bl 0x831a816c
	ctx.lr = 0x8282A090;
	sub_831A8130(ctx, base);
	// 8282A090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282A094: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8282A098: 3BFEFDF8  addi r31, r30, -0x208
	ctx.r[31].s64 = ctx.r[30].s64 + -520;
	// 8282A09C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A0A0: 83BE0054  lwz r29, 0x54(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282A0A4: 481E95DD  bl 0x82a13680
	ctx.lr = 0x8282A0A8;
	sub_82A13680(ctx, base);
	// 8282A0A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282A0AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282A0B0: 4BD60DB9  bl 0x8258ae68
	ctx.lr = 0x8282A0B4;
	sub_8258AE68(ctx, base);
	// 8282A0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A0B8: 83FE0054  lwz r31, 0x54(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282A0BC: 481E95CD  bl 0x82a13688
	ctx.lr = 0x8282A0C0;
	sub_82A13688(ctx, base);
	// 8282A0C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282A0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A0C8: 4BD60DE9  bl 0x8258aeb0
	ctx.lr = 0x8282A0CC;
	sub_8258AEB0(ctx, base);
	// 8282A0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282A0D0: 4897E0EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282A0D8 size=208
    let mut pc: u32 = 0x8282A0D8;
    'dispatch: loop {
        match pc {
            0x8282A0D8 => {
    //   block [0x8282A0D8..0x8282A1A8)
	// 8282A0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282A0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282A0E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282A0E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282A0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282A0EC: 83C30258  lwz r30, 0x258(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(600 as u32) ) } as u64;
	// 8282A0F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282A0F4: 81630254  lwz r11, 0x254(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(596 as u32) ) } as u64;
	// 8282A0F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8282A0FC: 419A0024  beq cr6, 0x8282a120
	if ctx.cr[6].eq {
	pc = 0x8282A120; continue 'dispatch;
	}
	// 8282A100: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 8282A104: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8282A108: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282A10C: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8282A110: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8282A114: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282A118: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282A11C: 4082FFE8  bne 0x8282a104
	if !ctx.cr[0].eq {
	pc = 0x8282A104; continue 'dispatch;
	}
	// 8282A120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282A124: 419A004C  beq cr6, 0x8282a170
	if ctx.cr[6].eq {
	pc = 0x8282A170; continue 'dispatch;
	}
	// 8282A128: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282A12C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8282A130: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8282A134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282A138: 4E800421  bctrl
	ctx.lr = 0x8282A13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282A13C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8282A140: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8282A144: C0030030  lfs f0, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282A148: C1A30034  lfs f13, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8282A14C: C1830038  lfs f12, 0x38(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8282A150: C163003C  lfs f11, 0x3c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8282A154: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8282A158: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8282A15C: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8282A160: D161005C  stfs f11, 0x5c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8282A164: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282A1A8 size=8
    let mut pc: u32 = 0x8282A1A8;
    'dispatch: loop {
        match pc {
            0x8282A1A8 => {
    //   block [0x8282A1A8..0x8282A1B0)
	// 8282A1A8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8282A1AC: 48000AAC  b 0x8282ac58
	sub_8282AC58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282A1B0 size=8
    let mut pc: u32 = 0x8282A1B0;
    'dispatch: loop {
        match pc {
            0x8282A1B0 => {
    //   block [0x8282A1B0..0x8282A1B8)
	// 8282A1B0: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 8282A1B4: 48000AA4  b 0x8282ac58
	sub_8282AC58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282A1B8 size=8
    let mut pc: u32 = 0x8282A1B8;
    'dispatch: loop {
        match pc {
            0x8282A1B8 => {
    //   block [0x8282A1B8..0x8282A1C0)
	// 8282A1B8: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 8282A1BC: 48000B6C  b 0x8282ad28
	sub_8282AD28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282A1C0 size=8
    let mut pc: u32 = 0x8282A1C0;
    'dispatch: loop {
        match pc {
            0x8282A1C0 => {
    //   block [0x8282A1C0..0x8282A1C8)
	// 8282A1C0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8282A1C4: 48000B64  b 0x8282ad28
	sub_8282AD28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282A1C8 size=72
    let mut pc: u32 = 0x8282A1C8;
    'dispatch: loop {
        match pc {
            0x8282A1C8 => {
    //   block [0x8282A1C8..0x8282A210)
	// 8282A1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282A1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282A1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282A1D4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 8282A1D8: 419A001C  beq cr6, 0x8282a1f4
	if ctx.cr[6].eq {
	pc = 0x8282A1F4; continue 'dispatch;
	}
	// 8282A1DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8282A1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8282A1E4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8282A1E8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8282A1EC: 4BFFF7D5  bl 0x828299c0
	ctx.lr = 0x8282A1F0;
	sub_828299C0(ctx, base);
	// 8282A1F0: 48000010  b 0x8282a200
	pc = 0x8282A200; continue 'dispatch;
	// 8282A1F4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282A1F8: 396B0900  addi r11, r11, 0x900
	ctx.r[11].s64 = ctx.r[11].s64 + 2304;
	// 8282A1FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282A200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282A204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282A208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282A20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282A210 size=236
    let mut pc: u32 = 0x8282A210;
    'dispatch: loop {
        match pc {
            0x8282A210 => {
    //   block [0x8282A210..0x8282A2FC)
	// 8282A210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282A214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282A218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282A21C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282A220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282A224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282A228: 4BFEDDC1  bl 0x82817fe8
	ctx.lr = 0x8282A22C;
	sub_82817FE8(ctx, base);
	// 8282A22C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282A230: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282A234: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282A238: 396BB2B4  addi r11, r11, -0x4d4c
	ctx.r[11].s64 = ctx.r[11].s64 + -19788;
	// 8282A23C: 394AB2A0  addi r10, r10, -0x4d60
	ctx.r[10].s64 = ctx.r[10].s64 + -19808;
	// 8282A240: 3929B254  addi r9, r9, -0x4dac
	ctx.r[9].s64 = ctx.r[9].s64 + -19884;
	// 8282A244: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282A248: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8282A24C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8282A250: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 8282A254: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282A258: 93DF0254  stw r30, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[30].u32 ) };
	// 8282A25C: 387F027C  addi r3, r31, 0x27c
	ctx.r[3].s64 = ctx.r[31].s64 + 636;
	// 8282A260: 93DF0258  stw r30, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[30].u32 ) };
	// 8282A264: 388BB1A4  addi r4, r11, -0x4e5c
	ctx.r[4].s64 = ctx.r[11].s64 + -20060;
	// 8282A268: 93DF025C  stw r30, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[30].u32 ) };
	// 8282A26C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8282A270: 93DF0260  stw r30, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[30].u32 ) };
	// 8282A274: 93DF0264  stw r30, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[30].u32 ) };
	// 8282A278: 93DF0268  stw r30, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u32 ) };
	// 8282A27C: 93DF026C  stw r30, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[30].u32 ) };
	// 8282A280: 93DF0270  stw r30, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[30].u32 ) };
	// 8282A284: 93DF0274  stw r30, 0x274(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), ctx.r[30].u32 ) };
	// 8282A288: 93DF0278  stw r30, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[30].u32 ) };
	// 8282A28C: 4897E285  bl 0x831a8510
	ctx.lr = 0x8282A290;
	sub_831A8510(ctx, base);
	// 8282A290: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8282A294: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8282A298: 93DF02B0  stw r30, 0x2b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(688 as u32), ctx.r[30].u32 ) };
	// 8282A29C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282A2A0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8282A2A4: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 8282A2A8: C00A08AC  lfs f0, 0x8ac(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282A2AC: 38E002C0  li r7, 0x2c0
	ctx.r[7].s64 = 704;
	// 8282A2B0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8282A2B4: D01F029C  stfs f0, 0x29c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 8282A2B8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8282A2BC: C1A9093C  lfs f13, 0x93c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2364 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8282A2C0: C1889450  lfs f12, -0x6bb0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8282A2C4: 90DF02A8  stw r6, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[6].u32 ) };
	// 8282A2C8: D1BF02A0  stfs f13, 0x2a0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(672 as u32), tmp.u32 ) };
	// 8282A2CC: 915F02AC  stw r10, 0x2ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[10].u32 ) };
	// 8282A2D0: D19F02A4  stfs f12, 0x2a4(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), tmp.u32 ) };
	// 8282A2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A2D8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282A300 size=76
    let mut pc: u32 = 0x8282A300;
    'dispatch: loop {
        match pc {
            0x8282A300 => {
    //   block [0x8282A300..0x8282A34C)
	// 8282A300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282A304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282A308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282A30C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282A310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282A314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282A318: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282A31C: 4BFFFBA5  bl 0x82829ec0
	ctx.lr = 0x8282A320;
	sub_82829EC0(ctx, base);
	// 8282A320: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282A324: 4182000C  beq 0x8282a330
	if ctx.cr[0].eq {
	pc = 0x8282A330; continue 'dispatch;
	}
	// 8282A328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A32C: 485C80AD  bl 0x82df23d8
	ctx.lr = 0x8282A330;
	sub_82DF23D8(ctx, base);
	// 8282A330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282A338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282A33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282A340: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282A344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282A348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282A350 size=1644
    let mut pc: u32 = 0x8282A350;
    'dispatch: loop {
        match pc {
            0x8282A350 => {
    //   block [0x8282A350..0x8282A9BC)
	// 8282A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282A354: 4897DE0D  bl 0x831a8160
	ctx.lr = 0x8282A358;
	sub_831A8130(ctx, base);
	// 8282A358: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 8282A35C: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 8282A360: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 8282A364: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282A368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282A36C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8282A370: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282A374: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8282A378: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282A37C: 4E800421  bctrl
	ctx.lr = 0x8282A380;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282A380: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282A384: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 8282A388: 4BFC4409  bl 0x827ee790
	ctx.lr = 0x8282A38C;
	sub_827EE790(ctx, base);
	// 8282A38C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282A390: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282A394: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8282A398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282A39C: 4E800421  bctrl
	ctx.lr = 0x8282A3A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282A3A0: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 8282A3A4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8282A3A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282A3AC: 419A0008  beq cr6, 0x8282a3b4
	if ctx.cr[6].eq {
	pc = 0x8282A3B4; continue 'dispatch;
	}
	// 8282A3B0: 4BA964E1  bl 0x822c0890
	ctx.lr = 0x8282A3B4;
	sub_822C0890(ctx, base);
	// 8282A3B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282A3B8: C80BD9A0  lfd f0, -0x2660(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-9824 as u32) ) };
	// 8282A3BC: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 8282A3C0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8282A3C4: 40980028  bge cr6, 0x8282a3ec
	if !ctx.cr[6].lt {
	pc = 0x8282A3EC; continue 'dispatch;
	}
	// 8282A3C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282A3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A3D0: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8282A3D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282A3D8: 4E800421  bctrl
	ctx.lr = 0x8282A3DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282A3DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282A3E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A3E4: 4BFFF335  bl 0x82829718
	ctx.lr = 0x8282A3E8;
	sub_82829718(ctx, base);
	// 8282A3E8: 48000038  b 0x8282a420
	pc = 0x8282A420; continue 'dispatch;
	// 8282A3EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8282A3F0: C80BA850  lfd f0, -0x57b0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22448 as u32) ) };
	// 8282A3F4: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 8282A3F8: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8282A3FC: 40990024  ble cr6, 0x8282a420
	if !ctx.cr[6].gt {
	pc = 0x8282A420; continue 'dispatch;
	}
	// 8282A400: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282A404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A408: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8282A40C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282A410: 4E800421  bctrl
	ctx.lr = 0x8282A414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282A414: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282A418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282A41C: 4BFFF29D  bl 0x828296b8
	ctx.lr = 0x8282A420;
	sub_828296B8(ctx, base);
	// 8282A420: 807F025C  lwz r3, 0x25c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(604 as u32) ) } as u64;
	// 8282A424: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282A428: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8282A42C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282A430: 4E800421  bctrl
	ctx.lr = 0x8282A434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282A434: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 8282A438: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8282A43C: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282A440: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8282A444: 39010140  addi r8, r1, 0x140
	ctx.r[8].s64 = ctx.r[1].s64 + 320;
	// 8282A448: 38E10150  addi r7, r1, 0x150
	ctx.r[7].s64 = ctx.r[1].s64 + 336;
	// 8282A44C: 13CB1C07  vcmpneb. (lvlx128) v30, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282A450: 38C10160  addi r6, r1, 0x160
	ctx.r[6].s64 = ctx.r[1].s64 + 352;
	// 8282A454: 39610170  addi r11, r1, 0x170
	ctx.r[11].s64 = ctx.r[1].s64 + 368;
	// 8282A458: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282A45C: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282A460: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282A9C0 size=340
    let mut pc: u32 = 0x8282A9C0;
    'dispatch: loop {
        match pc {
            0x8282A9C0 => {
    //   block [0x8282A9C0..0x8282AB14)
	// 8282A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282A9C4: 4897D7A9  bl 0x831a816c
	ctx.lr = 0x8282A9C8;
	sub_831A8130(ctx, base);
	// 8282A9C8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8282A9CC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282A9D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282A9D4: 817F02A8  lwz r11, 0x2a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(680 as u32) ) } as u64;
	// 8282A9D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282A9DC: 409A000C  bne cr6, 0x8282a9e8
	if !ctx.cr[6].eq {
	pc = 0x8282A9E8; continue 'dispatch;
	}
	// 8282A9E0: 83DF02AC  lwz r30, 0x2ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8282A9E4: 4800002C  b 0x8282aa10
	pc = 0x8282AA10; continue 'dispatch;
	// 8282A9E8: 817F02B0  lwz r11, 0x2b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(688 as u32) ) } as u64;
	// 8282A9EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282A9F0: 40990118  ble cr6, 0x8282ab08
	if !ctx.cr[6].gt {
	pc = 0x8282AB08; continue 'dispatch;
	}
	// 8282A9F4: 815F02AC  lwz r10, 0x2ac(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8282A9F8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8282A9FC: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 8282AA00: 41980008  blt cr6, 0x8282aa08
	if ctx.cr[6].lt {
	pc = 0x8282AA08; continue 'dispatch;
	}
	// 8282AA04: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8282AA08: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8282AA0C: 917F02B0  stw r11, 0x2b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(688 as u32), ctx.r[11].u32 ) };
	// 8282AA10: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8282AA14: 409900F4  ble cr6, 0x8282ab08
	if !ctx.cr[6].gt {
	pc = 0x8282AB08; continue 'dispatch;
	}
	// 8282AA18: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282AA1C: C3FF02A4  lfs f31, 0x2a4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(676 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282AA20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282AA24: 388BB510  addi r4, r11, -0x4af0
	ctx.r[4].s64 = ctx.r[11].s64 + -19184;
	// 8282AA28: 485C8FE1  bl 0x82df3a08
	ctx.lr = 0x8282AA2C;
	sub_82DF3A08(ctx, base);
	// 8282AA2C: 817F0278  lwz r11, 0x278(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8282AA30: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282AA34: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8282AA38: 394AB178  addi r10, r10, -0x4e88
	ctx.r[10].s64 = ctx.r[10].s64 + -20104;
	// 8282AA3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282AA40: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282AA44: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8282AA48: 485C8FC1  bl 0x82df3a08
	ctx.lr = 0x8282AA4C;
	sub_82DF3A08(ctx, base);
	// 8282AA4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282AA50: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282AA54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8282AA58: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8282AA5C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8282AA60: 4BFF63C9  bl 0x82820e28
	ctx.lr = 0x8282AA64;
	sub_82820E28(ctx, base);
	// 8282AA64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282AA68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282AA6C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8282AA70: 388AB1A8  addi r4, r10, -0x4e58
	ctx.r[4].s64 = ctx.r[10].s64 + -20056;
	// 8282AA74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282AA78: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282AA7C: 4803B26D  bl 0x82865ce8
	ctx.lr = 0x8282AA80;
	sub_82865CE8(ctx, base);
	// 8282AA80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282AA84: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282AA88: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8282AA8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282AA90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8282AA94: 419A0024  beq cr6, 0x8282aab8
	if ctx.cr[6].eq {
	pc = 0x8282AAB8; continue 'dispatch;
	}
	// 8282AA98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8282AA9C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282AAA0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282AAA4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282AAA8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282AAAC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282AAB0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282AAB4: 4082FFE8  bne 0x8282aa9c
	if !ctx.cr[0].eq {
	pc = 0x8282AA9C; continue 'dispatch;
	}
	// 8282AAB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282AABC: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8282AAC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AAC4: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8282AAC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282AACC: 4E800421  bctrl
	ctx.lr = 0x8282AAD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282AAD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282AAD4: 4BFC4BCD  bl 0x827ef6a0
	ctx.lr = 0x8282AAD8;
	sub_827EF6A0(ctx, base);
	// 8282AAD8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282AADC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282AAE0: 419A0008  beq cr6, 0x8282aae8
	if ctx.cr[6].eq {
	pc = 0x8282AAE8; continue 'dispatch;
	}
	// 8282AAE4: 4BA95DAD  bl 0x822c0890
	ctx.lr = 0x8282AAE8;
	sub_822C0890(ctx, base);
	// 8282AAE8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8282AAEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282AAF0: 419A0008  beq cr6, 0x8282aaf8
	if ctx.cr[6].eq {
	pc = 0x8282AAF8; continue 'dispatch;
	}
	// 8282AAF4: 4BA95D9D  bl 0x822c0890
	ctx.lr = 0x8282AAF8;
	sub_822C0890(ctx, base);
	// 8282AAF8: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8282AAFC: 485C892D  bl 0x82df3428
	ctx.lr = 0x8282AB00;
	sub_82DF3428(ctx, base);
	// 8282AB00: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8282AB04: 485C8925  bl 0x82df3428
	ctx.lr = 0x8282AB08;
	sub_82DF3428(ctx, base);
	// 8282AB08: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8282AB0C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8282AB10: 4897D6AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282AB18 size=12
    let mut pc: u32 = 0x8282AB18;
    'dispatch: loop {
        match pc {
            0x8282AB18 => {
    //   block [0x8282AB18..0x8282AB24)
	// 8282AB18: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8282AB1C: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8282AB20: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AB24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282AB24 size=8
    let mut pc: u32 = 0x8282AB24;
    'dispatch: loop {
        match pc {
            0x8282AB24 => {
    //   block [0x8282AB24..0x8282AB2C)
	// 8282AB24: 4BFFFE9C  b 0x8282a9c0
	sub_8282A9C0(ctx, base);
	return;
	// 8282AB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282AB30 size=184
    let mut pc: u32 = 0x8282AB30;
    'dispatch: loop {
        match pc {
            0x8282AB30 => {
    //   block [0x8282AB30..0x8282ABE8)
	// 8282AB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282AB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282AB38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282AB3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282AB40: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282AB44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282AB48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282AB4C: 388BB1C0  addi r4, r11, -0x4e40
	ctx.r[4].s64 = ctx.r[11].s64 + -20032;
	// 8282AB50: 485C8EB9  bl 0x82df3a08
	ctx.lr = 0x8282AB54;
	sub_82DF3A08(ctx, base);
	// 8282AB54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282AB58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282AB5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282AB60: C02B9584  lfs f1, -0x6a7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282AB64: 48038BD5  bl 0x82863738
	ctx.lr = 0x8282AB68;
	sub_82863738(ctx, base);
	// 8282AB68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282AB6C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282AB70: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8282AB74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282AB78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8282AB7C: 419A0024  beq cr6, 0x8282aba0
	if ctx.cr[6].eq {
	pc = 0x8282ABA0; continue 'dispatch;
	}
	// 8282AB80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8282AB84: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282AB88: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282AB8C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282AB90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282AB94: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282AB98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282AB9C: 4082FFE8  bne 0x8282ab84
	if !ctx.cr[0].eq {
	pc = 0x8282AB84; continue 'dispatch;
	}
	// 8282ABA0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8282ABA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282ABA8: 4BFC21B1  bl 0x827ecd58
	ctx.lr = 0x8282ABAC;
	sub_827ECD58(ctx, base);
	// 8282ABAC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282ABB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282ABB4: 419A0008  beq cr6, 0x8282abbc
	if ctx.cr[6].eq {
	pc = 0x8282ABBC; continue 'dispatch;
	}
	// 8282ABB8: 4BA95CD9  bl 0x822c0890
	ctx.lr = 0x8282ABBC;
	sub_822C0890(ctx, base);
	// 8282ABBC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8282ABC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282ABC4: 419A0008  beq cr6, 0x8282abcc
	if ctx.cr[6].eq {
	pc = 0x8282ABCC; continue 'dispatch;
	}
	// 8282ABC8: 4BA95CC9  bl 0x822c0890
	ctx.lr = 0x8282ABCC;
	sub_822C0890(ctx, base);
	// 8282ABCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282ABD0: 485C8859  bl 0x82df3428
	ctx.lr = 0x8282ABD4;
	sub_82DF3428(ctx, base);
	// 8282ABD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8282ABD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282ABDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282ABE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282ABE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282ABE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282ABE8 size=112
    let mut pc: u32 = 0x8282ABE8;
    'dispatch: loop {
        match pc {
            0x8282ABE8 => {
    //   block [0x8282ABE8..0x8282AC58)
	// 8282ABE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282ABEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282ABF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282ABF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282ABF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282ABFC: 4BFFF615  bl 0x8282a210
	ctx.lr = 0x8282AC00;
	sub_8282A210(ctx, base);
	// 8282AC00: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282AC04: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8282AC08: 394AB3A4  addi r10, r10, -0x4c5c
	ctx.r[10].s64 = ctx.r[10].s64 + -19548;
	// 8282AC0C: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282AC10: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 8282AC14: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8282AC18: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 8282AC1C: 38E002F0  li r7, 0x2f0
	ctx.r[7].s64 = 752;
	// 8282AC20: 3929B390  addi r9, r9, -0x4c70
	ctx.r[9].s64 = ctx.r[9].s64 + -19568;
	// 8282AC24: 3908B344  addi r8, r8, -0x4cbc
	ctx.r[8].s64 = ctx.r[8].s64 + -19644;
	// 8282AC28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8282AC2C: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8282AC30: 911F0208  stw r8, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[8].u32 ) };
	// 8282AC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AC38: 915F02E0  stw r10, 0x2e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), ctx.r[10].u32 ) };
	// 8282AC3C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282AC58 size=112
    let mut pc: u32 = 0x8282AC58;
    'dispatch: loop {
        match pc {
            0x8282AC58 => {
    //   block [0x8282AC58..0x8282ACC8)
	// 8282AC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282AC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282AC60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282AC64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282AC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282AC6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282AC70: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282AC74: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282AC78: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282AC7C: 396BB3A4  addi r11, r11, -0x4c5c
	ctx.r[11].s64 = ctx.r[11].s64 + -19548;
	// 8282AC80: 394AB390  addi r10, r10, -0x4c70
	ctx.r[10].s64 = ctx.r[10].s64 + -19568;
	// 8282AC84: 3929B344  addi r9, r9, -0x4cbc
	ctx.r[9].s64 = ctx.r[9].s64 + -19644;
	// 8282AC88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282AC8C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8282AC90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282AC94: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 8282AC98: 4BFFF229  bl 0x82829ec0
	ctx.lr = 0x8282AC9C;
	sub_82829EC0(ctx, base);
	// 8282AC9C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282ACA0: 4182000C  beq 0x8282acac
	if ctx.cr[0].eq {
	pc = 0x8282ACAC; continue 'dispatch;
	}
	// 8282ACA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282ACA8: 485C7731  bl 0x82df23d8
	ctx.lr = 0x8282ACAC;
	sub_82DF23D8(ctx, base);
	// 8282ACAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282ACB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282ACBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282ACC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282ACC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282ACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282ACC8 size=96
    let mut pc: u32 = 0x8282ACC8;
    'dispatch: loop {
        match pc {
            0x8282ACC8 => {
    //   block [0x8282ACC8..0x8282AD28)
	// 8282ACC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282ACCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282ACD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282ACD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282ACD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282ACDC: 4BFFF535  bl 0x8282a210
	ctx.lr = 0x8282ACE0;
	sub_8282A210(ctx, base);
	// 8282ACE0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282ACE4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282ACE8: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282ACEC: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 8282ACF0: 394AB48C  addi r10, r10, -0x4b74
	ctx.r[10].s64 = ctx.r[10].s64 + -19316;
	// 8282ACF4: C00B89AC  lfs f0, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282ACF8: 3929B478  addi r9, r9, -0x4b88
	ctx.r[9].s64 = ctx.r[9].s64 + -19336;
	// 8282ACFC: 3968B42C  addi r11, r8, -0x4bd4
	ctx.r[11].s64 = ctx.r[8].s64 + -19412;
	// 8282AD00: D01F02E0  stfs f0, 0x2e0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), tmp.u32 ) };
	// 8282AD04: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8282AD08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AD0C: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8282AD10: 917F0208  stw r11, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[11].u32 ) };
	// 8282AD14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282AD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282AD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282AD20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282AD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282AD28 size=112
    let mut pc: u32 = 0x8282AD28;
    'dispatch: loop {
        match pc {
            0x8282AD28 => {
    //   block [0x8282AD28..0x8282AD98)
	// 8282AD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282AD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282AD30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282AD34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282AD38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282AD3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282AD40: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282AD44: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282AD48: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282AD4C: 396BB48C  addi r11, r11, -0x4b74
	ctx.r[11].s64 = ctx.r[11].s64 + -19316;
	// 8282AD50: 394AB478  addi r10, r10, -0x4b88
	ctx.r[10].s64 = ctx.r[10].s64 + -19336;
	// 8282AD54: 3929B42C  addi r9, r9, -0x4bd4
	ctx.r[9].s64 = ctx.r[9].s64 + -19412;
	// 8282AD58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282AD5C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8282AD60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282AD64: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 8282AD68: 4BFFF159  bl 0x82829ec0
	ctx.lr = 0x8282AD6C;
	sub_82829EC0(ctx, base);
	// 8282AD6C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282AD70: 4182000C  beq 0x8282ad7c
	if ctx.cr[0].eq {
	pc = 0x8282AD7C; continue 'dispatch;
	}
	// 8282AD74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AD78: 485C7661  bl 0x82df23d8
	ctx.lr = 0x8282AD7C;
	sub_82DF23D8(ctx, base);
	// 8282AD7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AD80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282AD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282AD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282AD8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282AD90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282AD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282AD98 size=220
    let mut pc: u32 = 0x8282AD98;
    'dispatch: loop {
        match pc {
            0x8282AD98 => {
    //   block [0x8282AD98..0x8282AE74)
	// 8282AD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282AD9C: 4897D3D1  bl 0x831a816c
	ctx.lr = 0x8282ADA0;
	sub_831A8130(ctx, base);
	// 8282ADA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282ADA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282ADA8: 4BCE7589  bl 0x82512330
	ctx.lr = 0x8282ADAC;
	sub_82512330(ctx, base);
	// 8282ADAC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8282ADB0: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 8282ADB4: 396BA214  addi r11, r11, -0x5dec
	ctx.r[11].s64 = ctx.r[11].s64 + -24044;
	// 8282ADB8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282ADBC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282ADC0: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8282ADC4: 394AB594  addi r10, r10, -0x4a6c
	ctx.r[10].s64 = ctx.r[10].s64 + -19052;
	// 8282ADC8: 3929B57C  addi r9, r9, -0x4a84
	ctx.r[9].s64 = ctx.r[9].s64 + -19076;
	// 8282ADCC: 3968B5D4  addi r11, r8, -0x4a2c
	ctx.r[11].s64 = ctx.r[8].s64 + -18988;
	// 8282ADD0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8282ADD4: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8282ADD8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282ADDC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8282ADE0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282ADE4: 388AB518  addi r4, r10, -0x4ae8
	ctx.r[4].s64 = ctx.r[10].s64 + -19176;
	// 8282ADE8: 38A00398  li r5, 0x398
	ctx.r[5].s64 = 920;
	// 8282ADEC: 386002F0  li r3, 0x2f0
	ctx.r[3].s64 = 752;
	// 8282ADF0: 485C75F9  bl 0x82df23e8
	ctx.lr = 0x8282ADF4;
	sub_82DF23E8(ctx, base);
	// 8282ADF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282ADF8: 41820010  beq 0x8282ae08
	if ctx.cr[0].eq {
	pc = 0x8282AE08; continue 'dispatch;
	}
	// 8282ADFC: 4BFFFECD  bl 0x8282acc8
	ctx.lr = 0x8282AE00;
	sub_8282ACC8(ctx, base);
	// 8282AE00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8282AE04: 48000008  b 0x8282ae0c
	pc = 0x8282AE0C; continue 'dispatch;
	// 8282AE08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8282AE0C: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 8282AE10: 397F00E8  addi r11, r31, 0xe8
	ctx.r[11].s64 = ctx.r[31].s64 + 232;
	// 8282AE14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282AE18: 3BAB0004  addi r29, r11, 4
	ctx.r[29].s64 = ctx.r[11].s64 + 4;
	// 8282AE1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282AE20: 4BFFEF11  bl 0x82829d30
	ctx.lr = 0x8282AE24;
	sub_82829D30(ctx, base);
	// 8282AE24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8282AE28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282AE2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282AE30: 4BA951D1  bl 0x822c0000
	ctx.lr = 0x8282AE34;
	sub_822C0000(ctx, base);
	// 8282AE34: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8282AE38: 394000F0  li r10, 0xf0
	ctx.r[10].s64 = 240;
	// 8282AE3C: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 8282AE40: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282AE44: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8282AE48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AE4C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282AE50: C00908A8  lfs f0, 0x8a8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282AE54: C1A808A4  lfs f13, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282AE78 size=8
    let mut pc: u32 = 0x8282AE78;
    'dispatch: loop {
        match pc {
            0x8282AE78 => {
    //   block [0x8282AE78..0x8282AE80)
	// 8282AE78: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8282AE7C: 480012EC  b 0x8282c168
	sub_8282C168(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282AE80 size=8
    let mut pc: u32 = 0x8282AE80;
    'dispatch: loop {
        match pc {
            0x8282AE80 => {
    //   block [0x8282AE80..0x8282AE88)
	// 8282AE80: 3863FF1C  addi r3, r3, -0xe4
	ctx.r[3].s64 = ctx.r[3].s64 + -228;
	// 8282AE84: 480012E4  b 0x8282c168
	sub_8282C168(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282AE88 size=120
    let mut pc: u32 = 0x8282AE88;
    'dispatch: loop {
        match pc {
            0x8282AE88 => {
    //   block [0x8282AE88..0x8282AF00)
	// 8282AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282AE90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282AE94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282AE98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282AE9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282AEA0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282AEA4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282AEA8: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8282AEAC: 396BB594  addi r11, r11, -0x4a6c
	ctx.r[11].s64 = ctx.r[11].s64 + -19052;
	// 8282AEB0: 394AB57C  addi r10, r10, -0x4a84
	ctx.r[10].s64 = ctx.r[10].s64 + -19076;
	// 8282AEB4: 3929B5D4  addi r9, r9, -0x4a2c
	ctx.r[9].s64 = ctx.r[9].s64 + -18988;
	// 8282AEB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8282AEBC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8282AEC0: 3BDF00E4  addi r30, r31, 0xe4
	ctx.r[30].s64 = ctx.r[31].s64 + 228;
	// 8282AEC4: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 8282AEC8: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8282AECC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282AED0: 419A0008  beq cr6, 0x8282aed8
	if ctx.cr[6].eq {
	pc = 0x8282AED8; continue 'dispatch;
	}
	// 8282AED4: 4BA959BD  bl 0x822c0890
	ctx.lr = 0x8282AED8;
	sub_822C0890(ctx, base);
	// 8282AED8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282AEDC: 4BF7C77D  bl 0x827a7658
	ctx.lr = 0x8282AEE0;
	sub_827A7658(ctx, base);
	// 8282AEE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AEE4: 4BB2446D  bl 0x8234f350
	ctx.lr = 0x8282AEE8;
	sub_8234F350(ctx, base);
	// 8282AEE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282AEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282AEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282AEF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282AEF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282AEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282AF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282AF00 size=360
    let mut pc: u32 = 0x8282AF00;
    'dispatch: loop {
        match pc {
            0x8282AF00 => {
    //   block [0x8282AF00..0x8282B068)
	// 8282AF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282AF04: 4897D265  bl 0x831a8168
	ctx.lr = 0x8282AF08;
	sub_831A8130(ctx, base);
	// 8282AF08: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282AF0C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8282AF10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282AF14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282AF18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8282AF1C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8282AF20: 808BE268  lwz r4, -0x1d98(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7576 as u32) ) } as u64;
	// 8282AF24: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8282AF28: 485C8AE1  bl 0x82df3a08
	ctx.lr = 0x8282AF2C;
	sub_82DF3A08(ctx, base);
	// 8282AF2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282AF30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282AF34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282AF38: 4BCDD849  bl 0x82508780
	ctx.lr = 0x8282AF3C;
	sub_82508780(ctx, base);
	// 8282AF3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282AF40: 485C84E9  bl 0x82df3428
	ctx.lr = 0x8282AF44;
	sub_82DF3428(ctx, base);
	// 8282AF44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8282AF48: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 8282AF4C: 409A0008  bne cr6, 0x8282af54
	if !ctx.cr[6].eq {
	pc = 0x8282AF54; continue 'dispatch;
	}
	// 8282AF50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8282AF54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282AF58: 4BCDD849  bl 0x825087a0
	ctx.lr = 0x8282AF5C;
	sub_825087A0(ctx, base);
	// 8282AF5C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8282AF60: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8282AF64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8282AF68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282AF6C: 4BCE6A6D  bl 0x825119d8
	ctx.lr = 0x8282AF70;
	sub_825119D8(ctx, base);
	// 8282AF70: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282AF74: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8282AF78: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8282AF7C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8282AF80: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8282AF84: 486757AD  bl 0x82ea0730
	ctx.lr = 0x8282AF88;
	sub_82EA0730(ctx, base);
	// 8282AF88: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8282AF8C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8282AF90: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8282AF94: C19F0100  lfs f12, 0x100(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8282AF98: C17F0104  lfs f11, 0x104(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8282AF9C: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 8282AFA0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8282AFA4: C1BF0108  lfs f13, 0x108(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8282AFA8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8282AFAC: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8282AFB0: C029F614  lfs f1, -0x9ec(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2540 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282AFB4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8282AFB8: C00808A8  lfs f0, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282AFBC: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8282AFC0: D1610064  stfs f11, 0x64(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8282AFC4: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8282AFC8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282B068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282B068 size=584
    let mut pc: u32 = 0x8282B068;
    'dispatch: loop {
        match pc {
            0x8282B068 => {
    //   block [0x8282B068..0x8282B2B0)
	// 8282B068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282B06C: 4897D0FD  bl 0x831a8168
	ctx.lr = 0x8282B070;
	sub_831A8130(ctx, base);
	// 8282B070: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8282B074: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8282B078: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282B07C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8282B080: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8282B084: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8282B088: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8282B08C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8282B090: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282B094: 419A0018  beq cr6, 0x8282b0ac
	if ctx.cr[6].eq {
	pc = 0x8282B0AC; continue 'dispatch;
	}
	// 8282B098: 814B0208  lwz r10, 0x208(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(520 as u32) ) } as u64;
	// 8282B09C: 386B0208  addi r3, r11, 0x208
	ctx.r[3].s64 = ctx.r[11].s64 + 520;
	// 8282B0A0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8282B0A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282B0A8: 4E800421  bctrl
	ctx.lr = 0x8282B0AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282B0AC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B0B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282B0B4: 388BB518  addi r4, r11, -0x4ae8
	ctx.r[4].s64 = ctx.r[11].s64 + -19176;
	// 8282B0B8: 38A003C7  li r5, 0x3c7
	ctx.r[5].s64 = 967;
	// 8282B0BC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8282B0C0: 4BA95319  bl 0x822c03d8
	ctx.lr = 0x8282B0C4;
	sub_822C03D8(ctx, base);
	// 8282B0C4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282B0C8: 41820034  beq 0x8282b0fc
	if ctx.cr[0].eq {
	pc = 0x8282B0FC; continue 'dispatch;
	}
	// 8282B0CC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B0D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B0D4: 388BB660  addi r4, r11, -0x49a0
	ctx.r[4].s64 = ctx.r[11].s64 + -18848;
	// 8282B0D8: 485C8931  bl 0x82df3a08
	ctx.lr = 0x8282B0DC;
	sub_82DF3A08(ctx, base);
	// 8282B0DC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8282B0E0: 38BE000C  addi r5, r30, 0xc
	ctx.r[5].s64 = ctx.r[30].s64 + 12;
	// 8282B0E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282B0EC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8282B0F0: 4BD7E409  bl 0x825a94f8
	ctx.lr = 0x8282B0F4;
	sub_825A94F8(ctx, base);
	// 8282B0F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282B0F8: 48000008  b 0x8282b100
	pc = 0x8282B100; continue 'dispatch;
	// 8282B0FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282B100: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8282B104: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282B108: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8282B10C: 4BD7C6DD  bl 0x825a77e8
	ctx.lr = 0x8282B110;
	sub_825A77E8(ctx, base);
	// 8282B110: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282B114: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282B118: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8282B11C: 4BA94EE5  bl 0x822c0000
	ctx.lr = 0x8282B120;
	sub_822C0000(ctx, base);
	// 8282B120: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282B124: 4182000C  beq 0x8282b130
	if ctx.cr[0].eq {
	pc = 0x8282B130; continue 'dispatch;
	}
	// 8282B128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B12C: 485C82FD  bl 0x82df3428
	ctx.lr = 0x8282B130;
	sub_82DF3428(ctx, base);
	// 8282B130: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B134: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B138: 388BB654  addi r4, r11, -0x49ac
	ctx.r[4].s64 = ctx.r[11].s64 + -18860;
	// 8282B13C: 485C88CD  bl 0x82df3a08
	ctx.lr = 0x8282B140;
	sub_82DF3A08(ctx, base);
	// 8282B140: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282B144: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8282B148: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8282B14C: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8282B150: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8282B154: 419A0024  beq cr6, 0x8282b178
	if ctx.cr[6].eq {
	pc = 0x8282B178; continue 'dispatch;
	}
	// 8282B158: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8282B15C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282B160: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282B164: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282B168: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282B16C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282B170: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282B174: 4082FFE8  bne 0x8282b15c
	if !ctx.cr[0].eq {
	pc = 0x8282B15C; continue 'dispatch;
	}
	// 8282B178: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8282B17C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B180: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8282B184: 4BD76F8D  bl 0x825a2110
	ctx.lr = 0x8282B188;
	sub_825A2110(ctx, base);
	// 8282B188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B18C: 485C829D  bl 0x82df3428
	ctx.lr = 0x8282B190;
	sub_82DF3428(ctx, base);
	// 8282B190: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8282B194: 419A000C  beq cr6, 0x8282b1a0
	if ctx.cr[6].eq {
	pc = 0x8282B1A0; continue 'dispatch;
	}
	// 8282B198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282B19C: 4BA956F5  bl 0x822c0890
	ctx.lr = 0x8282B1A0;
	sub_822C0890(ctx, base);
	// 8282B1A0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B1A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B1A8: 388BB648  addi r4, r11, -0x49b8
	ctx.r[4].s64 = ctx.r[11].s64 + -18872;
	// 8282B1AC: 485C885D  bl 0x82df3a08
	ctx.lr = 0x8282B1B0;
	sub_82DF3A08(ctx, base);
	// 8282B1B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282B1B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8282B1B8: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 8282B1BC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8282B1C0: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282B1C4: C3CA6218  lfs f30, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8282B1C8: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B1CC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8282B1D0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B1D4: 4BD77FDD  bl 0x825a31b0
	ctx.lr = 0x8282B1D8;
	sub_825A31B0(ctx, base);
	// 8282B1D8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B1DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B1E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8282B1E4: 4BD7658D  bl 0x825a1770
	ctx.lr = 0x8282B1E8;
	sub_825A1770(ctx, base);
	// 8282B1E8: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8282B1EC: 485C823D  bl 0x82df3428
	ctx.lr = 0x8282B1F0;
	sub_82DF3428(ctx, base);
	// 8282B1F0: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8282B1F4: 4BA9DAC5  bl 0x822c8cb8
	ctx.lr = 0x8282B1F8;
	sub_822C8CB8(ctx, base);
	// 8282B1F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B1FC: 485C822D  bl 0x82df3428
	ctx.lr = 0x8282B200;
	sub_82DF3428(ctx, base);
	// 8282B200: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B204: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B208: 388BB63C  addi r4, r11, -0x49c4
	ctx.r[4].s64 = ctx.r[11].s64 + -18884;
	// 8282B20C: 485C87FD  bl 0x82df3a08
	ctx.lr = 0x8282B210;
	sub_82DF3A08(ctx, base);
	// 8282B210: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 8282B214: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8282B218: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B21C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8282B220: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B224: 4BD77F8D  bl 0x825a31b0
	ctx.lr = 0x8282B228;
	sub_825A31B0(ctx, base);
	// 8282B228: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B22C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B230: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8282B234: 4BD7653D  bl 0x825a1770
	ctx.lr = 0x8282B238;
	sub_825A1770(ctx, base);
	// 8282B238: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8282B23C: 485C81ED  bl 0x82df3428
	ctx.lr = 0x8282B240;
	sub_82DF3428(ctx, base);
	// 8282B240: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8282B244: 4BA9DA75  bl 0x822c8cb8
	ctx.lr = 0x8282B248;
	sub_822C8CB8(ctx, base);
	// 8282B248: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B24C: 485C81DD  bl 0x82df3428
	ctx.lr = 0x8282B250;
	sub_82DF3428(ctx, base);
	// 8282B250: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B254: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B258: 388BB630  addi r4, r11, -0x49d0
	ctx.r[4].s64 = ctx.r[11].s64 + -18896;
	// 8282B25C: 485C87AD  bl 0x82df3a08
	ctx.lr = 0x8282B260;
	sub_82DF3A08(ctx, base);
	// 8282B260: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 8282B264: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8282B268: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B26C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8282B270: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B274: 4BD77F3D  bl 0x825a31b0
	ctx.lr = 0x8282B278;
	sub_825A31B0(ctx, base);
	// 8282B278: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B27C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B280: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8282B284: 4BD764ED  bl 0x825a1770
	ctx.lr = 0x8282B288;
	sub_825A1770(ctx, base);
	// 8282B288: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 8282B28C: 485C819D  bl 0x82df3428
	ctx.lr = 0x8282B290;
	sub_82DF3428(ctx, base);
	// 8282B290: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 8282B294: 4BA9DA25  bl 0x822c8cb8
	ctx.lr = 0x8282B298;
	sub_822C8CB8(ctx, base);
	// 8282B298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B29C: 485C818D  bl 0x82df3428
	ctx.lr = 0x8282B2A0;
	sub_82DF3428(ctx, base);
	// 8282B2A0: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 8282B2A4: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8282B2A8: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8282B2AC: 4897CF0C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282B2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282B2B0 size=1284
    let mut pc: u32 = 0x8282B2B0;
    'dispatch: loop {
        match pc {
            0x8282B2B0 => {
    //   block [0x8282B2B0..0x8282B7B4)
	// 8282B2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282B2B4: 4897CEA5  bl 0x831a8158
	ctx.lr = 0x8282B2B8;
	sub_831A8130(ctx, base);
	// 8282B2B8: DBC1FFA8  stfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 8282B2BC: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8282B2C0: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282B2C4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8282B2C8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B2CC: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8282B2D0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8282B2D4: 3B4BB518  addi r26, r11, -0x4ae8
	ctx.r[26].s64 = ctx.r[11].s64 + -19176;
	// 8282B2D8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8282B2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282B2E0: 38A000F5  li r5, 0xf5
	ctx.r[5].s64 = 245;
	// 8282B2E4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8282B2E8: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8282B2EC: 4BA950ED  bl 0x822c03d8
	ctx.lr = 0x8282B2F0;
	sub_822C03D8(ctx, base);
	// 8282B2F0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282B2F4: 41820034  beq 0x8282b328
	if ctx.cr[0].eq {
	pc = 0x8282B328; continue 'dispatch;
	}
	// 8282B2F8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B2FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B300: 388BB6C8  addi r4, r11, -0x4938
	ctx.r[4].s64 = ctx.r[11].s64 + -18744;
	// 8282B304: 485C8705  bl 0x82df3a08
	ctx.lr = 0x8282B308;
	sub_82DF3A08(ctx, base);
	// 8282B308: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282B30C: 38B9006C  addi r5, r25, 0x6c
	ctx.r[5].s64 = ctx.r[25].s64 + 108;
	// 8282B310: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282B318: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8282B31C: 4BD810F5  bl 0x825ac410
	ctx.lr = 0x8282B320;
	sub_825AC410(ctx, base);
	// 8282B320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282B324: 48000008  b 0x8282b32c
	pc = 0x8282B32C; continue 'dispatch;
	// 8282B328: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282B32C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8282B330: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282B334: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8282B338: 4BCC8081  bl 0x824f33b8
	ctx.lr = 0x8282B33C;
	sub_824F33B8(ctx, base);
	// 8282B33C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282B340: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282B344: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8282B348: 4BA94CB9  bl 0x822c0000
	ctx.lr = 0x8282B34C;
	sub_822C0000(ctx, base);
	// 8282B34C: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282B350: 41820010  beq 0x8282b360
	if ctx.cr[0].eq {
	pc = 0x8282B360; continue 'dispatch;
	}
	// 8282B354: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B358: 579C003C  rlwinm r28, r28, 0, 0, 0x1e
	ctx.r[28].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 8282B35C: 485C80CD  bl 0x82df3428
	ctx.lr = 0x8282B360;
	sub_82DF3428(ctx, base);
	// 8282B360: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B364: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8282B368: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8282B36C: 3B6BB16C  addi r27, r11, -0x4e94
	ctx.r[27].s64 = ctx.r[11].s64 + -20116;
	// 8282B370: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 8282B374: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B378: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282B37C: 485C868D  bl 0x82df3a08
	ctx.lr = 0x8282B380;
	sub_82DF3A08(ctx, base);
	// 8282B380: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282B384: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282B388: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282B38C: 4BD80A35  bl 0x825abdc0
	ctx.lr = 0x8282B390;
	sub_825ABDC0(ctx, base);
	// 8282B390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B394: 485C8095  bl 0x82df3428
	ctx.lr = 0x8282B398;
	sub_82DF3428(ctx, base);
	// 8282B398: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8282B39C: 397B000C  addi r11, r27, 0xc
	ctx.r[11].s64 = ctx.r[27].s64 + 12;
	// 8282B3A0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8282B3A4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8282B3A8: 4198FFCC  blt cr6, 0x8282b374
	if ctx.cr[6].lt {
	pc = 0x8282B374; continue 'dispatch;
	}
	// 8282B3AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8282B3B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B3B4: 388B44A0  addi r4, r11, 0x44a0
	ctx.r[4].s64 = ctx.r[11].s64 + 17568;
	// 8282B3B8: 485C8651  bl 0x82df3a08
	ctx.lr = 0x8282B3BC;
	sub_82DF3A08(ctx, base);
	// 8282B3BC: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282B3C0: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 8282B3C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8282B3C8: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8282B3CC: 419A0024  beq cr6, 0x8282b3f0
	if ctx.cr[6].eq {
	pc = 0x8282B3F0; continue 'dispatch;
	}
	// 8282B3D0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8282B3D4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282B3D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282B3DC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282B3E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282B3E4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282B3E8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282B3EC: 4082FFE8  bne 0x8282b3d4
	if !ctx.cr[0].eq {
	pc = 0x8282B3D4; continue 'dispatch;
	}
	// 8282B3F0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8282B3F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B3F8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B3FC: 4BD76D15  bl 0x825a2110
	ctx.lr = 0x8282B400;
	sub_825A2110(ctx, base);
	// 8282B400: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B404: 485C8025  bl 0x82df3428
	ctx.lr = 0x8282B408;
	sub_82DF3428(ctx, base);
	// 8282B408: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8282B40C: 419A000C  beq cr6, 0x8282b418
	if ctx.cr[6].eq {
	pc = 0x8282B418; continue 'dispatch;
	}
	// 8282B410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282B414: 4BA9547D  bl 0x822c0890
	ctx.lr = 0x8282B418;
	sub_822C0890(ctx, base);
	// 8282B418: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8282B41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282B420: 38A000FC  li r5, 0xfc
	ctx.r[5].s64 = 252;
	// 8282B424: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8282B428: 4BA94FB1  bl 0x822c03d8
	ctx.lr = 0x8282B42C;
	sub_822C03D8(ctx, base);
	// 8282B42C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B430: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282B434: 3B4BA0D0  addi r26, r11, -0x5f30
	ctx.r[26].s64 = ctx.r[11].s64 + -24368;
	// 8282B438: 41820030  beq 0x8282b468
	if ctx.cr[0].eq {
	pc = 0x8282B468; continue 'dispatch;
	}
	// 8282B43C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8282B440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B444: 485C85C5  bl 0x82df3a08
	ctx.lr = 0x8282B448;
	sub_82DF3A08(ctx, base);
	// 8282B448: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282B44C: 38B90070  addi r5, r25, 0x70
	ctx.r[5].s64 = ctx.r[25].s64 + 112;
	// 8282B450: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282B458: 639C0002  ori r28, r28, 2
	ctx.r[28].u64 = ctx.r[28].u64 | 2;
	// 8282B45C: 4BD80FB5  bl 0x825ac410
	ctx.lr = 0x8282B460;
	sub_825AC410(ctx, base);
	// 8282B460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282B464: 48000008  b 0x8282b46c
	pc = 0x8282B46C; continue 'dispatch;
	// 8282B468: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282B46C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8282B470: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282B474: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8282B478: 4BCC7F41  bl 0x824f33b8
	ctx.lr = 0x8282B47C;
	sub_824F33B8(ctx, base);
	// 8282B47C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282B480: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282B484: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8282B488: 4BA94B79  bl 0x822c0000
	ctx.lr = 0x8282B48C;
	sub_822C0000(ctx, base);
	// 8282B48C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282B490: 4182000C  beq 0x8282b49c
	if ctx.cr[0].eq {
	pc = 0x8282B49C; continue 'dispatch;
	}
	// 8282B494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B498: 485C7F91  bl 0x82df3428
	ctx.lr = 0x8282B49C;
	sub_82DF3428(ctx, base);
	// 8282B49C: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8282B4A0: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 8282B4A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8282B4A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B4AC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282B4B0: 485C8559  bl 0x82df3a08
	ctx.lr = 0x8282B4B4;
	sub_82DF3A08(ctx, base);
	// 8282B4B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282B4B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282B4BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282B4C0: 4BD80901  bl 0x825abdc0
	ctx.lr = 0x8282B4C4;
	sub_825ABDC0(ctx, base);
	// 8282B4C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B4C8: 485C7F61  bl 0x82df3428
	ctx.lr = 0x8282B4CC;
	sub_82DF3428(ctx, base);
	// 8282B4CC: 397B000C  addi r11, r27, 0xc
	ctx.r[11].s64 = ctx.r[27].s64 + 12;
	// 8282B4D0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8282B4D4: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8282B4D8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8282B4DC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8282B4E0: 4198FFC8  blt cr6, 0x8282b4a8
	if ctx.cr[6].lt {
	pc = 0x8282B4A8; continue 'dispatch;
	}
	// 8282B4E4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8282B4E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B4EC: 485C851D  bl 0x82df3a08
	ctx.lr = 0x8282B4F0;
	sub_82DF3A08(ctx, base);
	// 8282B4F0: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282B4F4: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 8282B4F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8282B4FC: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8282B500: 419A0024  beq cr6, 0x8282b524
	if ctx.cr[6].eq {
	pc = 0x8282B524; continue 'dispatch;
	}
	// 8282B504: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8282B508: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282B50C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282B510: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282B514: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282B518: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282B51C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282B520: 4082FFE8  bne 0x8282b508
	if !ctx.cr[0].eq {
	pc = 0x8282B508; continue 'dispatch;
	}
	// 8282B524: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8282B528: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B52C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B530: 4BD76BE1  bl 0x825a2110
	ctx.lr = 0x8282B534;
	sub_825A2110(ctx, base);
	// 8282B534: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B538: 485C7EF1  bl 0x82df3428
	ctx.lr = 0x8282B53C;
	sub_82DF3428(ctx, base);
	// 8282B53C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8282B540: 419A000C  beq cr6, 0x8282b54c
	if ctx.cr[6].eq {
	pc = 0x8282B54C; continue 'dispatch;
	}
	// 8282B544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282B548: 4BA95349  bl 0x822c0890
	ctx.lr = 0x8282B54C;
	sub_822C0890(ctx, base);
	// 8282B54C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8282B550: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B554: 388BC014  addi r4, r11, -0x3fec
	ctx.r[4].s64 = ctx.r[11].s64 + -16364;
	// 8282B558: 485C84B1  bl 0x82df3a08
	ctx.lr = 0x8282B55C;
	sub_82DF3A08(ctx, base);
	// 8282B55C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282B560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8282B564: 3899009C  addi r4, r25, 0x9c
	ctx.r[4].s64 = ctx.r[25].s64 + 156;
	// 8282B568: 386101B0  addi r3, r1, 0x1b0
	ctx.r[3].s64 = ctx.r[1].s64 + 432;
	// 8282B56C: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282B570: C04A6218  lfs f2, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8282B574: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B578: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B57C: 4BD77C35  bl 0x825a31b0
	ctx.lr = 0x8282B580;
	sub_825A31B0(ctx, base);
	// 8282B580: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B584: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B588: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B58C: 4BD761E5  bl 0x825a1770
	ctx.lr = 0x8282B590;
	sub_825A1770(ctx, base);
	// 8282B590: 386101E8  addi r3, r1, 0x1e8
	ctx.r[3].s64 = ctx.r[1].s64 + 488;
	// 8282B594: 485C7E95  bl 0x82df3428
	ctx.lr = 0x8282B598;
	sub_82DF3428(ctx, base);
	// 8282B598: 386101C8  addi r3, r1, 0x1c8
	ctx.r[3].s64 = ctx.r[1].s64 + 456;
	// 8282B59C: 4BA9D71D  bl 0x822c8cb8
	ctx.lr = 0x8282B5A0;
	sub_822C8CB8(ctx, base);
	// 8282B5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B5A4: 485C7E85  bl 0x82df3428
	ctx.lr = 0x8282B5A8;
	sub_82DF3428(ctx, base);
	// 8282B5A8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B5AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B5B0: 388BB6B8  addi r4, r11, -0x4948
	ctx.r[4].s64 = ctx.r[11].s64 + -18760;
	// 8282B5B4: 485C8455  bl 0x82df3a08
	ctx.lr = 0x8282B5B8;
	sub_82DF3A08(ctx, base);
	// 8282B5B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282B5BC: 38990094  addi r4, r25, 0x94
	ctx.r[4].s64 = ctx.r[25].s64 + 148;
	// 8282B5C0: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B5C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8282B5C8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B5CC: C3CB08AC  lfs f30, 0x8ac(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2220 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8282B5D0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8282B5D4: 4BD77BDD  bl 0x825a31b0
	ctx.lr = 0x8282B5D8;
	sub_825A31B0(ctx, base);
	// 8282B5D8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B5DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B5E0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B5E4: 4BD7618D  bl 0x825a1770
	ctx.lr = 0x8282B5E8;
	sub_825A1770(ctx, base);
	// 8282B5E8: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8282B5EC: 485C7E3D  bl 0x82df3428
	ctx.lr = 0x8282B5F0;
	sub_82DF3428(ctx, base);
	// 8282B5F0: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8282B5F4: 4BA9D6C5  bl 0x822c8cb8
	ctx.lr = 0x8282B5F8;
	sub_822C8CB8(ctx, base);
	// 8282B5F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B5FC: 485C7E2D  bl 0x82df3428
	ctx.lr = 0x8282B600;
	sub_82DF3428(ctx, base);
	// 8282B600: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B608: 388BB6A8  addi r4, r11, -0x4958
	ctx.r[4].s64 = ctx.r[11].s64 + -18776;
	// 8282B60C: 485C83FD  bl 0x82df3a08
	ctx.lr = 0x8282B610;
	sub_82DF3A08(ctx, base);
	// 8282B610: 38990098  addi r4, r25, 0x98
	ctx.r[4].s64 = ctx.r[25].s64 + 152;
	// 8282B614: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 8282B618: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B61C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8282B620: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B624: 4BD77B8D  bl 0x825a31b0
	ctx.lr = 0x8282B628;
	sub_825A31B0(ctx, base);
	// 8282B628: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B62C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B630: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B634: 4BD7613D  bl 0x825a1770
	ctx.lr = 0x8282B638;
	sub_825A1770(ctx, base);
	// 8282B638: 38610168  addi r3, r1, 0x168
	ctx.r[3].s64 = ctx.r[1].s64 + 360;
	// 8282B63C: 485C7DED  bl 0x82df3428
	ctx.lr = 0x8282B640;
	sub_82DF3428(ctx, base);
	// 8282B640: 38610148  addi r3, r1, 0x148
	ctx.r[3].s64 = ctx.r[1].s64 + 328;
	// 8282B644: 4BA9D675  bl 0x822c8cb8
	ctx.lr = 0x8282B648;
	sub_822C8CB8(ctx, base);
	// 8282B648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B64C: 485C7DDD  bl 0x82df3428
	ctx.lr = 0x8282B650;
	sub_82DF3428(ctx, base);
	// 8282B650: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B654: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B658: 388BB69C  addi r4, r11, -0x4964
	ctx.r[4].s64 = ctx.r[11].s64 + -18788;
	// 8282B65C: 485C83AD  bl 0x82df3a08
	ctx.lr = 0x8282B660;
	sub_82DF3A08(ctx, base);
	// 8282B660: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8282B664: 38C003E8  li r6, 0x3e8
	ctx.r[6].s64 = 1000;
	// 8282B668: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8282B66C: 389900A0  addi r4, r25, 0xa0
	ctx.r[4].s64 = ctx.r[25].s64 + 160;
	// 8282B670: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8282B674: 4BD77D35  bl 0x825a33a8
	ctx.lr = 0x8282B678;
	sub_825A33A8(ctx, base);
	// 8282B678: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B67C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B680: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B684: 4BD765DD  bl 0x825a1c60
	ctx.lr = 0x8282B688;
	sub_825A1C60(ctx, base);
	// 8282B688: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8282B68C: 485C7D9D  bl 0x82df3428
	ctx.lr = 0x8282B690;
	sub_82DF3428(ctx, base);
	// 8282B690: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8282B694: 4BA9D625  bl 0x822c8cb8
	ctx.lr = 0x8282B698;
	sub_822C8CB8(ctx, base);
	// 8282B698: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B69C: 485C7D8D  bl 0x82df3428
	ctx.lr = 0x8282B6A0;
	sub_82DF3428(ctx, base);
	// 8282B6A0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B6A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B6A8: 388BB68C  addi r4, r11, -0x4974
	ctx.r[4].s64 = ctx.r[11].s64 + -18804;
	// 8282B6AC: 485C835D  bl 0x82df3a08
	ctx.lr = 0x8282B6B0;
	sub_82DF3A08(ctx, base);
	// 8282B6B0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8282B6B4: 38C003E8  li r6, 0x3e8
	ctx.r[6].s64 = 1000;
	// 8282B6B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8282B6BC: 389900A4  addi r4, r25, 0xa4
	ctx.r[4].s64 = ctx.r[25].s64 + 164;
	// 8282B6C0: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8282B6C4: 4BD77CE5  bl 0x825a33a8
	ctx.lr = 0x8282B6C8;
	sub_825A33A8(ctx, base);
	// 8282B6C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B6CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B6D0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B6D4: 4BD7658D  bl 0x825a1c60
	ctx.lr = 0x8282B6D8;
	sub_825A1C60(ctx, base);
	// 8282B6D8: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 8282B6DC: 485C7D4D  bl 0x82df3428
	ctx.lr = 0x8282B6E0;
	sub_82DF3428(ctx, base);
	// 8282B6E0: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 8282B6E4: 4BA9D5D5  bl 0x822c8cb8
	ctx.lr = 0x8282B6E8;
	sub_822C8CB8(ctx, base);
	// 8282B6E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B6EC: 485C7D3D  bl 0x82df3428
	ctx.lr = 0x8282B6F0;
	sub_82DF3428(ctx, base);
	// 8282B6F0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B6F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B6F8: 388BB67C  addi r4, r11, -0x4984
	ctx.r[4].s64 = ctx.r[11].s64 + -18820;
	// 8282B6FC: 485C830D  bl 0x82df3a08
	ctx.lr = 0x8282B700;
	sub_82DF3A08(ctx, base);
	// 8282B700: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282B704: 38990088  addi r4, r25, 0x88
	ctx.r[4].s64 = ctx.r[25].s64 + 136;
	// 8282B708: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B70C: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 8282B710: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B714: C3CBD200  lfs f30, -0x2e00(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11776 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8282B718: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8282B71C: 4BD77A95  bl 0x825a31b0
	ctx.lr = 0x8282B720;
	sub_825A31B0(ctx, base);
	// 8282B720: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B724: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B728: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B72C: 4BD76045  bl 0x825a1770
	ctx.lr = 0x8282B730;
	sub_825A1770(ctx, base);
	// 8282B730: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 8282B734: 485C7CF5  bl 0x82df3428
	ctx.lr = 0x8282B738;
	sub_82DF3428(ctx, base);
	// 8282B738: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 8282B73C: 4BA9D57D  bl 0x822c8cb8
	ctx.lr = 0x8282B740;
	sub_822C8CB8(ctx, base);
	// 8282B740: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B744: 485C7CE5  bl 0x82df3428
	ctx.lr = 0x8282B748;
	sub_82DF3428(ctx, base);
	// 8282B748: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282B74C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B750: 388BB66C  addi r4, r11, -0x4994
	ctx.r[4].s64 = ctx.r[11].s64 + -18836;
	// 8282B754: 485C82B5  bl 0x82df3a08
	ctx.lr = 0x8282B758;
	sub_82DF3A08(ctx, base);
	// 8282B758: 3899008C  addi r4, r25, 0x8c
	ctx.r[4].s64 = ctx.r[25].s64 + 140;
	// 8282B75C: 386101F0  addi r3, r1, 0x1f0
	ctx.r[3].s64 = ctx.r[1].s64 + 496;
	// 8282B760: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8282B764: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8282B768: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282B76C: 4BD77A45  bl 0x825a31b0
	ctx.lr = 0x8282B770;
	sub_825A31B0(ctx, base);
	// 8282B770: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282B774: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282B778: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8282B77C: 4BD75FF5  bl 0x825a1770
	ctx.lr = 0x8282B780;
	sub_825A1770(ctx, base);
	// 8282B780: 38610228  addi r3, r1, 0x228
	ctx.r[3].s64 = ctx.r[1].s64 + 552;
	// 8282B784: 485C7CA5  bl 0x82df3428
	ctx.lr = 0x8282B788;
	sub_82DF3428(ctx, base);
	// 8282B788: 38610208  addi r3, r1, 0x208
	ctx.r[3].s64 = ctx.r[1].s64 + 520;
	// 8282B78C: 4BA9D52D  bl 0x822c8cb8
	ctx.lr = 0x8282B790;
	sub_822C8CB8(ctx, base);
	// 8282B790: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282B794: 485C7C95  bl 0x82df3428
	ctx.lr = 0x8282B798;
	sub_82DF3428(ctx, base);
	// 8282B798: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8282B79C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8282B7A0: 4BFED1C9  bl 0x82818968
	ctx.lr = 0x8282B7A4;
	sub_82818968(ctx, base);
	// 8282B7A4: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 8282B7A8: CBC1FFA8  lfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8282B7AC: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8282B7B0: 4897C9F8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282B7B8 size=1632
    let mut pc: u32 = 0x8282B7B8;
    'dispatch: loop {
        match pc {
            0x8282B7B8 => {
    //   block [0x8282B7B8..0x8282BE18)
	// 8282B7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282B7BC: 4897C9AD  bl 0x831a8168
	ctx.lr = 0x8282B7C0;
	sub_831A8130(ctx, base);
	// 8282B7C0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8282B7C4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8282B7C8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282BE18 size=144
    let mut pc: u32 = 0x8282BE18;
    'dispatch: loop {
        match pc {
            0x8282BE18 => {
    //   block [0x8282BE18..0x8282BEA8)
	// 8282BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282BE20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282BE24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282BE28: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282BE2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8282BE30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282BE34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282BE38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282BE3C: 388B4A98  addi r4, r11, 0x4a98
	ctx.r[4].s64 = ctx.r[11].s64 + 19096;
	// 8282BE40: 485C7BC9  bl 0x82df3a08
	ctx.lr = 0x8282BE44;
	sub_82DF3A08(ctx, base);
	// 8282BE44: 389F00D8  addi r4, r31, 0xd8
	ctx.r[4].s64 = ctx.r[31].s64 + 216;
	// 8282BE48: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8282BE4C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8282BE50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8282BE54: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282BE58: 4BD77551  bl 0x825a33a8
	ctx.lr = 0x8282BE5C;
	sub_825A33A8(ctx, base);
	// 8282BE5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282BE60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282BE64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282BE68: 4BD76071  bl 0x825a1ed8
	ctx.lr = 0x8282BE6C;
	sub_825A1ED8(ctx, base);
	// 8282BE6C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8282BE70: 485C75B9  bl 0x82df3428
	ctx.lr = 0x8282BE74;
	sub_82DF3428(ctx, base);
	// 8282BE74: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8282BE78: 4BA9CE41  bl 0x822c8cb8
	ctx.lr = 0x8282BE7C;
	sub_822C8CB8(ctx, base);
	// 8282BE7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282BE80: 485C75A9  bl 0x82df3428
	ctx.lr = 0x8282BE84;
	sub_82DF3428(ctx, base);
	// 8282BE84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282BE88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282BE8C: 4BFFF425  bl 0x8282b2b0
	ctx.lr = 0x8282BE90;
	sub_8282B2B0(ctx, base);
	// 8282BE90: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8282BE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282BE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282BE9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282BEA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282BEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282BEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282BEA8 size=276
    let mut pc: u32 = 0x8282BEA8;
    'dispatch: loop {
        match pc {
            0x8282BEA8 => {
    //   block [0x8282BEA8..0x8282BFBC)
	// 8282BEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282BEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282BEB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282BEB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282BEB8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8282BEBC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282BEC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282BEC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282BEC8: C3FF029C  lfs f31, 0x29c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282BECC: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8282BED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8282BED4: 4E800421  bctrl
	ctx.lr = 0x8282BED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8282BED8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282BEDC: 4BFC1E65  bl 0x827edd40
	ctx.lr = 0x8282BEE0;
	sub_827EDD40(ctx, base);
	// 8282BEE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282BEE4: 418200B4  beq 0x8282bf98
	if ctx.cr[0].eq {
	pc = 0x8282BF98; continue 'dispatch;
	}
	// 8282BEE8: 3BDF02F0  addi r30, r31, 0x2f0
	ctx.r[30].s64 = ctx.r[31].s64 + 752;
	// 8282BEEC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8282BEF0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8282BEF4: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282BFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282BFC0 size=152
    let mut pc: u32 = 0x8282BFC0;
    'dispatch: loop {
        match pc {
            0x8282BFC0 => {
    //   block [0x8282BFC0..0x8282C058)
	// 8282BFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282BFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282BFC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282BFCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282BFD0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282BFD4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 8282BFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282BFDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282BFE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282BFE4: 388B4640  addi r4, r11, 0x4640
	ctx.r[4].s64 = ctx.r[11].s64 + 17984;
	// 8282BFE8: 485C7A21  bl 0x82df3a08
	ctx.lr = 0x8282BFEC;
	sub_82DF3A08(ctx, base);
	// 8282BFEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282BFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8282BFF4: 389F00D8  addi r4, r31, 0xd8
	ctx.r[4].s64 = ctx.r[31].s64 + 216;
	// 8282BFF8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282BFFC: C06B964C  lfs f3, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8282C000: C04A6218  lfs f2, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8282C004: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 8282C008: 4BD771A9  bl 0x825a31b0
	ctx.lr = 0x8282C00C;
	sub_825A31B0(ctx, base);
	// 8282C00C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8282C010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C014: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8282C018: 4BD75759  bl 0x825a1770
	ctx.lr = 0x8282C01C;
	sub_825A1770(ctx, base);
	// 8282C01C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8282C020: 485C7409  bl 0x82df3428
	ctx.lr = 0x8282C024;
	sub_82DF3428(ctx, base);
	// 8282C024: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8282C028: 4BA9CC91  bl 0x822c8cb8
	ctx.lr = 0x8282C02C;
	sub_822C8CB8(ctx, base);
	// 8282C02C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282C030: 485C73F9  bl 0x82df3428
	ctx.lr = 0x8282C034;
	sub_82DF3428(ctx, base);
	// 8282C034: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282C038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C03C: 4BFFF275  bl 0x8282b2b0
	ctx.lr = 0x8282C040;
	sub_8282B2B0(ctx, base);
	// 8282C040: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8282C044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282C048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282C04C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282C050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282C054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282C058 size=260
    let mut pc: u32 = 0x8282C058;
    'dispatch: loop {
        match pc {
            0x8282C058 => {
    //   block [0x8282C058..0x8282C15C)
	// 8282C058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C05C: 4897C111  bl 0x831a816c
	ctx.lr = 0x8282C060;
	sub_831A8130(ctx, base);
	// 8282C060: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8282C064: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C06C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282C070: 4BFFE2E1  bl 0x8282a350
	ctx.lr = 0x8282C074;
	sub_8282A350(ctx, base);
	// 8282C074: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282C078: C01F02E0  lfs f0, 0x2e0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(736 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282C07C: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282C080: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8282C084: 409800C0  bge cr6, 0x8282c144
	if !ctx.cr[6].lt {
	pc = 0x8282C144; continue 'dispatch;
	}
	// 8282C088: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8282C08C: 4BDABFC5  bl 0x825d8050
	ctx.lr = 0x8282C090;
	sub_825D8050(ctx, base);
	// 8282C090: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282C094: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8282C098: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8282C09C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282C0A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8282C0A4: 419A0024  beq cr6, 0x8282c0c8
	if ctx.cr[6].eq {
	pc = 0x8282C0C8; continue 'dispatch;
	}
	// 8282C0A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8282C0AC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282C0B0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C0B4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282C0B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282C0BC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C0C0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C0C4: 4082FFE8  bne 0x8282c0ac
	if !ctx.cr[0].eq {
	pc = 0x8282C0AC; continue 'dispatch;
	}
	// 8282C0C8: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 8282C0CC: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8282C0D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C0D4: 487DCEE5  bl 0x83008fb8
	ctx.lr = 0x8282C0D8;
	sub_83008FB8(ctx, base);
	// 8282C0D8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282C0DC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8282C0E0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282C0E4: 388BB518  addi r4, r11, -0x4ae8
	ctx.r[4].s64 = ctx.r[11].s64 + -19176;
	// 8282C0E8: 38A00369  li r5, 0x369
	ctx.r[5].s64 = 873;
	// 8282C0EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C0F0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8282C0F4: 4862C94D  bl 0x82e58a40
	ctx.lr = 0x8282C0F8;
	sub_82E58A40(ctx, base);
	// 8282C0F8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282C0FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C100: 419A0008  beq cr6, 0x8282c108
	if ctx.cr[6].eq {
	pc = 0x8282C108; continue 'dispatch;
	}
	// 8282C104: 4BA9478D  bl 0x822c0890
	ctx.lr = 0x8282C108;
	sub_822C0890(ctx, base);
	// 8282C108: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8282C10C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C110: 419A0008  beq cr6, 0x8282c118
	if ctx.cr[6].eq {
	pc = 0x8282C118; continue 'dispatch;
	}
	// 8282C114: 4BA9477D  bl 0x822c0890
	ctx.lr = 0x8282C118;
	sub_822C0890(ctx, base);
	// 8282C118: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C11C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282C120: 4827FBC9  bl 0x82aabce8
	ctx.lr = 0x8282C124;
	sub_82AABCE8(ctx, base);
	// 8282C124: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282C128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C12C: 4BFEBDED  bl 0x82817f18
	ctx.lr = 0x8282C130;
	sub_82817F18(ctx, base);
	// 8282C130: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8282C134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C138: 419A0018  beq cr6, 0x8282c150
	if ctx.cr[6].eq {
	pc = 0x8282C150; continue 'dispatch;
	}
	// 8282C13C: 4BA94755  bl 0x822c0890
	ctx.lr = 0x8282C140;
	sub_822C0890(ctx, base);
	// 8282C140: 48000010  b 0x8282c150
	pc = 0x8282C150; continue 'dispatch;
	// 8282C144: C1BE0000  lfs f13, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8282C148: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8282C14C: D01F02E0  stfs f0, 0x2e0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), tmp.u32 ) };
	// 8282C150: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8282C154: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8282C158: 4897C064  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282C160 size=4
    let mut pc: u32 = 0x8282C160;
    'dispatch: loop {
        match pc {
            0x8282C160 => {
    //   block [0x8282C160..0x8282C164)
	// 8282C160: 4BFFF658  b 0x8282b7b8
	sub_8282B7B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282C168 size=76
    let mut pc: u32 = 0x8282C168;
    'dispatch: loop {
        match pc {
            0x8282C168 => {
    //   block [0x8282C168..0x8282C1B4)
	// 8282C168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282C170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282C174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282C178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C17C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C180: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8282C184: 4BFFED05  bl 0x8282ae88
	ctx.lr = 0x8282C188;
	sub_8282AE88(ctx, base);
	// 8282C188: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282C18C: 4182000C  beq 0x8282c198
	if ctx.cr[0].eq {
	pc = 0x8282C198; continue 'dispatch;
	}
	// 8282C190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C194: 485C6245  bl 0x82df23d8
	ctx.lr = 0x8282C198;
	sub_82DF23D8(ctx, base);
	// 8282C198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C19C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282C1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282C1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282C1A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282C1AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282C1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282C1B8 size=356
    let mut pc: u32 = 0x8282C1B8;
    'dispatch: loop {
        match pc {
            0x8282C1B8 => {
    //   block [0x8282C1B8..0x8282C31C)
	// 8282C1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C1BC: 4897BFB1  bl 0x831a816c
	ctx.lr = 0x8282C1C0;
	sub_831A8130(ctx, base);
	// 8282C1C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C1C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C1C8: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8282C1CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C1D0: 419A00B0  beq cr6, 0x8282c280
	if ctx.cr[6].eq {
	pc = 0x8282C280; continue 'dispatch;
	}
	// 8282C1D4: 389F00F0  addi r4, r31, 0xf0
	ctx.r[4].s64 = ctx.r[31].s64 + 240;
	// 8282C1D8: 48295DB9  bl 0x82ac1f90
	ctx.lr = 0x8282C1DC;
	sub_82AC1F90(ctx, base);
	// 8282C1DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C1E0: 83DF00E8  lwz r30, 0xe8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8282C1E4: 4BCE5975  bl 0x82511b58
	ctx.lr = 0x8282C1E8;
	sub_82511B58(ctx, base);
	// 8282C1E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282C1EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C1F0: 4827F2E9  bl 0x82aab4d8
	ctx.lr = 0x8282C1F4;
	sub_82AAB4D8(ctx, base);
	// 8282C1F4: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8282C1F8: 815F00E8  lwz r10, 0xe8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8282C1FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282C200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8282C204: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8282C208: 419A0024  beq cr6, 0x8282c22c
	if ctx.cr[6].eq {
	pc = 0x8282C22C; continue 'dispatch;
	}
	// 8282C20C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8282C210: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282C214: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C218: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282C21C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282C220: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C224: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C228: 4082FFE8  bne 0x8282c210
	if !ctx.cr[0].eq {
	pc = 0x8282C210; continue 'dispatch;
	}
	// 8282C22C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C230: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282C234: 4BCE3295  bl 0x8250f4c8
	ctx.lr = 0x8282C238;
	sub_8250F4C8(ctx, base);
	// 8282C238: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282C23C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282C240: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 8282C244: 409A0008  bne cr6, 0x8282c24c
	if !ctx.cr[6].eq {
	pc = 0x8282C24C; continue 'dispatch;
	}
	// 8282C248: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8282C24C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C250: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8282C254: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 8282C258: 4BCE32C1  bl 0x8250f518
	ctx.lr = 0x8282C25C;
	sub_8250F518(ctx, base);
	// 8282C25C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282C260: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C264: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8282C268: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C26C: 4BCE129D  bl 0x8250d508
	ctx.lr = 0x8282C270;
	sub_8250D508(ctx, base);
	// 8282C270: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282C274: 485C5A1D  bl 0x82df1c90
	ctx.lr = 0x8282C278;
	sub_82DF1C90(ctx, base);
	// 8282C278: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8282C27C: 485C5A15  bl 0x82df1c90
	ctx.lr = 0x8282C280;
	sub_82DF1C90(ctx, base);
	// 8282C280: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8282C284: 4BDABDCD  bl 0x825d8050
	ctx.lr = 0x8282C288;
	sub_825D8050(ctx, base);
	// 8282C288: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282C28C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8282C290: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8282C294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282C298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8282C29C: 419A0024  beq cr6, 0x8282c2c0
	if ctx.cr[6].eq {
	pc = 0x8282C2C0; continue 'dispatch;
	}
	// 8282C2A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8282C2A4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282C2A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C2AC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282C2B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282C2B4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C2B8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C2BC: 4082FFE8  bne 0x8282c2a4
	if !ctx.cr[0].eq {
	pc = 0x8282C2A4; continue 'dispatch;
	}
	// 8282C2C0: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8282C2C4: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8282C2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C2CC: 487DCCED  bl 0x83008fb8
	ctx.lr = 0x8282C2D0;
	sub_83008FB8(ctx, base);
	// 8282C2D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282C2D4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282C2D8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8282C2DC: 388AB518  addi r4, r10, -0x4ae8
	ctx.r[4].s64 = ctx.r[10].s64 + -19176;
	// 8282C2E0: 38A003EC  li r5, 0x3ec
	ctx.r[5].s64 = 1004;
	// 8282C2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C2E8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282C2EC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8282C2F0: 4862C751  bl 0x82e58a40
	ctx.lr = 0x8282C2F4;
	sub_82E58A40(ctx, base);
	// 8282C2F4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282C2F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C2FC: 419A0008  beq cr6, 0x8282c304
	if ctx.cr[6].eq {
	pc = 0x8282C304; continue 'dispatch;
	}
	// 8282C300: 4BA94591  bl 0x822c0890
	ctx.lr = 0x8282C304;
	sub_822C0890(ctx, base);
	// 8282C304: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8282C308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C30C: 419A0008  beq cr6, 0x8282c314
	if ctx.cr[6].eq {
	pc = 0x8282C314; continue 'dispatch;
	}
	// 8282C310: 4BA94581  bl 0x822c0890
	ctx.lr = 0x8282C314;
	sub_822C0890(ctx, base);
	// 8282C314: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8282C318: 4897BEA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282C320 size=108
    let mut pc: u32 = 0x8282C320;
    'dispatch: loop {
        match pc {
            0x8282C320 => {
    //   block [0x8282C320..0x8282C38C)
	// 8282C320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C324: 4897BE49  bl 0x831a816c
	ctx.lr = 0x8282C328;
	sub_831A8130(ctx, base);
	// 8282C328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C32C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8282C330: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282C334: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282C338: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282C33C: 41820038  beq 0x8282c374
	if ctx.cr[0].eq {
	pc = 0x8282C374; continue 'dispatch;
	}
	// 8282C340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C344: 4897D645  bl 0x831a9988
	ctx.lr = 0x8282C348;
	sub_831A9988(ctx, base);
	// 8282C348: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8282C34C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282C350: 386B3644  addi r3, r11, 0x3644
	ctx.r[3].s64 = ctx.r[11].s64 + 13892;
	// 8282C354: 4897BDA5  bl 0x831a80f8
	ctx.lr = 0x8282C358;
	sub_831A80F8(ctx, base);
	// 8282C358: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282C35C: 41820018  beq 0x8282c374
	if ctx.cr[0].eq {
	pc = 0x8282C374; continue 'dispatch;
	}
	// 8282C360: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C364: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 8282C368: 4BFFFE51  bl 0x8282c1b8
	ctx.lr = 0x8282C36C;
	sub_8282C1B8(ctx, base);
	// 8282C36C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8282C370: 48000014  b 0x8282c384
	pc = 0x8282C384; continue 'dispatch;
	// 8282C374: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8282C378: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C37C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282C380: 4BCE6299  bl 0x82512618
	ctx.lr = 0x8282C384;
	sub_82512618(ctx, base);
	// 8282C384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282C388: 4897BE34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282C390 size=476
    let mut pc: u32 = 0x8282C390;
    'dispatch: loop {
        match pc {
            0x8282C390 => {
    //   block [0x8282C390..0x8282C56C)
	// 8282C390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C394: 4897BDD1  bl 0x831a8164
	ctx.lr = 0x8282C398;
	sub_831A8130(ctx, base);
	// 8282C398: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8282C39C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C3A0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282C3A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8282C3A8: 3BCBB518  addi r30, r11, -0x4ae8
	ctx.r[30].s64 = ctx.r[11].s64 + -19176;
	// 8282C3AC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8282C3B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C3B4: 38A00278  li r5, 0x278
	ctx.r[5].s64 = 632;
	// 8282C3B8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8282C3BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282C3C0: 38600300  li r3, 0x300
	ctx.r[3].s64 = 768;
	// 8282C3C4: 485C6025  bl 0x82df23e8
	ctx.lr = 0x8282C3C8;
	sub_82DF23E8(ctx, base);
	// 8282C3C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282C3CC: 41820010  beq 0x8282c3dc
	if ctx.cr[0].eq {
	pc = 0x8282C3DC; continue 'dispatch;
	}
	// 8282C3D0: 4BFFE819  bl 0x8282abe8
	ctx.lr = 0x8282C3D4;
	sub_8282ABE8(ctx, base);
	// 8282C3D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C3D8: 48000008  b 0x8282c3e0
	pc = 0x8282C3E0; continue 'dispatch;
	// 8282C3DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282C3E0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8282C3E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C3E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282C3EC: 4BFFD87D  bl 0x82829c68
	ctx.lr = 0x8282C3F0;
	sub_82829C68(ctx, base);
	// 8282C3F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282C3F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C3F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282C3FC: 4BA93C05  bl 0x822c0000
	ctx.lr = 0x8282C400;
	sub_822C0000(ctx, base);
	// 8282C400: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282C404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C408: 38A00279  li r5, 0x279
	ctx.r[5].s64 = 633;
	// 8282C40C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8282C410: 4BA93FC9  bl 0x822c03d8
	ctx.lr = 0x8282C414;
	sub_822C03D8(ctx, base);
	// 8282C414: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282C418: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282C41C: 418200E0  beq 0x8282c4fc
	if ctx.cr[0].eq {
	pc = 0x8282C4FC; continue 'dispatch;
	}
	// 8282C420: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282C424: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8282C428: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282C42C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8282C430: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8282C434: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282C438: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 8282C43C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282C440: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8282C444: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8282C448: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8282C44C: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8282C450: 409A0008  bne cr6, 0x8282c458
	if !ctx.cr[6].eq {
	pc = 0x8282C458; continue 'dispatch;
	}
	// 8282C454: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8282C458: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282C45C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C460: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8282C464: 419A0024  beq cr6, 0x8282c488
	if ctx.cr[6].eq {
	pc = 0x8282C488; continue 'dispatch;
	}
	// 8282C468: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8282C46C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8282C470: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C474: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8282C478: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8282C47C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C480: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C484: 4082FFE8  bne 0x8282c46c
	if !ctx.cr[0].eq {
	pc = 0x8282C46C; continue 'dispatch;
	}
	// 8282C488: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8282C48C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C490: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 8282C494: 419A0024  beq cr6, 0x8282c4b8
	if ctx.cr[6].eq {
	pc = 0x8282C4B8; continue 'dispatch;
	}
	// 8282C498: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8282C49C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282C4A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C4A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282C4A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282C4AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C4B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C4B4: 4082FFE8  bne 0x8282c49c
	if !ctx.cr[0].eq {
	pc = 0x8282C49C; continue 'dispatch;
	}
	// 8282C4B8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8282C4BC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8282C4C0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8282C4C4: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282C4C8: 48650779  bl 0x82e7cc40
	ctx.lr = 0x8282C4CC;
	sub_82E7CC40(ctx, base);
	// 8282C4CC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8282C4D0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282C4D4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8282C4D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8282C4DC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8282C4E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8282C4E4: 4BF6CA4D  bl 0x82798f30
	ctx.lr = 0x8282C4E8;
	sub_82798F30(ctx, base);
	// 8282C4E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282C4EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C4F0: 4BF6F179  bl 0x8279b668
	ctx.lr = 0x8282C4F4;
	sub_8279B668(ctx, base);
	// 8282C4F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C4F8: 48000008  b 0x8282c500
	pc = 0x8282C500; continue 'dispatch;
	// 8282C4FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282C500: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8282C504: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 8282C508: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C50C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C510: 4BD6B741  bl 0x82597c50
	ctx.lr = 0x8282C514;
	sub_82597C50(ctx, base);
	// 8282C514: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282C518: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C51C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C520: 4BA93AE1  bl 0x822c0000
	ctx.lr = 0x8282C524;
	sub_822C0000(ctx, base);
	// 8282C524: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282C528: 41820024  beq 0x8282c54c
	if ctx.cr[0].eq {
	pc = 0x8282C54C; continue 'dispatch;
	}
	// 8282C52C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8282C530: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C534: 419A0008  beq cr6, 0x8282c53c
	if ctx.cr[6].eq {
	pc = 0x8282C53C; continue 'dispatch;
	}
	// 8282C538: 4BA94359  bl 0x822c0890
	ctx.lr = 0x8282C53C;
	sub_822C0890(ctx, base);
	// 8282C53C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8282C540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C544: 419A0008  beq cr6, 0x8282c54c
	if ctx.cr[6].eq {
	pc = 0x8282C54C; continue 'dispatch;
	}
	// 8282C548: 4BA94349  bl 0x822c0890
	ctx.lr = 0x8282C54C;
	sub_822C0890(ctx, base);
	// 8282C54C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C550: 419A000C  beq cr6, 0x8282c55c
	if ctx.cr[6].eq {
	pc = 0x8282C55C; continue 'dispatch;
	}
	// 8282C554: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282C558: 4BA94339  bl 0x822c0890
	ctx.lr = 0x8282C55C;
	sub_822C0890(ctx, base);
	// 8282C55C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8282C560: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8282C564: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8282C568: 4897BC4C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282C570 size=476
    let mut pc: u32 = 0x8282C570;
    'dispatch: loop {
        match pc {
            0x8282C570 => {
    //   block [0x8282C570..0x8282C74C)
	// 8282C570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C574: 4897BBF1  bl 0x831a8164
	ctx.lr = 0x8282C578;
	sub_831A8130(ctx, base);
	// 8282C578: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8282C57C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C580: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282C584: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8282C588: 3BCBB518  addi r30, r11, -0x4ae8
	ctx.r[30].s64 = ctx.r[11].s64 + -19176;
	// 8282C58C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8282C590: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C594: 38A00310  li r5, 0x310
	ctx.r[5].s64 = 784;
	// 8282C598: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8282C59C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282C5A0: 386002F0  li r3, 0x2f0
	ctx.r[3].s64 = 752;
	// 8282C5A4: 485C5E45  bl 0x82df23e8
	ctx.lr = 0x8282C5A8;
	sub_82DF23E8(ctx, base);
	// 8282C5A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282C5AC: 41820010  beq 0x8282c5bc
	if ctx.cr[0].eq {
	pc = 0x8282C5BC; continue 'dispatch;
	}
	// 8282C5B0: 4BFFE719  bl 0x8282acc8
	ctx.lr = 0x8282C5B4;
	sub_8282ACC8(ctx, base);
	// 8282C5B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C5B8: 48000008  b 0x8282c5c0
	pc = 0x8282C5C0; continue 'dispatch;
	// 8282C5BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282C5C0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8282C5C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C5C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282C5CC: 4BFFD765  bl 0x82829d30
	ctx.lr = 0x8282C5D0;
	sub_82829D30(ctx, base);
	// 8282C5D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282C5D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C5D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282C5DC: 4BA93A25  bl 0x822c0000
	ctx.lr = 0x8282C5E0;
	sub_822C0000(ctx, base);
	// 8282C5E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282C5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C5E8: 38A00311  li r5, 0x311
	ctx.r[5].s64 = 785;
	// 8282C5EC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8282C5F0: 4BA93DE9  bl 0x822c03d8
	ctx.lr = 0x8282C5F4;
	sub_822C03D8(ctx, base);
	// 8282C5F4: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282C5F8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282C5FC: 418200E0  beq 0x8282c6dc
	if ctx.cr[0].eq {
	pc = 0x8282C6DC; continue 'dispatch;
	}
	// 8282C600: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282C604: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8282C608: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282C60C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8282C610: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8282C614: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282C618: 396A0208  addi r11, r10, 0x208
	ctx.r[11].s64 = ctx.r[10].s64 + 520;
	// 8282C61C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282C620: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8282C624: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8282C628: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8282C62C: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8282C630: 409A0008  bne cr6, 0x8282c638
	if !ctx.cr[6].eq {
	pc = 0x8282C638; continue 'dispatch;
	}
	// 8282C634: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8282C638: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282C63C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C640: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8282C644: 419A0024  beq cr6, 0x8282c668
	if ctx.cr[6].eq {
	pc = 0x8282C668; continue 'dispatch;
	}
	// 8282C648: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8282C64C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8282C650: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C654: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8282C658: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8282C65C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C660: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C664: 4082FFE8  bne 0x8282c64c
	if !ctx.cr[0].eq {
	pc = 0x8282C64C; continue 'dispatch;
	}
	// 8282C668: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8282C66C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C670: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 8282C674: 419A0024  beq cr6, 0x8282c698
	if ctx.cr[6].eq {
	pc = 0x8282C698; continue 'dispatch;
	}
	// 8282C678: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8282C67C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282C680: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C684: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282C688: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282C68C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C690: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C694: 4082FFE8  bne 0x8282c67c
	if !ctx.cr[0].eq {
	pc = 0x8282C67C; continue 'dispatch;
	}
	// 8282C698: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8282C69C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8282C6A0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8282C6A4: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282C6A8: 48650599  bl 0x82e7cc40
	ctx.lr = 0x8282C6AC;
	sub_82E7CC40(ctx, base);
	// 8282C6AC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8282C6B0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282C6B4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8282C6B8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8282C6BC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8282C6C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8282C6C4: 4BF6C86D  bl 0x82798f30
	ctx.lr = 0x8282C6C8;
	sub_82798F30(ctx, base);
	// 8282C6C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282C6CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C6D0: 4BF6EF99  bl 0x8279b668
	ctx.lr = 0x8282C6D4;
	sub_8279B668(ctx, base);
	// 8282C6D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C6D8: 48000008  b 0x8282c6e0
	pc = 0x8282C6E0; continue 'dispatch;
	// 8282C6DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282C6E0: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8282C6E4: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 8282C6E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C6EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C6F0: 4BD6B561  bl 0x82597c50
	ctx.lr = 0x8282C6F4;
	sub_82597C50(ctx, base);
	// 8282C6F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282C6F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C6FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C700: 4BA93901  bl 0x822c0000
	ctx.lr = 0x8282C704;
	sub_822C0000(ctx, base);
	// 8282C704: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282C708: 41820024  beq 0x8282c72c
	if ctx.cr[0].eq {
	pc = 0x8282C72C; continue 'dispatch;
	}
	// 8282C70C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8282C710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C714: 419A0008  beq cr6, 0x8282c71c
	if ctx.cr[6].eq {
	pc = 0x8282C71C; continue 'dispatch;
	}
	// 8282C718: 4BA94179  bl 0x822c0890
	ctx.lr = 0x8282C71C;
	sub_822C0890(ctx, base);
	// 8282C71C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8282C720: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C724: 419A0008  beq cr6, 0x8282c72c
	if ctx.cr[6].eq {
	pc = 0x8282C72C; continue 'dispatch;
	}
	// 8282C728: 4BA94169  bl 0x822c0890
	ctx.lr = 0x8282C72C;
	sub_822C0890(ctx, base);
	// 8282C72C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C730: 419A000C  beq cr6, 0x8282c73c
	if ctx.cr[6].eq {
	pc = 0x8282C73C; continue 'dispatch;
	}
	// 8282C734: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282C738: 4BA94159  bl 0x822c0890
	ctx.lr = 0x8282C73C;
	sub_822C0890(ctx, base);
	// 8282C73C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8282C740: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8282C744: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8282C748: 4897BA6C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282C750 size=476
    let mut pc: u32 = 0x8282C750;
    'dispatch: loop {
        match pc {
            0x8282C750 => {
    //   block [0x8282C750..0x8282C92C)
	// 8282C750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C754: 4897BA11  bl 0x831a8164
	ctx.lr = 0x8282C758;
	sub_831A8130(ctx, base);
	// 8282C758: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8282C75C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C760: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282C764: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8282C768: 3BCBB518  addi r30, r11, -0x4ae8
	ctx.r[30].s64 = ctx.r[11].s64 + -19176;
	// 8282C76C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8282C770: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C774: 38A00390  li r5, 0x390
	ctx.r[5].s64 = 912;
	// 8282C778: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8282C77C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282C780: 38600110  li r3, 0x110
	ctx.r[3].s64 = 272;
	// 8282C784: 485C5C65  bl 0x82df23e8
	ctx.lr = 0x8282C788;
	sub_82DF23E8(ctx, base);
	// 8282C788: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8282C78C: 41820010  beq 0x8282c79c
	if ctx.cr[0].eq {
	pc = 0x8282C79C; continue 'dispatch;
	}
	// 8282C790: 4BFFE609  bl 0x8282ad98
	ctx.lr = 0x8282C794;
	sub_8282AD98(ctx, base);
	// 8282C794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C798: 48000008  b 0x8282c7a0
	pc = 0x8282C7A0; continue 'dispatch;
	// 8282C79C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282C7A0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8282C7A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C7A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282C7AC: 4BFFD64D  bl 0x82829df8
	ctx.lr = 0x8282C7B0;
	sub_82829DF8(ctx, base);
	// 8282C7B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282C7B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C7B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8282C7BC: 4BA93845  bl 0x822c0000
	ctx.lr = 0x8282C7C0;
	sub_822C0000(ctx, base);
	// 8282C7C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282C7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C7C8: 38A00391  li r5, 0x391
	ctx.r[5].s64 = 913;
	// 8282C7CC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8282C7D0: 4BA93C09  bl 0x822c03d8
	ctx.lr = 0x8282C7D4;
	sub_822C03D8(ctx, base);
	// 8282C7D4: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282C7D8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8282C7DC: 418200E0  beq 0x8282c8bc
	if ctx.cr[0].eq {
	pc = 0x8282C8BC; continue 'dispatch;
	}
	// 8282C7E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8282C7E4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8282C7E8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8282C7EC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8282C7F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8282C7F4: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282C7F8: 396A00E4  addi r11, r10, 0xe4
	ctx.r[11].s64 = ctx.r[10].s64 + 228;
	// 8282C7FC: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8282C800: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8282C804: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8282C808: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8282C80C: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8282C810: 409A0008  bne cr6, 0x8282c818
	if !ctx.cr[6].eq {
	pc = 0x8282C818; continue 'dispatch;
	}
	// 8282C814: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8282C818: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8282C81C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C820: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8282C824: 419A0024  beq cr6, 0x8282c848
	if ctx.cr[6].eq {
	pc = 0x8282C848; continue 'dispatch;
	}
	// 8282C828: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8282C82C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8282C830: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C834: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8282C838: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8282C83C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C840: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C844: 4082FFE8  bne 0x8282c82c
	if !ctx.cr[0].eq {
	pc = 0x8282C82C; continue 'dispatch;
	}
	// 8282C848: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8282C84C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C850: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 8282C854: 419A0024  beq cr6, 0x8282c878
	if ctx.cr[6].eq {
	pc = 0x8282C878; continue 'dispatch;
	}
	// 8282C858: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8282C85C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8282C860: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C864: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8282C868: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8282C86C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8282C870: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8282C874: 4082FFE8  bne 0x8282c85c
	if !ctx.cr[0].eq {
	pc = 0x8282C85C; continue 'dispatch;
	}
	// 8282C878: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8282C87C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8282C880: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8282C884: C02B7590  lfs f1, 0x7590(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8282C888: 486503B9  bl 0x82e7cc40
	ctx.lr = 0x8282C88C;
	sub_82E7CC40(ctx, base);
	// 8282C88C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8282C890: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282C894: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8282C898: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8282C89C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8282C8A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8282C8A4: 4BF6C68D  bl 0x82798f30
	ctx.lr = 0x8282C8A8;
	sub_82798F30(ctx, base);
	// 8282C8A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282C8AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C8B0: 4BF6EDB9  bl 0x8279b668
	ctx.lr = 0x8282C8B4;
	sub_8279B668(ctx, base);
	// 8282C8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C8B8: 48000008  b 0x8282c8c0
	pc = 0x8282C8C0; continue 'dispatch;
	// 8282C8BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8282C8C0: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8282C8C4: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 8282C8C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C8CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C8D0: 4BD6B381  bl 0x82597c50
	ctx.lr = 0x8282C8D4;
	sub_82597C50(ctx, base);
	// 8282C8D4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8282C8D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282C8DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282C8E0: 4BA93721  bl 0x822c0000
	ctx.lr = 0x8282C8E4;
	sub_822C0000(ctx, base);
	// 8282C8E4: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282C8E8: 41820024  beq 0x8282c90c
	if ctx.cr[0].eq {
	pc = 0x8282C90C; continue 'dispatch;
	}
	// 8282C8EC: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8282C8F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C8F4: 419A0008  beq cr6, 0x8282c8fc
	if ctx.cr[6].eq {
	pc = 0x8282C8FC; continue 'dispatch;
	}
	// 8282C8F8: 4BA93F99  bl 0x822c0890
	ctx.lr = 0x8282C8FC;
	sub_822C0890(ctx, base);
	// 8282C8FC: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8282C900: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282C904: 419A0008  beq cr6, 0x8282c90c
	if ctx.cr[6].eq {
	pc = 0x8282C90C; continue 'dispatch;
	}
	// 8282C908: 4BA93F89  bl 0x822c0890
	ctx.lr = 0x8282C90C;
	sub_822C0890(ctx, base);
	// 8282C90C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282C910: 419A000C  beq cr6, 0x8282c91c
	if ctx.cr[6].eq {
	pc = 0x8282C91C; continue 'dispatch;
	}
	// 8282C914: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282C918: 4BA93F79  bl 0x822c0890
	ctx.lr = 0x8282C91C;
	sub_822C0890(ctx, base);
	// 8282C91C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8282C920: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8282C924: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8282C928: 4897B88C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282C930 size=128
    let mut pc: u32 = 0x8282C930;
    'dispatch: loop {
        match pc {
            0x8282C930 => {
    //   block [0x8282C930..0x8282C9B0)
	// 8282C930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C934: 4897B839  bl 0x831a816c
	ctx.lr = 0x8282C938;
	sub_831A8130(ctx, base);
	// 8282C938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C93C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8282C940: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282C944: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8282C948: 3BEBB194  addi r31, r11, -0x4e6c
	ctx.r[31].s64 = ctx.r[11].s64 + -20076;
	// 8282C94C: 816AB19C  lwz r11, -0x4e64(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20068 as u32) ) } as u64;
	// 8282C950: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8282C954: 40820024  bne 0x8282c978
	if !ctx.cr[0].eq {
	pc = 0x8282C978; continue 'dispatch;
	}
	// 8282C958: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8282C95C: 3D008283  lis r8, -0x7d7d
	ctx.r[8].s64 = -2105344000;
	// 8282C960: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8282C964: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 8282C968: 3908A1C8  addi r8, r8, -0x5e38
	ctx.r[8].s64 = ctx.r[8].s64 + -24120;
	// 8282C96C: 916AB19C  stw r11, -0x4e64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20068 as u32), ctx.r[11].u32 ) };
	// 8282C970: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8282C974: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8282C978: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8282C97C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8282C980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C984: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 8282C988: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8282C98C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8282C990: 4BE27C31  bl 0x826545c0
	ctx.lr = 0x8282C994;
	sub_826545C0(ctx, base);
	// 8282C994: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282C998: 4182000C  beq 0x8282c9a4
	if ctx.cr[0].eq {
	pc = 0x8282C9A4; continue 'dispatch;
	}
	// 8282C99C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8282C9A0: 48000008  b 0x8282c9a8
	pc = 0x8282C9A8; continue 'dispatch;
	// 8282C9A4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8282C9A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8282C9AC: 4897B810  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282C9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282C9B0 size=1004
    let mut pc: u32 = 0x8282C9B0;
    'dispatch: loop {
        match pc {
            0x8282C9B0 => {
    //   block [0x8282C9B0..0x8282CD9C)
	// 8282C9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282C9B4: 4897B7A5  bl 0x831a8158
	ctx.lr = 0x8282C9B8;
	sub_831A8130(ctx, base);
	// 8282C9B8: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8282C9BC: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282C9C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282C9C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8282C9C8: 4BFED871  bl 0x8281a238
	ctx.lr = 0x8282C9CC;
	sub_8281A238(ctx, base);
	// 8282C9CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282C9D0: 4BFBEC69  bl 0x827eb638
	ctx.lr = 0x8282C9D4;
	sub_827EB638(ctx, base);
	// 8282C9D4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282C9D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8282C9DC: 3B6BB518  addi r27, r11, -0x4ae8
	ctx.r[27].s64 = ctx.r[11].s64 + -19176;
	// 8282C9E0: 38A000A4  li r5, 0xa4
	ctx.r[5].s64 = 164;
	// 8282C9E4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8282C9E8: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 8282C9EC: 485C59FD  bl 0x82df23e8
	ctx.lr = 0x8282C9F0;
	sub_82DF23E8(ctx, base);
	// 8282C9F0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8282C9F4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8282C9F8: 41820030  beq 0x8282ca28
	if ctx.cr[0].eq {
	pc = 0x8282CA28; continue 'dispatch;
	}
	// 8282C9FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282CA00: 481E6C89  bl 0x82a13688
	ctx.lr = 0x8282CA04;
	sub_82A13688(ctx, base);
	// 8282CA04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8282CA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282CA0C: 481E6C75  bl 0x82a13680
	ctx.lr = 0x8282CA10;
	sub_82A13680(ctx, base);
	// 8282CA10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282CA14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282CA18: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8282CA1C: 4BD5E305  bl 0x8258ad20
	ctx.lr = 0x8282CA20;
	sub_8258AD20(ctx, base);
	// 8282CA20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282CA24: 48000008  b 0x8282ca2c
	pc = 0x8282CA2C; continue 'dispatch;
	// 8282CA28: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8282CA2C: 3B3F025C  addi r25, r31, 0x25c
	ctx.r[25].s64 = ctx.r[31].s64 + 604;
	// 8282CA30: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8282CA34: 4BBA6DF5  bl 0x823d3828
	ctx.lr = 0x8282CA38;
	sub_823D3828(ctx, base);
	// 8282CA38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8282CA3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282CA40: 80BF025C  lwz r5, 0x25c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(604 as u32) ) } as u64;
	// 8282CA44: 4BCDC15D  bl 0x82508ba0
	ctx.lr = 0x8282CA48;
	sub_82508BA0(ctx, base);
	// 8282CA48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282CA4C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8282CA50: 4BFBFB79  bl 0x827ec5c8
	ctx.lr = 0x8282CA54;
	sub_827EC5C8(ctx, base);
	// 8282CA54: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8282CA58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282CA5C: 419A0058  beq cr6, 0x8282cab4
	if ctx.cr[6].eq {
	pc = 0x8282CAB4; continue 'dispatch;
	}
	// 8282CA60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282CA64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282CA68: 388B7868  addi r4, r11, 0x7868
	ctx.r[4].s64 = ctx.r[11].s64 + 30824;
	// 8282CA6C: 485C6F9D  bl 0x82df3a08
	ctx.lr = 0x8282CA70;
	sub_82DF3A08(ctx, base);
	// 8282CA70: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282CA74: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8282CA78: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8282CA7C: 485E85B5  bl 0x82e15030
	ctx.lr = 0x8282CA80;
	sub_82E15030(ctx, base);
	// 8282CA80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8282CA84: 395F0254  addi r10, r31, 0x254
	ctx.r[10].s64 = ctx.r[31].s64 + 596;
	// 8282CA88: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8282CA8C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 8282CA90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282CA94: 917F0254  stw r11, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[11].u32 ) };
	// 8282CA98: 4BA979C9  bl 0x822c4460
	ctx.lr = 0x8282CA9C;
	sub_822C4460(ctx, base);
	// 8282CA9C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8282CAA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282CAA4: 419A0008  beq cr6, 0x8282caac
	if ctx.cr[6].eq {
	pc = 0x8282CAAC; continue 'dispatch;
	}
	// 8282CAA8: 4BA93DE9  bl 0x822c0890
	ctx.lr = 0x8282CAAC;
	sub_822C0890(ctx, base);
	// 8282CAAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282CAB0: 485C6979  bl 0x82df3428
	ctx.lr = 0x8282CAB4;
	sub_82DF3428(ctx, base);
	// 8282CAB4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8282CAB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282CABC: 419A0008  beq cr6, 0x8282cac4
	if ctx.cr[6].eq {
	pc = 0x8282CAC4; continue 'dispatch;
	}
	// 8282CAC0: 4BA93DD1  bl 0x822c0890
	ctx.lr = 0x8282CAC4;
	sub_822C0890(ctx, base);
	// 8282CAC4: 807F0254  lwz r3, 0x254(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(596 as u32) ) } as u64;
	// 8282CAC8: 3B9F0254  addi r28, r31, 0x254
	ctx.r[28].s64 = ctx.r[31].s64 + 596;
	// 8282CACC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282CAD0: 419A0198  beq cr6, 0x8282cc68
	if ctx.cr[6].eq {
	pc = 0x8282CC68; continue 'dispatch;
	}
	// 8282CAD4: 485E5C3D  bl 0x82e12710
	ctx.lr = 0x8282CAD8;
	sub_82E12710(ctx, base);
	// 8282CAD8: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282CADC: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 8282CAE0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8282CAE4: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 8282CAE8: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8282CAEC: 48673C45  bl 0x82ea0730
	ctx.lr = 0x8282CAF0;
	sub_82EA0730(ctx, base);
	// 8282CAF0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282CAF4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8282CAF8: 396BB700  addi r11, r11, -0x4900
	ctx.r[11].s64 = ctx.r[11].s64 + -18688;
	// 8282CAFC: 394AB6F0  addi r10, r10, -0x4910
	ctx.r[10].s64 = ctx.r[10].s64 + -18704;
	// 8282CB00: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 8282CB04: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 8282CB08: 38E00060  li r7, 0x60
	ctx.r[7].s64 = 96;
	// 8282CB0C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282CB10: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8282CB14: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8282CB18: B0E30004  sth r7, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282CDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282CDA0 size=344
    let mut pc: u32 = 0x8282CDA0;
    'dispatch: loop {
        match pc {
            0x8282CDA0 => {
    //   block [0x8282CDA0..0x8282CEF8)
	// 8282CDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282CDA4: 4897B3C9  bl 0x831a816c
	ctx.lr = 0x8282CDA8;
	sub_831A8130(ctx, base);
	// 8282CDA8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282CDAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282CDB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282CDB4: 4BFFFBFD  bl 0x8282c9b0
	ctx.lr = 0x8282CDB8;
	sub_8282C9B0(ctx, base);
	// 8282CDB8: 817D02E0  lwz r11, 0x2e0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(736 as u32) ) } as u64;
	// 8282CDBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282CDC0: 409900EC  ble cr6, 0x8282ceac
	if !ctx.cr[6].gt {
	pc = 0x8282CEAC; continue 'dispatch;
	}
	// 8282CDC4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8282CDC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8282CDCC: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8282CDD0: 3BCA6910  addi r30, r10, 0x6910
	ctx.r[30].s64 = ctx.r[10].s64 + 26896;
	// 8282CDD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8282CDD8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8282CDDC: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282CEF8 size=84
    let mut pc: u32 = 0x8282CEF8;
    'dispatch: loop {
        match pc {
            0x8282CEF8 => {
    //   block [0x8282CEF8..0x8282CF4C)
	// 8282CEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282CEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282CF00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282CF04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282CF08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282CF0C: 4BFFFAA5  bl 0x8282c9b0
	ctx.lr = 0x8282CF10;
	sub_8282C9B0(ctx, base);
	// 8282CF10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282CF14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282CF18: 4827EDD1  bl 0x82aabce8
	ctx.lr = 0x8282CF1C;
	sub_82AABCE8(ctx, base);
	// 8282CF1C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282CF20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282CF24: 4BFEAFF5  bl 0x82817f18
	ctx.lr = 0x8282CF28;
	sub_82817F18(ctx, base);
	// 8282CF28: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8282CF2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8282CF30: 419A0008  beq cr6, 0x8282cf38
	if ctx.cr[6].eq {
	pc = 0x8282CF38; continue 'dispatch;
	}
	// 8282CF34: 4BA9395D  bl 0x822c0890
	ctx.lr = 0x8282CF38;
	sub_822C0890(ctx, base);
	// 8282CF38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282CF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282CF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282CF44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282CF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282CF50 size=332
    let mut pc: u32 = 0x8282CF50;
    'dispatch: loop {
        match pc {
            0x8282CF50 => {
    //   block [0x8282CF50..0x8282D09C)
	// 8282CF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282CF54: 4897B215  bl 0x831a8168
	ctx.lr = 0x8282CF58;
	sub_831A8130(ctx, base);
	// 8282CF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282CF5C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8282CF60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8282CF64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282CF68: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8282CF6C: 41820038  beq 0x8282cfa4
	if ctx.cr[0].eq {
	pc = 0x8282CFA4; continue 'dispatch;
	}
	// 8282CF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282CF74: 4897CA15  bl 0x831a9988
	ctx.lr = 0x8282CF78;
	sub_831A9988(ctx, base);
	// 8282CF78: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8282CF7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282CF80: 386BCAE8  addi r3, r11, -0x3518
	ctx.r[3].s64 = ctx.r[11].s64 + -13592;
	// 8282CF84: 4897B175  bl 0x831a80f8
	ctx.lr = 0x8282CF88;
	sub_831A80F8(ctx, base);
	// 8282CF88: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282CF8C: 41820018  beq 0x8282cfa4
	if ctx.cr[0].eq {
	pc = 0x8282CFA4; continue 'dispatch;
	}
	// 8282CF90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282CF94: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8282CF98: 4BFFD141  bl 0x8282a0d8
	ctx.lr = 0x8282CF9C;
	sub_8282A0D8(ctx, base);
	// 8282CF9C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8282CFA0: 480000F4  b 0x8282d094
	pc = 0x8282D094; continue 'dispatch;
	// 8282CFA4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282CFA8: 419A00DC  beq cr6, 0x8282d084
	if ctx.cr[6].eq {
	pc = 0x8282D084; continue 'dispatch;
	}
	// 8282CFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282CFB0: 4897C9D9  bl 0x831a9988
	ctx.lr = 0x8282CFB4;
	sub_831A9988(ctx, base);
	// 8282CFB4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8282CFB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282CFBC: 386B6C0C  addi r3, r11, 0x6c0c
	ctx.r[3].s64 = ctx.r[11].s64 + 27660;
	// 8282CFC0: 4897B139  bl 0x831a80f8
	ctx.lr = 0x8282CFC4;
	sub_831A80F8(ctx, base);
	// 8282CFC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282CFC8: 41820014  beq 0x8282cfdc
	if ctx.cr[0].eq {
	pc = 0x8282CFDC; continue 'dispatch;
	}
	// 8282CFCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282CFD0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8282CFD4: 4803DF4D  bl 0x8286af20
	ctx.lr = 0x8282CFD8;
	sub_8286AF20(ctx, base);
	// 8282CFD8: 4BFFFFC4  b 0x8282cf9c
	pc = 0x8282CF9C; continue 'dispatch;
	// 8282CFDC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282CFE0: 419A00A4  beq cr6, 0x8282d084
	if ctx.cr[6].eq {
	pc = 0x8282D084; continue 'dispatch;
	}
	// 8282CFE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282CFE8: 4897C9A1  bl 0x831a9988
	ctx.lr = 0x8282CFEC;
	sub_831A9988(ctx, base);
	// 8282CFEC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8282CFF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282CFF4: 386B6BDC  addi r3, r11, 0x6bdc
	ctx.r[3].s64 = ctx.r[11].s64 + 27612;
	// 8282CFF8: 4897B101  bl 0x831a80f8
	ctx.lr = 0x8282CFFC;
	sub_831A80F8(ctx, base);
	// 8282CFFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282D000: 41820014  beq 0x8282d014
	if ctx.cr[0].eq {
	pc = 0x8282D014; continue 'dispatch;
	}
	// 8282D004: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D008: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8282D00C: 4BFFDB0D  bl 0x8282ab18
	ctx.lr = 0x8282D010;
	sub_8282AB18(ctx, base);
	// 8282D010: 4BFFFF8C  b 0x8282cf9c
	pc = 0x8282CF9C; continue 'dispatch;
	// 8282D014: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282D018: 419A006C  beq cr6, 0x8282d084
	if ctx.cr[6].eq {
	pc = 0x8282D084; continue 'dispatch;
	}
	// 8282D01C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D020: 4897C969  bl 0x831a9988
	ctx.lr = 0x8282D024;
	sub_831A9988(ctx, base);
	// 8282D024: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8282D028: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282D02C: 386B6BDC  addi r3, r11, 0x6bdc
	ctx.r[3].s64 = ctx.r[11].s64 + 27612;
	// 8282D030: 4897B0C9  bl 0x831a80f8
	ctx.lr = 0x8282D034;
	sub_831A80F8(ctx, base);
	// 8282D034: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282D038: 41820014  beq 0x8282d04c
	if ctx.cr[0].eq {
	pc = 0x8282D04C; continue 'dispatch;
	}
	// 8282D03C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D040: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8282D044: 4BFFDAED  bl 0x8282ab30
	ctx.lr = 0x8282D048;
	sub_8282AB30(ctx, base);
	// 8282D048: 4BFFFF54  b 0x8282cf9c
	pc = 0x8282CF9C; continue 'dispatch;
	// 8282D04C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8282D050: 419A0034  beq cr6, 0x8282d084
	if ctx.cr[6].eq {
	pc = 0x8282D084; continue 'dispatch;
	}
	// 8282D054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D058: 4897C931  bl 0x831a9988
	ctx.lr = 0x8282D05C;
	sub_831A9988(ctx, base);
	// 8282D05C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282D060: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282D064: 386BDBD4  addi r3, r11, -0x242c
	ctx.r[3].s64 = ctx.r[11].s64 + -9260;
	// 8282D068: 4897B091  bl 0x831a80f8
	ctx.lr = 0x8282D06C;
	sub_831A80F8(ctx, base);
	// 8282D06C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282D070: 41820014  beq 0x8282d084
	if ctx.cr[0].eq {
	pc = 0x8282D084; continue 'dispatch;
	}
	// 8282D074: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D078: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8282D07C: 4BFFC6FD  bl 0x82829778
	ctx.lr = 0x8282D080;
	sub_82829778(ctx, base);
	// 8282D080: 4BFFFF1C  b 0x8282cf9c
	pc = 0x8282CF9C; continue 'dispatch;
	// 8282D084: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8282D088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8282D090: 4BFEC7A1  bl 0x82819830
	ctx.lr = 0x8282D094;
	sub_82819830(ctx, base);
	// 8282D094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8282D098: 4897B120  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282D0A0 size=108
    let mut pc: u32 = 0x8282D0A0;
    'dispatch: loop {
        match pc {
            0x8282D0A0 => {
    //   block [0x8282D0A0..0x8282D10C)
	// 8282D0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282D0A4: 4897B0C9  bl 0x831a816c
	ctx.lr = 0x8282D0A8;
	sub_831A8130(ctx, base);
	// 8282D0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282D0AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8282D0B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282D0B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282D0B8: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282D0BC: 41820038  beq 0x8282d0f4
	if ctx.cr[0].eq {
	pc = 0x8282D0F4; continue 'dispatch;
	}
	// 8282D0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D0C4: 4897C8C5  bl 0x831a9988
	ctx.lr = 0x8282D0C8;
	sub_831A9988(ctx, base);
	// 8282D0C8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282D0CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282D0D0: 386BDBD4  addi r3, r11, -0x242c
	ctx.r[3].s64 = ctx.r[11].s64 + -9260;
	// 8282D0D4: 4897B025  bl 0x831a80f8
	ctx.lr = 0x8282D0D8;
	sub_831A80F8(ctx, base);
	// 8282D0D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282D0DC: 41820018  beq 0x8282d0f4
	if ctx.cr[0].eq {
	pc = 0x8282D0F4; continue 'dispatch;
	}
	// 8282D0E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D0E4: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 8282D0E8: 4BFFC7A1  bl 0x82829888
	ctx.lr = 0x8282D0EC;
	sub_82829888(ctx, base);
	// 8282D0EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8282D0F0: 48000014  b 0x8282d104
	pc = 0x8282D104; continue 'dispatch;
	// 8282D0F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8282D0F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D0FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282D100: 4BFFFE51  bl 0x8282cf50
	ctx.lr = 0x8282D104;
	sub_8282CF50(ctx, base);
	// 8282D104: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282D108: 4897B0B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282D110 size=108
    let mut pc: u32 = 0x8282D110;
    'dispatch: loop {
        match pc {
            0x8282D110 => {
    //   block [0x8282D110..0x8282D17C)
	// 8282D110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282D114: 4897B059  bl 0x831a816c
	ctx.lr = 0x8282D118;
	sub_831A8130(ctx, base);
	// 8282D118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282D11C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8282D120: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8282D124: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282D128: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282D12C: 41820038  beq 0x8282d164
	if ctx.cr[0].eq {
	pc = 0x8282D164; continue 'dispatch;
	}
	// 8282D130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D134: 4897C855  bl 0x831a9988
	ctx.lr = 0x8282D138;
	sub_831A9988(ctx, base);
	// 8282D138: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8282D13C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8282D140: 386BDBD4  addi r3, r11, -0x242c
	ctx.r[3].s64 = ctx.r[11].s64 + -9260;
	// 8282D144: 4897AFB5  bl 0x831a80f8
	ctx.lr = 0x8282D148;
	sub_831A80F8(ctx, base);
	// 8282D148: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8282D14C: 41820018  beq 0x8282d164
	if ctx.cr[0].eq {
	pc = 0x8282D164; continue 'dispatch;
	}
	// 8282D150: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D154: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 8282D158: 4BFFC819  bl 0x82829970
	ctx.lr = 0x8282D15C;
	sub_82829970(ctx, base);
	// 8282D15C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8282D160: 48000014  b 0x8282d174
	pc = 0x8282D174; continue 'dispatch;
	// 8282D164: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8282D168: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8282D16C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8282D170: 4BFFFDE1  bl 0x8282cf50
	ctx.lr = 0x8282D174;
	sub_8282CF50(ctx, base);
	// 8282D174: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8282D178: 4897B044  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282D180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8282D180 size=84
    let mut pc: u32 = 0x8282D180;
    'dispatch: loop {
        match pc {
            0x8282D180 => {
    //   block [0x8282D180..0x8282D1D4)
	// 8282D180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282D184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282D188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282D18C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282D190: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282D194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8282D198: 388BB21C  addi r4, r11, -0x4de4
	ctx.r[4].s64 = ctx.r[11].s64 + -19940;
	// 8282D19C: 4BFC368D  bl 0x827f0828
	ctx.lr = 0x8282D1A0;
	sub_827F0828(ctx, base);
	// 8282D1A0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282D1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D1A8: 388BB238  addi r4, r11, -0x4dc8
	ctx.r[4].s64 = ctx.r[11].s64 + -19912;
	// 8282D1AC: 4BFC8F45  bl 0x827f60f0
	ctx.lr = 0x8282D1B0;
	sub_827F60F0(ctx, base);
	// 8282D1B0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282D1B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D1B8: 388BB248  addi r4, r11, -0x4db8
	ctx.r[4].s64 = ctx.r[11].s64 + -19896;
	// 8282D1BC: 4BFC390D  bl 0x827f0ac8
	ctx.lr = 0x8282D1C0;
	sub_827F0AC8(ctx, base);
	// 8282D1C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8282D1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282D1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282D1CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282D1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282D1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8282D1D8 size=276
    let mut pc: u32 = 0x8282D1D8;
    'dispatch: loop {
        match pc {
            0x8282D1D8 => {
    //   block [0x8282D1D8..0x8282D2EC)
	// 8282D1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8282D1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8282D1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8282D1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8282D1E8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8282D1EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8282D1F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8282D1F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8282D1F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D1FC: 388BF9A0  addi r4, r11, -0x660
	ctx.r[4].s64 = ctx.r[11].s64 + -1632;
	// 8282D200: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8282D204: 485C6805  bl 0x82df3a08
	ctx.lr = 0x8282D208;
	sub_82DF3A08(ctx, base);
	// 8282D208: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8282D20C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282D210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D214: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282D218: C3EBA1C4  lfs f31, -0x5e3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8282D21C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282D220: 4BFBEA99  bl 0x827ebcb8
	ctx.lr = 0x8282D224;
	sub_827EBCB8(ctx, base);
	// 8282D224: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D228: 485C6201  bl 0x82df3428
	ctx.lr = 0x8282D22C;
	sub_82DF3428(ctx, base);
	// 8282D22C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282D230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D234: 388BB774  addi r4, r11, -0x488c
	ctx.r[4].s64 = ctx.r[11].s64 + -18572;
	// 8282D238: 485C67D1  bl 0x82df3a08
	ctx.lr = 0x8282D23C;
	sub_82DF3A08(ctx, base);
	// 8282D23C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282D240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D244: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8282D248: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282D24C: 4BFBEA6D  bl 0x827ebcb8
	ctx.lr = 0x8282D250;
	sub_827EBCB8(ctx, base);
	// 8282D250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D254: 485C61D5  bl 0x82df3428
	ctx.lr = 0x8282D258;
	sub_82DF3428(ctx, base);
	// 8282D258: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282D25C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D260: 388BB1A8  addi r4, r11, -0x4e58
	ctx.r[4].s64 = ctx.r[11].s64 + -20056;
	// 8282D264: 485C67A5  bl 0x82df3a08
	ctx.lr = 0x8282D268;
	sub_82DF3A08(ctx, base);
	// 8282D268: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282D26C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D270: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282D274: 4BFBE985  bl 0x827ebbf8
	ctx.lr = 0x8282D278;
	sub_827EBBF8(ctx, base);
	// 8282D278: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D27C: 485C61AD  bl 0x82df3428
	ctx.lr = 0x8282D280;
	sub_82DF3428(ctx, base);
	// 8282D280: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8282D284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D288: 388BB76C  addi r4, r11, -0x4894
	ctx.r[4].s64 = ctx.r[11].s64 + -18580;
	// 8282D28C: 485C677D  bl 0x82df3a08
	ctx.lr = 0x8282D290;
	sub_82DF3A08(ctx, base);
	// 8282D290: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282D294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D298: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282D29C: 4BFBE895  bl 0x827ebb30
	ctx.lr = 0x8282D2A0;
	sub_827EBB30(ctx, base);
	// 8282D2A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D2A4: 485C6185  bl 0x82df3428
	ctx.lr = 0x8282D2A8;
	sub_82DF3428(ctx, base);
	// 8282D2A8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 8282D2AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D2B0: 388BD748  addi r4, r11, -0x28b8
	ctx.r[4].s64 = ctx.r[11].s64 + -10424;
	// 8282D2B4: 485C6755  bl 0x82df3a08
	ctx.lr = 0x8282D2B8;
	sub_82DF3A08(ctx, base);
	// 8282D2B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8282D2BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8282D2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8282D2C4: 4BFBE86D  bl 0x827ebb30
	ctx.lr = 0x8282D2C8;
	sub_827EBB30(ctx, base);
	// 8282D2C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8282D2CC: 485C615D  bl 0x82df3428
	ctx.lr = 0x8282D2D0;
	sub_82DF3428(ctx, base);
	// 8282D2D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8282D2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8282D2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8282D2DC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8282D2E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8282D2E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8282D2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282D2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282D2F0 size=12
    let mut pc: u32 = 0x8282D2F0;
    'dispatch: loop {
        match pc {
            0x8282D2F0 => {
    //   block [0x8282D2F0..0x8282D2FC)
	// 8282D2F0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282D2F4: 386BB238  addi r3, r11, -0x4dc8
	ctx.r[3].s64 = ctx.r[11].s64 + -19912;
	// 8282D2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8282D300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8282D300 size=12
    let mut pc: u32 = 0x8282D300;
    'dispatch: loop {
        match pc {
            0x8282D300 => {
    //   block [0x8282D300..0x8282D30C)
	// 8282D300: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8282D304: 386BB21C  addi r3, r11, -0x4de4
	ctx.r[3].s64 = ctx.r[11].s64 + -19940;
	// 8282D308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


