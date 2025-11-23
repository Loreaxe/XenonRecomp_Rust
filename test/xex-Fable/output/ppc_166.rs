pub fn sub_82EF58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF58E8 size=16
    let mut pc: u32 = 0x82EF58E8;
    'dispatch: loop {
        match pc {
            0x82EF58E8 => {
    //   block [0x82EF58E8..0x82EF58F8)
	// 82EF58E8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF58EC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF58F0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF58F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF58F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF58F8 size=44
    let mut pc: u32 = 0x82EF58F8;
    'dispatch: loop {
        match pc {
            0x82EF58F8 => {
    //   block [0x82EF58F8..0x82EF5924)
	// 82EF58F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF58FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5904: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5908: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF590C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82EF5910: 4BFF78B1  bl 0x82eed1c0
	ctx.lr = 0x82EF5914;
	sub_82EED1C0(ctx, base);
	// 82EF5914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF591C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5928 size=16
    let mut pc: u32 = 0x82EF5928;
    'dispatch: loop {
        match pc {
            0x82EF5928 => {
    //   block [0x82EF5928..0x82EF5938)
	// 82EF5928: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF592C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5930: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 82EF5934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5938 size=44
    let mut pc: u32 = 0x82EF5938;
    'dispatch: loop {
        match pc {
            0x82EF5938 => {
    //   block [0x82EF5938..0x82EF5964)
	// 82EF5938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5944: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5948: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF594C: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 82EF5950: 4BFF7871  bl 0x82eed1c0
	ctx.lr = 0x82EF5954;
	sub_82EED1C0(ctx, base);
	// 82EF5954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF595C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5968 size=44
    let mut pc: u32 = 0x82EF5968;
    'dispatch: loop {
        match pc {
            0x82EF5968 => {
    //   block [0x82EF5968..0x82EF5994)
	// 82EF5968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5974: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5978: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF597C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82EF5980: 4BFF7841  bl 0x82eed1c0
	ctx.lr = 0x82EF5984;
	sub_82EED1C0(ctx, base);
	// 82EF5984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5998 size=164
    let mut pc: u32 = 0x82EF5998;
    'dispatch: loop {
        match pc {
            0x82EF5998 => {
    //   block [0x82EF5998..0x82EF5A08)
	// 82EF5998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF599C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF59A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF59A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF59A8: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF59AC: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF59B0: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 82EF59B4: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF59B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF59BC: 409A004C  bne cr6, 0x82ef5a08
	if !ctx.cr[6].eq {
	pc = 0x82EF5A08; continue 'dispatch;
	}
	// 82EF59C0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF59C4: 896B8F83  lbz r11, -0x707d(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28797 as u32) ) } as u64;
	// 82EF59C8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF59CC: 4082003C  bne 0x82ef5a08
	if !ctx.cr[0].eq {
	pc = 0x82EF5A08; continue 'dispatch;
	}
	// 82EF59D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF59D4: 3BEB8F83  addi r31, r11, -0x707d
	ctx.r[31].s64 = ctx.r[11].s64 + -28797;
	// 82EF59D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF59DC: 38CBCE7C  addi r6, r11, -0x3184
	ctx.r[6].s64 = ctx.r[11].s64 + -12676;
	// 82EF59E0: 38A0003A  li r5, 0x3a
	ctx.r[5].s64 = 58;
	// 82EF59E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF59E8: 388BCE20  addi r4, r11, -0x31e0
	ctx.r[4].s64 = ctx.r[11].s64 + -12768;
	// 82EF59EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF59F0: 4BFF5779  bl 0x82eeb168
	ctx.lr = 0x82EF59F4;
	sub_82EEB168(ctx, base);
	// 82EF59F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF59F8: 481FC3C1  bl 0x830f1db8
	ctx.lr = 0x82EF59FC;
	sub_830F1DB8(ctx, base);
	// 82EF59FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5A00: 40820008  bne 0x82ef5a08
	if !ctx.cr[0].eq {
	pc = 0x82EF5A08; continue 'dispatch;
	}
	// 82EF5A04: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82EF5A08; continue 'dispatch;
            }
            0x82EF5A08 => {
    //   block [0x82EF5A08..0x82EF5A3C)
	// 82EF5A08: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF5A0C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5A10: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5A14: 386AFFEC  addi r3, r10, -0x14
	ctx.r[3].s64 = ctx.r[10].s64 + -20;
	// 82EF5A18: 816BFFEC  lwz r11, -0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82EF5A1C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EF5A20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5A24: 4E800421  bctrl
	ctx.lr = 0x82EF5A28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF5A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5A34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF5A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5A40 size=12
    let mut pc: u32 = 0x82EF5A40;
    'dispatch: loop {
        match pc {
            0x82EF5A40 => {
    //   block [0x82EF5A40..0x82EF5A4C)
	// 82EF5A40: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5A44: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF5A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5A50 size=168
    let mut pc: u32 = 0x82EF5A50;
    'dispatch: loop {
        match pc {
            0x82EF5A50 => {
    //   block [0x82EF5A50..0x82EF5AF8)
	// 82EF5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5A58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5A5C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5A60: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5A64: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A68: 4BFFA8E1  bl 0x82ef0348
	ctx.lr = 0x82EF5A6C;
	sub_82EF0348(ctx, base);
	// 82EF5A6C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A70: 386B04F8  addi r3, r11, 0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + 1272;
	// 82EF5A74: 480000CD  bl 0x82ef5b40
	ctx.lr = 0x82EF5A78;
	sub_82EF5B40(ctx, base);
	// 82EF5A78: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5A7C: 396BCEC8  addi r11, r11, -0x3138
	ctx.r[11].s64 = ctx.r[11].s64 + -12600;
	// 82EF5A80: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A84: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5A88: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A8C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5A90: 394ACEBC  addi r10, r10, -0x3144
	ctx.r[10].s64 = ctx.r[10].s64 + -12612;
	// 82EF5A94: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF5A98: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A9C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5AA0: 394ACE8C  addi r10, r10, -0x3174
	ctx.r[10].s64 = ctx.r[10].s64 + -12660;
	// 82EF5AA4: 914B04F8  stw r10, 0x4f8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1272 as u32), ctx.r[10].u32 ) };
	// 82EF5AA8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AAC: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5AB0: 914B04FC  stw r10, 0x4fc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1276 as u32), ctx.r[10].u32 ) };
	// 82EF5AB4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF5ABC: 914B0500  stw r10, 0x500(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1280 as u32), ctx.r[10].u32 ) };
	// 82EF5AC0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5AC8: 994B0504  stb r10, 0x504(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1284 as u32), ctx.r[10].u8 ) };
	// 82EF5ACC: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5AD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5AD4: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5AD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5AE0: 4E800421  bctrl
	ctx.lr = 0x82EF5AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5AE4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5AF8 size=68
    let mut pc: u32 = 0x82EF5AF8;
    'dispatch: loop {
        match pc {
            0x82EF5AF8 => {
    //   block [0x82EF5AF8..0x82EF5B28)
	// 82EF5AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5B04: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5B08: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5B0C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B10: 480000B1  bl 0x82ef5bc0
	ctx.lr = 0x82EF5B14;
	sub_82EF5BC0(ctx, base);
	// 82EF5B14: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5B18: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5B1C: 4182000C  beq 0x82ef5b28
	if ctx.cr[0].eq {
	pc = 0x82EF5B28; continue 'dispatch;
	}
	// 82EF5B20: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B24: 4BFFFBA5  bl 0x82ef56c8
	ctx.lr = 0x82EF5B28;
	sub_82EF56C8(ctx, base);
	pc = 0x82EF5B28; continue 'dispatch;
            }
            0x82EF5B28 => {
    //   block [0x82EF5B28..0x82EF5B3C)
	// 82EF5B28: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5B40 size=28
    let mut pc: u32 = 0x82EF5B40;
    'dispatch: loop {
        match pc {
            0x82EF5B40 => {
    //   block [0x82EF5B40..0x82EF5B5C)
	// 82EF5B40: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5B48: 396BCF08  addi r11, r11, -0x30f8
	ctx.r[11].s64 = ctx.r[11].s64 + -12536;
	// 82EF5B4C: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5B50: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5B54: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5B60 size=68
    let mut pc: u32 = 0x82EF5B60;
    'dispatch: loop {
        match pc {
            0x82EF5B60 => {
    //   block [0x82EF5B60..0x82EF5B90)
	// 82EF5B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5B6C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5B70: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5B74: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B78: 48000031  bl 0x82ef5ba8
	ctx.lr = 0x82EF5B7C;
	sub_82EF5BA8(ctx, base);
	// 82EF5B7C: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5B80: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5B84: 4182000C  beq 0x82ef5b90
	if ctx.cr[0].eq {
	pc = 0x82EF5B90; continue 'dispatch;
	}
	// 82EF5B88: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B8C: 4B94FC25  bl 0x828457b0
	ctx.lr = 0x82EF5B90;
	sub_828457B0(ctx, base);
	pc = 0x82EF5B90; continue 'dispatch;
            }
            0x82EF5B90 => {
    //   block [0x82EF5B90..0x82EF5BA4)
	// 82EF5B90: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5BA8 size=24
    let mut pc: u32 = 0x82EF5BA8;
    'dispatch: loop {
        match pc {
            0x82EF5BA8 => {
    //   block [0x82EF5BA8..0x82EF5BC0)
	// 82EF5BA8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5BAC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5BB0: 396BCF08  addi r11, r11, -0x30f8
	ctx.r[11].s64 = ctx.r[11].s64 + -12536;
	// 82EF5BB4: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5BB8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5BC0 size=132
    let mut pc: u32 = 0x82EF5BC0;
    'dispatch: loop {
        match pc {
            0x82EF5BC0 => {
    //   block [0x82EF5BC0..0x82EF5C1C)
	// 82EF5BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5BCC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5BD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5BD4: 396BCEC8  addi r11, r11, -0x3138
	ctx.r[11].s64 = ctx.r[11].s64 + -12600;
	// 82EF5BD8: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5BDC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5BE0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5BE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5BE8: 394ACEBC  addi r10, r10, -0x3144
	ctx.r[10].s64 = ctx.r[10].s64 + -12612;
	// 82EF5BEC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF5BF0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5BF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5BF8: 394ACE8C  addi r10, r10, -0x3174
	ctx.r[10].s64 = ctx.r[10].s64 + -12660;
	// 82EF5BFC: 914B04F8  stw r10, 0x4f8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1272 as u32), ctx.r[10].u32 ) };
	// 82EF5C00: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5C04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF5C08: 419A0014  beq cr6, 0x82ef5c1c
	if ctx.cr[6].eq {
	pc = 0x82EF5C1C; continue 'dispatch;
	}
	// 82EF5C0C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5C10: 396B04F8  addi r11, r11, 0x4f8
	ctx.r[11].s64 = ctx.r[11].s64 + 1272;
	// 82EF5C14: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5C18: 4800000C  b 0x82ef5c24
	pc = 0x82EF5C24; continue 'dispatch;
            }
            0x82EF5C1C => {
    //   block [0x82EF5C1C..0x82EF5C24)
	// 82EF5C1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5C20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF5C24; continue 'dispatch;
            }
            0x82EF5C24 => {
    //   block [0x82EF5C24..0x82EF5C44)
	// 82EF5C24: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5C28: 4BFFFF81  bl 0x82ef5ba8
	ctx.lr = 0x82EF5C2C;
	sub_82EF5BA8(ctx, base);
	// 82EF5C2C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5C30: 4BFFA889  bl 0x82ef04b8
	ctx.lr = 0x82EF5C34;
	sub_82EF04B8(ctx, base);
	// 82EF5C34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5C48 size=60
    let mut pc: u32 = 0x82EF5C48;
    'dispatch: loop {
        match pc {
            0x82EF5C48 => {
    //   block [0x82EF5C48..0x82EF5C58)
	// 82EF5C48: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5C4C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5C50: 396BFB08  addi r11, r11, -0x4f8
	ctx.r[11].s64 = ctx.r[11].s64 + -1272;
	// 82EF5C54: 396B0500  addi r11, r11, 0x500
	ctx.r[11].s64 = ctx.r[11].s64 + 1280;
	pc = 0x82EF5C58; continue 'dispatch;
            }
            0x82EF5C58 => {
    //   block [0x82EF5C58..0x82EF5C84)
	// 82EF5C58: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82EF5C5C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5C60: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF5C64: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF5C68: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF5C6C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5C70: 4082FFE8  bne 0x82ef5c58
	if !ctx.cr[0].eq {
	pc = 0x82EF5C58; continue 'dispatch;
	}
	// 82EF5C74: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF5C78: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82EF5C7C: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EF5C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5C88 size=344
    let mut pc: u32 = 0x82EF5C88;
    'dispatch: loop {
        match pc {
            0x82EF5C88 => {
    //   block [0x82EF5C88..0x82EF5CF4)
	// 82EF5C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5C90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF5C94: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5C98: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EF5C9C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5CA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF5CA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5CA8: 4199004C  bgt cr6, 0x82ef5cf4
	if ctx.cr[6].gt {
	pc = 0x82EF5CF4; continue 'dispatch;
	}
	// 82EF5CAC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5CB0: 896B8F85  lbz r11, -0x707b(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28795 as u32) ) } as u64;
	// 82EF5CB4: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF5CB8: 4082003C  bne 0x82ef5cf4
	if !ctx.cr[0].eq {
	pc = 0x82EF5CF4; continue 'dispatch;
	}
	// 82EF5CBC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5CC0: 3BEB8F85  addi r31, r11, -0x707b
	ctx.r[31].s64 = ctx.r[11].s64 + -28795;
	// 82EF5CC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5CC8: 38CBC34C  addi r6, r11, -0x3cb4
	ctx.r[6].s64 = ctx.r[11].s64 + -15540;
	// 82EF5CCC: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82EF5CD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5CD4: 388BCF4C  addi r4, r11, -0x30b4
	ctx.r[4].s64 = ctx.r[11].s64 + -12468;
	// 82EF5CD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF5CDC: 4BFF548D  bl 0x82eeb168
	ctx.lr = 0x82EF5CE0;
	sub_82EEB168(ctx, base);
	// 82EF5CE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF5CE4: 481FC0D5  bl 0x830f1db8
	ctx.lr = 0x82EF5CE8;
	sub_830F1DB8(ctx, base);
	// 82EF5CE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5CEC: 40820008  bne 0x82ef5cf4
	if !ctx.cr[0].eq {
	pc = 0x82EF5CF4; continue 'dispatch;
	}
	// 82EF5CF0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82EF5CF4; continue 'dispatch;
            }
            0x82EF5CF4 => {
    //   block [0x82EF5CF4..0x82EF5D00)
	// 82EF5CF4: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5CF8: 396BFB08  addi r11, r11, -0x4f8
	ctx.r[11].s64 = ctx.r[11].s64 + -1272;
	// 82EF5CFC: 396B0500  addi r11, r11, 0x500
	ctx.r[11].s64 = ctx.r[11].s64 + 1280;
	pc = 0x82EF5D00; continue 'dispatch;
            }
            0x82EF5D00 => {
    //   block [0x82EF5D00..0x82EF5D90)
	// 82EF5D00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82EF5D04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5D08: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF5D0C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EF5D10: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF5D14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5D18: 4082FFE8  bne 0x82ef5d00
	if !ctx.cr[0].eq {
	pc = 0x82EF5D00; continue 'dispatch;
	}
	// 82EF5D1C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF5D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82EF5D24: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5D28: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5D2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5D30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5D34: 409A0094  bne cr6, 0x82ef5dc8
	if !ctx.cr[6].eq {
	pc = 0x82EF5DC8; continue 'dispatch;
	}
	// 82EF5D38: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5D3C: 896B000C  lbz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF5D40: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF5D44: 4182004C  beq 0x82ef5d90
	if ctx.cr[0].eq {
	pc = 0x82EF5D90; continue 'dispatch;
	}
	// 82EF5D48: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5D4C: 896B8F84  lbz r11, -0x707c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28796 as u32) ) } as u64;
	// 82EF5D50: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF5D54: 4082003C  bne 0x82ef5d90
	if !ctx.cr[0].eq {
	pc = 0x82EF5D90; continue 'dispatch;
	}
	// 82EF5D58: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5D5C: 3BEB8F84  addi r31, r11, -0x707c
	ctx.r[31].s64 = ctx.r[11].s64 + -28796;
	// 82EF5D60: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5D64: 38CBCF38  addi r6, r11, -0x30c8
	ctx.r[6].s64 = ctx.r[11].s64 + -12488;
	// 82EF5D68: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 82EF5D6C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5D70: 388BCF4C  addi r4, r11, -0x30b4
	ctx.r[4].s64 = ctx.r[11].s64 + -12468;
	// 82EF5D74: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82EF5D78: 4BFF53F1  bl 0x82eeb168
	ctx.lr = 0x82EF5D7C;
	sub_82EEB168(ctx, base);
	// 82EF5D7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF5D80: 481FC039  bl 0x830f1db8
	ctx.lr = 0x82EF5D84;
	sub_830F1DB8(ctx, base);
	// 82EF5D84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5D88: 40820008  bne 0x82ef5d90
	if !ctx.cr[0].eq {
	pc = 0x82EF5D90; continue 'dispatch;
	}
	// 82EF5D8C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82EF5D90; continue 'dispatch;
            }
            0x82EF5D90 => {
    //   block [0x82EF5D90..0x82EF5DC8)
	// 82EF5D90: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5D94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5D98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF5D9C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5DA0: 388BFB08  addi r4, r11, -0x4f8
	ctx.r[4].s64 = ctx.r[11].s64 + -1272;
	// 82EF5DA4: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5DA8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5DAC: 4BFF72D5  bl 0x82eed080
	ctx.lr = 0x82EF5DB0;
	sub_82EED080(ctx, base);
	// 82EF5DB0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5DB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5DB8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5DBC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5DC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5DC4: 4E800421  bctrl
	ctx.lr = 0x82EF5DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF5DC8 => {
    //   block [0x82EF5DC8..0x82EF5DE0)
	// 82EF5DC8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5DCC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF5DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5DD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF5DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5DE0 size=84
    let mut pc: u32 = 0x82EF5DE0;
    'dispatch: loop {
        match pc {
            0x82EF5DE0 => {
    //   block [0x82EF5DE0..0x82EF5E34)
	// 82EF5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5DEC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5DF0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5DF4: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF5DF8: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF5DFC: D0210094  stfs f1, 0x94(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82EF5E00: D041009C  stfs f2, 0x9c(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82EF5E04: C041009C  lfs f2, 0x9c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82EF5E08: C0210094  lfs f1, 0x94(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EF5E0C: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF5E10: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5E14: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5E18: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5E1C: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5E20: 4BFFC261  bl 0x82ef2080
	ctx.lr = 0x82EF5E24;
	sub_82EF2080(ctx, base);
	// 82EF5E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5E38 size=44
    let mut pc: u32 = 0x82EF5E38;
    'dispatch: loop {
        match pc {
            0x82EF5E38 => {
    //   block [0x82EF5E38..0x82EF5E64)
	// 82EF5E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5E44: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5E48: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5E4C: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5E50: 48000019  bl 0x82ef5e68
	ctx.lr = 0x82EF5E54;
	sub_82EF5E68(ctx, base);
	// 82EF5E54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5E68 size=44
    let mut pc: u32 = 0x82EF5E68;
    'dispatch: loop {
        match pc {
            0x82EF5E68 => {
    //   block [0x82EF5E68..0x82EF5E94)
	// 82EF5E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5E74: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5E78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF5E7C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5E80: 4BFFC3B1  bl 0x82ef2230
	ctx.lr = 0x82EF5E84;
	sub_82EF2230(ctx, base);
	// 82EF5E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5E98 size=44
    let mut pc: u32 = 0x82EF5E98;
    'dispatch: loop {
        match pc {
            0x82EF5E98 => {
    //   block [0x82EF5E98..0x82EF5EC4)
	// 82EF5E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5EA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5EA4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5EA8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5EAC: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5EB0: 4BFFC451  bl 0x82ef2300
	ctx.lr = 0x82EF5EB4;
	sub_82EF2300(ctx, base);
	// 82EF5EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5EC8 size=52
    let mut pc: u32 = 0x82EF5EC8;
    'dispatch: loop {
        match pc {
            0x82EF5EC8 => {
    //   block [0x82EF5EC8..0x82EF5EFC)
	// 82EF5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5ED4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5ED8: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82EF5EDC: 8881007F  lbz r4, 0x7f(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(127 as u32) ) } as u64;
	// 82EF5EE0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5EE4: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5EE8: 4BFFC529  bl 0x82ef2410
	ctx.lr = 0x82EF5EEC;
	sub_82EF2410(ctx, base);
	// 82EF5EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5F00 size=140
    let mut pc: u32 = 0x82EF5F00;
    'dispatch: loop {
        match pc {
            0x82EF5F00 => {
    //   block [0x82EF5F00..0x82EF5F8C)
	// 82EF5F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5F0C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5F10: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F14: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F18: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5F1C: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5F20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5F2C: 4E800421  bctrl
	ctx.lr = 0x82EF5F30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5F30: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF5F38: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5F3C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F40: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5F44: 4BFFC7C5  bl 0x82ef2708
	ctx.lr = 0x82EF5F48;
	sub_82EF2708(ctx, base);
	// 82EF5F48: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF5F4C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5F54: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5F58: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F5C: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F60: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5F64: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5F68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F6C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5F74: 4E800421  bctrl
	ctx.lr = 0x82EF5F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5F78: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5F7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5F90 size=140
    let mut pc: u32 = 0x82EF5F90;
    'dispatch: loop {
        match pc {
            0x82EF5F90 => {
    //   block [0x82EF5F90..0x82EF601C)
	// 82EF5F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5F9C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5FA0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FA4: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FA8: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5FAC: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5FB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5FB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5FBC: 4E800421  bctrl
	ctx.lr = 0x82EF5FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5FC0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF5FC8: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5FCC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FD0: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5FD4: 4BFFCABD  bl 0x82ef2a90
	ctx.lr = 0x82EF5FD8;
	sub_82EF2A90(ctx, base);
	// 82EF5FD8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF5FDC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5FE4: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5FE8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FEC: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FF0: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5FF4: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5FF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5FFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6004: 4E800421  bctrl
	ctx.lr = 0x82EF6008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6008: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF600C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6020 size=44
    let mut pc: u32 = 0x82EF6020;
    'dispatch: loop {
        match pc {
            0x82EF6020 => {
    //   block [0x82EF6020..0x82EF604C)
	// 82EF6020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF602C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF6030: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF6034: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF6038: 48000019  bl 0x82ef6050
	ctx.lr = 0x82EF603C;
	sub_82EF6050(ctx, base);
	// 82EF603C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6050 size=16
    let mut pc: u32 = 0x82EF6050;
    'dispatch: loop {
        match pc {
            0x82EF6050 => {
    //   block [0x82EF6050..0x82EF6060)
	// 82EF6050: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF6054: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6058: 806B0044  lwz r3, 0x44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF605C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6060 size=44
    let mut pc: u32 = 0x82EF6060;
    'dispatch: loop {
        match pc {
            0x82EF6060 => {
    //   block [0x82EF6060..0x82EF608C)
	// 82EF6060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF606C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF6070: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF6074: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF6078: 4BFFD8F1  bl 0x82ef3968
	ctx.lr = 0x82EF607C;
	sub_82EF3968(ctx, base);
	// 82EF607C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6090 size=44
    let mut pc: u32 = 0x82EF6090;
    'dispatch: loop {
        match pc {
            0x82EF6090 => {
    //   block [0x82EF6090..0x82EF60BC)
	// 82EF6090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF609C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF60A0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF60A4: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF60A8: 4BFFD941  bl 0x82ef39e8
	ctx.lr = 0x82EF60AC;
	sub_82EF39E8(ctx, base);
	// 82EF60AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF60B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF60B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF60B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF60C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF60C0 size=8
    let mut pc: u32 = 0x82EF60C0;
    'dispatch: loop {
        match pc {
            0x82EF60C0 => {
    //   block [0x82EF60C0..0x82EF60C8)
	// 82EF60C0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF60C4: 4BFFFA34  b 0x82ef5af8
	sub_82EF5AF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF60C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF60C8 size=8
    let mut pc: u32 = 0x82EF60C8;
    'dispatch: loop {
        match pc {
            0x82EF60C8 => {
    //   block [0x82EF60C8..0x82EF60D0)
	// 82EF60C8: 3863FB08  addi r3, r3, -0x4f8
	ctx.r[3].s64 = ctx.r[3].s64 + -1272;
	// 82EF60CC: 4BFFFA2C  b 0x82ef5af8
	sub_82EF5AF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF60D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF60D0 size=116
    let mut pc: u32 = 0x82EF60D0;
    'dispatch: loop {
        match pc {
            0x82EF60D0 => {
    //   block [0x82EF60D0..0x82EF60F0)
	// 82EF60D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF60D4: 4BDB3339  bl 0x82ca940c
	ctx.lr = 0x82EF60D8;
	sub_82CA93D0(ctx, base);
	// 82EF60D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF60DC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EF60E0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EF60E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF60E8: 409A0008  bne cr6, 0x82ef60f0
	if !ctx.cr[6].eq {
	pc = 0x82EF60F0; continue 'dispatch;
	}
	// 82EF60EC: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	pc = 0x82EF60F0; continue 'dispatch;
            }
            0x82EF60F0 => {
    //   block [0x82EF60F0..0x82EF613C)
	// 82EF60F0: 3BDD0002  addi r30, r29, 2
	ctx.r[30].s64 = ctx.r[29].s64 + 2;
	// 82EF60F4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF60F8: 7D7E2214  add r11, r30, r4
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[4].u64;
	// 82EF60FC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6100: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6104: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82EF6108: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF610C: 4E800421  bctrl
	ctx.lr = 0x82EF6110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6110: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF6114: 41820028  beq 0x82ef613c
	if ctx.cr[0].eq {
	pc = 0x82EF613C; continue 'dispatch;
	}
	// 82EF6118: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82EF611C: 395FFFFF  addi r10, r31, -1
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	// 82EF6120: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6124: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF6128: 7D6B5078  andc r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[10].u64;
	// 82EF612C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EF6130: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82EF6134: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF6138: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
            }
            0x82EF613C => {
    //   block [0x82EF613C..0x82EF6144)
	// 82EF613C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6140: 4BDB331C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6148 size=32
    let mut pc: u32 = 0x82EF6148;
    'dispatch: loop {
        match pc {
            0x82EF6148 => {
    //   block [0x82EF6148..0x82EF6168)
	// 82EF6148: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF614C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82EF6150: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6154: A144FFFE  lhz r10, -2(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82EF6158: 7C8A2050  subf r4, r10, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 82EF615C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6164: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6170 size=8
    let mut pc: u32 = 0x82EF6170;
    'dispatch: loop {
        match pc {
            0x82EF6170 => {
    //   block [0x82EF6170..0x82EF6178)
	// 82EF6170: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EF6174: 4BDB5FCC  b 0x82cac140
	sub_82CAC140(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6178 size=12
    let mut pc: u32 = 0x82EF6178;
    'dispatch: loop {
        match pc {
            0x82EF6178 => {
    //   block [0x82EF6178..0x82EF6184)
	// 82EF6178: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EF617C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF6180: 4BDB8E48  b 0x82caefc8
	sub_82CAEFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6188 size=116
    let mut pc: u32 = 0x82EF6188;
    'dispatch: loop {
        match pc {
            0x82EF6188 => {
    //   block [0x82EF6188..0x82EF6190)
	// 82EF6188: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82EF618C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	pc = 0x82EF6190; continue 'dispatch;
            }
            0x82EF6190 => {
    //   block [0x82EF6190..0x82EF61C4)
	// 82EF6190: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 82EF6194: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6198: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EF619C: 916B000C  stw r11, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF61A0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF61A4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF61A8: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EF61AC: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82EF61B0: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82EF61B4: 4082FFDC  bne 0x82ef6190
	if !ctx.cr[0].eq {
	pc = 0x82EF6190; continue 'dispatch;
	}
	// 82EF61B8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF61BC: 39430014  addi r10, r3, 0x14
	ctx.r[10].s64 = ctx.r[3].s64 + 20;
	// 82EF61C0: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	pc = 0x82EF61C4; continue 'dispatch;
            }
            0x82EF61C4 => {
    //   block [0x82EF61C4..0x82EF61E4)
	// 82EF61C4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF61C8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF61CC: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82EF61D0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF61D4: 4082FFF0  bne 0x82ef61c4
	if !ctx.cr[0].eq {
	pc = 0x82EF61C4; continue 'dispatch;
	}
	// 82EF61D8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EF61DC: 39430294  addi r10, r3, 0x294
	ctx.r[10].s64 = ctx.r[3].s64 + 660;
	// 82EF61E0: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	pc = 0x82EF61E4; continue 'dispatch;
            }
            0x82EF61E4 => {
    //   block [0x82EF61E4..0x82EF61FC)
	// 82EF61E4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF61E8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF61EC: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82EF61F0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF61F4: 4082FFF0  bne 0x82ef61e4
	if !ctx.cr[0].eq {
	pc = 0x82EF61E4; continue 'dispatch;
	}
	// 82EF61F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6200 size=212
    let mut pc: u32 = 0x82EF6200;
    'dispatch: loop {
        match pc {
            0x82EF6200 => {
    //   block [0x82EF6200..0x82EF6230)
	// 82EF6200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF620C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF621C: 817F0504  lwz r11, 0x504(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 82EF6220: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6224: 409A000C  bne cr6, 0x82ef6230
	if !ctx.cr[6].eq {
	pc = 0x82EF6230; continue 'dispatch;
	}
	// 82EF6228: 48007629  bl 0x82efd850
	ctx.lr = 0x82EF622C;
	sub_82EFD850(ctx, base);
	// 82EF622C: 907F0504  stw r3, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF6230; continue 'dispatch;
            }
            0x82EF6230 => {
    //   block [0x82EF6230..0x82EF62B8)
	// 82EF6230: 807F0504  lwz r3, 0x504(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 82EF6234: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EF6238: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF623C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6240: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82EF6244: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6248: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EF624C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6250: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF6254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6258: 4E800421  bctrl
	ctx.lr = 0x82EF625C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF625C: 7C6A1B79  or. r10, r3, r3
	ctx.r[10].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6260: 41820058  beq 0x82ef62b8
	if ctx.cr[0].eq {
	pc = 0x82EF62B8; continue 'dispatch;
	}
	// 82EF6264: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6268: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 82EF626C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6270: 391F0004  addi r8, r31, 4
	ctx.r[8].s64 = ctx.r[31].s64 + 4;
	// 82EF6274: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF6278: 57C6063E  clrlwi r6, r30, 0x18
	ctx.r[6].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82EF627C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF6280: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF6284: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6288: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF628C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6290: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF6294: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6298: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF629C: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EF62A0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF62A4: 98CA0002  stb r6, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 82EF62A8: 98AA0003  stb r5, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 82EF62AC: 98CBFFFE  stb r6, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[6].u8 ) };
	// 82EF62B0: 988BFFFF  stb r4, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[4].u8 ) };
	// 82EF62B4: 48000008  b 0x82ef62bc
	pc = 0x82EF62BC; continue 'dispatch;
            }
            0x82EF62B8 => {
    //   block [0x82EF62B8..0x82EF62BC)
	// 82EF62B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF62BC; continue 'dispatch;
            }
            0x82EF62BC => {
    //   block [0x82EF62BC..0x82EF62D4)
	// 82EF62BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF62C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF62C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF62C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF62CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF62D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF62D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF62D8 size=156
    let mut pc: u32 = 0x82EF62D8;
    'dispatch: loop {
        match pc {
            0x82EF62D8 => {
    //   block [0x82EF62D8..0x82EF6308)
	// 82EF62D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF62DC: 4BDB312D  bl 0x82ca9408
	ctx.lr = 0x82EF62E0;
	sub_82CA93D0(ctx, base);
	// 82EF62E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF62E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF62E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF62EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF62F0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EF62F4: 817E0508  lwz r11, 0x508(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1288 as u32) ) } as u64;
	// 82EF62F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF62FC: 409A000C  bne cr6, 0x82ef6308
	if !ctx.cr[6].eq {
	pc = 0x82EF6308; continue 'dispatch;
	}
	// 82EF6300: 48007551  bl 0x82efd850
	ctx.lr = 0x82EF6304;
	sub_82EFD850(ctx, base);
	// 82EF6304: 907E0508  stw r3, 0x508(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1288 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF6308; continue 'dispatch;
            }
            0x82EF6308 => {
    //   block [0x82EF6308..0x82EF6334)
	// 82EF6308: 807E0508  lwz r3, 0x508(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1288 as u32) ) } as u64;
	// 82EF630C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF6310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6314: 419A0020  beq cr6, 0x82ef6334
	if ctx.cr[6].eq {
	pc = 0x82EF6334; continue 'dispatch;
	}
	// 82EF6318: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF631C: 38DD0008  addi r6, r29, 8
	ctx.r[6].s64 = ctx.r[29].s64 + 8;
	// 82EF6320: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6324: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF6328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF632C: 4E800421  bctrl
	ctx.lr = 0x82EF6330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6330: 48000018  b 0x82ef6348
	pc = 0x82EF6348; continue 'dispatch;
            }
            0x82EF6334 => {
    //   block [0x82EF6334..0x82EF6348)
	// 82EF6334: 7FFFEA14  add r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82EF6338: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF633C: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82EF6340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6344: 4E800421  bctrl
	ctx.lr = 0x82EF6348;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF6348 => {
    //   block [0x82EF6348..0x82EF636C)
	// 82EF6348: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EF634C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6350: 419A001C  beq cr6, 0x82ef636c
	if ctx.cr[6].eq {
	pc = 0x82EF636C; continue 'dispatch;
	}
	// 82EF6354: 394000B1  li r10, 0xb1
	ctx.r[10].s64 = 177;
	// 82EF6358: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EF635C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF6360: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EF6364: 994B0006  stb r10, 6(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u8 ) };
	// 82EF6368: 992B0007  stb r9, 7(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(7 as u32), ctx.r[9].u8 ) };
	pc = 0x82EF636C; continue 'dispatch;
            }
            0x82EF636C => {
    //   block [0x82EF636C..0x82EF6374)
	// 82EF636C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF6370: 4BDB30E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6378 size=684
    let mut pc: u32 = 0x82EF6378;
    'dispatch: loop {
        match pc {
            0x82EF6378 => {
    //   block [0x82EF6378..0x82EF63F4)
	// 82EF6378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF637C: 4BDB307D  bl 0x82ca93f8
	ctx.lr = 0x82EF6380;
	sub_82CA93D0(ctx, base);
	// 82EF6380: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6384: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EF6388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF638C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF6390: 3B660004  addi r27, r6, 4
	ctx.r[27].s64 = ctx.r[6].s64 + 4;
	// 82EF6394: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF6398: 419A0068  beq cr6, 0x82ef6400
	if ctx.cr[6].eq {
	pc = 0x82EF6400; continue 'dispatch;
	}
	// 82EF639C: 2B1A1000  cmplwi cr6, r26, 0x1000
	ctx.cr[6].compare_u32(ctx.r[26].u32, 4096 as u32, &mut ctx.xer);
	// 82EF63A0: 40990054  ble cr6, 0x82ef63f4
	if !ctx.cr[6].gt {
	pc = 0x82EF63F4; continue 'dispatch;
	}
	// 82EF63A4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EF63A8: 4BFFFF31  bl 0x82ef62d8
	ctx.lr = 0x82EF63AC;
	sub_82EF62D8(ctx, base);
	// 82EF63AC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF63B0: 41820150  beq 0x82ef6500
	if ctx.cr[0].eq {
	pc = 0x82EF6500; continue 'dispatch;
	}
	// 82EF63B4: 3BDF0550  addi r30, r31, 0x550
	ctx.r[30].s64 = ctx.r[31].s64 + 1360;
	// 82EF63B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF63BC: 483C35A9  bl 0x832b9964
	ctx.lr = 0x82EF63C0;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF63C0: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82EF63C4: E8FF0530  ld r7, 0x530(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) };
	// 82EF63C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF63CC: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 82EF63D0: E93F0520  ld r9, 0x520(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF63D4: 79680020  clrldi r8, r11, 0x20
	ctx.r[8].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF63D8: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82EF63DC: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EF63E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF63E4: F91F0530  std r8, 0x530(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1328 as u32), ctx.r[8].u64 ) };
	// 82EF63E8: F97F0520  std r11, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u64 ) };
	// 82EF63EC: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF63F0: 48000224  b 0x82ef6614
	pc = 0x82EF6614; continue 'dispatch;
            }
            0x82EF63F4 => {
    //   block [0x82EF63F4..0x82EF6400)
	// 82EF63F4: 2B1A0004  cmplwi cr6, r26, 4
	ctx.cr[6].compare_u32(ctx.r[26].u32, 4 as u32, &mut ctx.xer);
	// 82EF63F8: 40980008  bge cr6, 0x82ef6400
	if !ctx.cr[6].lt {
	pc = 0x82EF6400; continue 'dispatch;
	}
	// 82EF63FC: 3B400004  li r26, 4
	ctx.r[26].s64 = 4;
	pc = 0x82EF6400; continue 'dispatch;
            }
            0x82EF6400 => {
    //   block [0x82EF6400..0x82EF6450)
	// 82EF6400: 7D7CD214  add r11, r28, r26
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[26].u64;
	// 82EF6404: 3B1F0550  addi r24, r31, 0x550
	ctx.r[24].s64 = ctx.r[31].s64 + 1360;
	// 82EF6408: 7F2BDA14  add r25, r11, r27
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EF640C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF6410: 39790001  addi r11, r25, 1
	ctx.r[11].s64 = ctx.r[25].s64 + 1;
	// 82EF6414: 616B000F  ori r11, r11, 0xf
	ctx.r[11].u64 = ctx.r[11].u64 | 15;
	// 82EF6418: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF641C: 23AB001C  subfic r29, r11, 0x1c
	ctx.xer.ca = ctx.r[11].u32 <= 28 as u32;
	ctx.r[29].s64 = (28 as i64) - ctx.r[11].s64;
	// 82EF6420: 483C3545  bl 0x832b9964
	ctx.lr = 0x82EF6424;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6424: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 82EF6428: 817F0510  lwz r11, 0x510(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1296 as u32) ) } as u64;
	// 82EF642C: 1D4A0014  mulli r10, r10, 0x14
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 20 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82EF6430: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF6434: 917F0510  stw r11, 0x510(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1296 as u32), ctx.r[11].u32 ) };
	// 82EF6438: 7D6AF82E  lwzx r11, r10, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF643C: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82EF6440: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82EF6444: 4198000C  blt cr6, 0x82ef6450
	if ctx.cr[6].lt {
	pc = 0x82EF6450; continue 'dispatch;
	}
	// 82EF6448: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EF644C: 4800000C  b 0x82ef6458
	pc = 0x82EF6458; continue 'dispatch;
            }
            0x82EF6450 => {
    //   block [0x82EF6450..0x82EF6458)
	// 82EF6450: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82EF6454: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	pc = 0x82EF6458; continue 'dispatch;
            }
            0x82EF6458 => {
    //   block [0x82EF6458..0x82EF6474)
	// 82EF6458: 1D7D0014  mulli r11, r29, 0x14
	ctx.r[11].s32 = ((ctx.r[29].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EF645C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6460: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82EF6464: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF6468: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF646C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6470: 4800001C  b 0x82ef648c
	pc = 0x82EF648C; continue 'dispatch;
            }
            0x82EF6474 => {
    //   block [0x82EF6474..0x82EF648C)
	// 82EF6474: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF6478: 41990024  bgt cr6, 0x82ef649c
	if ctx.cr[6].gt {
	pc = 0x82EF649C; continue 'dispatch;
	}
	// 82EF647C: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82EF6480: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EF6484: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6488: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82EF648C; continue 'dispatch;
            }
            0x82EF648C => {
    //   block [0x82EF648C..0x82EF649C)
	// 82EF648C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EF6490: 419AFFE4  beq cr6, 0x82ef6474
	if ctx.cr[6].eq {
	pc = 0x82EF6474; continue 'dispatch;
	}
	// 82EF6494: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF6498: 4099008C  ble cr6, 0x82ef6524
	if !ctx.cr[6].gt {
	pc = 0x82EF6524; continue 'dispatch;
	}
	pc = 0x82EF649C; continue 'dispatch;
            }
            0x82EF649C => {
    //   block [0x82EF649C..0x82EF64FC)
	// 82EF649C: 7F1D4800  cmpw cr6, r29, r9
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF64A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF64A4: 40990064  ble cr6, 0x82ef6508
	if !ctx.cr[6].gt {
	pc = 0x82EF6508; continue 'dispatch;
	}
	// 82EF64A8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EF64AC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EF64B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF64B4: 4BFFFE25  bl 0x82ef62d8
	ctx.lr = 0x82EF64B8;
	sub_82EF62D8(ctx, base);
	// 82EF64B8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF64BC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF64C0: 4182003C  beq 0x82ef64fc
	if ctx.cr[0].eq {
	pc = 0x82EF64FC; continue 'dispatch;
	}
	// 82EF64C4: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82EF64C8: E8FF0530  ld r7, 0x530(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) };
	// 82EF64CC: E93F0520  ld r9, 0x520(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF64D0: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 82EF64D4: 79680020  clrldi r8, r11, 0x20
	ctx.r[8].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF64D8: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82EF64DC: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EF64E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF64E4: F91F0530  std r8, 0x530(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1328 as u32), ctx.r[8].u64 ) };
	// 82EF64E8: F97F0520  std r11, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u64 ) };
	// 82EF64EC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF64F0: 483C3465  bl 0x832b9954
	ctx.lr = 0x82EF64F4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF64F4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82EF64F8: 48000124  b 0x82ef661c
	pc = 0x82EF661C; continue 'dispatch;
            }
            0x82EF64FC => {
    //   block [0x82EF64FC..0x82EF6500)
	// 82EF64FC: 483C3459  bl 0x832b9954
	ctx.lr = 0x82EF6500;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x82EF6500; continue 'dispatch;
            }
            0x82EF6500 => {
    //   block [0x82EF6500..0x82EF6508)
	// 82EF6500: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF6504: 48000118  b 0x82ef661c
	pc = 0x82EF661C; continue 'dispatch;
            }
            0x82EF6508 => {
    //   block [0x82EF6508..0x82EF6524)
	// 82EF6508: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82EF650C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF6510: 4BFFFCF1  bl 0x82ef6200
	ctx.lr = 0x82EF6514;
	sub_82EF6200(ctx, base);
	// 82EF6514: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF6518: 4082000C  bne 0x82ef6524
	if !ctx.cr[0].eq {
	pc = 0x82EF6524; continue 'dispatch;
	}
	// 82EF651C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF6520: 4BFFFFDC  b 0x82ef64fc
	pc = 0x82EF64FC; continue 'dispatch;
            }
            0x82EF6524 => {
    //   block [0x82EF6524..0x82EF654C)
	// 82EF6524: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EF6528: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF652C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF6530: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6534: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6538: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF653C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6540: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6544: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF6548: 48000044  b 0x82ef658c
	pc = 0x82EF658C; continue 'dispatch;
            }
            0x82EF654C => {
    //   block [0x82EF654C..0x82EF658C)
	// 82EF654C: 892BFFFF  lbz r9, -1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82EF6550: 394AFFEC  addi r10, r10, -0x14
	ctx.r[10].s64 = ctx.r[10].s64 + -20;
	// 82EF6554: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82EF6558: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EF655C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EF6560: 992BFFFF  stb r9, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 82EF6564: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF6568: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF656C: 9BC9FFFE  stb r30, -2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-2 as u32), ctx.r[30].u8 ) };
	// 82EF6570: 98E9FFFF  stb r7, -1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-1 as u32), ctx.r[7].u8 ) };
	// 82EF6574: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6578: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF657C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6580: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6584: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF6588: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82EF658C; continue 'dispatch;
            }
            0x82EF658C => {
    //   block [0x82EF658C..0x82EF65E0)
	// 82EF658C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EF6590: 4199FFBC  bgt cr6, 0x82ef654c
	if ctx.cr[6].gt {
	pc = 0x82EF654C; continue 'dispatch;
	}
	// 82EF6594: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82EF6598: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF659C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82EF65A0: 512A3830  rlwimi r10, r9, 7, 0, 0x18
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(7) as u64) & 0x00000000FFFFFF80) | (ctx.r[10].u64 & 0xFFFFFFFF0000007F);
	// 82EF65A4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF65A8: 994BFFFE  stb r10, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u8 ) };
	// 82EF65AC: 419A0034  beq cr6, 0x82ef65e0
	if ctx.cr[6].eq {
	pc = 0x82EF65E0; continue 'dispatch;
	}
	// 82EF65B0: 7D4BD214  add r10, r11, r26
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82EF65B4: 393AFFFF  addi r9, r26, -1
	ctx.r[9].s64 = ctx.r[26].s64 + -1;
	// 82EF65B8: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82EF65BC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EF65C0: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 82EF65C4: 7D5B5050  subf r10, r27, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[27].s64;
	// 82EF65C8: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF65CC: 41820014  beq 0x82ef65e0
	if ctx.cr[0].eq {
	pc = 0x82EF65E0; continue 'dispatch;
	}
	// 82EF65D0: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82EF65D4: B16AFFFC  sth r11, -4(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u16 ) };
	// 82EF65D8: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82EF65DC: 992AFFFF  stb r9, -1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	pc = 0x82EF65E0; continue 'dispatch;
            }
            0x82EF65E0 => {
    //   block [0x82EF65E0..0x82EF6614)
	// 82EF65E0: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 82EF65E4: E91F0530  ld r8, 0x530(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) };
	// 82EF65E8: 3979FFFC  addi r11, r25, -4
	ctx.r[11].s64 = ctx.r[25].s64 + -4;
	// 82EF65EC: 1D4A0014  mulli r10, r10, 0x14
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 20 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82EF65F0: 7D2AFAAA  lwax r9, r10, r31
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as i32) as i64;
	// 82EF65F4: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82EF65F8: E93F0520  ld r9, 0x520(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF65FC: 796A0020  clrldi r10, r11, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF6600: F91F0530  std r8, 0x530(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1328 as u32), ctx.r[8].u64 ) };
	// 82EF6604: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF6608: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF660C: F95F0520  std r10, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[10].u64 ) };
	// 82EF6610: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF6614; continue 'dispatch;
            }
            0x82EF6614 => {
    //   block [0x82EF6614..0x82EF661C)
	// 82EF6614: 483C3341  bl 0x832b9954
	ctx.lr = 0x82EF6618;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6618: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	pc = 0x82EF661C; continue 'dispatch;
            }
            0x82EF661C => {
    //   block [0x82EF661C..0x82EF6624)
	// 82EF661C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF6620: 4BDB2E28  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6628 size=400
    let mut pc: u32 = 0x82EF6628;
    'dispatch: loop {
        match pc {
            0x82EF6628 => {
    //   block [0x82EF6628..0x82EF6660)
	// 82EF6628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF662C: 4BDB2DD5  bl 0x82ca9400
	ctx.lr = 0x82EF6630;
	sub_82CA93D0(ctx, base);
	// 82EF6630: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6638: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF663C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6640: 419A0170  beq cr6, 0x82ef67b0
	if ctx.cr[6].eq {
	pc = 0x82EF67B0; continue 'dispatch;
	}
	// 82EF6644: 8944FFFB  lbz r10, -5(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-5 as u32) ) } as u64;
	// 82EF6648: 3964FFFC  addi r11, r4, -4
	ctx.r[11].s64 = ctx.r[4].s64 + -4;
	// 82EF664C: 8384FFFC  lwz r28, -4(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF6650: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6654: 4182000C  beq 0x82ef6660
	if ctx.cr[0].eq {
	pc = 0x82EF6660; continue 'dispatch;
	}
	// 82EF6658: A14BFFFC  lhz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF665C: 48000008  b 0x82ef6664
	pc = 0x82EF6664; continue 'dispatch;
            }
            0x82EF6660 => {
    //   block [0x82EF6660..0x82EF6664)
	// 82EF6660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82EF6664; continue 'dispatch;
            }
            0x82EF6664 => {
    //   block [0x82EF6664..0x82EF66CC)
	// 82EF6664: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82EF6668: 3B5F0550  addi r26, r31, 0x550
	ctx.r[26].s64 = ctx.r[31].s64 + 1360;
	// 82EF666C: 7FAA5850  subf r29, r10, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF6670: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF6674: 897DFFFE  lbz r11, -2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82EF6678: 557E067E  clrlwi r30, r11, 0x19
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82EF667C: 483C32E9  bl 0x832b9964
	ctx.lr = 0x82EF6680;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6680: 815F0514  lwz r10, 0x514(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1300 as u32) ) } as u64;
	// 82EF6684: 7B8B0020  clrldi r11, r28, 0x20
	ctx.r[11].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 82EF6688: E93F0528  ld r9, 0x528(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1320 as u32) ) };
	// 82EF668C: 2B1E0030  cmplwi cr6, r30, 0x30
	ctx.cr[6].compare_u32(ctx.r[30].u32, 48 as u32, &mut ctx.xer);
	// 82EF6690: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF6694: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF6698: 915F0514  stw r10, 0x514(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1300 as u32), ctx.r[10].u32 ) };
	// 82EF669C: F93F0528  std r9, 0x528(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1320 as u32), ctx.r[9].u64 ) };
	// 82EF66A0: E95F0538  ld r10, 0x538(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1336 as u32) ) };
	// 82EF66A4: 40990028  ble cr6, 0x82ef66cc
	if !ctx.cr[6].gt {
	pc = 0x82EF66CC; continue 'dispatch;
	}
	// 82EF66A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF66AC: F97F0538  std r11, 0x538(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1336 as u32), ctx.r[11].u64 ) };
	// 82EF66B0: 807F0508  lwz r3, 0x508(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1288 as u32) ) } as u64;
	// 82EF66B4: 5769063F  clrlwi. r9, r27, 0x18
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF66B8: 389DFFF8  addi r4, r29, -8
	ctx.r[4].s64 = ctx.r[29].s64 + -8;
	// 82EF66BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF66C0: 408200B0  bne 0x82ef6770
	if !ctx.cr[0].eq {
	pc = 0x82EF6770; continue 'dispatch;
	}
	// 82EF66C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF66C8: 480000AC  b 0x82ef6774
	pc = 0x82EF6774; continue 'dispatch;
            }
            0x82EF66CC => {
    //   block [0x82EF66CC..0x82EF66F0)
	// 82EF66CC: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82EF66D0: 57C906B4  rlwinm r9, r30, 0, 0x1a, 0x1a
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF66D4: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EF66D8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF66DC: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82EF66E0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82EF66E4: E96B0002  lwa r11, 0(r11)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as i32) as i64;
	// 82EF66E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF66EC: F97F0538  std r11, 0x538(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1336 as u32), ctx.r[11].u64 ) };
	pc = 0x82EF66F0; continue 'dispatch;
            }
            0x82EF66F0 => {
    //   block [0x82EF66F0..0x82EF6708)
	// 82EF66F0: 895DFFFF  lbz r10, -1(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82EF66F4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF66F8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF66FC: 4082000C  bne 0x82ef6708
	if !ctx.cr[0].eq {
	pc = 0x82EF6708; continue 'dispatch;
	}
	// 82EF6700: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82EF6704: 48000008  b 0x82ef670c
	pc = 0x82EF670C; continue 'dispatch;
            }
            0x82EF6708 => {
    //   block [0x82EF6708..0x82EF670C)
	// 82EF6708: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	pc = 0x82EF670C; continue 'dispatch;
            }
            0x82EF670C => {
    //   block [0x82EF670C..0x82EF6748)
	// 82EF670C: 7F1E4840  cmplw cr6, r30, r9
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF6710: 41990054  bgt cr6, 0x82ef6764
	if ctx.cr[6].gt {
	pc = 0x82EF6764; continue 'dispatch;
	}
	// 82EF6714: 894BFFFE  lbz r10, -2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82EF6718: 57C7063E  clrlwi r7, r30, 0x18
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82EF671C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF6720: 409A0060  bne cr6, 0x82ef6780
	if !ctx.cr[6].eq {
	pc = 0x82EF6780; continue 'dispatch;
	}
	// 82EF6724: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6728: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82EF672C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6730: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF6734: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6738: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF673C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EF6740: 40980008  bge cr6, 0x82ef6748
	if !ctx.cr[6].lt {
	pc = 0x82EF6748; continue 'dispatch;
	}
	// 82EF6744: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	pc = 0x82EF6748; continue 'dispatch;
            }
            0x82EF6748 => {
    //   block [0x82EF6748..0x82EF6764)
	// 82EF6748: 897DFFFF  lbz r11, -1(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82EF674C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EF6750: 39080014  addi r8, r8, 0x14
	ctx.r[8].s64 = ctx.r[8].s64 + 20;
	// 82EF6754: 396B00FF  addi r11, r11, 0xff
	ctx.r[11].s64 = ctx.r[11].s64 + 255;
	// 82EF6758: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF675C: 997DFFFF  stb r11, -1(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(-1 as u32), ctx.r[11].u8 ) };
	// 82EF6760: 4BFFFF90  b 0x82ef66f0
	pc = 0x82EF66F0; continue 'dispatch;
            }
            0x82EF6764 => {
    //   block [0x82EF6764..0x82EF6770)
	// 82EF6764: 807F0504  lwz r3, 0x504(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 82EF6768: 389DFFFC  addi r4, r29, -4
	ctx.r[4].s64 = ctx.r[29].s64 + -4;
	// 82EF676C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82EF6770; continue 'dispatch;
            }
            0x82EF6770 => {
    //   block [0x82EF6770..0x82EF6774)
	// 82EF6770: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	pc = 0x82EF6774; continue 'dispatch;
            }
            0x82EF6774 => {
    //   block [0x82EF6774..0x82EF6780)
	// 82EF6774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6778: 4E800421  bctrl
	ctx.lr = 0x82EF677C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF677C: 4800002C  b 0x82ef67a8
	pc = 0x82EF67A8; continue 'dispatch;
            }
            0x82EF6780 => {
    //   block [0x82EF6780..0x82EF67A8)
	// 82EF6780: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EF6784: 9BDDFFFE  stb r30, -2(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(-2 as u32), ctx.r[30].u8 ) };
	// 82EF6788: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF678C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF6790: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6794: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF6798: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF679C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF67A0: 93AA0004  stw r29, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EF67A4: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	pc = 0x82EF67A8; continue 'dispatch;
            }
            0x82EF67A8 => {
    //   block [0x82EF67A8..0x82EF67B0)
	// 82EF67A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF67AC: 483C31A9  bl 0x832b9954
	ctx.lr = 0x82EF67B0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x82EF67B0; continue 'dispatch;
            }
            0x82EF67B0 => {
    //   block [0x82EF67B0..0x82EF67B8)
	// 82EF67B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF67B4: 4BDB2C9C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF67B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF67B8 size=420
    let mut pc: u32 = 0x82EF67B8;
    'dispatch: loop {
        match pc {
            0x82EF67B8 => {
    //   block [0x82EF67B8..0x82EF67E0)
	// 82EF67B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF67BC: 4BDB2C45  bl 0x82ca9400
	ctx.lr = 0x82EF67C0;
	sub_82CA93D0(ctx, base);
	// 82EF67C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF67C4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EF67C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF67CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF67D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF67D4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF67D8: 419A0008  beq cr6, 0x82ef67e0
	if ctx.cr[6].eq {
	pc = 0x82EF67E0; continue 'dispatch;
	}
	// 82EF67DC: 83BAFFFC  lwz r29, -4(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	pc = 0x82EF67E0; continue 'dispatch;
            }
            0x82EF67E0 => {
    //   block [0x82EF67E0..0x82EF6800)
	// 82EF67E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF67E4: 409A0024  bne cr6, 0x82ef6808
	if !ctx.cr[6].eq {
	pc = 0x82EF6808; continue 'dispatch;
	}
	// 82EF67E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF67EC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF67F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF67F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF67F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF67FC: 4E800421  bctrl
	ctx.lr = 0x82EF6800;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF6800 => {
    //   block [0x82EF6800..0x82EF6808)
	// 82EF6800: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF6804: 48000150  b 0x82ef6954
	pc = 0x82EF6954; continue 'dispatch;
            }
            0x82EF6808 => {
    //   block [0x82EF6808..0x82EF6828)
	// 82EF6808: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF680C: 419A0034  beq cr6, 0x82ef6840
	if ctx.cr[6].eq {
	pc = 0x82EF6840; continue 'dispatch;
	}
	// 82EF6810: 897AFFFA  lbz r11, -6(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(-6 as u32) ) } as u64;
	// 82EF6814: 556B067E  clrlwi r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82EF6818: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 82EF681C: 4099000C  ble cr6, 0x82ef6828
	if !ctx.cr[6].gt {
	pc = 0x82EF6828; continue 'dispatch;
	}
	// 82EF6820: 817AFFF4  lwz r11, -0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EF6824: 48000014  b 0x82ef6838
	pc = 0x82EF6838; continue 'dispatch;
            }
            0x82EF6828 => {
    //   block [0x82EF6828..0x82EF6838)
	// 82EF6828: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF682C: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 20 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EF6830: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF6834: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	pc = 0x82EF6838; continue 'dispatch;
            }
            0x82EF6838 => {
    //   block [0x82EF6838..0x82EF6840)
	// 82EF6838: 3B8BFFFC  addi r28, r11, -4
	ctx.r[28].s64 = ctx.r[11].s64 + -4;
	// 82EF683C: 48000008  b 0x82ef6844
	pc = 0x82EF6844; continue 'dispatch;
            }
            0x82EF6840 => {
    //   block [0x82EF6840..0x82EF6844)
	// 82EF6840: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	pc = 0x82EF6844; continue 'dispatch;
            }
            0x82EF6844 => {
    //   block [0x82EF6844..0x82EF6890)
	// 82EF6844: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF6848: 41990084  bgt cr6, 0x82ef68cc
	if ctx.cr[6].gt {
	pc = 0x82EF68CC; continue 'dispatch;
	}
	// 82EF684C: 578BF87E  srwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF6850: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF6854: 40990070  ble cr6, 0x82ef68c4
	if !ctx.cr[6].gt {
	pc = 0x82EF68C4; continue 'dispatch;
	}
	// 82EF6858: 3B9F0550  addi r28, r31, 0x550
	ctx.r[28].s64 = ctx.r[31].s64 + 1360;
	// 82EF685C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF6860: 483C3105  bl 0x832b9964
	ctx.lr = 0x82EF6864;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6864: 817F0518  lwz r11, 0x518(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1304 as u32) ) } as u64;
	// 82EF6868: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82EF686C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF6870: 917F0518  stw r11, 0x518(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1304 as u32), ctx.r[11].u32 ) };
	// 82EF6874: 4099001C  ble cr6, 0x82ef6890
	if !ctx.cr[6].gt {
	pc = 0x82EF6890; continue 'dispatch;
	}
	// 82EF6878: 7D7DF050  subf r11, r29, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82EF687C: E95F0520  ld r10, 0x520(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF6880: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF6884: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF6888: F97F0520  std r11, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u64 ) };
	// 82EF688C: 48000018  b 0x82ef68a4
	pc = 0x82EF68A4; continue 'dispatch;
            }
            0x82EF6890 => {
    //   block [0x82EF6890..0x82EF68A4)
	// 82EF6890: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82EF6894: E95F0528  ld r10, 0x528(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1320 as u32) ) };
	// 82EF6898: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF689C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF68A0: F97F0528  std r11, 0x528(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1320 as u32), ctx.r[11].u64 ) };
	pc = 0x82EF68A4; continue 'dispatch;
            }
            0x82EF68A4 => {
    //   block [0x82EF68A4..0x82EF68C4)
	// 82EF68A4: 817AFFFC  lwz r11, -4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF68A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF68AC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EF68B0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF68B4: 917AFFFC  stw r11, -4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82EF68B8: 483C309D  bl 0x832b9954
	ctx.lr = 0x82EF68BC;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF68BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF68C0: 48000094  b 0x82ef6954
	pc = 0x82EF6954; continue 'dispatch;
            }
            0x82EF68C4 => {
    //   block [0x82EF68C4..0x82EF68CC)
	// 82EF68C4: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82EF68C8: 48000008  b 0x82ef68d0
	pc = 0x82EF68D0; continue 'dispatch;
            }
            0x82EF68CC => {
    //   block [0x82EF68CC..0x82EF68D0)
	// 82EF68CC: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	pc = 0x82EF68D0; continue 'dispatch;
            }
            0x82EF68D0 => {
    //   block [0x82EF68D0..0x82EF6920)
	// 82EF68D0: 3BBF0550  addi r29, r31, 0x550
	ctx.r[29].s64 = ctx.r[31].s64 + 1360;
	// 82EF68D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF68D8: 483C308D  bl 0x832b9964
	ctx.lr = 0x82EF68DC;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF68DC: 817F0518  lwz r11, 0x518(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1304 as u32) ) } as u64;
	// 82EF68E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF68E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF68E8: 917F0518  stw r11, 0x518(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1304 as u32), ctx.r[11].u32 ) };
	// 82EF68EC: 483C3069  bl 0x832b9954
	ctx.lr = 0x82EF68F0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF68F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF68F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF68F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF68FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6904: 4E800421  bctrl
	ctx.lr = 0x82EF6908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6908: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF690C: 40820014  bne 0x82ef6920
	if !ctx.cr[0].eq {
	pc = 0x82EF6920; continue 'dispatch;
	}
	// 82EF6910: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF6914: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF6918: 4099003C  ble cr6, 0x82ef6954
	if !ctx.cr[6].gt {
	pc = 0x82EF6954; continue 'dispatch;
	}
	// 82EF691C: 4BFFFEE4  b 0x82ef6800
	pc = 0x82EF6800; continue 'dispatch;
            }
            0x82EF6920 => {
    //   block [0x82EF6920..0x82EF6950)
	// 82EF6920: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EF6924: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF6928: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF692C: 4BDB2B55  bl 0x82ca9480
	ctx.lr = 0x82EF6930;
	sub_82CA9480(ctx, base);
	// 82EF6930: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF6934: 419A001C  beq cr6, 0x82ef6950
	if ctx.cr[6].eq {
	pc = 0x82EF6950; continue 'dispatch;
	}
	// 82EF6938: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF693C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF6940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6944: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF694C: 4E800421  bctrl
	ctx.lr = 0x82EF6950;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF6950 => {
    //   block [0x82EF6950..0x82EF6954)
	// 82EF6950: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x82EF6954; continue 'dispatch;
            }
            0x82EF6954 => {
    //   block [0x82EF6954..0x82EF695C)
	// 82EF6954: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF6958: 4BDB2AF8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6960 size=12
    let mut pc: u32 = 0x82EF6960;
    'dispatch: loop {
        match pc {
            0x82EF6960 => {
    //   block [0x82EF6960..0x82EF696C)
	// 82EF6960: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF6964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF6968: 4BFFFA10  b 0x82ef6378
	sub_82EF6378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6970 size=8
    let mut pc: u32 = 0x82EF6970;
    'dispatch: loop {
        match pc {
            0x82EF6970 => {
    //   block [0x82EF6970..0x82EF6978)
	// 82EF6970: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF6974: 4BFFFCB4  b 0x82ef6628
	sub_82EF6628(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6978 size=16
    let mut pc: u32 = 0x82EF6978;
    'dispatch: loop {
        match pc {
            0x82EF6978 => {
    //   block [0x82EF6978..0x82EF6984)
	// 82EF6978: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EF697C: 409A0008  bne cr6, 0x82ef6984
	if !ctx.cr[6].eq {
	pc = 0x82EF6984; continue 'dispatch;
	}
	// 82EF6980: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	pc = 0x82EF6984; continue 'dispatch;
            }
            0x82EF6984 => {
    //   block [0x82EF6984..0x82EF6988)
	// 82EF6984: 4BFFF9F4  b 0x82ef6378
	sub_82EF6378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6988 size=8
    let mut pc: u32 = 0x82EF6988;
    'dispatch: loop {
        match pc {
            0x82EF6988 => {
    //   block [0x82EF6988..0x82EF6990)
	// 82EF6988: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF698C: 4BFFFC9C  b 0x82ef6628
	sub_82EF6628(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6990 size=100
    let mut pc: u32 = 0x82EF6990;
    'dispatch: loop {
        match pc {
            0x82EF6990 => {
    //   block [0x82EF6990..0x82EF69E0)
	// 82EF6990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF699C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF69A0: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF69A4: 817F8F88  lwz r11, -0x7078(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF69A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF69AC: 409A0034  bne cr6, 0x82ef69e0
	if !ctx.cr[6].eq {
	pc = 0x82EF69E0; continue 'dispatch;
	}
	// 82EF69B0: 48006EA1  bl 0x82efd850
	ctx.lr = 0x82EF69B4;
	sub_82EFD850(ctx, base);
	// 82EF69B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF69B8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF69BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF69C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF69C4: 4E800421  bctrl
	ctx.lr = 0x82EF69C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF69C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF69CC: 907F8F88  stw r3, -0x7078(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28792 as u32), ctx.r[3].u32 ) };
	// 82EF69D0: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF69D4: 90630004  stw r3, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EF69D8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF69DC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
            }
            0x82EF69E0 => {
    //   block [0x82EF69E0..0x82EF69F4)
	// 82EF69E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF69E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF69E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF69EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF69F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF69F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF69F8 size=144
    let mut pc: u32 = 0x82EF69F8;
    'dispatch: loop {
        match pc {
            0x82EF69F8 => {
    //   block [0x82EF69F8..0x82EF6A2C)
	// 82EF69F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF69FC: 4BDB2A11  bl 0x82ca940c
	ctx.lr = 0x82EF6A00;
	sub_82CA93D0(ctx, base);
	// 82EF6A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6A04: 3FC08336  lis r30, -0x7cca
	ctx.r[30].s64 = -2093613056;
	// 82EF6A08: 817E8F88  lwz r11, -0x7078(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6A0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A10: 419A0070  beq cr6, 0x82ef6a80
	if ctx.cr[6].eq {
	pc = 0x82EF6A80; continue 'dispatch;
	}
	// 82EF6A14: 48006E3D  bl 0x82efd850
	ctx.lr = 0x82EF6A18;
	sub_82EFD850(ctx, base);
	// 82EF6A18: 817E8F88  lwz r11, -0x7078(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6A1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6A20: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF6A24: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A28: 419A002C  beq cr6, 0x82ef6a54
	if ctx.cr[6].eq {
	pc = 0x82EF6A54; continue 'dispatch;
	}
	pc = 0x82EF6A2C; continue 'dispatch;
            }
            0x82EF6A2C => {
    //   block [0x82EF6A2C..0x82EF6A54)
	// 82EF6A2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6A34: 83A42000  lwz r29, 0x2000(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8192 as u32) ) } as u64;
	// 82EF6A38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6A3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6A40: 4E800421  bctrl
	ctx.lr = 0x82EF6A44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6A44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6A48: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A4C: 409AFFE0  bne cr6, 0x82ef6a2c
	if !ctx.cr[6].eq {
	pc = 0x82EF6A2C; continue 'dispatch;
	}
	// 82EF6A50: 817E8F88  lwz r11, -0x7078(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28792 as u32) ) } as u64;
            }
            0x82EF6A54 => {
    //   block [0x82EF6A54..0x82EF6A80)
	// 82EF6A54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF6A58: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EF6A5C: 93AB000C  stw r29, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82EF6A60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6A64: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EF6A68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6A6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6A70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6A74: 4E800421  bctrl
	ctx.lr = 0x82EF6A78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6A78: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EF6A7C: 917E8F88  stw r11, -0x7078(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-28792 as u32), ctx.r[11].u32 ) };
            }
            0x82EF6A80 => {
    //   block [0x82EF6A80..0x82EF6A88)
	// 82EF6A80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6A84: 4BDB29D8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6A88 size=84
    let mut pc: u32 = 0x82EF6A88;
    'dispatch: loop {
        match pc {
            0x82EF6A88 => {
    //   block [0x82EF6A88..0x82EF6ADC)
	// 82EF6A88: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6A8C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EF6A90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82EF6A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A98: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82EF6A9C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF6AA4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6AA8: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6AAC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6AB0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6AB4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF6AB8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF6ABC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF6AC0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6AC4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF6AC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6ACC: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EF6AD0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF6AD4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82EF6AD8: 4BFFFF20  b 0x82ef69f8
	sub_82EF69F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6AE0 size=76
    let mut pc: u32 = 0x82EF6AE0;
    'dispatch: loop {
        match pc {
            0x82EF6AE0 => {
    //   block [0x82EF6AE0..0x82EF6B08)
	// 82EF6AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6AE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6AEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6AF0: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF6AF4: 807F8F8C  lwz r3, -0x7074(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28788 as u32) ) } as u64;
	// 82EF6AF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6AFC: 409A000C  bne cr6, 0x82ef6b08
	if !ctx.cr[6].eq {
	pc = 0x82EF6B08; continue 'dispatch;
	}
	// 82EF6B00: 48006D41  bl 0x82efd840
	ctx.lr = 0x82EF6B04;
	sub_82EFD840(ctx, base);
	// 82EF6B04: 907F8F8C  stw r3, -0x7074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28788 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF6B08; continue 'dispatch;
            }
            0x82EF6B08 => {
    //   block [0x82EF6B08..0x82EF6B2C)
	// 82EF6B08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6B0C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF6B10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6B14: 4E800421  bctrl
	ctx.lr = 0x82EF6B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6B24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6B30 size=72
    let mut pc: u32 = 0x82EF6B30;
    'dispatch: loop {
        match pc {
            0x82EF6B30 => {
    //   block [0x82EF6B30..0x82EF6B78)
	// 82EF6B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6B48: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EF6B4C: 396BCF64  addi r11, r11, -0x309c
	ctx.r[11].s64 = ctx.r[11].s64 + -12444;
	// 82EF6B50: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82EF6B54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF6B58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6B5C: 4BDB2E55  bl 0x82ca99b0
	ctx.lr = 0x82EF6B60;
	sub_82CA99B0(ctx, base);
	// 82EF6B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6B70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6B78 size=68
    let mut pc: u32 = 0x82EF6B78;
    'dispatch: loop {
        match pc {
            0x82EF6B78 => {
    //   block [0x82EF6B78..0x82EF6BA4)
	// 82EF6B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6B88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6B8C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6B90: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6B94: 396BC2C0  addi r11, r11, -0x3d40
	ctx.r[11].s64 = ctx.r[11].s64 + -15680;
	// 82EF6B98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6B9C: 41820008  beq 0x82ef6ba4
	if ctx.cr[0].eq {
	pc = 0x82EF6BA4; continue 'dispatch;
	}
	// 82EF6BA0: 4B94EC11  bl 0x828457b0
	ctx.lr = 0x82EF6BA4;
	sub_828457B0(ctx, base);
	pc = 0x82EF6BA4; continue 'dispatch;
            }
            0x82EF6BA4 => {
    //   block [0x82EF6BA4..0x82EF6BBC)
	// 82EF6BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6BC0 size=100
    let mut pc: u32 = 0x82EF6BC0;
    'dispatch: loop {
        match pc {
            0x82EF6BC0 => {
    //   block [0x82EF6BC0..0x82EF6C24)
	// 82EF6BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6BC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6BCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6BD4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6BD8: 387F0550  addi r3, r31, 0x550
	ctx.r[3].s64 = ctx.r[31].s64 + 1360;
	// 82EF6BDC: 396BCF80  addi r11, r11, -0x3080
	ctx.r[11].s64 = ctx.r[11].s64 + -12416;
	// 82EF6BE0: 38800190  li r4, 0x190
	ctx.r[4].s64 = 400;
	// 82EF6BE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6BE8: 48002159  bl 0x82ef8d40
	ctx.lr = 0x82EF6BEC;
	sub_82EF8D40(ctx, base);
	// 82EF6BEC: 387F0510  addi r3, r31, 0x510
	ctx.r[3].s64 = ctx.r[31].s64 + 1296;
	// 82EF6BF0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82EF6BF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF6BF8: 4BDB2DB9  bl 0x82ca99b0
	ctx.lr = 0x82EF6BFC;
	sub_82CA99B0(ctx, base);
	// 82EF6BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6C00: 4BFFF589  bl 0x82ef6188
	ctx.lr = 0x82EF6C04;
	sub_82EF6188(ctx, base);
	// 82EF6C04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6C08: 917F0504  stw r11, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[11].u32 ) };
	// 82EF6C0C: 917F0508  stw r11, 0x508(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1288 as u32), ctx.r[11].u32 ) };
	// 82EF6C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6C1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6C28 size=8
    let mut pc: u32 = 0x82EF6C28;
    'dispatch: loop {
        match pc {
            0x82EF6C28 => {
    //   block [0x82EF6C28..0x82EF6C30)
	// 82EF6C28: 38630510  addi r3, r3, 0x510
	ctx.r[3].s64 = ctx.r[3].s64 + 1296;
	// 82EF6C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6C30 size=92
    let mut pc: u32 = 0x82EF6C30;
    'dispatch: loop {
        match pc {
            0x82EF6C30 => {
    //   block [0x82EF6C30..0x82EF6C70)
	// 82EF6C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6C38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF6C3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6C48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF6C4C: 387F0550  addi r3, r31, 0x550
	ctx.r[3].s64 = ctx.r[31].s64 + 1360;
	// 82EF6C50: 480043E9  bl 0x82efb038
	ctx.lr = 0x82EF6C54;
	sub_82EFB038(ctx, base);
	// 82EF6C54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6C58: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6C5C: 396BC2C0  addi r11, r11, -0x3d40
	ctx.r[11].s64 = ctx.r[11].s64 + -15680;
	// 82EF6C60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6C64: 4182000C  beq 0x82ef6c70
	if ctx.cr[0].eq {
	pc = 0x82EF6C70; continue 'dispatch;
	}
	// 82EF6C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6C6C: 4B94EB45  bl 0x828457b0
	ctx.lr = 0x82EF6C70;
	sub_828457B0(ctx, base);
	pc = 0x82EF6C70; continue 'dispatch;
            }
            0x82EF6C70 => {
    //   block [0x82EF6C70..0x82EF6C8C)
	// 82EF6C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6C74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6C80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF6C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6C90 size=108
    let mut pc: u32 = 0x82EF6C90;
    'dispatch: loop {
        match pc {
            0x82EF6C90 => {
    //   block [0x82EF6C90..0x82EF6CD4)
	// 82EF6C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6CA0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6CA4: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 82EF6CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6CAC: 483C2CB9  bl 0x832b9964
	ctx.lr = 0x82EF6CB0;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6CB0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6CB4: 816B8F88  lwz r11, -0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6CB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6CBC: 419A0024  beq cr6, 0x82ef6ce0
	if ctx.cr[6].eq {
	pc = 0x82EF6CE0; continue 'dispatch;
	}
	// 82EF6CC0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6CC4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF6CC8: 419A0018  beq cr6, 0x82ef6ce0
	if ctx.cr[6].eq {
	pc = 0x82EF6CE0; continue 'dispatch;
	}
	// 82EF6CCC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6CD0: 48000008  b 0x82ef6cd8
	pc = 0x82EF6CD8; continue 'dispatch;
            }
            0x82EF6CD4 => {
    //   block [0x82EF6CD4..0x82EF6CD8)
	// 82EF6CD4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x82EF6CD8; continue 'dispatch;
            }
            0x82EF6CD8 => {
    //   block [0x82EF6CD8..0x82EF6CE0)
	// 82EF6CD8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF6CDC: 409AFFF8  bne cr6, 0x82ef6cd4
	if !ctx.cr[6].eq {
	pc = 0x82EF6CD4; continue 'dispatch;
	}
	pc = 0x82EF6CE0; continue 'dispatch;
            }
            0x82EF6CE0 => {
    //   block [0x82EF6CE0..0x82EF6CFC)
	// 82EF6CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6CE4: 483C2C71  bl 0x832b9954
	ctx.lr = 0x82EF6CE8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6CF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6D00 size=152
    let mut pc: u32 = 0x82EF6D00;
    'dispatch: loop {
        match pc {
            0x82EF6D00 => {
    //   block [0x82EF6D00..0x82EF6D50)
	// 82EF6D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6D08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6D0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6D10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6D14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6D18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6D1C: 409A005C  bne cr6, 0x82ef6d78
	if !ctx.cr[6].eq {
	pc = 0x82EF6D78; continue 'dispatch;
	}
	// 82EF6D20: 48006B31  bl 0x82efd850
	ctx.lr = 0x82EF6D24;
	sub_82EFD850(ctx, base);
	// 82EF6D24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6D28: 38802004  li r4, 0x2004
	ctx.r[4].s64 = 8196;
	// 82EF6D2C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6D30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6D34: 4E800421  bctrl
	ctx.lr = 0x82EF6D38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6D38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF6D3C: 41820048  beq 0x82ef6d84
	if ctx.cr[0].eq {
	pc = 0x82EF6D84; continue 'dispatch;
	}
	// 82EF6D40: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF6D44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6D48: 91432000  stw r10, 0x2000(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8192 as u32), ctx.r[10].u32 ) };
	// 82EF6D4C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
            }
            0x82EF6D50 => {
    //   block [0x82EF6D50..0x82EF6D78)
	// 82EF6D50: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF6D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF6D58: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF6D5C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EF6D60: 2B0B2000  cmplwi cr6, r11, 0x2000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8192 as u32, &mut ctx.xer);
	// 82EF6D64: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF6D68: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6D6C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6D70: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF6D74: 4198FFDC  blt cr6, 0x82ef6d50
	if ctx.cr[6].lt {
	pc = 0x82EF6D50; continue 'dispatch;
	}
	pc = 0x82EF6D78; continue 'dispatch;
            }
            0x82EF6D78 => {
    //   block [0x82EF6D78..0x82EF6D84)
	// 82EF6D78: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6D7C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6D80: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF6D84; continue 'dispatch;
            }
            0x82EF6D84 => {
    //   block [0x82EF6D84..0x82EF6D98)
	// 82EF6D84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6D98 size=192
    let mut pc: u32 = 0x82EF6D98;
    'dispatch: loop {
        match pc {
            0x82EF6D98 => {
    //   block [0x82EF6D98..0x82EF6DDC)
	// 82EF6D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6D9C: 4BDB2661  bl 0x82ca93fc
	ctx.lr = 0x82EF6DA0;
	sub_82CA93D0(ctx, base);
	// 82EF6DA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6DA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF6DA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF6DAC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF6DB0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF6DB4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EF6DB8: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF6DBC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82EF6DC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF6DC4: 419A008C  beq cr6, 0x82ef6e50
	if ctx.cr[6].eq {
	pc = 0x82EF6E50; continue 'dispatch;
	}
	// 82EF6DC8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EF6DCC: 409A0010  bne cr6, 0x82ef6ddc
	if !ctx.cr[6].eq {
	pc = 0x82EF6DDC; continue 'dispatch;
	}
	// 82EF6DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6DD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6DD8: 48000078  b 0x82ef6e50
	pc = 0x82EF6E50; continue 'dispatch;
            }
            0x82EF6DDC => {
    //   block [0x82EF6DDC..0x82EF6E50)
	// 82EF6DDC: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82EF6DE0: 396000CC  li r11, 0xcc
	ctx.r[11].s64 = 204;
	// 82EF6DE4: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82EF6DE8: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82EF6DEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6DF0: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 82EF6DF4: 997F000E  stb r11, 0xe(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 82EF6DF8: 997F000F  stb r11, 0xf(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 82EF6DFC: 996A0004  stb r11, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82EF6E00: 996A0005  stb r11, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82EF6E04: 996A0006  stb r11, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[11].u8 ) };
	// 82EF6E08: 996A0007  stb r11, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[11].u8 ) };
	// 82EF6E0C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 82EF6E10: 4BFFFEF1  bl 0x82ef6d00
	ctx.lr = 0x82EF6E14;
	sub_82EF6D00(ctx, base);
	// 82EF6E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6E18: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82EF6E1C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6E20: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82EF6E24: 93830010  stw r28, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82EF6E28: 93430014  stw r26, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82EF6E2C: 93630018  stw r27, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 82EF6E30: 9323001C  stw r25, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[25].u32 ) };
	// 82EF6E34: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF6E38: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6E3C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6E40: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EF6E44: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6E48: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF6E4C: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF6E50; continue 'dispatch;
            }
            0x82EF6E50 => {
    //   block [0x82EF6E50..0x82EF6E58)
	// 82EF6E50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF6E54: 4BDB25F8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6E58 size=132
    let mut pc: u32 = 0x82EF6E58;
    'dispatch: loop {
        match pc {
            0x82EF6E58 => {
    //   block [0x82EF6E58..0x82EF6EA4)
	// 82EF6E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6E5C: 4BDB25AD  bl 0x82ca9408
	ctx.lr = 0x82EF6E60;
	sub_82CA93D0(ctx, base);
	// 82EF6E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6E64: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF6E68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6E6C: 419A0068  beq cr6, 0x82ef6ed4
	if ctx.cr[6].eq {
	pc = 0x82EF6ED4; continue 'dispatch;
	}
	// 82EF6E70: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6E74: 3BA4FFF0  addi r29, r4, -0x10
	ctx.r[29].s64 = ctx.r[4].s64 + -16;
	// 82EF6E78: 3BCB8F90  addi r30, r11, -0x7070
	ctx.r[30].s64 = ctx.r[11].s64 + -28784;
	// 82EF6E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6E80: 483C2AE5  bl 0x832b9964
	ctx.lr = 0x82EF6E84;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6E84: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6E88: 3BEB8F88  addi r31, r11, -0x7078
	ctx.r[31].s64 = ctx.r[11].s64 + -28792;
	// 82EF6E8C: 806B8F88  lwz r3, -0x7078(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6E94: 419A0010  beq cr6, 0x82ef6ea4
	if ctx.cr[6].eq {
	pc = 0x82EF6EA4; continue 'dispatch;
	}
	// 82EF6E98: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6E9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6EA0: 4BFFFBE9  bl 0x82ef6a88
	ctx.lr = 0x82EF6EA4;
	sub_82EF6A88(ctx, base);
	pc = 0x82EF6EA4; continue 'dispatch;
            }
            0x82EF6EA4 => {
    //   block [0x82EF6EA4..0x82EF6EC0)
	// 82EF6EA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6EA8: 483C2AAD  bl 0x832b9954
	ctx.lr = 0x82EF6EAC;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6EAC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6EB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6EB4: 409A000C  bne cr6, 0x82ef6ec0
	if !ctx.cr[6].eq {
	pc = 0x82EF6EC0; continue 'dispatch;
	}
	// 82EF6EB8: 48006989  bl 0x82efd840
	ctx.lr = 0x82EF6EBC;
	sub_82EFD840(ctx, base);
	// 82EF6EBC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF6EC0; continue 'dispatch;
            }
            0x82EF6EC0 => {
    //   block [0x82EF6EC0..0x82EF6ED4)
	// 82EF6EC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6EC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6EC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6ED0: 4E800421  bctrl
	ctx.lr = 0x82EF6ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF6ED4 => {
    //   block [0x82EF6ED4..0x82EF6EDC)
	// 82EF6ED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF6ED8: 4BDB2580  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6EE0 size=132
    let mut pc: u32 = 0x82EF6EE0;
    'dispatch: loop {
        match pc {
            0x82EF6EE0 => {
    //   block [0x82EF6EE0..0x82EF6F2C)
	// 82EF6EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6EE4: 4BDB2525  bl 0x82ca9408
	ctx.lr = 0x82EF6EE8;
	sub_82CA93D0(ctx, base);
	// 82EF6EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6EEC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF6EF0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6EF4: 419A0068  beq cr6, 0x82ef6f5c
	if ctx.cr[6].eq {
	pc = 0x82EF6F5C; continue 'dispatch;
	}
	// 82EF6EF8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6EFC: 3BA4FFF0  addi r29, r4, -0x10
	ctx.r[29].s64 = ctx.r[4].s64 + -16;
	// 82EF6F00: 3BCB8F90  addi r30, r11, -0x7070
	ctx.r[30].s64 = ctx.r[11].s64 + -28784;
	// 82EF6F04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6F08: 483C2A5D  bl 0x832b9964
	ctx.lr = 0x82EF6F0C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6F0C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6F10: 3BEB8F88  addi r31, r11, -0x7078
	ctx.r[31].s64 = ctx.r[11].s64 + -28792;
	// 82EF6F14: 806B8F88  lwz r3, -0x7078(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6F18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6F1C: 419A0010  beq cr6, 0x82ef6f2c
	if ctx.cr[6].eq {
	pc = 0x82EF6F2C; continue 'dispatch;
	}
	// 82EF6F20: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6F24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6F28: 4BFFFB61  bl 0x82ef6a88
	ctx.lr = 0x82EF6F2C;
	sub_82EF6A88(ctx, base);
	pc = 0x82EF6F2C; continue 'dispatch;
            }
            0x82EF6F2C => {
    //   block [0x82EF6F2C..0x82EF6F48)
	// 82EF6F2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6F30: 483C2A25  bl 0x832b9954
	ctx.lr = 0x82EF6F34;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6F34: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6F38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6F3C: 409A000C  bne cr6, 0x82ef6f48
	if !ctx.cr[6].eq {
	pc = 0x82EF6F48; continue 'dispatch;
	}
	// 82EF6F40: 48006901  bl 0x82efd840
	ctx.lr = 0x82EF6F44;
	sub_82EFD840(ctx, base);
	// 82EF6F44: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF6F48; continue 'dispatch;
            }
            0x82EF6F48 => {
    //   block [0x82EF6F48..0x82EF6F5C)
	// 82EF6F48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6F4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6F50: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6F54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6F58: 4E800421  bctrl
	ctx.lr = 0x82EF6F5C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF6F5C => {
    //   block [0x82EF6F5C..0x82EF6F64)
	// 82EF6F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF6F60: 4BDB24F8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6F68 size=4
    let mut pc: u32 = 0x82EF6F68;
    'dispatch: loop {
        match pc {
            0x82EF6F68 => {
    //   block [0x82EF6F68..0x82EF6F6C)
	// 82EF6F68: 4BFFFD28  b 0x82ef6c90
	sub_82EF6C90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6F70 size=124
    let mut pc: u32 = 0x82EF6F70;
    'dispatch: loop {
        match pc {
            0x82EF6F70 => {
    //   block [0x82EF6F70..0x82EF6FBC)
	// 82EF6F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6F74: 4BDB2485  bl 0x82ca93f8
	ctx.lr = 0x82EF6F78;
	sub_82CA93D0(ctx, base);
	// 82EF6F78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6F7C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6F80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF6F84: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 82EF6F88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF6F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6F90: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF6F94: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EF6F98: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EF6F9C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82EF6FA0: 483C29C5  bl 0x832b9964
	ctx.lr = 0x82EF6FA4;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6FA4: 3FA08336  lis r29, -0x7cca
	ctx.r[29].s64 = -2093613056;
	// 82EF6FA8: 807D8F88  lwz r3, -0x7078(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6FAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6FB0: 409A000C  bne cr6, 0x82ef6fbc
	if !ctx.cr[6].eq {
	pc = 0x82EF6FBC; continue 'dispatch;
	}
	// 82EF6FB4: 4BFFF9DD  bl 0x82ef6990
	ctx.lr = 0x82EF6FB8;
	sub_82EF6990(ctx, base);
	// 82EF6FB8: 807D8F88  lwz r3, -0x7078(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-28792 as u32) ) } as u64;
	pc = 0x82EF6FBC; continue 'dispatch;
            }
            0x82EF6FBC => {
    //   block [0x82EF6FBC..0x82EF6FEC)
	// 82EF6FBC: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82EF6FC0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82EF6FC4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82EF6FC8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EF6FCC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6FD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF6FD4: 4BFFFDC5  bl 0x82ef6d98
	ctx.lr = 0x82EF6FD8;
	sub_82EF6D98(ctx, base);
	// 82EF6FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6FDC: 483C2979  bl 0x832b9954
	ctx.lr = 0x82EF6FE0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6FE0: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82EF6FE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF6FE8: 4BDB2460  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6FF0 size=60
    let mut pc: u32 = 0x82EF6FF0;
    'dispatch: loop {
        match pc {
            0x82EF6FF0 => {
    //   block [0x82EF6FF0..0x82EF7018)
	// 82EF6FF0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EF6FF4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF6FF8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82EF6FFC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82EF7000: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82EF7004: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EF7008: 409A0010  bne cr6, 0x82ef7018
	if !ctx.cr[6].eq {
	pc = 0x82EF7018; continue 'dispatch;
	}
	// 82EF700C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF7010: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7014: 48000054  b 0x82ef7068
	sub_82EF702C(ctx, base);
	return;
            }
            0x82EF7018 => {
    //   block [0x82EF7018..0x82EF702C)
	// 82EF7018: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF701C: 409A0010  bne cr6, 0x82ef702c
	if !ctx.cr[6].eq {
		sub_82EF702C(ctx, base);
		return;
	}
	// 82EF7020: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82EF7024: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF7028: 4BFFFF48  b 0x82ef6f70
	sub_82EF6F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF702C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF702C size=68
    let mut pc: u32 = 0x82EF702C;
    'dispatch: loop {
        match pc {
            0x82EF702C => {
    //   block [0x82EF702C..0x82EF7070)
	// 82EF702C: 392B000C  addi r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 + 12;
	// 82EF7030: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF7034: 394000CC  li r10, 0xcc
	ctx.r[10].s64 = 204;
	// 82EF7038: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82EF703C: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82EF7040: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF7044: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82EF7048: 994B000D  stb r10, 0xd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82EF704C: 994B000E  stb r10, 0xe(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u8 ) };
	// 82EF7050: 994B000F  stb r10, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 82EF7054: 99490004  stb r10, 4(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82EF7058: 99490005  stb r10, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82EF705C: 99490006  stb r10, 6(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[10].u8 ) };
	// 82EF7060: 39490004  addi r10, r9, 4
	ctx.r[10].s64 = ctx.r[9].s64 + 4;
	// 82EF7064: 99090007  stb r8, 7(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(7 as u32), ctx.r[8].u8 ) };
	// 82EF7068: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EF706C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7070 size=116
    let mut pc: u32 = 0x82EF7070;
    'dispatch: loop {
        match pc {
            0x82EF7070 => {
    //   block [0x82EF7070..0x82EF70A8)
	// 82EF7070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7074: 4BDB238D  bl 0x82ca9400
	ctx.lr = 0x82EF7078;
	sub_82CA93D0(ctx, base);
	// 82EF7078: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF707C: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF7080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF7084: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF7088: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF708C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EF7090: 807F8F8C  lwz r3, -0x7074(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28788 as u32) ) } as u64;
	// 82EF7094: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF7098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF709C: 409A000C  bne cr6, 0x82ef70a8
	if !ctx.cr[6].eq {
	pc = 0x82EF70A8; continue 'dispatch;
	}
	// 82EF70A0: 480067A1  bl 0x82efd840
	ctx.lr = 0x82EF70A4;
	sub_82EFD840(ctx, base);
	// 82EF70A4: 907F8F8C  stw r3, -0x7074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28788 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF70A8; continue 'dispatch;
            }
            0x82EF70A8 => {
    //   block [0x82EF70A8..0x82EF70DC)
	// 82EF70A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF70AC: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 82EF70B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF70B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF70B8: 4E800421  bctrl
	ctx.lr = 0x82EF70BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF70BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF70C0: 4182001C  beq 0x82ef70dc
	if ctx.cr[0].eq {
	pc = 0x82EF70DC; continue 'dispatch;
	}
	// 82EF70C4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EF70C8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EF70CC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF70D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF70D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF70D8: 4BFFFE99  bl 0x82ef6f70
	ctx.lr = 0x82EF70DC;
	sub_82EF6F70(ctx, base);
            }
            0x82EF70DC => {
    //   block [0x82EF70DC..0x82EF70E4)
	// 82EF70DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF70E0: 4BDB2370  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF70E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF70E8 size=284
    let mut pc: u32 = 0x82EF70E8;
    'dispatch: loop {
        match pc {
            0x82EF70E8 => {
    //   block [0x82EF70E8..0x82EF7128)
	// 82EF70E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF70EC: 4BDB2311  bl 0x82ca93fc
	ctx.lr = 0x82EF70F0;
	sub_82CA93D0(ctx, base);
	// 82EF70F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF70F4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF70F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF70FC: 3BAB8F88  addi r29, r11, -0x7078
	ctx.r[29].s64 = ctx.r[11].s64 + -28792;
	// 82EF7100: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EF7104: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82EF7108: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EF710C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EF7110: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7114: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82EF7118: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF711C: 409A000C  bne cr6, 0x82ef7128
	if !ctx.cr[6].eq {
	pc = 0x82EF7128; continue 'dispatch;
	}
	// 82EF7120: 48006721  bl 0x82efd840
	ctx.lr = 0x82EF7124;
	sub_82EFD840(ctx, base);
	// 82EF7124: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF7128; continue 'dispatch;
            }
            0x82EF7128 => {
    //   block [0x82EF7128..0x82EF7160)
	// 82EF7128: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF712C: 409A0034  bne cr6, 0x82ef7160
	if !ctx.cr[6].eq {
	pc = 0x82EF7160; continue 'dispatch;
	}
	// 82EF7130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7134: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82EF7138: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF713C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7140: 4E800421  bctrl
	ctx.lr = 0x82EF7144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7144: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EF7148: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EF714C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF7150: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EF7154: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF7158: 4BFFFE19  bl 0x82ef6f70
	ctx.lr = 0x82EF715C;
	sub_82EF6F70(ctx, base);
	// 82EF715C: 480000A0  b 0x82ef71fc
	pc = 0x82EF71FC; continue 'dispatch;
            }
            0x82EF7160 => {
    //   block [0x82EF7160..0x82EF7194)
	// 82EF7160: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7164: 409A0058  bne cr6, 0x82ef71bc
	if !ctx.cr[6].eq {
	pc = 0x82EF71BC; continue 'dispatch;
	}
	// 82EF7168: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF716C: 3BDEFFF0  addi r30, r30, -0x10
	ctx.r[30].s64 = ctx.r[30].s64 + -16;
	// 82EF7170: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 82EF7174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7178: 483C27ED  bl 0x832b9964
	ctx.lr = 0x82EF717C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF717C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7180: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7184: 419A0010  beq cr6, 0x82ef7194
	if ctx.cr[6].eq {
	pc = 0x82EF7194; continue 'dispatch;
	}
	// 82EF7188: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EF718C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7190: 4BFFF8F9  bl 0x82ef6a88
	ctx.lr = 0x82EF7194;
	sub_82EF6A88(ctx, base);
	pc = 0x82EF7194; continue 'dispatch;
            }
            0x82EF7194 => {
    //   block [0x82EF7194..0x82EF71B4)
	// 82EF7194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7198: 483C27BD  bl 0x832b9954
	ctx.lr = 0x82EF719C;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF719C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF71A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF71A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF71A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF71AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF71B0: 4E800421  bctrl
	ctx.lr = 0x82EF71B4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF71B4 => {
    //   block [0x82EF71B4..0x82EF71BC)
	// 82EF71B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF71B8: 48000044  b 0x82ef71fc
	pc = 0x82EF71FC; continue 'dispatch;
            }
            0x82EF71BC => {
    //   block [0x82EF71BC..0x82EF71FC)
	// 82EF71BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF71C0: 389EFFF0  addi r4, r30, -0x10
	ctx.r[4].s64 = ctx.r[30].s64 + -16;
	// 82EF71C4: 83DEFFF0  lwz r30, -0x10(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EF71C8: 38BF0014  addi r5, r31, 0x14
	ctx.r[5].s64 = ctx.r[31].s64 + 20;
	// 82EF71CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF71D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF71D4: 4E800421  bctrl
	ctx.lr = 0x82EF71D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF71D8: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EF71DC: 4182FFD8  beq 0x82ef71b4
	if ctx.cr[0].eq {
	pc = 0x82EF71B4; continue 'dispatch;
	}
	// 82EF71E0: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82EF71E4: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82EF71E8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EF71EC: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82EF71F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF71F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF71F8: 4BFFFDF9  bl 0x82ef6ff0
	ctx.lr = 0x82EF71FC;
	sub_82EF6FF0(ctx, base);
            }
            0x82EF71FC => {
    //   block [0x82EF71FC..0x82EF7204)
	// 82EF71FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF7200: 4BDB224C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7208 size=132
    let mut pc: u32 = 0x82EF7208;
    'dispatch: loop {
        match pc {
            0x82EF7208 => {
    //   block [0x82EF7208..0x82EF7248)
	// 82EF7208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF720C: 4BDB21ED  bl 0x82ca93f8
	ctx.lr = 0x82EF7210;
	sub_82CA93D0(ctx, base);
	// 82EF7210: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7214: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF7218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF721C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF7220: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF7224: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EF7228: 807F8F8C  lwz r3, -0x7074(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28788 as u32) ) } as u64;
	// 82EF722C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF7230: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82EF7234: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 82EF7238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF723C: 409A000C  bne cr6, 0x82ef7248
	if !ctx.cr[6].eq {
	pc = 0x82EF7248; continue 'dispatch;
	}
	// 82EF7240: 48006601  bl 0x82efd840
	ctx.lr = 0x82EF7244;
	sub_82EFD840(ctx, base);
	// 82EF7244: 907F8F8C  stw r3, -0x7074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28788 as u32), ctx.r[3].u32 ) };
	pc = 0x82EF7248; continue 'dispatch;
            }
            0x82EF7248 => {
    //   block [0x82EF7248..0x82EF7284)
	// 82EF7248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF724C: 38DC0010  addi r6, r28, 0x10
	ctx.r[6].s64 = ctx.r[28].s64 + 16;
	// 82EF7250: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF7254: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82EF7258: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF725C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7260: 4E800421  bctrl
	ctx.lr = 0x82EF7264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7264: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF7268: 4182001C  beq 0x82ef7284
	if ctx.cr[0].eq {
	pc = 0x82EF7284; continue 'dispatch;
	}
	// 82EF726C: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82EF7270: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EF7274: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EF7278: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EF727C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7280: 4BFFFCF1  bl 0x82ef6f70
	ctx.lr = 0x82EF7284;
	sub_82EF6F70(ctx, base);
            }
            0x82EF7284 => {
    //   block [0x82EF7284..0x82EF728C)
	// 82EF7284: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF7288: 4BDB21C0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7290 size=64
    let mut pc: u32 = 0x82EF7290;
    'dispatch: loop {
        match pc {
            0x82EF7290 => {
    //   block [0x82EF7290..0x82EF72C0)
	// 82EF7290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF729C: 480065C5  bl 0x82efd860
	ctx.lr = 0x82EF72A0;
	sub_82EFD860(ctx, base);
	// 82EF72A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF72A4: 4182001C  beq 0x82ef72c0
	if ctx.cr[0].eq {
	pc = 0x82EF72C0; continue 'dispatch;
	}
	// 82EF72A8: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EF72AC: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EF72B0: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EF72B4: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EF72B8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF72BC: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x82EF72C0; continue 'dispatch;
            }
            0x82EF72C0 => {
    //   block [0x82EF72C0..0x82EF72D0)
	// 82EF72C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF72C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF72C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF72CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF72D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF72D0 size=76
    let mut pc: u32 = 0x82EF72D0;
    'dispatch: loop {
        match pc {
            0x82EF72D0 => {
    //   block [0x82EF72D0..0x82EF731C)
	// 82EF72D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF72D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF72D8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EF72DC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EF72E0: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EF72E4: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EF72E8: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EF72EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF72F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF72F4: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 82EF72F8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EF72FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF7300: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7304: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF7308: 4BDB5431  bl 0x82cac738
	ctx.lr = 0x82EF730C;
	sub_82CAC738(ctx, base);
	// 82EF730C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF7310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7320 size=104
    let mut pc: u32 = 0x82EF7320;
    'dispatch: loop {
        match pc {
            0x82EF7320 => {
    //   block [0x82EF7320..0x82EF735C)
	// 82EF7320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF732C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7334: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF7338: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF733C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7340: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7344: 419A0018  beq cr6, 0x82ef735c
	if ctx.cr[6].eq {
	pc = 0x82EF735C; continue 'dispatch;
	}
	// 82EF7348: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF734C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7350: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7358: 4E800421  bctrl
	ctx.lr = 0x82EF735C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF735C => {
    //   block [0x82EF735C..0x82EF7388)
	// 82EF735C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7364: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7368: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF736C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7370: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF737C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF7380: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7388 size=240
    let mut pc: u32 = 0x82EF7388;
    'dispatch: loop {
        match pc {
            0x82EF7388 => {
    //   block [0x82EF7388..0x82EF73FC)
	// 82EF7388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7390: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7394: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF739C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF73A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF73A4: 409A0084  bne cr6, 0x82ef7428
	if !ctx.cr[6].eq {
	pc = 0x82EF7428; continue 'dispatch;
	}
	// 82EF73A8: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF73AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF73B0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF73B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF73BC: 4E800421  bctrl
	ctx.lr = 0x82EF73C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF73C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF73C8: 419A0034  beq cr6, 0x82ef73fc
	if ctx.cr[6].eq {
	pc = 0x82EF73FC; continue 'dispatch;
	}
	// 82EF73CC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF73D4: 419A0028  beq cr6, 0x82ef73fc
	if ctx.cr[6].eq {
	pc = 0x82EF73FC; continue 'dispatch;
	}
	// 82EF73D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF73E0: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF73E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF73E8: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF73EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF73F0: 4E800421  bctrl
	ctx.lr = 0x82EF73F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF73F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF73F8: 48000008  b 0x82ef7400
	pc = 0x82EF7400; continue 'dispatch;
            }
            0x82EF73FC => {
    //   block [0x82EF73FC..0x82EF7400)
	// 82EF73FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82EF7400; continue 'dispatch;
            }
            0x82EF7400 => {
    //   block [0x82EF7400..0x82EF7420)
	// 82EF7400: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF7404: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7408: 419A0018  beq cr6, 0x82ef7420
	if ctx.cr[6].eq {
	pc = 0x82EF7420; continue 'dispatch;
	}
	// 82EF740C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7410: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF7414: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF741C: 4E800421  bctrl
	ctx.lr = 0x82EF7420;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF7420 => {
    //   block [0x82EF7420..0x82EF7428)
	// 82EF7420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7424: 48000040  b 0x82ef7464
	pc = 0x82EF7464; continue 'dispatch;
            }
            0x82EF7428 => {
    //   block [0x82EF7428..0x82EF7460)
	// 82EF7428: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF742C: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF7430: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7434: 419A002C  beq cr6, 0x82ef7460
	if ctx.cr[6].eq {
	pc = 0x82EF7460; continue 'dispatch;
	}
	// 82EF7438: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF743C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF7440: 419A0020  beq cr6, 0x82ef7460
	if ctx.cr[6].eq {
	pc = 0x82EF7460; continue 'dispatch;
	}
	// 82EF7444: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7448: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF744C: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF7450: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7458: 4E800421  bctrl
	ctx.lr = 0x82EF745C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF745C: 48000008  b 0x82ef7464
	pc = 0x82EF7464; continue 'dispatch;
            }
            0x82EF7460 => {
    //   block [0x82EF7460..0x82EF7464)
	// 82EF7460: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF7464; continue 'dispatch;
            }
            0x82EF7464 => {
    //   block [0x82EF7464..0x82EF7478)
	// 82EF7464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF746C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7470: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7478 size=220
    let mut pc: u32 = 0x82EF7478;
    'dispatch: loop {
        match pc {
            0x82EF7478 => {
    //   block [0x82EF7478..0x82EF74BC)
	// 82EF7478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF747C: 4BDB1F91  bl 0x82ca940c
	ctx.lr = 0x82EF7480;
	sub_82CA93D0(ctx, base);
	// 82EF7480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7484: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7488: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF748C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF7490: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF7494: 409A0028  bne cr6, 0x82ef74bc
	if !ctx.cr[6].eq {
	pc = 0x82EF74BC; continue 'dispatch;
	}
	// 82EF7498: 80840004  lwz r4, 4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF749C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82EF74A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF74A4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF74A8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF74AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF74B0: 4E800421  bctrl
	ctx.lr = 0x82EF74B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF74B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF74B8: 4800000C  b 0x82ef74c4
	pc = 0x82EF74C4; continue 'dispatch;
            }
            0x82EF74BC => {
    //   block [0x82EF74BC..0x82EF74C4)
	// 82EF74BC: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF74C0: 388B001C  addi r4, r11, 0x1c
	ctx.r[4].s64 = ctx.r[11].s64 + 28;
	pc = 0x82EF74C4; continue 'dispatch;
            }
            0x82EF74C4 => {
    //   block [0x82EF74C4..0x82EF74F4)
	// 82EF74C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF74C8: 4BFFFE59  bl 0x82ef7320
	ctx.lr = 0x82EF74CC;
	sub_82EF7320(ctx, base);
	// 82EF74CC: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF74D0: 41820024  beq 0x82ef74f4
	if ctx.cr[0].eq {
	pc = 0x82EF74F4; continue 'dispatch;
	}
	// 82EF74D4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF74D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF74DC: 419A0018  beq cr6, 0x82ef74f4
	if ctx.cr[6].eq {
	pc = 0x82EF74F4; continue 'dispatch;
	}
	// 82EF74E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF74E4: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF74E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF74EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF74F0: 4E800421  bctrl
	ctx.lr = 0x82EF74F4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF74F4 => {
    //   block [0x82EF74F4..0x82EF7524)
	// 82EF74F4: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF74F8: 83A1005C  lwz r29, 0x5c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF74FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7500: 419A0024  beq cr6, 0x82ef7524
	if ctx.cr[6].eq {
	pc = 0x82EF7524; continue 'dispatch;
	}
	// 82EF7504: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7508: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF750C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7510: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7518: 4E800421  bctrl
	ctx.lr = 0x82EF751C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF751C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF7520: 48000008  b 0x82ef7528
	pc = 0x82EF7528; continue 'dispatch;
            }
            0x82EF7524 => {
    //   block [0x82EF7524..0x82EF7528)
	// 82EF7524: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82EF7528; continue 'dispatch;
            }
            0x82EF7528 => {
    //   block [0x82EF7528..0x82EF7548)
	// 82EF7528: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF752C: 419A001C  beq cr6, 0x82ef7548
	if ctx.cr[6].eq {
	pc = 0x82EF7548; continue 'dispatch;
	}
	// 82EF7530: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7534: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF7538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF753C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7544: 4E800421  bctrl
	ctx.lr = 0x82EF7548;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF7548 => {
    //   block [0x82EF7548..0x82EF7554)
	// 82EF7548: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF754C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF7550: 4BDB1F0C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7558 size=12
    let mut pc: u32 = 0x82EF7558;
    'dispatch: loop {
        match pc {
            0x82EF7558 => {
    //   block [0x82EF7558..0x82EF7564)
	// 82EF7558: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF755C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EF7560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7568 size=152
    let mut pc: u32 = 0x82EF7568;
    'dispatch: loop {
        match pc {
            0x82EF7568 => {
    //   block [0x82EF7568..0x82EF75AC)
	// 82EF7568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF756C: 4BDB1E99  bl 0x82ca9404
	ctx.lr = 0x82EF7570;
	sub_82CA93D0(ctx, base);
	// 82EF7570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7574: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF7578: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF757C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EF7580: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7584: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82EF7588: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF758C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF7590: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF7594: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7598: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF759C: 419A0058  beq cr6, 0x82ef75f4
	if ctx.cr[6].eq {
	pc = 0x82EF75F4; continue 'dispatch;
	}
	// 82EF75A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF75A4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF75A8: 409A004C  bne cr6, 0x82ef75f4
	if !ctx.cr[6].eq {
	pc = 0x82EF75F4; continue 'dispatch;
	}
	pc = 0x82EF75AC; continue 'dispatch;
            }
            0x82EF75AC => {
    //   block [0x82EF75AC..0x82EF75CC)
	// 82EF75AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF75B0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF75B4: 409A0018  bne cr6, 0x82ef75cc
	if !ctx.cr[6].eq {
	pc = 0x82EF75CC; continue 'dispatch;
	}
	// 82EF75B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF75BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EF75C0: 4BFFFDC9  bl 0x82ef7388
	ctx.lr = 0x82EF75C4;
	sub_82EF7388(ctx, base);
	// 82EF75C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF75C8: 40820024  bne 0x82ef75ec
	if !ctx.cr[0].eq {
	pc = 0x82EF75EC; continue 'dispatch;
	}
	pc = 0x82EF75CC; continue 'dispatch;
            }
            0x82EF75CC => {
    //   block [0x82EF75CC..0x82EF75EC)
	// 82EF75CC: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF75D0: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82EF75D4: 419A0020  beq cr6, 0x82ef75f4
	if ctx.cr[6].eq {
	pc = 0x82EF75F4; continue 'dispatch;
	}
	// 82EF75D8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF75DC: 578B2036  slwi r11, r28, 4
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF75E0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF75E4: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF75E8: 4BFFFFC4  b 0x82ef75ac
	pc = 0x82EF75AC; continue 'dispatch;
            }
            0x82EF75EC => {
    //   block [0x82EF75EC..0x82EF75F4)
	// 82EF75EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF75F0: 48000008  b 0x82ef75f8
	pc = 0x82EF75F8; continue 'dispatch;
            }
            0x82EF75F4 => {
    //   block [0x82EF75F4..0x82EF75F8)
	// 82EF75F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x82EF75F8; continue 'dispatch;
            }
            0x82EF75F8 => {
    //   block [0x82EF75F8..0x82EF7600)
	// 82EF75F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF75FC: 4BDB1E58  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7600 size=140
    let mut pc: u32 = 0x82EF7600;
    'dispatch: loop {
        match pc {
            0x82EF7600 => {
    //   block [0x82EF7600..0x82EF763C)
	// 82EF7600: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 82EF7604: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7608: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82EF760C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7610: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF7614: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7618: 2F0AFFFE  cmpwi cr6, r10, -2
	ctx.cr[6].compare_i32(ctx.r[10].s32, -2, &mut ctx.xer);
	// 82EF761C: 419A0068  beq cr6, 0x82ef7684
	if ctx.cr[6].eq {
	pc = 0x82EF7684; continue 'dispatch;
	}
	// 82EF7620: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7624: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7628: 5547D1BE  srwi r7, r10, 6
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(6);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF762C: 7CEA5278  xor r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 ^ ctx.r[10].u64;
	// 82EF7630: 7D4A4038  and r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[8].u64;
	// 82EF7634: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF7638: 409A004C  bne cr6, 0x82ef7684
	if !ctx.cr[6].eq {
	pc = 0x82EF7684; continue 'dispatch;
	}
	pc = 0x82EF763C; continue 'dispatch;
            }
            0x82EF763C => {
    //   block [0x82EF763C..0x82EF7668)
	// 82EF763C: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7640: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF7644: 54E6D1BE  srwi r6, r7, 6
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(6);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EF7648: 7CC73A78  xor r7, r6, r7
	ctx.r[7].u64 = ctx.r[6].u64 ^ ctx.r[7].u64;
	// 82EF764C: 7CE74038  and r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[8].u64;
	// 82EF7650: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF7654: 409A0014  bne cr6, 0x82ef7668
	if !ctx.cr[6].eq {
	pc = 0x82EF7668; continue 'dispatch;
	}
	// 82EF7658: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF765C: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7660: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF7664: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82EF7668; continue 'dispatch;
            }
            0x82EF7668 => {
    //   block [0x82EF7668..0x82EF7684)
	// 82EF7668: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF766C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EF7670: 419A0014  beq cr6, 0x82ef7684
	if ctx.cr[6].eq {
	pc = 0x82EF7684; continue 'dispatch;
	}
	// 82EF7674: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF7678: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF767C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF7680: 4BFFFFBC  b 0x82ef763c
	pc = 0x82EF763C; continue 'dispatch;
            }
            0x82EF7684 => {
    //   block [0x82EF7684..0x82EF768C)
	// 82EF7684: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EF7688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7690 size=192
    let mut pc: u32 = 0x82EF7690;
    'dispatch: loop {
        match pc {
            0x82EF7690 => {
    //   block [0x82EF7690..0x82EF770C)
	// 82EF7690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7694: 4BDB1D71  bl 0x82ca9404
	ctx.lr = 0x82EF7698;
	sub_82CA93D0(ctx, base);
	// 82EF7698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF769C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF76A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF76A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF76A8: 480062A1  bl 0x82efd948
	ctx.lr = 0x82EF76AC;
	sub_82EFD948(ctx, base);
	// 82EF76AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF76B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF76B4: 396BCF9C  addi r11, r11, -0x3064
	ctx.r[11].s64 = ctx.r[11].s64 + -12388;
	// 82EF76B8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82EF76BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF76C0: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82EF76C4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EF76C8: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82EF76CC: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EF76D0: 48006899  bl 0x82efdf68
	ctx.lr = 0x82EF76D4;
	sub_82EFDF68(ctx, base);
	// 82EF76D4: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF76D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF76DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF76E0: 480024A1  bl 0x82ef9b80
	ctx.lr = 0x82EF76E4;
	sub_82EF9B80(ctx, base);
	// 82EF76E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF76E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF76EC: 4800636D  bl 0x82efda58
	ctx.lr = 0x82EF76F0;
	sub_82EFDA58(ctx, base);
	// 82EF76F0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF76F4: 419A0018  beq cr6, 0x82ef770c
	if ctx.cr[6].eq {
	pc = 0x82EF770C; continue 'dispatch;
	}
	// 82EF76F8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF76FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF7700: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7708: 4E800421  bctrl
	ctx.lr = 0x82EF770C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF770C => {
    //   block [0x82EF770C..0x82EF772C)
	// 82EF770C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7714: 419A0018  beq cr6, 0x82ef772c
	if ctx.cr[6].eq {
	pc = 0x82EF772C; continue 'dispatch;
	}
	// 82EF7718: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF771C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF7720: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7728: 4E800421  bctrl
	ctx.lr = 0x82EF772C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF772C => {
    //   block [0x82EF772C..0x82EF7750)
	// 82EF772C: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82EF7730: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF7734: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EF7738: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF773C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EF7740: 4804DCE1  bl 0x82f45420
	ctx.lr = 0x82EF7744;
	sub_82F45420(ctx, base);
	// 82EF7744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF774C: 4BDB1D08  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7750 size=112
    let mut pc: u32 = 0x82EF7750;
    'dispatch: loop {
        match pc {
            0x82EF7750 => {
    //   block [0x82EF7750..0x82EF7780)
	// 82EF7750: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7754: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7758: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF775C: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7760: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF7764: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	// 82EF7768: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF776C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7770: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7774: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7778: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF777C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	pc = 0x82EF7780; continue 'dispatch;
            }
            0x82EF7780 => {
    //   block [0x82EF7780..0x82EF77C0)
	// 82EF7780: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7784: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7788: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EF778C: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF7790: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EF7794: 2F09FFFE  cmpwi cr6, r9, -2
	ctx.cr[6].compare_i32(ctx.r[9].s32, -2, &mut ctx.xer);
	// 82EF7798: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82EF779C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF77A0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF77A4: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF77A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF77B4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF77B8: 4099FFC8  ble cr6, 0x82ef7780
	if !ctx.cr[6].gt {
	pc = 0x82EF7780; continue 'dispatch;
	}
	// 82EF77BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF77C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF77C0 size=332
    let mut pc: u32 = 0x82EF77C0;
    'dispatch: loop {
        match pc {
            0x82EF77C0 => {
    //   block [0x82EF77C0..0x82EF7804)
	// 82EF77C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF77C4: 4BDB1C3D  bl 0x82ca9400
	ctx.lr = 0x82EF77C8;
	sub_82CA93D0(ctx, base);
	// 82EF77C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF77CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF77D0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EF77D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF77DC: 419A0128  beq cr6, 0x82ef7904
	if ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF77E0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF77E8: 419A001C  beq cr6, 0x82ef7804
	if ctx.cr[6].eq {
	pc = 0x82EF7804; continue 'dispatch;
	}
	// 82EF77EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77F0: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF77F4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF77F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF77FC: 4E800421  bctrl
	ctx.lr = 0x82EF7800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7800: 48000008  b 0x82ef7808
	pc = 0x82EF7808; continue 'dispatch;
            }
            0x82EF7804 => {
    //   block [0x82EF7804..0x82EF7808)
	// 82EF7804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF7808; continue 'dispatch;
            }
            0x82EF7808 => {
    //   block [0x82EF7808..0x82EF7840)
	// 82EF7808: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF780C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7810: 7D5E1838  and r30, r10, r3
	ctx.r[30].u64 = ctx.r[10].u64 & ctx.r[3].u64;
	// 82EF7814: 57CA2036  slwi r10, r30, 4
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7818: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF781C: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF7820: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7824: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF7828: 419A00DC  beq cr6, 0x82ef7904
	if ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF782C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7830: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF7834: 409A00D0  bne cr6, 0x82ef7904
	if !ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF7838: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82EF783C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	pc = 0x82EF7840; continue 'dispatch;
            }
            0x82EF7840 => {
    //   block [0x82EF7840..0x82EF7860)
	// 82EF7840: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7844: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82EF7848: 409A0018  bne cr6, 0x82ef7860
	if !ctx.cr[6].eq {
	pc = 0x82EF7860; continue 'dispatch;
	}
	// 82EF784C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF7850: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EF7854: 4BFFFB35  bl 0x82ef7388
	ctx.lr = 0x82EF7858;
	sub_82EF7388(ctx, base);
	// 82EF7858: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF785C: 40820028  bne 0x82ef7884
	if !ctx.cr[0].eq {
	pc = 0x82EF7884; continue 'dispatch;
	}
	pc = 0x82EF7860; continue 'dispatch;
            }
            0x82EF7860 => {
    //   block [0x82EF7860..0x82EF7884)
	// 82EF7860: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82EF7864: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7868: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EF786C: 419A0098  beq cr6, 0x82ef7904
	if ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF7870: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7874: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7878: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF787C: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF7880: 4BFFFFC0  b 0x82ef7840
	pc = 0x82EF7840; continue 'dispatch;
            }
            0x82EF7884 => {
    //   block [0x82EF7884..0x82EF78DC)
	// 82EF7884: 3920FFFE  li r9, -2
	ctx.r[9].s64 = -2;
	// 82EF7888: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EF788C: 409A0050  bne cr6, 0x82ef78dc
	if !ctx.cr[6].eq {
	pc = 0x82EF78DC; continue 'dispatch;
	}
	// 82EF7890: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7894: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82EF7898: 419A0058  beq cr6, 0x82ef78f0
	if ctx.cr[6].eq {
	pc = 0x82EF78F0; continue 'dispatch;
	}
	// 82EF789C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78A0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF78A4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF78A8: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 82EF78AC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF78B0: 394B0008  addi r10, r11, 8
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	// 82EF78B4: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF78B8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF78BC: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF78C0: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EF78C4: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82EF78C8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF78CC: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF78D0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF78D4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF78D8: 48000018  b 0x82ef78f0
	pc = 0x82EF78F0; continue 'dispatch;
            }
            0x82EF78DC => {
    //   block [0x82EF78DC..0x82EF78F0)
	// 82EF78DC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78E0: 578B2036  slwi r11, r28, 4
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF78E4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF78EC: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	pc = 0x82EF78F0; continue 'dispatch;
            }
            0x82EF78F0 => {
    //   block [0x82EF78F0..0x82EF7904)
	// 82EF78F0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF78F4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78F8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF7900: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF7904; continue 'dispatch;
            }
            0x82EF7904 => {
    //   block [0x82EF7904..0x82EF790C)
	// 82EF7904: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF7908: 4BDB1B48  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7910 size=140
    let mut pc: u32 = 0x82EF7910;
    'dispatch: loop {
        match pc {
            0x82EF7910 => {
    //   block [0x82EF7910..0x82EF7940)
	// 82EF7910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF791C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7924: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF7928: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF792C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF7934: 409A000C  bne cr6, 0x82ef7940
	if !ctx.cr[6].eq {
	pc = 0x82EF7940; continue 'dispatch;
	}
	// 82EF7938: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EF793C: 48000048  b 0x82ef7984
	pc = 0x82EF7984; continue 'dispatch;
            }
            0x82EF7940 => {
    //   block [0x82EF7940..0x82EF7968)
	// 82EF7940: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7944: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7948: 419A0020  beq cr6, 0x82ef7968
	if ctx.cr[6].eq {
	pc = 0x82EF7968; continue 'dispatch;
	}
	// 82EF794C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7950: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7954: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF795C: 4E800421  bctrl
	ctx.lr = 0x82EF7960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7960: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EF7964: 48000008  b 0x82ef796c
	pc = 0x82EF796C; continue 'dispatch;
            }
            0x82EF7968 => {
    //   block [0x82EF7968..0x82EF796C)
	// 82EF7968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EF796C; continue 'dispatch;
            }
            0x82EF796C => {
    //   block [0x82EF796C..0x82EF7984)
	// 82EF796C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7970: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF7974: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7978: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF797C: 7D455838  and r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82EF7980: 4BFFFBE9  bl 0x82ef7568
	ctx.lr = 0x82EF7984;
	sub_82EF7568(ctx, base);
	pc = 0x82EF7984; continue 'dispatch;
            }
            0x82EF7984 => {
    //   block [0x82EF7984..0x82EF799C)
	// 82EF7984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF798C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF7994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF79A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF79A0 size=392
    let mut pc: u32 = 0x82EF79A0;
    'dispatch: loop {
        match pc {
            0x82EF79A0 => {
    //   block [0x82EF79A0..0x82EF79CC)
	// 82EF79A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF79A4: 4BDB1A69  bl 0x82ca940c
	ctx.lr = 0x82EF79A8;
	sub_82CA93D0(ctx, base);
	// 82EF79A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF79AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF79B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF79B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF79B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF79BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF79C0: 409A000C  bne cr6, 0x82ef79cc
	if !ctx.cr[6].eq {
	pc = 0x82EF79CC; continue 'dispatch;
	}
	// 82EF79C4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EF79C8: 4800002C  b 0x82ef79f4
	pc = 0x82EF79F4; continue 'dispatch;
            }
            0x82EF79CC => {
    //   block [0x82EF79CC..0x82EF79F4)
	// 82EF79CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF79D0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF79D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF79D8: 1D290005  mulli r9, r9, 5
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * 5 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82EF79DC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF79E0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF79E4: 40990018  ble cr6, 0x82ef79fc
	if !ctx.cr[6].gt {
	pc = 0x82EF79FC; continue 'dispatch;
	}
	// 82EF79E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF79EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF79F0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	pc = 0x82EF79F4; continue 'dispatch;
            }
            0x82EF79F4 => {
    //   block [0x82EF79F4..0x82EF79FC)
	// 82EF79F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF79F8: 48000769  bl 0x82ef8160
	ctx.lr = 0x82EF79FC;
	sub_82EF8160(ctx, base);
	pc = 0x82EF79FC; continue 'dispatch;
            }
            0x82EF79FC => {
    //   block [0x82EF79FC..0x82EF7A48)
	// 82EF79FC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A00: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A04: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A08: 7D27F038  and r7, r9, r30
	ctx.r[7].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 82EF7A0C: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EF7A10: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7A14: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF7A18: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A1C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF7A20: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF7A24: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A28: 2F05FFFE  cmpwi cr6, r5, -2
	ctx.cr[6].compare_i32(ctx.r[5].s32, -2, &mut ctx.xer);
	// 82EF7A2C: 409A001C  bne cr6, 0x82ef7a48
	if !ctx.cr[6].eq {
	pc = 0x82EF7A48; continue 'dispatch;
	}
	// 82EF7A30: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EF7A34: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7A38: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A3C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7A40: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A44: 48000074  b 0x82ef7ab8
	pc = 0x82EF7AB8; continue 'dispatch;
            }
            0x82EF7A48 => {
    //   block [0x82EF7A48..0x82EF7A50)
	// 82EF7A48: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A4C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	pc = 0x82EF7A50; continue 'dispatch;
            }
            0x82EF7A50 => {
    //   block [0x82EF7A50..0x82EF7AA8)
	// 82EF7A50: 39060001  addi r8, r6, 1
	ctx.r[8].s64 = ctx.r[6].s64 + 1;
	// 82EF7A54: 7D064838  and r6, r8, r9
	ctx.r[6].u64 = ctx.r[8].u64 & ctx.r[9].u64;
	// 82EF7A58: 54C82036  slwi r8, r6, 4
	ctx.r[8].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF7A5C: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EF7A60: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7A64: 2F08FFFE  cmpwi cr6, r8, -2
	ctx.cr[6].compare_i32(ctx.r[8].s32, -2, &mut ctx.xer);
	// 82EF7A68: 409AFFE8  bne cr6, 0x82ef7a50
	if !ctx.cr[6].eq {
	pc = 0x82EF7A50; continue 'dispatch;
	}
	// 82EF7A6C: 54C92036  slwi r9, r6, 4
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF7A70: 7D095214  add r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF7A74: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A78: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82EF7A7C: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF7A80: 409A0044  bne cr6, 0x82ef7ac4
	if !ctx.cr[6].eq {
	pc = 0x82EF7AC4; continue 'dispatch;
	}
	// 82EF7A84: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF7A88: 419A0020  beq cr6, 0x82ef7aa8
	if ctx.cr[6].eq {
	pc = 0x82EF7AA8; continue 'dispatch;
	}
	// 82EF7A8C: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EF7A90: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A94: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF7A98: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7A9C: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7AA0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF7AA4: 9148000C  stw r10, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	pc = 0x82EF7AA8; continue 'dispatch;
            }
            0x82EF7AA8 => {
    //   block [0x82EF7AA8..0x82EF7AB8)
	// 82EF7AA8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7AAC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7AB0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7AB4: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	pc = 0x82EF7AB8; continue 'dispatch;
            }
            0x82EF7AB8 => {
    //   block [0x82EF7AB8..0x82EF7AC0)
	// 82EF7AB8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EF7ABC: 48000060  b 0x82ef7b1c
	pc = 0x82EF7B1C; continue 'dispatch;
            }
            0x82EF7AC0 => {
    //   block [0x82EF7AC0..0x82EF7AC4)
	// 82EF7AC0: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82EF7AC4; continue 'dispatch;
            }
            0x82EF7AC4 => {
    //   block [0x82EF7AC4..0x82EF7B00)
	// 82EF7AC4: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF7AC8: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF7ACC: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82EF7AD0: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7AD4: 7F043800  cmpw cr6, r4, r7
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EF7AD8: 409AFFE8  bne cr6, 0x82ef7ac0
	if !ctx.cr[6].eq {
	pc = 0x82EF7AC0; continue 'dispatch;
	}
	// 82EF7ADC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF7AE0: 419A0020  beq cr6, 0x82ef7b00
	if ctx.cr[6].eq {
	pc = 0x82EF7B00; continue 'dispatch;
	}
	// 82EF7AE4: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EF7AE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7AEC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF7AF0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7AF4: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7AF8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF7AFC: 9148000C  stw r10, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	pc = 0x82EF7B00; continue 'dispatch;
            }
            0x82EF7B00 => {
    //   block [0x82EF7B00..0x82EF7B1C)
	// 82EF7B00: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF7B04: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EF7B08: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B0C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EF7B10: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B14: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7B18: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	pc = 0x82EF7B1C; continue 'dispatch;
            }
            0x82EF7B1C => {
    //   block [0x82EF7B1C..0x82EF7B28)
	// 82EF7B1C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EF7B20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7B24: 4BDB1938  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7B28 size=372
    let mut pc: u32 = 0x82EF7B28;
    'dispatch: loop {
        match pc {
            0x82EF7B28 => {
    //   block [0x82EF7B28..0x82EF7B54)
	// 82EF7B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7B2C: 4BDB18E1  bl 0x82ca940c
	ctx.lr = 0x82EF7B30;
	sub_82CA93D0(ctx, base);
	// 82EF7B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7B34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7B38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF7B3C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF7B40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF7B48: 409A000C  bne cr6, 0x82ef7b54
	if !ctx.cr[6].eq {
	pc = 0x82EF7B54; continue 'dispatch;
	}
	// 82EF7B4C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EF7B50: 4800002C  b 0x82ef7b7c
	pc = 0x82EF7B7C; continue 'dispatch;
            }
            0x82EF7B54 => {
    //   block [0x82EF7B54..0x82EF7B7C)
	// 82EF7B54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B58: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B5C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF7B60: 1D290005  mulli r9, r9, 5
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * 5 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82EF7B64: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7B68: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF7B6C: 40990018  ble cr6, 0x82ef7b84
	if !ctx.cr[6].gt {
	pc = 0x82EF7B84; continue 'dispatch;
	}
	// 82EF7B70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF7B78: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	pc = 0x82EF7B7C; continue 'dispatch;
            }
            0x82EF7B7C => {
    //   block [0x82EF7B7C..0x82EF7B84)
	// 82EF7B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7B80: 480004A1  bl 0x82ef8020
	ctx.lr = 0x82EF7B84;
	sub_82EF8020(ctx, base);
	pc = 0x82EF7B84; continue 'dispatch;
            }
            0x82EF7B84 => {
    //   block [0x82EF7B84..0x82EF7BCC)
	// 82EF7B84: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B88: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B8C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B90: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 82EF7B94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF7B98: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82EF7B9C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7BA0: 550A1838  slwi r10, r8, 3
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7BA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7BA8: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF7BAC: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF7BB0: 2F04FFFE  cmpwi cr6, r4, -2
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2, &mut ctx.xer);
	// 82EF7BB4: 409A0018  bne cr6, 0x82ef7bcc
	if !ctx.cr[6].eq {
	pc = 0x82EF7BCC; continue 'dispatch;
	}
	// 82EF7BB8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EF7BBC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7BC0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7BC4: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7BC8: 480000CC  b 0x82ef7c94
	pc = 0x82EF7C94; continue 'dispatch;
            }
            0x82EF7BCC => {
    //   block [0x82EF7BCC..0x82EF7BD4)
	// 82EF7BCC: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7BD0: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	pc = 0x82EF7BD4; continue 'dispatch;
            }
            0x82EF7BD4 => {
    //   block [0x82EF7BD4..0x82EF7C2C)
	// 82EF7BD4: 39460001  addi r10, r6, 1
	ctx.r[10].s64 = ctx.r[6].s64 + 1;
	// 82EF7BD8: 7D464038  and r6, r10, r8
	ctx.r[6].u64 = ctx.r[10].u64 & ctx.r[8].u64;
	// 82EF7BDC: 39460001  addi r10, r6, 1
	ctx.r[10].s64 = ctx.r[6].s64 + 1;
	// 82EF7BE0: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7BE4: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF7BE8: 2F0AFFFE  cmpwi cr6, r10, -2
	ctx.cr[6].compare_i32(ctx.r[10].s32, -2, &mut ctx.xer);
	// 82EF7BEC: 409AFFE8  bne cr6, 0x82ef7bd4
	if !ctx.cr[6].eq {
	pc = 0x82EF7BD4; continue 'dispatch;
	}
	// 82EF7BF0: 80650004  lwz r3, 4(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7BF4: 38E60001  addi r7, r6, 1
	ctx.r[7].s64 = ctx.r[6].s64 + 1;
	// 82EF7BF8: 39450004  addi r10, r5, 4
	ctx.r[10].s64 = ctx.r[5].s64 + 4;
	// 82EF7BFC: 547FD1BE  srwi r31, r3, 6
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EF7C00: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF7C04: 7FE31A78  xor r3, r31, r3
	ctx.r[3].u64 = ctx.r[31].u64 ^ ctx.r[3].u64;
	// 82EF7C08: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EF7C0C: 7C634038  and r3, r3, r8
	ctx.r[3].u64 = ctx.r[3].u64 & ctx.r[8].u64;
	// 82EF7C10: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF7C14: 409A0028  bne cr6, 0x82ef7c3c
	if !ctx.cr[6].eq {
	pc = 0x82EF7C3C; continue 'dispatch;
	}
	// 82EF7C18: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EF7C1C: 419A0010  beq cr6, 0x82ef7c2c
	if ctx.cr[6].eq {
	pc = 0x82EF7C2C; continue 'dispatch;
	}
	// 82EF7C20: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF7C24: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C28: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF7C2C; continue 'dispatch;
            }
            0x82EF7C2C => {
    //   block [0x82EF7C2C..0x82EF7C3C)
	// 82EF7C2C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C30: 90C50000  stw r6, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF7C34: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7C38: 4800005C  b 0x82ef7c94
	pc = 0x82EF7C94; continue 'dispatch;
            }
            0x82EF7C3C => {
    //   block [0x82EF7C3C..0x82EF7C50)
	// 82EF7C3C: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C40: 547FD1BE  srwi r31, r3, 6
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EF7C44: 7FE31A78  xor r3, r31, r3
	ctx.r[3].u64 = ctx.r[31].u64 ^ ctx.r[3].u64;
	// 82EF7C48: 7C684038  and r8, r3, r8
	ctx.r[8].u64 = ctx.r[3].u64 & ctx.r[8].u64;
	// 82EF7C4C: 48000008  b 0x82ef7c54
	pc = 0x82EF7C54; continue 'dispatch;
            }
            0x82EF7C50 => {
    //   block [0x82EF7C50..0x82EF7C54)
	// 82EF7C50: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82EF7C54; continue 'dispatch;
            }
            0x82EF7C54 => {
    //   block [0x82EF7C54..0x82EF7C80)
	// 82EF7C54: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82EF7C58: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF7C5C: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82EF7C60: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C64: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF7C68: 409AFFE8  bne cr6, 0x82ef7c50
	if !ctx.cr[6].eq {
	pc = 0x82EF7C50; continue 'dispatch;
	}
	// 82EF7C6C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EF7C70: 419A0010  beq cr6, 0x82ef7c80
	if ctx.cr[6].eq {
	pc = 0x82EF7C80; continue 'dispatch;
	}
	// 82EF7C74: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF7C78: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C7C: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF7C80; continue 'dispatch;
            }
            0x82EF7C80 => {
    //   block [0x82EF7C80..0x82EF7C94)
	// 82EF7C80: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EF7C84: 90C80000  stw r6, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF7C88: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C8C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7C90: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	pc = 0x82EF7C94; continue 'dispatch;
            }
            0x82EF7C94 => {
    //   block [0x82EF7C94..0x82EF7C9C)
	// 82EF7C94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7C98: 4BDB17C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7CA0 size=280
    let mut pc: u32 = 0x82EF7CA0;
    'dispatch: loop {
        match pc {
            0x82EF7CA0 => {
    //   block [0x82EF7CA0..0x82EF7D00)
	// 82EF7CA0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82EF7CA4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EF7CA8: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7CAC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF7CB0: 419A00FC  beq cr6, 0x82ef7dac
	if ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7CB4: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7CB8: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7CBC: 54CBD1BE  srwi r11, r6, 6
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7CC0: 7D6B3278  xor r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[6].u64;
	// 82EF7CC4: 7D6A3838  and r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	// 82EF7CC8: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82EF7CCC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7CD0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EF7CD4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7CD8: 2F09FFFE  cmpwi cr6, r9, -2
	ctx.cr[6].compare_i32(ctx.r[9].s32, -2, &mut ctx.xer);
	// 82EF7CDC: 419A00D0  beq cr6, 0x82ef7dac
	if ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7CE0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7CE4: 5525D1BE  srwi r5, r9, 6
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shr(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF7CE8: 7CA94A78  xor r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 ^ ctx.r[9].u64;
	// 82EF7CEC: 7D293838  and r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[7].u64;
	// 82EF7CF0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF7CF4: 409A00B8  bne cr6, 0x82ef7dac
	if !ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7CF8: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82EF7CFC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	pc = 0x82EF7D00; continue 'dispatch;
            }
            0x82EF7D00 => {
    //   block [0x82EF7D00..0x82EF7D28)
	// 82EF7D00: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7D04: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82EF7D08: 57FED1BE  srwi r30, r31, 6
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shr(6);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EF7D0C: 7FDFFA78  xor r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 ^ ctx.r[31].u64;
	// 82EF7D10: 7FFF3838  and r31, r31, r7
	ctx.r[31].u64 = ctx.r[31].u64 & ctx.r[7].u64;
	// 82EF7D14: 7F1F2840  cmplw cr6, r31, r5
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF7D18: 409A0010  bne cr6, 0x82ef7d28
	if !ctx.cr[6].eq {
	pc = 0x82EF7D28; continue 'dispatch;
	}
	// 82EF7D1C: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D20: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82EF7D24: 419A0024  beq cr6, 0x82ef7d48
	if ctx.cr[6].eq {
	pc = 0x82EF7D48; continue 'dispatch;
	}
	pc = 0x82EF7D28; continue 'dispatch;
            }
            0x82EF7D28 => {
    //   block [0x82EF7D28..0x82EF7D48)
	// 82EF7D28: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82EF7D2C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D30: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82EF7D34: 419A0078  beq cr6, 0x82ef7dac
	if ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7D38: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82EF7D3C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7D40: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EF7D44: 4BFFFFBC  b 0x82ef7d00
	pc = 0x82EF7D00; continue 'dispatch;
            }
            0x82EF7D48 => {
    //   block [0x82EF7D48..0x82EF7D88)
	// 82EF7D48: 3920FFFE  li r9, -2
	ctx.r[9].s64 = -2;
	// 82EF7D4C: 7F055000  cmpw cr6, r5, r10
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF7D50: 409A0038  bne cr6, 0x82ef7d88
	if !ctx.cr[6].eq {
	pc = 0x82EF7D88; continue 'dispatch;
	}
	// 82EF7D54: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D58: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82EF7D5C: 419A003C  beq cr6, 0x82ef7d98
	if ctx.cr[6].eq {
	pc = 0x82EF7D98; continue 'dispatch;
	}
	// 82EF7D60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF7D64: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF7D68: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7D6C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82EF7D70: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D74: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF7D78: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7D7C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF7D80: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF7D84: 48000014  b 0x82ef7d98
	pc = 0x82EF7D98; continue 'dispatch;
            }
            0x82EF7D88 => {
    //   block [0x82EF7D88..0x82EF7D98)
	// 82EF7D88: 39440001  addi r10, r4, 1
	ctx.r[10].s64 = ctx.r[4].s64 + 1;
	// 82EF7D8C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D90: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7D94: 7CEA412E  stwx r7, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u32) };
	pc = 0x82EF7D98; continue 'dispatch;
            }
            0x82EF7D98 => {
    //   block [0x82EF7D98..0x82EF7DAC)
	// 82EF7D98: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF7D9C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7DA0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7DA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF7DA8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF7DAC; continue 'dispatch;
            }
            0x82EF7DAC => {
    //   block [0x82EF7DAC..0x82EF7DB8)
	// 82EF7DAC: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7DB0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EF7DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7DB8 size=40
    let mut pc: u32 = 0x82EF7DB8;
    'dispatch: loop {
        match pc {
            0x82EF7DB8 => {
    //   block [0x82EF7DB8..0x82EF7DC0)
	// 82EF7DB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF7DBC: 38E30004  addi r7, r3, 4
	ctx.r[7].s64 = ctx.r[3].s64 + 4;
	pc = 0x82EF7DC0; continue 'dispatch;
            }
            0x82EF7DC0 => {
    //   block [0x82EF7DC0..0x82EF7DE0)
	// 82EF7DC0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF7DC4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7DC8: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF7DCC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF7DD0: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7DD4: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7DD8: 4082FFE8  bne 0x82ef7dc0
	if !ctx.cr[0].eq {
	pc = 0x82EF7DC0; continue 'dispatch;
	}
	// 82EF7DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7DE0 size=92
    let mut pc: u32 = 0x82EF7DE0;
    'dispatch: loop {
        match pc {
            0x82EF7DE0 => {
    //   block [0x82EF7DE0..0x82EF7DEC)
	// 82EF7DE0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7DE4: 39430004  addi r10, r3, 4
	ctx.r[10].s64 = ctx.r[3].s64 + 4;
	// 82EF7DE8: 48000044  b 0x82ef7e2c
	pc = 0x82EF7E2C; continue 'dispatch;
            }
            0x82EF7DEC => {
    //   block [0x82EF7DEC..0x82EF7DF0)
	// 82EF7DEC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	pc = 0x82EF7DF0; continue 'dispatch;
            }
            0x82EF7DF0 => {
    //   block [0x82EF7DF0..0x82EF7E14)
	// 82EF7DF0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF7DF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7DF8: 7D005028  lwarx r8, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 82EF7DFC: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF7E00: 409A0014  bne cr6, 0x82ef7e14
	if !ctx.cr[6].eq {
	pc = 0x82EF7E14; continue 'dispatch;
	}
	// 82EF7E04: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7E08: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7E0C: 4082FFE4  bne 0x82ef7df0
	if !ctx.cr[0].eq {
	pc = 0x82EF7DF0; continue 'dispatch;
	}
	// 82EF7E10: 4800000C  b 0x82ef7e1c
	pc = 0x82EF7E1C; continue 'dispatch;
            }
            0x82EF7E14 => {
    //   block [0x82EF7E14..0x82EF7E1C)
	// 82EF7E14: 7D00512D  stwcx. r8, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7E18: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	pc = 0x82EF7E1C; continue 'dispatch;
            }
            0x82EF7E1C => {
    //   block [0x82EF7E1C..0x82EF7E2C)
	// 82EF7E1C: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82EF7E20: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF7E24: 419A0018  beq cr6, 0x82ef7e3c
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82EF7E3C);
		return;
	}
	// 82EF7E28: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82EF7E2C; continue 'dispatch;
            }
            0x82EF7E2C => {
    //   block [0x82EF7E2C..0x82EF7E3C)
	// 82EF7E2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF7E30: 409AFFBC  bne cr6, 0x82ef7dec
	if !ctx.cr[6].eq {
	pc = 0x82EF7DEC; continue 'dispatch;
	}
	// 82EF7E34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF7E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7E48 size=108
    let mut pc: u32 = 0x82EF7E48;
    'dispatch: loop {
        match pc {
            0x82EF7E48 => {
    //   block [0x82EF7E48..0x82EF7E80)
	// 82EF7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7E58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7E5C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EF7E60: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF7E64: 480010DD  bl 0x82ef8f40
	ctx.lr = 0x82EF7E68;
	sub_82EF8F40(ctx, base);
	// 82EF7E68: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF7E6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF7E70: 419A002C  beq cr6, 0x82ef7e9c
	if ctx.cr[6].eq {
	pc = 0x82EF7E9C; continue 'dispatch;
	}
	// 82EF7E74: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF7E78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF7E7C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	pc = 0x82EF7E80; continue 'dispatch;
            }
            0x82EF7E80 => {
    //   block [0x82EF7E80..0x82EF7E9C)
	// 82EF7E80: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF7E84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7E88: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EF7E8C: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF7E90: 7D00592D  stwcx. r8, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7E94: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7E98: 4082FFE8  bne 0x82ef7e80
	if !ctx.cr[0].eq {
	pc = 0x82EF7E80; continue 'dispatch;
	}
	pc = 0x82EF7E9C; continue 'dispatch;
            }
            0x82EF7E9C => {
    //   block [0x82EF7E9C..0x82EF7EB4)
	// 82EF7E9C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF7EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF7EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7EAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7EB8 size=168
    let mut pc: u32 = 0x82EF7EB8;
    'dispatch: loop {
        match pc {
            0x82EF7EB8 => {
    //   block [0x82EF7EB8..0x82EF7EE0)
	// 82EF7EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7EBC: 4BDB1551  bl 0x82ca940c
	ctx.lr = 0x82EF7EC0;
	sub_82CA93D0(ctx, base);
	// 82EF7EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7EC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7ECC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EF7ED0: 409A0034  bne cr6, 0x82ef7f04
	if !ctx.cr[6].eq {
	pc = 0x82EF7F04; continue 'dispatch;
	}
	// 82EF7ED4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7ED8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EF7EDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	pc = 0x82EF7EE0; continue 'dispatch;
            }
            0x82EF7EE0 => {
    //   block [0x82EF7EE0..0x82EF7F04)
	// 82EF7EE0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF7EE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7EE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF7EEC: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EF7EF0: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7EF4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7EF8: 4082FFE8  bne 0x82ef7ee0
	if !ctx.cr[0].eq {
	pc = 0x82EF7EE0; continue 'dispatch;
	}
	// 82EF7EFC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7F00: 48000058  b 0x82ef7f58
	pc = 0x82EF7F58; continue 'dispatch;
            }
            0x82EF7F04 => {
    //   block [0x82EF7F04..0x82EF7F4C)
	// 82EF7F04: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82EF7F08: 419A004C  beq cr6, 0x82ef7f54
	if ctx.cr[6].eq {
	pc = 0x82EF7F54; continue 'dispatch;
	}
	// 82EF7F0C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7F10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7F14: 4BFFFF35  bl 0x82ef7e48
	ctx.lr = 0x82EF7F18;
	sub_82EF7E48(ctx, base);
	// 82EF7F18: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF7F1C: 41820030  beq 0x82ef7f4c
	if ctx.cr[0].eq {
	pc = 0x82EF7F4C; continue 'dispatch;
	}
	// 82EF7F20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF7F24: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EF7F28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF7F2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7F30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7F34: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7F38: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7F40: 4E800421  bctrl
	ctx.lr = 0x82EF7F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7F44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF7F48: 48000010  b 0x82ef7f58
	pc = 0x82EF7F58; continue 'dispatch;
            }
            0x82EF7F4C => {
    //   block [0x82EF7F4C..0x82EF7F54)
	// 82EF7F4C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EF7F50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF7F54; continue 'dispatch;
            }
            0x82EF7F54 => {
    //   block [0x82EF7F54..0x82EF7F58)
	// 82EF7F54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF7F58; continue 'dispatch;
            }
            0x82EF7F58 => {
    //   block [0x82EF7F58..0x82EF7F60)
	// 82EF7F58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7F5C: 4BDB1500  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7F60 size=88
    let mut pc: u32 = 0x82EF7F60;
    'dispatch: loop {
        match pc {
            0x82EF7F60 => {
    //   block [0x82EF7F60..0x82EF7FB8)
	// 82EF7F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF7F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7F74: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF7F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF7F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7F80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF7F84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF7F88: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF7F8C: 4BFFF4ED  bl 0x82ef7478
	ctx.lr = 0x82EF7F90;
	sub_82EF7478(ctx, base);
	// 82EF7F90: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EF7F94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7F9C: 4BFFFA05  bl 0x82ef79a0
	ctx.lr = 0x82EF7FA0;
	sub_82EF79A0(ctx, base);
	// 82EF7FA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7FAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF7FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7FB8 size=100
    let mut pc: u32 = 0x82EF7FB8;
    'dispatch: loop {
        match pc {
            0x82EF7FB8 => {
    //   block [0x82EF7FB8..0x82EF8004)
	// 82EF7FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7FC8: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7FCC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7FD0: 419A0034  beq cr6, 0x82ef8004
	if ctx.cr[6].eq {
	pc = 0x82EF8004; continue 'dispatch;
	}
	// 82EF7FD4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7FD8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7FDC: 5569D1BE  srwi r9, r11, 6
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF7FE0: 7D2B5A78  xor r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 ^ ctx.r[11].u64;
	// 82EF7FE4: 7D655038  and r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82EF7FE8: 4BFFF619  bl 0x82ef7600
	ctx.lr = 0x82EF7FEC;
	sub_82EF7600(ctx, base);
	// 82EF7FEC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF7FF0: 41800014  blt 0x82ef8004
	if ctx.cr[0].lt {
	pc = 0x82EF8004; continue 'dispatch;
	}
	// 82EF7FF4: 546B1838  slwi r11, r3, 3
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7FF8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF7FFC: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EF8000: 48000008  b 0x82ef8008
	pc = 0x82EF8008; continue 'dispatch;
            }
            0x82EF8004 => {
    //   block [0x82EF8004..0x82EF8008)
	// 82EF8004: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF8008; continue 'dispatch;
            }
            0x82EF8008 => {
    //   block [0x82EF8008..0x82EF801C)
	// 82EF8008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF800C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF8020 size=320
    let mut pc: u32 = 0x82EF8020;
    'dispatch: loop {
        match pc {
            0x82EF8020 => {
    //   block [0x82EF8020..0x82EF803C)
	// 82EF8020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8024: 4BDB13DD  bl 0x82ca9400
	ctx.lr = 0x82EF8028;
	sub_82CA93D0(ctx, base);
	// 82EF8028: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF802C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EF8030: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF8034: 409A0008  bne cr6, 0x82ef803c
	if !ctx.cr[6].eq {
	pc = 0x82EF803C; continue 'dispatch;
	}
	// 82EF8038: 4800011C  b 0x82ef8154
	pc = 0x82EF8154; continue 'dispatch;
            }
            0x82EF803C => {
    //   block [0x82EF803C..0x82EF804C)
	// 82EF803C: 2B040008  cmplwi cr6, r4, 8
	ctx.cr[6].compare_u32(ctx.r[4].u32, 8 as u32, &mut ctx.xer);
	// 82EF8040: 4098000C  bge cr6, 0x82ef804c
	if !ctx.cr[6].lt {
	pc = 0x82EF804C; continue 'dispatch;
	}
	// 82EF8044: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82EF8048: 4800004C  b 0x82ef8094
	pc = 0x82EF8094; continue 'dispatch;
            }
            0x82EF804C => {
    //   block [0x82EF804C..0x82EF8094)
	// 82EF804C: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EF8050: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF8054: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82EF8058: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EF805C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82EF8060: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82EF8064: 4B2FBD4D  bl 0x821f3db0
	ctx.lr = 0x82EF8068;
	sub_821F3DB0(ctx, base);
	// 82EF8068: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EF806C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF8070: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EF8074: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF8078: C00B3FA8  lfs f0, 0x3fa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF807C: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF8080: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82EF8084: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EF8088: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82EF808C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF8090: 7D3F5830  slw r31, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x82EF8094; continue 'dispatch;
            }
            0x82EF8094 => {
    //   block [0x82EF8094..0x82EF80CC)
	// 82EF8094: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF8098: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82EF809C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF80A0: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF80A4: 480057BD  bl 0x82efd860
	ctx.lr = 0x82EF80A8;
	sub_82EFD860(ctx, base);
	// 82EF80A8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF80AC: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82EF80B0: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EF80B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF80B8: 3B60FFFE  li r27, -2
	ctx.r[27].s64 = -2;
	// 82EF80BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF80C0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF80C4: 419A0020  beq cr6, 0x82ef80e4
	if ctx.cr[6].eq {
	pc = 0x82EF80E4; continue 'dispatch;
	}
	// 82EF80C8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x82EF80CC; continue 'dispatch;
            }
            0x82EF80CC => {
    //   block [0x82EF80CC..0x82EF80E4)
	// 82EF80CC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF80D0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EF80D4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF80D8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF80DC: 936A0008  stw r27, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82EF80E0: 4082FFEC  bne 0x82ef80cc
	if !ctx.cr[0].eq {
	pc = 0x82EF80CC; continue 'dispatch;
	}
	pc = 0x82EF80E4; continue 'dispatch;
            }
            0x82EF80E4 => {
    //   block [0x82EF80E4..0x82EF80FC)
	// 82EF80E4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF80E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF80EC: 419A0058  beq cr6, 0x82ef8144
	if ctx.cr[6].eq {
	pc = 0x82EF8144; continue 'dispatch;
	}
	// 82EF80F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF80F4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82EF80F8: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	pc = 0x82EF80FC; continue 'dispatch;
            }
            0x82EF80FC => {
    //   block [0x82EF80FC..0x82EF8130)
	// 82EF80FC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8100: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EF8104: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF8108: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF810C: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF8110: 419A0020  beq cr6, 0x82ef8130
	if ctx.cr[6].eq {
	pc = 0x82EF8130; continue 'dispatch;
	}
	// 82EF8114: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8118: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82EF811C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8120: 556AD1BE  srwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8124: 7D455A78  xor r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82EF8128: 4BFFFA01  bl 0x82ef7b28
	ctx.lr = 0x82EF812C;
	sub_82EF7B28(ctx, base);
	// 82EF812C: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	pc = 0x82EF8130; continue 'dispatch;
            }
            0x82EF8130 => {
    //   block [0x82EF8130..0x82EF8144)
	// 82EF8130: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF8134: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82EF8138: 4082FFC4  bne 0x82ef80fc
	if !ctx.cr[0].eq {
	pc = 0x82EF80FC; continue 'dispatch;
	}
	// 82EF813C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8140: 48005761  bl 0x82efd8a0
	ctx.lr = 0x82EF8144;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF8144; continue 'dispatch;
            }
            0x82EF8144 => {
    //   block [0x82EF8144..0x82EF8154)
	// 82EF8144: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF814C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF8150: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF8154; continue 'dispatch;
            }
            0x82EF8154 => {
    //   block [0x82EF8154..0x82EF8160)
	// 82EF8154: 480754A5  bl 0x82f6d5f8
	ctx.lr = 0x82EF8158;
	sub_82F6D5F8(ctx, base);
	// 82EF8158: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF815C: 4BDB12F4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF8160 size=308
    let mut pc: u32 = 0x82EF8160;
    'dispatch: loop {
        match pc {
            0x82EF8160 => {
    //   block [0x82EF8160..0x82EF817C)
	// 82EF8160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8164: 4BDB129D  bl 0x82ca9400
	ctx.lr = 0x82EF8168;
	sub_82CA93D0(ctx, base);
	// 82EF8168: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF816C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EF8170: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF8174: 409A0008  bne cr6, 0x82ef817c
	if !ctx.cr[6].eq {
	pc = 0x82EF817C; continue 'dispatch;
	}
	// 82EF8178: 48000110  b 0x82ef8288
	pc = 0x82EF8288; continue 'dispatch;
            }
            0x82EF817C => {
    //   block [0x82EF817C..0x82EF818C)
	// 82EF817C: 2B040008  cmplwi cr6, r4, 8
	ctx.cr[6].compare_u32(ctx.r[4].u32, 8 as u32, &mut ctx.xer);
	// 82EF8180: 4098000C  bge cr6, 0x82ef818c
	if !ctx.cr[6].lt {
	pc = 0x82EF818C; continue 'dispatch;
	}
	// 82EF8184: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82EF8188: 4800004C  b 0x82ef81d4
	pc = 0x82EF81D4; continue 'dispatch;
            }
            0x82EF818C => {
    //   block [0x82EF818C..0x82EF81D4)
	// 82EF818C: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EF8190: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF8194: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82EF8198: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EF819C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82EF81A0: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82EF81A4: 4B2FBC0D  bl 0x821f3db0
	ctx.lr = 0x82EF81A8;
	sub_821F3DB0(ctx, base);
	// 82EF81A8: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EF81AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF81B0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EF81B4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF81B8: C00B3FA8  lfs f0, 0x3fa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF81BC: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF81C0: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82EF81C4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EF81C8: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82EF81CC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF81D0: 7D3F5830  slw r31, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x82EF81D4; continue 'dispatch;
            }
            0x82EF81D4 => {
    //   block [0x82EF81D4..0x82EF820C)
	// 82EF81D4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF81D8: 57EB2036  slwi r11, r31, 4
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF81DC: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF81E0: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EF81E4: 4800567D  bl 0x82efd860
	ctx.lr = 0x82EF81E8;
	sub_82EFD860(ctx, base);
	// 82EF81E8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF81EC: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82EF81F0: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EF81F4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF81F8: 3B60FFFE  li r27, -2
	ctx.r[27].s64 = -2;
	// 82EF81FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF8200: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF8204: 419A0020  beq cr6, 0x82ef8224
	if ctx.cr[6].eq {
	pc = 0x82EF8224; continue 'dispatch;
	}
	// 82EF8208: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x82EF820C; continue 'dispatch;
            }
            0x82EF820C => {
    //   block [0x82EF820C..0x82EF8224)
	// 82EF820C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8210: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EF8214: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8218: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82EF821C: 936A0008  stw r27, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82EF8220: 4082FFEC  bne 0x82ef820c
	if !ctx.cr[0].eq {
	pc = 0x82EF820C; continue 'dispatch;
	}
	pc = 0x82EF8224; continue 'dispatch;
            }
            0x82EF8224 => {
    //   block [0x82EF8224..0x82EF823C)
	// 82EF8224: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF822C: 419A004C  beq cr6, 0x82ef8278
	if ctx.cr[6].eq {
	pc = 0x82EF8278; continue 'dispatch;
	}
	// 82EF8230: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8234: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82EF8238: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	pc = 0x82EF823C; continue 'dispatch;
            }
            0x82EF823C => {
    //   block [0x82EF823C..0x82EF8264)
	// 82EF823C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8240: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EF8244: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF8248: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF824C: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF8250: 419A0014  beq cr6, 0x82ef8264
	if ctx.cr[6].eq {
	pc = 0x82EF8264; continue 'dispatch;
	}
	// 82EF8254: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82EF8258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF825C: 4BFFFD05  bl 0x82ef7f60
	ctx.lr = 0x82EF8260;
	sub_82EF7F60(ctx, base);
	// 82EF8260: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	pc = 0x82EF8264; continue 'dispatch;
            }
            0x82EF8264 => {
    //   block [0x82EF8264..0x82EF8278)
	// 82EF8264: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF8268: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82EF826C: 4082FFD0  bne 0x82ef823c
	if !ctx.cr[0].eq {
	pc = 0x82EF823C; continue 'dispatch;
	}
	// 82EF8270: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8274: 4800562D  bl 0x82efd8a0
	ctx.lr = 0x82EF8278;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF8278; continue 'dispatch;
            }
            0x82EF8278 => {
    //   block [0x82EF8278..0x82EF8288)
	// 82EF8278: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF827C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8280: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF8284: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF8288; continue 'dispatch;
            }
            0x82EF8288 => {
    //   block [0x82EF8288..0x82EF8294)
	// 82EF8288: 480237C1  bl 0x82f1ba48
	ctx.lr = 0x82EF828C;
	sub_82F1BA48(ctx, base);
	// 82EF828C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8290: 4BDB11C0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8298 size=156
    let mut pc: u32 = 0x82EF8298;
    'dispatch: loop {
        match pc {
            0x82EF8298 => {
    //   block [0x82EF8298..0x82EF82B4)
	// 82EF8298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF82A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF82A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF82A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF82AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EF82B0: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	pc = 0x82EF82B4; continue 'dispatch;
            }
            0x82EF82B4 => {
    //   block [0x82EF82B4..0x82EF8308)
	// 82EF82B4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF82B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF82BC: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF82C0: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF82C4: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF82C8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF82CC: 4082FFE8  bne 0x82ef82b4
	if !ctx.cr[0].eq {
	pc = 0x82EF82B4; continue 'dispatch;
	}
	// 82EF82D0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF82D4: 7C2004AC  lwsync
	// 82EF82D8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF82DC: 40820044  bne 0x82ef8320
	if !ctx.cr[0].eq {
	pc = 0x82EF8320; continue 'dispatch;
	}
	// 82EF82E0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF82E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF82E8: 419A0020  beq cr6, 0x82ef8308
	if ctx.cr[6].eq {
	pc = 0x82EF8308; continue 'dispatch;
	}
	// 82EF82EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF82F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF82F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF82F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF82FC: 4E800421  bctrl
	ctx.lr = 0x82EF8300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF8304: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
            }
            0x82EF8308 => {
    //   block [0x82EF8308..0x82EF8320)
	// 82EF8308: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF830C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF8310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8314: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF831C: 4E800421  bctrl
	ctx.lr = 0x82EF8320;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF8320 => {
    //   block [0x82EF8320..0x82EF8334)
	// 82EF8320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF832C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8338 size=248
    let mut pc: u32 = 0x82EF8338;
    'dispatch: loop {
        match pc {
            0x82EF8338 => {
    //   block [0x82EF8338..0x82EF8388)
	// 82EF8338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF833C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF834C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8350: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF8354: 394ACF9C  addi r10, r10, -0x3064
	ctx.r[10].s64 = ctx.r[10].s64 + -12388;
	// 82EF8358: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF835C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8360: 3BCB0014  addi r30, r11, 0x14
	ctx.r[30].s64 = ctx.r[11].s64 + 20;
	// 82EF8364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8368: 483C15FD  bl 0x832b9964
	ctx.lr = 0x82EF836C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF836C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8370: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8374: 409A0014  bne cr6, 0x82ef8388
	if !ctx.cr[6].eq {
	pc = 0x82EF8388; continue 'dispatch;
	}
	// 82EF8378: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF837C: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82EF8380: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82EF8384: 4BFFF43D  bl 0x82ef77c0
	ctx.lr = 0x82EF8388;
	sub_82EF77C0(ctx, base);
	pc = 0x82EF8388; continue 'dispatch;
            }
            0x82EF8388 => {
    //   block [0x82EF8388..0x82EF83A4)
	// 82EF8388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF838C: 483C15C9  bl 0x832b9954
	ctx.lr = 0x82EF8390;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8390: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF8394: 48001875  bl 0x82ef9c08
	ctx.lr = 0x82EF8398;
	sub_82EF9C08(ctx, base);
	// 82EF8398: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF839C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EF83A0: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	pc = 0x82EF83A4; continue 'dispatch;
            }
            0x82EF83A4 => {
    //   block [0x82EF83A4..0x82EF83D0)
	// 82EF83A4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF83A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF83AC: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EF83B0: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF83B4: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF83B8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF83BC: 4082FFE8  bne 0x82ef83a4
	if !ctx.cr[0].eq {
	pc = 0x82EF83A4; continue 'dispatch;
	}
	// 82EF83C0: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EF83C4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF83C8: 40820008  bne 0x82ef83d0
	if !ctx.cr[0].eq {
	pc = 0x82EF83D0; continue 'dispatch;
	}
	// 82EF83CC: 480054D5  bl 0x82efd8a0
	ctx.lr = 0x82EF83D0;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF83D0; continue 'dispatch;
            }
            0x82EF83D0 => {
    //   block [0x82EF83D0..0x82EF83F0)
	// 82EF83D0: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF83D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF83D8: 419A0018  beq cr6, 0x82ef83f0
	if ctx.cr[6].eq {
	pc = 0x82EF83F0; continue 'dispatch;
	}
	// 82EF83DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF83E0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF83E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF83E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF83EC: 4E800421  bctrl
	ctx.lr = 0x82EF83F0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF83F0 => {
    //   block [0x82EF83F0..0x82EF8410)
	// 82EF83F0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF83F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF83F8: 419A0018  beq cr6, 0x82ef8410
	if ctx.cr[6].eq {
	pc = 0x82EF8410; continue 'dispatch;
	}
	// 82EF83FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8400: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF8404: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF840C: 4E800421  bctrl
	ctx.lr = 0x82EF8410;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF8410 => {
    //   block [0x82EF8410..0x82EF8430)
	// 82EF8410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8414: 48005605  bl 0x82efda18
	ctx.lr = 0x82EF8418;
	sub_82EFDA18(ctx, base);
	// 82EF8418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF841C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8430 size=144
    let mut pc: u32 = 0x82EF8430;
    'dispatch: loop {
        match pc {
            0x82EF8430 => {
    //   block [0x82EF8430..0x82EF8490)
	// 82EF8430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8434: 4BDB0FD5  bl 0x82ca9408
	ctx.lr = 0x82EF8438;
	sub_82CA93D0(ctx, base);
	// 82EF8438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF843C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8440: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF8444: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8448: 3B8B0014  addi r28, r11, 0x14
	ctx.r[28].s64 = ctx.r[11].s64 + 20;
	// 82EF844C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8450: 483C1515  bl 0x832b9964
	ctx.lr = 0x82EF8454;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF8454: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF8458: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82EF845C: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82EF8460: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF8464: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8468: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 82EF846C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8470: 4BFFF4A1  bl 0x82ef7910
	ctx.lr = 0x82EF8474;
	sub_82EF7910(ctx, base);
	// 82EF8474: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF8478: 41800018  blt 0x82ef8490
	if ctx.cr[0].lt {
	pc = 0x82EF8490; continue 'dispatch;
	}
	// 82EF847C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF8480: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8484: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8488: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF848C: 48000008  b 0x82ef8494
	pc = 0x82EF8494; continue 'dispatch;
            }
            0x82EF8490 => {
    //   block [0x82EF8490..0x82EF8494)
	// 82EF8490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EF8494; continue 'dispatch;
            }
            0x82EF8494 => {
    //   block [0x82EF8494..0x82EF84C0)
	// 82EF8494: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF8498: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EF849C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF84A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF84A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF84A8: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF84AC: 480023AD  bl 0x82efa858
	ctx.lr = 0x82EF84B0;
	sub_82EFA858(ctx, base);
	// 82EF84B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF84B4: 483C14A1  bl 0x832b9954
	ctx.lr = 0x82EF84B8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF84B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF84BC: 4BDB0F9C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF84C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF84C0 size=96
    let mut pc: u32 = 0x82EF84C0;
    'dispatch: loop {
        match pc {
            0x82EF84C0 => {
    //   block [0x82EF84C0..0x82EF8520)
	// 82EF84C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF84C4: 4BDB0F49  bl 0x82ca940c
	ctx.lr = 0x82EF84C8;
	sub_82CA93D0(ctx, base);
	// 82EF84C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF84CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF84D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF84D4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF84D8: 3BCB0014  addi r30, r11, 0x14
	ctx.r[30].s64 = ctx.r[11].s64 + 20;
	// 82EF84DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF84E0: 483C1485  bl 0x832b9964
	ctx.lr = 0x82EF84E4;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF84E4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EF84E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF84EC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF84F0: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82EF84F4: 480060A5  bl 0x82efe598
	ctx.lr = 0x82EF84F8;
	sub_82EFE598(ctx, base);
	// 82EF84F8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF84FC: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82EF8500: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82EF8504: 4BFFF2BD  bl 0x82ef77c0
	ctx.lr = 0x82EF8508;
	sub_82EF77C0(ctx, base);
	// 82EF8508: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF850C: 4800234D  bl 0x82efa858
	ctx.lr = 0x82EF8510;
	sub_82EFA858(ctx, base);
	// 82EF8510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8514: 483C1441  bl 0x832b9954
	ctx.lr = 0x82EF8518;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF851C: 4BDB0F40  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8520 size=108
    let mut pc: u32 = 0x82EF8520;
    'dispatch: loop {
        match pc {
            0x82EF8520 => {
    //   block [0x82EF8520..0x82EF858C)
	// 82EF8520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF852C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8538: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF853C: 4800540D  bl 0x82efd948
	ctx.lr = 0x82EF8540;
	sub_82EFD948(ctx, base);
	// 82EF8540: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF8544: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82EF8548: 396BCFA0  addi r11, r11, -0x3060
	ctx.r[11].s64 = ctx.r[11].s64 + -12384;
	// 82EF854C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF8550: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8554: 480007ED  bl 0x82ef8d40
	ctx.lr = 0x82EF8558;
	sub_82EF8D40(ctx, base);
	// 82EF8558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF855C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8560: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EF8564: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF8568: 480054F1  bl 0x82efda58
	ctx.lr = 0x82EF856C;
	sub_82EFDA58(ctx, base);
	// 82EF856C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EF8570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF857C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8590 size=260
    let mut pc: u32 = 0x82EF8590;
    'dispatch: loop {
        match pc {
            0x82EF8590 => {
    //   block [0x82EF8590..0x82EF85D8)
	// 82EF8590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8594: 4BDB0E75  bl 0x82ca9408
	ctx.lr = 0x82EF8598;
	sub_82CA93D0(ctx, base);
	// 82EF8598: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF859C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF85A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF85A4: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 82EF85A8: 396BCFA0  addi r11, r11, -0x3060
	ctx.r[11].s64 = ctx.r[11].s64 + -12384;
	// 82EF85AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF85B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF85B4: 483C13B1  bl 0x832b9964
	ctx.lr = 0x82EF85B8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF85B8: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF85BC: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 82EF85C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF85C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF85C8: 409A0010  bne cr6, 0x82ef85d8
	if !ctx.cr[6].eq {
	pc = 0x82EF85D8; continue 'dispatch;
	}
	// 82EF85CC: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82EF85D0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82EF85D4: 48000034  b 0x82ef8608
	pc = 0x82EF8608; continue 'dispatch;
            }
            0x82EF85D8 => {
    //   block [0x82EF85D8..0x82EF85E0)
	// 82EF85D8: 392A0008  addi r9, r10, 8
	ctx.r[9].s64 = ctx.r[10].s64 + 8;
	// 82EF85DC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x82EF85E0; continue 'dispatch;
            }
            0x82EF85E0 => {
    //   block [0x82EF85E0..0x82EF8600)
	// 82EF85E0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF85E4: 2F08FFFE  cmpwi cr6, r8, -2
	ctx.cr[6].compare_i32(ctx.r[8].s32, -2, &mut ctx.xer);
	// 82EF85E8: 409A0018  bne cr6, 0x82ef8600
	if !ctx.cr[6].eq {
	pc = 0x82EF8600; continue 'dispatch;
	}
	// 82EF85EC: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF85F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF85F4: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82EF85F8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EF85FC: 4099FFE4  ble cr6, 0x82ef85e0
	if !ctx.cr[6].gt {
	pc = 0x82EF85E0; continue 'dispatch;
	}
	pc = 0x82EF8600; continue 'dispatch;
            }
            0x82EF8600 => {
    //   block [0x82EF8600..0x82EF8608)
	// 82EF8600: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82EF8604: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF8608; continue 'dispatch;
            }
            0x82EF8608 => {
    //   block [0x82EF8608..0x82EF8634)
	// 82EF8608: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82EF860C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF8610: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82EF8614: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF8618: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82EF861C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82EF8620: 48042399  bl 0x82f3a9b8
	ctx.lr = 0x82EF8624;
	sub_82F3A9B8(ctx, base);
	// 82EF8624: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8628: 40820044  bne 0x82ef866c
	if !ctx.cr[0].eq {
	pc = 0x82EF866C; continue 'dispatch;
	}
	// 82EF862C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82EF8630: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	pc = 0x82EF8634; continue 'dispatch;
            }
            0x82EF8634 => {
    //   block [0x82EF8634..0x82EF866C)
	// 82EF8634: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF8638: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF863C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF8640: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8644: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8648: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF864C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8650: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EF8654: 4803A365  bl 0x82f329b8
	ctx.lr = 0x82EF8658;
	sub_82F329B8(ctx, base);
	// 82EF8658: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF865C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF8660: 48042359  bl 0x82f3a9b8
	ctx.lr = 0x82EF8664;
	sub_82F3A9B8(ctx, base);
	// 82EF8664: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8668: 4182FFCC  beq 0x82ef8634
	if ctx.cr[0].eq {
	pc = 0x82EF8634; continue 'dispatch;
	}
	pc = 0x82EF866C; continue 'dispatch;
            }
            0x82EF866C => {
    //   block [0x82EF866C..0x82EF8694)
	// 82EF866C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8670: 483C12E5  bl 0x832b9954
	ctx.lr = 0x82EF8674;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8674: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8678: 480233D1  bl 0x82f1ba48
	ctx.lr = 0x82EF867C;
	sub_82F1BA48(ctx, base);
	// 82EF867C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8680: 480029B9  bl 0x82efb038
	ctx.lr = 0x82EF8684;
	sub_82EFB038(ctx, base);
	// 82EF8684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8688: 48005391  bl 0x82efda18
	ctx.lr = 0x82EF868C;
	sub_82EFDA18(ctx, base);
	// 82EF868C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8690: 4BDB0DC8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8698 size=148
    let mut pc: u32 = 0x82EF8698;
    'dispatch: loop {
        match pc {
            0x82EF8698 => {
    //   block [0x82EF8698..0x82EF86E4)
	// 82EF8698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF869C: 4BDB0D71  bl 0x82ca940c
	ctx.lr = 0x82EF86A0;
	sub_82CA93D0(ctx, base);
	// 82EF86A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF86A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF86A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF86AC: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF86B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF86B4: 483C12B1  bl 0x832b9964
	ctx.lr = 0x82EF86B8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF86B8: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82EF86BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF86C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF86C4: 4BFFF24D  bl 0x82ef7910
	ctx.lr = 0x82EF86C8;
	sub_82EF7910(ctx, base);
	// 82EF86C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF86CC: 41800018  blt 0x82ef86e4
	if ctx.cr[0].lt {
	pc = 0x82EF86E4; continue 'dispatch;
	}
	// 82EF86D0: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF86D4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF86D8: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF86DC: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF86E0: 48000008  b 0x82ef86e8
	pc = 0x82EF86E8; continue 'dispatch;
            }
            0x82EF86E4 => {
    //   block [0x82EF86E4..0x82EF86E8)
	// 82EF86E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	pc = 0x82EF86E8; continue 'dispatch;
            }
            0x82EF86E8 => {
    //   block [0x82EF86E8..0x82EF8710)
	// 82EF86E8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82EF86EC: 419A0038  beq cr6, 0x82ef8724
	if ctx.cr[6].eq {
	pc = 0x82EF8724; continue 'dispatch;
	}
	// 82EF86F0: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF86F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF86F8: 409A002C  bne cr6, 0x82ef8724
	if !ctx.cr[6].eq {
	pc = 0x82EF8724; continue 'dispatch;
	}
	// 82EF86FC: 80660004  lwz r3, 4(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8700: 4BFFF6E1  bl 0x82ef7de0
	ctx.lr = 0x82EF8704;
	sub_82EF7DE0(ctx, base);
	// 82EF8704: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8708: 4182001C  beq 0x82ef8724
	if ctx.cr[0].eq {
	pc = 0x82EF8724; continue 'dispatch;
	}
	// 82EF870C: 83E60004  lwz r31, 4(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x82EF8710; continue 'dispatch;
            }
            0x82EF8710 => {
    //   block [0x82EF8710..0x82EF8724)
	// 82EF8710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8714: 483C1241  bl 0x832b9954
	ctx.lr = 0x82EF8718;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF871C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8720: 4BDB0D3C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82EF8724 => {
    //   block [0x82EF8724..0x82EF872C)
	// 82EF8724: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8728: 4BFFFFE8  b 0x82ef8710
	pc = 0x82EF8710; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8730 size=328
    let mut pc: u32 = 0x82EF8730;
    'dispatch: loop {
        match pc {
            0x82EF8730 => {
    //   block [0x82EF8730..0x82EF8780)
	// 82EF8730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8734: 4BDB0CD1  bl 0x82ca9404
	ctx.lr = 0x82EF8738;
	sub_82CA93D0(ctx, base);
	// 82EF8738: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF873C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF8740: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF8744: 3B7E0014  addi r27, r30, 0x14
	ctx.r[27].s64 = ctx.r[30].s64 + 20;
	// 82EF8748: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF874C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF8750: 483C1215  bl 0x832b9964
	ctx.lr = 0x82EF8754;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF8754: 3BBE0030  addi r29, r30, 0x30
	ctx.r[29].s64 = ctx.r[30].s64 + 48;
	// 82EF8758: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF875C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8760: 4BFFF1B1  bl 0x82ef7910
	ctx.lr = 0x82EF8764;
	sub_82EF7910(ctx, base);
	// 82EF8764: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF8768: 41800018  blt 0x82ef8780
	if ctx.cr[0].lt {
	pc = 0x82EF8780; continue 'dispatch;
	}
	// 82EF876C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF8770: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8774: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8778: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF877C: 48000008  b 0x82ef8784
	pc = 0x82EF8784; continue 'dispatch;
            }
            0x82EF8780 => {
    //   block [0x82EF8780..0x82EF8784)
	// 82EF8780: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	pc = 0x82EF8784; continue 'dispatch;
            }
            0x82EF8784 => {
    //   block [0x82EF8784..0x82EF87B4)
	// 82EF8784: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82EF8788: 419A0050  beq cr6, 0x82ef87d8
	if ctx.cr[6].eq {
	pc = 0x82EF87D8; continue 'dispatch;
	}
	// 82EF878C: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8790: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8794: 409A0080  bne cr6, 0x82ef8814
	if !ctx.cr[6].eq {
	pc = 0x82EF8814; continue 'dispatch;
	}
	// 82EF8798: 80660004  lwz r3, 4(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF879C: 4BFFF645  bl 0x82ef7de0
	ctx.lr = 0x82EF87A0;
	sub_82EF7DE0(ctx, base);
	// 82EF87A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF87A4: 41820028  beq 0x82ef87cc
	if ctx.cr[0].eq {
	pc = 0x82EF87CC; continue 'dispatch;
	}
	// 82EF87A8: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF87AC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82EF87B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF87B4; continue 'dispatch;
            }
            0x82EF87B4 => {
    //   block [0x82EF87B4..0x82EF87B8)
	// 82EF87B4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	pc = 0x82EF87B8; continue 'dispatch;
            }
            0x82EF87B8 => {
    //   block [0x82EF87B8..0x82EF87CC)
	// 82EF87B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF87BC: 483C1199  bl 0x832b9954
	ctx.lr = 0x82EF87C0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF87C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF87C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF87C8: 4BDB0C8C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82EF87CC => {
    //   block [0x82EF87CC..0x82EF87D8)
	// 82EF87CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF87D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF87D4: 4BFFEFED  bl 0x82ef77c0
	ctx.lr = 0x82EF87D8;
	sub_82EF77C0(ctx, base);
	pc = 0x82EF87D8; continue 'dispatch;
            }
            0x82EF87D8 => {
    //   block [0x82EF87D8..0x82EF8814)
	// 82EF87D8: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82EF87DC: 48005085  bl 0x82efd860
	ctx.lr = 0x82EF87E0;
	sub_82EFD860(ctx, base);
	// 82EF87E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF87E4: 4182005C  beq 0x82ef8840
	if ctx.cr[0].eq {
	pc = 0x82EF8840; continue 'dispatch;
	}
	// 82EF87E8: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EF87EC: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EF87F0: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EF87F4: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EF87F8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF87FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF8800: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF8804: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF8808: 4BFFEE89  bl 0x82ef7690
	ctx.lr = 0x82EF880C;
	sub_82EF7690(ctx, base);
	// 82EF880C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF8810: 48000034  b 0x82ef8844
	pc = 0x82EF8844; continue 'dispatch;
            }
            0x82EF8814 => {
    //   block [0x82EF8814..0x82EF8840)
	// 82EF8814: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8818: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82EF881C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF8820: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF8824: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF8828: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF882C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8834: 4E800421  bctrl
	ctx.lr = 0x82EF8838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8838: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF883C: 4BFFFF7C  b 0x82ef87b8
	pc = 0x82EF87B8; continue 'dispatch;
            }
            0x82EF8840 => {
    //   block [0x82EF8840..0x82EF8844)
	// 82EF8840: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82EF8844; continue 'dispatch;
            }
            0x82EF8844 => {
    //   block [0x82EF8844..0x82EF8854)
	// 82EF8844: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8848: 409A000C  bne cr6, 0x82ef8854
	if !ctx.cr[6].eq {
	pc = 0x82EF8854; continue 'dispatch;
	}
	// 82EF884C: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82EF8850: 4BFFFF68  b 0x82ef87b8
	pc = 0x82EF87B8; continue 'dispatch;
            }
            0x82EF8854 => {
    //   block [0x82EF8854..0x82EF8878)
	// 82EF8854: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF8858: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82EF885C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF8860: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF8864: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8868: 4BFFF6F9  bl 0x82ef7f60
	ctx.lr = 0x82EF886C;
	sub_82EF7F60(ctx, base);
	// 82EF886C: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82EF8870: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EF8874: 4BFFFF40  b 0x82ef87b4
	pc = 0x82EF87B4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8878 size=144
    let mut pc: u32 = 0x82EF8878;
    'dispatch: loop {
        match pc {
            0x82EF8878 => {
    //   block [0x82EF8878..0x82EF88DC)
	// 82EF8878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF887C: 4BDB0B91  bl 0x82ca940c
	ctx.lr = 0x82EF8880;
	sub_82CA93D0(ctx, base);
	// 82EF8880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8888: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF888C: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 82EF8890: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82EF8894: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8898: 483C10CD  bl 0x832b9964
	ctx.lr = 0x82EF889C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF889C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF88A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF88A4: 419A0054  beq cr6, 0x82ef88f8
	if ctx.cr[6].eq {
	pc = 0x82EF88F8; continue 'dispatch;
	}
	// 82EF88A8: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 82EF88AC: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EF88B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF88B4: 4BFFF705  bl 0x82ef7fb8
	ctx.lr = 0x82EF88B8;
	sub_82EF7FB8(ctx, base);
	// 82EF88B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF88BC: 4082003C  bne 0x82ef88f8
	if !ctx.cr[0].eq {
	pc = 0x82EF88F8; continue 'dispatch;
	}
	// 82EF88C0: 57CBD1BE  srwi r11, r30, 6
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF88C4: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EF88C8: 7D65F278  xor r5, r11, r30
	ctx.r[5].u64 = ctx.r[11].u64 ^ ctx.r[30].u64;
	// 82EF88CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF88D0: 4BFFF259  bl 0x82ef7b28
	ctx.lr = 0x82EF88D4;
	sub_82EF7B28(ctx, base);
	// 82EF88D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF88D8: 391E0004  addi r8, r30, 4
	ctx.r[8].s64 = ctx.r[30].s64 + 4;
	pc = 0x82EF88DC; continue 'dispatch;
            }
            0x82EF88DC => {
    //   block [0x82EF88DC..0x82EF88F8)
	// 82EF88DC: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF88E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF88E4: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF88E8: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF88EC: 7D20412D  stwcx. r9, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF88F0: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF88F4: 4082FFE8  bne 0x82ef88dc
	if !ctx.cr[0].eq {
	pc = 0x82EF88DC; continue 'dispatch;
	}
	pc = 0x82EF88F8; continue 'dispatch;
            }
            0x82EF88F8 => {
    //   block [0x82EF88F8..0x82EF8908)
	// 82EF88F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF88FC: 483C1059  bl 0x832b9954
	ctx.lr = 0x82EF8900;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8900: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8904: 4BDB0B58  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8908 size=84
    let mut pc: u32 = 0x82EF8908;
    'dispatch: loop {
        match pc {
            0x82EF8908 => {
    //   block [0x82EF8908..0x82EF894C)
	// 82EF8908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF890C: 4BDB0B01  bl 0x82ca940c
	ctx.lr = 0x82EF8910;
	sub_82CA93D0(ctx, base);
	// 82EF8910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8918: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF891C: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF8920: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82EF8924: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8928: 483C103D  bl 0x832b9964
	ctx.lr = 0x82EF892C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF892C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8934: 419A0018  beq cr6, 0x82ef894c
	if ctx.cr[6].eq {
	pc = 0x82EF894C; continue 'dispatch;
	}
	// 82EF8938: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EF893C: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EF8940: 4BFFF361  bl 0x82ef7ca0
	ctx.lr = 0x82EF8944;
	sub_82EF7CA0(ctx, base);
	// 82EF8944: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8948: 4BFFF951  bl 0x82ef8298
	ctx.lr = 0x82EF894C;
	sub_82EF8298(ctx, base);
	pc = 0x82EF894C; continue 'dispatch;
            }
            0x82EF894C => {
    //   block [0x82EF894C..0x82EF895C)
	// 82EF894C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8950: 483C1005  bl 0x832b9954
	ctx.lr = 0x82EF8954;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8958: 4BDB0B04  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8960 size=240
    let mut pc: u32 = 0x82EF8960;
    'dispatch: loop {
        match pc {
            0x82EF8960 => {
    //   block [0x82EF8960..0x82EF89A8)
	// 82EF8960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8964: 4BDB0AA9  bl 0x82ca940c
	ctx.lr = 0x82EF8968;
	sub_82CA93D0(ctx, base);
	// 82EF8968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF896C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF8970: 3BBE0014  addi r29, r30, 0x14
	ctx.r[29].s64 = ctx.r[30].s64 + 20;
	// 82EF8974: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8978: 483C0FED  bl 0x832b9964
	ctx.lr = 0x82EF897C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF897C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8984: 419A00BC  beq cr6, 0x82ef8a40
	if ctx.cr[6].eq {
	pc = 0x82EF8A40; continue 'dispatch;
	}
	// 82EF8988: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF898C: 390B0014  addi r8, r11, 0x14
	ctx.r[8].s64 = ctx.r[11].s64 + 20;
	// 82EF8990: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8994: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF8998: 409A0010  bne cr6, 0x82ef89a8
	if !ctx.cr[6].eq {
	pc = 0x82EF89A8; continue 'dispatch;
	}
	// 82EF899C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF89A0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82EF89A4: 48000034  b 0x82ef89d8
	pc = 0x82EF89D8; continue 'dispatch;
            }
            0x82EF89A8 => {
    //   block [0x82EF89A8..0x82EF89B0)
	// 82EF89A8: 392A0008  addi r9, r10, 8
	ctx.r[9].s64 = ctx.r[10].s64 + 8;
	// 82EF89AC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82EF89B0; continue 'dispatch;
            }
            0x82EF89B0 => {
    //   block [0x82EF89B0..0x82EF89D0)
	// 82EF89B0: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF89B4: 2F07FFFE  cmpwi cr6, r7, -2
	ctx.cr[6].compare_i32(ctx.r[7].s32, -2, &mut ctx.xer);
	// 82EF89B8: 409A0018  bne cr6, 0x82ef89d0
	if !ctx.cr[6].eq {
	pc = 0x82EF89D0; continue 'dispatch;
	}
	// 82EF89BC: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF89C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF89C4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82EF89C8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF89CC: 4099FFE4  ble cr6, 0x82ef89b0
	if !ctx.cr[6].gt {
	pc = 0x82EF89B0; continue 'dispatch;
	}
	pc = 0x82EF89D0; continue 'dispatch;
            }
            0x82EF89D0 => {
    //   block [0x82EF89D0..0x82EF89D8)
	// 82EF89D0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82EF89D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF89D8; continue 'dispatch;
            }
            0x82EF89D8 => {
    //   block [0x82EF89D8..0x82EF89FC)
	// 82EF89D8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82EF89DC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF89E0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82EF89E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF89E8: 48041FD1  bl 0x82f3a9b8
	ctx.lr = 0x82EF89EC;
	sub_82F3A9B8(ctx, base);
	// 82EF89EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF89F0: 40820044  bne 0x82ef8a34
	if !ctx.cr[0].eq {
	pc = 0x82EF8A34; continue 'dispatch;
	}
	// 82EF89F4: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82EF89F8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	pc = 0x82EF89FC; continue 'dispatch;
            }
            0x82EF89FC => {
    //   block [0x82EF89FC..0x82EF8A34)
	// 82EF89FC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8A00: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF8A04: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8A08: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8A0C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8A10: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8A14: 4BFFF885  bl 0x82ef8298
	ctx.lr = 0x82EF8A18;
	sub_82EF8298(ctx, base);
	// 82EF8A18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8A1C: 4BFFED35  bl 0x82ef7750
	ctx.lr = 0x82EF8A20;
	sub_82EF7750(ctx, base);
	// 82EF8A20: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF8A24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8A28: 48041F91  bl 0x82f3a9b8
	ctx.lr = 0x82EF8A2C;
	sub_82F3A9B8(ctx, base);
	// 82EF8A2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8A30: 4182FFCC  beq 0x82ef89fc
	if ctx.cr[0].eq {
	pc = 0x82EF89FC; continue 'dispatch;
	}
	pc = 0x82EF8A34; continue 'dispatch;
            }
            0x82EF8A34 => {
    //   block [0x82EF8A34..0x82EF8A40)
	// 82EF8A34: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8A38: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EF8A3C: 48074BBD  bl 0x82f6d5f8
	ctx.lr = 0x82EF8A40;
	sub_82F6D5F8(ctx, base);
	pc = 0x82EF8A40; continue 'dispatch;
            }
            0x82EF8A40 => {
    //   block [0x82EF8A40..0x82EF8A50)
	// 82EF8A40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8A44: 483C0F11  bl 0x832b9954
	ctx.lr = 0x82EF8A48;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8A4C: 4BDB0A10  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8A50 size=252
    let mut pc: u32 = 0x82EF8A50;
    'dispatch: loop {
        match pc {
            0x82EF8A50 => {
    //   block [0x82EF8A50..0x82EF8AB4)
	// 82EF8A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8A54: 4BDB09B5  bl 0x82ca9408
	ctx.lr = 0x82EF8A58;
	sub_82CA93D0(ctx, base);
	// 82EF8A58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8A60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8A64: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 82EF8A68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8A6C: 483C0EF9  bl 0x832b9964
	ctx.lr = 0x82EF8A70;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF8A70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8A74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8A78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF8A7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8A80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8A84: 4E800421  bctrl
	ctx.lr = 0x82EF8A88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8A88: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82EF8A8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF8A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8A94: 4BFFEE7D  bl 0x82ef7910
	ctx.lr = 0x82EF8A98;
	sub_82EF7910(ctx, base);
	// 82EF8A98: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF8A9C: 41800018  blt 0x82ef8ab4
	if ctx.cr[0].lt {
	pc = 0x82EF8AB4; continue 'dispatch;
	}
	// 82EF8AA0: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82EF8AA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AA8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8AAC: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF8AB0: 48000008  b 0x82ef8ab8
	pc = 0x82EF8AB8; continue 'dispatch;
            }
            0x82EF8AB4 => {
    //   block [0x82EF8AB4..0x82EF8AB8)
	// 82EF8AB4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82EF8AB8; continue 'dispatch;
            }
            0x82EF8AB8 => {
    //   block [0x82EF8AB8..0x82EF8AD8)
	// 82EF8AB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8ABC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF8AC0: 419A0018  beq cr6, 0x82ef8ad8
	if ctx.cr[6].eq {
	pc = 0x82EF8AD8; continue 'dispatch;
	}
	// 82EF8AC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AC8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF8ACC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8AD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8AD4: 4E800421  bctrl
	ctx.lr = 0x82EF8AD8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF8AD8 => {
    //   block [0x82EF8AD8..0x82EF8B3C)
	// 82EF8AD8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EF8ADC: 419A0060  beq cr6, 0x82ef8b3c
	if ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8AE0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8AE8: 409A0054  bne cr6, 0x82ef8b3c
	if !ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8AEC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8AF0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF8AF4: 409A0048  bne cr6, 0x82ef8b3c
	if !ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8AF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF8B00: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF8B04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8B08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8B0C: 4E800421  bctrl
	ctx.lr = 0x82EF8B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8B10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF8B14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8B18: 4BFFECA9  bl 0x82ef77c0
	ctx.lr = 0x82EF8B1C;
	sub_82EF77C0(ctx, base);
	// 82EF8B1C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF8B20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF8B24: 419A0018  beq cr6, 0x82ef8b3c
	if ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8B28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8B2C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF8B30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8B38: 4E800421  bctrl
	ctx.lr = 0x82EF8B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF8B3C => {
    //   block [0x82EF8B3C..0x82EF8B4C)
	// 82EF8B3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8B40: 483C0E15  bl 0x832b9954
	ctx.lr = 0x82EF8B44;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8B44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8B48: 4BDB0910  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8B50 size=140
    let mut pc: u32 = 0x82EF8B50;
    'dispatch: loop {
        match pc {
            0x82EF8B50 => {
    //   block [0x82EF8B50..0x82EF8BBC)
	// 82EF8B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8B64: 48004DE5  bl 0x82efd948
	ctx.lr = 0x82EF8B68;
	sub_82EFD948(ctx, base);
	// 82EF8B68: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF8B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF8B70: 396BCFB0  addi r11, r11, -0x3050
	ctx.r[11].s64 = ctx.r[11].s64 + -12368;
	// 82EF8B74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF8B78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8B80: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF8B84: 48004ED5  bl 0x82efda58
	ctx.lr = 0x82EF8B88;
	sub_82EFDA58(ctx, base);
	// 82EF8B88: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82EF8B8C: 48004CD5  bl 0x82efd860
	ctx.lr = 0x82EF8B90;
	sub_82EFD860(ctx, base);
	// 82EF8B90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8B94: 41820028  beq 0x82ef8bbc
	if ctx.cr[0].eq {
	pc = 0x82EF8BBC; continue 'dispatch;
	}
	// 82EF8B98: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EF8B9C: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EF8BA0: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EF8BA4: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EF8BA8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF8BAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF8BB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF8BB4: 4BFFF96D  bl 0x82ef8520
	ctx.lr = 0x82EF8BB8;
	sub_82EF8520(ctx, base);
	// 82EF8BB8: 48000008  b 0x82ef8bc0
	pc = 0x82EF8BC0; continue 'dispatch;
            }
            0x82EF8BBC => {
    //   block [0x82EF8BBC..0x82EF8BC0)
	// 82EF8BBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF8BC0; continue 'dispatch;
            }
            0x82EF8BC0 => {
    //   block [0x82EF8BC0..0x82EF8BDC)
	// 82EF8BC0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82EF8BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8BD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8BE0 size=108
    let mut pc: u32 = 0x82EF8BE0;
    'dispatch: loop {
        match pc {
            0x82EF8BE0 => {
    //   block [0x82EF8BE0..0x82EF8C28)
	// 82EF8BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8BEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8BF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8BF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF8BF8: 396BCFB0  addi r11, r11, -0x3050
	ctx.r[11].s64 = ctx.r[11].s64 + -12368;
	// 82EF8BFC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8C00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8C04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF8C08: 419A0020  beq cr6, 0x82ef8c28
	if ctx.cr[6].eq {
	pc = 0x82EF8C28; continue 'dispatch;
	}
	// 82EF8C0C: 4BFFFD55  bl 0x82ef8960
	ctx.lr = 0x82EF8C10;
	sub_82EF8960(ctx, base);
	// 82EF8C10: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8C14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF8C18: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8C1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8C24: 4E800421  bctrl
	ctx.lr = 0x82EF8C28;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EF8C28 => {
    //   block [0x82EF8C28..0x82EF8C4C)
	// 82EF8C28: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82EF8C2C: 480749CD  bl 0x82f6d5f8
	ctx.lr = 0x82EF8C30;
	sub_82F6D5F8(ctx, base);
	// 82EF8C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8C34: 48004DE5  bl 0x82efda18
	ctx.lr = 0x82EF8C38;
	sub_82EFDA18(ctx, base);
	// 82EF8C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8C50 size=76
    let mut pc: u32 = 0x82EF8C50;
    'dispatch: loop {
        match pc {
            0x82EF8C50 => {
    //   block [0x82EF8C50..0x82EF8C80)
	// 82EF8C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8C68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8C6C: 4BFFF6CD  bl 0x82ef8338
	ctx.lr = 0x82EF8C70;
	sub_82EF8338(ctx, base);
	// 82EF8C70: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8C74: 4182000C  beq 0x82ef8c80
	if ctx.cr[0].eq {
	pc = 0x82EF8C80; continue 'dispatch;
	}
	// 82EF8C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8C7C: 48004C25  bl 0x82efd8a0
	ctx.lr = 0x82EF8C80;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF8C80; continue 'dispatch;
            }
            0x82EF8C80 => {
    //   block [0x82EF8C80..0x82EF8C9C)
	// 82EF8C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8C90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8CA0 size=76
    let mut pc: u32 = 0x82EF8CA0;
    'dispatch: loop {
        match pc {
            0x82EF8CA0 => {
    //   block [0x82EF8CA0..0x82EF8CD0)
	// 82EF8CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8CB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8CB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8CBC: 4BFFF8D5  bl 0x82ef8590
	ctx.lr = 0x82EF8CC0;
	sub_82EF8590(ctx, base);
	// 82EF8CC0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8CC4: 4182000C  beq 0x82ef8cd0
	if ctx.cr[0].eq {
	pc = 0x82EF8CD0; continue 'dispatch;
	}
	// 82EF8CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8CCC: 48004BD5  bl 0x82efd8a0
	ctx.lr = 0x82EF8CD0;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF8CD0; continue 'dispatch;
            }
            0x82EF8CD0 => {
    //   block [0x82EF8CD0..0x82EF8CEC)
	// 82EF8CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8CD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8CE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8CF0 size=76
    let mut pc: u32 = 0x82EF8CF0;
    'dispatch: loop {
        match pc {
            0x82EF8CF0 => {
    //   block [0x82EF8CF0..0x82EF8D20)
	// 82EF8CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8D04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8D08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8D0C: 4BFFFED5  bl 0x82ef8be0
	ctx.lr = 0x82EF8D10;
	sub_82EF8BE0(ctx, base);
	// 82EF8D10: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8D14: 4182000C  beq 0x82ef8d20
	if ctx.cr[0].eq {
	pc = 0x82EF8D20; continue 'dispatch;
	}
	// 82EF8D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8D1C: 48004B85  bl 0x82efd8a0
	ctx.lr = 0x82EF8D20;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF8D20; continue 'dispatch;
            }
            0x82EF8D20 => {
    //   block [0x82EF8D20..0x82EF8D3C)
	// 82EF8D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8D24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8D30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8D34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8D40 size=48
    let mut pc: u32 = 0x82EF8D40;
    'dispatch: loop {
        match pc {
            0x82EF8D40 => {
    //   block [0x82EF8D40..0x82EF8D70)
	// 82EF8D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8D48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8D4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8D54: 483C0F41  bl 0x832b9c94
	ctx.lr = 0x82EF8D58;
	// extern call 0x832B9C94 → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82EF8D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8D5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8D70 size=116
    let mut pc: u32 = 0x82EF8D70;
    'dispatch: loop {
        match pc {
            0x82EF8D70 => {
    //   block [0x82EF8D70..0x82EF8D9C)
	// 82EF8D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8D74: 4BDB0695  bl 0x82ca9408
	ctx.lr = 0x82EF8D78;
	sub_82CA93D0(ctx, base);
	// 82EF8D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8D7C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF8D80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF8D84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF8D88: 2B1E0020  cmplwi cr6, r30, 0x20
	ctx.cr[6].compare_u32(ctx.r[30].u32, 32 as u32, &mut ctx.xer);
	// 82EF8D8C: 40990010  ble cr6, 0x82ef8d9c
	if !ctx.cr[6].gt {
	pc = 0x82EF8D9C; continue 'dispatch;
	}
	// 82EF8D90: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF8D94: 48004ACD  bl 0x82efd860
	ctx.lr = 0x82EF8D98;
	sub_82EFD860(ctx, base);
	// 82EF8D98: 48000008  b 0x82ef8da0
	pc = 0x82EF8DA0; continue 'dispatch;
            }
            0x82EF8D9C => {
    //   block [0x82EF8D9C..0x82EF8DA0)
	// 82EF8D9C: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	pc = 0x82EF8DA0; continue 'dispatch;
            }
            0x82EF8DA0 => {
    //   block [0x82EF8DA0..0x82EF8DB0)
	// 82EF8DA0: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF8DA4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8DA8: 419A0030  beq cr6, 0x82ef8dd8
	if ctx.cr[6].eq {
	pc = 0x82EF8DD8; continue 'dispatch;
	}
	// 82EF8DAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82EF8DB0; continue 'dispatch;
            }
            0x82EF8DB0 => {
    //   block [0x82EF8DB0..0x82EF8DD8)
	// 82EF8DB0: 7C7FE02E  lwzx r3, r31, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EF8DB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8DB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8DC0: 4E800421  bctrl
	ctx.lr = 0x82EF8DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8DC4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8DC8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF8DCC: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82EF8DD0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EF8DD4: 4082FFDC  bne 0x82ef8db0
	if !ctx.cr[0].eq {
	pc = 0x82EF8DB0; continue 'dispatch;
	}
            }
            0x82EF8DD8 => {
    //   block [0x82EF8DD8..0x82EF8DE4)
	// 82EF8DD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8DE0: 4BDB0678  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8DE8 size=204
    let mut pc: u32 = 0x82EF8DE8;
    'dispatch: loop {
        match pc {
            0x82EF8DE8 => {
    //   block [0x82EF8DE8..0x82EF8E0C)
	// 82EF8DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8DEC: 4BDB061D  bl 0x82ca9408
	ctx.lr = 0x82EF8DF0;
	sub_82CA93D0(ctx, base);
	// 82EF8DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8DF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8DF8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF8DFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8E00: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8E04: 419A0034  beq cr6, 0x82ef8e38
	if ctx.cr[6].eq {
	pc = 0x82EF8E38; continue 'dispatch;
	}
	// 82EF8E08: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	pc = 0x82EF8E0C; continue 'dispatch;
            }
            0x82EF8E0C => {
    //   block [0x82EF8E0C..0x82EF8E38)
	// 82EF8E0C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8E18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8E1C: 4E800421  bctrl
	ctx.lr = 0x82EF8E20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8E20: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8E24: 41820014  beq 0x82ef8e38
	if ctx.cr[0].eq {
	pc = 0x82EF8E38; continue 'dispatch;
	}
	// 82EF8E28: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF8E2C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF8E30: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF8E34: 4198FFD8  blt cr6, 0x82ef8e0c
	if ctx.cr[6].lt {
	pc = 0x82EF8E0C; continue 'dispatch;
	}
            }
            0x82EF8E38 => {
    //   block [0x82EF8E38..0x82EF8E4C)
	// 82EF8E38: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF8E3C: 409A0038  bne cr6, 0x82ef8e74
	if !ctx.cr[6].eq {
	pc = 0x82EF8E74; continue 'dispatch;
	}
	// 82EF8E40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8E44: 419A0028  beq cr6, 0x82ef8e6c
	if ctx.cr[6].eq {
	pc = 0x82EF8E6C; continue 'dispatch;
	}
	// 82EF8E48: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	pc = 0x82EF8E4C; continue 'dispatch;
            }
            0x82EF8E4C => {
    //   block [0x82EF8E4C..0x82EF8E6C)
	// 82EF8E4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8E58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8E5C: 4E800421  bctrl
	ctx.lr = 0x82EF8E60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8E60: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF8E64: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EF8E68: 4082FFE4  bne 0x82ef8e4c
	if !ctx.cr[0].eq {
	pc = 0x82EF8E4C; continue 'dispatch;
	}
            }
            0x82EF8E6C => {
    //   block [0x82EF8E6C..0x82EF8E74)
	// 82EF8E6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF8E70: 4800003C  b 0x82ef8eac
	pc = 0x82EF8EAC; continue 'dispatch;
            }
            0x82EF8E74 => {
    //   block [0x82EF8E74..0x82EF8E84)
	// 82EF8E74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF8E78: 419A0030  beq cr6, 0x82ef8ea8
	if ctx.cr[6].eq {
	pc = 0x82EF8EA8; continue 'dispatch;
	}
	// 82EF8E7C: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8E80: 7FCBE214  add r30, r11, r28
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	pc = 0x82EF8E84; continue 'dispatch;
            }
            0x82EF8E84 => {
    //   block [0x82EF8E84..0x82EF8EA8)
	// 82EF8E84: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82EF8E88: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82EF8E8C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E94: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8E98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8E9C: 4E800421  bctrl
	ctx.lr = 0x82EF8EA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8EA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF8EA4: 409AFFE0  bne cr6, 0x82ef8e84
	if !ctx.cr[6].eq {
	pc = 0x82EF8E84; continue 'dispatch;
	}
            }
            0x82EF8EA8 => {
    //   block [0x82EF8EA8..0x82EF8EAC)
	// 82EF8EA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF8EAC; continue 'dispatch;
            }
            0x82EF8EAC => {
    //   block [0x82EF8EAC..0x82EF8EB4)
	// 82EF8EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8EB0: 4BDB05A8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8EB8 size=136
    let mut pc: u32 = 0x82EF8EB8;
    'dispatch: loop {
        match pc {
            0x82EF8EB8 => {
    //   block [0x82EF8EB8..0x82EF8ED8)
	// 82EF8EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8EBC: 4BDB054D  bl 0x82ca9408
	ctx.lr = 0x82EF8EC0;
	sub_82CA93D0(ctx, base);
	// 82EF8EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8EC4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF8EC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8ECC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF8ED0: 419A0048  beq cr6, 0x82ef8f18
	if ctx.cr[6].eq {
	pc = 0x82EF8F18; continue 'dispatch;
	}
	// 82EF8ED4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	pc = 0x82EF8ED8; continue 'dispatch;
            }
            0x82EF8ED8 => {
    //   block [0x82EF8ED8..0x82EF8F18)
	// 82EF8ED8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8EDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8EE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8EE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8EE8: 4E800421  bctrl
	ctx.lr = 0x82EF8EEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8EEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF8EF0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8EF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8EFC: 4E800421  bctrl
	ctx.lr = 0x82EF8F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8F00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8F04: 40820020  bne 0x82ef8f24
	if !ctx.cr[0].eq {
	pc = 0x82EF8F24; continue 'dispatch;
	}
	// 82EF8F08: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF8F0C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EF8F10: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF8F14: 4198FFC4  blt cr6, 0x82ef8ed8
	if ctx.cr[6].lt {
	pc = 0x82EF8ED8; continue 'dispatch;
	}
            }
            0x82EF8F18 => {
    //   block [0x82EF8F18..0x82EF8F1C)
	// 82EF8F18: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x82EF8F1C; continue 'dispatch;
            }
            0x82EF8F1C => {
    //   block [0x82EF8F1C..0x82EF8F24)
	// 82EF8F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8F20: 4BDB0538  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82EF8F24 => {
    //   block [0x82EF8F24..0x82EF8F40)
	// 82EF8F24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8F28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8F2C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8F34: 4E800421  bctrl
	ctx.lr = 0x82EF8F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8F3C: 4BFFFFE0  b 0x82ef8f1c
	pc = 0x82EF8F1C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8F40 size=160
    let mut pc: u32 = 0x82EF8F40;
    'dispatch: loop {
        match pc {
            0x82EF8F40 => {
    //   block [0x82EF8F40..0x82EF8F90)
	// 82EF8F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8F44: 4BDB04C9  bl 0x82ca940c
	ctx.lr = 0x82EF8F48;
	sub_82CA93D0(ctx, base);
	// 82EF8F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8F4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8F50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8F54: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82EF8F58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8F5C: 48001BFD  bl 0x82efab58
	ctx.lr = 0x82EF8F60;
	sub_82EFAB58(ctx, base);
	// 82EF8F60: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8F64: 419A004C  beq cr6, 0x82ef8fb0
	if ctx.cr[6].eq {
	pc = 0x82EF8FB0; continue 'dispatch;
	}
	// 82EF8F68: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8F6C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EF8F70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8F74: 4082003C  bne 0x82ef8fb0
	if !ctx.cr[0].eq {
	pc = 0x82EF8FB0; continue 'dispatch;
	}
	// 82EF8F78: 419A0018  beq cr6, 0x82ef8f90
	if ctx.cr[6].eq {
	pc = 0x82EF8F90; continue 'dispatch;
	}
	// 82EF8F7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF8F80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF8F84: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF8F88: 48002079  bl 0x82efb000
	ctx.lr = 0x82EF8F8C;
	sub_82EFB000(ctx, base);
	// 82EF8F8C: 48000024  b 0x82ef8fb0
	pc = 0x82EF8FB0; continue 'dispatch;
            }
            0x82EF8F90 => {
    //   block [0x82EF8F90..0x82EF8F94)
	// 82EF8F90: 3BDF0038  addi r30, r31, 0x38
	ctx.r[30].s64 = ctx.r[31].s64 + 56;
	pc = 0x82EF8F94; continue 'dispatch;
            }
            0x82EF8F94 => {
    //   block [0x82EF8F94..0x82EF8FB0)
	// 82EF8F94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF8F98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF8F9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8FA0: 48002061  bl 0x82efb000
	ctx.lr = 0x82EF8FA4;
	sub_82EFB000(ctx, base);
	// 82EF8FA4: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8FA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8FAC: 4182FFE8  beq 0x82ef8f94
	if ctx.cr[0].eq {
	pc = 0x82EF8F94; continue 'dispatch;
	}
	pc = 0x82EF8FB0; continue 'dispatch;
            }
            0x82EF8FB0 => {
    //   block [0x82EF8FB0..0x82EF8FCC)
	// 82EF8FB0: 8BDF0018  lbz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8FB4: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82EF8FB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8FBC: 41820010  beq 0x82ef8fcc
	if ctx.cr[0].eq {
	pc = 0x82EF8FCC; continue 'dispatch;
	}
	// 82EF8FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF8FC4: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF8FC8: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	pc = 0x82EF8FCC; continue 'dispatch;
            }
            0x82EF8FCC => {
    //   block [0x82EF8FCC..0x82EF8FE0)
	// 82EF8FCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8FD0: 48002AD1  bl 0x82efbaa0
	ctx.lr = 0x82EF8FD4;
	sub_82EFBAA0(ctx, base);
	// 82EF8FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8FDC: 4BDB0480  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8FE0 size=80
    let mut pc: u32 = 0x82EF8FE0;
    'dispatch: loop {
        match pc {
            0x82EF8FE0 => {
    //   block [0x82EF8FE0..0x82EF9030)
	// 82EF8FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8FF8: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82EF8FFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9000: 48001B59  bl 0x82efab58
	ctx.lr = 0x82EF9004;
	sub_82EFAB58(ctx, base);
	// 82EF9004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF900C: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EF9010: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF9014: 48002A8D  bl 0x82efbaa0
	ctx.lr = 0x82EF9018;
	sub_82EFBAA0(ctx, base);
	// 82EF9018: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF901C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9024: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF902C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9030 size=8
    let mut pc: u32 = 0x82EF9030;
    'dispatch: loop {
        match pc {
            0x82EF9030 => {
    //   block [0x82EF9030..0x82EF9038)
	// 82EF9030: 88630018  lbz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9038 size=20
    let mut pc: u32 = 0x82EF9038;
    'dispatch: loop {
        match pc {
            0x82EF9038 => {
    //   block [0x82EF9038..0x82EF904C)
	// 82EF9038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF903C: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82EF9040: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82EF9044: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF9048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9050 size=20
    let mut pc: u32 = 0x82EF9050;
    'dispatch: loop {
        match pc {
            0x82EF9050 => {
    //   block [0x82EF9050..0x82EF9064)
	// 82EF9050: 8163FFEC  lwz r11, -0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82EF9054: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF9058: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF905C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF9060: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9068 size=96
    let mut pc: u32 = 0x82EF9068;
    'dispatch: loop {
        match pc {
            0x82EF9068 => {
    //   block [0x82EF9068..0x82EF90A4)
	// 82EF9068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF907C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9080: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82EF9084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9088: 48001AD1  bl 0x82efab58
	ctx.lr = 0x82EF908C;
	sub_82EFAB58(ctx, base);
	// 82EF908C: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82EF9090: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF9094: 41820010  beq 0x82ef90a4
	if ctx.cr[0].eq {
	pc = 0x82EF90A4; continue 'dispatch;
	}
	// 82EF9098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF909C: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82EF90A0: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	pc = 0x82EF90A4; continue 'dispatch;
            }
            0x82EF90A4 => {
    //   block [0x82EF90A4..0x82EF90C8)
	// 82EF90A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF90A8: 480029F9  bl 0x82efbaa0
	ctx.lr = 0x82EF90AC;
	sub_82EFBAA0(ctx, base);
	// 82EF90AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF90B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF90B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF90B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF90BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF90C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF90C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF90C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF90C8 size=288
    let mut pc: u32 = 0x82EF90C8;
    'dispatch: loop {
        match pc {
            0x82EF90C8 => {
    //   block [0x82EF90C8..0x82EF90F0)
	// 82EF90C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF90CC: 4BDB0335  bl 0x82ca9400
	ctx.lr = 0x82EF90D0;
	sub_82CA93D0(ctx, base);
	// 82EF90D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF90D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF90D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF90DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF90E0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF90E4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF90E8: 40990010  ble cr6, 0x82ef90f8
	if !ctx.cr[6].gt {
	pc = 0x82EF90F8; continue 'dispatch;
	}
	// 82EF90EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF90F0; continue 'dispatch;
            }
            0x82EF90F0 => {
    //   block [0x82EF90F0..0x82EF90F8)
	// 82EF90F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF90F4: 4BDB035C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x82EF90F8 => {
    //   block [0x82EF90F8..0x82EF9118)
	// 82EF90F8: 3B7F0020  addi r27, r31, 0x20
	ctx.r[27].s64 = ctx.r[31].s64 + 32;
	// 82EF90FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF9100: 48001A59  bl 0x82efab58
	ctx.lr = 0x82EF9104;
	sub_82EFAB58(ctx, base);
	// 82EF9104: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9108: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF910C: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82EF9110: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF9114: 41990024  bgt cr6, 0x82ef9138
	if ctx.cr[6].gt {
	pc = 0x82EF9138; continue 'dispatch;
	}
	pc = 0x82EF9118; continue 'dispatch;
            }
            0x82EF9118 => {
    //   block [0x82EF9118..0x82EF9128)
	// 82EF9118: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF911C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82EF9120: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EF9124: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF9128; continue 'dispatch;
            }
            0x82EF9128 => {
    //   block [0x82EF9128..0x82EF9138)
	// 82EF9128: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF912C: 48002975  bl 0x82efbaa0
	ctx.lr = 0x82EF9130;
	sub_82EFBAA0(ctx, base);
	// 82EF9130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9134: 4BFFFFBC  b 0x82ef90f0
	pc = 0x82EF90F0; continue 'dispatch;
            }
            0x82EF9138 => {
    //   block [0x82EF9138..0x82EF9140)
	// 82EF9138: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF913C: 409A000C  bne cr6, 0x82ef9148
	if !ctx.cr[6].eq {
	pc = 0x82EF9148; continue 'dispatch;
	}
	pc = 0x82EF9140; continue 'dispatch;
            }
            0x82EF9140 => {
    //   block [0x82EF9140..0x82EF9148)
	// 82EF9140: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF9144: 4BFFFFE4  b 0x82ef9128
	pc = 0x82EF9128; continue 'dispatch;
            }
            0x82EF9148 => {
    //   block [0x82EF9148..0x82EF9164)
	// 82EF9148: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EF914C: 409A0040  bne cr6, 0x82ef918c
	if !ctx.cr[6].eq {
	pc = 0x82EF918C; continue 'dispatch;
	}
	// 82EF9150: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9154: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82EF9158: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF915C: 4099FFBC  ble cr6, 0x82ef9118
	if !ctx.cr[6].gt {
	pc = 0x82EF9118; continue 'dispatch;
	}
	// 82EF9160: 3BDF003C  addi r30, r31, 0x3c
	ctx.r[30].s64 = ctx.r[31].s64 + 60;
	pc = 0x82EF9164; continue 'dispatch;
            }
            0x82EF9164 => {
    //   block [0x82EF9164..0x82EF918C)
	// 82EF9164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF9168: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF916C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9170: 48001E91  bl 0x82efb000
	ctx.lr = 0x82EF9174;
	sub_82EFB000(ctx, base);
	// 82EF9174: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9178: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF917C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EF9180: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9184: 4199FFE0  bgt cr6, 0x82ef9164
	if ctx.cr[6].gt {
	pc = 0x82EF9164; continue 'dispatch;
	}
	// 82EF9188: 4BFFFF90  b 0x82ef9118
	pc = 0x82EF9118; continue 'dispatch;
            }
            0x82EF918C => {
    //   block [0x82EF918C..0x82EF91A0)
	// 82EF918C: 480063E5  bl 0x82eff570
	ctx.lr = 0x82EF9190;
	sub_82EFF570(ctx, base);
	// 82EF9190: 3BBF003C  addi r29, r31, 0x3c
	ctx.r[29].s64 = ctx.r[31].s64 + 60;
	// 82EF9194: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EF9198: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF919C: 48000034  b 0x82ef91d0
	pc = 0x82EF91D0; continue 'dispatch;
            }
            0x82EF91A0 => {
    //   block [0x82EF91A0..0x82EF91D0)
	// 82EF91A0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF91A4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF91A8: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EF91AC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF91B0: 4099FF68  ble cr6, 0x82ef9118
	if !ctx.cr[6].gt {
	pc = 0x82EF9118; continue 'dispatch;
	}
	// 82EF91B4: 480063BD  bl 0x82eff570
	ctx.lr = 0x82EF91B8;
	sub_82EFF570(ctx, base);
	// 82EF91B8: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF91BC: 574A003E  slwi r10, r26, 0
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF91C0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF91C4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF91C8: 4098FF78  bge cr6, 0x82ef9140
	if !ctx.cr[6].lt {
	pc = 0x82EF9140; continue 'dispatch;
	}
	// 82EF91CC: 7CABF050  subf r5, r11, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	pc = 0x82EF91D0; continue 'dispatch;
            }
            0x82EF91D0 => {
    //   block [0x82EF91D0..0x82EF91E8)
	// 82EF91D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF91D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF91D8: 48001E29  bl 0x82efb000
	ctx.lr = 0x82EF91DC;
	sub_82EFB000(ctx, base);
	// 82EF91DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF91E0: 4082FFC0  bne 0x82ef91a0
	if !ctx.cr[0].eq {
	pc = 0x82EF91A0; continue 'dispatch;
	}
	// 82EF91E4: 4BFFFF5C  b 0x82ef9140
	pc = 0x82EF9140; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF91E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF91E8 size=116
    let mut pc: u32 = 0x82EF91E8;
    'dispatch: loop {
        match pc {
            0x82EF91E8 => {
    //   block [0x82EF91E8..0x82EF9218)
	// 82EF91E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF91EC: 4BDB0221  bl 0x82ca940c
	ctx.lr = 0x82EF91F0;
	sub_82CA93D0(ctx, base);
	// 82EF91F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF91F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF91F8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82EF91FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9200: 48001959  bl 0x82efab58
	ctx.lr = 0x82EF9204;
	sub_82EFAB58(ctx, base);
	// 82EF9204: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9208: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF920C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9210: 41980028  blt cr6, 0x82ef9238
	if ctx.cr[6].lt {
	pc = 0x82EF9238; continue 'dispatch;
	}
	// 82EF9214: 3BBF003C  addi r29, r31, 0x3c
	ctx.r[29].s64 = ctx.r[31].s64 + 60;
	pc = 0x82EF9218; continue 'dispatch;
            }
            0x82EF9218 => {
    //   block [0x82EF9218..0x82EF9238)
	// 82EF9218: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF921C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF9220: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9224: 48001DDD  bl 0x82efb000
	ctx.lr = 0x82EF9228;
	sub_82EFB000(ctx, base);
	// 82EF9228: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF922C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9230: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9234: 4098FFE4  bge cr6, 0x82ef9218
	if !ctx.cr[6].lt {
	pc = 0x82EF9218; continue 'dispatch;
	}
	pc = 0x82EF9238; continue 'dispatch;
            }
            0x82EF9238 => {
    //   block [0x82EF9238..0x82EF925C)
	// 82EF9238: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF923C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9240: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF9244: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9248: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF924C: 48002855  bl 0x82efbaa0
	ctx.lr = 0x82EF9250;
	sub_82EFBAA0(ctx, base);
	// 82EF9250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9258: 4BDB0204  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9260 size=128
    let mut pc: u32 = 0x82EF9260;
    'dispatch: loop {
        match pc {
            0x82EF9260 => {
    //   block [0x82EF9260..0x82EF9298)
	// 82EF9260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9264: 4BDB01A5  bl 0x82ca9408
	ctx.lr = 0x82EF9268;
	sub_82CA93D0(ctx, base);
	// 82EF9268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF926C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9270: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9274: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82EF9278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF927C: 480018DD  bl 0x82efab58
	ctx.lr = 0x82EF9280;
	sub_82EFAB58(ctx, base);
	// 82EF9280: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9284: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9288: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF928C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9290: 4099002C  ble cr6, 0x82ef92bc
	if !ctx.cr[6].gt {
	pc = 0x82EF92BC; continue 'dispatch;
	}
	// 82EF9294: 3B9F003C  addi r28, r31, 0x3c
	ctx.r[28].s64 = ctx.r[31].s64 + 60;
	pc = 0x82EF9298; continue 'dispatch;
            }
            0x82EF9298 => {
    //   block [0x82EF9298..0x82EF92BC)
	// 82EF9298: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF929C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF92A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF92A4: 48001D5D  bl 0x82efb000
	ctx.lr = 0x82EF92A8;
	sub_82EFB000(ctx, base);
	// 82EF92A8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF92AC: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF92B0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF92B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF92B8: 4199FFE0  bgt cr6, 0x82ef9298
	if ctx.cr[6].gt {
	pc = 0x82EF9298; continue 'dispatch;
	}
	pc = 0x82EF92BC; continue 'dispatch;
            }
            0x82EF92BC => {
    //   block [0x82EF92BC..0x82EF92E0)
	// 82EF92BC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF92C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF92C4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF92C8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF92CC: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF92D0: 480027D1  bl 0x82efbaa0
	ctx.lr = 0x82EF92D4;
	sub_82EFBAA0(ctx, base);
	// 82EF92D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF92D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF92DC: 4BDB017C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF92E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF92E0 size=32
    let mut pc: u32 = 0x82EF92E0;
    'dispatch: loop {
        match pc {
            0x82EF92E0 => {
    //   block [0x82EF92E0..0x82EF92F8)
	// 82EF92E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF92E4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF92E8: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF92EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF92F0: 41810008  bgt 0x82ef92f8
	if ctx.cr[0].gt {
	pc = 0x82EF92F8; continue 'dispatch;
	}
	// 82EF92F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EF92F8; continue 'dispatch;
            }
            0x82EF92F8 => {
    //   block [0x82EF92F8..0x82EF9300)
	// 82EF92F8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF92FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9300 size=16
    let mut pc: u32 = 0x82EF9300;
    'dispatch: loop {
        match pc {
            0x82EF9300 => {
    //   block [0x82EF9300..0x82EF9310)
	// 82EF9300: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF9304: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9308: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF930C: 4BFFFDBC  b 0x82ef90c8
	sub_82EF90C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9310 size=32
    let mut pc: u32 = 0x82EF9310;
    'dispatch: loop {
        match pc {
            0x82EF9310 => {
    //   block [0x82EF9310..0x82EF9328)
	// 82EF9310: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9314: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9318: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF931C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF9320: 41810008  bgt 0x82ef9328
	if ctx.cr[0].gt {
	pc = 0x82EF9328; continue 'dispatch;
	}
	// 82EF9324: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EF9328; continue 'dispatch;
            }
            0x82EF9328 => {
    //   block [0x82EF9328..0x82EF9330)
	// 82EF9328: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF932C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9330 size=20
    let mut pc: u32 = 0x82EF9330;
    'dispatch: loop {
        match pc {
            0x82EF9330 => {
    //   block [0x82EF9330..0x82EF9344)
	// 82EF9330: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9338: 409A000C  bne cr6, 0x82ef9344
	if !ctx.cr[6].eq {
		sub_82EF9344(ctx, base);
		return;
	}
	// 82EF933C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF9340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9344 size=40
    let mut pc: u32 = 0x82EF9344;
    'dispatch: loop {
        match pc {
            0x82EF9344 => {
    //   block [0x82EF9344..0x82EF9364)
	// 82EF9344: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9348: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF934C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9350: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF9354: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF9358: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF935C: 40980008  bge cr6, 0x82ef9364
	if !ctx.cr[6].lt {
	pc = 0x82EF9364; continue 'dispatch;
	}
	// 82EF9360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EF9364; continue 'dispatch;
            }
            0x82EF9364 => {
    //   block [0x82EF9364..0x82EF936C)
	// 82EF9364: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF9368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9370 size=20
    let mut pc: u32 = 0x82EF9370;
    'dispatch: loop {
        match pc {
            0x82EF9370 => {
    //   block [0x82EF9370..0x82EF9384)
	// 82EF9370: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EF9374: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF937C: 409A0008  bne cr6, 0x82ef9384
	if !ctx.cr[6].eq {
		sub_82EF9384(ctx, base);
		return;
	}
	// 82EF9380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9384(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9384 size=12
    let mut pc: u32 = 0x82EF9384;
    'dispatch: loop {
        match pc {
            0x82EF9384 => {
    //   block [0x82EF9384..0x82EF9390)
	// 82EF9384: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9388: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF938C: 4BFFFD3C  b 0x82ef90c8
	sub_82EF90C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9390 size=20
    let mut pc: u32 = 0x82EF9390;
    'dispatch: loop {
        match pc {
            0x82EF9390 => {
    //   block [0x82EF9390..0x82EF93A4)
	// 82EF9390: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9394: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF9398: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EF939C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82EF93A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF93A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF93A8 size=20
    let mut pc: u32 = 0x82EF93A8;
    'dispatch: loop {
        match pc {
            0x82EF93A8 => {
    //   block [0x82EF93A8..0x82EF93BC)
	// 82EF93A8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF93AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF93B0: 409A000C  bne cr6, 0x82ef93bc
	if !ctx.cr[6].eq {
		sub_82EF93BC(ctx, base);
		return;
	}
	// 82EF93B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF93B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF93BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF93BC size=40
    let mut pc: u32 = 0x82EF93BC;
    'dispatch: loop {
        match pc {
            0x82EF93BC => {
    //   block [0x82EF93BC..0x82EF93DC)
	// 82EF93BC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF93C0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF93C4: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF93C8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF93CC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF93D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF93D4: 40980008  bge cr6, 0x82ef93dc
	if !ctx.cr[6].lt {
	pc = 0x82EF93DC; continue 'dispatch;
	}
	// 82EF93D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EF93DC; continue 'dispatch;
            }
            0x82EF93DC => {
    //   block [0x82EF93DC..0x82EF93E4)
	// 82EF93DC: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF93E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF93E8 size=96
    let mut pc: u32 = 0x82EF93E8;
    'dispatch: loop {
        match pc {
            0x82EF93E8 => {
    //   block [0x82EF93E8..0x82EF9430)
	// 82EF93E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF93EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF93F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF93F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF93F8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82EF93FC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF9400: 3BEB8FB0  addi r31, r11, -0x7050
	ctx.r[31].s64 = ctx.r[11].s64 + -28752;
	// 82EF9404: 816A8FB4  lwz r11, -0x704c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28748 as u32) ) } as u64;
	// 82EF9408: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF940C: 40820024  bne 0x82ef9430
	if !ctx.cr[0].eq {
	pc = 0x82EF9430; continue 'dispatch;
	}
	// 82EF9410: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EF9414: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82EF9418: 3929C558  addi r9, r9, -0x3aa8
	ctx.r[9].s64 = ctx.r[9].s64 + -15016;
	// 82EF941C: 916A8FB4  stw r11, -0x704c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28748 as u32), ctx.r[11].u32 ) };
	// 82EF9420: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82EF9424: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF9428: 386B8F98  addi r3, r11, -0x7068
	ctx.r[3].s64 = ctx.r[11].s64 + -28776;
	// 82EF942C: 4BDB0AF5  bl 0x82ca9f20
	ctx.lr = 0x82EF9430;
	sub_82CA9F20(ctx, base);
	pc = 0x82EF9430; continue 'dispatch;
            }
            0x82EF9430 => {
    //   block [0x82EF9430..0x82EF9448)
	// 82EF9430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF943C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9448 size=196
    let mut pc: u32 = 0x82EF9448;
    'dispatch: loop {
        match pc {
            0x82EF9448 => {
    //   block [0x82EF9448..0x82EF9480)
	// 82EF9448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF945C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9460: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9464: 556A17BF  rlwinm. r10, r11, 2, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF9468: 41820028  beq 0x82ef9490
	if ctx.cr[0].eq {
	pc = 0x82EF9490; continue 'dispatch;
	}
	// 82EF946C: 55690FFF  rlwinm. r9, r11, 1, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF9470: 41820010  beq 0x82ef9480
	if ctx.cr[0].eq {
	pc = 0x82EF9480; continue 'dispatch;
	}
	// 82EF9474: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF9478: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF947C: 41980078  blt cr6, 0x82ef94f4
	if ctx.cr[6].lt {
	pc = 0x82EF94F4; continue 'dispatch;
	}
	pc = 0x82EF9480; continue 'dispatch;
            }
            0x82EF9480 => {
    //   block [0x82EF9480..0x82EF9490)
	// 82EF9480: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF9484: 4182000C  beq 0x82ef9490
	if ctx.cr[0].eq {
	pc = 0x82EF9490; continue 'dispatch;
	}
	// 82EF9488: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF948C: 419A0068  beq cr6, 0x82ef94f4
	if ctx.cr[6].eq {
	pc = 0x82EF94F4; continue 'dispatch;
	}
	pc = 0x82EF9490; continue 'dispatch;
            }
            0x82EF9490 => {
    //   block [0x82EF9490..0x82EF94B4)
	// 82EF9490: 556B0002  rlwinm r11, r11, 0, 0, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF9494: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9498: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF949C: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82EF94A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF94A4: 409A001C  bne cr6, 0x82ef94c0
	if !ctx.cr[6].eq {
	pc = 0x82EF94C0; continue 'dispatch;
	}
	// 82EF94A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF94AC: 419A0008  beq cr6, 0x82ef94b4
	if ctx.cr[6].eq {
	pc = 0x82EF94B4; continue 'dispatch;
	}
	// 82EF94B0: 480043F1  bl 0x82efd8a0
	ctx.lr = 0x82EF94B4;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF94B4; continue 'dispatch;
            }
            0x82EF94B4 => {
    //   block [0x82EF94B4..0x82EF94C0)
	// 82EF94B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF94B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF94BC: 48000038  b 0x82ef94f4
	pc = 0x82EF94F4; continue 'dispatch;
            }
            0x82EF94C0 => {
    //   block [0x82EF94C0..0x82EF94D8)
	// 82EF94C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF94C4: 419A0014  beq cr6, 0x82ef94d8
	if ctx.cr[6].eq {
	pc = 0x82EF94D8; continue 'dispatch;
	}
	// 82EF94C8: 54841838  slwi r4, r4, 3
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF94CC: 480043B5  bl 0x82efd880
	ctx.lr = 0x82EF94D0;
	sub_82EFD880(ctx, base);
	// 82EF94D0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF94D4: 48000020  b 0x82ef94f4
	pc = 0x82EF94F4; continue 'dispatch;
            }
            0x82EF94D8 => {
    //   block [0x82EF94D8..0x82EF94F4)
	// 82EF94D8: 549E1838  slwi r30, r4, 3
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EF94DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF94E0: 48004381  bl 0x82efd860
	ctx.lr = 0x82EF94E4;
	sub_82EFD860(ctx, base);
	// 82EF94E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF94E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF94EC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF94F0: 4BDB04C1  bl 0x82ca99b0
	ctx.lr = 0x82EF94F4;
	sub_82CA99B0(ctx, base);
	pc = 0x82EF94F4; continue 'dispatch;
            }
            0x82EF94F4 => {
    //   block [0x82EF94F4..0x82EF950C)
	// 82EF94F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF94F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF94FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9500: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9510 size=4
    let mut pc: u32 = 0x82EF9510;
    'dispatch: loop {
        match pc {
            0x82EF9510 => {
    //   block [0x82EF9510..0x82EF9514)
	// 82EF9510: 4BFFFED8  b 0x82ef93e8
	sub_82EF93E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9518 size=132
    let mut pc: u32 = 0x82EF9518;
    'dispatch: loop {
        match pc {
            0x82EF9518 => {
    //   block [0x82EF9518..0x82EF9550)
	// 82EF9518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF951C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF952C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9534: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9538: 48039E51  bl 0x82f33388
	ctx.lr = 0x82EF953C;
	sub_82F33388(ctx, base);
	// 82EF953C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF9544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9548: 4099003C  ble cr6, 0x82ef9584
	if !ctx.cr[6].gt {
	pc = 0x82EF9584; continue 'dispatch;
	}
	// 82EF954C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EF9550; continue 'dispatch;
            }
            0x82EF9550 => {
    //   block [0x82EF9550..0x82EF9584)
	// 82EF9550: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9554: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF9558: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF955C: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF9560: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EF9564: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF9568: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF956C: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF9570: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9574: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF9578: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF957C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF9580: 4198FFD0  blt cr6, 0x82ef9550
	if ctx.cr[6].lt {
	pc = 0x82EF9550; continue 'dispatch;
	}
	pc = 0x82EF9584; continue 'dispatch;
            }
            0x82EF9584 => {
    //   block [0x82EF9584..0x82EF959C)
	// 82EF9584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9590: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF95A0 size=128
    let mut pc: u32 = 0x82EF95A0;
    'dispatch: loop {
        match pc {
            0x82EF95A0 => {
    //   block [0x82EF95A0..0x82EF95CC)
	// 82EF95A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF95A4: 4BDAFE61  bl 0x82ca9404
	ctx.lr = 0x82EF95A8;
	sub_82CA93D0(ctx, base);
	// 82EF95A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF95AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF95B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF95B4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF95B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF95BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF95C0: 409A000C  bne cr6, 0x82ef95cc
	if !ctx.cr[6].eq {
	pc = 0x82EF95CC; continue 'dispatch;
	}
	// 82EF95C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF95C8: 48000050  b 0x82ef9618
	pc = 0x82EF9618; continue 'dispatch;
            }
            0x82EF95CC => {
    //   block [0x82EF95CC..0x82EF9618)
	// 82EF95CC: 3BCB0010  addi r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 16;
	// 82EF95D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF95D4: 483C0391  bl 0x832b9964
	ctx.lr = 0x82EF95D8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF95D8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF95DC: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82EF95E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF95E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF95E8: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82EF95EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF95F0: 48039D99  bl 0x82f33388
	ctx.lr = 0x82EF95F4;
	sub_82F33388(ctx, base);
	// 82EF95F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF95F8: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF95FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9600: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF9604: 394BFFF8  addi r10, r11, -8
	ctx.r[10].s64 = ctx.r[11].s64 + -8;
	// 82EF9608: 938BFFF8  stw r28, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[28].u32 ) };
	// 82EF960C: 936BFFFC  stw r27, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[27].u32 ) };
	// 82EF9610: 483C0345  bl 0x832b9954
	ctx.lr = 0x82EF9614;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF9614: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82EF9618; continue 'dispatch;
            }
            0x82EF9618 => {
    //   block [0x82EF9618..0x82EF9620)
	// 82EF9618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF961C: 4BDAFE38  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9620 size=108
    let mut pc: u32 = 0x82EF9620;
    'dispatch: loop {
        match pc {
            0x82EF9620 => {
    //   block [0x82EF9620..0x82EF964C)
	// 82EF9620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF962C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9634: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9638: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82EF963C: 409A0010  bne cr6, 0x82ef964c
	if !ctx.cr[6].eq {
	pc = 0x82EF964C; continue 'dispatch;
	}
	// 82EF9640: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9644: 48039D45  bl 0x82f33388
	ctx.lr = 0x82EF9648;
	sub_82F33388(ctx, base);
	// 82EF9648: 48000030  b 0x82ef9678
	pc = 0x82EF9678; continue 'dispatch;
            }
            0x82EF964C => {
    //   block [0x82EF964C..0x82EF9678)
	// 82EF964C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9650: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF9654: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82EF9658: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF965C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF9660: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 82EF9664: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF9668: 4BDB74C9  bl 0x82cb0b30
	ctx.lr = 0x82EF966C;
	sub_82CB0B30(ctx, base);
	// 82EF966C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9670: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF9674: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF9678; continue 'dispatch;
            }
            0x82EF9678 => {
    //   block [0x82EF9678..0x82EF968C)
	// 82EF9678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9690 size=176
    let mut pc: u32 = 0x82EF9690;
    'dispatch: loop {
        match pc {
            0x82EF9690 => {
    //   block [0x82EF9690..0x82EF96BC)
	// 82EF9690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9694: 4BDAFD71  bl 0x82ca9404
	ctx.lr = 0x82EF9698;
	sub_82CA93D0(ctx, base);
	// 82EF9698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF969C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF96A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF96A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF96A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF96AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF96B0: 409A000C  bne cr6, 0x82ef96bc
	if !ctx.cr[6].eq {
	pc = 0x82EF96BC; continue 'dispatch;
	}
	// 82EF96B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF96B8: 48000080  b 0x82ef9738
	pc = 0x82EF9738; continue 'dispatch;
            }
            0x82EF96BC => {
    //   block [0x82EF96BC..0x82EF96E8)
	// 82EF96BC: 3BAB0010  addi r29, r11, 0x10
	ctx.r[29].s64 = ctx.r[11].s64 + 16;
	// 82EF96C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF96C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF96C8: 483C029D  bl 0x832b9964
	ctx.lr = 0x82EF96CC;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF96CC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF96D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF96D4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF96D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF96DC: 419A0050  beq cr6, 0x82ef972c
	if ctx.cr[6].eq {
	pc = 0x82EF972C; continue 'dispatch;
	}
	// 82EF96E0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82EF96E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x82EF96E8; continue 'dispatch;
            }
            0x82EF96E8 => {
    //   block [0x82EF96E8..0x82EF9704)
	// 82EF96E8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF96EC: 7F09E040  cmplw cr6, r9, r28
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF96F0: 409A0014  bne cr6, 0x82ef9704
	if !ctx.cr[6].eq {
	pc = 0x82EF9704; continue 'dispatch;
	}
	// 82EF96F4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF96F8: 7F09D840  cmplw cr6, r9, r27
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82EF96FC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF9700: 419A0008  beq cr6, 0x82ef9708
	if ctx.cr[6].eq {
	pc = 0x82EF9708; continue 'dispatch;
	}
	pc = 0x82EF9704; continue 'dispatch;
            }
            0x82EF9704 => {
    //   block [0x82EF9704..0x82EF9708)
	// 82EF9704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x82EF9708; continue 'dispatch;
            }
            0x82EF9708 => {
    //   block [0x82EF9708..0x82EF9724)
	// 82EF9708: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF970C: 40820018  bne 0x82ef9724
	if !ctx.cr[0].eq {
	pc = 0x82EF9724; continue 'dispatch;
	}
	// 82EF9710: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82EF9714: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF9718: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF971C: 4198FFCC  blt cr6, 0x82ef96e8
	if ctx.cr[6].lt {
	pc = 0x82EF96E8; continue 'dispatch;
	}
	// 82EF9720: 4800000C  b 0x82ef972c
	pc = 0x82EF972C; continue 'dispatch;
            }
            0x82EF9724 => {
    //   block [0x82EF9724..0x82EF972C)
	// 82EF9724: 4BFFFEFD  bl 0x82ef9620
	ctx.lr = 0x82EF9728;
	sub_82EF9620(ctx, base);
	// 82EF9728: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	pc = 0x82EF972C; continue 'dispatch;
            }
            0x82EF972C => {
    //   block [0x82EF972C..0x82EF9738)
	// 82EF972C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9730: 483C0225  bl 0x832b9954
	ctx.lr = 0x82EF9734;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF9734: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82EF9738; continue 'dispatch;
            }
            0x82EF9738 => {
    //   block [0x82EF9738..0x82EF9740)
	// 82EF9738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF973C: 4BDAFD18  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9740 size=156
    let mut pc: u32 = 0x82EF9740;
    'dispatch: loop {
        match pc {
            0x82EF9740 => {
    //   block [0x82EF9740..0x82EF9768)
	// 82EF9740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9744: 4BDAFCC5  bl 0x82ca9408
	ctx.lr = 0x82EF9748;
	sub_82CA93D0(ctx, base);
	// 82EF9748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF974C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF9750: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF9754: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF9758: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF975C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9760: 40990038  ble cr6, 0x82ef9798
	if !ctx.cr[6].gt {
	pc = 0x82EF9798; continue 'dispatch;
	}
	// 82EF9764: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82EF9768; continue 'dispatch;
            }
            0x82EF9768 => {
    //   block [0x82EF9768..0x82EF9798)
	// 82EF9768: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF976C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF9770: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF9774: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF9778: 4BFFFE29  bl 0x82ef95a0
	ctx.lr = 0x82EF977C;
	sub_82EF95A0(ctx, base);
	// 82EF977C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9780: 41820024  beq 0x82ef97a4
	if ctx.cr[0].eq {
	pc = 0x82EF97A4; continue 'dispatch;
	}
	// 82EF9784: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9788: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF978C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF9790: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF9794: 4198FFD4  blt cr6, 0x82ef9768
	if ctx.cr[6].lt {
	pc = 0x82EF9768; continue 'dispatch;
	}
	pc = 0x82EF9798; continue 'dispatch;
            }
            0x82EF9798 => {
    //   block [0x82EF9798..0x82EF979C)
	// 82EF9798: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82EF979C; continue 'dispatch;
            }
            0x82EF979C => {
    //   block [0x82EF979C..0x82EF97A4)
	// 82EF979C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF97A0: 4BDAFCB8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x82EF97A4 => {
    //   block [0x82EF97A4..0x82EF97B0)
	// 82EF97A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF97A8: 419A002C  beq cr6, 0x82ef97d4
	if ctx.cr[6].eq {
	pc = 0x82EF97D4; continue 'dispatch;
	}
	// 82EF97AC: 57FD103A  slwi r29, r31, 2
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	pc = 0x82EF97B0; continue 'dispatch;
            }
            0x82EF97B0 => {
    //   block [0x82EF97B0..0x82EF97D4)
	// 82EF97B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF97B4: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 82EF97B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF97BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF97C0: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82EF97C4: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF97C8: 4BFFFEC9  bl 0x82ef9690
	ctx.lr = 0x82EF97CC;
	sub_82EF9690(ctx, base);
	// 82EF97CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF97D0: 409AFFE0  bne cr6, 0x82ef97b0
	if !ctx.cr[6].eq {
	pc = 0x82EF97B0; continue 'dispatch;
	}
	pc = 0x82EF97D4; continue 'dispatch;
            }
            0x82EF97D4 => {
    //   block [0x82EF97D4..0x82EF97DC)
	// 82EF97D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF97D8: 4BFFFFC4  b 0x82ef979c
	pc = 0x82EF979C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF97E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF97E0 size=88
    let mut pc: u32 = 0x82EF97E0;
    'dispatch: loop {
        match pc {
            0x82EF97E0 => {
    //   block [0x82EF97E0..0x82EF9808)
	// 82EF97E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF97E4: 4BDAFC25  bl 0x82ca9408
	ctx.lr = 0x82EF97E8;
	sub_82CA93D0(ctx, base);
	// 82EF97E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF97EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF97F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF97F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF97F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF97FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9800: 40990030  ble cr6, 0x82ef9830
	if !ctx.cr[6].gt {
	pc = 0x82EF9830; continue 'dispatch;
	}
	// 82EF9804: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82EF9808; continue 'dispatch;
            }
            0x82EF9808 => {
    //   block [0x82EF9808..0x82EF9830)
	// 82EF9808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF980C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF9810: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF9814: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF9818: 4BFFFE79  bl 0x82ef9690
	ctx.lr = 0x82EF981C;
	sub_82EF9690(ctx, base);
	// 82EF981C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9820: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EF9824: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF9828: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF982C: 4198FFDC  blt cr6, 0x82ef9808
	if ctx.cr[6].lt {
	pc = 0x82EF9808; continue 'dispatch;
	}
	pc = 0x82EF9830; continue 'dispatch;
            }
            0x82EF9830 => {
    //   block [0x82EF9830..0x82EF9838)
	// 82EF9830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9834: 4BDAFC24  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9838 size=80
    let mut pc: u32 = 0x82EF9838;
    'dispatch: loop {
        match pc {
            0x82EF9838 => {
    //   block [0x82EF9838..0x82EF9888)
	// 82EF9838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF984C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9850: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82EF9854: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9858: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF985C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF9860: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF9864: 4BFFF4DD  bl 0x82ef8d40
	ctx.lr = 0x82EF9868;
	sub_82EF8D40(ctx, base);
	// 82EF9868: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF986C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9870: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF987C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9888 size=152
    let mut pc: u32 = 0x82EF9888;
    'dispatch: loop {
        match pc {
            0x82EF9888 => {
    //   block [0x82EF9888..0x82EF98E0)
	// 82EF9888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF989C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF98A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF98A4: 480040A5  bl 0x82efd948
	ctx.lr = 0x82EF98A8;
	sub_82EFD948(ctx, base);
	// 82EF98A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF98AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF98B0: 396BCFB4  addi r11, r11, -0x304c
	ctx.r[11].s64 = ctx.r[11].s64 + -12364;
	// 82EF98B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF98B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF98BC: 4800419D  bl 0x82efda58
	ctx.lr = 0x82EF98C0;
	sub_82EFDA58(ctx, base);
	// 82EF98C0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF98C4: 4182001C  beq 0x82ef98e0
	if ctx.cr[0].eq {
	pc = 0x82EF98E0; continue 'dispatch;
	}
	// 82EF98C8: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82EF98CC: 48003F95  bl 0x82efd860
	ctx.lr = 0x82EF98D0;
	sub_82EFD860(ctx, base);
	// 82EF98D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF98D4: 4182000C  beq 0x82ef98e0
	if ctx.cr[0].eq {
	pc = 0x82EF98E0; continue 'dispatch;
	}
	// 82EF98D8: 4BFFFF61  bl 0x82ef9838
	ctx.lr = 0x82EF98DC;
	sub_82EF9838(ctx, base);
	// 82EF98DC: 48000008  b 0x82ef98e4
	pc = 0x82EF98E4; continue 'dispatch;
            }
            0x82EF98E0 => {
    //   block [0x82EF98E0..0x82EF98E4)
	// 82EF98E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EF98E4; continue 'dispatch;
            }
            0x82EF98E4 => {
    //   block [0x82EF98E4..0x82EF9904)
	// 82EF98E4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82EF98E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF98EC: 419A0018  beq cr6, 0x82ef9904
	if ctx.cr[6].eq {
	pc = 0x82EF9904; continue 'dispatch;
	}
	// 82EF98F0: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF98F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF98F8: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82EF98FC: 512AF002  rlwimi r10, r9, 0x1e, 0, 1
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(30) as u64) & 0x00000000C0000000) | (ctx.r[10].u64 & 0xFFFFFFFF3FFFFFFF);
	// 82EF9900: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	pc = 0x82EF9904; continue 'dispatch;
            }
            0x82EF9904 => {
    //   block [0x82EF9904..0x82EF9920)
	// 82EF9904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF990C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9914: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF991C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9920 size=76
    let mut pc: u32 = 0x82EF9920;
    'dispatch: loop {
        match pc {
            0x82EF9920 => {
    //   block [0x82EF9920..0x82EF9958)
	// 82EF9920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF992C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9930: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9934: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82EF9938: 48001701  bl 0x82efb038
	ctx.lr = 0x82EF993C;
	sub_82EFB038(ctx, base);
	// 82EF993C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82EF9940: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9944: 48039A45  bl 0x82f33388
	ctx.lr = 0x82EF9948;
	sub_82F33388(ctx, base);
	// 82EF9948: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF994C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9950: 419A0008  beq cr6, 0x82ef9958
	if ctx.cr[6].eq {
	pc = 0x82EF9958; continue 'dispatch;
	}
	// 82EF9954: 48003F4D  bl 0x82efd8a0
	ctx.lr = 0x82EF9958;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF9958; continue 'dispatch;
            }
            0x82EF9958 => {
    //   block [0x82EF9958..0x82EF996C)
	// 82EF9958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF995C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9970 size=188
    let mut pc: u32 = 0x82EF9970;
    'dispatch: loop {
        match pc {
            0x82EF9970 => {
    //   block [0x82EF9970..0x82EF99B8)
	// 82EF9970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9974: 4BDAFA99  bl 0x82ca940c
	ctx.lr = 0x82EF9978;
	sub_82CA93D0(ctx, base);
	// 82EF9978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF997C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9980: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 82EF9984: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9988: 483BFFDD  bl 0x832b9964
	ctx.lr = 0x82EF998C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF998C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9994: 419A0088  beq cr6, 0x82ef9a1c
	if ctx.cr[6].eq {
	pc = 0x82EF9A1C; continue 'dispatch;
	}
	// 82EF9998: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82EF999C: 409A001C  bne cr6, 0x82ef99b8
	if !ctx.cr[6].eq {
	pc = 0x82EF99B8; continue 'dispatch;
	}
	// 82EF99A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF99A4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF99A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF99AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF99B0: 4E800421  bctrl
	ctx.lr = 0x82EF99B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF99B4: 48000068  b 0x82ef9a1c
	pc = 0x82EF9A1C; continue 'dispatch;
            }
            0x82EF99B8 => {
    //   block [0x82EF99B8..0x82EF99E4)
	// 82EF99B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF99BC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82EF99C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF99C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF99C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF99CC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF99D0: 4BFFFB49  bl 0x82ef9518
	ctx.lr = 0x82EF99D4;
	sub_82EF9518(ctx, base);
	// 82EF99D4: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF99D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF99DC: 419A0024  beq cr6, 0x82ef9a00
	if ctx.cr[6].eq {
	pc = 0x82EF9A00; continue 'dispatch;
	}
	// 82EF99E0: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	pc = 0x82EF99E4; continue 'dispatch;
            }
            0x82EF99E4 => {
    //   block [0x82EF99E4..0x82EF9A00)
	// 82EF99E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF99E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF99EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF99F0: 4E800421  bctrl
	ctx.lr = 0x82EF99F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF99F4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF99F8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82EF99FC: 4082FFE8  bne 0x82ef99e4
	if !ctx.cr[0].eq {
	pc = 0x82EF99E4; continue 'dispatch;
	}
            }
            0x82EF9A00 => {
    //   block [0x82EF9A00..0x82EF9A1C)
	// 82EF9A00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9A04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF9A08: 48039981  bl 0x82f33388
	ctx.lr = 0x82EF9A0C;
	sub_82F33388(ctx, base);
	// 82EF9A0C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9A10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9A14: 419A0008  beq cr6, 0x82ef9a1c
	if ctx.cr[6].eq {
	pc = 0x82EF9A1C; continue 'dispatch;
	}
	// 82EF9A18: 48003E89  bl 0x82efd8a0
	ctx.lr = 0x82EF9A1C;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF9A1C; continue 'dispatch;
            }
            0x82EF9A1C => {
    //   block [0x82EF9A1C..0x82EF9A2C)
	// 82EF9A1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9A20: 483BFF35  bl 0x832b9954
	ctx.lr = 0x82EF9A24;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF9A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9A28: 4BDAFA34  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9A30 size=16
    let mut pc: u32 = 0x82EF9A30;
    'dispatch: loop {
        match pc {
            0x82EF9A30 => {
    //   block [0x82EF9A30..0x82EF9A40)
	// 82EF9A30: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF9A34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9A38: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82EF9A3C: 4BFFFF34  b 0x82ef9970
	sub_82EF9970(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9A48 size=108
    let mut pc: u32 = 0x82EF9A48;
    'dispatch: loop {
        match pc {
            0x82EF9A48 => {
    //   block [0x82EF9A48..0x82EF9A60)
	// 82EF9A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9A54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9A58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9A5C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82EF9A60; continue 'dispatch;
            }
            0x82EF9A60 => {
    //   block [0x82EF9A60..0x82EF9AA0)
	// 82EF9A60: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF9A64: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9A68: 7D40F828  lwarx r10, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF9A6C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF9A70: 7D20F92D  stwcx. r9, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF9A74: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9A78: 4082FFE8  bne 0x82ef9a60
	if !ctx.cr[0].eq {
	pc = 0x82EF9A60; continue 'dispatch;
	}
	// 82EF9A7C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF9A80: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9A84: 4082001C  bne 0x82ef9aa0
	if !ctx.cr[0].eq {
	pc = 0x82EF9AA0; continue 'dispatch;
	}
	// 82EF9A88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9A8C: 419A0014  beq cr6, 0x82ef9aa0
	if ctx.cr[6].eq {
	pc = 0x82EF9AA0; continue 'dispatch;
	}
	// 82EF9A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9A94: 4BFFFE8D  bl 0x82ef9920
	ctx.lr = 0x82EF9A98;
	sub_82EF9920(ctx, base);
	// 82EF9A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9A9C: 48003E05  bl 0x82efd8a0
	ctx.lr = 0x82EF9AA0;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EF9AA0; continue 'dispatch;
            }
            0x82EF9AA0 => {
    //   block [0x82EF9AA0..0x82EF9AB4)
	// 82EF9AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9AB8 size=120
    let mut pc: u32 = 0x82EF9AB8;
    'dispatch: loop {
        match pc {
            0x82EF9AB8 => {
    //   block [0x82EF9AB8..0x82EF9AE4)
	// 82EF9AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9ACC: 83E30010  lwz r31, 0x10(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF9AD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9AD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9AD8: 419A0040  beq cr6, 0x82ef9b18
	if ctx.cr[6].eq {
	pc = 0x82EF9B18; continue 'dispatch;
	}
	// 82EF9ADC: 7C2004AC  lwsync
	// 82EF9AE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	pc = 0x82EF9AE4; continue 'dispatch;
            }
            0x82EF9AE4 => {
    //   block [0x82EF9AE4..0x82EF9B14)
	// 82EF9AE4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF9AE8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9AEC: 7D60F828  lwarx r11, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EF9AF0: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF9AF4: 7D20F92D  stwcx. r9, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF9AF8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9AFC: 4082FFE8  bne 0x82ef9ae4
	if !ctx.cr[0].eq {
	pc = 0x82EF9AE4; continue 'dispatch;
	}
	// 82EF9B00: 7C2004AC  lwsync
	// 82EF9B04: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9B08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9B0C: 419A0008  beq cr6, 0x82ef9b14
	if ctx.cr[6].eq {
	pc = 0x82EF9B14; continue 'dispatch;
	}
	// 82EF9B10: 4BFFFF39  bl 0x82ef9a48
	ctx.lr = 0x82EF9B14;
	sub_82EF9A48(ctx, base);
	pc = 0x82EF9B14; continue 'dispatch;
            }
            0x82EF9B14 => {
    //   block [0x82EF9B14..0x82EF9B18)
	// 82EF9B14: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	pc = 0x82EF9B18; continue 'dispatch;
            }
            0x82EF9B18 => {
    //   block [0x82EF9B18..0x82EF9B30)
	// 82EF9B18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9B24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9B28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9B30 size=76
    let mut pc: u32 = 0x82EF9B30;
    'dispatch: loop {
        match pc {
            0x82EF9B30 => {
    //   block [0x82EF9B30..0x82EF9B60)
	// 82EF9B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9B48: 396BCFB4  addi r11, r11, -0x304c
	ctx.r[11].s64 = ctx.r[11].s64 + -12364;
	// 82EF9B4C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF9B50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9B54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9B58: 419A0008  beq cr6, 0x82ef9b60
	if ctx.cr[6].eq {
	pc = 0x82EF9B60; continue 'dispatch;
	}
	// 82EF9B5C: 4BFFFEED  bl 0x82ef9a48
	ctx.lr = 0x82EF9B60;
	sub_82EF9A48(ctx, base);
	pc = 0x82EF9B60; continue 'dispatch;
            }
            0x82EF9B60 => {
    //   block [0x82EF9B60..0x82EF9B7C)
	// 82EF9B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9B64: 48003EB5  bl 0x82efda18
	ctx.lr = 0x82EF9B68;
	sub_82EFDA18(ctx, base);
	// 82EF9B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9B74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9B80 size=136
    let mut pc: u32 = 0x82EF9B80;
    'dispatch: loop {
        match pc {
            0x82EF9B80 => {
    //   block [0x82EF9B80..0x82EF9C08)
	// 82EF9B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9B88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9B8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9B90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9B94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9B98: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF9B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9BA0: 4BFFFCE9  bl 0x82ef9888
	ctx.lr = 0x82EF9BA4;
	sub_82EF9888(ctx, base);
	// 82EF9BA4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9BA8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9BAC: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EF9BB0: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9BB4: 394ACFD4  addi r10, r10, -0x302c
	ctx.r[10].s64 = ctx.r[10].s64 + -12332;
	// 82EF9BB8: 3929CFC0  addi r9, r9, -0x3040
	ctx.r[9].s64 = ctx.r[9].s64 + -12352;
	// 82EF9BBC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9BC0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF9BC4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82EF9BC8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EF9BCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF9BD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9BD4: 48000EF5  bl 0x82efaac8
	ctx.lr = 0x82EF9BD8;
	sub_82EFAAC8(ctx, base);
	// 82EF9BD8: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF9BDC: 480013D5  bl 0x82efafb0
	ctx.lr = 0x82EF9BE0;
	sub_82EFAFB0(ctx, base);
	// 82EF9BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9BE4: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82EF9BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9BEC: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF9BF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9BFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9C00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9C08 size=112
    let mut pc: u32 = 0x82EF9C08;
    'dispatch: loop {
        match pc {
            0x82EF9C08 => {
    //   block [0x82EF9C08..0x82EF9C78)
	// 82EF9C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9C18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9C20: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9C24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9C28: 396BCFD4  addi r11, r11, -0x302c
	ctx.r[11].s64 = ctx.r[11].s64 + -12332;
	// 82EF9C2C: 394ACFC0  addi r10, r10, -0x3040
	ctx.r[10].s64 = ctx.r[10].s64 + -12352;
	// 82EF9C30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9C34: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF9C38: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF9C3C: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF9C40: 48001501  bl 0x82efb140
	ctx.lr = 0x82EF9C44;
	sub_82EFB140(ctx, base);
	// 82EF9C44: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82EF9C48: 48001491  bl 0x82efb0d8
	ctx.lr = 0x82EF9C4C;
	sub_82EFB0D8(ctx, base);
	// 82EF9C4C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9C54: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9C58: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9C5C: 4BFFFED5  bl 0x82ef9b30
	ctx.lr = 0x82EF9C60;
	sub_82EF9B30(ctx, base);
	// 82EF9C60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9C6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9C70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9C78 size=136
    let mut pc: u32 = 0x82EF9C78;
    'dispatch: loop {
        match pc {
            0x82EF9C78 => {
    //   block [0x82EF9C78..0x82EF9CE8)
	// 82EF9C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9C80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9C84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9C90: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82EF9C94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9C98: 48000EC1  bl 0x82efab58
	ctx.lr = 0x82EF9C9C;
	sub_82EFAB58(ctx, base);
	// 82EF9C9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF9CA0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF9CA4: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EF9CA8: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF9CAC: 4800135D  bl 0x82efb008
	ctx.lr = 0x82EF9CB0;
	sub_82EFB008(ctx, base);
	// 82EF9CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9CB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9CB8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF9CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9CC0: 4BFFFDF9  bl 0x82ef9ab8
	ctx.lr = 0x82EF9CC4;
	sub_82EF9AB8(ctx, base);
	// 82EF9CC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9CC8: 48001DD9  bl 0x82efbaa0
	ctx.lr = 0x82EF9CCC;
	sub_82EFBAA0(ctx, base);
	// 82EF9CCC: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9CD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9CD4: 419A0014  beq cr6, 0x82ef9ce8
	if ctx.cr[6].eq {
	pc = 0x82EF9CE8; continue 'dispatch;
	}
	// 82EF9CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9CDC: 4BFFFC95  bl 0x82ef9970
	ctx.lr = 0x82EF9CE0;
	sub_82EF9970(ctx, base);
	// 82EF9CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9CE4: 4BFFFD65  bl 0x82ef9a48
	ctx.lr = 0x82EF9CE8;
	sub_82EF9A48(ctx, base);
	pc = 0x82EF9CE8; continue 'dispatch;
            }
            0x82EF9CE8 => {
    //   block [0x82EF9CE8..0x82EF9D00)
	// 82EF9CE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9CF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9CF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9D00 size=136
    let mut pc: u32 = 0x82EF9D00;
    'dispatch: loop {
        match pc {
            0x82EF9D00 => {
    //   block [0x82EF9D00..0x82EF9D88)
	// 82EF9D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9D08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9D0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9D14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9D18: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF9D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9D20: 4BFFFB69  bl 0x82ef9888
	ctx.lr = 0x82EF9D24;
	sub_82EF9888(ctx, base);
	// 82EF9D24: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9D28: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9D2C: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EF9D30: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9D34: 394ACFF4  addi r10, r10, -0x300c
	ctx.r[10].s64 = ctx.r[10].s64 + -12300;
	// 82EF9D38: 3929CFE0  addi r9, r9, -0x3020
	ctx.r[9].s64 = ctx.r[9].s64 + -12320;
	// 82EF9D3C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9D40: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF9D44: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82EF9D48: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EF9D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF9D50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9D54: 48000D75  bl 0x82efaac8
	ctx.lr = 0x82EF9D58;
	sub_82EFAAC8(ctx, base);
	// 82EF9D58: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9D5C: 48001255  bl 0x82efafb0
	ctx.lr = 0x82EF9D60;
	sub_82EFAFB0(ctx, base);
	// 82EF9D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9D64: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EF9D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9D6C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9D88 size=112
    let mut pc: u32 = 0x82EF9D88;
    'dispatch: loop {
        match pc {
            0x82EF9D88 => {
    //   block [0x82EF9D88..0x82EF9DF8)
	// 82EF9D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9D9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9DA0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9DA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9DA8: 396BCFF4  addi r11, r11, -0x300c
	ctx.r[11].s64 = ctx.r[11].s64 + -12300;
	// 82EF9DAC: 394ACFE0  addi r10, r10, -0x3020
	ctx.r[10].s64 = ctx.r[10].s64 + -12320;
	// 82EF9DB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9DB4: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9DB8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF9DBC: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF9DC0: 48001381  bl 0x82efb140
	ctx.lr = 0x82EF9DC4;
	sub_82EFB140(ctx, base);
	// 82EF9DC4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82EF9DC8: 48001311  bl 0x82efb0d8
	ctx.lr = 0x82EF9DCC;
	sub_82EFB0D8(ctx, base);
	// 82EF9DCC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9DD4: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9DD8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9DDC: 4BFFFD55  bl 0x82ef9b30
	ctx.lr = 0x82EF9DE0;
	sub_82EF9B30(ctx, base);
	// 82EF9DE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9DEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9DF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9DF8 size=164
    let mut pc: u32 = 0x82EF9DF8;
    'dispatch: loop {
        match pc {
            0x82EF9DF8 => {
    //   block [0x82EF9DF8..0x82EF9E40)
	// 82EF9DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9DFC: 4BDAF60D  bl 0x82ca9408
	ctx.lr = 0x82EF9E00;
	sub_82CA93D0(ctx, base);
	// 82EF9E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9E04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9E08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9E0C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF9E10: 419A0080  beq cr6, 0x82ef9e90
	if ctx.cr[6].eq {
	pc = 0x82EF9E90; continue 'dispatch;
	}
	// 82EF9E14: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82EF9E18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9E1C: 48000D3D  bl 0x82efab58
	ctx.lr = 0x82EF9E20;
	sub_82EFAB58(ctx, base);
	// 82EF9E20: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9E24: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF9E28: 7D7E5851  subf. r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9E2C: 41800014  blt 0x82ef9e40
	if ctx.cr[0].lt {
	pc = 0x82EF9E40; continue 'dispatch;
	}
	// 82EF9E30: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9E34: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82EF9E38: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9E3C: 48000008  b 0x82ef9e44
	pc = 0x82EF9E44; continue 'dispatch;
            }
            0x82EF9E40 => {
    //   block [0x82EF9E40..0x82EF9E44)
	// 82EF9E40: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	pc = 0x82EF9E44; continue 'dispatch;
            }
            0x82EF9E44 => {
    //   block [0x82EF9E44..0x82EF9E58)
	// 82EF9E44: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82EF9E48: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9E4C: 409A000C  bne cr6, 0x82ef9e58
	if !ctx.cr[6].eq {
	pc = 0x82EF9E58; continue 'dispatch;
	}
	// 82EF9E50: 480011B9  bl 0x82efb008
	ctx.lr = 0x82EF9E54;
	sub_82EFB008(ctx, base);
	// 82EF9E54: 48000008  b 0x82ef9e5c
	pc = 0x82EF9E5C; continue 'dispatch;
            }
            0x82EF9E58 => {
    //   block [0x82EF9E58..0x82EF9E5C)
	// 82EF9E58: 480011B9  bl 0x82efb010
	ctx.lr = 0x82EF9E5C;
	sub_82EFB010(ctx, base);
	pc = 0x82EF9E5C; continue 'dispatch;
            }
            0x82EF9E5C => {
    //   block [0x82EF9E5C..0x82EF9E90)
	// 82EF9E5C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF9E60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9E68: 4BFFFC51  bl 0x82ef9ab8
	ctx.lr = 0x82EF9E6C;
	sub_82EF9AB8(ctx, base);
	// 82EF9E6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9E70: 48001C31  bl 0x82efbaa0
	ctx.lr = 0x82EF9E74;
	sub_82EFBAA0(ctx, base);
	// 82EF9E74: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9E78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9E7C: 419A0014  beq cr6, 0x82ef9e90
	if ctx.cr[6].eq {
	pc = 0x82EF9E90; continue 'dispatch;
	}
	// 82EF9E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9E84: 4BFFFAED  bl 0x82ef9970
	ctx.lr = 0x82EF9E88;
	sub_82EF9970(ctx, base);
	// 82EF9E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9E8C: 4BFFFBBD  bl 0x82ef9a48
	ctx.lr = 0x82EF9E90;
	sub_82EF9A48(ctx, base);
	pc = 0x82EF9E90; continue 'dispatch;
            }
            0x82EF9E90 => {
    //   block [0x82EF9E90..0x82EF9E9C)
	// 82EF9E90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF9E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9E98: 4BDAF5C0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9EA0 size=164
    let mut pc: u32 = 0x82EF9EA0;
    'dispatch: loop {
        match pc {
            0x82EF9EA0 => {
    //   block [0x82EF9EA0..0x82EF9EDC)
	// 82EF9EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9EA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9EAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9EB8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82EF9EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9EC0: 48000C99  bl 0x82efab58
	ctx.lr = 0x82EF9EC4;
	sub_82EFAB58(ctx, base);
	// 82EF9EC4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9EC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9ECC: 40990010  ble cr6, 0x82ef9edc
	if !ctx.cr[6].gt {
	pc = 0x82EF9EDC; continue 'dispatch;
	}
	// 82EF9ED0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9ED4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF9ED8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	pc = 0x82EF9EDC; continue 'dispatch;
            }
            0x82EF9EDC => {
    //   block [0x82EF9EDC..0x82EF9F14)
	// 82EF9EDC: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9EE0: 48001129  bl 0x82efb008
	ctx.lr = 0x82EF9EE4;
	sub_82EFB008(ctx, base);
	// 82EF9EE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9EE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9EEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF9EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9EF4: 4BFFFBC5  bl 0x82ef9ab8
	ctx.lr = 0x82EF9EF8;
	sub_82EF9AB8(ctx, base);
	// 82EF9EF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9EFC: 48001BA5  bl 0x82efbaa0
	ctx.lr = 0x82EF9F00;
	sub_82EFBAA0(ctx, base);
	// 82EF9F00: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9F04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9F08: 419A000C  beq cr6, 0x82ef9f14
	if ctx.cr[6].eq {
	pc = 0x82EF9F14; continue 'dispatch;
	}
	// 82EF9F0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9F10: 4BFFFA61  bl 0x82ef9970
	ctx.lr = 0x82EF9F14;
	sub_82EF9970(ctx, base);
	pc = 0x82EF9F14; continue 'dispatch;
            }
            0x82EF9F14 => {
    //   block [0x82EF9F14..0x82EF9F28)
	// 82EF9F14: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9F18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9F1C: 419A000C  beq cr6, 0x82ef9f28
	if ctx.cr[6].eq {
	pc = 0x82EF9F28; continue 'dispatch;
	}
	// 82EF9F20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9F24: 4BFFFB25  bl 0x82ef9a48
	ctx.lr = 0x82EF9F28;
	sub_82EF9A48(ctx, base);
	pc = 0x82EF9F28; continue 'dispatch;
            }
            0x82EF9F28 => {
    //   block [0x82EF9F28..0x82EF9F44)
	// 82EF9F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9F2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9F38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9F3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9F48 size=152
    let mut pc: u32 = 0x82EF9F48;
    'dispatch: loop {
        match pc {
            0x82EF9F48 => {
    //   block [0x82EF9F48..0x82EF9F88)
	// 82EF9F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9F4C: 4BDAF4BD  bl 0x82ca9408
	ctx.lr = 0x82EF9F50;
	sub_82CA93D0(ctx, base);
	// 82EF9F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9F58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9F5C: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82EF9F60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9F64: 48000BF5  bl 0x82efab58
	ctx.lr = 0x82EF9F68;
	sub_82EFAB58(ctx, base);
	// 82EF9F68: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9F6C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF9F70: 7D7E5851  subf. r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9F74: 41800014  blt 0x82ef9f88
	if ctx.cr[0].lt {
	pc = 0x82EF9F88; continue 'dispatch;
	}
	// 82EF9F78: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9F7C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82EF9F80: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9F84: 48000008  b 0x82ef9f8c
	pc = 0x82EF9F8C; continue 'dispatch;
            }
            0x82EF9F88 => {
    //   block [0x82EF9F88..0x82EF9F8C)
	// 82EF9F88: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	pc = 0x82EF9F8C; continue 'dispatch;
            }
            0x82EF9F8C => {
    //   block [0x82EF9F8C..0x82EF9FC0)
	// 82EF9F8C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9F90: 48001081  bl 0x82efb010
	ctx.lr = 0x82EF9F94;
	sub_82EFB010(ctx, base);
	// 82EF9F94: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF9F98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9FA0: 4BFFFB19  bl 0x82ef9ab8
	ctx.lr = 0x82EF9FA4;
	sub_82EF9AB8(ctx, base);
	// 82EF9FA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9FA8: 48001AF9  bl 0x82efbaa0
	ctx.lr = 0x82EF9FAC;
	sub_82EFBAA0(ctx, base);
	// 82EF9FAC: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9FB0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9FB4: 419A000C  beq cr6, 0x82ef9fc0
	if ctx.cr[6].eq {
	pc = 0x82EF9FC0; continue 'dispatch;
	}
	// 82EF9FB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9FBC: 4BFFF9B5  bl 0x82ef9970
	ctx.lr = 0x82EF9FC0;
	sub_82EF9970(ctx, base);
	pc = 0x82EF9FC0; continue 'dispatch;
            }
            0x82EF9FC0 => {
    //   block [0x82EF9FC0..0x82EF9FD4)
	// 82EF9FC0: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9FC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9FC8: 419A000C  beq cr6, 0x82ef9fd4
	if ctx.cr[6].eq {
	pc = 0x82EF9FD4; continue 'dispatch;
	}
	// 82EF9FCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9FD0: 4BFFFA79  bl 0x82ef9a48
	ctx.lr = 0x82EF9FD4;
	sub_82EF9A48(ctx, base);
	pc = 0x82EF9FD4; continue 'dispatch;
            }
            0x82EF9FD4 => {
    //   block [0x82EF9FD4..0x82EF9FE0)
	// 82EF9FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9FDC: 4BDAF47C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9FE0 size=12
    let mut pc: u32 = 0x82EF9FE0;
    'dispatch: loop {
        match pc {
            0x82EF9FE0 => {
    //   block [0x82EF9FE0..0x82EF9FEC)
	// 82EF9FE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9FE4: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF9FE8: 4BFFFE10  b 0x82ef9df8
	sub_82EF9DF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9FF0 size=108
    let mut pc: u32 = 0x82EF9FF0;
    'dispatch: loop {
        match pc {
            0x82EF9FF0 => {
    //   block [0x82EF9FF0..0x82EFA05C)
	// 82EF9FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9FF4: 4BDAF419  bl 0x82ca940c
	ctx.lr = 0x82EF9FF8;
	sub_82CA93D0(ctx, base);
	// 82EF9FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9FFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA000: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFA004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA008: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFA00C: 4BFFF87D  bl 0x82ef9888
	ctx.lr = 0x82EFA010;
	sub_82EF9888(ctx, base);
	// 82EFA010: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFA014: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFA018: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFA01C: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFA020: 394AD014  addi r10, r10, -0x2fec
	ctx.r[10].s64 = ctx.r[10].s64 + -12268;
	// 82EFA024: 3929D000  addi r9, r9, -0x3000
	ctx.r[9].s64 = ctx.r[9].s64 + -12288;
	// 82EFA028: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFA02C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EFA030: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA034: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFA038: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFA03C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFA040: 388B9A30  addi r4, r11, -0x65d0
	ctx.r[4].s64 = ctx.r[11].s64 + -26064;
	// 82EFA044: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82EFA048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA04C: 4BFFF555  bl 0x82ef95a0
	ctx.lr = 0x82EFA050;
	sub_82EF95A0(ctx, base);
	// 82EFA050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA058: 4BDAF404  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA068 size=116
    let mut pc: u32 = 0x82EFA068;
    'dispatch: loop {
        match pc {
            0x82EFA068 => {
    //   block [0x82EFA068..0x82EFA0DC)
	// 82EFA068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA080: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFA084: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFA088: 396BD014  addi r11, r11, -0x2fec
	ctx.r[11].s64 = ctx.r[11].s64 + -12268;
	// 82EFA08C: 394AD000  addi r10, r10, -0x3000
	ctx.r[10].s64 = ctx.r[10].s64 + -12288;
	// 82EFA090: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFA094: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA098: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EFA09C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFA0A0: 388B9A30  addi r4, r11, -0x65d0
	ctx.r[4].s64 = ctx.r[11].s64 + -26064;
	// 82EFA0A4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFA0A8: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EFA0AC: 4BFFF5E5  bl 0x82ef9690
	ctx.lr = 0x82EFA0B0;
	sub_82EF9690(ctx, base);
	// 82EFA0B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFA0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA0B8: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFA0BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFA0C0: 4BFFFA71  bl 0x82ef9b30
	ctx.lr = 0x82EFA0C4;
	sub_82EF9B30(ctx, base);
	// 82EFA0C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA0D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA0D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA0E0 size=20
    let mut pc: u32 = 0x82EFA0E0;
    'dispatch: loop {
        match pc {
            0x82EFA0E0 => {
    //   block [0x82EFA0E0..0x82EFA0F4)
	// 82EFA0E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFA0E4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA0E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFA0EC: 409A0008  bne cr6, 0x82efa0f4
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82EFA0F4);
		return;
	}
	// 82EFA0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA100 size=112
    let mut pc: u32 = 0x82EFA100;
    'dispatch: loop {
        match pc {
            0x82EFA100 => {
    //   block [0x82EFA100..0x82EFA154)
	// 82EFA100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA10C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA118: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82EFA11C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA120: 48003741  bl 0x82efd860
	ctx.lr = 0x82EFA124;
	sub_82EFD860(ctx, base);
	// 82EFA124: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFA128: 4182002C  beq 0x82efa154
	if ctx.cr[0].eq {
	pc = 0x82EFA154; continue 'dispatch;
	}
	// 82EFA12C: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EFA130: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFA134: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EFA138: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EFA13C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFA140: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFA144: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFA148: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFA14C: 4BFFFEA5  bl 0x82ef9ff0
	ctx.lr = 0x82EFA150;
	sub_82EF9FF0(ctx, base);
	// 82EFA150: 48000008  b 0x82efa158
	pc = 0x82EFA158; continue 'dispatch;
            }
            0x82EFA154 => {
    //   block [0x82EFA154..0x82EFA158)
	// 82EFA154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFA158; continue 'dispatch;
            }
            0x82EFA158 => {
    //   block [0x82EFA158..0x82EFA170)
	// 82EFA158: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA164: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA170 size=132
    let mut pc: u32 = 0x82EFA170;
    'dispatch: loop {
        match pc {
            0x82EFA170 => {
    //   block [0x82EFA170..0x82EFA1A4)
	// 82EFA170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA174: 4BDAF295  bl 0x82ca9408
	ctx.lr = 0x82EFA178;
	sub_82CA93D0(ctx, base);
	// 82EFA178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA17C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA180: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA184: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA188: 41820048  beq 0x82efa1d0
	if ctx.cr[0].eq {
	pc = 0x82EFA1D0; continue 'dispatch;
	}
	// 82EFA18C: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFA190: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFA194: 1D6A003C  mulli r11, r10, 0x3c
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 60 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EFA198: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA19C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFA1A0: 41800018  blt 0x82efa1b8
	if ctx.cr[0].lt {
	pc = 0x82EFA1B8; continue 'dispatch;
	}
	pc = 0x82EFA1A4; continue 'dispatch;
            }
            0x82EFA1A4 => {
    //   block [0x82EFA1A4..0x82EFA1B8)
	// 82EFA1A4: 3BDEFFC4  addi r30, r30, -0x3c
	ctx.r[30].s64 = ctx.r[30].s64 + -60;
	// 82EFA1A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA1AC: 4BFFFA5D  bl 0x82ef9c08
	ctx.lr = 0x82EFA1B0;
	sub_82EF9C08(ctx, base);
	// 82EFA1B0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA1B4: 4080FFF0  bge 0x82efa1a4
	if !ctx.cr[0].lt {
	pc = 0x82EFA1A4; continue 'dispatch;
	}
	pc = 0x82EFA1B8; continue 'dispatch;
            }
            0x82EFA1B8 => {
    //   block [0x82EFA1B8..0x82EFA1C8)
	// 82EFA1B8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA1BC: 4182000C  beq 0x82efa1c8
	if ctx.cr[0].eq {
	pc = 0x82EFA1C8; continue 'dispatch;
	}
	// 82EFA1C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA1C4: 4B94B5ED  bl 0x828457b0
	ctx.lr = 0x82EFA1C8;
	sub_828457B0(ctx, base);
	pc = 0x82EFA1C8; continue 'dispatch;
            }
            0x82EFA1C8 => {
    //   block [0x82EFA1C8..0x82EFA1D0)
	// 82EFA1C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA1CC: 48000020  b 0x82efa1ec
	pc = 0x82EFA1EC; continue 'dispatch;
            }
            0x82EFA1D0 => {
    //   block [0x82EFA1D0..0x82EFA1E8)
	// 82EFA1D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA1D4: 4BFFFA35  bl 0x82ef9c08
	ctx.lr = 0x82EFA1D8;
	sub_82EF9C08(ctx, base);
	// 82EFA1D8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA1DC: 4182000C  beq 0x82efa1e8
	if ctx.cr[0].eq {
	pc = 0x82EFA1E8; continue 'dispatch;
	}
	// 82EFA1E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA1E4: 480036BD  bl 0x82efd8a0
	ctx.lr = 0x82EFA1E8;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFA1E8; continue 'dispatch;
            }
            0x82EFA1E8 => {
    //   block [0x82EFA1E8..0x82EFA1EC)
	// 82EFA1E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82EFA1EC; continue 'dispatch;
            }
            0x82EFA1EC => {
    //   block [0x82EFA1EC..0x82EFA1F4)
	// 82EFA1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFA1F0: 4BDAF268  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA1F8 size=132
    let mut pc: u32 = 0x82EFA1F8;
    'dispatch: loop {
        match pc {
            0x82EFA1F8 => {
    //   block [0x82EFA1F8..0x82EFA22C)
	// 82EFA1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA1FC: 4BDAF20D  bl 0x82ca9408
	ctx.lr = 0x82EFA200;
	sub_82CA93D0(ctx, base);
	// 82EFA200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA204: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA208: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA20C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA210: 41820048  beq 0x82efa258
	if ctx.cr[0].eq {
	pc = 0x82EFA258; continue 'dispatch;
	}
	// 82EFA214: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFA218: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFA21C: 554B3032  slwi r11, r10, 6
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA220: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA224: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFA228: 41800018  blt 0x82efa240
	if ctx.cr[0].lt {
	pc = 0x82EFA240; continue 'dispatch;
	}
	pc = 0x82EFA22C; continue 'dispatch;
            }
            0x82EFA22C => {
    //   block [0x82EFA22C..0x82EFA240)
	// 82EFA22C: 3BDEFFC0  addi r30, r30, -0x40
	ctx.r[30].s64 = ctx.r[30].s64 + -64;
	// 82EFA230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA234: 4BFFFB55  bl 0x82ef9d88
	ctx.lr = 0x82EFA238;
	sub_82EF9D88(ctx, base);
	// 82EFA238: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA23C: 4080FFF0  bge 0x82efa22c
	if !ctx.cr[0].lt {
	pc = 0x82EFA22C; continue 'dispatch;
	}
	pc = 0x82EFA240; continue 'dispatch;
            }
            0x82EFA240 => {
    //   block [0x82EFA240..0x82EFA250)
	// 82EFA240: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA244: 4182000C  beq 0x82efa250
	if ctx.cr[0].eq {
	pc = 0x82EFA250; continue 'dispatch;
	}
	// 82EFA248: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA24C: 4B94B565  bl 0x828457b0
	ctx.lr = 0x82EFA250;
	sub_828457B0(ctx, base);
	pc = 0x82EFA250; continue 'dispatch;
            }
            0x82EFA250 => {
    //   block [0x82EFA250..0x82EFA258)
	// 82EFA250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA254: 48000020  b 0x82efa274
	pc = 0x82EFA274; continue 'dispatch;
            }
            0x82EFA258 => {
    //   block [0x82EFA258..0x82EFA270)
	// 82EFA258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA25C: 4BFFFB2D  bl 0x82ef9d88
	ctx.lr = 0x82EFA260;
	sub_82EF9D88(ctx, base);
	// 82EFA260: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA264: 4182000C  beq 0x82efa270
	if ctx.cr[0].eq {
	pc = 0x82EFA270; continue 'dispatch;
	}
	// 82EFA268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA26C: 48003635  bl 0x82efd8a0
	ctx.lr = 0x82EFA270;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFA270; continue 'dispatch;
            }
            0x82EFA270 => {
    //   block [0x82EFA270..0x82EFA274)
	// 82EFA270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82EFA274; continue 'dispatch;
            }
            0x82EFA274 => {
    //   block [0x82EFA274..0x82EFA27C)
	// 82EFA274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFA278: 4BDAF1E0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA280 size=76
    let mut pc: u32 = 0x82EFA280;
    'dispatch: loop {
        match pc {
            0x82EFA280 => {
    //   block [0x82EFA280..0x82EFA2B0)
	// 82EFA280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA29C: 4BFFF895  bl 0x82ef9b30
	ctx.lr = 0x82EFA2A0;
	sub_82EF9B30(ctx, base);
	// 82EFA2A0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA2A4: 4182000C  beq 0x82efa2b0
	if ctx.cr[0].eq {
	pc = 0x82EFA2B0; continue 'dispatch;
	}
	// 82EFA2A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA2AC: 480035F5  bl 0x82efd8a0
	ctx.lr = 0x82EFA2B0;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFA2B0; continue 'dispatch;
            }
            0x82EFA2B0 => {
    //   block [0x82EFA2B0..0x82EFA2CC)
	// 82EFA2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA2B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA2C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA2C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA2D0 size=76
    let mut pc: u32 = 0x82EFA2D0;
    'dispatch: loop {
        match pc {
            0x82EFA2D0 => {
    //   block [0x82EFA2D0..0x82EFA308)
	// 82EFA2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA2DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA2E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA2E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA2EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA2F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA2F4: 4E800421  bctrl
	ctx.lr = 0x82EFA2F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA2F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA2FC: 4182000C  beq 0x82efa308
	if ctx.cr[0].eq {
	pc = 0x82EFA308; continue 'dispatch;
	}
	// 82EFA300: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA304: 4BFFF975  bl 0x82ef9c78
	ctx.lr = 0x82EFA308;
	sub_82EF9C78(ctx, base);
            }
            0x82EFA308 => {
    //   block [0x82EFA308..0x82EFA31C)
	// 82EFA308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFA30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA320 size=360
    let mut pc: u32 = 0x82EFA320;
    'dispatch: loop {
        match pc {
            0x82EFA320 => {
    //   block [0x82EFA320..0x82EFA350)
	// 82EFA320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA324: 4BDAF0E1  bl 0x82ca9404
	ctx.lr = 0x82EFA328;
	sub_82CA93D0(ctx, base);
	// 82EFA328: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA32C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFA330: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFA334: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA338: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA33C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA340: 4E800421  bctrl
	ctx.lr = 0x82EFA344;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA344: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA348: 41820010  beq 0x82efa358
	if ctx.cr[0].eq {
	pc = 0x82EFA358; continue 'dispatch;
	}
	// 82EFA34C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
            }
            0x82EFA350 => {
    //   block [0x82EFA350..0x82EFA358)
	// 82EFA350: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82EFA354: 4BDAF100  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82EFA358 => {
    //   block [0x82EFA358..0x82EFA368)
	// 82EFA358: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFA35C: 409A000C  bne cr6, 0x82efa368
	if !ctx.cr[6].eq {
	pc = 0x82EFA368; continue 'dispatch;
	}
	// 82EFA360: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFA364: 4BFFFFEC  b 0x82efa350
	pc = 0x82EFA350; continue 'dispatch;
            }
            0x82EFA368 => {
    //   block [0x82EFA368..0x82EFA3A4)
	// 82EFA368: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA36C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA370: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA374: 4BFFF80D  bl 0x82ef9b80
	ctx.lr = 0x82EFA378;
	sub_82EF9B80(ctx, base);
	// 82EFA378: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82EFA37C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EFA380: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFA384: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EFA388: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EFA38C: 388AA2D0  addi r4, r10, -0x5d30
	ctx.r[4].s64 = ctx.r[10].s64 + -23856;
	// 82EFA390: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA394: 4BFFF20D  bl 0x82ef95a0
	ctx.lr = 0x82EFA398;
	sub_82EF95A0(ctx, base);
	// 82EFA398: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA39C: 40820018  bne 0x82efa3b4
	if !ctx.cr[0].eq {
	pc = 0x82EFA3B4; continue 'dispatch;
	}
	// 82EFA3A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82EFA3A4; continue 'dispatch;
            }
            0x82EFA3A4 => {
    //   block [0x82EFA3A4..0x82EFA3B4)
	// 82EFA3A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA3A8: 4BFFF861  bl 0x82ef9c08
	ctx.lr = 0x82EFA3AC;
	sub_82EF9C08(ctx, base);
	// 82EFA3AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA3B0: 4BFFFFA0  b 0x82efa350
	pc = 0x82EFA350; continue 'dispatch;
            }
            0x82EFA3B4 => {
    //   block [0x82EFA3B4..0x82EFA3EC)
	// 82EFA3B4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA3B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA3BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA3C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA3C4: 4E800421  bctrl
	ctx.lr = 0x82EFA3C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA3C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA3CC: 41820020  beq 0x82efa3ec
	if ctx.cr[0].eq {
	pc = 0x82EFA3EC; continue 'dispatch;
	}
	// 82EFA3D0: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA3D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EFA3D8: 388BA2D0  addi r4, r11, -0x5d30
	ctx.r[4].s64 = ctx.r[11].s64 + -23856;
	// 82EFA3DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA3E0: 4BFFF2B1  bl 0x82ef9690
	ctx.lr = 0x82EFA3E4;
	sub_82EF9690(ctx, base);
	// 82EFA3E4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82EFA3E8: 4BFFFFBC  b 0x82efa3a4
	pc = 0x82EFA3A4; continue 'dispatch;
            }
            0x82EFA3EC => {
    //   block [0x82EFA3EC..0x82EFA408)
	// 82EFA3EC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EFA3F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA3F4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82EFA3F8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA3FC: 419A000C  beq cr6, 0x82efa408
	if ctx.cr[6].eq {
	pc = 0x82EFA408; continue 'dispatch;
	}
	// 82EFA400: 48005171  bl 0x82eff570
	ctx.lr = 0x82EFA404;
	sub_82EFF570(ctx, base);
	// 82EFA404: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	pc = 0x82EFA408; continue 'dispatch;
            }
            0x82EFA408 => {
    //   block [0x82EFA408..0x82EFA410)
	// 82EFA408: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFA40C: 48000048  b 0x82efa454
	pc = 0x82EFA454; continue 'dispatch;
            }
            0x82EFA410 => {
    //   block [0x82EFA410..0x82EFA450)
	// 82EFA410: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA414: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA418: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA41C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA420: 4E800421  bctrl
	ctx.lr = 0x82EFA424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA424: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA428: 40820040  bne 0x82efa468
	if !ctx.cr[0].eq {
	pc = 0x82EFA468; continue 'dispatch;
	}
	// 82EFA42C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA430: 419A0020  beq cr6, 0x82efa450
	if ctx.cr[6].eq {
	pc = 0x82EFA450; continue 'dispatch;
	}
	// 82EFA434: 4800513D  bl 0x82eff570
	ctx.lr = 0x82EFA438;
	sub_82EFF570(ctx, base);
	// 82EFA438: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA43C: 57AA003E  slwi r10, r29, 0
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFA440: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFA444: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EFA448: 40980024  bge cr6, 0x82efa46c
	if !ctx.cr[6].lt {
	pc = 0x82EFA46C; continue 'dispatch;
	}
	// 82EFA44C: 7FCBF850  subf r30, r11, r31
	ctx.r[30].s64 = ctx.r[31].s64 - ctx.r[11].s64;
            }
            0x82EFA450 => {
    //   block [0x82EFA450..0x82EFA454)
	// 82EFA450: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	pc = 0x82EFA454; continue 'dispatch;
            }
            0x82EFA454 => {
    //   block [0x82EFA454..0x82EFA468)
	// 82EFA454: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA458: 4BFFEAE9  bl 0x82ef8f40
	ctx.lr = 0x82EFA45C;
	sub_82EF8F40(ctx, base);
	// 82EFA45C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA460: 4082FFB0  bne 0x82efa410
	if !ctx.cr[0].eq {
	pc = 0x82EFA410; continue 'dispatch;
	}
	// 82EFA464: 48000008  b 0x82efa46c
	pc = 0x82EFA46C; continue 'dispatch;
            }
            0x82EFA468 => {
    //   block [0x82EFA468..0x82EFA46C)
	// 82EFA468: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	pc = 0x82EFA46C; continue 'dispatch;
            }
            0x82EFA46C => {
    //   block [0x82EFA46C..0x82EFA488)
	// 82EFA46C: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA470: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EFA474: 388BA2D0  addi r4, r11, -0x5d30
	ctx.r[4].s64 = ctx.r[11].s64 + -23856;
	// 82EFA478: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA47C: 4BFFF215  bl 0x82ef9690
	ctx.lr = 0x82EFA480;
	sub_82EF9690(ctx, base);
	// 82EFA480: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82EFA484: 4BFFFF20  b 0x82efa3a4
	pc = 0x82EFA3A4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA488 size=104
    let mut pc: u32 = 0x82EFA488;
    'dispatch: loop {
        match pc {
            0x82EFA488 => {
    //   block [0x82EFA488..0x82EFA4AC)
	// 82EFA488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA48C: 4BDAEF81  bl 0x82ca940c
	ctx.lr = 0x82EFA490;
	sub_82CA93D0(ctx, base);
	// 82EFA490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA494: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA498: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA49C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA4A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFA4A4: 4099003C  ble cr6, 0x82efa4e0
	if !ctx.cr[6].gt {
	pc = 0x82EFA4E0; continue 'dispatch;
	}
	// 82EFA4A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82EFA4AC; continue 'dispatch;
            }
            0x82EFA4AC => {
    //   block [0x82EFA4AC..0x82EFA4E0)
	// 82EFA4AC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFA4B0: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EFA4B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA4B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA4BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA4C0: 4E800421  bctrl
	ctx.lr = 0x82EFA4C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA4C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA4C8: 41820020  beq 0x82efa4e8
	if ctx.cr[0].eq {
	pc = 0x82EFA4E8; continue 'dispatch;
	}
	// 82EFA4CC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA4D0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EFA4D4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EFA4D8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA4DC: 4198FFD0  blt cr6, 0x82efa4ac
	if ctx.cr[6].lt {
	pc = 0x82EFA4AC; continue 'dispatch;
	}
            }
            0x82EFA4E0 => {
    //   block [0x82EFA4E0..0x82EFA4E8)
	// 82EFA4E0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA4E4: 4BFFF795  bl 0x82ef9c78
	ctx.lr = 0x82EFA4E8;
	sub_82EF9C78(ctx, base);
	pc = 0x82EFA4E8; continue 'dispatch;
            }
            0x82EFA4E8 => {
    //   block [0x82EFA4E8..0x82EFA4F0)
	// 82EFA4E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA4EC: 4BDAEF70  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA4F0 size=424
    let mut pc: u32 = 0x82EFA4F0;
    'dispatch: loop {
        match pc {
            0x82EFA4F0 => {
    //   block [0x82EFA4F0..0x82EFA52C)
	// 82EFA4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA4F4: 4BDAEF11  bl 0x82ca9404
	ctx.lr = 0x82EFA4F8;
	sub_82CA93D0(ctx, base);
	// 82EFA4F8: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA4FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA500: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA504: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFA508: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFA50C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFA510: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82EFA514: 4BFFE85D  bl 0x82ef8d70
	ctx.lr = 0x82EFA518;
	sub_82EF8D70(ctx, base);
	// 82EFA518: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA51C: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA520: 4BFFE8C9  bl 0x82ef8de8
	ctx.lr = 0x82EFA524;
	sub_82EF8DE8(ctx, base);
	// 82EFA524: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA528: 41820020  beq 0x82efa548
	if ctx.cr[0].eq {
	pc = 0x82EFA548; continue 'dispatch;
	}
	pc = 0x82EFA52C; continue 'dispatch;
            }
            0x82EFA52C => {
    //   block [0x82EFA52C..0x82EFA540)
	// 82EFA52C: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA530: 396100A4  addi r11, r1, 0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + 164;
	// 82EFA534: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA538: 419A0008  beq cr6, 0x82efa540
	if ctx.cr[6].eq {
	pc = 0x82EFA540; continue 'dispatch;
	}
	// 82EFA53C: 48003365  bl 0x82efd8a0
	ctx.lr = 0x82EFA540;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFA540; continue 'dispatch;
            }
            0x82EFA540 => {
    //   block [0x82EFA540..0x82EFA548)
	// 82EFA540: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFA544: 4800014C  b 0x82efa690
	pc = 0x82EFA690; continue 'dispatch;
            }
            0x82EFA548 => {
    //   block [0x82EFA548..0x82EFA550)
	// 82EFA548: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFA54C: 409A0020  bne cr6, 0x82efa56c
	if !ctx.cr[6].eq {
	pc = 0x82EFA56C; continue 'dispatch;
	}
	pc = 0x82EFA550; continue 'dispatch;
            }
            0x82EFA550 => {
    //   block [0x82EFA550..0x82EFA564)
	// 82EFA550: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA554: 396100A4  addi r11, r1, 0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + 164;
	// 82EFA558: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA55C: 419A0008  beq cr6, 0x82efa564
	if ctx.cr[6].eq {
	pc = 0x82EFA564; continue 'dispatch;
	}
	// 82EFA560: 48003341  bl 0x82efd8a0
	ctx.lr = 0x82EFA564;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFA564; continue 'dispatch;
            }
            0x82EFA564 => {
    //   block [0x82EFA564..0x82EFA56C)
	// 82EFA564: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFA568: 48000128  b 0x82efa690
	pc = 0x82EFA690; continue 'dispatch;
            }
            0x82EFA56C => {
    //   block [0x82EFA56C..0x82EFA5B8)
	// 82EFA56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA570: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA574: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA578: 4BFFF609  bl 0x82ef9b80
	ctx.lr = 0x82EFA57C;
	sub_82EF9B80(ctx, base);
	// 82EFA57C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82EFA580: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82EFA584: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFA588: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EFA58C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA590: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82EFA594: 388AA488  addi r4, r10, -0x5b78
	ctx.r[4].s64 = ctx.r[10].s64 + -23416;
	// 82EFA598: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA59C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82EFA5A0: 4BFFF1A1  bl 0x82ef9740
	ctx.lr = 0x82EFA5A4;
	sub_82EF9740(ctx, base);
	// 82EFA5A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA5A8: 40820010  bne 0x82efa5b8
	if !ctx.cr[0].eq {
	pc = 0x82EFA5B8; continue 'dispatch;
	}
	// 82EFA5AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA5B0: 4BFFF659  bl 0x82ef9c08
	ctx.lr = 0x82EFA5B4;
	sub_82EF9C08(ctx, base);
	// 82EFA5B4: 4BFFFF9C  b 0x82efa550
	pc = 0x82EFA550; continue 'dispatch;
            }
            0x82EFA5B8 => {
    //   block [0x82EFA5B8..0x82EFA5E8)
	// 82EFA5B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA5BC: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA5C0: 4BFFE829  bl 0x82ef8de8
	ctx.lr = 0x82EFA5C4;
	sub_82EF8DE8(ctx, base);
	// 82EFA5C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA5C8: 41820020  beq 0x82efa5e8
	if ctx.cr[0].eq {
	pc = 0x82EFA5E8; continue 'dispatch;
	}
	// 82EFA5CC: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA5D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA5D4: 388BA488  addi r4, r11, -0x5b78
	ctx.r[4].s64 = ctx.r[11].s64 + -23416;
	// 82EFA5D8: 4BFFF209  bl 0x82ef97e0
	ctx.lr = 0x82EFA5DC;
	sub_82EF97E0(ctx, base);
	// 82EFA5DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA5E0: 4BFFF629  bl 0x82ef9c08
	ctx.lr = 0x82EFA5E4;
	sub_82EF9C08(ctx, base);
	// 82EFA5E4: 4BFFFF48  b 0x82efa52c
	pc = 0x82EFA52C; continue 'dispatch;
            }
            0x82EFA5E8 => {
    //   block [0x82EFA5E8..0x82EFA604)
	// 82EFA5E8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EFA5EC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA5F0: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82EFA5F4: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA5F8: 419A000C  beq cr6, 0x82efa604
	if ctx.cr[6].eq {
	pc = 0x82EFA604; continue 'dispatch;
	}
	// 82EFA5FC: 48004F75  bl 0x82eff570
	ctx.lr = 0x82EFA600;
	sub_82EFF570(ctx, base);
	// 82EFA600: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	pc = 0x82EFA604; continue 'dispatch;
            }
            0x82EFA604 => {
    //   block [0x82EFA604..0x82EFA60C)
	// 82EFA604: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFA608: 48000040  b 0x82efa648
	pc = 0x82EFA648; continue 'dispatch;
            }
            0x82EFA60C => {
    //   block [0x82EFA60C..0x82EFA644)
	// 82EFA60C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA610: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA614: 4BFFE7D5  bl 0x82ef8de8
	ctx.lr = 0x82EFA618;
	sub_82EF8DE8(ctx, base);
	// 82EFA618: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA61C: 40820040  bne 0x82efa65c
	if !ctx.cr[0].eq {
	pc = 0x82EFA65C; continue 'dispatch;
	}
	// 82EFA620: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA624: 419A0020  beq cr6, 0x82efa644
	if ctx.cr[6].eq {
	pc = 0x82EFA644; continue 'dispatch;
	}
	// 82EFA628: 48004F49  bl 0x82eff570
	ctx.lr = 0x82EFA62C;
	sub_82EFF570(ctx, base);
	// 82EFA62C: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA630: 57AA003E  slwi r10, r29, 0
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFA634: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFA638: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EFA63C: 40980024  bge cr6, 0x82efa660
	if !ctx.cr[6].lt {
	pc = 0x82EFA660; continue 'dispatch;
	}
	// 82EFA640: 7FCBF850  subf r30, r11, r31
	ctx.r[30].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	pc = 0x82EFA644; continue 'dispatch;
            }
            0x82EFA644 => {
    //   block [0x82EFA644..0x82EFA648)
	// 82EFA644: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	pc = 0x82EFA648; continue 'dispatch;
            }
            0x82EFA648 => {
    //   block [0x82EFA648..0x82EFA65C)
	// 82EFA648: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA64C: 4BFFE8F5  bl 0x82ef8f40
	ctx.lr = 0x82EFA650;
	sub_82EF8F40(ctx, base);
	// 82EFA650: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA654: 4082FFB8  bne 0x82efa60c
	if !ctx.cr[0].eq {
	pc = 0x82EFA60C; continue 'dispatch;
	}
	// 82EFA658: 48000008  b 0x82efa660
	pc = 0x82EFA660; continue 'dispatch;
            }
            0x82EFA65C => {
    //   block [0x82EFA65C..0x82EFA660)
	// 82EFA65C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	pc = 0x82EFA660; continue 'dispatch;
            }
            0x82EFA660 => {
    //   block [0x82EFA660..0x82EFA68C)
	// 82EFA660: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA664: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA668: 388BA488  addi r4, r11, -0x5b78
	ctx.r[4].s64 = ctx.r[11].s64 + -23416;
	// 82EFA66C: 4BFFF175  bl 0x82ef97e0
	ctx.lr = 0x82EFA670;
	sub_82EF97E0(ctx, base);
	// 82EFA670: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA674: 4BFFF595  bl 0x82ef9c08
	ctx.lr = 0x82EFA678;
	sub_82EF9C08(ctx, base);
	// 82EFA678: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA67C: 396100A4  addi r11, r1, 0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + 164;
	// 82EFA680: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA684: 419A0008  beq cr6, 0x82efa68c
	if ctx.cr[6].eq {
	pc = 0x82EFA68C; continue 'dispatch;
	}
	// 82EFA688: 48003219  bl 0x82efd8a0
	ctx.lr = 0x82EFA68C;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFA68C; continue 'dispatch;
            }
            0x82EFA68C => {
    //   block [0x82EFA68C..0x82EFA690)
	// 82EFA68C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	pc = 0x82EFA690; continue 'dispatch;
            }
            0x82EFA690 => {
    //   block [0x82EFA690..0x82EFA698)
	// 82EFA690: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82EFA694: 4BDAEDC0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA698 size=124
    let mut pc: u32 = 0x82EFA698;
    'dispatch: loop {
        match pc {
            0x82EFA698 => {
    //   block [0x82EFA698..0x82EFA6BC)
	// 82EFA698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA69C: 4BDAED71  bl 0x82ca940c
	ctx.lr = 0x82EFA6A0;
	sub_82CA93D0(ctx, base);
	// 82EFA6A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA6A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA6A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA6AC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA6B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFA6B4: 40990058  ble cr6, 0x82efa70c
	if !ctx.cr[6].gt {
	pc = 0x82EFA70C; continue 'dispatch;
	}
	// 82EFA6B8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82EFA6BC; continue 'dispatch;
            }
            0x82EFA6BC => {
    //   block [0x82EFA6BC..0x82EFA704)
	// 82EFA6BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA6C0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFA6C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA6C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA6CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA6D0: 4E800421  bctrl
	ctx.lr = 0x82EFA6D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA6D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA6D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA6E0: 4E800421  bctrl
	ctx.lr = 0x82EFA6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA6E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA6E8: 4082001C  bne 0x82efa704
	if !ctx.cr[0].eq {
	pc = 0x82EFA704; continue 'dispatch;
	}
	// 82EFA6EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA6F0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EFA6F4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EFA6F8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA6FC: 4198FFC0  blt cr6, 0x82efa6bc
	if ctx.cr[6].lt {
	pc = 0x82EFA6BC; continue 'dispatch;
	}
	// 82EFA700: 4800000C  b 0x82efa70c
	pc = 0x82EFA70C; continue 'dispatch;
            }
            0x82EFA704 => {
    //   block [0x82EFA704..0x82EFA70C)
	// 82EFA704: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA708: 4BFFF571  bl 0x82ef9c78
	ctx.lr = 0x82EFA70C;
	sub_82EF9C78(ctx, base);
	pc = 0x82EFA70C; continue 'dispatch;
            }
            0x82EFA70C => {
    //   block [0x82EFA70C..0x82EFA714)
	// 82EFA70C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA710: 4BDAED4C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA718 size=316
    let mut pc: u32 = 0x82EFA718;
    'dispatch: loop {
        match pc {
            0x82EFA718 => {
    //   block [0x82EFA718..0x82EFA744)
	// 82EFA718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA71C: 4BDAECE5  bl 0x82ca9400
	ctx.lr = 0x82EFA720;
	sub_82CA93D0(ctx, base);
	// 82EFA720: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA724: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFA728: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA72C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFA730: 4BFFE789  bl 0x82ef8eb8
	ctx.lr = 0x82EFA734;
	sub_82EF8EB8(ctx, base);
	// 82EFA734: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EFA738: 409A000C  bne cr6, 0x82efa744
	if !ctx.cr[6].eq {
	pc = 0x82EFA744; continue 'dispatch;
	}
	// 82EFA73C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFA740: 409A000C  bne cr6, 0x82efa74c
	if !ctx.cr[6].eq {
	pc = 0x82EFA74C; continue 'dispatch;
	}
	pc = 0x82EFA744; continue 'dispatch;
            }
            0x82EFA744 => {
    //   block [0x82EFA744..0x82EFA74C)
	// 82EFA744: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82EFA748: 4BDAED08  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x82EFA74C => {
    //   block [0x82EFA74C..0x82EFA790)
	// 82EFA74C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA750: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA754: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA758: 4BFFF429  bl 0x82ef9b80
	ctx.lr = 0x82EFA75C;
	sub_82EF9B80(ctx, base);
	// 82EFA75C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82EFA760: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EFA764: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82EFA768: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82EFA76C: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFA770: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EFA774: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA778: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82EFA77C: 388AA698  addi r4, r10, -0x5968
	ctx.r[4].s64 = ctx.r[10].s64 + -22888;
	// 82EFA780: 4BFFEFC1  bl 0x82ef9740
	ctx.lr = 0x82EFA784;
	sub_82EF9740(ctx, base);
	// 82EFA784: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA788: 40820018  bne 0x82efa7a0
	if !ctx.cr[0].eq {
	pc = 0x82EFA7A0; continue 'dispatch;
	}
	// 82EFA78C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	pc = 0x82EFA790; continue 'dispatch;
            }
            0x82EFA790 => {
    //   block [0x82EFA790..0x82EFA7A0)
	// 82EFA790: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA794: 4BFFF475  bl 0x82ef9c08
	ctx.lr = 0x82EFA798;
	sub_82EF9C08(ctx, base);
	// 82EFA798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA79C: 4BFFFFA8  b 0x82efa744
	pc = 0x82EFA744; continue 'dispatch;
            }
            0x82EFA7A0 => {
    //   block [0x82EFA7A0..0x82EFA7B8)
	// 82EFA7A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA7A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA7A8: 4BFFE711  bl 0x82ef8eb8
	ctx.lr = 0x82EFA7AC;
	sub_82EF8EB8(ctx, base);
	// 82EFA7AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA7B0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA7B4: 419A0018  beq cr6, 0x82efa7cc
	if ctx.cr[6].eq {
	pc = 0x82EFA7CC; continue 'dispatch;
	}
	pc = 0x82EFA7B8; continue 'dispatch;
            }
            0x82EFA7B8 => {
    //   block [0x82EFA7B8..0x82EFA7CC)
	// 82EFA7B8: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA7BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA7C0: 388BA698  addi r4, r11, -0x5968
	ctx.r[4].s64 = ctx.r[11].s64 + -22888;
	// 82EFA7C4: 4BFFF01D  bl 0x82ef97e0
	ctx.lr = 0x82EFA7C8;
	sub_82EF97E0(ctx, base);
	// 82EFA7C8: 4BFFFFC8  b 0x82efa790
	pc = 0x82EFA790; continue 'dispatch;
            }
            0x82EFA7CC => {
    //   block [0x82EFA7CC..0x82EFA7E0)
	// 82EFA7CC: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82EFA7D0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EFA7D4: 419A000C  beq cr6, 0x82efa7e0
	if ctx.cr[6].eq {
	pc = 0x82EFA7E0; continue 'dispatch;
	}
	// 82EFA7D8: 48004D99  bl 0x82eff570
	ctx.lr = 0x82EFA7DC;
	sub_82EFF570(ctx, base);
	// 82EFA7DC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	pc = 0x82EFA7E0; continue 'dispatch;
            }
            0x82EFA7E0 => {
    //   block [0x82EFA7E0..0x82EFA800)
	// 82EFA7E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA7E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA7E8: 4BFFE6D1  bl 0x82ef8eb8
	ctx.lr = 0x82EFA7EC;
	sub_82EF8EB8(ctx, base);
	// 82EFA7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA7F0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA7F4: 409AFFC4  bne cr6, 0x82efa7b8
	if !ctx.cr[6].eq {
	pc = 0x82EFA7B8; continue 'dispatch;
	}
	// 82EFA7F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFA7FC: 48000044  b 0x82efa840
	pc = 0x82EFA840; continue 'dispatch;
            }
            0x82EFA800 => {
    //   block [0x82EFA800..0x82EFA83C)
	// 82EFA800: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA804: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA808: 4BFFE6B1  bl 0x82ef8eb8
	ctx.lr = 0x82EFA80C;
	sub_82EF8EB8(ctx, base);
	// 82EFA80C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA810: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA814: 409AFFA4  bne cr6, 0x82efa7b8
	if !ctx.cr[6].eq {
	pc = 0x82EFA7B8; continue 'dispatch;
	}
	// 82EFA818: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EFA81C: 419A0020  beq cr6, 0x82efa83c
	if ctx.cr[6].eq {
	pc = 0x82EFA83C; continue 'dispatch;
	}
	// 82EFA820: 48004D51  bl 0x82eff570
	ctx.lr = 0x82EFA824;
	sub_82EFF570(ctx, base);
	// 82EFA824: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA828: 574A003E  slwi r10, r26, 0
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFA82C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFA830: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EFA834: 4098FF84  bge cr6, 0x82efa7b8
	if !ctx.cr[6].lt {
	pc = 0x82EFA7B8; continue 'dispatch;
	}
	// 82EFA838: 7F6BF050  subf r27, r11, r30
	ctx.r[27].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	pc = 0x82EFA83C; continue 'dispatch;
            }
            0x82EFA83C => {
    //   block [0x82EFA83C..0x82EFA840)
	// 82EFA83C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	pc = 0x82EFA840; continue 'dispatch;
            }
            0x82EFA840 => {
    //   block [0x82EFA840..0x82EFA854)
	// 82EFA840: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA844: 4BFFE6FD  bl 0x82ef8f40
	ctx.lr = 0x82EFA848;
	sub_82EF8F40(ctx, base);
	// 82EFA848: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA84C: 4082FFB4  bne 0x82efa800
	if !ctx.cr[0].eq {
	pc = 0x82EFA800; continue 'dispatch;
	}
	// 82EFA850: 4BFFFF68  b 0x82efa7b8
	pc = 0x82EFA7B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA858 size=112
    let mut pc: u32 = 0x82EFA858;
    'dispatch: loop {
        match pc {
            0x82EFA858 => {
    //   block [0x82EFA858..0x82EFA8C0)
	// 82EFA858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA85C: 4BDAEBB1  bl 0x82ca940c
	ctx.lr = 0x82EFA860;
	sub_82CA93D0(ctx, base);
	// 82EFA860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA868: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82EFA86C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA870: 480002E9  bl 0x82efab58
	ctx.lr = 0x82EFA874;
	sub_82EFAB58(ctx, base);
	// 82EFA874: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFA878: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA87C: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EFA880: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EFA884: 9BBF0019  stb r29, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 82EFA888: 48000789  bl 0x82efb010
	ctx.lr = 0x82EFA88C;
	sub_82EFB010(ctx, base);
	// 82EFA88C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82EFA890: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFA894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA898: 4BFFF221  bl 0x82ef9ab8
	ctx.lr = 0x82EFA89C;
	sub_82EF9AB8(ctx, base);
	// 82EFA89C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA8A0: 48001201  bl 0x82efbaa0
	ctx.lr = 0x82EFA8A4;
	sub_82EFBAA0(ctx, base);
	// 82EFA8A4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFA8A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFA8AC: 419A0014  beq cr6, 0x82efa8c0
	if ctx.cr[6].eq {
	pc = 0x82EFA8C0; continue 'dispatch;
	}
	// 82EFA8B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8B4: 4BFFF0BD  bl 0x82ef9970
	ctx.lr = 0x82EFA8B8;
	sub_82EF9970(ctx, base);
	// 82EFA8B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8BC: 4BFFF18D  bl 0x82ef9a48
	ctx.lr = 0x82EFA8C0;
	sub_82EF9A48(ctx, base);
	pc = 0x82EFA8C0; continue 'dispatch;
            }
            0x82EFA8C0 => {
    //   block [0x82EFA8C0..0x82EFA8C8)
	// 82EFA8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFA8C4: 4BDAEB98  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA8C8 size=76
    let mut pc: u32 = 0x82EFA8C8;
    'dispatch: loop {
        match pc {
            0x82EFA8C8 => {
    //   block [0x82EFA8C8..0x82EFA8F8)
	// 82EFA8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA8DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA8E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA8E4: 4BFFF785  bl 0x82efa068
	ctx.lr = 0x82EFA8E8;
	sub_82EFA068(ctx, base);
	// 82EFA8E8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA8EC: 4182000C  beq 0x82efa8f8
	if ctx.cr[0].eq {
	pc = 0x82EFA8F8; continue 'dispatch;
	}
	// 82EFA8F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8F4: 48002FAD  bl 0x82efd8a0
	ctx.lr = 0x82EFA8F8;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFA8F8; continue 'dispatch;
            }
            0x82EFA8F8 => {
    //   block [0x82EFA8F8..0x82EFA914)
	// 82EFA8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA908: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA90C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA918 size=8
    let mut pc: u32 = 0x82EFA918;
    'dispatch: loop {
        match pc {
            0x82EFA918 => {
    //   block [0x82EFA918..0x82EFA920)
	// 82EFA918: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFA91C: 4BFFF8DC  b 0x82efa1f8
	sub_82EFA1F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA920 size=8
    let mut pc: u32 = 0x82EFA920;
    'dispatch: loop {
        match pc {
            0x82EFA920 => {
    //   block [0x82EFA920..0x82EFA928)
	// 82EFA920: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFA924: 4BFFF84C  b 0x82efa170
	sub_82EFA170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA928 size=56
    let mut pc: u32 = 0x82EFA928;
    'dispatch: loop {
        match pc {
            0x82EFA928 => {
    //   block [0x82EFA928..0x82EFA960)
	// 82EFA928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA934: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EFA938: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EFA93C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFA940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA944: 4BFFFDD5  bl 0x82efa718
	ctx.lr = 0x82EFA948;
	sub_82EFA718(ctx, base);
	// 82EFA948: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EFA94C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFA950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFA954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA960 size=24
    let mut pc: u32 = 0x82EFA960;
    'dispatch: loop {
        match pc {
            0x82EFA960 => {
    //   block [0x82EFA960..0x82EFA978)
	// 82EFA960: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA964: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EFA968: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFA96C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA970: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA974: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA978 size=24
    let mut pc: u32 = 0x82EFA978;
    'dispatch: loop {
        match pc {
            0x82EFA978 => {
    //   block [0x82EFA978..0x82EFA990)
	// 82EFA978: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA97C: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EFA980: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFA984: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFA988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA98C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA990 size=24
    let mut pc: u32 = 0x82EFA990;
    'dispatch: loop {
        match pc {
            0x82EFA990 => {
    //   block [0x82EFA990..0x82EFA9A8)
	// 82EFA990: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA994: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EFA998: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFA99C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFA9A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA9A4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA9A8 size=124
    let mut pc: u32 = 0x82EFA9A8;
    'dispatch: loop {
        match pc {
            0x82EFA9A8 => {
    //   block [0x82EFA9A8..0x82EFA9F8)
	// 82EFA9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA9B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA9B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA9B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA9BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFA9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFA9C4: 394AD02C  addi r10, r10, -0x2fd4
	ctx.r[10].s64 = ctx.r[10].s64 + -12244;
	// 82EFA9C8: 54A9063F  clrlwi. r9, r5, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EFA9CC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EFA9D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFA9D4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EFA9D8: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82EFA9DC: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82EFA9E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFA9E4: 41820014  beq 0x82efa9f8
	if ctx.cr[0].eq {
	pc = 0x82EFA9F8; continue 'dispatch;
	}
	// 82EFA9E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA9EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA9F0: 48110F49  bl 0x8300b938
	ctx.lr = 0x82EFA9F4;
	sub_8300B938(ctx, base);
	// 82EFA9F4: 48000014  b 0x82efaa08
	pc = 0x82EFAA08; continue 'dispatch;
            }
            0x82EFA9F8 => {
    //   block [0x82EFA9F8..0x82EFAA08)
	// 82EFA9F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EFA9FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EFAA00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFAA04: 4BDCC4AD  bl 0x82cc6eb0
	ctx.lr = 0x82EFAA08;
	sub_82CC6EB0(ctx, base);
	pc = 0x82EFAA08; continue 'dispatch;
            }
            0x82EFAA08 => {
    //   block [0x82EFAA08..0x82EFAA24)
	// 82EFAA08: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFAA0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAA1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAA28 size=72
    let mut pc: u32 = 0x82EFAA28;
    'dispatch: loop {
        match pc {
            0x82EFAA28 => {
    //   block [0x82EFAA28..0x82EFAA5C)
	// 82EFAA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAA30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAA34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAA38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAA3C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EFAA40: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAA44: 4B29BF85  bl 0x821969c8
	ctx.lr = 0x82EFAA48;
	sub_821969C8(ctx, base);
	// 82EFAA48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAA4C: 40820010  bne 0x82efaa5c
	if !ctx.cr[0].eq {
	pc = 0x82EFAA5C; continue 'dispatch;
	}
	// 82EFAA50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAA54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFAA58: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFAA5C; continue 'dispatch;
            }
            0x82EFAA5C => {
    //   block [0x82EFAA5C..0x82EFAA70)
	// 82EFAA5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAA68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAA70 size=84
    let mut pc: u32 = 0x82EFAA70;
    'dispatch: loop {
        match pc {
            0x82EFAA70 => {
    //   block [0x82EFAA70..0x82EFAAA0)
	// 82EFAA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAA78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAA7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAA80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAA84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFAA88: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAA8C: 4B29BF3D  bl 0x821969c8
	ctx.lr = 0x82EFAA90;
	sub_821969C8(ctx, base);
	// 82EFAA90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAA94: 4182000C  beq 0x82efaaa0
	if ctx.cr[0].eq {
	pc = 0x82EFAAA0; continue 'dispatch;
	}
	// 82EFAA98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAA9C: 48000014  b 0x82efaab0
	pc = 0x82EFAAB0; continue 'dispatch;
            }
            0x82EFAAA0 => {
    //   block [0x82EFAAA0..0x82EFAAB0)
	// 82EFAAA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAAA4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFAAA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFAAAC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFAAB0; continue 'dispatch;
            }
            0x82EFAAB0 => {
    //   block [0x82EFAAB0..0x82EFAAC4)
	// 82EFAAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAAC8 size=140
    let mut pc: u32 = 0x82EFAAC8;
    'dispatch: loop {
        match pc {
            0x82EFAAC8 => {
    //   block [0x82EFAAC8..0x82EFAB30)
	// 82EFAAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAAD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFAAD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAAD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAADC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFAAE0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EFAAE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAAE8: 4BFFEDA1  bl 0x82ef9888
	ctx.lr = 0x82EFAAEC;
	sub_82EF9888(ctx, base);
	// 82EFAAEC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFAAF0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFAAF4: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFAAF8: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFAAFC: 394AD060  addi r10, r10, -0x2fa0
	ctx.r[10].s64 = ctx.r[10].s64 + -12192;
	// 82EFAB00: 3929D04C  addi r9, r9, -0x2fb4
	ctx.r[9].s64 = ctx.r[9].s64 + -12212;
	// 82EFAB04: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFAB08: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFAB0C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82EFAB10: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFAB14: 48002D4D  bl 0x82efd860
	ctx.lr = 0x82EFAB18;
	sub_82EFD860(ctx, base);
	// 82EFAB18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAB1C: 41820014  beq 0x82efab30
	if ctx.cr[0].eq {
	pc = 0x82EFAB30; continue 'dispatch;
	}
	// 82EFAB20: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFAB24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFAB28: 4BFFFE81  bl 0x82efa9a8
	ctx.lr = 0x82EFAB2C;
	sub_82EFA9A8(ctx, base);
	// 82EFAB2C: 48000008  b 0x82efab34
	pc = 0x82EFAB34; continue 'dispatch;
            }
            0x82EFAB30 => {
    //   block [0x82EFAB30..0x82EFAB34)
	// 82EFAB30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFAB34; continue 'dispatch;
            }
            0x82EFAB34 => {
    //   block [0x82EFAB34..0x82EFAB54)
	// 82EFAB34: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82EFAB38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAB3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFAB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAB48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFAB4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAB58 size=8
    let mut pc: u32 = 0x82EFAB58;
    'dispatch: loop {
        match pc {
            0x82EFAB58 => {
    //   block [0x82EFAB58..0x82EFAB60)
	// 82EFAB58: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAB5C: 4BFFFECC  b 0x82efaa28
	sub_82EFAA28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAB68 size=20
    let mut pc: u32 = 0x82EFAB68;
    'dispatch: loop {
        match pc {
            0x82EFAB68 => {
    //   block [0x82EFAB68..0x82EFAB7C)
	// 82EFAB68: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAB6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAB70: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFAB74: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFAB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAB80 size=8
    let mut pc: u32 = 0x82EFAB80;
    'dispatch: loop {
        match pc {
            0x82EFAB80 => {
    //   block [0x82EFAB80..0x82EFAB88)
	// 82EFAB80: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAB84: 4BFFFEEC  b 0x82efaa70
	sub_82EFAA70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAB88 size=68
    let mut pc: u32 = 0x82EFAB88;
    'dispatch: loop {
        match pc {
            0x82EFAB88 => {
    //   block [0x82EFAB88..0x82EFABCC)
	// 82EFAB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAB90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAB94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAB98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFAB9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFABA0: 4BFFE1A1  bl 0x82ef8d40
	ctx.lr = 0x82EFABA4;
	sub_82EF8D40(ctx, base);
	// 82EFABA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFABA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFABAC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFABB0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFABB4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EFABB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFABBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFABC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFABC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFABC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFABD0 size=88
    let mut pc: u32 = 0x82EFABD0;
    'dispatch: loop {
        match pc {
            0x82EFABD0 => {
    //   block [0x82EFABD0..0x82EFABE8)
	// 82EFABD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFABD4: 4BDAE839  bl 0x82ca940c
	ctx.lr = 0x82EFABD8;
	sub_82CA93D0(ctx, base);
	// 82EFABD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFABDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFABE0: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFABE4: 4800001C  b 0x82efac00
	pc = 0x82EFAC00; continue 'dispatch;
            }
            0x82EFABE8 => {
    //   block [0x82EFABE8..0x82EFAC00)
	// 82EFABE8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82EFABEC: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFABF0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFABF4: 4BDC7BBD  bl 0x82cc27b0
	ctx.lr = 0x82EFABF8;
	sub_82CC27B0(ctx, base);
	// 82EFABF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFABFC: 48002CA5  bl 0x82efd8a0
	ctx.lr = 0x82EFAC00;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFAC00; continue 'dispatch;
            }
            0x82EFAC00 => {
    //   block [0x82EFAC00..0x82EFAC28)
	// 82EFAC00: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFAC04: 409AFFE4  bne cr6, 0x82efabe8
	if !ctx.cr[6].eq {
	pc = 0x82EFABE8; continue 'dispatch;
	}
	// 82EFAC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFAC0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAC10: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFAC14: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFAC18: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EFAC1C: 4800041D  bl 0x82efb038
	ctx.lr = 0x82EFAC20;
	sub_82EFB038(ctx, base);
	// 82EFAC20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFAC24: 4BDAE838  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAC28 size=112
    let mut pc: u32 = 0x82EFAC28;
    'dispatch: loop {
        match pc {
            0x82EFAC28 => {
    //   block [0x82EFAC28..0x82EFAC50)
	// 82EFAC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAC30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAC34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAC38: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFAC3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFAC40: 419A0010  beq cr6, 0x82efac50
	if ctx.cr[6].eq {
	pc = 0x82EFAC50; continue 'dispatch;
	}
	// 82EFAC44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAC48: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFAC4C: 48000034  b 0x82efac80
	pc = 0x82EFAC80; continue 'dispatch;
            }
            0x82EFAC50 => {
    //   block [0x82EFAC50..0x82EFAC80)
	// 82EFAC50: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82EFAC54: 48002C0D  bl 0x82efd860
	ctx.lr = 0x82EFAC58;
	sub_82EFD860(ctx, base);
	// 82EFAC58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAC5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFAC60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EFAC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFAC68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFAC6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFAC70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAC74: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFAC78: 4BDC8069  bl 0x82cc2ce0
	ctx.lr = 0x82EFAC7C;
	sub_82CC2CE0(ctx, base);
	// 82EFAC7C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x82EFAC80; continue 'dispatch;
            }
            0x82EFAC80 => {
    //   block [0x82EFAC80..0x82EFAC98)
	// 82EFAC80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAC84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAC90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAC98 size=80
    let mut pc: u32 = 0x82EFAC98;
    'dispatch: loop {
        match pc {
            0x82EFAC98 => {
    //   block [0x82EFAC98..0x82EFACE8)
	// 82EFAC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFACA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFACA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFACA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFACAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFACB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFACB4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFACB8: 4B292F61  bl 0x8218dc18
	ctx.lr = 0x82EFACBC;
	sub_8218DC18(ctx, base);
	// 82EFACBC: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFACC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFACC4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFACC8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EFACCC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82EFACD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFACD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFACD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFACDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFACE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFACE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFACE8 size=32
    let mut pc: u32 = 0x82EFACE8;
    'dispatch: loop {
        match pc {
            0x82EFACE8 => {
    //   block [0x82EFACE8..0x82EFACF0)
	// 82EFACE8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFACEC: 48000010  b 0x82efacfc
	pc = 0x82EFACFC; continue 'dispatch;
            }
            0x82EFACF0 => {
    //   block [0x82EFACF0..0x82EFACFC)
	// 82EFACF0: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EFACF4: 419A0014  beq cr6, 0x82efad08
	if ctx.cr[6].eq {
		sub_82EFAD08(ctx, base);
		return;
	}
	// 82EFACF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x82EFACFC; continue 'dispatch;
            }
            0x82EFACFC => {
    //   block [0x82EFACFC..0x82EFAD08)
	// 82EFACFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD00: 409AFFF0  bne cr6, 0x82efacf0
	if !ctx.cr[6].eq {
	pc = 0x82EFACF0; continue 'dispatch;
	}
	// 82EFAD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAD08 size=56
    let mut pc: u32 = 0x82EFAD08;
    'dispatch: loop {
        match pc {
            0x82EFAD08 => {
    //   block [0x82EFAD08..0x82EFAD20)
	// 82EFAD08: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD10: 419A0010  beq cr6, 0x82efad20
	if ctx.cr[6].eq {
	pc = 0x82EFAD20; continue 'dispatch;
	}
	// 82EFAD14: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAD18: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EFAD1C: 4800000C  b 0x82efad28
	pc = 0x82EFAD28; continue 'dispatch;
            }
            0x82EFAD20 => {
    //   block [0x82EFAD20..0x82EFAD28)
	// 82EFAD20: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAD24: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFAD28; continue 'dispatch;
            }
            0x82EFAD28 => {
    //   block [0x82EFAD28..0x82EFAD40)
	// 82EFAD28: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAD2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD30: 419A0010  beq cr6, 0x82efad40
	if ctx.cr[6].eq {
		sub_82EFAD40(ctx, base);
		return;
	}
	// 82EFAD34: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD38: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFAD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAD40 size=12
    let mut pc: u32 = 0x82EFAD40;
    'dispatch: loop {
        match pc {
            0x82EFAD40 => {
    //   block [0x82EFAD40..0x82EFAD4C)
	// 82EFAD40: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD44: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFAD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAD50 size=348
    let mut pc: u32 = 0x82EFAD50;
    'dispatch: loop {
        match pc {
            0x82EFAD50 => {
    //   block [0x82EFAD50..0x82EFAD88)
	// 82EFAD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAD54: 4BDAE6A9  bl 0x82ca93fc
	ctx.lr = 0x82EFAD58;
	sub_82CA93D0(ctx, base);
	// 82EFAD58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAD5C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFAD60: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFAD64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAD68: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EFAD6C: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 82EFAD70: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAD74: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD78: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD7C: 409A000C  bne cr6, 0x82efad88
	if !ctx.cr[6].eq {
	pc = 0x82EFAD88; continue 'dispatch;
	}
	// 82EFAD80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAD84: 48000120  b 0x82efaea4
	pc = 0x82EFAEA4; continue 'dispatch;
            }
            0x82EFAD88 => {
    //   block [0x82EFAD88..0x82EFADBC)
	// 82EFAD88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAD8C: 483BEBD9  bl 0x832b9964
	ctx.lr = 0x82EFAD90;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAD94: 4BFFFE95  bl 0x82efac28
	ctx.lr = 0x82EFAD98;
	sub_82EFAC28(ctx, base);
	// 82EFAD98: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFAD9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFADA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFADA4: 419A0018  beq cr6, 0x82efadbc
	if ctx.cr[6].eq {
	pc = 0x82EFADBC; continue 'dispatch;
	}
	// 82EFADA8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFADAC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFADB0: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EFADB4: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFADB8: 48000010  b 0x82efadc8
	pc = 0x82EFADC8; continue 'dispatch;
            }
            0x82EFADBC => {
    //   block [0x82EFADBC..0x82EFADC8)
	// 82EFADBC: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EFADC0: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFADC4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	pc = 0x82EFADC8; continue 'dispatch;
            }
            0x82EFADC8 => {
    //   block [0x82EFADC8..0x82EFADF4)
	// 82EFADC8: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EFADCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFADD0: 483BEB85  bl 0x832b9954
	ctx.lr = 0x82EFADD4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFADD4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFADD8: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFADDC: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EFADE0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFADE4: 41820028  beq 0x82efae0c
	if ctx.cr[0].eq {
	pc = 0x82EFAE0C; continue 'dispatch;
	}
	// 82EFADE8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EFADEC: 419A0034  beq cr6, 0x82efae20
	if ctx.cr[6].eq {
	pc = 0x82EFAE20; continue 'dispatch;
	}
	// 82EFADF0: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	pc = 0x82EFADF4; continue 'dispatch;
            }
            0x82EFADF4 => {
    //   block [0x82EFADF4..0x82EFAE0C)
	// 82EFADF4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFADF8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFADFC: 48110BC5  bl 0x8300b9c0
	ctx.lr = 0x82EFAE00;
	sub_8300B9C0(ctx, base);
	// 82EFAE00: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EFAE04: 4082FFF0  bne 0x82efadf4
	if !ctx.cr[0].eq {
	pc = 0x82EFADF4; continue 'dispatch;
	}
	// 82EFAE08: 48000018  b 0x82efae20
	pc = 0x82EFAE20; continue 'dispatch;
            }
            0x82EFAE0C => {
    //   block [0x82EFAE0C..0x82EFAE20)
	// 82EFAE0C: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAE10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFAE14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFAE18: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAE1C: 4BDCC12D  bl 0x82cc6f48
	ctx.lr = 0x82EFAE20;
	sub_82CC6F48(ctx, base);
	pc = 0x82EFAE20; continue 'dispatch;
            }
            0x82EFAE20 => {
    //   block [0x82EFAE20..0x82EFAE38)
	// 82EFAE20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFAE24: 4BFFEC0D  bl 0x82ef9a30
	ctx.lr = 0x82EFAE28;
	sub_82EF9A30(ctx, base);
	// 82EFAE28: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 82EFAE2C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EFAE30: 419A0008  beq cr6, 0x82efae38
	if ctx.cr[6].eq {
	pc = 0x82EFAE38; continue 'dispatch;
	}
	// 82EFAE34: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	pc = 0x82EFAE38; continue 'dispatch;
            }
            0x82EFAE38 => {
    //   block [0x82EFAE38..0x82EFAE6C)
	// 82EFAE38: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAE3C: 4B29BB8D  bl 0x821969c8
	ctx.lr = 0x82EFAE40;
	sub_821969C8(ctx, base);
	// 82EFAE40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFAE44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE48: 483BEB1D  bl 0x832b9964
	ctx.lr = 0x82EFAE4C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAE4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EFAE50: 419A001C  beq cr6, 0x82efae6c
	if ctx.cr[6].eq {
	pc = 0x82EFAE6C; continue 'dispatch;
	}
	// 82EFAE54: 2B1D0080  cmplwi cr6, r29, 0x80
	ctx.cr[6].compare_u32(ctx.r[29].u32, 128 as u32, &mut ctx.xer);
	// 82EFAE58: 419A0014  beq cr6, 0x82efae6c
	if ctx.cr[6].eq {
	pc = 0x82EFAE6C; continue 'dispatch;
	}
	// 82EFAE5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFAE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE64: 4BFFFE85  bl 0x82eface8
	ctx.lr = 0x82EFAE68;
	sub_82EFACE8(ctx, base);
	// 82EFAE68: 48000008  b 0x82efae70
	pc = 0x82EFAE70; continue 'dispatch;
            }
            0x82EFAE6C => {
    //   block [0x82EFAE6C..0x82EFAE70)
	// 82EFAE6C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	pc = 0x82EFAE70; continue 'dispatch;
            }
            0x82EFAE70 => {
    //   block [0x82EFAE70..0x82EFAE90)
	// 82EFAE70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFAE74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE78: 4BFFFE21  bl 0x82efac98
	ctx.lr = 0x82EFAE7C;
	sub_82EFAC98(ctx, base);
	// 82EFAE7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE80: 483BEAD5  bl 0x832b9954
	ctx.lr = 0x82EFAE84;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFAE84: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EFAE88: 419A0018  beq cr6, 0x82efaea0
	if ctx.cr[6].eq {
	pc = 0x82EFAEA0; continue 'dispatch;
	}
	// 82EFAE8C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	pc = 0x82EFAE90; continue 'dispatch;
            }
            0x82EFAE90 => {
    //   block [0x82EFAE90..0x82EFAEA0)
	// 82EFAE90: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAE94: 4BFFFB95  bl 0x82efaa28
	ctx.lr = 0x82EFAE98;
	sub_82EFAA28(ctx, base);
	// 82EFAE98: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFAE9C: 4082FFF4  bne 0x82efae90
	if !ctx.cr[0].eq {
	pc = 0x82EFAE90; continue 'dispatch;
	}
	pc = 0x82EFAEA0; continue 'dispatch;
            }
            0x82EFAEA0 => {
    //   block [0x82EFAEA0..0x82EFAEA4)
	// 82EFAEA0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	pc = 0x82EFAEA4; continue 'dispatch;
            }
            0x82EFAEA4 => {
    //   block [0x82EFAEA4..0x82EFAEAC)
	// 82EFAEA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFAEA8: 4BDAE5A4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAEB0 size=116
    let mut pc: u32 = 0x82EFAEB0;
    'dispatch: loop {
        match pc {
            0x82EFAEB0 => {
    //   block [0x82EFAEB0..0x82EFAEF4)
	// 82EFAEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAEB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAEBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAEC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAEC4: 483BEAA1  bl 0x832b9964
	ctx.lr = 0x82EFAEC8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAEC8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFAECC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAED0: 41820038  beq 0x82efaf08
	if ctx.cr[0].eq {
	pc = 0x82EFAF08; continue 'dispatch;
	}
	// 82EFAED4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAED8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFAEDC: 419A0018  beq cr6, 0x82efaef4
	if ctx.cr[6].eq {
	pc = 0x82EFAEF4; continue 'dispatch;
	}
	// 82EFAEE0: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFAEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EFAEE8: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFAEEC: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EFAEF0: 48000010  b 0x82efaf00
	pc = 0x82EFAF00; continue 'dispatch;
            }
            0x82EFAEF4 => {
    //   block [0x82EFAEF4..0x82EFAF00)
	// 82EFAEF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFAEF8: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFAEFC: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	pc = 0x82EFAF00; continue 'dispatch;
            }
            0x82EFAF00 => {
    //   block [0x82EFAF00..0x82EFAF08)
	// 82EFAF00: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAF04: 4B3CE485  bl 0x822c9388
	ctx.lr = 0x82EFAF08;
	sub_822C9388(ctx, base);
	pc = 0x82EFAF08; continue 'dispatch;
            }
            0x82EFAF08 => {
    //   block [0x82EFAF08..0x82EFAF24)
	// 82EFAF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAF0C: 483BEA49  bl 0x832b9954
	ctx.lr = 0x82EFAF10;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFAF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAF1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAF28 size=132
    let mut pc: u32 = 0x82EFAF28;
    'dispatch: loop {
        match pc {
            0x82EFAF28 => {
    //   block [0x82EFAF28..0x82EFAF4C)
	// 82EFAF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAF30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFAF34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAF38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAF3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAF40: 483BEA25  bl 0x832b9964
	ctx.lr = 0x82EFAF44;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAF44: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFAF48: 4800000C  b 0x82efaf54
	pc = 0x82EFAF54; continue 'dispatch;
            }
            0x82EFAF4C => {
    //   block [0x82EFAF4C..0x82EFAF54)
	// 82EFAF4C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAF50: 4B3CE439  bl 0x822c9388
	ctx.lr = 0x82EFAF54;
	sub_822C9388(ctx, base);
	pc = 0x82EFAF54; continue 'dispatch;
            }
            0x82EFAF54 => {
    //   block [0x82EFAF54..0x82EFAF7C)
	// 82EFAF54: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFAF58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAF5C: 41820028  beq 0x82efaf84
	if ctx.cr[0].eq {
	pc = 0x82EFAF84; continue 'dispatch;
	}
	// 82EFAF60: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAF64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFAF68: 419A0014  beq cr6, 0x82efaf7c
	if ctx.cr[6].eq {
	pc = 0x82EFAF7C; continue 'dispatch;
	}
	// 82EFAF6C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFAF70: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFAF74: 93CA0008  stw r30, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EFAF78: 4800000C  b 0x82efaf84
	pc = 0x82EFAF84; continue 'dispatch;
            }
            0x82EFAF7C => {
    //   block [0x82EFAF7C..0x82EFAF84)
	// 82EFAF7C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EFAF80: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	pc = 0x82EFAF84; continue 'dispatch;
            }
            0x82EFAF84 => {
    //   block [0x82EFAF84..0x82EFAFAC)
	// 82EFAF84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAF88: 409AFFC4  bne cr6, 0x82efaf4c
	if !ctx.cr[6].eq {
	pc = 0x82EFAF4C; continue 'dispatch;
	}
	// 82EFAF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAF90: 483BE9C5  bl 0x832b9954
	ctx.lr = 0x82EFAF94;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFAF94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFAF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAFA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFAFA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAFB0 size=76
    let mut pc: u32 = 0x82EFAFB0;
    'dispatch: loop {
        match pc {
            0x82EFAFB0 => {
    //   block [0x82EFAFB0..0x82EFAFDC)
	// 82EFAFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAFBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAFC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAFC4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFAFC8: 48002899  bl 0x82efd860
	ctx.lr = 0x82EFAFCC;
	sub_82EFD860(ctx, base);
	// 82EFAFCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAFD0: 4182000C  beq 0x82efafdc
	if ctx.cr[0].eq {
	pc = 0x82EFAFDC; continue 'dispatch;
	}
	// 82EFAFD4: 4BFFFBB5  bl 0x82efab88
	ctx.lr = 0x82EFAFD8;
	sub_82EFAB88(ctx, base);
	// 82EFAFD8: 48000008  b 0x82efafe0
	pc = 0x82EFAFE0; continue 'dispatch;
            }
            0x82EFAFDC => {
    //   block [0x82EFAFDC..0x82EFAFE0)
	// 82EFAFDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFAFE0; continue 'dispatch;
            }
            0x82EFAFE0 => {
    //   block [0x82EFAFE0..0x82EFAFFC)
	// 82EFAFE0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFAFE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAFE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAFF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB000 size=8
    let mut pc: u32 = 0x82EFB000;
    'dispatch: loop {
        match pc {
            0x82EFB000 => {
    //   block [0x82EFB000..0x82EFB008)
	// 82EFB000: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB004: 4BFFFD4C  b 0x82efad50
	sub_82EFAD50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB008 size=8
    let mut pc: u32 = 0x82EFB008;
    'dispatch: loop {
        match pc {
            0x82EFB008 => {
    //   block [0x82EFB008..0x82EFB010)
	// 82EFB008: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB00C: 4BFFFEA4  b 0x82efaeb0
	sub_82EFAEB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB010 size=8
    let mut pc: u32 = 0x82EFB010;
    'dispatch: loop {
        match pc {
            0x82EFB010 => {
    //   block [0x82EFB010..0x82EFB018)
	// 82EFB010: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB014: 4BFFFF14  b 0x82efaf28
	sub_82EFAF28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB018 size=24
    let mut pc: u32 = 0x82EFB018;
    'dispatch: loop {
        match pc {
            0x82EFB018 => {
    //   block [0x82EFB018..0x82EFB030)
	// 82EFB018: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFB01C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB020: 419A0010  beq cr6, 0x82efb030
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82EFB030);
		return;
	}
	// 82EFB024: 80830034  lwz r4, 0x34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFB028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFB02C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB038 size=4
    let mut pc: u32 = 0x82EFB038;
    'dispatch: loop {
        match pc {
            0x82EFB038 => {
    //   block [0x82EFB038..0x82EFB03C)
	// 82EFB038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB040 size=40
    let mut pc: u32 = 0x82EFB040;
    'dispatch: loop {
        match pc {
            0x82EFB040 => {
    //   block [0x82EFB040..0x82EFB068)
	// 82EFB040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB04C: 1C6403E8  mulli r3, r4, 0x3e8
	ctx.r[3].s32 = ((ctx.r[4].s32 as i64 * 1000 as i64) as i32);
	ctx.r[3].s64 = ctx.r[3].s32 as i64;
	// 82EFB050: 4BDC7E31  bl 0x82cc2e80
	ctx.lr = 0x82EFB054;
	sub_82CC2E80(ctx, base);
	// 82EFB054: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB068 size=40
    let mut pc: u32 = 0x82EFB068;
    'dispatch: loop {
        match pc {
            0x82EFB068 => {
    //   block [0x82EFB068..0x82EFB090)
	// 82EFB068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB074: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EFB078: 4BDC7E09  bl 0x82cc2e80
	ctx.lr = 0x82EFB07C;
	sub_82CC2E80(ctx, base);
	// 82EFB07C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB090 size=68
    let mut pc: u32 = 0x82EFB090;
    'dispatch: loop {
        match pc {
            0x82EFB090 => {
    //   block [0x82EFB090..0x82EFB0BC)
	// 82EFB090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB09C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB0A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB0A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB0A8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFB0AC: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFB0B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB0B4: 41820008  beq 0x82efb0bc
	if ctx.cr[0].eq {
	pc = 0x82EFB0BC; continue 'dispatch;
	}
	// 82EFB0B8: 4B94A6F9  bl 0x828457b0
	ctx.lr = 0x82EFB0BC;
	sub_828457B0(ctx, base);
	pc = 0x82EFB0BC; continue 'dispatch;
            }
            0x82EFB0BC => {
    //   block [0x82EFB0BC..0x82EFB0D4)
	// 82EFB0BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB0D8 size=104
    let mut pc: u32 = 0x82EFB0D8;
    'dispatch: loop {
        match pc {
            0x82EFB0D8 => {
    //   block [0x82EFB0D8..0x82EFB12C)
	// 82EFB0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB0DC: 4BDAE32D  bl 0x82ca9408
	ctx.lr = 0x82EFB0E0;
	sub_82CA93D0(ctx, base);
	// 82EFB0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB0E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB0E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB0EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFB0F0: 392BD060  addi r9, r11, -0x2fa0
	ctx.r[9].s64 = ctx.r[11].s64 + -12192;
	// 82EFB0F4: 394AD04C  addi r10, r10, -0x2fb4
	ctx.r[10].s64 = ctx.r[10].s64 + -12212;
	// 82EFB0F8: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB0FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB100: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EFB104: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 82EFB108: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EFB10C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFB110: 3B8BC544  addi r28, r11, -0x3abc
	ctx.r[28].s64 = ctx.r[11].s64 + -15036;
	// 82EFB114: 419A0018  beq cr6, 0x82efb12c
	if ctx.cr[6].eq {
	pc = 0x82EFB12C; continue 'dispatch;
	}
	// 82EFB118: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB11C: 4BDC7695  bl 0x82cc27b0
	ctx.lr = 0x82EFB120;
	sub_82CC27B0(ctx, base);
	// 82EFB120: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82EFB124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB128: 48002779  bl 0x82efd8a0
	ctx.lr = 0x82EFB12C;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFB12C; continue 'dispatch;
            }
            0x82EFB12C => {
    //   block [0x82EFB12C..0x82EFB140)
	// 82EFB12C: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFB130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB134: 4BFFE9FD  bl 0x82ef9b30
	ctx.lr = 0x82EFB138;
	sub_82EF9B30(ctx, base);
	// 82EFB138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFB13C: 4BDAE31C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB140 size=64
    let mut pc: u32 = 0x82EFB140;
    'dispatch: loop {
        match pc {
            0x82EFB140 => {
    //   block [0x82EFB140..0x82EFB16C)
	// 82EFB140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB14C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB150: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB154: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB158: 419A0014  beq cr6, 0x82efb16c
	if ctx.cr[6].eq {
	pc = 0x82EFB16C; continue 'dispatch;
	}
	// 82EFB15C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB160: 4BFFFA71  bl 0x82efabd0
	ctx.lr = 0x82EFB164;
	sub_82EFABD0(ctx, base);
	// 82EFB164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB168: 48002739  bl 0x82efd8a0
	ctx.lr = 0x82EFB16C;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFB16C; continue 'dispatch;
            }
            0x82EFB16C => {
    //   block [0x82EFB16C..0x82EFB180)
	// 82EFB16C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB180 size=120
    let mut pc: u32 = 0x82EFB180;
    'dispatch: loop {
        match pc {
            0x82EFB180 => {
    //   block [0x82EFB180..0x82EFB1C8)
	// 82EFB180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB188: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB18C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB190: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB198: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB19C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFB1A0: 396BD080  addi r11, r11, -0x2f80
	ctx.r[11].s64 = ctx.r[11].s64 + -12160;
	// 82EFB1A4: 394AD06C  addi r10, r10, -0x2f94
	ctx.r[10].s64 = ctx.r[10].s64 + -12180;
	// 82EFB1A8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFB1AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFB1B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB1B4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EFB1B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFB1BC: 419A000C  beq cr6, 0x82efb1c8
	if ctx.cr[6].eq {
	pc = 0x82EFB1C8; continue 'dispatch;
	}
	// 82EFB1C0: 4BDC75F1  bl 0x82cc27b0
	ctx.lr = 0x82EFB1C4;
	sub_82CC27B0(ctx, base);
	// 82EFB1C4: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	pc = 0x82EFB1C8; continue 'dispatch;
            }
            0x82EFB1C8 => {
    //   block [0x82EFB1C8..0x82EFB1F8)
	// 82EFB1C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB1CC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EFB1D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB1D4: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFB1D8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFB1DC: 4BFFE955  bl 0x82ef9b30
	ctx.lr = 0x82EFB1E0;
	sub_82EF9B30(ctx, base);
	// 82EFB1E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB1EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFB1F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB1F8 size=12
    let mut pc: u32 = 0x82EFB1F8;
    'dispatch: loop {
        match pc {
            0x82EFB1F8 => {
    //   block [0x82EFB1F8..0x82EFB204)
	// 82EFB1F8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB1FC: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82EFB200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB208 size=12
    let mut pc: u32 = 0x82EFB208;
    'dispatch: loop {
        match pc {
            0x82EFB208 => {
    //   block [0x82EFB208..0x82EFB214)
	// 82EFB208: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB20C: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82EFB210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB218 size=28
    let mut pc: u32 = 0x82EFB218;
    'dispatch: loop {
        match pc {
            0x82EFB218 => {
    //   block [0x82EFB218..0x82EFB22C)
	// 82EFB218: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFB21C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB220: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFB224: 41990008  bgt cr6, 0x82efb22c
	if ctx.cr[6].gt {
	pc = 0x82EFB22C; continue 'dispatch;
	}
	// 82EFB228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EFB22C; continue 'dispatch;
            }
            0x82EFB22C => {
    //   block [0x82EFB22C..0x82EFB234)
	// 82EFB22C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EFB230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB238 size=36
    let mut pc: u32 = 0x82EFB238;
    'dispatch: loop {
        match pc {
            0x82EFB238 => {
    //   block [0x82EFB238..0x82EFB24C)
	// 82EFB238: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFB23C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB240: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFB244: 41990008  bgt cr6, 0x82efb24c
	if ctx.cr[6].gt {
	pc = 0x82EFB24C; continue 'dispatch;
	}
	// 82EFB248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EFB24C; continue 'dispatch;
            }
            0x82EFB24C => {
    //   block [0x82EFB24C..0x82EFB25C)
	// 82EFB24C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB250: 4182000C  beq 0x82efb25c
	if ctx.cr[0].eq {
		sub_82EFB25C(ctx, base);
		return;
	}
	// 82EFB254: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82EFB258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB25C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB25C size=12
    let mut pc: u32 = 0x82EFB25C;
    'dispatch: loop {
        match pc {
            0x82EFB25C => {
    //   block [0x82EFB25C..0x82EFB268)
	// 82EFB25C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB260: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82EFB264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB268 size=352
    let mut pc: u32 = 0x82EFB268;
    'dispatch: loop {
        match pc {
            0x82EFB268 => {
    //   block [0x82EFB268..0x82EFB294)
	// 82EFB268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB26C: 4BDAE1A1  bl 0x82ca940c
	ctx.lr = 0x82EFB270;
	sub_82CA93D0(ctx, base);
	// 82EFB270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB278: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFB27C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFB280: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB288: 409A000C  bne cr6, 0x82efb294
	if !ctx.cr[6].eq {
	pc = 0x82EFB294; continue 'dispatch;
	}
	// 82EFB28C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EFB290: 4800002C  b 0x82efb2bc
	pc = 0x82EFB2BC; continue 'dispatch;
            }
            0x82EFB294 => {
    //   block [0x82EFB294..0x82EFB2BC)
	// 82EFB294: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB298: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB29C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EFB2A0: 1D290005  mulli r9, r9, 5
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * 5 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82EFB2A4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFB2A8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFB2AC: 40990018  ble cr6, 0x82efb2c4
	if !ctx.cr[6].gt {
	pc = 0x82EFB2C4; continue 'dispatch;
	}
	// 82EFB2B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB2B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFB2B8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	pc = 0x82EFB2BC; continue 'dispatch;
            }
            0x82EFB2BC => {
    //   block [0x82EFB2BC..0x82EFB2C4)
	// 82EFB2BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB2C0: 48000219  bl 0x82efb4d8
	ctx.lr = 0x82EFB2C4;
	sub_82EFB4D8(ctx, base);
	pc = 0x82EFB2C4; continue 'dispatch;
            }
            0x82EFB2C4 => {
    //   block [0x82EFB2C4..0x82EFB308)
	// 82EFB2C4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2C8: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB2CC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2D0: 7D48F038  and r8, r10, r30
	ctx.r[8].u64 = ctx.r[10].u64 & ctx.r[30].u64;
	// 82EFB2D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFB2D8: 1D48000C  mulli r10, r8, 0xc
	ctx.r[10].s32 = ((ctx.r[8].s32 as i64 * 12 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82EFB2DC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB2E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2E4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFB2E8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EFB2EC: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2F0: 2F05FFFE  cmpwi cr6, r5, -2
	ctx.cr[6].compare_i32(ctx.r[5].s32, -2, &mut ctx.xer);
	// 82EFB2F4: 409A0014  bne cr6, 0x82efb308
	if !ctx.cr[6].eq {
	pc = 0x82EFB308; continue 'dispatch;
	}
	// 82EFB2F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFB2FC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB300: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB304: 48000064  b 0x82efb368
	pc = 0x82EFB368; continue 'dispatch;
            }
            0x82EFB308 => {
    //   block [0x82EFB308..0x82EFB310)
	// 82EFB308: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB30C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	pc = 0x82EFB310; continue 'dispatch;
            }
            0x82EFB310 => {
    //   block [0x82EFB310..0x82EFB360)
	// 82EFB310: 38E60001  addi r7, r6, 1
	ctx.r[7].s64 = ctx.r[6].s64 + 1;
	// 82EFB314: 7CE64838  and r6, r7, r9
	ctx.r[6].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82EFB318: 1CE6000C  mulli r7, r6, 0xc
	ctx.r[7].s32 = ((ctx.r[6].s32 as i64 * 12 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82EFB31C: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EFB320: 80E70008  lwz r7, 8(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB324: 2F07FFFE  cmpwi cr6, r7, -2
	ctx.cr[6].compare_i32(ctx.r[7].s32, -2, &mut ctx.xer);
	// 82EFB328: 409AFFE8  bne cr6, 0x82efb310
	if !ctx.cr[6].eq {
	pc = 0x82EFB310; continue 'dispatch;
	}
	// 82EFB32C: 1CE6000C  mulli r7, r6, 0xc
	ctx.r[7].s32 = ((ctx.r[6].s32 as i64 * 12 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82EFB330: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB334: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EFB338: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EFB33C: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 82EFB340: 409A0034  bne cr6, 0x82efb374
	if !ctx.cr[6].eq {
	pc = 0x82EFB374; continue 'dispatch;
	}
	// 82EFB344: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EFB348: 419A0018  beq cr6, 0x82efb360
	if ctx.cr[6].eq {
	pc = 0x82EFB360; continue 'dispatch;
	}
	// 82EFB34C: 90A70000  stw r5, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EFB350: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB354: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFB358: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB35C: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFB360; continue 'dispatch;
            }
            0x82EFB360 => {
    //   block [0x82EFB360..0x82EFB368)
	// 82EFB360: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB364: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	pc = 0x82EFB368; continue 'dispatch;
            }
            0x82EFB368 => {
    //   block [0x82EFB368..0x82EFB370)
	// 82EFB368: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFB36C: 48000050  b 0x82efb3bc
	pc = 0x82EFB3BC; continue 'dispatch;
            }
            0x82EFB370 => {
    //   block [0x82EFB370..0x82EFB374)
	// 82EFB370: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82EFB374; continue 'dispatch;
            }
            0x82EFB374 => {
    //   block [0x82EFB374..0x82EFB3A8)
	// 82EFB374: 1D29000C  mulli r9, r9, 0xc
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * 12 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82EFB378: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFB37C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82EFB380: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB384: 7F044000  cmpw cr6, r4, r8
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EFB388: 409AFFE8  bne cr6, 0x82efb370
	if !ctx.cr[6].eq {
	pc = 0x82EFB370; continue 'dispatch;
	}
	// 82EFB38C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EFB390: 419A0018  beq cr6, 0x82efb3a8
	if ctx.cr[6].eq {
	pc = 0x82EFB3A8; continue 'dispatch;
	}
	// 82EFB394: 90A70000  stw r5, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EFB398: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB39C: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFB3A0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB3A4: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFB3A8; continue 'dispatch;
            }
            0x82EFB3A8 => {
    //   block [0x82EFB3A8..0x82EFB3BC)
	// 82EFB3A8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFB3AC: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EFB3B0: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB3B4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB3B8: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	pc = 0x82EFB3BC; continue 'dispatch;
            }
            0x82EFB3BC => {
    //   block [0x82EFB3BC..0x82EFB3C8)
	// 82EFB3BC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EFB3C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB3C4: 4BDAE098  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB3C8 size=132
    let mut pc: u32 = 0x82EFB3C8;
    'dispatch: loop {
        match pc {
            0x82EFB3C8 => {
    //   block [0x82EFB3C8..0x82EFB3FC)
	// 82EFB3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB3CC: 4BDAE03D  bl 0x82ca9408
	ctx.lr = 0x82EFB3D0;
	sub_82CA93D0(ctx, base);
	// 82EFB3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB3D4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFB3D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFB3DC: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB3E0: 41820048  beq 0x82efb428
	if ctx.cr[0].eq {
	pc = 0x82EFB428; continue 'dispatch;
	}
	// 82EFB3E4: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFB3E8: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFB3EC: 1D6A001C  mulli r11, r10, 0x1c
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 28 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EFB3F0: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB3F4: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFB3F8: 41800018  blt 0x82efb410
	if ctx.cr[0].lt {
	pc = 0x82EFB410; continue 'dispatch;
	}
	pc = 0x82EFB3FC; continue 'dispatch;
            }
            0x82EFB3FC => {
    //   block [0x82EFB3FC..0x82EFB410)
	// 82EFB3FC: 3BDEFFE4  addi r30, r30, -0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + -28;
	// 82EFB400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB404: 4BFFFCD5  bl 0x82efb0d8
	ctx.lr = 0x82EFB408;
	sub_82EFB0D8(ctx, base);
	// 82EFB408: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB40C: 4080FFF0  bge 0x82efb3fc
	if !ctx.cr[0].lt {
	pc = 0x82EFB3FC; continue 'dispatch;
	}
	pc = 0x82EFB410; continue 'dispatch;
            }
            0x82EFB410 => {
    //   block [0x82EFB410..0x82EFB420)
	// 82EFB410: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB414: 4182000C  beq 0x82efb420
	if ctx.cr[0].eq {
	pc = 0x82EFB420; continue 'dispatch;
	}
	// 82EFB418: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB41C: 4B94A395  bl 0x828457b0
	ctx.lr = 0x82EFB420;
	sub_828457B0(ctx, base);
	pc = 0x82EFB420; continue 'dispatch;
            }
            0x82EFB420 => {
    //   block [0x82EFB420..0x82EFB428)
	// 82EFB420: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB424: 48000020  b 0x82efb444
	pc = 0x82EFB444; continue 'dispatch;
            }
            0x82EFB428 => {
    //   block [0x82EFB428..0x82EFB440)
	// 82EFB428: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB42C: 4BFFFCAD  bl 0x82efb0d8
	ctx.lr = 0x82EFB430;
	sub_82EFB0D8(ctx, base);
	// 82EFB430: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB434: 4182000C  beq 0x82efb440
	if ctx.cr[0].eq {
	pc = 0x82EFB440; continue 'dispatch;
	}
	// 82EFB438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB43C: 48002465  bl 0x82efd8a0
	ctx.lr = 0x82EFB440;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFB440; continue 'dispatch;
            }
            0x82EFB440 => {
    //   block [0x82EFB440..0x82EFB444)
	// 82EFB440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82EFB444; continue 'dispatch;
            }
            0x82EFB444 => {
    //   block [0x82EFB444..0x82EFB44C)
	// 82EFB444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFB448: 4BDAE010  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB450 size=132
    let mut pc: u32 = 0x82EFB450;
    'dispatch: loop {
        match pc {
            0x82EFB450 => {
    //   block [0x82EFB450..0x82EFB484)
	// 82EFB450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB454: 4BDADFB5  bl 0x82ca9408
	ctx.lr = 0x82EFB458;
	sub_82CA93D0(ctx, base);
	// 82EFB458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB45C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFB460: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFB464: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB468: 41820048  beq 0x82efb4b0
	if ctx.cr[0].eq {
	pc = 0x82EFB4B0; continue 'dispatch;
	}
	// 82EFB46C: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFB470: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFB474: 1D6A0038  mulli r11, r10, 0x38
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 56 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EFB478: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB47C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFB480: 41800018  blt 0x82efb498
	if ctx.cr[0].lt {
	pc = 0x82EFB498; continue 'dispatch;
	}
	pc = 0x82EFB484; continue 'dispatch;
            }
            0x82EFB484 => {
    //   block [0x82EFB484..0x82EFB498)
	// 82EFB484: 3BDEFFC8  addi r30, r30, -0x38
	ctx.r[30].s64 = ctx.r[30].s64 + -56;
	// 82EFB488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB48C: 4BFFFCF5  bl 0x82efb180
	ctx.lr = 0x82EFB490;
	sub_82EFB180(ctx, base);
	// 82EFB490: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB494: 4080FFF0  bge 0x82efb484
	if !ctx.cr[0].lt {
	pc = 0x82EFB484; continue 'dispatch;
	}
	pc = 0x82EFB498; continue 'dispatch;
            }
            0x82EFB498 => {
    //   block [0x82EFB498..0x82EFB4A8)
	// 82EFB498: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB49C: 4182000C  beq 0x82efb4a8
	if ctx.cr[0].eq {
	pc = 0x82EFB4A8; continue 'dispatch;
	}
	// 82EFB4A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB4A4: 4B94A30D  bl 0x828457b0
	ctx.lr = 0x82EFB4A8;
	sub_828457B0(ctx, base);
	pc = 0x82EFB4A8; continue 'dispatch;
            }
            0x82EFB4A8 => {
    //   block [0x82EFB4A8..0x82EFB4B0)
	// 82EFB4A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB4AC: 48000020  b 0x82efb4cc
	pc = 0x82EFB4CC; continue 'dispatch;
            }
            0x82EFB4B0 => {
    //   block [0x82EFB4B0..0x82EFB4C8)
	// 82EFB4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB4B4: 4BFFFCCD  bl 0x82efb180
	ctx.lr = 0x82EFB4B8;
	sub_82EFB180(ctx, base);
	// 82EFB4B8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB4BC: 4182000C  beq 0x82efb4c8
	if ctx.cr[0].eq {
	pc = 0x82EFB4C8; continue 'dispatch;
	}
	// 82EFB4C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB4C4: 480023DD  bl 0x82efd8a0
	ctx.lr = 0x82EFB4C8;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFB4C8; continue 'dispatch;
            }
            0x82EFB4C8 => {
    //   block [0x82EFB4C8..0x82EFB4CC)
	// 82EFB4C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82EFB4CC; continue 'dispatch;
            }
            0x82EFB4CC => {
    //   block [0x82EFB4CC..0x82EFB4D4)
	// 82EFB4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFB4D0: 4BDADF88  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EFB4D8 size=320
    let mut pc: u32 = 0x82EFB4D8;
    'dispatch: loop {
        match pc {
            0x82EFB4D8 => {
    //   block [0x82EFB4D8..0x82EFB4F4)
	// 82EFB4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB4DC: 4BDADF25  bl 0x82ca9400
	ctx.lr = 0x82EFB4E0;
	sub_82CA93D0(ctx, base);
	// 82EFB4E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB4E4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EFB4E8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFB4EC: 409A0008  bne cr6, 0x82efb4f4
	if !ctx.cr[6].eq {
	pc = 0x82EFB4F4; continue 'dispatch;
	}
	// 82EFB4F0: 4800011C  b 0x82efb60c
	pc = 0x82EFB60C; continue 'dispatch;
            }
            0x82EFB4F4 => {
    //   block [0x82EFB4F4..0x82EFB504)
	// 82EFB4F4: 2B040008  cmplwi cr6, r4, 8
	ctx.cr[6].compare_u32(ctx.r[4].u32, 8 as u32, &mut ctx.xer);
	// 82EFB4F8: 4098000C  bge cr6, 0x82efb504
	if !ctx.cr[6].lt {
	pc = 0x82EFB504; continue 'dispatch;
	}
	// 82EFB4FC: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82EFB500: 4800004C  b 0x82efb54c
	pc = 0x82EFB54C; continue 'dispatch;
            }
            0x82EFB504 => {
    //   block [0x82EFB504..0x82EFB54C)
	// 82EFB504: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EFB508: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EFB50C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82EFB510: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EFB514: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82EFB518: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82EFB51C: 4B2F8895  bl 0x821f3db0
	ctx.lr = 0x82EFB520;
	sub_821F3DB0(ctx, base);
	// 82EFB520: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EFB524: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EFB528: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EFB52C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EFB530: C00B3FA8  lfs f0, 0x3fa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFB534: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFB538: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82EFB53C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EFB540: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82EFB544: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFB548: 7D3F5830  slw r31, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	pc = 0x82EFB54C; continue 'dispatch;
            }
            0x82EFB54C => {
    //   block [0x82EFB54C..0x82EFB584)
	// 82EFB54C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EFB550: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 12 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EFB554: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EFB558: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EFB55C: 48002305  bl 0x82efd860
	ctx.lr = 0x82EFB560;
	sub_82EFD860(ctx, base);
	// 82EFB560: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EFB564: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82EFB568: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFB56C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB570: 3B60FFFE  li r27, -2
	ctx.r[27].s64 = -2;
	// 82EFB574: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB578: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFB57C: 419A0020  beq cr6, 0x82efb59c
	if ctx.cr[6].eq {
	pc = 0x82EFB59C; continue 'dispatch;
	}
	// 82EFB580: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x82EFB584; continue 'dispatch;
            }
            0x82EFB584 => {
    //   block [0x82EFB584..0x82EFB59C)
	// 82EFB584: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB588: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB58C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFB590: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82EFB594: 936A0008  stw r27, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82EFB598: 4082FFEC  bne 0x82efb584
	if !ctx.cr[0].eq {
	pc = 0x82EFB584; continue 'dispatch;
	}
	pc = 0x82EFB59C; continue 'dispatch;
            }
            0x82EFB59C => {
    //   block [0x82EFB59C..0x82EFB5B4)
	// 82EFB59C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB5A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB5A4: 419A0058  beq cr6, 0x82efb5fc
	if ctx.cr[6].eq {
	pc = 0x82EFB5FC; continue 'dispatch;
	}
	// 82EFB5A8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB5AC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82EFB5B0: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	pc = 0x82EFB5B4; continue 'dispatch;
            }
            0x82EFB5B4 => {
    //   block [0x82EFB5B4..0x82EFB5E8)
	// 82EFB5B4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB5B8: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EFB5BC: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EFB5C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB5C4: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EFB5C8: 419A0020  beq cr6, 0x82efb5e8
	if ctx.cr[6].eq {
	pc = 0x82EFB5E8; continue 'dispatch;
	}
	// 82EFB5CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB5D0: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82EFB5D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFB5D8: 556AD1BE  srwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFB5DC: 7D455A78  xor r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82EFB5E0: 4BFFFC89  bl 0x82efb268
	ctx.lr = 0x82EFB5E4;
	sub_82EFB268(ctx, base);
	// 82EFB5E4: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	pc = 0x82EFB5E8; continue 'dispatch;
            }
            0x82EFB5E8 => {
    //   block [0x82EFB5E8..0x82EFB5FC)
	// 82EFB5E8: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EFB5EC: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82EFB5F0: 4082FFC4  bne 0x82efb5b4
	if !ctx.cr[0].eq {
	pc = 0x82EFB5B4; continue 'dispatch;
	}
	// 82EFB5F4: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB5F8: 480022A9  bl 0x82efd8a0
	ctx.lr = 0x82EFB5FC;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFB5FC; continue 'dispatch;
            }
            0x82EFB5FC => {
    //   block [0x82EFB5FC..0x82EFB60C)
	// 82EFB5FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFB604: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EFB608: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFB60C; continue 'dispatch;
            }
            0x82EFB60C => {
    //   block [0x82EFB60C..0x82EFB618)
	// 82EFB60C: 4809496D  bl 0x82f8ff78
	ctx.lr = 0x82EFB610;
	sub_82F8FF78(ctx, base);
	// 82EFB610: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFB614: 4BDADE3C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB618 size=8
    let mut pc: u32 = 0x82EFB618;
    'dispatch: loop {
        match pc {
            0x82EFB618 => {
    //   block [0x82EFB618..0x82EFB620)
	// 82EFB618: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFB61C: 4BFFFDAC  b 0x82efb3c8
	sub_82EFB3C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB620 size=8
    let mut pc: u32 = 0x82EFB620;
    'dispatch: loop {
        match pc {
            0x82EFB620 => {
    //   block [0x82EFB620..0x82EFB628)
	// 82EFB620: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFB624: 4BFFFE2C  b 0x82efb450
	sub_82EFB450(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB628 size=84
    let mut pc: u32 = 0x82EFB628;
    'dispatch: loop {
        match pc {
            0x82EFB628 => {
    //   block [0x82EFB628..0x82EFB67C)
	// 82EFB628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB638: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB63C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFB640: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82EFB644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFB648: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFB64C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB650: 4BFFF479  bl 0x82efaac8
	ctx.lr = 0x82EFB654;
	sub_82EFAAC8(ctx, base);
	// 82EFB654: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82EFB658: 4BFFF959  bl 0x82efafb0
	ctx.lr = 0x82EFB65C;
	sub_82EFAFB0(ctx, base);
	// 82EFB65C: 4B3236AD  bl 0x8221ed08
	ctx.lr = 0x82EFB660;
	sub_8221ED08(ctx, base);
	// 82EFB660: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82EFB664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB674: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB680 size=92
    let mut pc: u32 = 0x82EFB680;
    'dispatch: loop {
        match pc {
            0x82EFB680 => {
    //   block [0x82EFB680..0x82EFB6B4)
	// 82EFB680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB68C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFB698: 83FE0020  lwz r31, 0x20(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFB69C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB6A0: 419A0014  beq cr6, 0x82efb6b4
	if ctx.cr[6].eq {
	pc = 0x82EFB6B4; continue 'dispatch;
	}
	// 82EFB6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB6A8: 4BFFF529  bl 0x82efabd0
	ctx.lr = 0x82EFB6AC;
	sub_82EFABD0(ctx, base);
	// 82EFB6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB6B0: 480021F1  bl 0x82efd8a0
	ctx.lr = 0x82EFB6B4;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFB6B4; continue 'dispatch;
            }
            0x82EFB6B4 => {
    //   block [0x82EFB6B4..0x82EFB6DC)
	// 82EFB6B4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82EFB6B8: 4BFFFA21  bl 0x82efb0d8
	ctx.lr = 0x82EFB6BC;
	sub_82EFB0D8(ctx, base);
	// 82EFB6BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB6C0: 480948B9  bl 0x82f8ff78
	ctx.lr = 0x82EFB6C4;
	sub_82F8FF78(ctx, base);
	// 82EFB6C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB6D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFB6D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB6E0 size=124
    let mut pc: u32 = 0x82EFB6E0;
    'dispatch: loop {
        match pc {
            0x82EFB6E0 => {
    //   block [0x82EFB6E0..0x82EFB700)
	// 82EFB6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB6EC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB6F0: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB6F4: 41820054  beq 0x82efb748
	if ctx.cr[0].eq {
	pc = 0x82EFB748; continue 'dispatch;
	}
	// 82EFB6F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFB6FC: 38E3001C  addi r7, r3, 0x1c
	ctx.r[7].s64 = ctx.r[3].s64 + 28;
	pc = 0x82EFB700; continue 'dispatch;
            }
            0x82EFB700 => {
    //   block [0x82EFB700..0x82EFB740)
	// 82EFB700: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFB704: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB708: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB70C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFB710: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB714: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB718: 4082FFE8  bne 0x82efb700
	if !ctx.cr[0].eq {
	pc = 0x82EFB700; continue 'dispatch;
	}
	// 82EFB71C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFB720: 7C2004AC  lwsync
	// 82EFB724: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFB728: 41980020  blt cr6, 0x82efb748
	if ctx.cr[6].lt {
	pc = 0x82EFB748; continue 'dispatch;
	}
	// 82EFB72C: 409A0014  bne cr6, 0x82efb740
	if !ctx.cr[6].eq {
	pc = 0x82EFB740; continue 'dispatch;
	}
	// 82EFB730: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFB734: 4BDCC735  bl 0x82cc7e68
	ctx.lr = 0x82EFB738;
	sub_82CC7E68(ctx, base);
	// 82EFB738: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EFB73C: 419A000C  beq cr6, 0x82efb748
	if ctx.cr[6].eq {
	pc = 0x82EFB748; continue 'dispatch;
	}
	pc = 0x82EFB740; continue 'dispatch;
            }
            0x82EFB740 => {
    //   block [0x82EFB740..0x82EFB748)
	// 82EFB740: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB744: 48000008  b 0x82efb74c
	pc = 0x82EFB74C; continue 'dispatch;
            }
            0x82EFB748 => {
    //   block [0x82EFB748..0x82EFB74C)
	// 82EFB748: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFB74C; continue 'dispatch;
            }
            0x82EFB74C => {
    //   block [0x82EFB74C..0x82EFB75C)
	// 82EFB74C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB760 size=80
    let mut pc: u32 = 0x82EFB760;
    'dispatch: loop {
        match pc {
            0x82EFB760 => {
    //   block [0x82EFB760..0x82EFB764)
	// 82EFB760: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x82EFB764; continue 'dispatch;
            }
            0x82EFB764 => {
    //   block [0x82EFB764..0x82EFB770)
	// 82EFB764: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB768: 7D232038  and r3, r9, r4
	ctx.r[3].u64 = ctx.r[9].u64 & ctx.r[4].u64;
	// 82EFB76C: 7C2004AC  lwsync
	pc = 0x82EFB770; continue 'dispatch;
            }
            0x82EFB770 => {
    //   block [0x82EFB770..0x82EFB794)
	// 82EFB770: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFB774: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB778: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB77C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB780: 409A0014  bne cr6, 0x82efb794
	if !ctx.cr[6].eq {
	pc = 0x82EFB794; continue 'dispatch;
	}
	// 82EFB784: 7C60592D  stwcx. r3, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB788: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB78C: 4082FFE4  bne 0x82efb770
	if !ctx.cr[0].eq {
	pc = 0x82EFB770; continue 'dispatch;
	}
	// 82EFB790: 4800000C  b 0x82efb79c
	pc = 0x82EFB79C; continue 'dispatch;
            }
            0x82EFB794 => {
    //   block [0x82EFB794..0x82EFB79C)
	// 82EFB794: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB798: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	pc = 0x82EFB79C; continue 'dispatch;
            }
            0x82EFB79C => {
    //   block [0x82EFB79C..0x82EFB7B0)
	// 82EFB79C: 7D4A5378  mr r10, r10
	ctx.r[10].u64 = ctx.r[10].u64;
	// 82EFB7A0: 7C2004AC  lwsync
	// 82EFB7A4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB7A8: 409AFFBC  bne cr6, 0x82efb764
	if !ctx.cr[6].eq {
	pc = 0x82EFB764; continue 'dispatch;
	}
	// 82EFB7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB7B0 size=80
    let mut pc: u32 = 0x82EFB7B0;
    'dispatch: loop {
        match pc {
            0x82EFB7B0 => {
    //   block [0x82EFB7B0..0x82EFB7B4)
	// 82EFB7B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x82EFB7B4; continue 'dispatch;
            }
            0x82EFB7B4 => {
    //   block [0x82EFB7B4..0x82EFB7C0)
	// 82EFB7B4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB7B8: 7D232378  or r3, r9, r4
	ctx.r[3].u64 = ctx.r[9].u64 | ctx.r[4].u64;
	// 82EFB7BC: 7C2004AC  lwsync
	pc = 0x82EFB7C0; continue 'dispatch;
            }
            0x82EFB7C0 => {
    //   block [0x82EFB7C0..0x82EFB7E4)
	// 82EFB7C0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFB7C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB7C8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB7CC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB7D0: 409A0014  bne cr6, 0x82efb7e4
	if !ctx.cr[6].eq {
	pc = 0x82EFB7E4; continue 'dispatch;
	}
	// 82EFB7D4: 7C60592D  stwcx. r3, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB7D8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB7DC: 4082FFE4  bne 0x82efb7c0
	if !ctx.cr[0].eq {
	pc = 0x82EFB7C0; continue 'dispatch;
	}
	// 82EFB7E0: 4800000C  b 0x82efb7ec
	pc = 0x82EFB7EC; continue 'dispatch;
            }
            0x82EFB7E4 => {
    //   block [0x82EFB7E4..0x82EFB7EC)
	// 82EFB7E4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB7E8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	pc = 0x82EFB7EC; continue 'dispatch;
            }
            0x82EFB7EC => {
    //   block [0x82EFB7EC..0x82EFB800)
	// 82EFB7EC: 7D4A5378  mr r10, r10
	ctx.r[10].u64 = ctx.r[10].u64;
	// 82EFB7F0: 7C2004AC  lwsync
	// 82EFB7F4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB7F8: 409AFFBC  bne cr6, 0x82efb7b4
	if !ctx.cr[6].eq {
	pc = 0x82EFB7B4; continue 'dispatch;
	}
	// 82EFB7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB800 size=124
    let mut pc: u32 = 0x82EFB800;
    'dispatch: loop {
        match pc {
            0x82EFB800 => {
    //   block [0x82EFB800..0x82EFB87C)
	// 82EFB800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB804: 4BDADC09  bl 0x82ca940c
	ctx.lr = 0x82EFB808;
	sub_82CA93D0(ctx, base);
	// 82EFB808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB80C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFB810: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFB814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB818: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFB81C: 4BFFE06D  bl 0x82ef9888
	ctx.lr = 0x82EFB820;
	sub_82EF9888(ctx, base);
	// 82EFB820: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB824: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFB828: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFB82C: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFB830: 394AD080  addi r10, r10, -0x2f80
	ctx.r[10].s64 = ctx.r[10].s64 + -12160;
	// 82EFB834: 3929D06C  addi r9, r9, -0x2f94
	ctx.r[9].s64 = ctx.r[9].s64 + -12180;
	// 82EFB838: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFB83C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFB840: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFB844: 7C2004AC  lwsync
	// 82EFB848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFB84C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFB850: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFB854: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EFB858: 7C2004AC  lwsync
	// 82EFB85C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFB860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB864: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EFB868: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82EFB86C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EFB870: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82EFB874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB878: 4BDADBE4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB8A0 size=124
    let mut pc: u32 = 0x82EFB8A0;
    'dispatch: loop {
        match pc {
            0x82EFB8A0 => {
    //   block [0x82EFB8A0..0x82EFB8DC)
	// 82EFB8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB8B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB8B4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB8B8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB8BC: 41820048  beq 0x82efb904
	if ctx.cr[0].eq {
	pc = 0x82EFB904; continue 'dispatch;
	}
	// 82EFB8C0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFB8C4: 4BDC6C7D  bl 0x82cc2540
	ctx.lr = 0x82EFB8C8;
	sub_82CC2540(ctx, base);
	// 82EFB8C8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EFB8CC: 419A0038  beq cr6, 0x82efb904
	if ctx.cr[6].eq {
	pc = 0x82EFB904; continue 'dispatch;
	}
	// 82EFB8D0: 7C2004AC  lwsync
	// 82EFB8D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFB8D8: 393F001C  addi r9, r31, 0x1c
	ctx.r[9].s64 = ctx.r[31].s64 + 28;
	pc = 0x82EFB8DC; continue 'dispatch;
            }
            0x82EFB8DC => {
    //   block [0x82EFB8DC..0x82EFB904)
	// 82EFB8DC: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFB8E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB8E4: 7D404828  lwarx r10, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB8E8: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFB8EC: 7D00492D  stwcx. r8, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB8F0: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB8F4: 4082FFE8  bne 0x82efb8dc
	if !ctx.cr[0].eq {
	pc = 0x82EFB8DC; continue 'dispatch;
	}
	// 82EFB8F8: 7C2004AC  lwsync
	// 82EFB8FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB900: 48000008  b 0x82efb908
	pc = 0x82EFB908; continue 'dispatch;
            }
            0x82EFB904 => {
    //   block [0x82EFB904..0x82EFB908)
	// 82EFB904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFB908; continue 'dispatch;
            }
            0x82EFB908 => {
    //   block [0x82EFB908..0x82EFB91C)
	// 82EFB908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB920 size=172
    let mut pc: u32 = 0x82EFB920;
    'dispatch: loop {
        match pc {
            0x82EFB920 => {
    //   block [0x82EFB920..0x82EFB974)
	// 82EFB920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB92C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFB93C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EFB940: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82EFB944: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFB948: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB94C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFB950: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFB954: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB958: 4BFFE161  bl 0x82ef9ab8
	ctx.lr = 0x82EFB95C;
	sub_82EF9AB8(ctx, base);
	// 82EFB95C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB960: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB964: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFB968: 4182000C  beq 0x82efb974
	if ctx.cr[0].eq {
	pc = 0x82EFB974; continue 'dispatch;
	}
	// 82EFB96C: 48110055  bl 0x8300b9c0
	ctx.lr = 0x82EFB970;
	sub_8300B9C0(ctx, base);
	// 82EFB970: 48000010  b 0x82efb980
	pc = 0x82EFB980; continue 'dispatch;
            }
            0x82EFB974 => {
    //   block [0x82EFB974..0x82EFB980)
	// 82EFB974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFB978: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFB97C: 4BDCB5CD  bl 0x82cc6f48
	ctx.lr = 0x82EFB980;
	sub_82CC6F48(ctx, base);
	pc = 0x82EFB980; continue 'dispatch;
            }
            0x82EFB980 => {
    //   block [0x82EFB980..0x82EFB9A4)
	// 82EFB980: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB984: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFB988: 419A001C  beq cr6, 0x82efb9a4
	if ctx.cr[6].eq {
	pc = 0x82EFB9A4; continue 'dispatch;
	}
	// 82EFB98C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFB990: 409A0014  bne cr6, 0x82efb9a4
	if !ctx.cr[6].eq {
	pc = 0x82EFB9A4; continue 'dispatch;
	}
	// 82EFB994: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB998: 419A001C  beq cr6, 0x82efb9b4
	if ctx.cr[6].eq {
	pc = 0x82EFB9B4; continue 'dispatch;
	}
	// 82EFB99C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB9A0: 4BFFDFD1  bl 0x82ef9970
	ctx.lr = 0x82EFB9A4;
	sub_82EF9970(ctx, base);
	pc = 0x82EFB9A4; continue 'dispatch;
            }
            0x82EFB9A4 => {
    //   block [0x82EFB9A4..0x82EFB9B4)
	// 82EFB9A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB9A8: 419A000C  beq cr6, 0x82efb9b4
	if ctx.cr[6].eq {
	pc = 0x82EFB9B4; continue 'dispatch;
	}
	// 82EFB9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB9B0: 4BFFE099  bl 0x82ef9a48
	ctx.lr = 0x82EFB9B4;
	sub_82EF9A48(ctx, base);
	pc = 0x82EFB9B4; continue 'dispatch;
            }
            0x82EFB9B4 => {
    //   block [0x82EFB9B4..0x82EFB9CC)
	// 82EFB9B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB9C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFB9C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB9D0 size=100
    let mut pc: u32 = 0x82EFB9D0;
    'dispatch: loop {
        match pc {
            0x82EFB9D0 => {
    //   block [0x82EFB9D0..0x82EFBA0C)
	// 82EFB9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB9D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB9DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB9E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFB9EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB9F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB9F4: 419A0024  beq cr6, 0x82efba18
	if ctx.cr[6].eq {
	pc = 0x82EFBA18; continue 'dispatch;
	}
	// 82EFB9F8: 4BFFF079  bl 0x82efaa70
	ctx.lr = 0x82EFB9FC;
	sub_82EFAA70(ctx, base);
	// 82EFB9FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBA00: 4082000C  bne 0x82efba0c
	if !ctx.cr[0].eq {
	pc = 0x82EFBA0C; continue 'dispatch;
	}
	// 82EFBA04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFBA08: 48000014  b 0x82efba1c
	pc = 0x82EFBA1C; continue 'dispatch;
            }
            0x82EFBA0C => {
    //   block [0x82EFBA0C..0x82EFBA18)
	// 82EFBA0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBA14: 4BFFFF0D  bl 0x82efb920
	ctx.lr = 0x82EFBA18;
	sub_82EFB920(ctx, base);
	pc = 0x82EFBA18; continue 'dispatch;
            }
            0x82EFBA18 => {
    //   block [0x82EFBA18..0x82EFBA1C)
	// 82EFBA18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFBA1C; continue 'dispatch;
            }
            0x82EFBA1C => {
    //   block [0x82EFBA1C..0x82EFBA34)
	// 82EFBA1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBA28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBA2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBA38 size=100
    let mut pc: u32 = 0x82EFBA38;
    'dispatch: loop {
        match pc {
            0x82EFBA38 => {
    //   block [0x82EFBA38..0x82EFBA74)
	// 82EFBA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBA40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBA44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBA48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBA4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBA50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFBA54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBA58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBA5C: 419A0018  beq cr6, 0x82efba74
	if ctx.cr[6].eq {
	pc = 0x82EFBA74; continue 'dispatch;
	}
	// 82EFBA60: 4BFFFF71  bl 0x82efb9d0
	ctx.lr = 0x82EFBA64;
	sub_82EFB9D0(ctx, base);
	// 82EFBA64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBA68: 4082000C  bne 0x82efba74
	if !ctx.cr[0].eq {
	pc = 0x82EFBA74; continue 'dispatch;
	}
	// 82EFBA6C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82EFBA70: 48000014  b 0x82efba84
	pc = 0x82EFBA84; continue 'dispatch;
            }
            0x82EFBA74 => {
    //   block [0x82EFBA74..0x82EFBA84)
	// 82EFBA74: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFBA78: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82EFBA7C: 409A0008  bne cr6, 0x82efba84
	if !ctx.cr[6].eq {
	pc = 0x82EFBA84; continue 'dispatch;
	}
	// 82EFBA80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFBA84; continue 'dispatch;
            }
            0x82EFBA84 => {
    //   block [0x82EFBA84..0x82EFBA9C)
	// 82EFBA84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBA90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBA94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFBAA0 size=12
    let mut pc: u32 = 0x82EFBAA0;
    'dispatch: loop {
        match pc {
            0x82EFBAA0 => {
    //   block [0x82EFBAA0..0x82EFBAAC)
	// 82EFBAA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFBAA4: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBAA8: 4BFFFE78  b 0x82efb920
	sub_82EFB920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFBAC0 size=12
    let mut pc: u32 = 0x82EFBAC0;
    'dispatch: loop {
        match pc {
            0x82EFBAC0 => {
    //   block [0x82EFBAC0..0x82EFBACC)
	// 82EFBAC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFBAC4: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBAC8: 4BFFFF70  b 0x82efba38
	sub_82EFBA38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBAD0 size=52
    let mut pc: u32 = 0x82EFBAD0;
    'dispatch: loop {
        match pc {
            0x82EFBAD0 => {
    //   block [0x82EFBAD0..0x82EFBB04)
	// 82EFBAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBADC: 3883FFEC  addi r4, r3, -0x14
	ctx.r[4].s64 = ctx.r[3].s64 + -20;
	// 82EFBAE0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBAE4: 4BFFFEED  bl 0x82efb9d0
	ctx.lr = 0x82EFBAE8;
	sub_82EFB9D0(ctx, base);
	// 82EFBAE8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EFBAEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFBAF0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFBAF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFBAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBB08 size=44
    let mut pc: u32 = 0x82EFBB08;
    'dispatch: loop {
        match pc {
            0x82EFBB08 => {
    //   block [0x82EFBB08..0x82EFBB34)
	// 82EFBB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBB14: 3883FFEC  addi r4, r3, -0x14
	ctx.r[4].s64 = ctx.r[3].s64 + -20;
	// 82EFBB18: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBB1C: 4BFFFE05  bl 0x82efb920
	ctx.lr = 0x82EFBB20;
	sub_82EFB920(ctx, base);
	// 82EFBB20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFBB24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFBB28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBB2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBB38 size=144
    let mut pc: u32 = 0x82EFBB38;
    'dispatch: loop {
        match pc {
            0x82EFBB38 => {
    //   block [0x82EFBB38..0x82EFBBB0)
	// 82EFBB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBB40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBB44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBB48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBB4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFBB50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFBB54: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFBB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBB5C: 4BFFDF5D  bl 0x82ef9ab8
	ctx.lr = 0x82EFBB60;
	sub_82EF9AB8(ctx, base);
	// 82EFBB60: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 82EFBB64: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 82EFBB68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBB6C: 4BFFFBF5  bl 0x82efb760
	ctx.lr = 0x82EFBB70;
	sub_82EFB760(ctx, base);
	// 82EFBB70: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82EFBB74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBB78: 4BFFFC39  bl 0x82efb7b0
	ctx.lr = 0x82EFBB7C;
	sub_82EFB7B0(ctx, base);
	// 82EFBB7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBB80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFBB84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBB88: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBB8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBB90: 4E800421  bctrl
	ctx.lr = 0x82EFBB94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBB94: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBB98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFBB9C: 419A0014  beq cr6, 0x82efbbb0
	if ctx.cr[6].eq {
	pc = 0x82EFBBB0; continue 'dispatch;
	}
	// 82EFBBA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBBA4: 4BFFDDCD  bl 0x82ef9970
	ctx.lr = 0x82EFBBA8;
	sub_82EF9970(ctx, base);
	// 82EFBBA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBBAC: 4BFFDE9D  bl 0x82ef9a48
	ctx.lr = 0x82EFBBB0;
	sub_82EF9A48(ctx, base);
            }
            0x82EFBBB0 => {
    //   block [0x82EFBBB0..0x82EFBBC8)
	// 82EFBBB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBBBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBBC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBBC8 size=104
    let mut pc: u32 = 0x82EFBBC8;
    'dispatch: loop {
        match pc {
            0x82EFBBC8 => {
    //   block [0x82EFBBC8..0x82EFBC00)
	// 82EFBBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBBD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBBD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBBD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBBDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBBE0: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 82EFBBE4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBBE8: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBBEC: 41820014  beq 0x82efbc00
	if ctx.cr[0].eq {
	pc = 0x82EFBC00; continue 'dispatch;
	}
	// 82EFBBF0: 4BFFFCB1  bl 0x82efb8a0
	ctx.lr = 0x82EFBBF4;
	sub_82EFB8A0(ctx, base);
	// 82EFBBF4: 3880FFF7  li r4, -9
	ctx.r[4].s64 = -9;
	// 82EFBBF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBBFC: 4BFFFB65  bl 0x82efb760
	ctx.lr = 0x82EFBC00;
	sub_82EFB760(ctx, base);
	pc = 0x82EFBC00; continue 'dispatch;
            }
            0x82EFBC00 => {
    //   block [0x82EFBC00..0x82EFBC30)
	// 82EFBC00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBC04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBC08: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFBC0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBC10: 4E800421  bctrl
	ctx.lr = 0x82EFBC14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBC14: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82EFBC18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBC24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBC28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBC30 size=120
    let mut pc: u32 = 0x82EFBC30;
    'dispatch: loop {
        match pc {
            0x82EFBC30 => {
    //   block [0x82EFBC30..0x82EFBC7C)
	// 82EFBC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBC38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBC3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBC40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBC44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBC48: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EFBC4C: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EFBC50: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBC54: 4BFFEDD5  bl 0x82efaa28
	ctx.lr = 0x82EFBC58;
	sub_82EFAA28(ctx, base);
	// 82EFBC58: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EFBC5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBC60: 48066DB1  bl 0x82f62a10
	ctx.lr = 0x82EFBC64;
	sub_82F62A10(ctx, base);
	// 82EFBC64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBC68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBC6C: 419A0010  beq cr6, 0x82efbc7c
	if ctx.cr[6].eq {
	pc = 0x82EFBC7C; continue 'dispatch;
	}
	// 82EFBC70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBC74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBC78: 409A000C  bne cr6, 0x82efbc84
	if !ctx.cr[6].eq {
	pc = 0x82EFBC84; continue 'dispatch;
	}
	pc = 0x82EFBC7C; continue 'dispatch;
            }
            0x82EFBC7C => {
    //   block [0x82EFBC7C..0x82EFBC84)
	// 82EFBC7C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFBC80: 4BFFF231  bl 0x82efaeb0
	ctx.lr = 0x82EFBC84;
	sub_82EFAEB0(ctx, base);
	pc = 0x82EFBC84; continue 'dispatch;
            }
            0x82EFBC84 => {
    //   block [0x82EFBC84..0x82EFBCA8)
	// 82EFBC84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBC88: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBC8C: 4BFFFC95  bl 0x82efb920
	ctx.lr = 0x82EFBC90;
	sub_82EFB920(ctx, base);
	// 82EFBC90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBC9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBCA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBCA8 size=116
    let mut pc: u32 = 0x82EFBCA8;
    'dispatch: loop {
        match pc {
            0x82EFBCA8 => {
    //   block [0x82EFBCA8..0x82EFBCCC)
	// 82EFBCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBCB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBCB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBCB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBCBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBCC0: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EFBCC4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBCC8: 4BFFED61  bl 0x82efaa28
	ctx.lr = 0x82EFBCCC;
	sub_82EFAA28(ctx, base);
	pc = 0x82EFBCCC; continue 'dispatch;
            }
            0x82EFBCCC => {
    //   block [0x82EFBCCC..0x82EFBCF8)
	// 82EFBCCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBCD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBCD4: 419A0024  beq cr6, 0x82efbcf8
	if ctx.cr[6].eq {
	pc = 0x82EFBCF8; continue 'dispatch;
	}
	// 82EFBCD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBCDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBCE0: 419A0018  beq cr6, 0x82efbcf8
	if ctx.cr[6].eq {
	pc = 0x82EFBCF8; continue 'dispatch;
	}
	// 82EFBCE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFBCE8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFBCEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBCF0: 4BFFF061  bl 0x82efad50
	ctx.lr = 0x82EFBCF4;
	sub_82EFAD50(ctx, base);
	// 82EFBCF4: 4BFFFFD8  b 0x82efbccc
	pc = 0x82EFBCCC; continue 'dispatch;
            }
            0x82EFBCF8 => {
    //   block [0x82EFBCF8..0x82EFBD1C)
	// 82EFBCF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBCFC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBD00: 4BFFFC21  bl 0x82efb920
	ctx.lr = 0x82EFBD04;
	sub_82EFB920(ctx, base);
	// 82EFBD04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBD10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBD14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBD20 size=124
    let mut pc: u32 = 0x82EFBD20;
    'dispatch: loop {
        match pc {
            0x82EFBD20 => {
    //   block [0x82EFBD20..0x82EFBD58)
	// 82EFBD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBD24: 4BDAD6E9  bl 0x82ca940c
	ctx.lr = 0x82EFBD28;
	sub_82CA93D0(ctx, base);
	// 82EFBD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBD2C: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EFBD30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFBD34: 817F8FB8  lwz r11, -0x7048(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBD38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBD3C: 409A0024  bne cr6, 0x82efbd60
	if !ctx.cr[6].eq {
	pc = 0x82EFBD60; continue 'dispatch;
	}
	// 82EFBD40: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFBD44: 48001B1D  bl 0x82efd860
	ctx.lr = 0x82EFBD48;
	sub_82EFD860(ctx, base);
	// 82EFBD48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFBD4C: 4182000C  beq 0x82efbd58
	if ctx.cr[0].eq {
	pc = 0x82EFBD58; continue 'dispatch;
	}
	// 82EFBD50: 4BFFF8D9  bl 0x82efb628
	ctx.lr = 0x82EFBD54;
	sub_82EFB628(ctx, base);
	// 82EFBD54: 48000008  b 0x82efbd5c
	pc = 0x82EFBD5C; continue 'dispatch;
            }
            0x82EFBD58 => {
    //   block [0x82EFBD58..0x82EFBD5C)
	// 82EFBD58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82EFBD5C; continue 'dispatch;
            }
            0x82EFBD5C => {
    //   block [0x82EFBD5C..0x82EFBD60)
	// 82EFBD5C: 907F8FB8  stw r3, -0x7048(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28744 as u32), ctx.r[3].u32 ) };
	pc = 0x82EFBD60; continue 'dispatch;
            }
            0x82EFBD60 => {
    //   block [0x82EFBD60..0x82EFBD9C)
	// 82EFBD60: 83FF8FB8  lwz r31, -0x7048(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBD64: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82EFBD68: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EFBD6C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBD70: 4BFFECB9  bl 0x82efaa28
	ctx.lr = 0x82EFBD74;
	sub_82EFAA28(ctx, base);
	// 82EFBD74: 57ABD1BE  srwi r11, r29, 6
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shr(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFBD78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFBD7C: 7D65EA78  xor r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 ^ ctx.r[29].u64;
	// 82EFBD80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBD84: 4BFFF4E5  bl 0x82efb268
	ctx.lr = 0x82EFBD88;
	sub_82EFB268(ctx, base);
	// 82EFBD88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBD8C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBD90: 4BFFFB91  bl 0x82efb920
	ctx.lr = 0x82EFBD94;
	sub_82EFB920(ctx, base);
	// 82EFBD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFBD98: 4BDAD6C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBDA0 size=124
    let mut pc: u32 = 0x82EFBDA0;
    'dispatch: loop {
        match pc {
            0x82EFBDA0 => {
    //   block [0x82EFBDA0..0x82EFBDFC)
	// 82EFBDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBDA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBDAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBDB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBDB4: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EFBDB8: 817F8FB8  lwz r11, -0x7048(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBDBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBDC0: 419A0044  beq cr6, 0x82efbe04
	if ctx.cr[6].eq {
	pc = 0x82EFBE04; continue 'dispatch;
	}
	// 82EFBDC4: 807F8FB8  lwz r3, -0x7048(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBDC8: 4BFFFEE1  bl 0x82efbca8
	ctx.lr = 0x82EFBDCC;
	sub_82EFBCA8(ctx, base);
	// 82EFBDCC: 817F8FB8  lwz r11, -0x7048(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBDD0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFBDD4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBDD8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFBDDC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBDE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBDE4: 419A0018  beq cr6, 0x82efbdfc
	if ctx.cr[6].eq {
	pc = 0x82EFBDFC; continue 'dispatch;
	}
	// 82EFBDE8: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBDEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBDF0: 4BFFF891  bl 0x82efb680
	ctx.lr = 0x82EFBDF4;
	sub_82EFB680(ctx, base);
	// 82EFBDF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBDF8: 48001AA9  bl 0x82efd8a0
	ctx.lr = 0x82EFBDFC;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFBDFC; continue 'dispatch;
            }
            0x82EFBDFC => {
    //   block [0x82EFBDFC..0x82EFBE04)
	// 82EFBDFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFBE00: 917F8FB8  stw r11, -0x7048(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28744 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFBE04; continue 'dispatch;
            }
            0x82EFBE04 => {
    //   block [0x82EFBE04..0x82EFBE1C)
	// 82EFBE04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBE10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBE14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFBE20 size=4
    let mut pc: u32 = 0x82EFBE20;
    'dispatch: loop {
        match pc {
            0x82EFBE20 => {
    //   block [0x82EFBE20..0x82EFBE24)
	// 82EFBE20: 4BFFFF80  b 0x82efbda0
	sub_82EFBDA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBE28 size=108
    let mut pc: u32 = 0x82EFBE28;
    'dispatch: loop {
        match pc {
            0x82EFBE28 => {
    //   block [0x82EFBE28..0x82EFBE54)
	// 82EFBE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBE30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBE34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBE38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBE3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBE40: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFBE44: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82EFBE48: 419A000C  beq cr6, 0x82efbe54
	if ctx.cr[6].eq {
	pc = 0x82EFBE54; continue 'dispatch;
	}
	// 82EFBE4C: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82EFBE50: 4BDC6821  bl 0x82cc2670
	ctx.lr = 0x82EFBE54;
	sub_82CC2670(ctx, base);
	pc = 0x82EFBE54; continue 'dispatch;
            }
            0x82EFBE54 => {
    //   block [0x82EFBE54..0x82EFBE94)
	// 82EFBE54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBE58: 4BFFFD71  bl 0x82efbbc8
	ctx.lr = 0x82EFBE5C;
	sub_82EFBBC8(ctx, base);
	// 82EFBE5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFBE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBE64: 4BFFFCD5  bl 0x82efbb38
	ctx.lr = 0x82EFBE68;
	sub_82EFBB38(ctx, base);
	// 82EFBE68: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFBE6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFBE70: 806B8FB8  lwz r3, -0x7048(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBE74: 4BFFFDBD  bl 0x82efbc30
	ctx.lr = 0x82EFBE78;
	sub_82EFBC30(ctx, base);
	// 82EFBE78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBE7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBE88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBE8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBE98 size=296
    let mut pc: u32 = 0x82EFBE98;
    'dispatch: loop {
        match pc {
            0x82EFBE98 => {
    //   block [0x82EFBE98..0x82EFBEB4)
	// 82EFBE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBE9C: 4BDAD56D  bl 0x82ca9408
	ctx.lr = 0x82EFBEA0;
	sub_82CA93D0(ctx, base);
	// 82EFBEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBEA4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFBEA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBEAC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EFBEB0: 409A000C  bne cr6, 0x82efbebc
	if !ctx.cr[6].eq {
	pc = 0x82EFBEBC; continue 'dispatch;
	}
	pc = 0x82EFBEB4; continue 'dispatch;
            }
            0x82EFBEB4 => {
    //   block [0x82EFBEB4..0x82EFBEBC)
	// 82EFBEB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFBEB8: 48000100  b 0x82efbfb8
	pc = 0x82EFBFB8; continue 'dispatch;
            }
            0x82EFBEBC => {
    //   block [0x82EFBEBC..0x82EFBED8)
	// 82EFBEBC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBEC0: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82EFBEC4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFBEC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBECC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFBED0: 41990008  bgt cr6, 0x82efbed8
	if ctx.cr[6].gt {
	pc = 0x82EFBED8; continue 'dispatch;
	}
	// 82EFBED4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x82EFBED8; continue 'dispatch;
            }
            0x82EFBED8 => {
    //   block [0x82EFBED8..0x82EFBEEC)
	// 82EFBED8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBEDC: 40820010  bne 0x82efbeec
	if !ctx.cr[0].eq {
	pc = 0x82EFBEEC; continue 'dispatch;
	}
	// 82EFBEE0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBEE4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBEE8: 41820018  beq 0x82efbf00
	if ctx.cr[0].eq {
	pc = 0x82EFBF00; continue 'dispatch;
	}
	pc = 0x82EFBEEC; continue 'dispatch;
            }
            0x82EFBEEC => {
    //   block [0x82EFBEEC..0x82EFBF00)
	// 82EFBEEC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EFBEF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBEF4: 4BFFE42D  bl 0x82efa320
	ctx.lr = 0x82EFBEF8;
	sub_82EFA320(ctx, base);
	// 82EFBEF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBEFC: 4182FFB8  beq 0x82efbeb4
	if ctx.cr[0].eq {
	pc = 0x82EFBEB4; continue 'dispatch;
	}
	pc = 0x82EFBF00; continue 'dispatch;
            }
            0x82EFBF00 => {
    //   block [0x82EFBF00..0x82EFBF14)
	// 82EFBF00: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFBF04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFBF08: 419A000C  beq cr6, 0x82efbf14
	if ctx.cr[6].eq {
	pc = 0x82EFBF14; continue 'dispatch;
	}
	// 82EFBF0C: 4BDC68A5  bl 0x82cc27b0
	ctx.lr = 0x82EFBF10;
	sub_82CC27B0(ctx, base);
	// 82EFBF10: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	pc = 0x82EFBF14; continue 'dispatch;
            }
            0x82EFBF14 => {
    //   block [0x82EFBF14..0x82EFBFB4)
	// 82EFBF14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBF1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBF24: 4E800421  bctrl
	ctx.lr = 0x82EFBF28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBF28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBF2C: 4BFFFDF5  bl 0x82efbd20
	ctx.lr = 0x82EFBF30;
	sub_82EFBD20(ctx, base);
	// 82EFBF30: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82EFBF34: 7C2004AC  lwsync
	// 82EFBF38: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFBF3C: 7C2004AC  lwsync
	// 82EFBF40: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 82EFBF44: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFBF48: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFBF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFBF50: 556BF738  rlwinm r11, r11, 0x1e, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82EFBF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFBF58: 696B0008  xori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 ^ 8;
	// 82EFBF5C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EFBF60: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFBF64: 38AABE28  addi r5, r10, -0x41d8
	ctx.r[5].s64 = ctx.r[10].s64 + -16856;
	// 82EFBF68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFBF6C: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFBF70: 4810D589  bl 0x830094f8
	ctx.lr = 0x82EFBF74;
	sub_830094F8(ctx, base);
	// 82EFBF74: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82EFBF78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFBF7C: 40820038  bne 0x82efbfb4
	if !ctx.cr[0].eq {
	pc = 0x82EFBFB4; continue 'dispatch;
	}
	// 82EFBF80: 7C2004AC  lwsync
	// 82EFBF84: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EFBF88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBF8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFBF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBF94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBF98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBF9C: 4E800421  bctrl
	ctx.lr = 0x82EFBFA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBFA0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFBFA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFBFA8: 806B8FB8  lwz r3, -0x7048(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBFAC: 4BFFFC85  bl 0x82efbc30
	ctx.lr = 0x82EFBFB0;
	sub_82EFBC30(ctx, base);
	// 82EFBFB0: 4BFFFF04  b 0x82efbeb4
	pc = 0x82EFBEB4; continue 'dispatch;
            }
            0x82EFBFB4 => {
    //   block [0x82EFBFB4..0x82EFBFB8)
	// 82EFBFB4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82EFBFB8; continue 'dispatch;
            }
            0x82EFBFB8 => {
    //   block [0x82EFBFB8..0x82EFBFC0)
	// 82EFBFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFBFBC: 4BDAD49C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBFC0 size=100
    let mut pc: u32 = 0x82EFBFC0;
    'dispatch: loop {
        match pc {
            0x82EFBFC0 => {
    //   block [0x82EFBFC0..0x82EFC024)
	// 82EFBFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBFC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBFCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBFD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBFD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBFD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFBFDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBFE0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFBFE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBFE8: 4E800421  bctrl
	ctx.lr = 0x82EFBFEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBFEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBFF0: 4BFFFB49  bl 0x82efbb38
	ctx.lr = 0x82EFBFF4;
	sub_82EFBB38(ctx, base);
	// 82EFBFF4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFBFF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFBFFC: 806B8FB8  lwz r3, -0x7048(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFC000: 4BFFFC31  bl 0x82efbc30
	ctx.lr = 0x82EFC004;
	sub_82EFBC30(ctx, base);
	// 82EFC004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC008: 4810D3C9  bl 0x830093d0
	ctx.lr = 0x82EFC00C;
	sub_830093D0(ctx, base);
	// 82EFC00C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC018: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC01C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC028 size=156
    let mut pc: u32 = 0x82EFC028;
    'dispatch: loop {
        match pc {
            0x82EFC028 => {
    //   block [0x82EFC028..0x82EFC0B8)
	// 82EFC028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC02C: 4BDAD3D5  bl 0x82ca9400
	ctx.lr = 0x82EFC030;
	sub_82CA93D0(ctx, base);
	// 82EFC030: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC034: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFC038: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFC03C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC040: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EFC044: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EFC048: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82EFC04C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EFC050: 4BFFD839  bl 0x82ef9888
	ctx.lr = 0x82EFC054;
	sub_82EF9888(ctx, base);
	// 82EFC054: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC058: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC05C: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFC060: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFC064: 394AD080  addi r10, r10, -0x2f80
	ctx.r[10].s64 = ctx.r[10].s64 + -12160;
	// 82EFC068: 3929D06C  addi r9, r9, -0x2f94
	ctx.r[9].s64 = ctx.r[9].s64 + -12180;
	// 82EFC06C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFC070: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFC074: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFC078: 7C2004AC  lwsync
	// 82EFC07C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC080: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFC084: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFC088: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EFC08C: 7C2004AC  lwsync
	// 82EFC090: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFC094: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFC098: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82EFC09C: 935F002C  stw r26, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82EFC0A0: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82EFC0A4: 939F0034  stw r28, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[28].u32 ) };
	// 82EFC0A8: 419A0010  beq cr6, 0x82efc0b8
	if ctx.cr[6].eq {
	pc = 0x82EFC0B8; continue 'dispatch;
	}
	// 82EFC0AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFC0B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC0B4: 4BFFFDE5  bl 0x82efbe98
	ctx.lr = 0x82EFC0B8;
	sub_82EFBE98(ctx, base);
	pc = 0x82EFC0B8; continue 'dispatch;
            }
            0x82EFC0B8 => {
    //   block [0x82EFC0B8..0x82EFC0C4)
	// 82EFC0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC0BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFC0C0: 4BDAD390  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC0C8 size=68
    let mut pc: u32 = 0x82EFC0C8;
    'dispatch: loop {
        match pc {
            0x82EFC0C8 => {
    //   block [0x82EFC0C8..0x82EFC0F4)
	// 82EFC0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC0D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC0D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC0DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC0E0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFC0E4: 396BD0A4  addi r11, r11, -0x2f5c
	ctx.r[11].s64 = ctx.r[11].s64 + -12124;
	// 82EFC0E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC0EC: 41820008  beq 0x82efc0f4
	if ctx.cr[0].eq {
	pc = 0x82EFC0F4; continue 'dispatch;
	}
	// 82EFC0F0: 4B9496C1  bl 0x828457b0
	ctx.lr = 0x82EFC0F4;
	sub_828457B0(ctx, base);
	pc = 0x82EFC0F4; continue 'dispatch;
            }
            0x82EFC0F4 => {
    //   block [0x82EFC0F4..0x82EFC10C)
	// 82EFC0F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC110 size=84
    let mut pc: u32 = 0x82EFC110;
    'dispatch: loop {
        match pc {
            0x82EFC110 => {
    //   block [0x82EFC110..0x82EFC164)
	// 82EFC110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC11C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC124: 48001825  bl 0x82efd948
	ctx.lr = 0x82EFC128;
	sub_82EFD948(ctx, base);
	// 82EFC128: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC12C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC130: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFC134: 396BD0A4  addi r11, r11, -0x2f5c
	ctx.r[11].s64 = ctx.r[11].s64 + -12124;
	// 82EFC138: 394AD0BC  addi r10, r10, -0x2f44
	ctx.r[10].s64 = ctx.r[10].s64 + -12100;
	// 82EFC13C: 3929D0B0  addi r9, r9, -0x2f50
	ctx.r[9].s64 = ctx.r[9].s64 + -12112;
	// 82EFC140: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EFC144: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFC148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC14C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82EFC150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC15C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC168 size=8
    let mut pc: u32 = 0x82EFC168;
    'dispatch: loop {
        match pc {
            0x82EFC168 => {
    //   block [0x82EFC168..0x82EFC170)
	// 82EFC168: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82EFC16C: 48000004  b 0x82efc170
	sub_82EFC170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC170 size=108
    let mut pc: u32 = 0x82EFC170;
    'dispatch: loop {
        match pc {
            0x82EFC170 => {
    //   block [0x82EFC170..0x82EFC19C)
	// 82EFC170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC17C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFC18C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFC190: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82EFC194: 409A0008  bne cr6, 0x82efc19c
	if !ctx.cr[6].eq {
	pc = 0x82EFC19C; continue 'dispatch;
	}
	// 82EFC198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82EFC19C; continue 'dispatch;
            }
            0x82EFC19C => {
    //   block [0x82EFC19C..0x82EFC1C0)
	// 82EFC19C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC1A4: 394AD0A4  addi r10, r10, -0x2f5c
	ctx.r[10].s64 = ctx.r[10].s64 + -12124;
	// 82EFC1A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFC1AC: 4800186D  bl 0x82efda18
	ctx.lr = 0x82EFC1B0;
	sub_82EFDA18(ctx, base);
	// 82EFC1B0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC1B4: 4182000C  beq 0x82efc1c0
	if ctx.cr[0].eq {
	pc = 0x82EFC1C0; continue 'dispatch;
	}
	// 82EFC1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC1BC: 480016E5  bl 0x82efd8a0
	ctx.lr = 0x82EFC1C0;
	sub_82EFD8A0(ctx, base);
	pc = 0x82EFC1C0; continue 'dispatch;
            }
            0x82EFC1C0 => {
    //   block [0x82EFC1C0..0x82EFC1DC)
	// 82EFC1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC1C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC1D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC1D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC1E0 size=128
    let mut pc: u32 = 0x82EFC1E0;
    'dispatch: loop {
        match pc {
            0x82EFC1E0 => {
    //   block [0x82EFC1E0..0x82EFC230)
	// 82EFC1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC1E4: 4BDAD229  bl 0x82ca940c
	ctx.lr = 0x82EFC1E8;
	sub_82CA93D0(ctx, base);
	// 82EFC1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC1EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFC1F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFC1F4: 4BFFFF1D  bl 0x82efc110
	ctx.lr = 0x82EFC1F8;
	sub_82EFC110(ctx, base);
	// 82EFC1F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC1FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC200: 396BD0DC  addi r11, r11, -0x2f24
	ctx.r[11].s64 = ctx.r[11].s64 + -12068;
	// 82EFC204: 394AD0D0  addi r10, r10, -0x2f30
	ctx.r[10].s64 = ctx.r[10].s64 + -12080;
	// 82EFC208: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 82EFC20C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC210: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC214: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFC218: 419A0018  beq cr6, 0x82efc230
	if ctx.cr[6].eq {
	pc = 0x82EFC230; continue 'dispatch;
	}
	// 82EFC21C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC224: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC22C: 4E800421  bctrl
	ctx.lr = 0x82EFC230;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC230 => {
    //   block [0x82EFC230..0x82EFC254)
	// 82EFC230: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82EFC234: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFC238: 419A001C  beq cr6, 0x82efc254
	if ctx.cr[6].eq {
	pc = 0x82EFC254; continue 'dispatch;
	}
	// 82EFC23C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC240: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFC244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC248: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFC24C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC250: 4E800421  bctrl
	ctx.lr = 0x82EFC254;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC254 => {
    //   block [0x82EFC254..0x82EFC260)
	// 82EFC254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC25C: 4BDAD200  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC260 size=8
    let mut pc: u32 = 0x82EFC260;
    'dispatch: loop {
        match pc {
            0x82EFC260 => {
    //   block [0x82EFC260..0x82EFC268)
	// 82EFC260: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82EFC264: 4800069C  b 0x82efc900
	sub_82EFC900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC268 size=160
    let mut pc: u32 = 0x82EFC268;
    'dispatch: loop {
        match pc {
            0x82EFC268 => {
    //   block [0x82EFC268..0x82EFC2BC)
	// 82EFC268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC27C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC280: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC284: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC288: 396BD0DC  addi r11, r11, -0x2f24
	ctx.r[11].s64 = ctx.r[11].s64 + -12068;
	// 82EFC28C: 394AD0D0  addi r10, r10, -0x2f30
	ctx.r[10].s64 = ctx.r[10].s64 + -12080;
	// 82EFC290: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC294: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82EFC298: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC29C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC2A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC2A4: 419A0018  beq cr6, 0x82efc2bc
	if ctx.cr[6].eq {
	pc = 0x82EFC2BC; continue 'dispatch;
	}
	// 82EFC2A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC2AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFC2B0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFC2B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC2B8: 4E800421  bctrl
	ctx.lr = 0x82EFC2BC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC2BC => {
    //   block [0x82EFC2BC..0x82EFC2DC)
	// 82EFC2BC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC2C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC2C4: 419A0018  beq cr6, 0x82efc2dc
	if ctx.cr[6].eq {
	pc = 0x82EFC2DC; continue 'dispatch;
	}
	// 82EFC2C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC2CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC2D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC2D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC2D8: 4E800421  bctrl
	ctx.lr = 0x82EFC2DC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC2DC => {
    //   block [0x82EFC2DC..0x82EFC308)
	// 82EFC2DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC2E4: 396BD0A4  addi r11, r11, -0x2f5c
	ctx.r[11].s64 = ctx.r[11].s64 + -12124;
	// 82EFC2E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC2EC: 4800172D  bl 0x82efda18
	ctx.lr = 0x82EFC2F0;
	sub_82EFDA18(ctx, base);
	// 82EFC2F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC2FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC308 size=220
    let mut pc: u32 = 0x82EFC308;
    'dispatch: loop {
        match pc {
            0x82EFC308 => {
    //   block [0x82EFC308..0x82EFC364)
	// 82EFC308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC30C: 4BDAD101  bl 0x82ca940c
	ctx.lr = 0x82EFC310;
	sub_82CA93D0(ctx, base);
	// 82EFC310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC318: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFC31C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC320: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC324: 409A00B8  bne cr6, 0x82efc3dc
	if !ctx.cr[6].eq {
	pc = 0x82EFC3DC; continue 'dispatch;
	}
	// 82EFC328: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC32C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFC330: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC334: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC338: 4E800421  bctrl
	ctx.lr = 0x82EFC33C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC33C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC340: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFC344: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFC348: 419A001C  beq cr6, 0x82efc364
	if ctx.cr[6].eq {
	pc = 0x82EFC364; continue 'dispatch;
	}
	// 82EFC34C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC350: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFC354: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC358: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC35C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC360: 4E800421  bctrl
	ctx.lr = 0x82EFC364;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC364 => {
    //   block [0x82EFC364..0x82EFC3B4)
	// 82EFC364: 57CB003E  slwi r11, r30, 0
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFC368: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EFC36C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC370: 419A0068  beq cr6, 0x82efc3d8
	if ctx.cr[6].eq {
	pc = 0x82EFC3D8; continue 'dispatch;
	}
	// 82EFC374: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFC378: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82EFC37C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFC380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC384: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC38C: 4E800421  bctrl
	ctx.lr = 0x82EFC390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC390: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC394: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC398: 4182001C  beq 0x82efc3b4
	if ctx.cr[0].eq {
	pc = 0x82EFC3B4; continue 'dispatch;
	}
	// 82EFC39C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC3A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFC3A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFC3A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC3AC: 4E800421  bctrl
	ctx.lr = 0x82EFC3B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC3B0: 48000028  b 0x82efc3d8
	pc = 0x82EFC3D8; continue 'dispatch;
            }
            0x82EFC3B4 => {
    //   block [0x82EFC3B4..0x82EFC3D0)
	// 82EFC3B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC3B8: 419A0018  beq cr6, 0x82efc3d0
	if ctx.cr[6].eq {
	pc = 0x82EFC3D0; continue 'dispatch;
	}
	// 82EFC3BC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC3C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC3C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC3C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC3CC: 4E800421  bctrl
	ctx.lr = 0x82EFC3D0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC3D0 => {
    //   block [0x82EFC3D0..0x82EFC3D8)
	// 82EFC3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC3D4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFC3D8; continue 'dispatch;
            }
            0x82EFC3D8 => {
    //   block [0x82EFC3D8..0x82EFC3DC)
	// 82EFC3D8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	pc = 0x82EFC3DC; continue 'dispatch;
            }
            0x82EFC3DC => {
    //   block [0x82EFC3DC..0x82EFC3E4)
	// 82EFC3DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC3E0: 4BDAD07C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC3E8 size=132
    let mut pc: u32 = 0x82EFC3E8;
    'dispatch: loop {
        match pc {
            0x82EFC3E8 => {
    //   block [0x82EFC3E8..0x82EFC420)
	// 82EFC3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC3F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC3F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC3F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC3FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC400: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC404: 419A0054  beq cr6, 0x82efc458
	if ctx.cr[6].eq {
	pc = 0x82EFC458; continue 'dispatch;
	}
	// 82EFC408: 2F050002  cmpwi cr6, r5, 2
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2, &mut ctx.xer);
	// 82EFC40C: 409A004C  bne cr6, 0x82efc458
	if !ctx.cr[6].eq {
	pc = 0x82EFC458; continue 'dispatch;
	}
	// 82EFC410: 357FFFF0  addic. r11, r31, -0x10
	ctx.xer.ca = (ctx.r[31].u32 > (!(-16 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC414: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFC418: 40820008  bne 0x82efc420
	if !ctx.cr[0].eq {
	pc = 0x82EFC420; continue 'dispatch;
	}
	// 82EFC41C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	pc = 0x82EFC420; continue 'dispatch;
            }
            0x82EFC420 => {
    //   block [0x82EFC420..0x82EFC450)
	// 82EFC420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC424: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFC428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC42C: 4E800421  bctrl
	ctx.lr = 0x82EFC430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC430: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC438: 419A0018  beq cr6, 0x82efc450
	if ctx.cr[6].eq {
	pc = 0x82EFC450; continue 'dispatch;
	}
	// 82EFC43C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC440: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC444: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC44C: 4E800421  bctrl
	ctx.lr = 0x82EFC450;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC450 => {
    //   block [0x82EFC450..0x82EFC458)
	// 82EFC450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC454: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82EFC458; continue 'dispatch;
            }
            0x82EFC458 => {
    //   block [0x82EFC458..0x82EFC46C)
	// 82EFC458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC470 size=84
    let mut pc: u32 = 0x82EFC470;
    'dispatch: loop {
        match pc {
            0x82EFC470 => {
    //   block [0x82EFC470..0x82EFC4C4)
	// 82EFC470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC474: 4BDACF99  bl 0x82ca940c
	ctx.lr = 0x82EFC478;
	sub_82CA93D0(ctx, base);
	// 82EFC478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC47C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC480: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFC484: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EFC488: 4BFFFD59  bl 0x82efc1e0
	ctx.lr = 0x82EFC48C;
	sub_82EFC1E0(ctx, base);
	// 82EFC48C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC490: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC494: 392BD0FC  addi r9, r11, -0x2f04
	ctx.r[9].s64 = ctx.r[11].s64 + -12036;
	// 82EFC498: 394AD0F0  addi r10, r10, -0x2f10
	ctx.r[10].s64 = ctx.r[10].s64 + -12048;
	// 82EFC49C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC4A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EFC4A4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC4A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC4AC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFC4B0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82EFC4B4: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82EFC4B8: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82EFC4BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC4C0: 4BDACF9C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC4D0 size=104
    let mut pc: u32 = 0x82EFC4D0;
    'dispatch: loop {
        match pc {
            0x82EFC4D0 => {
    //   block [0x82EFC4D0..0x82EFC51C)
	// 82EFC4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC4D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC4DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC4E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC4E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC4E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC4EC: 396BD0FC  addi r11, r11, -0x2f04
	ctx.r[11].s64 = ctx.r[11].s64 + -12036;
	// 82EFC4F0: 394AD0F0  addi r10, r10, -0x2f10
	ctx.r[10].s64 = ctx.r[10].s64 + -12048;
	// 82EFC4F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC4F8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC4FC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC500: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC504: 419A0018  beq cr6, 0x82efc51c
	if ctx.cr[6].eq {
	pc = 0x82EFC51C; continue 'dispatch;
	}
	// 82EFC508: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC50C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC510: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC518: 4E800421  bctrl
	ctx.lr = 0x82EFC51C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82EFC51C => {
    //   block [0x82EFC51C..0x82EFC538)
	// 82EFC51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC520: 4BFFFD49  bl 0x82efc268
	ctx.lr = 0x82EFC524;
	sub_82EFC268(ctx, base);
	// 82EFC524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC538 size=36
    let mut pc: u32 = 0x82EFC538;
    'dispatch: loop {
        match pc {
            0x82EFC538 => {
    //   block [0x82EFC538..0x82EFC55C)
	// 82EFC538: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFC53C: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFC540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC544: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82EFC548: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC54C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC550: 419A000C  beq cr6, 0x82efc55c
	if ctx.cr[6].eq {
		sub_82EFC55C(ctx, base);
		return;
	}
	// 82EFC554: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


