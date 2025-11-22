pub fn sub_82DB2830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2830 size=12
    let mut pc: u32 = 0x82DB2830;
    'dispatch: loop {
        match pc {
            0x82DB2830 => {
    //   block [0x82DB2830..0x82DB283C)
	// 82DB2830: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB2834: 386B4D6C  addi r3, r11, 0x4d6c
	ctx.r[3].s64 = ctx.r[11].s64 + 19820;
	// 82DB2838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2840 size=100
    let mut pc: u32 = 0x82DB2840;
    'dispatch: loop {
        match pc {
            0x82DB2840 => {
    //   block [0x82DB2840..0x82DB28A4)
	// 82DB2840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB284C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB2850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB2858: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB285C: 4800B8AD  bl 0x82dbe108
	ctx.lr = 0x82DB2860;
	sub_82DBE108(ctx, base);
	// 82DB2860: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB2864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB2868: 419A0020  beq cr6, 0x82db2888
	if ctx.cr[6].eq {
	pc = 0x82DB2888; continue 'dispatch;
	}
	// 82DB286C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2870: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB2874: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB2878: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB287C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB2880: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB2884: 4BFA2A45  bl 0x82d552c8
	ctx.lr = 0x82DB2888;
	sub_82D552C8(ctx, base);
	// 82DB2888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB288C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB2890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2898: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB289C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB28A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB28A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB28A8 size=20
    let mut pc: u32 = 0x82DB28A8;
    'dispatch: loop {
        match pc {
            0x82DB28A8 => {
    //   block [0x82DB28A8..0x82DB28BC)
	// 82DB28A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB28AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB28B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB28B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB28B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB28C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB28C0 size=8
    let mut pc: u32 = 0x82DB28C0;
    'dispatch: loop {
        match pc {
            0x82DB28C0 => {
    //   block [0x82DB28C0..0x82DB28C8)
	// 82DB28C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB28C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB28C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB28C8 size=44
    let mut pc: u32 = 0x82DB28C8;
    'dispatch: loop {
        match pc {
            0x82DB28C8 => {
    //   block [0x82DB28C8..0x82DB28F4)
	// 82DB28C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB28CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB28D0: 396B0E48  addi r11, r11, 0xe48
	ctx.r[11].s64 = ctx.r[11].s64 + 3656;
	// 82DB28D4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB28D8: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DB28DC: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 82DB28E0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB28E4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB28E8: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DB28EC: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DB28F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB28F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB28F8 size=12
    let mut pc: u32 = 0x82DB28F8;
    'dispatch: loop {
        match pc {
            0x82DB28F8 => {
    //   block [0x82DB28F8..0x82DB2904)
	// 82DB28F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB28FC: 386B0E48  addi r3, r11, 0xe48
	ctx.r[3].s64 = ctx.r[11].s64 + 3656;
	// 82DB2900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2908 size=100
    let mut pc: u32 = 0x82DB2908;
    'dispatch: loop {
        match pc {
            0x82DB2908 => {
    //   block [0x82DB2908..0x82DB296C)
	// 82DB2908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB2914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB2918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB291C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB2920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB2924: 48013645  bl 0x82dc5f68
	ctx.lr = 0x82DB2928;
	sub_82DC5F68(ctx, base);
	// 82DB2928: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB292C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB2930: 419A0020  beq cr6, 0x82db2950
	if ctx.cr[6].eq {
	pc = 0x82DB2950; continue 'dispatch;
	}
	// 82DB2934: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2938: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB293C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB2940: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB2944: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB2948: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB294C: 4BFA297D  bl 0x82d552c8
	ctx.lr = 0x82DB2950;
	sub_82D552C8(ctx, base);
	// 82DB2950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB2954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB2958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB295C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2960: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB2964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB2968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2970 size=4
    let mut pc: u32 = 0x82DB2970;
    'dispatch: loop {
        match pc {
            0x82DB2970 => {
    //   block [0x82DB2970..0x82DB2974)
	// 82DB2970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2978 size=20
    let mut pc: u32 = 0x82DB2978;
    'dispatch: loop {
        match pc {
            0x82DB2978 => {
    //   block [0x82DB2978..0x82DB298C)
	// 82DB2978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB297C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2980: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2988: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2990 size=8
    let mut pc: u32 = 0x82DB2990;
    'dispatch: loop {
        match pc {
            0x82DB2990 => {
    //   block [0x82DB2990..0x82DB2998)
	// 82DB2990: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2994: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2998 size=32
    let mut pc: u32 = 0x82DB2998;
    'dispatch: loop {
        match pc {
            0x82DB2998 => {
    //   block [0x82DB2998..0x82DB29B8)
	// 82DB2998: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB299C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB29A0: 396B4DF4  addi r11, r11, 0x4df4
	ctx.r[11].s64 = ctx.r[11].s64 + 19956;
	// 82DB29A4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DB29A8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB29AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB29B0: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB29B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB29B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB29B8 size=12
    let mut pc: u32 = 0x82DB29B8;
    'dispatch: loop {
        match pc {
            0x82DB29B8 => {
    //   block [0x82DB29B8..0x82DB29C4)
	// 82DB29B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB29BC: 386B4DF4  addi r3, r11, 0x4df4
	ctx.r[3].s64 = ctx.r[11].s64 + 19956;
	// 82DB29C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB29C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB29C8 size=20
    let mut pc: u32 = 0x82DB29C8;
    'dispatch: loop {
        match pc {
            0x82DB29C8 => {
    //   block [0x82DB29C8..0x82DB29DC)
	// 82DB29C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB29CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB29D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB29D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB29D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB29E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB29E0 size=8
    let mut pc: u32 = 0x82DB29E0;
    'dispatch: loop {
        match pc {
            0x82DB29E0 => {
    //   block [0x82DB29E0..0x82DB29E8)
	// 82DB29E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB29E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB29E8 size=32
    let mut pc: u32 = 0x82DB29E8;
    'dispatch: loop {
        match pc {
            0x82DB29E8 => {
    //   block [0x82DB29E8..0x82DB2A08)
	// 82DB29E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB29EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB29F0: 396B4AC4  addi r11, r11, 0x4ac4
	ctx.r[11].s64 = ctx.r[11].s64 + 19140;
	// 82DB29F4: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82DB29F8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB29FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB2A00: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB2A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A08 size=12
    let mut pc: u32 = 0x82DB2A08;
    'dispatch: loop {
        match pc {
            0x82DB2A08 => {
    //   block [0x82DB2A08..0x82DB2A14)
	// 82DB2A08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB2A0C: 386B4AC4  addi r3, r11, 0x4ac4
	ctx.r[3].s64 = ctx.r[11].s64 + 19140;
	// 82DB2A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A18 size=4
    let mut pc: u32 = 0x82DB2A18;
    'dispatch: loop {
        match pc {
            0x82DB2A18 => {
    //   block [0x82DB2A18..0x82DB2A1C)
	// 82DB2A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A20 size=4
    let mut pc: u32 = 0x82DB2A20;
    'dispatch: loop {
        match pc {
            0x82DB2A20 => {
    //   block [0x82DB2A20..0x82DB2A24)
	// 82DB2A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A28 size=12
    let mut pc: u32 = 0x82DB2A28;
    'dispatch: loop {
        match pc {
            0x82DB2A28 => {
    //   block [0x82DB2A28..0x82DB2A34)
	// 82DB2A28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB2A2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2A30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A34 size=8
    let mut pc: u32 = 0x82DB2A34;
    'dispatch: loop {
        match pc {
            0x82DB2A34 => {
    //   block [0x82DB2A34..0x82DB2A3C)
	// 82DB2A34: 480024A4  b 0x82db4ed8
	sub_82DB4ED8(ctx, base);
	return;
	// 82DB2A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A40 size=20
    let mut pc: u32 = 0x82DB2A40;
    'dispatch: loop {
        match pc {
            0x82DB2A40 => {
    //   block [0x82DB2A40..0x82DB2A54)
	// 82DB2A40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2A44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2A48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2A4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2A50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2A58 size=44
    let mut pc: u32 = 0x82DB2A58;
    'dispatch: loop {
        match pc {
            0x82DB2A58 => {
    //   block [0x82DB2A58..0x82DB2A84)
	// 82DB2A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2A60: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2A64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB2A6C: 4800246D  bl 0x82db4ed8
	ctx.lr = 0x82DB2A70;
	sub_82DB4ED8(ctx, base);
	// 82DB2A70: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB2A74: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82DB2A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A88 size=4
    let mut pc: u32 = 0x82DB2A88;
    'dispatch: loop {
        match pc {
            0x82DB2A88 => {
    //   block [0x82DB2A88..0x82DB2A8C)
	// 82DB2A88: 48001DB8  b 0x82db4840
	sub_82DB4840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2A90 size=20
    let mut pc: u32 = 0x82DB2A90;
    'dispatch: loop {
        match pc {
            0x82DB2A90 => {
    //   block [0x82DB2A90..0x82DB2AA4)
	// 82DB2A90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2A94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2A98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2A9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2AA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2AA8 size=8
    let mut pc: u32 = 0x82DB2AA8;
    'dispatch: loop {
        match pc {
            0x82DB2AA8 => {
    //   block [0x82DB2AA8..0x82DB2AB0)
	// 82DB2AA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2AAC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2AB0 size=128
    let mut pc: u32 = 0x82DB2AB0;
    'dispatch: loop {
        match pc {
            0x82DB2AB0 => {
    //   block [0x82DB2AB0..0x82DB2B30)
	// 82DB2AB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2AB4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DB2AB8: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82DB2ABC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB2AC0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB2AC4: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82DB2AC8: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DB2ACC: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DB2AD0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DB2AD4: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DB2AD8: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82DB2ADC: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82DB2AE0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2AE4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DB2AE8: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DB2AEC: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82DB2AF0: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82DB2AF4: 38E71544  addi r7, r7, 0x1544
	ctx.r[7].s64 = ctx.r[7].s64 + 5444;
	// 82DB2AF8: 38C61538  addi r6, r6, 0x1538
	ctx.r[6].s64 = ctx.r[6].s64 + 5432;
	// 82DB2AFC: 38A51524  addi r5, r5, 0x1524
	ctx.r[5].s64 = ctx.r[5].s64 + 5412;
	// 82DB2B00: 396B1518  addi r11, r11, 0x1518
	ctx.r[11].s64 = ctx.r[11].s64 + 5400;
	// 82DB2B04: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DB2B08: 3884150C  addi r4, r4, 0x150c
	ctx.r[4].s64 = ctx.r[4].s64 + 5388;
	// 82DB2B0C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DB2B10: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82DB2B14: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DB2B18: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DB2B1C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82DB2B20: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB2B24: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DB2B28: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DB2B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2B30 size=12
    let mut pc: u32 = 0x82DB2B30;
    'dispatch: loop {
        match pc {
            0x82DB2B30 => {
    //   block [0x82DB2B30..0x82DB2B3C)
	// 82DB2B30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2B34: 386B1544  addi r3, r11, 0x1544
	ctx.r[3].s64 = ctx.r[11].s64 + 5444;
	// 82DB2B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2B40 size=100
    let mut pc: u32 = 0x82DB2B40;
    'dispatch: loop {
        match pc {
            0x82DB2B40 => {
    //   block [0x82DB2B40..0x82DB2BA4)
	// 82DB2B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB2B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB2B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2B54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB2B58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB2B5C: 48013C6D  bl 0x82dc67c8
	ctx.lr = 0x82DB2B60;
	sub_82DC67C8(ctx, base);
	// 82DB2B60: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB2B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB2B68: 419A0020  beq cr6, 0x82db2b88
	if ctx.cr[6].eq {
	pc = 0x82DB2B88; continue 'dispatch;
	}
	// 82DB2B6C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2B70: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB2B74: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB2B78: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB2B7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB2B80: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB2B84: 4BFA2745  bl 0x82d552c8
	ctx.lr = 0x82DB2B88;
	sub_82D552C8(ctx, base);
	// 82DB2B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB2B8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB2B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2B98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB2B9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB2BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2BA8 size=8
    let mut pc: u32 = 0x82DB2BA8;
    'dispatch: loop {
        match pc {
            0x82DB2BA8 => {
    //   block [0x82DB2BA8..0x82DB2BB0)
	// 82DB2BA8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82DB2BAC: 4BFFFF94  b 0x82db2b40
	sub_82DB2B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2BB0 size=8
    let mut pc: u32 = 0x82DB2BB0;
    'dispatch: loop {
        match pc {
            0x82DB2BB0 => {
    //   block [0x82DB2BB0..0x82DB2BB8)
	// 82DB2BB0: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82DB2BB4: 4BFFFF8C  b 0x82db2b40
	sub_82DB2B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2BB8 size=8
    let mut pc: u32 = 0x82DB2BB8;
    'dispatch: loop {
        match pc {
            0x82DB2BB8 => {
    //   block [0x82DB2BB8..0x82DB2BC0)
	// 82DB2BB8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DB2BBC: 4BFFFF84  b 0x82db2b40
	sub_82DB2B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2BC0 size=8
    let mut pc: u32 = 0x82DB2BC0;
    'dispatch: loop {
        match pc {
            0x82DB2BC0 => {
    //   block [0x82DB2BC0..0x82DB2BC8)
	// 82DB2BC0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82DB2BC4: 4BFFFF7C  b 0x82db2b40
	sub_82DB2B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2BC8 size=20
    let mut pc: u32 = 0x82DB2BC8;
    'dispatch: loop {
        match pc {
            0x82DB2BC8 => {
    //   block [0x82DB2BC8..0x82DB2BDC)
	// 82DB2BC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2BCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2BD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2BD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2BD8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2BE0 size=8
    let mut pc: u32 = 0x82DB2BE0;
    'dispatch: loop {
        match pc {
            0x82DB2BE0 => {
    //   block [0x82DB2BE0..0x82DB2BE8)
	// 82DB2BE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2BE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2BE8 size=56
    let mut pc: u32 = 0x82DB2BE8;
    'dispatch: loop {
        match pc {
            0x82DB2BE8 => {
    //   block [0x82DB2BE8..0x82DB2C20)
	// 82DB2BE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2BEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB2BF0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DB2BF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DB2BF8: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB2BFC: 394A3734  addi r10, r10, 0x3734
	ctx.r[10].s64 = ctx.r[10].s64 + 14132;
	// 82DB2C00: 392924EC  addi r9, r9, 0x24ec
	ctx.r[9].s64 = ctx.r[9].s64 + 9452;
	// 82DB2C04: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 82DB2C08: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DB2C0C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DB2C10: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB2C14: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DB2C18: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB2C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2C20 size=12
    let mut pc: u32 = 0x82DB2C20;
    'dispatch: loop {
        match pc {
            0x82DB2C20 => {
    //   block [0x82DB2C20..0x82DB2C2C)
	// 82DB2C20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB2C24: 386B3734  addi r3, r11, 0x3734
	ctx.r[3].s64 = ctx.r[11].s64 + 14132;
	// 82DB2C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2C30 size=8
    let mut pc: u32 = 0x82DB2C30;
    'dispatch: loop {
        match pc {
            0x82DB2C30 => {
    //   block [0x82DB2C30..0x82DB2C38)
	// 82DB2C30: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82DB2C34: 48000004  b 0x82db2c38
	sub_82DB2C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2C38 size=100
    let mut pc: u32 = 0x82DB2C38;
    'dispatch: loop {
        match pc {
            0x82DB2C38 => {
    //   block [0x82DB2C38..0x82DB2C9C)
	// 82DB2C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2C40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB2C44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB2C48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2C4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB2C50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB2C54: 4800858D  bl 0x82dbb1e0
	ctx.lr = 0x82DB2C58;
	sub_82DBB1E0(ctx, base);
	// 82DB2C58: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB2C5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB2C60: 419A0020  beq cr6, 0x82db2c80
	if ctx.cr[6].eq {
	pc = 0x82DB2C80; continue 'dispatch;
	}
	// 82DB2C64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2C68: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB2C6C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB2C70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB2C74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB2C78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB2C7C: 4BFA264D  bl 0x82d552c8
	ctx.lr = 0x82DB2C80;
	sub_82D552C8(ctx, base);
	// 82DB2C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB2C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB2C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2C90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB2C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB2C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2CA0 size=20
    let mut pc: u32 = 0x82DB2CA0;
    'dispatch: loop {
        match pc {
            0x82DB2CA0 => {
    //   block [0x82DB2CA0..0x82DB2CB4)
	// 82DB2CA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2CA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2CA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2CAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2CB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2CB8 size=8
    let mut pc: u32 = 0x82DB2CB8;
    'dispatch: loop {
        match pc {
            0x82DB2CB8 => {
    //   block [0x82DB2CB8..0x82DB2CC0)
	// 82DB2CB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2CBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2CC0 size=56
    let mut pc: u32 = 0x82DB2CC0;
    'dispatch: loop {
        match pc {
            0x82DB2CC0 => {
    //   block [0x82DB2CC0..0x82DB2CF8)
	// 82DB2CC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2CC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB2CC8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB2CCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DB2CD0: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB2CD4: 394A16C8  addi r10, r10, 0x16c8
	ctx.r[10].s64 = ctx.r[10].s64 + 5832;
	// 82DB2CD8: 392916A4  addi r9, r9, 0x16a4
	ctx.r[9].s64 = ctx.r[9].s64 + 5796;
	// 82DB2CDC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82DB2CE0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DB2CE4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB2CE8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB2CEC: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DB2CF0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB2CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2CF8 size=12
    let mut pc: u32 = 0x82DB2CF8;
    'dispatch: loop {
        match pc {
            0x82DB2CF8 => {
    //   block [0x82DB2CF8..0x82DB2D04)
	// 82DB2CF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2CFC: 386B16C8  addi r3, r11, 0x16c8
	ctx.r[3].s64 = ctx.r[11].s64 + 5832;
	// 82DB2D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2D08 size=100
    let mut pc: u32 = 0x82DB2D08;
    'dispatch: loop {
        match pc {
            0x82DB2D08 => {
    //   block [0x82DB2D08..0x82DB2D6C)
	// 82DB2D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2D10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB2D14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB2D18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB2D20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB2D24: 48013DED  bl 0x82dc6b10
	ctx.lr = 0x82DB2D28;
	sub_82DC6B10(ctx, base);
	// 82DB2D28: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB2D2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB2D30: 419A0020  beq cr6, 0x82db2d50
	if ctx.cr[6].eq {
	pc = 0x82DB2D50; continue 'dispatch;
	}
	// 82DB2D34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2D38: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB2D3C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB2D40: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB2D44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB2D48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB2D4C: 4BFA257D  bl 0x82d552c8
	ctx.lr = 0x82DB2D50;
	sub_82D552C8(ctx, base);
	// 82DB2D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB2D54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB2D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2D60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB2D64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB2D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2D70 size=8
    let mut pc: u32 = 0x82DB2D70;
    'dispatch: loop {
        match pc {
            0x82DB2D70 => {
    //   block [0x82DB2D70..0x82DB2D78)
	// 82DB2D70: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DB2D74: 4BFFFF94  b 0x82db2d08
	sub_82DB2D08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2D78 size=4
    let mut pc: u32 = 0x82DB2D78;
    'dispatch: loop {
        match pc {
            0x82DB2D78 => {
    //   block [0x82DB2D78..0x82DB2D7C)
	// 82DB2D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2D80 size=20
    let mut pc: u32 = 0x82DB2D80;
    'dispatch: loop {
        match pc {
            0x82DB2D80 => {
    //   block [0x82DB2D80..0x82DB2D94)
	// 82DB2D80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2D84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2D88: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2D8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2D90: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2D98 size=12
    let mut pc: u32 = 0x82DB2D98;
    'dispatch: loop {
        match pc {
            0x82DB2D98 => {
    //   block [0x82DB2D98..0x82DB2DA4)
	// 82DB2D98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB2D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2DA0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2DA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2DA4 size=8
    let mut pc: u32 = 0x82DB2DA4;
    'dispatch: loop {
        match pc {
            0x82DB2DA4 => {
    //   block [0x82DB2DA4..0x82DB2DAC)
	// 82DB2DA4: 4801470C  b 0x82dc74b0
	sub_82DC74B0(ctx, base);
	return;
	// 82DB2DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2DB0 size=44
    let mut pc: u32 = 0x82DB2DB0;
    'dispatch: loop {
        match pc {
            0x82DB2DB0 => {
    //   block [0x82DB2DB0..0x82DB2DDC)
	// 82DB2DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2DB8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2DBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB2DC4: 480146ED  bl 0x82dc74b0
	ctx.lr = 0x82DB2DC8;
	sub_82DC74B0(ctx, base);
	// 82DB2DC8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB2DCC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DB2DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2DE0 size=20
    let mut pc: u32 = 0x82DB2DE0;
    'dispatch: loop {
        match pc {
            0x82DB2DE0 => {
    //   block [0x82DB2DE0..0x82DB2DF4)
	// 82DB2DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2DE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2DE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2DF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2DF8 size=8
    let mut pc: u32 = 0x82DB2DF8;
    'dispatch: loop {
        match pc {
            0x82DB2DF8 => {
    //   block [0x82DB2DF8..0x82DB2E00)
	// 82DB2DF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2DFC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2E00 size=44
    let mut pc: u32 = 0x82DB2E00;
    'dispatch: loop {
        match pc {
            0x82DB2E00 => {
    //   block [0x82DB2E00..0x82DB2E2C)
	// 82DB2E00: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB2E04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB2E08: 396B346C  addi r11, r11, 0x346c
	ctx.r[11].s64 = ctx.r[11].s64 + 13420;
	// 82DB2E0C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB2E10: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DB2E14: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 82DB2E18: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB2E1C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB2E20: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DB2E24: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DB2E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2E30 size=12
    let mut pc: u32 = 0x82DB2E30;
    'dispatch: loop {
        match pc {
            0x82DB2E30 => {
    //   block [0x82DB2E30..0x82DB2E3C)
	// 82DB2E30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB2E34: 386B346C  addi r3, r11, 0x346c
	ctx.r[3].s64 = ctx.r[11].s64 + 13420;
	// 82DB2E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2E40 size=144
    let mut pc: u32 = 0x82DB2E40;
    'dispatch: loop {
        match pc {
            0x82DB2E40 => {
    //   block [0x82DB2E40..0x82DB2ED0)
	// 82DB2E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB2E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB2E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2E54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB2E58: 3BC5FFA0  addi r30, r5, -0x60
	ctx.r[30].s64 = ctx.r[5].s64 + -96;
	// 82DB2E5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DB2E60: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB2E64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2E68: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB2E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2E70: 4E800421  bctrl
	ctx.lr = 0x82DB2E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB2E74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DB2E78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB2E7C: 41980038  blt cr6, 0x82db2eb4
	if ctx.cr[6].lt {
	pc = 0x82DB2EB4; continue 'dispatch;
	}
	// 82DB2E80: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DB2E84: 41990030  bgt cr6, 0x82db2eb4
	if ctx.cr[6].gt {
	pc = 0x82DB2EB4; continue 'dispatch;
	}
	// 82DB2E88: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB2E8C: 393F0060  addi r9, r31, 0x60
	ctx.r[9].s64 = ctx.r[31].s64 + 96;
	// 82DB2E90: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB2E94: 409A0014  bne cr6, 0x82db2ea8
	if !ctx.cr[6].eq {
	pc = 0x82DB2EA8; continue 'dispatch;
	}
	// 82DB2E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB2E9C: 386B0060  addi r3, r11, 0x60
	ctx.r[3].s64 = ctx.r[11].s64 + 96;
	// 82DB2EA0: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82DB2EA4: 48000014  b 0x82db2eb8
	pc = 0x82DB2EB8; continue 'dispatch;
	// 82DB2EA8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82DB2EAC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DB2EB0: 48000008  b 0x82db2eb8
	pc = 0x82DB2EB8; continue 'dispatch;
	// 82DB2EB4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DB2EB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB2EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2EC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB2EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB2ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2ED0 size=4
    let mut pc: u32 = 0x82DB2ED0;
    'dispatch: loop {
        match pc {
            0x82DB2ED0 => {
    //   block [0x82DB2ED0..0x82DB2ED4)
	// 82DB2ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2ED8 size=20
    let mut pc: u32 = 0x82DB2ED8;
    'dispatch: loop {
        match pc {
            0x82DB2ED8 => {
    //   block [0x82DB2ED8..0x82DB2EEC)
	// 82DB2ED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2EDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2EE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2EE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2EE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2EF0 size=8
    let mut pc: u32 = 0x82DB2EF0;
    'dispatch: loop {
        match pc {
            0x82DB2EF0 => {
    //   block [0x82DB2EF0..0x82DB2EF8)
	// 82DB2EF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2EF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2EF8 size=44
    let mut pc: u32 = 0x82DB2EF8;
    'dispatch: loop {
        match pc {
            0x82DB2EF8 => {
    //   block [0x82DB2EF8..0x82DB2F24)
	// 82DB2EF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2EFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB2F00: 396B191C  addi r11, r11, 0x191c
	ctx.r[11].s64 = ctx.r[11].s64 + 6428;
	// 82DB2F04: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB2F08: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DB2F0C: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82DB2F10: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB2F14: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB2F18: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DB2F1C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DB2F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2F28 size=12
    let mut pc: u32 = 0x82DB2F28;
    'dispatch: loop {
        match pc {
            0x82DB2F28 => {
    //   block [0x82DB2F28..0x82DB2F34)
	// 82DB2F28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB2F2C: 386B191C  addi r3, r11, 0x191c
	ctx.r[3].s64 = ctx.r[11].s64 + 6428;
	// 82DB2F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2F38 size=116
    let mut pc: u32 = 0x82DB2F38;
    'dispatch: loop {
        match pc {
            0x82DB2F38 => {
    //   block [0x82DB2F38..0x82DB2FAC)
	// 82DB2F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2F40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB2F44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB2F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2F4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB2F50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB2F54: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82DB2F58: 4BFFEBE9  bl 0x82db1b40
	ctx.lr = 0x82DB2F5C;
	sub_82DB1B40(ctx, base);
	// 82DB2F5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DB2F60: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB2F64: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DB2F68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB2F6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB2F70: 419A0020  beq cr6, 0x82db2f90
	if ctx.cr[6].eq {
	pc = 0x82DB2F90; continue 'dispatch;
	}
	// 82DB2F74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2F78: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB2F7C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB2F80: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB2F84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB2F88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB2F8C: 4BFA233D  bl 0x82d552c8
	ctx.lr = 0x82DB2F90;
	sub_82D552C8(ctx, base);
	// 82DB2F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB2F94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB2F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB2F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB2FA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB2FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB2FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2FB0 size=4
    let mut pc: u32 = 0x82DB2FB0;
    'dispatch: loop {
        match pc {
            0x82DB2FB0 => {
    //   block [0x82DB2FB0..0x82DB2FB4)
	// 82DB2FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2FB8 size=12
    let mut pc: u32 = 0x82DB2FB8;
    'dispatch: loop {
        match pc {
            0x82DB2FB8 => {
    //   block [0x82DB2FB8..0x82DB2FC4)
	// 82DB2FB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB2FBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB2FC0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2FC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2FC4 size=8
    let mut pc: u32 = 0x82DB2FC4;
    'dispatch: loop {
        match pc {
            0x82DB2FC4 => {
    //   block [0x82DB2FC4..0x82DB2FCC)
	// 82DB2FC4: 48009A2C  b 0x82dbc9f0
	sub_82DBC9F0(ctx, base);
	return;
	// 82DB2FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB2FD0 size=20
    let mut pc: u32 = 0x82DB2FD0;
    'dispatch: loop {
        match pc {
            0x82DB2FD0 => {
    //   block [0x82DB2FD0..0x82DB2FE4)
	// 82DB2FD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2FD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2FD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB2FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB2FE0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB2FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB2FE8 size=44
    let mut pc: u32 = 0x82DB2FE8;
    'dispatch: loop {
        match pc {
            0x82DB2FE8 => {
    //   block [0x82DB2FE8..0x82DB3014)
	// 82DB2FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB2FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB2FF0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB2FF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB2FF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB2FFC: 480099F5  bl 0x82dbc9f0
	ctx.lr = 0x82DB3000;
	sub_82DBC9F0(ctx, base);
	// 82DB3000: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB3004: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DB3008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB300C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3018 size=4
    let mut pc: u32 = 0x82DB3018;
    'dispatch: loop {
        match pc {
            0x82DB3018 => {
    //   block [0x82DB3018..0x82DB301C)
	// 82DB3018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3020 size=20
    let mut pc: u32 = 0x82DB3020;
    'dispatch: loop {
        match pc {
            0x82DB3020 => {
    //   block [0x82DB3020..0x82DB3034)
	// 82DB3020: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3024: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3028: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB302C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3030: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3038 size=8
    let mut pc: u32 = 0x82DB3038;
    'dispatch: loop {
        match pc {
            0x82DB3038 => {
    //   block [0x82DB3038..0x82DB3040)
	// 82DB3038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB303C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3040 size=32
    let mut pc: u32 = 0x82DB3040;
    'dispatch: loop {
        match pc {
            0x82DB3040 => {
    //   block [0x82DB3040..0x82DB3060)
	// 82DB3040: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB3048: 396B1B08  addi r11, r11, 0x1b08
	ctx.r[11].s64 = ctx.r[11].s64 + 6920;
	// 82DB304C: 39200016  li r9, 0x16
	ctx.r[9].s64 = 22;
	// 82DB3050: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB3054: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB3058: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3060 size=12
    let mut pc: u32 = 0x82DB3060;
    'dispatch: loop {
        match pc {
            0x82DB3060 => {
    //   block [0x82DB3060..0x82DB306C)
	// 82DB3060: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3064: 386B1B08  addi r3, r11, 0x1b08
	ctx.r[3].s64 = ctx.r[11].s64 + 6920;
	// 82DB3068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3070 size=152
    let mut pc: u32 = 0x82DB3070;
    'dispatch: loop {
        match pc {
            0x82DB3070 => {
    //   block [0x82DB3070..0x82DB3108)
	// 82DB3070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB307C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3088: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB308C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB3090: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB3094: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB3098: 409A0020  bne cr6, 0x82db30b8
	if !ctx.cr[6].eq {
	pc = 0x82DB30B8; continue 'dispatch;
	}
	// 82DB309C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB30A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DB30A4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DB30A8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB30AC: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB30B0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB30B4: 4BFA2215  bl 0x82d552c8
	ctx.lr = 0x82DB30B8;
	sub_82D552C8(ctx, base);
	// 82DB30B8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DB30BC: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB30C0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DB30C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB30C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB30CC: 419A0020  beq cr6, 0x82db30ec
	if ctx.cr[6].eq {
	pc = 0x82DB30EC; continue 'dispatch;
	}
	// 82DB30D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB30D4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB30D8: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB30DC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB30E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB30E4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB30E8: 4BFA21E1  bl 0x82d552c8
	ctx.lr = 0x82DB30EC;
	sub_82D552C8(ctx, base);
	// 82DB30EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB30F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB30F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB30F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB30FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB3100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3108 size=20
    let mut pc: u32 = 0x82DB3108;
    'dispatch: loop {
        match pc {
            0x82DB3108 => {
    //   block [0x82DB3108..0x82DB311C)
	// 82DB3108: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB310C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3110: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3118: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3120 size=12
    let mut pc: u32 = 0x82DB3120;
    'dispatch: loop {
        match pc {
            0x82DB3120 => {
    //   block [0x82DB3120..0x82DB312C)
	// 82DB3120: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB3124: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3128: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB312C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB312C size=8
    let mut pc: u32 = 0x82DB312C;
    'dispatch: loop {
        match pc {
            0x82DB312C => {
    //   block [0x82DB312C..0x82DB3134)
	// 82DB312C: 4800A7B4  b 0x82dbd8e0
	sub_82DBD8E0(ctx, base);
	return;
	// 82DB3130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3138 size=44
    let mut pc: u32 = 0x82DB3138;
    'dispatch: loop {
        match pc {
            0x82DB3138 => {
    //   block [0x82DB3138..0x82DB3164)
	// 82DB3138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3140: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB314C: 4800A795  bl 0x82dbd8e0
	ctx.lr = 0x82DB3150;
	sub_82DBD8E0(ctx, base);
	// 82DB3150: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB3154: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DB3158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB315C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3168 size=4
    let mut pc: u32 = 0x82DB3168;
    'dispatch: loop {
        match pc {
            0x82DB3168 => {
    //   block [0x82DB3168..0x82DB316C)
	// 82DB3168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3170 size=4
    let mut pc: u32 = 0x82DB3170;
    'dispatch: loop {
        match pc {
            0x82DB3170 => {
    //   block [0x82DB3170..0x82DB3174)
	// 82DB3170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3178 size=20
    let mut pc: u32 = 0x82DB3178;
    'dispatch: loop {
        match pc {
            0x82DB3178 => {
    //   block [0x82DB3178..0x82DB318C)
	// 82DB3178: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB317C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3180: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3188: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3190 size=8
    let mut pc: u32 = 0x82DB3190;
    'dispatch: loop {
        match pc {
            0x82DB3190 => {
    //   block [0x82DB3190..0x82DB3198)
	// 82DB3190: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3194: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3198 size=32
    let mut pc: u32 = 0x82DB3198;
    'dispatch: loop {
        match pc {
            0x82DB3198 => {
    //   block [0x82DB3198..0x82DB31B8)
	// 82DB3198: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB319C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB31A0: 396B1E94  addi r11, r11, 0x1e94
	ctx.r[11].s64 = ctx.r[11].s64 + 7828;
	// 82DB31A4: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82DB31A8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB31AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB31B0: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB31B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB31B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB31B8 size=12
    let mut pc: u32 = 0x82DB31B8;
    'dispatch: loop {
        match pc {
            0x82DB31B8 => {
    //   block [0x82DB31B8..0x82DB31C4)
	// 82DB31B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB31BC: 386B1E94  addi r3, r11, 0x1e94
	ctx.r[3].s64 = ctx.r[11].s64 + 7828;
	// 82DB31C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB31C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB31C8 size=152
    let mut pc: u32 = 0x82DB31C8;
    'dispatch: loop {
        match pc {
            0x82DB31C8 => {
    //   block [0x82DB31C8..0x82DB3260)
	// 82DB31C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB31CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB31D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB31D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB31D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB31DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB31E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB31E4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB31E8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB31EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB31F0: 409A0020  bne cr6, 0x82db3210
	if !ctx.cr[6].eq {
	pc = 0x82DB3210; continue 'dispatch;
	}
	// 82DB31F4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB31F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DB31FC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DB3200: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB3204: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB3208: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB320C: 4BFA20BD  bl 0x82d552c8
	ctx.lr = 0x82DB3210;
	sub_82D552C8(ctx, base);
	// 82DB3210: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DB3214: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB3218: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DB321C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB3220: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB3224: 419A0020  beq cr6, 0x82db3244
	if ctx.cr[6].eq {
	pc = 0x82DB3244; continue 'dispatch;
	}
	// 82DB3228: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB322C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB3230: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB3234: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB3238: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB323C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB3240: 4BFA2089  bl 0x82d552c8
	ctx.lr = 0x82DB3244;
	sub_82D552C8(ctx, base);
	// 82DB3244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3248: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB324C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3254: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB3258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3260 size=20
    let mut pc: u32 = 0x82DB3260;
    'dispatch: loop {
        match pc {
            0x82DB3260 => {
    //   block [0x82DB3260..0x82DB3274)
	// 82DB3260: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3268: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB326C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3270: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3278 size=8
    let mut pc: u32 = 0x82DB3278;
    'dispatch: loop {
        match pc {
            0x82DB3278 => {
    //   block [0x82DB3278..0x82DB3280)
	// 82DB3278: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB327C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3280 size=24
    let mut pc: u32 = 0x82DB3280;
    'dispatch: loop {
        match pc {
            0x82DB3280 => {
    //   block [0x82DB3280..0x82DB3298)
	// 82DB3280: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3284: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB3288: 396B86C8  addi r11, r11, -0x7938
	ctx.r[11].s64 = ctx.r[11].s64 + -31032;
	// 82DB328C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB3290: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB3294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3298 size=12
    let mut pc: u32 = 0x82DB3298;
    'dispatch: loop {
        match pc {
            0x82DB3298 => {
    //   block [0x82DB3298..0x82DB32A4)
	// 82DB3298: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB329C: 386B86C8  addi r3, r11, -0x7938
	ctx.r[3].s64 = ctx.r[11].s64 + -31032;
	// 82DB32A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB32A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB32A8 size=124
    let mut pc: u32 = 0x82DB32A8;
    'dispatch: loop {
        match pc {
            0x82DB32A8 => {
    //   block [0x82DB32A8..0x82DB3324)
	// 82DB32A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB32AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB32B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB32B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB32B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB32BC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB32C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB32C4: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82DB32C8: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB32CC: 4800D23D  bl 0x82dc0508
	ctx.lr = 0x82DB32D0;
	sub_82DC0508(ctx, base);
	// 82DB32D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB32D4: 4801ADA5  bl 0x82dce078
	ctx.lr = 0x82DB32D8;
	sub_82DCE078(ctx, base);
	// 82DB32D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB32DC: 480183CD  bl 0x82dcb6a8
	ctx.lr = 0x82DB32E0;
	sub_82DCB6A8(ctx, base);
	// 82DB32E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB32E4: 4801774D  bl 0x82dcaa30
	ctx.lr = 0x82DB32E8;
	sub_82DCAA30(ctx, base);
	// 82DB32E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB32EC: 48016BFD  bl 0x82dc9ee8
	ctx.lr = 0x82DB32F0;
	sub_82DC9EE8(ctx, base);
	// 82DB32F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB32F4: 4805569D  bl 0x82e08990
	ctx.lr = 0x82DB32F8;
	sub_82E08990(ctx, base);
	// 82DB32F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB32FC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB3300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3304: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82DB3308: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB330C: 4800D1FD  bl 0x82dc0508
	ctx.lr = 0x82DB3310;
	sub_82DC0508(ctx, base);
	// 82DB3310: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB3314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB331C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3328 size=76
    let mut pc: u32 = 0x82DB3328;
    'dispatch: loop {
        match pc {
            0x82DB3328 => {
    //   block [0x82DB3328..0x82DB3374)
	// 82DB3328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB332C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3330: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3334: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB333C: 4801EB85  bl 0x82dd1ec0
	ctx.lr = 0x82DB3340;
	sub_82DD1EC0(ctx, base);
	// 82DB3340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3344: 4801D8C5  bl 0x82dd0c08
	ctx.lr = 0x82DB3348;
	sub_82DD0C08(ctx, base);
	// 82DB3348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB334C: 480561FD  bl 0x82e09548
	ctx.lr = 0x82DB3350;
	sub_82E09548(ctx, base);
	// 82DB3350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3354: 4801BC45  bl 0x82dcef98
	ctx.lr = 0x82DB3358;
	sub_82DCEF98(ctx, base);
	// 82DB3358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB335C: 48055A25  bl 0x82e08d80
	ctx.lr = 0x82DB3360;
	sub_82E08D80(ctx, base);
	// 82DB3360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB3364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB336C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3378 size=164
    let mut pc: u32 = 0x82DB3378;
    'dispatch: loop {
        match pc {
            0x82DB3378 => {
    //   block [0x82DB3378..0x82DB341C)
	// 82DB3378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB338C: 48027C85  bl 0x82ddb010
	ctx.lr = 0x82DB3390;
	sub_82DDB010(ctx, base);
	// 82DB3390: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB3394: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB3398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB339C: 480587ED  bl 0x82e0bb88
	ctx.lr = 0x82DB33A0;
	sub_82E0BB88(ctx, base);
	// 82DB33A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33A4: 48057E55  bl 0x82e0b1f8
	ctx.lr = 0x82DB33A8;
	sub_82E0B1F8(ctx, base);
	// 82DB33A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33AC: 48027A65  bl 0x82ddae10
	ctx.lr = 0x82DB33B0;
	sub_82DDAE10(ctx, base);
	// 82DB33B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33B4: 48056F7D  bl 0x82e0a330
	ctx.lr = 0x82DB33B8;
	sub_82E0A330(ctx, base);
	// 82DB33B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33BC: 4802719D  bl 0x82dda558
	ctx.lr = 0x82DB33C0;
	sub_82DDA558(ctx, base);
	// 82DB33C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33C4: 480263E5  bl 0x82dd97a8
	ctx.lr = 0x82DB33C8;
	sub_82DD97A8(ctx, base);
	// 82DB33C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33CC: 4802531D  bl 0x82dd86e8
	ctx.lr = 0x82DB33D0;
	sub_82DD86E8(ctx, base);
	// 82DB33D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33D4: 480245C5  bl 0x82dd7998
	ctx.lr = 0x82DB33D8;
	sub_82DD7998(ctx, base);
	// 82DB33D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33DC: 480235E5  bl 0x82dd69c0
	ctx.lr = 0x82DB33E0;
	sub_82DD69C0(ctx, base);
	// 82DB33E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33E4: 48022265  bl 0x82dd5648
	ctx.lr = 0x82DB33E8;
	sub_82DD5648(ctx, base);
	// 82DB33E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33EC: 48056A55  bl 0x82e09e40
	ctx.lr = 0x82DB33F0;
	sub_82E09E40(ctx, base);
	// 82DB33F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33F4: 48020B65  bl 0x82dd3f58
	ctx.lr = 0x82DB33F8;
	sub_82DD3F58(ctx, base);
	// 82DB33F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB33FC: 4801F835  bl 0x82dd2c30
	ctx.lr = 0x82DB3400;
	sub_82DD2C30(ctx, base);
	// 82DB3400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3404: 480176FD  bl 0x82dcab00
	ctx.lr = 0x82DB3408;
	sub_82DCAB00(ctx, base);
	// 82DB3408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB340C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3420 size=220
    let mut pc: u32 = 0x82DB3420;
    'dispatch: loop {
        match pc {
            0x82DB3420 => {
    //   block [0x82DB3420..0x82DB34FC)
	// 82DB3420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3424: 4BEF5FE9  bl 0x82ca940c
	ctx.lr = 0x82DB3428;
	sub_82CA93D0(ctx, base);
	// 82DB3428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB342C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3430: 4802CC91  bl 0x82de00c0
	ctx.lr = 0x82DB3434;
	sub_82DE00C0(ctx, base);
	// 82DB3434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3438: 4802C7E1  bl 0x82ddfc18
	ctx.lr = 0x82DB343C;
	sub_82DDFC18(ctx, base);
	// 82DB343C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3440: 4802B1B9  bl 0x82dde5f8
	ctx.lr = 0x82DB3444;
	sub_82DDE5F8(ctx, base);
	// 82DB3444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3448: 4BFFFE61  bl 0x82db32a8
	ctx.lr = 0x82DB344C;
	sub_82DB32A8(ctx, base);
	// 82DB344C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DB3450: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB3454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3458: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82DB345C: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3460: 4800D0A9  bl 0x82dc0508
	ctx.lr = 0x82DB3464;
	sub_82DC0508(ctx, base);
	// 82DB3464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3468: 4802A599  bl 0x82ddda00
	ctx.lr = 0x82DB346C;
	sub_82DDDA00(ctx, base);
	// 82DB346C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DB3470: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB3474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3478: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 82DB347C: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3480: 4800D089  bl 0x82dc0508
	ctx.lr = 0x82DB3484;
	sub_82DC0508(ctx, base);
	// 82DB3484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3488: 4801EA39  bl 0x82dd1ec0
	ctx.lr = 0x82DB348C;
	sub_82DD1EC0(ctx, base);
	// 82DB348C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3490: 4801D779  bl 0x82dd0c08
	ctx.lr = 0x82DB3494;
	sub_82DD0C08(ctx, base);
	// 82DB3494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3498: 480560B1  bl 0x82e09548
	ctx.lr = 0x82DB349C;
	sub_82E09548(ctx, base);
	// 82DB349C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34A0: 4801BAF9  bl 0x82dcef98
	ctx.lr = 0x82DB34A4;
	sub_82DCEF98(ctx, base);
	// 82DB34A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34A8: 480558D9  bl 0x82e08d80
	ctx.lr = 0x82DB34AC;
	sub_82E08D80(ctx, base);
	// 82DB34AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB34B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34B4: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82DB34B8: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB34BC: 4800D04D  bl 0x82dc0508
	ctx.lr = 0x82DB34C0;
	sub_82DC0508(ctx, base);
	// 82DB34C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34C4: 48059E65  bl 0x82e0d328
	ctx.lr = 0x82DB34C8;
	sub_82E0D328(ctx, base);
	// 82DB34C8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB34CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34D0: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 82DB34D4: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB34D8: 4800D031  bl 0x82dc0508
	ctx.lr = 0x82DB34DC;
	sub_82DC0508(ctx, base);
	// 82DB34DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34E0: 48028F49  bl 0x82ddc428
	ctx.lr = 0x82DB34E4;
	sub_82DDC428(ctx, base);
	// 82DB34E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34E8: 48028501  bl 0x82ddb9e8
	ctx.lr = 0x82DB34EC;
	sub_82DDB9E8(ctx, base);
	// 82DB34EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB34F0: 4BFFFE89  bl 0x82db3378
	ctx.lr = 0x82DB34F4;
	sub_82DB3378(ctx, base);
	// 82DB34F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB34F8: 4BEF5F64  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3500 size=4
    let mut pc: u32 = 0x82DB3500;
    'dispatch: loop {
        match pc {
            0x82DB3500 => {
    //   block [0x82DB3500..0x82DB3504)
	// 82DB3500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3508 size=20
    let mut pc: u32 = 0x82DB3508;
    'dispatch: loop {
        match pc {
            0x82DB3508 => {
    //   block [0x82DB3508..0x82DB351C)
	// 82DB3508: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB350C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3510: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3518: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3520 size=88
    let mut pc: u32 = 0x82DB3520;
    'dispatch: loop {
        match pc {
            0x82DB3520 => {
    //   block [0x82DB3520..0x82DB3578)
	// 82DB3520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB352C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3534: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DB3538: 419A002C  beq cr6, 0x82db3564
	if ctx.cr[6].eq {
	pc = 0x82DB3564; continue 'dispatch;
	}
	// 82DB353C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB3540: 480094B1  bl 0x82dbc9f0
	ctx.lr = 0x82DB3544;
	sub_82DBC9F0(ctx, base);
	// 82DB3544: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB354C: 396B2020  addi r11, r11, 0x2020
	ctx.r[11].s64 = ctx.r[11].s64 + 8224;
	// 82DB3550: 394A1FFC  addi r10, r10, 0x1ffc
	ctx.r[10].s64 = ctx.r[10].s64 + 8188;
	// 82DB3554: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82DB3558: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB355C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB3560: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB3564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB3568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB356C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3578 size=48
    let mut pc: u32 = 0x82DB3578;
    'dispatch: loop {
        match pc {
            0x82DB3578 => {
    //   block [0x82DB3578..0x82DB35A8)
	// 82DB3578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB357C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3580: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB358C: 48009465  bl 0x82dbc9f0
	ctx.lr = 0x82DB3590;
	sub_82DBC9F0(ctx, base);
	// 82DB3590: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3594: 386B2020  addi r3, r11, 0x2020
	ctx.r[3].s64 = ctx.r[11].s64 + 8224;
	// 82DB3598: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DB359C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB35A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB35A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB35A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB35A8 size=172
    let mut pc: u32 = 0x82DB35A8;
    'dispatch: loop {
        match pc {
            0x82DB35A8 => {
    //   block [0x82DB35A8..0x82DB3654)
	// 82DB35A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB35AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB35B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB35B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB35B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB35BC: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DB35C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB35C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB35C8: 409A0020  bne cr6, 0x82db35e8
	if !ctx.cr[6].eq {
	pc = 0x82DB35E8; continue 'dispatch;
	}
	// 82DB35CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB35D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DB35D4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DB35D8: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB35DC: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DB35E0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB35E4: 4BFA1CE5  bl 0x82d552c8
	ctx.lr = 0x82DB35E8;
	sub_82D552C8(ctx, base);
	// 82DB35E8: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DB35EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB35F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB35F4: 409A0024  bne cr6, 0x82db3618
	if !ctx.cr[6].eq {
	pc = 0x82DB3618; continue 'dispatch;
	}
	// 82DB35F8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB35FC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DB3600: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DB3604: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DB3608: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DB360C: 1CAB0038  mulli r5, r11, 0x38
	ctx.r[5].s64 = ctx.r[11].s64 * 56;
	// 82DB3610: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB3614: 4BFA1CB5  bl 0x82d552c8
	ctx.lr = 0x82DB3618;
	sub_82D552C8(ctx, base);
	// 82DB3618: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DB361C: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 82DB3620: 409A0008  bne cr6, 0x82db3628
	if !ctx.cr[6].eq {
	pc = 0x82DB3628; continue 'dispatch;
	}
	// 82DB3624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB3628: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB362C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DB3630: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB3634: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DB3638: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB363C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB3640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB3644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB364C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3658 size=8
    let mut pc: u32 = 0x82DB3658;
    'dispatch: loop {
        match pc {
            0x82DB3658 => {
    //   block [0x82DB3658..0x82DB3660)
	// 82DB3658: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DB365C: 48000004  b 0x82db3660
	sub_82DB3660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3660 size=100
    let mut pc: u32 = 0x82DB3660;
    'dispatch: loop {
        match pc {
            0x82DB3660 => {
    //   block [0x82DB3660..0x82DB36C4)
	// 82DB3660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB366C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3678: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB367C: 4BFFFF2D  bl 0x82db35a8
	ctx.lr = 0x82DB3680;
	sub_82DB35A8(ctx, base);
	// 82DB3680: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB3684: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB3688: 419A0020  beq cr6, 0x82db36a8
	if ctx.cr[6].eq {
	pc = 0x82DB36A8; continue 'dispatch;
	}
	// 82DB368C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3690: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB3694: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB3698: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB369C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB36A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB36A4: 4BFA1C25  bl 0x82d552c8
	ctx.lr = 0x82DB36A8;
	sub_82D552C8(ctx, base);
	// 82DB36A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB36AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB36B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB36B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB36B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB36BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB36C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB36C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB36C8 size=20
    let mut pc: u32 = 0x82DB36C8;
    'dispatch: loop {
        match pc {
            0x82DB36C8 => {
    //   block [0x82DB36C8..0x82DB36DC)
	// 82DB36C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB36CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB36D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB36D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB36D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB36E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB36E0 size=8
    let mut pc: u32 = 0x82DB36E0;
    'dispatch: loop {
        match pc {
            0x82DB36E0 => {
    //   block [0x82DB36E0..0x82DB36E8)
	// 82DB36E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB36E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB36E8 size=128
    let mut pc: u32 = 0x82DB36E8;
    'dispatch: loop {
        match pc {
            0x82DB36E8 => {
    //   block [0x82DB36E8..0x82DB3768)
	// 82DB36E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB36EC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DB36F0: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82DB36F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB36F8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB36FC: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82DB3700: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DB3704: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DB3708: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DB370C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DB3710: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82DB3714: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82DB3718: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB371C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DB3720: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DB3724: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82DB3728: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82DB372C: 38E72114  addi r7, r7, 0x2114
	ctx.r[7].s64 = ctx.r[7].s64 + 8468;
	// 82DB3730: 38C62108  addi r6, r6, 0x2108
	ctx.r[6].s64 = ctx.r[6].s64 + 8456;
	// 82DB3734: 38A520F4  addi r5, r5, 0x20f4
	ctx.r[5].s64 = ctx.r[5].s64 + 8436;
	// 82DB3738: 396B20E8  addi r11, r11, 0x20e8
	ctx.r[11].s64 = ctx.r[11].s64 + 8424;
	// 82DB373C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DB3740: 388420DC  addi r4, r4, 0x20dc
	ctx.r[4].s64 = ctx.r[4].s64 + 8412;
	// 82DB3744: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DB3748: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82DB374C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DB3750: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DB3754: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82DB3758: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB375C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DB3760: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DB3764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3768 size=12
    let mut pc: u32 = 0x82DB3768;
    'dispatch: loop {
        match pc {
            0x82DB3768 => {
    //   block [0x82DB3768..0x82DB3774)
	// 82DB3768: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB376C: 386B2114  addi r3, r11, 0x2114
	ctx.r[3].s64 = ctx.r[11].s64 + 8468;
	// 82DB3770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3778 size=100
    let mut pc: u32 = 0x82DB3778;
    'dispatch: loop {
        match pc {
            0x82DB3778 => {
    //   block [0x82DB3778..0x82DB37DC)
	// 82DB3778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB377C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB3784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB378C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB3794: 4800391D  bl 0x82db70b0
	ctx.lr = 0x82DB3798;
	sub_82DB70B0(ctx, base);
	// 82DB3798: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB379C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB37A0: 419A0020  beq cr6, 0x82db37c0
	if ctx.cr[6].eq {
	pc = 0x82DB37C0; continue 'dispatch;
	}
	// 82DB37A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB37A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB37AC: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB37B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB37B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB37B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB37BC: 4BFA1B0D  bl 0x82d552c8
	ctx.lr = 0x82DB37C0;
	sub_82D552C8(ctx, base);
	// 82DB37C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB37C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB37C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB37CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB37D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB37D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB37D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB37E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB37E0 size=8
    let mut pc: u32 = 0x82DB37E0;
    'dispatch: loop {
        match pc {
            0x82DB37E0 => {
    //   block [0x82DB37E0..0x82DB37E8)
	// 82DB37E0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82DB37E4: 4BFFFF94  b 0x82db3778
	sub_82DB3778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB37E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB37E8 size=8
    let mut pc: u32 = 0x82DB37E8;
    'dispatch: loop {
        match pc {
            0x82DB37E8 => {
    //   block [0x82DB37E8..0x82DB37F0)
	// 82DB37E8: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82DB37EC: 4BFFFF8C  b 0x82db3778
	sub_82DB3778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB37F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB37F0 size=8
    let mut pc: u32 = 0x82DB37F0;
    'dispatch: loop {
        match pc {
            0x82DB37F0 => {
    //   block [0x82DB37F0..0x82DB37F8)
	// 82DB37F0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DB37F4: 4BFFFF84  b 0x82db3778
	sub_82DB3778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB37F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB37F8 size=8
    let mut pc: u32 = 0x82DB37F8;
    'dispatch: loop {
        match pc {
            0x82DB37F8 => {
    //   block [0x82DB37F8..0x82DB3800)
	// 82DB37F8: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82DB37FC: 4BFFFF7C  b 0x82db3778
	sub_82DB3778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3800 size=20
    let mut pc: u32 = 0x82DB3800;
    'dispatch: loop {
        match pc {
            0x82DB3800 => {
    //   block [0x82DB3800..0x82DB3814)
	// 82DB3800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3808: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB380C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3810: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3818 size=8
    let mut pc: u32 = 0x82DB3818;
    'dispatch: loop {
        match pc {
            0x82DB3818 => {
    //   block [0x82DB3818..0x82DB3820)
	// 82DB3818: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB381C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3820 size=80
    let mut pc: u32 = 0x82DB3820;
    'dispatch: loop {
        match pc {
            0x82DB3820 => {
    //   block [0x82DB3820..0x82DB3870)
	// 82DB3820: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB3824: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB3828: 396B2894  addi r11, r11, 0x2894
	ctx.r[11].s64 = ctx.r[11].s64 + 10388;
	// 82DB382C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB3830: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DB3834: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82DB3838: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DB383C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB3840: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB3844: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DB3848: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DB384C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB3850: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3870 size=12
    let mut pc: u32 = 0x82DB3870;
    'dispatch: loop {
        match pc {
            0x82DB3870 => {
    //   block [0x82DB3870..0x82DB387C)
	// 82DB3870: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB3874: 386B2894  addi r3, r11, 0x2894
	ctx.r[3].s64 = ctx.r[11].s64 + 10388;
	// 82DB3878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3880 size=124
    let mut pc: u32 = 0x82DB3880;
    'dispatch: loop {
        match pc {
            0x82DB3880 => {
    //   block [0x82DB3880..0x82DB38FC)
	// 82DB3880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB388C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3894: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3898: 396B21FC  addi r11, r11, 0x21fc
	ctx.r[11].s64 = ctx.r[11].s64 + 8700;
	// 82DB389C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB38A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB38A4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB38A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB38AC: 419A0030  beq cr6, 0x82db38dc
	if ctx.cr[6].eq {
	pc = 0x82DB38DC; continue 'dispatch;
	}
	// 82DB38B0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB38B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB38B8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DB38BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB38C0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB38C4: 409A0018  bne cr6, 0x82db38dc
	if !ctx.cr[6].eq {
	pc = 0x82DB38DC; continue 'dispatch;
	}
	// 82DB38C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB38CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB38D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB38D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB38D8: 4E800421  bctrl
	ctx.lr = 0x82DB38DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB38DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DB38E0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DB38E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB38E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB38EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB38F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB38F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB38F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3900 size=100
    let mut pc: u32 = 0x82DB3900;
    'dispatch: loop {
        match pc {
            0x82DB3900 => {
    //   block [0x82DB3900..0x82DB3964)
	// 82DB3900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB390C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB391C: 4BFFFF65  bl 0x82db3880
	ctx.lr = 0x82DB3920;
	sub_82DB3880(ctx, base);
	// 82DB3920: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB3924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB3928: 419A0020  beq cr6, 0x82db3948
	if ctx.cr[6].eq {
	pc = 0x82DB3948; continue 'dispatch;
	}
	// 82DB392C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3930: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB3934: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB3938: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB393C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB3940: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB3944: 4BFA1985  bl 0x82d552c8
	ctx.lr = 0x82DB3948;
	sub_82D552C8(ctx, base);
	// 82DB3948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB394C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB3950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3958: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB395C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3968 size=100
    let mut pc: u32 = 0x82DB3968;
    'dispatch: loop {
        match pc {
            0x82DB3968 => {
    //   block [0x82DB3968..0x82DB39CC)
	// 82DB3968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3970: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB3974: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3978: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB397C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3980: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB3984: 48002E95  bl 0x82db6818
	ctx.lr = 0x82DB3988;
	sub_82DB6818(ctx, base);
	// 82DB3988: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB398C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB3990: 419A0020  beq cr6, 0x82db39b0
	if ctx.cr[6].eq {
	pc = 0x82DB39B0; continue 'dispatch;
	}
	// 82DB3994: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3998: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB399C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB39A0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB39A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB39A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB39AC: 4BFA191D  bl 0x82d552c8
	ctx.lr = 0x82DB39B0;
	sub_82D552C8(ctx, base);
	// 82DB39B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB39B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB39B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB39BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB39C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB39C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB39C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB39D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB39D0 size=20
    let mut pc: u32 = 0x82DB39D0;
    'dispatch: loop {
        match pc {
            0x82DB39D0 => {
    //   block [0x82DB39D0..0x82DB39E4)
	// 82DB39D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB39D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB39D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB39DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB39E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB39E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB39E8 size=12
    let mut pc: u32 = 0x82DB39E8;
    'dispatch: loop {
        match pc {
            0x82DB39E8 => {
    //   block [0x82DB39E8..0x82DB39F4)
	// 82DB39E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB39EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB39F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB39F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB39F4 size=8
    let mut pc: u32 = 0x82DB39F4;
    'dispatch: loop {
        match pc {
            0x82DB39F4 => {
    //   block [0x82DB39F4..0x82DB39FC)
	// 82DB39F4: 4802D0DC  b 0x82de0ad0
	sub_82DE0AD0(ctx, base);
	return;
	// 82DB39F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3A00 size=44
    let mut pc: u32 = 0x82DB3A00;
    'dispatch: loop {
        match pc {
            0x82DB3A00 => {
    //   block [0x82DB3A00..0x82DB3A2C)
	// 82DB3A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3A08: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3A0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3A10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB3A14: 4802D0BD  bl 0x82de0ad0
	ctx.lr = 0x82DB3A18;
	sub_82DE0AD0(ctx, base);
	// 82DB3A18: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB3A1C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DB3A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3A30 size=20
    let mut pc: u32 = 0x82DB3A30;
    'dispatch: loop {
        match pc {
            0x82DB3A30 => {
    //   block [0x82DB3A30..0x82DB3A44)
	// 82DB3A30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3A34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3A38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3A3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3A40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3A48 size=8
    let mut pc: u32 = 0x82DB3A48;
    'dispatch: loop {
        match pc {
            0x82DB3A48 => {
    //   block [0x82DB3A48..0x82DB3A50)
	// 82DB3A48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3A4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3A50 size=32
    let mut pc: u32 = 0x82DB3A50;
    'dispatch: loop {
        match pc {
            0x82DB3A50 => {
    //   block [0x82DB3A50..0x82DB3A70)
	// 82DB3A50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3A54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB3A58: 396B2314  addi r11, r11, 0x2314
	ctx.r[11].s64 = ctx.r[11].s64 + 8980;
	// 82DB3A5C: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82DB3A60: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB3A64: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB3A68: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB3A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3A70 size=12
    let mut pc: u32 = 0x82DB3A70;
    'dispatch: loop {
        match pc {
            0x82DB3A70 => {
    //   block [0x82DB3A70..0x82DB3A7C)
	// 82DB3A70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3A74: 386B2314  addi r3, r11, 0x2314
	ctx.r[3].s64 = ctx.r[11].s64 + 8980;
	// 82DB3A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3A80 size=20
    let mut pc: u32 = 0x82DB3A80;
    'dispatch: loop {
        match pc {
            0x82DB3A80 => {
    //   block [0x82DB3A80..0x82DB3A94)
	// 82DB3A80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3A84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3A88: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3A90: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3A98 size=140
    let mut pc: u32 = 0x82DB3A98;
    'dispatch: loop {
        match pc {
            0x82DB3A98 => {
    //   block [0x82DB3A98..0x82DB3B24)
	// 82DB3A98: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DB3A9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3AA0: 419A007C  beq cr6, 0x82db3b1c
	if ctx.cr[6].eq {
	pc = 0x82DB3B1C; continue 'dispatch;
	}
	// 82DB3AA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3AA8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82DB3AAC: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82DB3AB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB3AB4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB3AB8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DB3ABC: B3E30006  sth r31, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[31].u16 ) };
	// 82DB3AC0: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82DB3AC4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DB3AC8: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DB3ACC: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82DB3AD0: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82DB3AD4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3AD8: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DB3ADC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DB3AE0: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82DB3AE4: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82DB3AE8: 38E786B8  addi r7, r7, -0x7948
	ctx.r[7].s64 = ctx.r[7].s64 + -31048;
	// 82DB3AEC: 38C686AC  addi r6, r6, -0x7954
	ctx.r[6].s64 = ctx.r[6].s64 + -31060;
	// 82DB3AF0: 38A58698  addi r5, r5, -0x7968
	ctx.r[5].s64 = ctx.r[5].s64 + -31080;
	// 82DB3AF4: 396B868C  addi r11, r11, -0x7974
	ctx.r[11].s64 = ctx.r[11].s64 + -31092;
	// 82DB3AF8: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DB3AFC: 38848680  addi r4, r4, -0x7980
	ctx.r[4].s64 = ctx.r[4].s64 + -31104;
	// 82DB3B00: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DB3B04: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DB3B08: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DB3B0C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82DB3B10: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB3B14: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DB3B18: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82DB3B1C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82DB3B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3B28 size=12
    let mut pc: u32 = 0x82DB3B28;
    'dispatch: loop {
        match pc {
            0x82DB3B28 => {
    //   block [0x82DB3B28..0x82DB3B34)
	// 82DB3B28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3B2C: 386B86B8  addi r3, r11, -0x7948
	ctx.r[3].s64 = ctx.r[11].s64 + -31048;
	// 82DB3B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3B38 size=20
    let mut pc: u32 = 0x82DB3B38;
    'dispatch: loop {
        match pc {
            0x82DB3B38 => {
    //   block [0x82DB3B38..0x82DB3B4C)
	// 82DB3B38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3B3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3B40: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3B44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3B48: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3B50 size=8
    let mut pc: u32 = 0x82DB3B50;
    'dispatch: loop {
        match pc {
            0x82DB3B50 => {
    //   block [0x82DB3B50..0x82DB3B58)
	// 82DB3B50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3B54: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3B58 size=32
    let mut pc: u32 = 0x82DB3B58;
    'dispatch: loop {
        match pc {
            0x82DB3B58 => {
    //   block [0x82DB3B58..0x82DB3B78)
	// 82DB3B58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3B5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB3B60: 396B23C4  addi r11, r11, 0x23c4
	ctx.r[11].s64 = ctx.r[11].s64 + 9156;
	// 82DB3B64: 3920001B  li r9, 0x1b
	ctx.r[9].s64 = 27;
	// 82DB3B68: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB3B6C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB3B70: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB3B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3B78 size=12
    let mut pc: u32 = 0x82DB3B78;
    'dispatch: loop {
        match pc {
            0x82DB3B78 => {
    //   block [0x82DB3B78..0x82DB3B84)
	// 82DB3B78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3B7C: 386B23C4  addi r3, r11, 0x23c4
	ctx.r[3].s64 = ctx.r[11].s64 + 9156;
	// 82DB3B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3B88 size=72
    let mut pc: u32 = 0x82DB3B88;
    'dispatch: loop {
        match pc {
            0x82DB3B88 => {
    //   block [0x82DB3B88..0x82DB3BD0)
	// 82DB3B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3B98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3BA0: 396B2454  addi r11, r11, 0x2454
	ctx.r[11].s64 = ctx.r[11].s64 + 9300;
	// 82DB3BA4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DB3BA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB3BAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB3BB0: 419A000C  beq cr6, 0x82db3bbc
	if ctx.cr[6].eq {
	pc = 0x82DB3BBC; continue 'dispatch;
	}
	// 82DB3BB4: 4BA91BFD  bl 0x828457b0
	ctx.lr = 0x82DB3BB8;
	sub_828457B0(ctx, base);
	// 82DB3BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3BBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB3BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3BC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3BD0 size=20
    let mut pc: u32 = 0x82DB3BD0;
    'dispatch: loop {
        match pc {
            0x82DB3BD0 => {
    //   block [0x82DB3BD0..0x82DB3BE4)
	// 82DB3BD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3BD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB3BD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3BDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3BE0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3BE8 size=8
    let mut pc: u32 = 0x82DB3BE8;
    'dispatch: loop {
        match pc {
            0x82DB3BE8 => {
    //   block [0x82DB3BE8..0x82DB3BF0)
	// 82DB3BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3BEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3BF0 size=48
    let mut pc: u32 = 0x82DB3BF0;
    'dispatch: loop {
        match pc {
            0x82DB3BF0 => {
    //   block [0x82DB3BF0..0x82DB3C20)
	// 82DB3BF0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3BF4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB3BF8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB3BFC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DB3C00: 396B2454  addi r11, r11, 0x2454
	ctx.r[11].s64 = ctx.r[11].s64 + 9300;
	// 82DB3C04: 394A2474  addi r10, r10, 0x2474
	ctx.r[10].s64 = ctx.r[10].s64 + 9332;
	// 82DB3C08: 39292464  addi r9, r9, 0x2464
	ctx.r[9].s64 = ctx.r[9].s64 + 9316;
	// 82DB3C0C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DB3C10: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DB3C14: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB3C18: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DB3C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3C20 size=12
    let mut pc: u32 = 0x82DB3C20;
    'dispatch: loop {
        match pc {
            0x82DB3C20 => {
    //   block [0x82DB3C20..0x82DB3C2C)
	// 82DB3C20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3C24: 386B2474  addi r3, r11, 0x2474
	ctx.r[3].s64 = ctx.r[11].s64 + 9332;
	// 82DB3C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3C30 size=100
    let mut pc: u32 = 0x82DB3C30;
    'dispatch: loop {
        match pc {
            0x82DB3C30 => {
    //   block [0x82DB3C30..0x82DB3C94)
	// 82DB3C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3C38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB3C3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3C48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB3C4C: 4802E765  bl 0x82de23b0
	ctx.lr = 0x82DB3C50;
	sub_82DE23B0(ctx, base);
	// 82DB3C50: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB3C54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB3C58: 419A0020  beq cr6, 0x82db3c78
	if ctx.cr[6].eq {
	pc = 0x82DB3C78; continue 'dispatch;
	}
	// 82DB3C5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3C60: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB3C64: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82DB3C68: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB3C6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB3C70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB3C74: 4BFA1655  bl 0x82d552c8
	ctx.lr = 0x82DB3C78;
	sub_82D552C8(ctx, base);
	// 82DB3C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3C7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB3C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3C88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB3C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3C98 size=8
    let mut pc: u32 = 0x82DB3C98;
    'dispatch: loop {
        match pc {
            0x82DB3C98 => {
    //   block [0x82DB3C98..0x82DB3CA0)
	// 82DB3C98: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82DB3C9C: 4BFFFF94  b 0x82db3c30
	sub_82DB3C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3CA0 size=8
    let mut pc: u32 = 0x82DB3CA0;
    'dispatch: loop {
        match pc {
            0x82DB3CA0 => {
    //   block [0x82DB3CA0..0x82DB3CA8)
	// 82DB3CA0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82DB3CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3CA8 size=108
    let mut pc: u32 = 0x82DB3CA8;
    'dispatch: loop {
        match pc {
            0x82DB3CA8 => {
    //   block [0x82DB3CA8..0x82DB3D14)
	// 82DB3CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3CB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB3CB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3CB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3CBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB3CC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB3CC4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB3CC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3CCC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DB3CD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3CD4: 4E800421  bctrl
	ctx.lr = 0x82DB3CD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB3CD8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB3D18 size=128
    let mut pc: u32 = 0x82DB3D18;
    'dispatch: loop {
        match pc {
            0x82DB3D18 => {
    //   block [0x82DB3D18..0x82DB3D98)
	// 82DB3D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3D20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3D24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3D28: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82DB3D2C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3D30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB3D34: 388B6E60  addi r4, r11, 0x6e60
	ctx.r[4].s64 = ctx.r[11].s64 + 28256;
	// 82DB3D38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB3D3C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DB3D40: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB3D44: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB3D48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3D4C: 4E800421  bctrl
	ctx.lr = 0x82DB3D50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB3D50: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DB3D54: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB3D98 size=220
    let mut pc: u32 = 0x82DB3D98;
    'dispatch: loop {
        match pc {
            0x82DB3D98 => {
    //   block [0x82DB3D98..0x82DB3E74)
	// 82DB3D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB3DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3DA8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3DAC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DB3DB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3DB4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DB3DB8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DB3DBC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DB3DC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DB3DC4: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82DB3DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB3DCC: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB3DD0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB3DD4: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82DB3DD8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DB3DDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB3DE0: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82DB3DE4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DB3DE8: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DB3DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3DF0: 4E800421  bctrl
	ctx.lr = 0x82DB3DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB3DF4: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB3DF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB3DFC: 419A0060  beq cr6, 0x82db3e5c
	if ctx.cr[6].eq {
	pc = 0x82DB3E5C; continue 'dispatch;
	}
	// 82DB3E00: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DB3E04: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB3E08: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DB3E0C: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3E10: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB3E14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DB3E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3E78 size=48
    let mut pc: u32 = 0x82DB3E78;
    'dispatch: loop {
        match pc {
            0x82DB3E78 => {
    //   block [0x82DB3E78..0x82DB3EA8)
	// 82DB3E78: 548A0000  rlwinm r10, r4, 0, 0, 0
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB3E7C: 548B007E  clrlwi r11, r4, 1
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82DB3E80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB3E84: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB3E88: 214A0020  subfic r10, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[10].s64;
	// 82DB3E8C: 7D6B5430  srw r11, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB3E90: 409A0018  bne cr6, 0x82db3ea8
	if !ctx.cr[6].eq {
		sub_82DB3EA8(ctx, base);
		return;
	}
	// 82DB3E94: 81430040  lwz r10, 0x40(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB3E98: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB3E9C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB3EA0: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB3EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3EA8 size=28
    let mut pc: u32 = 0x82DB3EA8;
    'dispatch: loop {
        match pc {
            0x82DB3EA8 => {
    //   block [0x82DB3EA8..0x82DB3EC4)
	// 82DB3EA8: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DB3EAC: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DB3EB0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DB3EB4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB3EB8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB3EBC: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB3EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3EC8 size=16
    let mut pc: u32 = 0x82DB3EC8;
    'dispatch: loop {
        match pc {
            0x82DB3EC8 => {
    //   block [0x82DB3EC8..0x82DB3ED8)
	// 82DB3EC8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3ED8 size=4
    let mut pc: u32 = 0x82DB3ED8;
    'dispatch: loop {
        match pc {
            0x82DB3ED8 => {
    //   block [0x82DB3ED8..0x82DB3EDC)
	// 82DB3ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB3EE0 size=4
    let mut pc: u32 = 0x82DB3EE0;
    'dispatch: loop {
        match pc {
            0x82DB3EE0 => {
    //   block [0x82DB3EE0..0x82DB3EE4)
	// 82DB3EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB3EE8 size=224
    let mut pc: u32 = 0x82DB3EE8;
    'dispatch: loop {
        match pc {
            0x82DB3EE8 => {
    //   block [0x82DB3EE8..0x82DB3FC8)
	// 82DB3EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB3EF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB3EF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB3EF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3EFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB3F00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3F04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB3F08: 388B29E8  addi r4, r11, 0x29e8
	ctx.r[4].s64 = ctx.r[11].s64 + 10728;
	// 82DB3F0C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DB3F10: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3F14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB3F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3F1C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB3F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3F24: 4E800421  bctrl
	ctx.lr = 0x82DB3F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB3F28: 80DE0050  lwz r6, 0x50(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB3F2C: 397E0070  addi r11, r30, 0x70
	ctx.r[11].s64 = ctx.r[30].s64 + 112;
	// 82DB3F30: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DB3F34: 419A0030  beq cr6, 0x82db3f64
	if ctx.cr[6].eq {
	pc = 0x82DB3F64; continue 'dispatch;
	}
	// 82DB3F38: 815E0054  lwz r10, 0x54(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB3F3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB3F40: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3F44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB3F48: 55483830  slwi r8, r10, 7
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DB3F4C: 388B29D4  addi r4, r11, 0x29d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10708;
	// 82DB3F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3F54: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82DB3F58: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB3F5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3F60: 4E800421  bctrl
	ctx.lr = 0x82DB3F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB3F64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB3F68: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DB3F6C: 80DE0058  lwz r6, 0x58(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DB3F70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB3F74: 388A29C4  addi r4, r10, 0x29c4
	ctx.r[4].s64 = ctx.r[10].s64 + 10692;
	// 82DB3F78: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3F80: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB3F84: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB3F88: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB3F8C: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DB3F90: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82DB3F94: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DB3F98: 4E800421  bctrl
	ctx.lr = 0x82DB3F9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB3F9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB3FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB3FA4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB3FA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB3FAC: 4E800421  bctrl
	ctx.lr = 0x82DB3FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB3FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB3FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3FBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB3FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB3FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB3FC8 size=228
    let mut pc: u32 = 0x82DB3FC8;
    'dispatch: loop {
        match pc {
            0x82DB3FC8 => {
    //   block [0x82DB3FC8..0x82DB40AC)
	// 82DB3FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB3FCC: 4BEF5431  bl 0x82ca93fc
	ctx.lr = 0x82DB3FD0;
	sub_82CA93D0(ctx, base);
	// 82DB3FD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB3FD4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DB3FD8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DB3FDC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DB3FE0: 817B005C  lwz r11, 0x5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DB3FE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB3FE8: 409900AC  ble cr6, 0x82db4094
	if !ctx.cr[6].gt {
	pc = 0x82DB4094; continue 'dispatch;
	}
	// 82DB3FEC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DB3FF0: 817B0058  lwz r11, 0x58(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DB3FF4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DB3FF8: 7FEBE214  add r31, r11, r28
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DB3FFC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB4000: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB4004: 4099007C  ble cr6, 0x82db4080
	if !ctx.cr[6].gt {
	pc = 0x82DB4080; continue 'dispatch;
	}
	// 82DB4008: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB400C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB4010: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82DB4014: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DB4018: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DB401C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4020: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB4024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB4028: 4E800421  bctrl
	ctx.lr = 0x82DB402C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB402C: 897F0020  lbz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB4030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB4034: 409A000C  bne cr6, 0x82db4040
	if !ctx.cr[6].eq {
	pc = 0x82DB4040; continue 'dispatch;
	}
	// 82DB4038: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 82DB403C: 48000020  b 0x82db405c
	pc = 0x82DB405C; continue 'dispatch;
	// 82DB4040: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DB4044: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DB4048: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82DB404C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82DB4050: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB4054: 556B06F2  rlwinm r11, r11, 0, 0x1b, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB4058: 396B01D0  addi r11, r11, 0x1d0
	ctx.r[11].s64 = ctx.r[11].s64 + 464;
	// 82DB405C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DB4060: 41980040  blt cr6, 0x82db40a0
	if ctx.cr[6].lt {
	pc = 0x82DB40A0; continue 'dispatch;
	}
	// 82DB4064: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB4068: 41990038  bgt cr6, 0x82db40a0
	if ctx.cr[6].gt {
	pc = 0x82DB40A0; continue 'dispatch;
	}
	// 82DB406C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB4070: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DB4074: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DB4078: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB407C: 4198FF90  blt cr6, 0x82db400c
	if ctx.cr[6].lt {
	pc = 0x82DB400C; continue 'dispatch;
	}
	// 82DB4080: 817B005C  lwz r11, 0x5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DB4084: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82DB4088: 3B9C0090  addi r28, r28, 0x90
	ctx.r[28].s64 = ctx.r[28].s64 + 144;
	// 82DB408C: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB4090: 4198FF60  blt cr6, 0x82db3ff0
	if ctx.cr[6].lt {
	pc = 0x82DB3FF0; continue 'dispatch;
	}
	// 82DB4094: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82DB4098: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DB409C: 4BEF53B0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82DB40A0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DB40A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DB40A8: 4BEF53A4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB40B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB40B0 size=60
    let mut pc: u32 = 0x82DB40B0;
    'dispatch: loop {
        match pc {
            0x82DB40B0 => {
    //   block [0x82DB40B0..0x82DB40EC)
	// 82DB40B0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB40B4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DB40B8: 54880000  rlwinm r8, r4, 0, 0, 0
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB40BC: 5489007E  clrlwi r9, r4, 1
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82DB40C0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DB40C4: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DB40C8: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB40CC: 7D672038  and r7, r11, r4
	ctx.r[7].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82DB40D0: 7D2B4430  srw r11, r9, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB40D4: 409A0024  bne cr6, 0x82db40f8
	if !ctx.cr[6].eq {
		sub_82DB40F8(ctx, base);
		return;
	}
	// 82DB40D8: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB40DC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82DB40E0: 409A000C  bne cr6, 0x82db40ec
	if !ctx.cr[6].eq {
		sub_82DB40EC(ctx, base);
		return;
	}
	// 82DB40E4: 39630070  addi r11, r3, 0x70
	ctx.r[11].s64 = ctx.r[3].s64 + 112;
	// 82DB40E8: 48000024  b 0x82db410c
	sub_82DB40F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB40EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB40EC size=12
    let mut pc: u32 = 0x82DB40EC;
    'dispatch: loop {
        match pc {
            0x82DB40EC => {
    //   block [0x82DB40EC..0x82DB40F8)
	// 82DB40EC: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB40F0: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB40F4: 48000014  b 0x82db4108
	sub_82DB40F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB40F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB40F8 size=80
    let mut pc: u32 = 0x82DB40F8;
    'dispatch: loop {
        match pc {
            0x82DB40F8 => {
    //   block [0x82DB40F8..0x82DB4148)
	// 82DB40F8: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DB40FC: 81430058  lwz r10, 0x58(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DB4100: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DB4104: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB4108: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB410C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB4110: A10B0002  lhz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DB4114: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB4118: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB411C: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82DB4120: 419A0038  beq cr6, 0x82db4158
	if ctx.cr[6].eq {
		sub_82DB4158(ctx, base);
		return;
	}
	// 82DB4124: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DB4128: A0CB0008  lhz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB412C: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82DB4130: 7D6639D6  mullw r11, r6, r7
	ctx.r[11].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82DB4134: 409A0014  bne cr6, 0x82db4148
	if !ctx.cr[6].eq {
		sub_82DB4148(ctx, base);
		return;
	}
	// 82DB4138: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB413C: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82DB4140: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DB4144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB4148 size=16
    let mut pc: u32 = 0x82DB4148;
    'dispatch: loop {
        match pc {
            0x82DB4148 => {
    //   block [0x82DB4148..0x82DB4158)
	// 82DB4148: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB414C: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82DB4150: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DB4154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB4158 size=8
    let mut pc: u32 = 0x82DB4158;
    'dispatch: loop {
        match pc {
            0x82DB4158 => {
    //   block [0x82DB4158..0x82DB4160)
	// 82DB4158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DB415C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB4160 size=168
    let mut pc: u32 = 0x82DB4160;
    'dispatch: loop {
        match pc {
            0x82DB4160 => {
    //   block [0x82DB4160..0x82DB4208)
	// 82DB4160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB4168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB416C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4170: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB4174: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DB4178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB417C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB4180: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DB4184: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB4188: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DB418C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB4190: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB4194: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DB4198: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DB419C: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DB41A0: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB41A4: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DB41A8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DB41AC: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB41B0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DB41B4: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB41B8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DB41BC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DB41C0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB4208 size=264
    let mut pc: u32 = 0x82DB4208;
    'dispatch: loop {
        match pc {
            0x82DB4208 => {
    //   block [0x82DB4208..0x82DB4310)
	// 82DB4208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB420C: 4BEF51F1  bl 0x82ca93fc
	ctx.lr = 0x82DB4210;
	sub_82CA93D0(ctx, base);
	// 82DB4210: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4214: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB4218: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DB421C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DB4220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB4224: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DB4228: C00B0C64  lfs f0, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB422C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB4230: D01B0000  stfs f0, 0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DB4234: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB4310 size=212
    let mut pc: u32 = 0x82DB4310;
    'dispatch: loop {
        match pc {
            0x82DB4310 => {
    //   block [0x82DB4310..0x82DB43E4)
	// 82DB4310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4314: 4BEF50ED  bl 0x82ca9400
	ctx.lr = 0x82DB4318;
	sub_82CA93D0(ctx, base);
	// 82DB4318: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82DB431C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4320: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB4324: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB4328: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB432C: 3B7F0010  addi r27, r31, 0x10
	ctx.r[27].s64 = ctx.r[31].s64 + 16;
	// 82DB4330: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DB4334: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB4338: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB433C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB43E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB43E8 size=408
    let mut pc: u32 = 0x82DB43E8;
    'dispatch: loop {
        match pc {
            0x82DB43E8 => {
    //   block [0x82DB43E8..0x82DB4580)
	// 82DB43E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB43EC: 4BEF501D  bl 0x82ca9408
	ctx.lr = 0x82DB43F0;
	sub_82CA93D0(ctx, base);
	// 82DB43F0: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB4580 size=420
    let mut pc: u32 = 0x82DB4580;
    'dispatch: loop {
        match pc {
            0x82DB4580 => {
    //   block [0x82DB4580..0x82DB4724)
	// 82DB4580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4584: 4BEF4E79  bl 0x82ca93fc
	ctx.lr = 0x82DB4588;
	sub_82CA93D0(ctx, base);
	// 82DB4588: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB458C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DB4590: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB4598: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB459C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82DB45A0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB45A4: 579D103A  slwi r29, r28, 2
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DB45A8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DB45AC: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82DB45B0: B3DF0008  sth r30, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u16 ) };
	// 82DB45B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB45B8: 9B3F0000  stb r25, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82DB45BC: 9B3F0001  stb r25, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[25].u8 ) };
	// 82DB45C0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DB45C4: B3DF0002  sth r30, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[30].u16 ) };
	// 82DB45C8: B33F000A  sth r25, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[25].u16 ) };
	// 82DB45CC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82DB45D0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DB45D4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DB45D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB45DC: 4BFA0C6D  bl 0x82d55248
	ctx.lr = 0x82DB45E0;
	sub_82D55248(ctx, base);
	// 82DB45E0: 57A9F0BE  srwi r9, r29, 2
	ctx.r[9].u32 = ctx.r[29].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DB45E4: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82DB45E8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82DB45EC: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82DB45F0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DB45F4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DB45F8: 40990020  ble cr6, 0x82db4618
	if !ctx.cr[6].gt {
	pc = 0x82DB4618; continue 'dispatch;
	}
	// 82DB45FC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4600: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DB4604: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DB4608: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DB460C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DB4610: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DB4614: 409AFFE8  bne cr6, 0x82db45fc
	if !ctx.cr[6].eq {
	pc = 0x82DB45FC; continue 'dispatch;
	}
	// 82DB4618: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DB461C: 40990038  ble cr6, 0x82db4654
	if !ctx.cr[6].gt {
	pc = 0x82DB4654; continue 'dispatch;
	}
	// 82DB4620: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82DB4624: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB4628: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB462C: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB4630: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DB4634: 419A0010  beq cr6, 0x82db4644
	if ctx.cr[6].eq {
	pc = 0x82DB4644; continue 'dispatch;
	}
	// 82DB4638: A12B0006  lhz r9, 6(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB463C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DB4640: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB4644: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DB4648: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DB464C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DB4650: 409AFFD4  bne cr6, 0x82db4624
	if !ctx.cr[6].eq {
	pc = 0x82DB4624; continue 'dispatch;
	}
	// 82DB4654: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82DB4658: 117F038C  vspltisw v11, -1
	for i in 0..4 {
		ctx.v[11].u32[i] = 4294967295;
	}
	// 82DB465C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB4660: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DB4664: 39000060  li r8, 0x60
	ctx.r[8].s64 = 96;
	// 82DB4668: C00ABE14  lfs f0, -0x41ec(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB466C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB4728 size=276
    let mut pc: u32 = 0x82DB4728;
    'dispatch: loop {
        match pc {
            0x82DB4728 => {
    //   block [0x82DB4728..0x82DB483C)
	// 82DB4728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB472C: 4BEF4CD1  bl 0x82ca93fc
	ctx.lr = 0x82DB4730;
	sub_82CA93D0(ctx, base);
	// 82DB4730: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4734: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DB4738: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB473C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB4740: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB4744: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DB4748: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB474C: 579D103A  slwi r29, r28, 2
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DB4750: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DB4754: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82DB4758: B3DF0008  sth r30, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u16 ) };
	// 82DB475C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB4760: 9B5F0000  stb r26, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 82DB4764: 9B5F0001  stb r26, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 82DB4768: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DB476C: B3DF0002  sth r30, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[30].u16 ) };
	// 82DB4770: B35F000A  sth r26, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[26].u16 ) };
	// 82DB4774: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82DB4778: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DB477C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DB4780: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB4784: 4BFA0AC5  bl 0x82d55248
	ctx.lr = 0x82DB4788;
	sub_82D55248(ctx, base);
	// 82DB4788: 57A9F0BE  srwi r9, r29, 2
	ctx.r[9].u32 = ctx.r[29].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DB478C: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82DB4790: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82DB4794: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82DB4798: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DB479C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DB47A0: 40990020  ble cr6, 0x82db47c0
	if !ctx.cr[6].gt {
	pc = 0x82DB47C0; continue 'dispatch;
	}
	// 82DB47A4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB47A8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DB47AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DB47B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DB47B4: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DB47B8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DB47BC: 409AFFE8  bne cr6, 0x82db47a4
	if !ctx.cr[6].eq {
	pc = 0x82DB47A4; continue 'dispatch;
	}
	// 82DB47C0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DB47C4: 40990034  ble cr6, 0x82db47f8
	if !ctx.cr[6].gt {
	pc = 0x82DB47F8; continue 'dispatch;
	}
	// 82DB47C8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB47CC: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB47D0: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB47D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB47D8: 419A0010  beq cr6, 0x82db47e8
	if ctx.cr[6].eq {
	pc = 0x82DB47E8; continue 'dispatch;
	}
	// 82DB47DC: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB47E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DB47E4: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB47E8: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DB47EC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DB47F0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DB47F4: 409AFFD4  bne cr6, 0x82db47c8
	if !ctx.cr[6].eq {
	pc = 0x82DB47C8; continue 'dispatch;
	}
	// 82DB47F8: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB4840 size=160
    let mut pc: u32 = 0x82DB4840;
    'dispatch: loop {
        match pc {
            0x82DB4840 => {
    //   block [0x82DB4840..0x82DB48E0)
	// 82DB4840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4844: 4BEF4BC9  bl 0x82ca940c
	ctx.lr = 0x82DB4848;
	sub_82CA93D0(ctx, base);
	// 82DB4848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB484C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB4850: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DB4854: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB4858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB485C: 4099005C  ble cr6, 0x82db48b8
	if !ctx.cr[6].gt {
	pc = 0x82DB48B8; continue 'dispatch;
	}
	// 82DB4860: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DB4864: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB4868: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB486C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB4870: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB4874: 419A0030  beq cr6, 0x82db48a4
	if ctx.cr[6].eq {
	pc = 0x82DB48A4; continue 'dispatch;
	}
	// 82DB4878: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB487C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB4880: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DB4884: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB4888: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB488C: 409A0018  bne cr6, 0x82db48a4
	if !ctx.cr[6].eq {
	pc = 0x82DB48A4; continue 'dispatch;
	}
	// 82DB4890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4894: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB4898: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB489C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB48A0: 4E800421  bctrl
	ctx.lr = 0x82DB48A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB48A4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB48A8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DB48AC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DB48B0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB48B4: 4198FFB0  blt cr6, 0x82db4864
	if ctx.cr[6].lt {
	pc = 0x82DB4864; continue 'dispatch;
	}
	// 82DB48B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB48BC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB48C0: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB48C4: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB48C8: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB48CC: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB48D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB48D4: 4BFA09F5  bl 0x82d552c8
	ctx.lr = 0x82DB48D8;
	sub_82D552C8(ctx, base);
	// 82DB48D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB48DC: 4BEF4B80  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB48E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB48E0 size=12
    let mut pc: u32 = 0x82DB48E0;
    'dispatch: loop {
        match pc {
            0x82DB48E0 => {
    //   block [0x82DB48E0..0x82DB48EC)
	// 82DB48E0: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB48E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DB48E8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB48EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB48EC size=32
    let mut pc: u32 = 0x82DB48EC;
    'dispatch: loop {
        match pc {
            0x82DB48EC => {
    //   block [0x82DB48EC..0x82DB490C)
	// 82DB48EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB48F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB48F4: 80830050  lwz r4, 0x50(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB48F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB48FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4900: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB4904: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB4908: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB490C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB490C size=4
    let mut pc: u32 = 0x82DB490C;
    'dispatch: loop {
        match pc {
            0x82DB490C => {
    //   block [0x82DB490C..0x82DB4910)
	// 82DB490C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB4910 size=32
    let mut pc: u32 = 0x82DB4910;
    'dispatch: loop {
        match pc {
            0x82DB4910 => {
    //   block [0x82DB4910..0x82DB4930)
	// 82DB4910: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4914: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB4918: 80830058  lwz r4, 0x58(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DB491C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB4920: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4924: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB4928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB492C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB4930 size=332
    let mut pc: u32 = 0x82DB4930;
    'dispatch: loop {
        match pc {
            0x82DB4930 => {
    //   block [0x82DB4930..0x82DB4A7C)
	// 82DB4930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB4938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB493C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB4940: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DB4944: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4948: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DB494C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DB4950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB4954: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DB4958: 48004641  bl 0x82db8f98
	ctx.lr = 0x82DB495C;
	sub_82DB8F98(ctx, base);
	// 82DB495C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB4960: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82DB4964: 395F0070  addi r10, r31, 0x70
	ctx.r[10].s64 = ctx.r[31].s64 + 112;
	// 82DB4968: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DB496C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB4A80 size=972
    let mut pc: u32 = 0x82DB4A80;
    'dispatch: loop {
        match pc {
            0x82DB4A80 => {
    //   block [0x82DB4A80..0x82DB4E4C)
	// 82DB4A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4A84: 4BEF4971  bl 0x82ca93f4
	ctx.lr = 0x82DB4A88;
	sub_82CA93D0(ctx, base);
	// 82DB4A88: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82DB4A8C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4A90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB4A94: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DB4A98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB4A9C: 480044FD  bl 0x82db8f98
	ctx.lr = 0x82DB4AA0;
	sub_82DB8F98(ctx, base);
	// 82DB4AA0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB4AA4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DB4AA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB4AAC: 3BBF0070  addi r29, r31, 0x70
	ctx.r[29].s64 = ctx.r[31].s64 + 112;
	// 82DB4AB0: 394A29F8  addi r10, r10, 0x29f8
	ctx.r[10].s64 = ctx.r[10].s64 + 10744;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB4E50 size=132
    let mut pc: u32 = 0x82DB4E50;
    'dispatch: loop {
        match pc {
            0x82DB4E50 => {
    //   block [0x82DB4E50..0x82DB4ED4)
	// 82DB4E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB4E58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB4E5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4E60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB4E64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB4E68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB4E6C: 396B2A1C  addi r11, r11, 0x2a1c
	ctx.r[11].s64 = ctx.r[11].s64 + 10780;
	// 82DB4E70: 394A29F8  addi r10, r10, 0x29f8
	ctx.r[10].s64 = ctx.r[10].s64 + 10744;
	// 82DB4E74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB4E78: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB4E7C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DB4E80: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB4E84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB4E88: 409A0020  bne cr6, 0x82db4ea8
	if !ctx.cr[6].eq {
	pc = 0x82DB4EA8; continue 'dispatch;
	}
	// 82DB4E8C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB4E90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DB4E94: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DB4E98: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DB4E9C: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DB4EA0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB4EA4: 4BFA0425  bl 0x82d552c8
	ctx.lr = 0x82DB4EA8;
	sub_82D552C8(ctx, base);
	// 82DB4EA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB4EAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DB4EB0: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB4EB4: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DB4EB8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB4EBC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB4EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB4EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB4EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB4ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB4ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB4ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB4ED8 size=392
    let mut pc: u32 = 0x82DB4ED8;
    'dispatch: loop {
        match pc {
            0x82DB4ED8 => {
    //   block [0x82DB4ED8..0x82DB5060)
	// 82DB4ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB4EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB4EE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB4EE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB4EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB4EEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB4EF0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB4EF4: 392A2A1C  addi r9, r10, 0x2a1c
	ctx.r[9].s64 = ctx.r[10].s64 + 10780;
	// 82DB4EF8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB4EFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB4F00: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB5060 size=364
    let mut pc: u32 = 0x82DB5060;
    'dispatch: loop {
        match pc {
            0x82DB5060 => {
    //   block [0x82DB5060..0x82DB51CC)
	// 82DB5060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB5064: 4BEF4389  bl 0x82ca93ec
	ctx.lr = 0x82DB5068;
	sub_82CA93D0(ctx, base);
	// 82DB5068: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB506C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DB5070: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82DB5074: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82DB5078: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DB507C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB5080: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB5084: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB5088: 40990038  ble cr6, 0x82db50c0
	if !ctx.cr[6].gt {
	pc = 0x82DB50C0; continue 'dispatch;
	}
	// 82DB508C: 815D0050  lwz r10, 0x50(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB5090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DB5094: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB5098: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DB509C: 93EA0034  stw r31, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 82DB50A0: 811D0054  lwz r8, 0x54(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB50A4: 815D0050  lwz r10, 0x50(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB50A8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DB50AC: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB50B0: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 82DB50B4: 81080018  lwz r8, 0x18(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB50B8: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82DB50BC: 4198FFD8  blt cr6, 0x82db5094
	if ctx.cr[6].lt {
	pc = 0x82DB5094; continue 'dispatch;
	}
	// 82DB50C0: 3AFD0060  addi r23, r29, 0x60
	ctx.r[23].s64 = ctx.r[29].s64 + 96;
	// 82DB50C4: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB50C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DB50CC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DB50D0: 40980014  bge cr6, 0x82db50e4
	if !ctx.cr[6].lt {
	pc = 0x82DB50E4; continue 'dispatch;
	}
	// 82DB50D4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DB50D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB50DC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DB50E0: 4BFA1E31  bl 0x82d56f10
	ctx.lr = 0x82DB50E4;
	sub_82D56F10(ctx, base);
	// 82DB50E4: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB50E8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DB50EC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DB50F0: 40980024  bge cr6, 0x82db5114
	if !ctx.cr[6].lt {
	pc = 0x82DB5114; continue 'dispatch;
	}
	// 82DB50F4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB50F8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB50FC: 41980008  blt cr6, 0x82db5104
	if ctx.cr[6].lt {
	pc = 0x82DB5104; continue 'dispatch;
	}
	// 82DB5100: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DB5104: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DB5108: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DB510C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DB5110: 4BFA1E01  bl 0x82d56f10
	ctx.lr = 0x82DB5114;
	sub_82D56F10(ctx, base);
	// 82DB5114: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DB5118: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DB511C: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB5120: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82DB5124: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB5128: 40990090  ble cr6, 0x82db51b8
	if !ctx.cr[6].gt {
	pc = 0x82DB51B8; continue 'dispatch;
	}
	// 82DB512C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DB5130: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB5134: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DB5138: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82DB513C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB5140: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB5144: 40990060  ble cr6, 0x82db51a4
	if !ctx.cr[6].gt {
	pc = 0x82DB51A4; continue 'dispatch;
	}
	// 82DB5148: 3975FFFB  addi r11, r21, -5
	ctx.r[11].s64 = ctx.r[21].s64 + -5;
	// 82DB514C: 577E083C  slwi r30, r27, 1
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DB5150: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DB5154: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DB5158: 69790001  xori r25, r11, 1
	ctx.r[25].u64 = ctx.r[11].u64 ^ 1;
	// 82DB515C: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB5160: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DB5164: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5168: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DB516C: 214A0020  subfic r10, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[10].s64;
	// 82DB5170: 7F8BF214  add r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DB5174: 7F0B5030  slw r11, r24, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[24].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB5178: 7D63FB78  or r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 82DB517C: 4802D8F5  bl 0x82de2a70
	ctx.lr = 0x82DB5180;
	sub_82DE2A70(ctx, base);
	// 82DB5180: B07C0000  sth r3, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 82DB5184: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB5188: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DB518C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DB5190: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82DB5194: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DB5198: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB519C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB51A0: 4198FFBC  blt cr6, 0x82db515c
	if ctx.cr[6].lt {
	pc = 0x82DB515C; continue 'dispatch;
	}
	// 82DB51A4: 817D0054  lwz r11, 0x54(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB51A8: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82DB51AC: 3B5A0080  addi r26, r26, 0x80
	ctx.r[26].s64 = ctx.r[26].s64 + 128;
	// 82DB51B0: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB51B4: 4198FF7C  blt cr6, 0x82db5130
	if ctx.cr[6].lt {
	pc = 0x82DB5130; continue 'dispatch;
	}
	// 82DB51B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DB51BC: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82DB51C0: 9ABD006C  stb r21, 0x6c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[21].u8 ) };
	// 82DB51C4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DB51C8: 4BEF4274  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB51D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB51D0 size=200
    let mut pc: u32 = 0x82DB51D0;
    'dispatch: loop {
        match pc {
            0x82DB51D0 => {
    //   block [0x82DB51D0..0x82DB5298)
	// 82DB51D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB51D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB51D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB51DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB51E0: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB51E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB51E8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DB51EC: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DB51F0: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB51F4: 4082000C  bne 0x82db5200
	if !ctx.cr[0].eq {
	pc = 0x82DB5200; continue 'dispatch;
	}
	// 82DB51F8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DB51FC: 48000084  b 0x82db5280
	pc = 0x82DB5280; continue 'dispatch;
	// 82DB5200: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB5204: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB5208: 409A0008  bne cr6, 0x82db5210
	if !ctx.cr[6].eq {
	pc = 0x82DB5210; continue 'dispatch;
	}
	// 82DB520C: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82DB5210: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5214: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB5218: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DB521C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB5220: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB5224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB5228: 4E800421  bctrl
	ctx.lr = 0x82DB522C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB522C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB5230: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82DB5234: 419A000C  beq cr6, 0x82db5240
	if ctx.cr[6].eq {
	pc = 0x82DB5240; continue 'dispatch;
	}
	// 82DB5238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB523C: 48000044  b 0x82db5280
	pc = 0x82DB5280; continue 'dispatch;
	// 82DB5240: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DB5244: 38C30040  addi r6, r3, 0x40
	ctx.r[6].s64 = ctx.r[3].s64 + 64;
	// 82DB5248: 38A30030  addi r5, r3, 0x30
	ctx.r[5].s64 = ctx.r[3].s64 + 48;
	// 82DB524C: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 82DB5250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB5254: C02BB384  lfs f1, -0x4c7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB5258: 48005B61  bl 0x82dbadb8
	ctx.lr = 0x82DB525C;
	sub_82DBADB8(ctx, base);
	// 82DB525C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB5264: 419AFFD4  beq cr6, 0x82db5238
	if ctx.cr[6].eq {
	pc = 0x82DB5238; continue 'dispatch;
	}
	// 82DB5268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB526C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DB5270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB5274: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB5278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB527C: 4E800421  bctrl
	ctx.lr = 0x82DB5280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB5280: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82DB5284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB5288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB528C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB5290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB5294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB5298 size=308
    let mut pc: u32 = 0x82DB5298;
    'dispatch: loop {
        match pc {
            0x82DB5298 => {
    //   block [0x82DB5298..0x82DB53CC)
	// 82DB5298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB529C: 4BEF4161  bl 0x82ca93fc
	ctx.lr = 0x82DB52A0;
	sub_82CA93D0(ctx, base);
	// 82DB52A0: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB52A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DB52A8: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82DB52AC: 548A007E  clrlwi r10, r4, 1
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82DB52B0: 549B0000  rlwinm r27, r4, 0, 0, 0
	ctx.r[27].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB52B4: 3F408330  lis r26, -0x7cd0
	ctx.r[26].s64 = -2094006272;
	// 82DB52B8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB52BC: 212B0020  subfic r9, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DB52C0: 7F2B5C30  srw r11, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[25].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB52C4: 7D7F2038  and r31, r11, r4
	ctx.r[31].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82DB52C8: 7D5E4C30  srw r30, r10, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[10].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB52CC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DB52D0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DB52D4: 409A004C  bne cr6, 0x82db5320
	if !ctx.cr[6].eq {
	pc = 0x82DB5320; continue 'dispatch;
	}
	// 82DB52D8: 815D0040  lwz r10, 0x40(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB52DC: 57CB3830  slwi r11, r30, 7
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB52E0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB52E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB52E8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB52EC: 41980068  blt cr6, 0x82db5354
	if ctx.cr[6].lt {
	pc = 0x82DB5354; continue 'dispatch;
	}
	// 82DB52F0: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DB52F4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DB52F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DB52FC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DB5300: 41980054  blt cr6, 0x82db5354
	if ctx.cr[6].lt {
	pc = 0x82DB5354; continue 'dispatch;
	}
	// 82DB5304: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DB5308: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB530C: 419A00B4  beq cr6, 0x82db53c0
	if ctx.cr[6].eq {
	pc = 0x82DB53C0; continue 'dispatch;
	}
	// 82DB5310: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82DB5314: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB5318: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82DB531C: 4BFFFFB0  b 0x82db52cc
	pc = 0x82DB52CC; continue 'dispatch;
	// 82DB5320: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB5324: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DB5328: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DB532C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB5330: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB5334: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB5338: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB533C: 41980018  blt cr6, 0x82db5354
	if ctx.cr[6].lt {
	pc = 0x82DB5354; continue 'dispatch;
	}
	// 82DB5340: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DB5344: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DB5348: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DB534C: 40980074  bge cr6, 0x82db53c0
	if !ctx.cr[6].lt {
	pc = 0x82DB53C0; continue 'dispatch;
	}
	// 82DB5350: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DB5354: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB5358: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB535C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5360: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DB5364: 216B0020  subfic r11, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DB5368: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB536C: 7FCB5830  slw r11, r30, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[30].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB5370: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 82DB5374: 7D7CFB78  or r28, r11, r31
	ctx.r[28].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 82DB5378: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DB537C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DB5380: 4E800421  bctrl
	ctx.lr = 0x82DB5384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB5384: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB5388: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82DB538C: 409A0028  bne cr6, 0x82db53b4
	if !ctx.cr[6].eq {
	pc = 0x82DB53B4; continue 'dispatch;
	}
	// 82DB5390: 38C30040  addi r6, r3, 0x40
	ctx.r[6].s64 = ctx.r[3].s64 + 64;
	// 82DB5394: C03AB384  lfs f1, -0x4c7c(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB5398: 38A30030  addi r5, r3, 0x30
	ctx.r[5].s64 = ctx.r[3].s64 + 48;
	// 82DB539C: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 82DB53A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB53A4: 48005A15  bl 0x82dbadb8
	ctx.lr = 0x82DB53A8;
	sub_82DBADB8(ctx, base);
	// 82DB53A8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB53AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB53B0: 409AFF1C  bne cr6, 0x82db52cc
	if !ctx.cr[6].eq {
	pc = 0x82DB52CC; continue 'dispatch;
	}
	// 82DB53B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DB53B8: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DB53BC: 4BEF4090  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82DB53C0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DB53C4: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DB53C8: 4BEF4084  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB53D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB53D0 size=1068
    let mut pc: u32 = 0x82DB53D0;
    'dispatch: loop {
        match pc {
            0x82DB53D0 => {
    //   block [0x82DB53D0..0x82DB57FC)
	// 82DB53D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB53D4: 4BEF4031  bl 0x82ca9404
	ctx.lr = 0x82DB53D8;
	sub_82CA93D0(ctx, base);
	// 82DB53D8: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB53DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DB53E0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DB53E4: 392BFFF0  addi r9, r11, -0x10
	ctx.r[9].s64 = ctx.r[11].s64 + -16;
	// 82DB53E8: 548B0000  rlwinm r11, r4, 0, 0, 0
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB53EC: 5488007E  clrlwi r8, r4, 1
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82DB53F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB53F4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DB53F8: 81690018  lwz r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB53FC: 20EB0020  subfic r7, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[7].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DB5400: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB5404: 7D642038  and r4, r11, r4
	ctx.r[4].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82DB5408: 7D0B3C30  srw r11, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB540C: 409A02CC  bne cr6, 0x82db56d8
	if !ctx.cr[6].eq {
	pc = 0x82DB56D8; continue 'dispatch;
	}
	// 82DB5410: 81490054  lwz r10, 0x54(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB5414: 390100DF  addi r8, r1, 0xdf
	ctx.r[8].s64 = ctx.r[1].s64 + 223;
	// 82DB5418: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82DB541C: 55050030  rlwinm r5, r8, 0, 0, 0x18
	ctx.r[5].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB5420: 409A000C  bne cr6, 0x82db542c
	if !ctx.cr[6].eq {
	pc = 0x82DB542C; continue 'dispatch;
	}
	// 82DB5424: 38A90070  addi r5, r9, 0x70
	ctx.r[5].s64 = ctx.r[9].s64 + 112;
	// 82DB5428: 4800005C  b 0x82db5484
	pc = 0x82DB5484; continue 'dispatch;
	// 82DB542C: 81490050  lwz r10, 0x50(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB5430: 55673830  slwi r7, r11, 7
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DB5434: 55663830  slwi r6, r11, 7
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DB5438: 7D675214  add r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82DB543C: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82DB5440: 7CE65850  subf r7, r6, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82DB5444: 7D4A3850  subf r10, r10, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82DB5448: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82DB544C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DB5450: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5454: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DB5458: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB545C: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB5460: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DB5464: 83CB000C  lwz r30, 0xc(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB5468: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DB546C: 90EAFFF8  stw r7, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 82DB5470: 90CAFFFC  stw r6, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[6].u32 ) };
	// 82DB5474: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DB5478: 93CA0004  stw r30, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DB547C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DB5480: 4199FFD0  bgt cr6, 0x82db5450
	if ctx.cr[6].gt {
	pc = 0x82DB5450; continue 'dispatch;
	}
	// 82DB5484: 89650031  lbz r11, 0x31(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(49 as u32) ) } as u64;
	// 82DB5488: 8145002C  lwz r10, 0x2c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DB548C: 7D670774  extsb r7, r11
	ctx.r[7].s64 = ctx.r[11].s8 as i64;
	// 82DB5490: 81050028  lwz r8, 0x28(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DB5494: 7D6A21D6  mullw r11, r10, r4
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82DB5498: 88C50030  lbz r6, 0x30(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DB549C: 7CEA2038  and r10, r7, r4
	ctx.r[10].u64 = ctx.r[7].u64 & ctx.r[4].u64;
	// 82DB54A0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DB54A4: 69480001  xori r8, r10, 1
	ctx.r[8].u64 = ctx.r[10].u64 ^ 1;
	// 82DB54A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DB54AC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82DB54B0: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DB54B4: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82DB54B8: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82DB54BC: 409A001C  bne cr6, 0x82db54d8
	if !ctx.cr[6].eq {
	pc = 0x82DB54D8; continue 'dispatch;
	}
	// 82DB54C0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB54C4: A3AB0000  lhz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB54C8: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DB54CC: 7F8A5A2E  lhzx r28, r10, r11
	ctx.r[28].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB54D0: 7F685A2E  lhzx r27, r8, r11
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB54D4: 48000018  b 0x82db54ec
	pc = 0x82DB54EC; continue 'dispatch;
	// 82DB54D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB54DC: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB54E0: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DB54E4: 7F8A582E  lwzx r28, r10, r11
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB54E8: 7F68582E  lwzx r27, r8, r11
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB54EC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82DB54F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB54F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DB54F8: 419A0050  beq cr6, 0x82db5548
	if ctx.cr[6].eq {
	pc = 0x82DB5548; continue 'dispatch;
	}
	// 82DB54FC: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DB5500: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB5504: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82DB5508: 394A4CA4  addi r10, r10, 0x4ca4
	ctx.r[10].s64 = ctx.r[10].s64 + 19620;
	// 82DB550C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82DB5510: C00BB264  lfs f0, -0x4d9c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19868 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB5514: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82DB5518: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DB551C: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DB5520: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB5524: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB5528: B3E30006  sth r31, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[31].u16 ) };
	// 82DB552C: B1030014  sth r8, 0x14(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u16 ) };
	// 82DB5530: 98C30016  stb r6, 0x16(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[6].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5800 size=24
    let mut pc: u32 = 0x82DB5800;
    'dispatch: loop {
        match pc {
            0x82DB5800 => {
    //   block [0x82DB5800..0x82DB5818)
	// 82DB5800: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DB5804: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DB5808: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DB580C: 38AB0040  addi r5, r11, 0x40
	ctx.r[5].s64 = ctx.r[11].s64 + 64;
	// 82DB5810: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DB5814: 4B4E45DC  b 0x82299df0
	sub_82299DF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB5818 size=188
    let mut pc: u32 = 0x82DB5818;
    'dispatch: loop {
        match pc {
            0x82DB5818 => {
    //   block [0x82DB5818..0x82DB58D4)
	// 82DB5818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB581C: 4BEF3BED  bl 0x82ca9408
	ctx.lr = 0x82DB5820;
	sub_82CA93D0(ctx, base);
	// 82DB5820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB5824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB5828: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB582C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB5830: 409A001C  bne cr6, 0x82db584c
	if !ctx.cr[6].eq {
	pc = 0x82DB584C; continue 'dispatch;
	}
	// 82DB5834: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82DB5838: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DB583C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DB5840: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DB5844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB5848: 4BEF3C10  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82DB584C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5850: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82DB5854: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB5858: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82DB585C: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB5860: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DB5864: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DB5868: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB586C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB5874: 4E800421  bctrl
	ctx.lr = 0x82DB5878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB5878: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB587C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB5880: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB5884: 55653830  slwi r5, r11, 7
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB5888: 4BFA34A9  bl 0x82d58d30
	ctx.lr = 0x82DB588C;
	sub_82D58D30(ctx, base);
	// 82DB588C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB5890: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DB5894: 4099001C  ble cr6, 0x82db58b0
	if !ctx.cr[6].gt {
	pc = 0x82DB58B0; continue 'dispatch;
	}
	// 82DB5898: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DB589C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB58A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB58A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB58A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB58AC: 4E800421  bctrl
	ctx.lr = 0x82DB58B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB58B0: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB58B4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82DB58B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB58BC: 556A3830  slwi r10, r11, 7
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB58C0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DB58C4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DB58C8: 386AFF80  addi r3, r10, -0x80
	ctx.r[3].s64 = ctx.r[10].s64 + -128;
	// 82DB58CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB58D0: 4BEF3B88  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB58D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB58D8 size=168
    let mut pc: u32 = 0x82DB58D8;
    'dispatch: loop {
        match pc {
            0x82DB58D8 => {
    //   block [0x82DB58D8..0x82DB5980)
	// 82DB58D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB58DC: 4BEF3B2D  bl 0x82ca9408
	ctx.lr = 0x82DB58E0;
	sub_82CA93D0(ctx, base);
	// 82DB58E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB58E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB58E8: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB58EC: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82DB58F0: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82DB58F4: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82DB58F8: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DB58FC: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DB5900: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB5904: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB5908: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB590C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB5910: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB5914: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5918: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB591C: 4E800421  bctrl
	ctx.lr = 0x82DB5920;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB5920: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DB5924: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DB5928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB592C: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB5930: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB5934: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB5938: 4BFA33F9  bl 0x82d58d30
	ctx.lr = 0x82DB593C;
	sub_82D58D30(ctx, base);
	// 82DB593C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DB5940: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DB5944: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5948: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB594C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB5950: 4E800421  bctrl
	ctx.lr = 0x82DB5954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB5954: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DB5958: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82DB595C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB5960: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB5964: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB5968: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DB596C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB5970: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DB5974: 386AFF70  addi r3, r10, -0x90
	ctx.r[3].s64 = ctx.r[10].s64 + -144;
	// 82DB5978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB597C: 4BEF3ADC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5980 size=244
    let mut pc: u32 = 0x82DB5980;
    'dispatch: loop {
        match pc {
            0x82DB5980 => {
    //   block [0x82DB5980..0x82DB5A74)
	// 82DB5980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB5984: 4BEF3A85  bl 0x82ca9408
	ctx.lr = 0x82DB5988;
	sub_82CA93D0(ctx, base);
	// 82DB5988: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5A78 size=368
    let mut pc: u32 = 0x82DB5A78;
    'dispatch: loop {
        match pc {
            0x82DB5A78 => {
    //   block [0x82DB5A78..0x82DB5BE8)
	// 82DB5A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB5A7C: 4BEF3991  bl 0x82ca940c
	ctx.lr = 0x82DB5A80;
	sub_82CA93D0(ctx, base);
	// 82DB5A80: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5BE8 size=164
    let mut pc: u32 = 0x82DB5BE8;
    'dispatch: loop {
        match pc {
            0x82DB5BE8 => {
    //   block [0x82DB5BE8..0x82DB5C8C)
	// 82DB5BE8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DB5BEC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DB5BF0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DB5BF4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DB5BF8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5BFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DB5C00: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB5C04: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DB5C08: 4200FFF0  bdnz 0x82db5bf8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DB5BF8; continue 'dispatch;
	}
	// 82DB5C0C: 81240018  lwz r9, 0x18(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB5C10: 39640040  addi r11, r4, 0x40
	ctx.r[11].s64 = ctx.r[4].s64 + 64;
	// 82DB5C14: 39430040  addi r10, r3, 0x40
	ctx.r[10].s64 = ctx.r[3].s64 + 64;
	// 82DB5C18: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DB5C1C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DB5C20: 91230018  stw r9, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82DB5C24: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82DB5C28: 80C4001C  lwz r6, 0x1c(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB5C2C: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82DB5C30: 80C40020  lwz r6, 0x20(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB5C34: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82DB5C38: 80C40024  lwz r6, 0x24(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB5C3C: 90C30024  stw r6, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 82DB5C40: 80C40028  lwz r6, 0x28(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DB5C44: 90C30028  stw r6, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82DB5C48: 80C4002C  lwz r6, 0x2c(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DB5C4C: 90C3002C  stw r6, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82DB5C50: 88C40030  lbz r6, 0x30(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DB5C54: 98C30030  stb r6, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[6].u8 ) };
	// 82DB5C58: 88C40031  lbz r6, 0x31(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(49 as u32) ) } as u64;
	// 82DB5C5C: 98C30031  stb r6, 0x31(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(49 as u32), ctx.r[6].u8 ) };
	// 82DB5C60: 80C40034  lwz r6, 0x34(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DB5C64: 90C30034  stw r6, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5C90 size=188
    let mut pc: u32 = 0x82DB5C90;
    'dispatch: loop {
        match pc {
            0x82DB5C90 => {
    //   block [0x82DB5C90..0x82DB5D4C)
	// 82DB5C90: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DB5C94: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DB5C98: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DB5C9C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DB5CA0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5CA4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DB5CA8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB5CAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DB5CB0: 4200FFF0  bdnz 0x82db5ca0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DB5CA0; continue 'dispatch;
	}
	// 82DB5CB4: 81240018  lwz r9, 0x18(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB5CB8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82DB5CBC: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82DB5CC0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DB5CC4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DB5CC8: 91230018  stw r9, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82DB5CCC: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82DB5CD0: 80C4001C  lwz r6, 0x1c(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB5CD4: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82DB5CD8: 88C40020  lbz r6, 0x20(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB5CDC: 98C30020  stb r6, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u8 ) };
	// 82DB5CE0: 88C40021  lbz r6, 0x21(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DB5CE4: 98C30021  stb r6, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[6].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5D50 size=8
    let mut pc: u32 = 0x82DB5D50;
    'dispatch: loop {
        match pc {
            0x82DB5D50 => {
    //   block [0x82DB5D50..0x82DB5D58)
	// 82DB5D50: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DB5D54: 48000004  b 0x82db5d58
	sub_82DB5D58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB5D58 size=100
    let mut pc: u32 = 0x82DB5D58;
    'dispatch: loop {
        match pc {
            0x82DB5D58 => {
    //   block [0x82DB5D58..0x82DB5DBC)
	// 82DB5D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB5D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB5D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB5D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB5D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB5D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB5D70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB5D74: 4BFFF0DD  bl 0x82db4e50
	ctx.lr = 0x82DB5D78;
	sub_82DB4E50(ctx, base);
	// 82DB5D78: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DB5D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB5D80: 419A0020  beq cr6, 0x82db5da0
	if ctx.cr[6].eq {
	pc = 0x82DB5DA0; continue 'dispatch;
	}
	// 82DB5D84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5D88: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DB5D8C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DB5D90: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB5D94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB5D98: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DB5D9C: 4BF9F52D  bl 0x82d552c8
	ctx.lr = 0x82DB5DA0;
	sub_82D552C8(ctx, base);
	// 82DB5DA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB5DA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB5DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB5DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB5DB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB5DB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB5DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB5DC0 size=96
    let mut pc: u32 = 0x82DB5DC0;
    'dispatch: loop {
        match pc {
            0x82DB5DC0 => {
    //   block [0x82DB5DC0..0x82DB5E20)
	// 82DB5DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB5DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB5DC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB5DCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB5DD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB5DD4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB5DD8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DB5DDC: 388B2A48  addi r4, r11, 0x2a48
	ctx.r[4].s64 = ctx.r[11].s64 + 10824;
	// 82DB5DE0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB5DE4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5DE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB5DEC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB5DF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB5DF4: 4E800421  bctrl
	ctx.lr = 0x82DB5DF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB5DF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5DFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB5E00: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB5E04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB5E08: 4E800421  bctrl
	ctx.lr = 0x82DB5E0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB5E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB5E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB5E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB5E18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB5E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5E20 size=8
    let mut pc: u32 = 0x82DB5E20;
    'dispatch: loop {
        match pc {
            0x82DB5E20 => {
    //   block [0x82DB5E20..0x82DB5E28)
	// 82DB5E20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DB5E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5E28 size=16
    let mut pc: u32 = 0x82DB5E28;
    'dispatch: loop {
        match pc {
            0x82DB5E28 => {
    //   block [0x82DB5E28..0x82DB5E38)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5E38 size=12
    let mut pc: u32 = 0x82DB5E38;
    'dispatch: loop {
        match pc {
            0x82DB5E38 => {
    //   block [0x82DB5E38..0x82DB5E44)
	// 82DB5E38: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82DB5E3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB5E40: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5E44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5E44 size=32
    let mut pc: u32 = 0x82DB5E44;
    'dispatch: loop {
        match pc {
            0x82DB5E44 => {
    //   block [0x82DB5E44..0x82DB5E64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5E68 size=16
    let mut pc: u32 = 0x82DB5E68;
    'dispatch: loop {
        match pc {
            0x82DB5E68 => {
    //   block [0x82DB5E68..0x82DB5E78)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5E78 size=16
    let mut pc: u32 = 0x82DB5E78;
    'dispatch: loop {
        match pc {
            0x82DB5E78 => {
    //   block [0x82DB5E78..0x82DB5E88)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5E88 size=60
    let mut pc: u32 = 0x82DB5E88;
    'dispatch: loop {
        match pc {
            0x82DB5E88 => {
    //   block [0x82DB5E88..0x82DB5EC4)
	// 82DB5E88: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB5E8C: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82DB5E90: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DB5E94: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DB5E98: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82DB5E9C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5EC8 size=32
    let mut pc: u32 = 0x82DB5EC8;
    'dispatch: loop {
        match pc {
            0x82DB5EC8 => {
    //   block [0x82DB5EC8..0x82DB5EE8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5EE8 size=44
    let mut pc: u32 = 0x82DB5EE8;
    'dispatch: loop {
        match pc {
            0x82DB5EE8 => {
    //   block [0x82DB5EE8..0x82DB5F14)
	// 82DB5EE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB5EEC: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DB5EF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB5EF4: 396B503C  addi r11, r11, 0x503c
	ctx.r[11].s64 = ctx.r[11].s64 + 20540;
	// 82DB5EF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB5EFC: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82DB5F00: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB5F04: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB5F08: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DB5F0C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DB5F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB5F18 size=144
    let mut pc: u32 = 0x82DB5F18;
    'dispatch: loop {
        match pc {
            0x82DB5F18 => {
    //   block [0x82DB5F18..0x82DB5FA8)
	// 82DB5F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB5F1C: 4BEF34F1  bl 0x82ca940c
	ctx.lr = 0x82DB5F20;
	sub_82CA93D0(ctx, base);
	// 82DB5F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB5F24: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5F28: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DB5F2C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DB5F30: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DB5F34: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB5F38: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB5F3C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB5F40: 40980020  bge cr6, 0x82db5f60
	if !ctx.cr[6].lt {
	pc = 0x82DB5F60; continue 'dispatch;
	}
	// 82DB5F44: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB5F48: 39292A54  addi r9, r9, 0x2a54
	ctx.r[9].s64 = ctx.r[9].s64 + 10836;
	// 82DB5F4C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB5F50: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB5F54: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DB5F58: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB5F5C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB5F60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DB5F64: C0240010  lfs f1, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB5F68: 4BFFB9D9  bl 0x82db1940
	ctx.lr = 0x82DB5F6C;
	sub_82DB1940(ctx, base);
	// 82DB5F6C: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DB5F70: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB5F74: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB5F78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB5F7C: 40980020  bge cr6, 0x82db5f9c
	if !ctx.cr[6].lt {
	pc = 0x82DB5F9C; continue 'dispatch;
	}
	// 82DB5F80: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB5F84: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB5F88: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB5F8C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB5F90: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB5F94: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB5F98: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB5F9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DB5FA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB5FA4: 4BEF34B8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5FA8 size=8
    let mut pc: u32 = 0x82DB5FA8;
    'dispatch: loop {
        match pc {
            0x82DB5FA8 => {
    //   block [0x82DB5FA8..0x82DB5FB0)
	// 82DB5FA8: D0230018  stfs f1, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DB5FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5FB0 size=8
    let mut pc: u32 = 0x82DB5FB0;
    'dispatch: loop {
        match pc {
            0x82DB5FB0 => {
    //   block [0x82DB5FB0..0x82DB5FB8)
	// 82DB5FB0: C0230018  lfs f1, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB5FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB5FB8 size=20
    let mut pc: u32 = 0x82DB5FB8;
    'dispatch: loop {
        match pc {
            0x82DB5FB8 => {
    //   block [0x82DB5FB8..0x82DB5FCC)
	// 82DB5FB8: E9640000  ld r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DB5FBC: E9440008  ld r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DB5FC0: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82DB5FC4: F9430008  std r10, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82DB5FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5FD0 size=8
    let mut pc: u32 = 0x82DB5FD0;
    'dispatch: loop {
        match pc {
            0x82DB5FD0 => {
    //   block [0x82DB5FD0..0x82DB5FD8)
	// 82DB5FD0: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DB5FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5FD8 size=8
    let mut pc: u32 = 0x82DB5FD8;
    'dispatch: loop {
        match pc {
            0x82DB5FD8 => {
    //   block [0x82DB5FD8..0x82DB5FE0)
	// 82DB5FD8: C0230010  lfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB5FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5FE0 size=8
    let mut pc: u32 = 0x82DB5FE0;
    'dispatch: loop {
        match pc {
            0x82DB5FE0 => {
    //   block [0x82DB5FE0..0x82DB5FE8)
	// 82DB5FE0: D0230014  stfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DB5FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5FE8 size=8
    let mut pc: u32 = 0x82DB5FE8;
    'dispatch: loop {
        match pc {
            0x82DB5FE8 => {
    //   block [0x82DB5FE8..0x82DB5FF0)
	// 82DB5FE8: C0230014  lfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB5FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB5FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB5FF0 size=120
    let mut pc: u32 = 0x82DB5FF0;
    'dispatch: loop {
        match pc {
            0x82DB5FF0 => {
    //   block [0x82DB5FF0..0x82DB6068)
	// 82DB5FF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB5FF4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DB5FF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB5FFC: C00B0AB4  lfs f0, 0xab4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2740 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB6000: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB6004: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DB6008: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82DB600C: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DB6010: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB6014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DB6018: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DB601C: C0090B24  lfs f0, 0xb24(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB6020: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DB6024: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DB6028: C1A90A78  lfs f13, 0xa78(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2680 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB602C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DB6030: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DB6034: C0090BF8  lfs f0, 0xbf8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB6038: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82DB603C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6068 size=12
    let mut pc: u32 = 0x82DB6068;
    'dispatch: loop {
        match pc {
            0x82DB6068 => {
    //   block [0x82DB6068..0x82DB6074)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6078 size=72
    let mut pc: u32 = 0x82DB6078;
    'dispatch: loop {
        match pc {
            0x82DB6078 => {
    //   block [0x82DB6078..0x82DB60C0)
	// 82DB6078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB6084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6088: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB608C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB6090: 396B2A64  addi r11, r11, 0x2a64
	ctx.r[11].s64 = ctx.r[11].s64 + 10852;
	// 82DB6094: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DB6098: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB609C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB60A0: 419A000C  beq cr6, 0x82db60ac
	if ctx.cr[6].eq {
	pc = 0x82DB60AC; continue 'dispatch;
	}
	// 82DB60A4: 4BA8F70D  bl 0x828457b0
	ctx.lr = 0x82DB60A8;
	sub_828457B0(ctx, base);
	// 82DB60A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB60AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB60B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB60B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB60B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB60BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB60C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB60C0 size=252
    let mut pc: u32 = 0x82DB60C0;
    'dispatch: loop {
        match pc {
            0x82DB60C0 => {
    //   block [0x82DB60C0..0x82DB61BC)
	// 82DB60C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB60C4: 4BEF333D  bl 0x82ca9400
	ctx.lr = 0x82DB60C8;
	sub_82CA93D0(ctx, base);
	// 82DB60C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB60CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB60D0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DB60D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB60D8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DB60DC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB60E0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB60E4: 7F6BEA14  add r27, r11, r29
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DB60E8: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DB60EC: 7F9F5850  subf r28, r31, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82DB60F0: 7F0AD800  cmpw cr6, r10, r27
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82DB60F4: 40980024  bge cr6, 0x82db6118
	if !ctx.cr[6].lt {
	pc = 0x82DB6118; continue 'dispatch;
	}
	// 82DB60F8: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB60FC: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB6100: 41980008  blt cr6, 0x82db6108
	if ctx.cr[6].lt {
	pc = 0x82DB6108; continue 'dispatch;
	}
	// 82DB6104: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82DB6108: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DB610C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DB6110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB6114: 4BFA0DFD  bl 0x82d56f10
	ctx.lr = 0x82DB6118;
	sub_82D56F10(ctx, base);
	// 82DB6118: 7D3FEA14  add r9, r31, r29
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82DB611C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6120: 57E71838  slwi r7, r31, 3
	ctx.r[7].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DB6124: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 82DB6128: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DB612C: 7D0A3A14  add r8, r10, r7
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DB6130: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DB6134: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB6138: 41980034  blt cr6, 0x82db616c
	if ctx.cr[6].lt {
	pc = 0x82DB616C; continue 'dispatch;
	}
	// 82DB613C: 55661838  slwi r6, r11, 3
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DB6140: 7D2A4050  subf r9, r10, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82DB6144: 7D465214  add r10, r6, r10
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 82DB6148: 7D095214  add r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DB614C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB6150: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB6154: 80C80000  lwz r6, 0(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6158: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82DB615C: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB6160: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB6164: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 82DB6168: 4098FFE0  bge cr6, 0x82db6148
	if !ctx.cr[6].lt {
	pc = 0x82DB6148; continue 'dispatch;
	}
	// 82DB616C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6170: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 82DB6174: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82DB6178: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB617C: 41980034  blt cr6, 0x82db61b0
	if ctx.cr[6].lt {
	pc = 0x82DB61B0; continue 'dispatch;
	}
	// 82DB6180: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DB6184: 7D2AD050  subf r9, r10, r26
	ctx.r[9].s64 = ctx.r[26].s64 - ctx.r[10].s64;
	// 82DB6188: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DB618C: 7D095214  add r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DB6190: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB6194: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB6198: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB619C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DB61A0: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB61A4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB61A8: 394AFFF8  addi r10, r10, -8
	ctx.r[10].s64 = ctx.r[10].s64 + -8;
	// 82DB61AC: 4098FFE0  bge cr6, 0x82db618c
	if !ctx.cr[6].lt {
	pc = 0x82DB618C; continue 'dispatch;
	}
	// 82DB61B0: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DB61B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DB61B8: 4BEF3298  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB61C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB61C0 size=128
    let mut pc: u32 = 0x82DB61C0;
    'dispatch: loop {
        match pc {
            0x82DB61C0 => {
    //   block [0x82DB61C0..0x82DB6240)
	// 82DB61C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB61C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB61C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB61CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB61D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB61D4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB61D8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB61DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB61E0: 409A0020  bne cr6, 0x82db6200
	if !ctx.cr[6].eq {
	pc = 0x82DB6200; continue 'dispatch;
	}
	// 82DB61E4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB61E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DB61EC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DB61F0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB61F4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB61F8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB61FC: 4BF9F0CD  bl 0x82d552c8
	ctx.lr = 0x82DB6200;
	sub_82D552C8(ctx, base);
	// 82DB6200: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB6204: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB6208: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DB620C: 409A0020  bne cr6, 0x82db622c
	if !ctx.cr[6].eq {
	pc = 0x82DB622C; continue 'dispatch;
	}
	// 82DB6210: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6214: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DB6218: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DB621C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB6220: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DB6224: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB6228: 4BF9F0A1  bl 0x82d552c8
	ctx.lr = 0x82DB622C;
	sub_82D552C8(ctx, base);
	// 82DB622C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB6230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB6234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB6238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB6240 size=828
    let mut pc: u32 = 0x82DB6240;
    'dispatch: loop {
        match pc {
            0x82DB6240 => {
    //   block [0x82DB6240..0x82DB657C)
	// 82DB6240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6244: 4BEF31AD  bl 0x82ca93f0
	ctx.lr = 0x82DB6248;
	sub_82CA93D0(ctx, base);
	// 82DB6248: DBC1FF98  stfd f30, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 82DB624C: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82DB6250: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6254: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DB6258: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82DB625C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB6260: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82DB6264: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DB6268: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82DB626C: 480575C5  bl 0x82e0d830
	ctx.lr = 0x82DB6270;
	sub_82E0D830(ctx, base);
	// 82DB6270: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB6274: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DB6278: 48057E99  bl 0x82e0e110
	ctx.lr = 0x82DB627C;
	sub_82E0E110(ctx, base);
	// 82DB627C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB6280: 895F001C  lbz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB6284: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB6288: C3EB0C14  lfs f31, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DB628C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB6290: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DB6294: C3CB0C18  lfs f30, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82DB6298: 419A0010  beq cr6, 0x82db62a8
	if ctx.cr[6].eq {
	pc = 0x82DB62A8; continue 'dispatch;
	}
	// 82DB629C: 897F001E  lbz r11, 0x1e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(30 as u32) ) } as u64;
	// 82DB62A0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DB62A4: 409A0008  bne cr6, 0x82db62ac
	if !ctx.cr[6].eq {
	pc = 0x82DB62AC; continue 'dispatch;
	}
	// 82DB62A8: D3C10070  stfs f30, 0x70(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82DB62AC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DB62B0: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82DB62B4: 480574D5  bl 0x82e0d788
	ctx.lr = 0x82DB62B8;
	sub_82E0D788(ctx, base);
	// 82DB62B8: 897F001F  lbz r11, 0x1f(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(31 as u32) ) } as u64;
	// 82DB62BC: 895F001E  lbz r10, 0x1e(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(30 as u32) ) } as u64;
	// 82DB62C0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DB62C4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DB62C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB62CC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DB62D0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DB62D4: 697E0001  xori r30, r11, 1
	ctx.r[30].u64 = ctx.r[11].u64 ^ 1;
	// 82DB62D8: 419A0008  beq cr6, 0x82db62e0
	if ctx.cr[6].eq {
	pc = 0x82DB62E0; continue 'dispatch;
	}
	// 82DB62DC: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82DB62E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB62E4: D3C1006C  stfs f30, 0x6c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DB62E8: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 82DB62EC: D3E100B8  stfs f31, 0xb8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 82DB62F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB62F4: C00B0A94  lfs f0, 0xa94(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB62F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB62FC: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DB6300: 93A100D0  stw r29, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[29].u32 ) };
	// 82DB6304: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DB6308: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 82DB630C: C1AB0AB4  lfs f13, 0xab4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2740 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB6310: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DB6314: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DB6318: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DB631C: 996100D4  stb r11, 0xd4(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[11].u8 ) };
	// 82DB6320: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6580 size=144
    let mut pc: u32 = 0x82DB6580;
    'dispatch: loop {
        match pc {
            0x82DB6580 => {
    //   block [0x82DB6580..0x82DB6610)
	// 82DB6580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6584: 4BEF2E89  bl 0x82ca940c
	ctx.lr = 0x82DB6588;
	sub_82CA93D0(ctx, base);
	// 82DB6588: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB658C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB6590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB6594: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DB6598: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB659C: 897E0020  lbz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB65A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB65A4: 419A0038  beq cr6, 0x82db65dc
	if ctx.cr[6].eq {
	pc = 0x82DB65DC; continue 'dispatch;
	}
	// 82DB65A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB65AC: 4805851D  bl 0x82e0eac8
	ctx.lr = 0x82DB65B0;
	sub_82E0EAC8(ctx, base);
	// 82DB65B0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DB65B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DB65B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB65BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB65C0: 4BFFFC81  bl 0x82db6240
	ctx.lr = 0x82DB65C4;
	sub_82DB6240(ctx, base);
	// 82DB65C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB65C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB65CC: 48058575  bl 0x82e0eb40
	ctx.lr = 0x82DB65D0;
	sub_82E0EB40(ctx, base);
	// 82DB65D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB65D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DB65D8: 4BEF2E84  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82DB65DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB65E0: 48057D21  bl 0x82e0e300
	ctx.lr = 0x82DB65E4;
	sub_82E0E300(ctx, base);
	// 82DB65E4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DB65E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DB65EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB65F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB65F4: 4BFFFC4D  bl 0x82db6240
	ctx.lr = 0x82DB65F8;
	sub_82DB6240(ctx, base);
	// 82DB65F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB65FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB6600: 48057C31  bl 0x82e0e230
	ctx.lr = 0x82DB6604;
	sub_82E0E230(ctx, base);
	// 82DB6604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB6608: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DB660C: 4BEF2E50  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6610 size=180
    let mut pc: u32 = 0x82DB6610;
    'dispatch: loop {
        match pc {
            0x82DB6610 => {
    //   block [0x82DB6610..0x82DB66C4)
	// 82DB6610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6618: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB661C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB6620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6624: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB6628: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB662C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB6630: 388B2ABC  addi r4, r11, 0x2abc
	ctx.r[4].s64 = ctx.r[11].s64 + 10940;
	// 82DB6634: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DB6638: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB663C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB6640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB6644: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB6648: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB664C: 4E800421  bctrl
	ctx.lr = 0x82DB6650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB6650: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6654: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DB6658: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DB665C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB6660: 388B5F1C  addi r4, r11, 0x5f1c
	ctx.r[4].s64 = ctx.r[11].s64 + 24348;
	// 82DB6664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB6668: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB666C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB6670: 4E800421  bctrl
	ctx.lr = 0x82DB6674;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB6674: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6678: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB667C: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB6680: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB6684: 388B2AB4  addi r4, r11, 0x2ab4
	ctx.r[4].s64 = ctx.r[11].s64 + 10932;
	// 82DB6688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB668C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB6690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB6694: 4E800421  bctrl
	ctx.lr = 0x82DB6698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB6698: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB669C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB66A0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB66A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB66A8: 4E800421  bctrl
	ctx.lr = 0x82DB66AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB66AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB66B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB66B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB66B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB66BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB66C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB66C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB66C8 size=144
    let mut pc: u32 = 0x82DB66C8;
    'dispatch: loop {
        match pc {
            0x82DB66C8 => {
    //   block [0x82DB66C8..0x82DB6758)
	// 82DB66C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB66CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB66D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB66D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB66D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB66DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB66E0: 3BC5FFC0  addi r30, r5, -0x40
	ctx.r[30].s64 = ctx.r[5].s64 + -64;
	// 82DB66E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DB66E8: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DB66EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB66F0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB66F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB66F8: 4E800421  bctrl
	ctx.lr = 0x82DB66FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB66FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DB6700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB6704: 41980038  blt cr6, 0x82db673c
	if ctx.cr[6].lt {
	pc = 0x82DB673C; continue 'dispatch;
	}
	// 82DB6708: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DB670C: 41990030  bgt cr6, 0x82db673c
	if ctx.cr[6].gt {
	pc = 0x82DB673C; continue 'dispatch;
	}
	// 82DB6710: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DB6714: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	// 82DB6718: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB671C: 409A0014  bne cr6, 0x82db6730
	if !ctx.cr[6].eq {
	pc = 0x82DB6730; continue 'dispatch;
	}
	// 82DB6720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB6724: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82DB6728: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DB672C: 48000014  b 0x82db6740
	pc = 0x82DB6740; continue 'dispatch;
	// 82DB6730: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82DB6734: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82DB6738: 48000008  b 0x82db6740
	pc = 0x82DB6740; continue 'dispatch;
	// 82DB673C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DB6740: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB6744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB6748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB674C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB6750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB6754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6758 size=56
    let mut pc: u32 = 0x82DB6758;
    'dispatch: loop {
        match pc {
            0x82DB6758 => {
    //   block [0x82DB6758..0x82DB6790)
	// 82DB6758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB675C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6760: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6764: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DB6768: 80830010  lwz r4, 0x10(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB676C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DB6770: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DB6774: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DB6778: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB677C: 48059255  bl 0x82e0f9d0
	ctx.lr = 0x82DB6780;
	sub_82E0F9D0(ctx, base);
	// 82DB6780: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DB6784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB6788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB678C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6790 size=132
    let mut pc: u32 = 0x82DB6790;
    'dispatch: loop {
        match pc {
            0x82DB6790 => {
    //   block [0x82DB6790..0x82DB6814)
	// 82DB6790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6798: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB679C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DB67A0: 80830010  lwz r4, 0x10(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB67A4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DB67A8: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82DB67AC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82DB67B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6818 size=68
    let mut pc: u32 = 0x82DB6818;
    'dispatch: loop {
        match pc {
            0x82DB6818 => {
    //   block [0x82DB6818..0x82DB685C)
	// 82DB6818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB681C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB6824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6828: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB682C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB6830: 396B2894  addi r11, r11, 0x2894
	ctx.r[11].s64 = ctx.r[11].s64 + 10388;
	// 82DB6834: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DB6838: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB683C: 4BFFB305  bl 0x82db1b40
	ctx.lr = 0x82DB6840;
	sub_82DB1B40(ctx, base);
	// 82DB6840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB6844: 4BFFD03D  bl 0x82db3880
	ctx.lr = 0x82DB6848;
	sub_82DB3880(ctx, base);
	// 82DB6848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB684C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB6850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB6854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB6858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6860 size=20
    let mut pc: u32 = 0x82DB6860;
    'dispatch: loop {
        match pc {
            0x82DB6860 => {
    //   block [0x82DB6860..0x82DB6874)
	// 82DB6860: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DB6864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6868: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB686C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB6870: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6878 size=96
    let mut pc: u32 = 0x82DB6878;
    'dispatch: loop {
        match pc {
            0x82DB6878 => {
    //   block [0x82DB6878..0x82DB68D8)
	// 82DB6878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB687C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82DB6880: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB6884: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82DB6888: 396B21FC  addi r11, r11, 0x21fc
	ctx.r[11].s64 = ctx.r[11].s64 + 8700;
	// 82DB688C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB6890: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB6894: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB6898: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DB689C: A1650004  lhz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB68A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB68A4: 419A0010  beq cr6, 0x82db68b4
	if ctx.cr[6].eq {
	pc = 0x82DB68B4; continue 'dispatch;
	}
	// 82DB68A8: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB68AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB68B0: B1650006  sth r11, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB68B4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82DB68B8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB68D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB68D8 size=136
    let mut pc: u32 = 0x82DB68D8;
    'dispatch: loop {
        match pc {
            0x82DB68D8 => {
    //   block [0x82DB68D8..0x82DB6960)
	// 82DB68D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB68DC: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82DB68E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB68E4: 396B21FC  addi r11, r11, 0x21fc
	ctx.r[11].s64 = ctx.r[11].s64 + 8700;
	// 82DB68E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB68EC: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82DB68F0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DB68F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB68F8: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DB68FC: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DB6900: A1650004  lhz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB6904: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB6908: 419A0010  beq cr6, 0x82db6918
	if ctx.cr[6].eq {
	pc = 0x82DB6918; continue 'dispatch;
	}
	// 82DB690C: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB6910: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB6914: B1650006  sth r11, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB6918: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DB691C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DB6920: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB6924: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB6928: 396B2894  addi r11, r11, 0x2894
	ctx.r[11].s64 = ctx.r[11].s64 + 10388;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6960 size=16
    let mut pc: u32 = 0x82DB6960;
    'dispatch: loop {
        match pc {
            0x82DB6960 => {
    //   block [0x82DB6960..0x82DB6970)
	// 82DB6960: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB6964: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB6968: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB696C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6970 size=284
    let mut pc: u32 = 0x82DB6970;
    'dispatch: loop {
        match pc {
            0x82DB6970 => {
    //   block [0x82DB6970..0x82DB6A8C)
	// 82DB6970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6974: 4BEF2A91  bl 0x82ca9404
	ctx.lr = 0x82DB6978;
	sub_82CA93D0(ctx, base);
	// 82DB6978: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB697C: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6980: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DB6984: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DB6988: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82DB698C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DB6990: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DB6994: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB6998: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB699C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB69A0: 40980020  bge cr6, 0x82db69c0
	if !ctx.cr[6].lt {
	pc = 0x82DB69C0; continue 'dispatch;
	}
	// 82DB69A4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB69A8: 39292AC8  addi r9, r9, 0x2ac8
	ctx.r[9].s64 = ctx.r[9].s64 + 10952;
	// 82DB69AC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB69B0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB69B4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DB69B8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB69BC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB69C0: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB69C4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DB69C8: 394100CF  addi r10, r1, 0xcf
	ctx.r[10].s64 = ctx.r[1].s64 + 207;
	// 82DB69CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB69D0: 55460036  rlwinm r6, r10, 0, 0, 0x1b
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB69D4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82DB69D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB69DC: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6A90 size=228
    let mut pc: u32 = 0x82DB6A90;
    'dispatch: loop {
        match pc {
            0x82DB6A90 => {
    //   block [0x82DB6A90..0x82DB6B74)
	// 82DB6A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB6A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB6AA0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6AA4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6AA8: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DB6AAC: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82DB6AB0: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DB6AB4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB6AB8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB6ABC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB6AC0: 40980020  bge cr6, 0x82db6ae0
	if !ctx.cr[6].lt {
	pc = 0x82DB6AE0; continue 'dispatch;
	}
	// 82DB6AC4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB6AC8: 39292AC8  addi r9, r9, 0x2ac8
	ctx.r[9].s64 = ctx.r[9].s64 + 10952;
	// 82DB6ACC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB6AD0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB6AD4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DB6AD8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB6ADC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DB6AE0: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DB6AE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB6AE8: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB6AEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB6AF0: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82DB6AF4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DB6AF8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DB6AFC: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82DB6B00: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82DB6B04: 914100A4  stw r10, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 82DB6B08: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82DB6B0C: 409A0008  bne cr6, 0x82db6b14
	if !ctx.cr[6].eq {
	pc = 0x82DB6B14; continue 'dispatch;
	}
	// 82DB6B10: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DB6B14: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB6B18: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DB6B1C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DB6B20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DB6B24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DB6B28: 4805A4A9  bl 0x82e10fd0
	ctx.lr = 0x82DB6B2C;
	sub_82E10FD0(ctx, base);
	// 82DB6B2C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DB6B30: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB6B34: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB6B38: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB6B3C: 40980020  bge cr6, 0x82db6b5c
	if !ctx.cr[6].lt {
	pc = 0x82DB6B5C; continue 'dispatch;
	}
	// 82DB6B40: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB6B44: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB6B48: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB6B4C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB6B50: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DB6B54: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB6B58: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB6B5C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DB6B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB6B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB6B68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB6B6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB6B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6B78 size=260
    let mut pc: u32 = 0x82DB6B78;
    'dispatch: loop {
        match pc {
            0x82DB6B78 => {
    //   block [0x82DB6B78..0x82DB6C7C)
	// 82DB6B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6B80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB6B84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB6B88: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6B8C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6C80 size=4
    let mut pc: u32 = 0x82DB6C80;
    'dispatch: loop {
        match pc {
            0x82DB6C80 => {
    //   block [0x82DB6C80..0x82DB6C84)
	// 82DB6C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6C88 size=260
    let mut pc: u32 = 0x82DB6C88;
    'dispatch: loop {
        match pc {
            0x82DB6C88 => {
    //   block [0x82DB6C88..0x82DB6D8C)
	// 82DB6C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB6C90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB6C94: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6C98: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DB6C9C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6CA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB6CA4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DB6CA8: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6D90 size=60
    let mut pc: u32 = 0x82DB6D90;
    'dispatch: loop {
        match pc {
            0x82DB6D90 => {
    //   block [0x82DB6D90..0x82DB6DCC)
	// 82DB6D90: 7CAB3278  xor r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 ^ ctx.r[6].u64;
	// 82DB6D94: 556B001E  rlwinm r11, r11, 0, 0, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB6D98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB6D9C: 409A0050  bne cr6, 0x82db6dec
	if !ctx.cr[6].eq {
		sub_82DB6DEC(ctx, base);
		return;
	}
	// 82DB6DA0: 54AB001E  rlwinm r11, r5, 0, 0, 0xf
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB6DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB6DA8: 419A0044  beq cr6, 0x82db6dec
	if ctx.cr[6].eq {
		sub_82DB6DEC(ctx, base);
		return;
	}
	// 82DB6DAC: 54CBD97E  srwi r11, r6, 5
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB6DB0: 7D6B2A78  xor r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[5].u64;
	// 82DB6DB4: 556B05B4  rlwinm r11, r11, 0, 0x16, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB6DB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB6DBC: 409A0010  bne cr6, 0x82db6dcc
	if !ctx.cr[6].eq {
		sub_82DB6DCC(ctx, base);
		return;
	}
	// 82DB6DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DB6DC4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DB6DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6DCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6DCC size=32
    let mut pc: u32 = 0x82DB6DCC;
    'dispatch: loop {
        match pc {
            0x82DB6DCC => {
    //   block [0x82DB6DCC..0x82DB6DEC)
	// 82DB6DCC: 54ABD97E  srwi r11, r5, 5
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB6DD0: 7D6B3278  xor r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[6].u64;
	// 82DB6DD4: 556B05B4  rlwinm r11, r11, 0, 0x16, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB6DD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB6DDC: 419AFFE4  beq cr6, 0x82db6dc0
	if ctx.cr[6].eq {
		sub_82DB6D90(ctx, base);
		return;
	}
	// 82DB6DE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DB6DE4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DB6DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6DEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6DEC size=56
    let mut pc: u32 = 0x82DB6DEC;
    'dispatch: loop {
        match pc {
            0x82DB6DEC => {
    //   block [0x82DB6DEC..0x82DB6E24)
	// 82DB6DEC: 54CA06FE  clrlwi r10, r6, 0x1b
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 82DB6DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB6DF4: 54AB06FE  clrlwi r11, r5, 0x1b
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x0000001Fu64;
	// 82DB6DF8: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 82DB6DFC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB6E00: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DB6E04: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB6E08: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82DB6E0C: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 82DB6E10: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DB6E14: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DB6E18: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DB6E1C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DB6E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6E28 size=104
    let mut pc: u32 = 0x82DB6E28;
    'dispatch: loop {
        match pc {
            0x82DB6E28 => {
    //   block [0x82DB6E28..0x82DB6E90)
	// 82DB6E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6E2C: 4BEF25DD  bl 0x82ca9408
	ctx.lr = 0x82DB6E30;
	sub_82CA93D0(ctx, base);
	// 82DB6E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6E34: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6E38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB6E3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DB6E40: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82DB6E44: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82DB6E48: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82DB6E4C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB6E50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB6E54: 4E800421  bctrl
	ctx.lr = 0x82DB6E58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB6E58: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6E5C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DB6E60: 808100D4  lwz r4, 0xd4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82DB6E64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB6E68: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB6E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB6E70: 4E800421  bctrl
	ctx.lr = 0x82DB6E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB6E74: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DB6E78: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DB6E7C: 389DFFF4  addi r4, r29, -0xc
	ctx.r[4].s64 = ctx.r[29].s64 + -12;
	// 82DB6E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB6E84: 4BFFFF0D  bl 0x82db6d90
	ctx.lr = 0x82DB6E88;
	sub_82DB6D90(ctx, base);
	// 82DB6E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB6E8C: 4BEF25CC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6E90 size=76
    let mut pc: u32 = 0x82DB6E90;
    'dispatch: loop {
        match pc {
            0x82DB6E90 => {
    //   block [0x82DB6E90..0x82DB6EDC)
	// 82DB6E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB6E94: 4BEF2579  bl 0x82ca940c
	ctx.lr = 0x82DB6E98;
	sub_82CA93D0(ctx, base);
	// 82DB6E98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB6E9C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6EA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB6EA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB6EA8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DB6EAC: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82DB6EB0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82DB6EB4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB6EB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB6EBC: 4E800421  bctrl
	ctx.lr = 0x82DB6EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB6EC0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DB6EC4: 389EFFF0  addi r4, r30, -0x10
	ctx.r[4].s64 = ctx.r[30].s64 + -16;
	// 82DB6EC8: 80BD0020  lwz r5, 0x20(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB6ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB6ED0: 4BFFFEC1  bl 0x82db6d90
	ctx.lr = 0x82DB6ED4;
	sub_82DB6D90(ctx, base);
	// 82DB6ED4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB6ED8: 4BEF2584  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6EE0 size=84
    let mut pc: u32 = 0x82DB6EE0;
    'dispatch: loop {
        match pc {
            0x82DB6EE0 => {
    //   block [0x82DB6EE0..0x82DB6F34)
	// 82DB6EE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB6EE4: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82DB6EE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DB6EEC: 7D0A4830  slw r10, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB6EF0: 7D472038  and r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 82DB6EF4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DB6EF8: 419A0010  beq cr6, 0x82db6f08
	if ctx.cr[6].eq {
	pc = 0x82DB6F08; continue 'dispatch;
	}
	// 82DB6EFC: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6F00: 7CE72B78  or r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 82DB6F04: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DB6F08: 7D4A2838  and r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[5].u64;
	// 82DB6F0C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB6F10: 419A0010  beq cr6, 0x82db6f20
	if ctx.cr[6].eq {
	pc = 0x82DB6F20; continue 'dispatch;
	}
	// 82DB6F14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6F18: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 82DB6F1C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB6F20: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DB6F24: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DB6F28: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 82DB6F2C: 4198FFC0  blt cr6, 0x82db6eec
	if ctx.cr[6].lt {
	pc = 0x82DB6EEC; continue 'dispatch;
	}
	// 82DB6F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6F38 size=56
    let mut pc: u32 = 0x82DB6F38;
    'dispatch: loop {
        match pc {
            0x82DB6F38 => {
    //   block [0x82DB6F38..0x82DB6F70)
	// 82DB6F38: 3964000D  addi r11, r4, 0xd
	ctx.r[11].s64 = ctx.r[4].s64 + 13;
	// 82DB6F3C: 3925000D  addi r9, r5, 0xd
	ctx.r[9].s64 = ctx.r[5].s64 + 13;
	// 82DB6F40: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB6F44: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB6F48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB6F4C: 7CEA182E  lwzx r7, r10, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DB6F50: 7D282830  slw r8, r9, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB6F54: 7D292030  slw r9, r9, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB6F58: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82DB6F5C: 7D0A192E  stwx r8, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 82DB6F60: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DB6F64: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82DB6F68: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 82DB6F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6F70 size=56
    let mut pc: u32 = 0x82DB6F70;
    'dispatch: loop {
        match pc {
            0x82DB6F70 => {
    //   block [0x82DB6F70..0x82DB6FA8)
	// 82DB6F70: 3964000D  addi r11, r4, 0xd
	ctx.r[11].s64 = ctx.r[4].s64 + 13;
	// 82DB6F74: 3925000D  addi r9, r5, 0xd
	ctx.r[9].s64 = ctx.r[5].s64 + 13;
	// 82DB6F78: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB6F7C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB6F80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB6F84: 7CEA182E  lwzx r7, r10, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DB6F88: 7D282830  slw r8, r9, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB6F8C: 7D292030  slw r9, r9, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB6F90: 7CE84078  andc r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 & !ctx.r[8].u64;
	// 82DB6F94: 7D0A192E  stwx r8, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 82DB6F98: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DB6F9C: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 82DB6FA0: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 82DB6FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB6FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB6FA8 size=84
    let mut pc: u32 = 0x82DB6FA8;
    'dispatch: loop {
        match pc {
            0x82DB6FA8 => {
    //   block [0x82DB6FA8..0x82DB6FFC)
	// 82DB6FA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB6FAC: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82DB6FB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DB6FB4: 7D0A4830  slw r10, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DB6FB8: 7D472038  and r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 82DB6FBC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DB6FC0: 419A0010  beq cr6, 0x82db6fd0
	if ctx.cr[6].eq {
	pc = 0x82DB6FD0; continue 'dispatch;
	}
	// 82DB6FC4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6FC8: 7CE72878  andc r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 & !ctx.r[5].u64;
	// 82DB6FCC: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DB6FD0: 7D4A2838  and r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[5].u64;
	// 82DB6FD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DB6FD8: 419A0010  beq cr6, 0x82db6fe8
	if ctx.cr[6].eq {
	pc = 0x82DB6FE8; continue 'dispatch;
	}
	// 82DB6FDC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB6FE0: 7D4A2078  andc r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[4].u64;
	// 82DB6FE4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB6FE8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DB6FEC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DB6FF0: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 82DB6FF4: 4198FFC0  blt cr6, 0x82db6fb4
	if ctx.cr[6].lt {
	pc = 0x82DB6FB4; continue 'dispatch;
	}
	// 82DB6FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB7000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB7000 size=172
    let mut pc: u32 = 0x82DB7000;
    'dispatch: loop {
        match pc {
            0x82DB7000 => {
    //   block [0x82DB7000..0x82DB70AC)
	// 82DB7000: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DB7004: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB7008: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB700C: 394A85E8  addi r10, r10, -0x7a18
	ctx.r[10].s64 = ctx.r[10].s64 + -31256;
	// 82DB7010: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB7014: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DB7018: 39298610  addi r9, r9, -0x79f0
	ctx.r[9].s64 = ctx.r[9].s64 + -31216;
	// 82DB701C: B0A30006  sth r5, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82DB7020: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DB7024: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DB7028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB702C: 39088624  addi r8, r8, -0x79dc
	ctx.r[8].s64 = ctx.r[8].s64 + -31196;
	// 82DB7030: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82DB7034: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82DB7038: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB703C: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DB7040: 3FE08203  lis r31, -0x7dfd
	ctx.r[31].s64 = -2113732608;
	// 82DB7044: 38E78604  addi r7, r7, -0x79fc
	ctx.r[7].s64 = ctx.r[7].s64 + -31228;
	// 82DB7048: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DB704C: 394A20E8  addi r10, r10, 0x20e8
	ctx.r[10].s64 = ctx.r[10].s64 + 8424;
	// 82DB7050: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82DB7054: 38C62114  addi r6, r6, 0x2114
	ctx.r[6].s64 = ctx.r[6].s64 + 8468;
	// 82DB7058: 38A52108  addi r5, r5, 0x2108
	ctx.r[5].s64 = ctx.r[5].s64 + 8456;
	// 82DB705C: 388420F4  addi r4, r4, 0x20f4
	ctx.r[4].s64 = ctx.r[4].s64 + 8436;
	// 82DB7060: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82DB7064: 3BFF20DC  addi r31, r31, 0x20dc
	ctx.r[31].s64 = ctx.r[31].s64 + 8412;
	// 82DB7068: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB706C: 91230020  stw r9, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82DB7070: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82DB7074: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82DB7078: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82DB707C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82DB7080: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DB7084: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82DB7088: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82DB708C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DB7090: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB7094: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DB7098: 4200FFF8  bdnz 0x82db7090
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DB7090; continue 'dispatch;
	}
	// 82DB709C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DB70A0: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DB70A4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82DB70A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB70B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB70B0 size=64
    let mut pc: u32 = 0x82DB70B0;
    'dispatch: loop {
        match pc {
            0x82DB70B0 => {
    //   block [0x82DB70B0..0x82DB70F0)
	// 82DB70B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB70B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB70B8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB70BC: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DB70C0: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DB70C4: 396B2114  addi r11, r11, 0x2114
	ctx.r[11].s64 = ctx.r[11].s64 + 8468;
	// 82DB70C8: 394A2108  addi r10, r10, 0x2108
	ctx.r[10].s64 = ctx.r[10].s64 + 8456;
	// 82DB70CC: 392920F4  addi r9, r9, 0x20f4
	ctx.r[9].s64 = ctx.r[9].s64 + 8436;
	// 82DB70D0: 390820E8  addi r8, r8, 0x20e8
	ctx.r[8].s64 = ctx.r[8].s64 + 8424;
	// 82DB70D4: 38E720DC  addi r7, r7, 0x20dc
	ctx.r[7].s64 = ctx.r[7].s64 + 8412;
	// 82DB70D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB70DC: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DB70E0: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DB70E4: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DB70E8: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82DB70EC: 4B8A83EC  b 0x8265f4d8
	sub_8265F4D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB70F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB70F0 size=16
    let mut pc: u32 = 0x82DB70F0;
    'dispatch: loop {
        match pc {
            0x82DB70F0 => {
    //   block [0x82DB70F0..0x82DB7100)
	// 82DB70F0: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB70F4: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 82DB70F8: 80A5001C  lwz r5, 0x1c(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB70FC: 4BFFFC94  b 0x82db6d90
	sub_82DB6D90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB7100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB7100 size=16
    let mut pc: u32 = 0x82DB7100;
    'dispatch: loop {
        match pc {
            0x82DB7100 => {
    //   block [0x82DB7100..0x82DB7110)
	// 82DB7100: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB7104: 3884FFEC  addi r4, r4, -0x14
	ctx.r[4].s64 = ctx.r[4].s64 + -20;
	// 82DB7108: 80A50024  lwz r5, 0x24(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB710C: 4BFFFC84  b 0x82db6d90
	sub_82DB6D90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB7110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB7110 size=348
    let mut pc: u32 = 0x82DB7110;
    'dispatch: loop {
        match pc {
            0x82DB7110 => {
    //   block [0x82DB7110..0x82DB726C)
	// 82DB7110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB7114: 4BEF22F1  bl 0x82ca9404
	ctx.lr = 0x82DB7118;
	sub_82CA93D0(ctx, base);
	// 82DB7118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB711C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB7120: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DB7124: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DB7128: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DB712C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DB7130: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82DB7134: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB7138: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82DB713C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB7140: 4E800421  bctrl
	ctx.lr = 0x82DB7144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB7144: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB7148: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DB714C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB7150: 409A0028  bne cr6, 0x82db7178
	if !ctx.cr[6].eq {
	pc = 0x82DB7178; continue 'dispatch;
	}
	// 82DB7154: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB7158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB715C: 419A0014  beq cr6, 0x82db7170
	if ctx.cr[6].eq {
	pc = 0x82DB7170; continue 'dispatch;
	}
	// 82DB7160: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DB7164: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB7168: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB716C: 409AFFF4  bne cr6, 0x82db7160
	if !ctx.cr[6].eq {
	pc = 0x82DB7160; continue 'dispatch;
	}
	// 82DB7170: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB7174: 48000068  b 0x82db71dc
	pc = 0x82DB71DC; continue 'dispatch;
	// 82DB7178: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB717C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DB7180: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DB7184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB7188: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB718C: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 82DB7190: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB7194: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB7198: 5569077A  rlwinm r9, r11, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB719C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DB71A0: 409A0058  bne cr6, 0x82db71f8
	if !ctx.cr[6].eq {
	pc = 0x82DB71F8; continue 'dispatch;
	}
	// 82DB71A4: 55690294  rlwinm r9, r11, 0, 0xa, 0xa
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB71A8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DB71AC: 409A006C  bne cr6, 0x82db7218
	if !ctx.cr[6].eq {
	pc = 0x82DB7218; continue 'dispatch;
	}
	// 82DB71B0: 5569035A  rlwinm r9, r11, 0, 0xd, 0xd
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB71B4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DB71B8: 409A007C  bne cr6, 0x82db7234
	if !ctx.cr[6].eq {
	pc = 0x82DB7234; continue 'dispatch;
	}
	// 82DB71BC: 556B0318  rlwinm r11, r11, 0, 0xc, 0xc
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB71C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB71C4: 409A0094  bne cr6, 0x82db7258
	if !ctx.cr[6].eq {
	pc = 0x82DB7258; continue 'dispatch;
	}
	// 82DB71C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB71CC: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB71D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB71D4: 409AFFB0  bne cr6, 0x82db7184
	if !ctx.cr[6].eq {
	pc = 0x82DB7184; continue 'dispatch;
	}
	// 82DB71D8: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB71DC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DB71E0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DB71E4: 389CFFF4  addi r4, r28, -0xc
	ctx.r[4].s64 = ctx.r[28].s64 + -12;
	// 82DB71E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DB71EC: 4BFFFBA5  bl 0x82db6d90
	ctx.lr = 0x82DB71F0;
	sub_82DB6D90(ctx, base);
	// 82DB71F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB71F4: 4BEF2260  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82DB71F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB71FC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB7200: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DB7204: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB7208: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB720C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB7210: 4E800421  bctrl
	ctx.lr = 0x82DB7214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB7214: 4BFFFFC8  b 0x82db71dc
	pc = 0x82DB71DC; continue 'dispatch;
	// 82DB7218: 48051131  bl 0x82e08348
	ctx.lr = 0x82DB721C;
	sub_82E08348(ctx, base);
	// 82DB721C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB7220: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB7224: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB7228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB722C: 4E800421  bctrl
	ctx.lr = 0x82DB7230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB7230: 4BFFFFAC  b 0x82db71dc
	pc = 0x82DB71DC; continue 'dispatch;
	// 82DB7234: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB7238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB723C: 419A0014  beq cr6, 0x82db7250
	if ctx.cr[6].eq {
	pc = 0x82DB7250; continue 'dispatch;
	}
	// 82DB7240: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DB7244: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB7248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB724C: 409AFFF4  bne cr6, 0x82db7240
	if !ctx.cr[6].eq {
	pc = 0x82DB7240; continue 'dispatch;
	}
	// 82DB7250: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB7254: 4BFFFF88  b 0x82db71dc
	pc = 0x82DB71DC; continue 'dispatch;
	// 82DB7258: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DB725C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DB7260: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DB7264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB7268: 4BEF21EC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB7270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB7270 size=8
    let mut pc: u32 = 0x82DB7270;
    'dispatch: loop {
        match pc {
            0x82DB7270 => {
    //   block [0x82DB7270..0x82DB7278)
	// 82DB7270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DB7274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB7278 size=116
    let mut pc: u32 = 0x82DB7278;
    'dispatch: loop {
        match pc {
            0x82DB7278 => {
    //   block [0x82DB7278..0x82DB72EC)
	// 82DB7278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB7280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB7284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB7288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB728C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB7290: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB7294: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB7298: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB729C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB72A0: 4E800421  bctrl
	ctx.lr = 0x82DB72A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB72A4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82DB72A8: 419A0028  beq cr6, 0x82db72d0
	if ctx.cr[6].eq {
	pc = 0x82DB72D0; continue 'dispatch;
	}
	// 82DB72AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB72B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DB72B4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DB72B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB72BC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB72C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB72C4: 4E800421  bctrl
	ctx.lr = 0x82DB72C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB72C8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82DB72CC: 409AFFE0  bne cr6, 0x82db72ac
	if !ctx.cr[6].eq {
	pc = 0x82DB72AC; continue 'dispatch;
	}
	// 82DB72D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB72D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB72D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB72DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB72E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB72E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB72E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB72F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB72F0 size=8
    let mut pc: u32 = 0x82DB72F0;
    'dispatch: loop {
        match pc {
            0x82DB72F0 => {
    //   block [0x82DB72F0..0x82DB72F8)
	// 82DB72F0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB72F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB72F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB72F8 size=4
    let mut pc: u32 = 0x82DB72F8;
    'dispatch: loop {
        match pc {
            0x82DB72F8 => {
    //   block [0x82DB72F8..0x82DB72FC)
	// 82DB72F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB7300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB7300 size=516
    let mut pc: u32 = 0x82DB7300;
    'dispatch: loop {
        match pc {
            0x82DB7300 => {
    //   block [0x82DB7300..0x82DB7504)
	// 82DB7300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB7304: 4BEF20F5  bl 0x82ca93f8
	ctx.lr = 0x82DB7308;
	sub_82CA93D0(ctx, base);
	// 82DB7308: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 82DB730C: 4BEF69C9  bl 0x82cadcd4
	ctx.lr = 0x82DB7310;
	sub_82CADCA0(ctx, base);
	// 82DB7310: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB7314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB7318: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DB731C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82DB7320: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82DB7324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB7328: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DB732C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DB7330: C36B0C14  lfs f27, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82DB7334: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 82DB7338: D37E0000  stfs f27, 0(r30)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DB733C: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB7508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB7508 size=432
    let mut pc: u32 = 0x82DB7508;
    'dispatch: loop {
        match pc {
            0x82DB7508 => {
    //   block [0x82DB7508..0x82DB76B8)
	// 82DB7508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB750C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB7510: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB7514: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB7518: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DB751C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB7520: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB7524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB7528: 394B07F4  addi r10, r11, 0x7f4
	ctx.r[10].s64 = ctx.r[11].s64 + 2036;
	// 82DB752C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB7530: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DB7534: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 82DB7538: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB753C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB7540: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB7544: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DB7548: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB754C: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB7550: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DB7554: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DB7558: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82DB755C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB7560: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB7564: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DB7568: 89440020  lbz r10, 0x20(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB756C: 995F001C  stb r10, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 82DB7570: C1A40018  lfs f13, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB7574: 81240014  lwz r9, 0x14(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB7578: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB757C: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB7580: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DB7584: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82DB7588: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DB758C: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82DB7590: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 82DB7594: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82DB7598: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DB759C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DB75A0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB75A4: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DB75A8: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB75AC: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DB75B0: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DB75B4: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DB75B8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DB75BC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82DB75C0: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DB75C4: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DB75C8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB76B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB76B8 size=516
    let mut pc: u32 = 0x82DB76B8;
    'dispatch: loop {
        match pc {
            0x82DB76B8 => {
    //   block [0x82DB76B8..0x82DB78BC)
	// 82DB76B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB76BC: 4BEF1D49  bl 0x82ca9404
	ctx.lr = 0x82DB76C0;
	sub_82CA93D0(ctx, base);
	// 82DB76C0: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82DB76C4: 4BEF6615  bl 0x82cadcd8
	ctx.lr = 0x82DB76C8;
	sub_82CADCA0(ctx, base);
	// 82DB76C8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB76CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB76D0: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 82DB76D4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DB76D8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DB76DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DB76E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DB76E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB76E8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DB76EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB76F0: 4E800421  bctrl
	ctx.lr = 0x82DB76F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB76F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB76F8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DB76FC: C1BF0054  lfs f13, 0x54(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB7700: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB7704: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB7708: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DB770C: C3AB0BFC  lfs f29, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82DB7710: FFC0F890  fmr f30, f31
	ctx.f[30].f64 = ctx.f[31].f64;
	// 82DB7714: 409800A0  bge cr6, 0x82db77b4
	if !ctx.cr[6].lt {
	pc = 0x82DB77B4; continue 'dispatch;
	}
	// 82DB7718: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB771C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DB7720: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB7724: 40990060  ble cr6, 0x82db7784
	if !ctx.cr[6].gt {
	pc = 0x82DB7784; continue 'dispatch;
	}
	// 82DB7728: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB772C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB7730: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB7734: 40990040  ble cr6, 0x82db7774
	if !ctx.cr[6].gt {
	pc = 0x82DB7774; continue 'dispatch;
	}
	// 82DB7738: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB773C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DB7740: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB7744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB7748: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DB774C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB7750: 4E800421  bctrl
	ctx.lr = 0x82DB7754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB7754: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DB7758: EDBE0828  fsubs f13, f30, f1
	ctx.f[13].f64 = (((ctx.f[30].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DB775C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB7760: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DB7764: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB7768: FFE0F86E  fsel f31, f0, f1, f31
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[31].f64 };
	// 82DB776C: FFCD0FAE  fsel f30, f13, f30, f1
	ctx.f[30].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[1].f64 };
	// 82DB7770: 4198FFC8  blt cr6, 0x82db7738
	if ctx.cr[6].lt {
	pc = 0x82DB7738; continue 'dispatch;
	}
	// 82DB7774: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB7778: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DB777C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DB7780: 4198FFA8  blt cr6, 0x82db7728
	if ctx.cr[6].lt {
	pc = 0x82DB7728; continue 'dispatch;
	}
	// 82DB7784: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB7788: EC0D07F2  fmuls f0, f13, f31
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 82DB778C: EDAD07B2  fmuls f13, f13, f30
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[30].f64) as f32) as f64);
	// 82DB7790: FD600090  fmr f11, f0
	ctx.f[11].f64 = ctx.f[0].f64;
	// 82DB7794: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DB7798: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82DB779C: FDAC6AEE  fsel f13, f12, f11, f13
	ctx.f[13].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[13].f64 };
	// 82DB77A0: ED8D002A  fadds f12, f13, f0
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DB77A4: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DB77A8: EDAC0772  fmuls f13, f12, f29
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[29].f64) as f32) as f64);
	// 82DB77AC: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DB77B0: D01F0054  stfs f0, 0x54(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DB77B4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DB77B8: D3A10050  stfs f29, 0x50(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DB77BC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82DB77C0: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB77C4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB78C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB78C0 size=4216
    let mut pc: u32 = 0x82DB78C0;
    'dispatch: loop {
        match pc {
            0x82DB78C0 => {
    //   block [0x82DB78C0..0x82DB8938)
	// 82DB78C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB78C4: 4BEF1B0D  bl 0x82ca93d0
	ctx.lr = 0x82DB78C8;
	sub_82CA93D0(ctx, base);
	// 82DB78C8: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82DB78CC: 4BEF63E9  bl 0x82cadcb4
	ctx.lr = 0x82DB78D0;
	sub_82CADCA0(ctx, base);
	// 82DB78D0: 3980FEE0  li r12, -0x120
	ctx.r[12].s64 = -288;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB8938 size=124
    let mut pc: u32 = 0x82DB8938;
    'dispatch: loop {
        match pc {
            0x82DB8938 => {
    //   block [0x82DB8938..0x82DB89B4)
	// 82DB8938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB893C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB8940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB8944: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8948: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DB894C: C0060010  lfs f0, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB8950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB8954: 90C1006C  stw r6, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[6].u32 ) };
	// 82DB8958: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB895C: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DB8960: 394A2B20  addi r10, r10, 0x2b20
	ctx.r[10].s64 = ctx.r[10].s64 + 11040;
	// 82DB8964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB8968: C0290C18  lfs f1, 0xc18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB896C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB8970: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DB8974: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DB8978: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DB897C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DB8980: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DB8984: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82DB8988: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB898C: 99210068  stb r9, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u8 ) };
	// 82DB8990: 4BFFEF31  bl 0x82db78c0
	ctx.lr = 0x82DB8994;
	sub_82DB78C0(ctx, base);
	// 82DB8994: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DB8998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB899C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DB89A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DB89A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB89A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB89AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB89B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB89B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB89B8 size=84
    let mut pc: u32 = 0x82DB89B8;
    'dispatch: loop {
        match pc {
            0x82DB89B8 => {
    //   block [0x82DB89B8..0x82DB8A0C)
	// 82DB89B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB89BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB89C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB89C4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DB89C8: C0060004  lfs f0, 4(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB89CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB89D0: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82DB89D4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB89D8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DB89DC: 394A0F2C  addi r10, r10, 0xf2c
	ctx.r[10].s64 = ctx.r[10].s64 + 3884;
	// 82DB89E0: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82DB89E4: C0290C18  lfs f1, 0xc18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB89E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB89EC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DB89F0: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82DB89F4: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB89F8: 4BFFEEC9  bl 0x82db78c0
	ctx.lr = 0x82DB89FC;
	sub_82DB78C0(ctx, base);
	// 82DB89FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB8A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB8A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB8A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB8A10 size=148
    let mut pc: u32 = 0x82DB8A10;
    'dispatch: loop {
        match pc {
            0x82DB8A10 => {
    //   block [0x82DB8A10..0x82DB8AA4)
	// 82DB8A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB8A18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8A1C: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82DB8A20: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DB8A24: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DB8A28: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DB8A2C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DB8A30: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DB8A34: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DB8A38: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DB8A3C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DB8A40: 4200FFF0  bdnz 0x82db8a30
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DB8A30; continue 'dispatch;
	}
	// 82DB8A44: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB8A48: C1830024  lfs f12, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DB8A4C: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB8A50: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB8A54: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 82DB8A58: 40990008  ble cr6, 0x82db8a60
	if !ctx.cr[6].gt {
	pc = 0x82DB8A60; continue 'dispatch;
	}
	// 82DB8A5C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DB8A60: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB8A64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DB8A68: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DB8A6C: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DB8A70: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DB8A74: C1A10074  lfs f13, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB8A78: C0240034  lfs f1, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DB8A7C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DB8A80: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DB8A84: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82DB8A88: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8A8C: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82DB8A90: 4BFFEE31  bl 0x82db78c0
	ctx.lr = 0x82DB8A94;
	sub_82DB78C0(ctx, base);
	// 82DB8A94: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DB8A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB8A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB8AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB8AA8 size=16
    let mut pc: u32 = 0x82DB8AA8;
    'dispatch: loop {
        match pc {
            0x82DB8AA8 => {
    //   block [0x82DB8AA8..0x82DB8AB8)
	// 82DB8AA8: C0050010  lfs f0, 0x10(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB8AAC: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DB8AB0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DB8AB4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB8AB8 size=76
    let mut pc: u32 = 0x82DB8AB8;
    'dispatch: loop {
        match pc {
            0x82DB8AB8 => {
    //   block [0x82DB8AB8..0x82DB8B04)
	// 82DB8AB8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB8ABC: D0030004  stfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8B08 size=8
    let mut pc: u32 = 0x82DB8B08;
    'dispatch: loop {
        match pc {
            0x82DB8B08 => {
    //   block [0x82DB8B08..0x82DB8B10)
	// 82DB8B08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DB8B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8B10 size=20
    let mut pc: u32 = 0x82DB8B10;
    'dispatch: loop {
        match pc {
            0x82DB8B10 => {
    //   block [0x82DB8B10..0x82DB8B24)
	// 82DB8B10: 548B07FE  clrlwi r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DB8B14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB8B18: 409A000C  bne cr6, 0x82db8b24
	if !ctx.cr[6].eq {
		sub_82DB8B24(ctx, base);
		return;
	}
	// 82DB8B1C: 60830001  ori r3, r4, 1
	ctx.r[3].u64 = ctx.r[4].u64 | 1;
	// 82DB8B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8B24 size=64
    let mut pc: u32 = 0x82DB8B24;
    'dispatch: loop {
        match pc {
            0x82DB8B24 => {
    //   block [0x82DB8B24..0x82DB8B64)
	// 82DB8B24: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB8B28: 548AFC7E  rlwinm r10, r4, 0x1f, 0x11, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DB8B2C: 548B843E  srwi r11, r4, 0x10
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB8B30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DB8B34: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB8B38: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DB8B3C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DB8B40: 409A0024  bne cr6, 0x82db8b64
	if !ctx.cr[6].eq {
		sub_82DB8B64(ctx, base);
		return;
	}
	// 82DB8B44: 81290014  lwz r9, 0x14(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB8B48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB8B4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB8B50: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DB8B54: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DB8B58: 409A000C  bne cr6, 0x82db8b64
	if !ctx.cr[6].eq {
		sub_82DB8B64(ctx, base);
		return;
	}
	// 82DB8B5C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DB8B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8B64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8B64 size=16
    let mut pc: u32 = 0x82DB8B64;
    'dispatch: loop {
        match pc {
            0x82DB8B64 => {
    //   block [0x82DB8B64..0x82DB8B74)
	// 82DB8B64: 556B7820  slwi r11, r11, 0xf
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(15);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB8B68: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DB8B6C: 5563083C  slwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DB8B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB8B78 size=132
    let mut pc: u32 = 0x82DB8B78;
    'dispatch: loop {
        match pc {
            0x82DB8B78 => {
    //   block [0x82DB8B78..0x82DB8BFC)
	// 82DB8B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB8B80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DB8B84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB8B88: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DB8B8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8B90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DB8B94: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DB8B98: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DB8B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB8BA0: 480003F9  bl 0x82db8f98
	ctx.lr = 0x82DB8BA4;
	sub_82DB8F98(ctx, base);
	// 82DB8BA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8BA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB8BAC: D3FF001C  stfs f31, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DB8BB0: 396B09D8  addi r11, r11, 0x9d8
	ctx.r[11].s64 = ctx.r[11].s64 + 2520;
	// 82DB8BB4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82DB8BB8: 394A09B4  addi r10, r10, 0x9b4
	ctx.r[10].s64 = ctx.r[10].s64 + 2484;
	// 82DB8BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB8BC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB8BC4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB8BC8: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB8BCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB8BD0: 419A0010  beq cr6, 0x82db8be0
	if ctx.cr[6].eq {
	pc = 0x82DB8BE0; continue 'dispatch;
	}
	// 82DB8BD4: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB8BD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB8BDC: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB8BE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DB8BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB8BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB8BEC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DB8BF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB8BF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB8BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


