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
    //   block [0x82DB3A98..0x82DB3B1C)
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
	pc = 0x82DB3B1C; continue 'dispatch;
            }
            0x82DB3B1C => {
    //   block [0x82DB3B1C..0x82DB3B24)
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
    // ---- function 0x82DB3B50 size=40
    let mut pc: u32 = 0x82DB3B50;
    'dispatch: loop {
        match pc {
            0x82DB3B50 => {
    //   block [0x82DB3B50..0x82DB3B78)
	// 82DB3B50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3B54: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
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
    //   block [0x82DB3B88..0x82DB3BBC)
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
	pc = 0x82DB3BBC; continue 'dispatch;
            }
            0x82DB3BBC => {
    //   block [0x82DB3BBC..0x82DB3BD0)
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
    // ---- function 0x82DB3BE8 size=56
    let mut pc: u32 = 0x82DB3BE8;
    'dispatch: loop {
        match pc {
            0x82DB3BE8 => {
    //   block [0x82DB3BE8..0x82DB3C20)
	// 82DB3BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB3BEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
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
    //   block [0x82DB3C30..0x82DB3C78)
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
	pc = 0x82DB3C78; continue 'dispatch;
            }
            0x82DB3C78 => {
    //   block [0x82DB3C78..0x82DB3C94)
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
    //   block [0x82DB3D98..0x82DB3E5C)
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
            0x82DB3E5C => {
    //   block [0x82DB3E5C..0x82DB3E74)
	// 82DB3E5C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DB3E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB3E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB3E68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DB3E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB3E70: 4E800020  blr
	return;
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
    //   block [0x82DB3EE8..0x82DB3F64)
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
            }
            0x82DB3F64 => {
    //   block [0x82DB3F64..0x82DB3FC8)
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
    //   block [0x82DB3FC8..0x82DB3FF0)
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
	pc = 0x82DB3FF0; continue 'dispatch;
            }
            0x82DB3FF0 => {
    //   block [0x82DB3FF0..0x82DB400C)
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
	pc = 0x82DB400C; continue 'dispatch;
            }
            0x82DB400C => {
    //   block [0x82DB400C..0x82DB4040)
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
            }
            0x82DB4040 => {
    //   block [0x82DB4040..0x82DB405C)
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
	pc = 0x82DB405C; continue 'dispatch;
            }
            0x82DB405C => {
    //   block [0x82DB405C..0x82DB4080)
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
	pc = 0x82DB4080; continue 'dispatch;
            }
            0x82DB4080 => {
    //   block [0x82DB4080..0x82DB4094)
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
	pc = 0x82DB4094; continue 'dispatch;
            }
            0x82DB4094 => {
    //   block [0x82DB4094..0x82DB40A0)
	// 82DB4094: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82DB4098: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DB409C: 4BEF53B0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DB40A0 => {
    //   block [0x82DB40A0..0x82DB40AC)
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
    // ---- function 0x82DB40B0 size=152
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
	pc = 0x82DB40F8; continue 'dispatch;
	}
	// 82DB40D8: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB40DC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82DB40E0: 409A000C  bne cr6, 0x82db40ec
	if !ctx.cr[6].eq {
	pc = 0x82DB40EC; continue 'dispatch;
	}
	// 82DB40E4: 39630070  addi r11, r3, 0x70
	ctx.r[11].s64 = ctx.r[3].s64 + 112;
	// 82DB40E8: 48000024  b 0x82db410c
	pc = 0x82DB410C; continue 'dispatch;
            }
            0x82DB40EC => {
    //   block [0x82DB40EC..0x82DB40F8)
	// 82DB40EC: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB40F0: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB40F4: 48000014  b 0x82db4108
	pc = 0x82DB4108; continue 'dispatch;
            }
            0x82DB40F8 => {
    //   block [0x82DB40F8..0x82DB4108)
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
	pc = 0x82DB4108; continue 'dispatch;
            }
            0x82DB4108 => {
    //   block [0x82DB4108..0x82DB410C)
	// 82DB4108: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	pc = 0x82DB410C; continue 'dispatch;
            }
            0x82DB410C => {
    //   block [0x82DB410C..0x82DB4148)
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
	ctx.r[11].s32 = ((ctx.r[6].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DB4134: 409A0014  bne cr6, 0x82db4148
	if !ctx.cr[6].eq {
		sub_82DB4148(ctx, base);
		return;
	}
	// 82DB4138: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB413C: 7D6B41D6  mullw r11, r11, r8
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
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
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
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
    //   block [0x82DB4580..0x82DB45FC)
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
	pc = 0x82DB45FC; continue 'dispatch;
            }
            0x82DB45FC => {
    //   block [0x82DB45FC..0x82DB4618)
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
	pc = 0x82DB4618; continue 'dispatch;
            }
            0x82DB4618 => {
    //   block [0x82DB4618..0x82DB4624)
	// 82DB4618: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DB461C: 40990038  ble cr6, 0x82db4654
	if !ctx.cr[6].gt {
	pc = 0x82DB4654; continue 'dispatch;
	}
	// 82DB4620: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	pc = 0x82DB4624; continue 'dispatch;
            }
            0x82DB4624 => {
    //   block [0x82DB4624..0x82DB4644)
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
	pc = 0x82DB4644; continue 'dispatch;
            }
            0x82DB4644 => {
    //   block [0x82DB4644..0x82DB4654)
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
	pc = 0x82DB4654; continue 'dispatch;
            }
            0x82DB4654 => {
    //   block [0x82DB4654..0x82DB4724)
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
    //   block [0x82DB4728..0x82DB47A4)
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
	pc = 0x82DB47A4; continue 'dispatch;
            }
            0x82DB47A4 => {
    //   block [0x82DB47A4..0x82DB47C0)
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
	pc = 0x82DB47C0; continue 'dispatch;
            }
            0x82DB47C0 => {
    //   block [0x82DB47C0..0x82DB47C8)
	// 82DB47C0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DB47C4: 40990034  ble cr6, 0x82db47f8
	if !ctx.cr[6].gt {
	pc = 0x82DB47F8; continue 'dispatch;
	}
	pc = 0x82DB47C8; continue 'dispatch;
            }
            0x82DB47C8 => {
    //   block [0x82DB47C8..0x82DB47E8)
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
	pc = 0x82DB47E8; continue 'dispatch;
            }
            0x82DB47E8 => {
    //   block [0x82DB47E8..0x82DB47F8)
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
	pc = 0x82DB47F8; continue 'dispatch;
            }
            0x82DB47F8 => {
    //   block [0x82DB47F8..0x82DB483C)
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
    //   block [0x82DB4840..0x82DB4864)
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
	pc = 0x82DB4864; continue 'dispatch;
            }
            0x82DB4864 => {
    //   block [0x82DB4864..0x82DB48A4)
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
            }
            0x82DB48A4 => {
    //   block [0x82DB48A4..0x82DB48B8)
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
	pc = 0x82DB48B8; continue 'dispatch;
            }
            0x82DB48B8 => {
    //   block [0x82DB48B8..0x82DB48E0)
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
    // ---- function 0x82DB48E0 size=44
    let mut pc: u32 = 0x82DB48E0;
    'dispatch: loop {
        match pc {
            0x82DB48E0 => {
    //   block [0x82DB48E0..0x82DB490C)
	// 82DB48E0: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DB48E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DB48E8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
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
    //   block [0x82DB4E50..0x82DB4EA8)
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
	pc = 0x82DB4EA8; continue 'dispatch;
            }
            0x82DB4EA8 => {
    //   block [0x82DB4EA8..0x82DB4ED4)
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
    //   block [0x82DB5060..0x82DB5094)
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
	pc = 0x82DB5094; continue 'dispatch;
            }
            0x82DB5094 => {
    //   block [0x82DB5094..0x82DB50C0)
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
	pc = 0x82DB50C0; continue 'dispatch;
            }
            0x82DB50C0 => {
    //   block [0x82DB50C0..0x82DB50E4)
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
	pc = 0x82DB50E4; continue 'dispatch;
            }
            0x82DB50E4 => {
    //   block [0x82DB50E4..0x82DB5104)
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
	pc = 0x82DB5104; continue 'dispatch;
            }
            0x82DB5104 => {
    //   block [0x82DB5104..0x82DB5114)
	// 82DB5104: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DB5108: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DB510C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DB5110: 4BFA1E01  bl 0x82d56f10
	ctx.lr = 0x82DB5114;
	sub_82D56F10(ctx, base);
	pc = 0x82DB5114; continue 'dispatch;
            }
            0x82DB5114 => {
    //   block [0x82DB5114..0x82DB5130)
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
	pc = 0x82DB5130; continue 'dispatch;
            }
            0x82DB5130 => {
    //   block [0x82DB5130..0x82DB515C)
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
	pc = 0x82DB515C; continue 'dispatch;
            }
            0x82DB515C => {
    //   block [0x82DB515C..0x82DB51A4)
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
	pc = 0x82DB51A4; continue 'dispatch;
            }
            0x82DB51A4 => {
    //   block [0x82DB51A4..0x82DB51B8)
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
	pc = 0x82DB51B8; continue 'dispatch;
            }
            0x82DB51B8 => {
    //   block [0x82DB51B8..0x82DB51CC)
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
    //   block [0x82DB51D0..0x82DB5200)
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
            }
            0x82DB5200 => {
    //   block [0x82DB5200..0x82DB5210)
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
	pc = 0x82DB5210; continue 'dispatch;
            }
            0x82DB5210 => {
    //   block [0x82DB5210..0x82DB5238)
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
            }
            0x82DB5238 => {
    //   block [0x82DB5238..0x82DB5240)
	// 82DB5238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB523C: 48000044  b 0x82db5280
	pc = 0x82DB5280; continue 'dispatch;
            }
            0x82DB5240 => {
    //   block [0x82DB5240..0x82DB5280)
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
            }
            0x82DB5280 => {
    //   block [0x82DB5280..0x82DB5298)
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
    //   block [0x82DB5298..0x82DB52CC)
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
	pc = 0x82DB52CC; continue 'dispatch;
            }
            0x82DB52CC => {
    //   block [0x82DB52CC..0x82DB5320)
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
            }
            0x82DB5320 => {
    //   block [0x82DB5320..0x82DB5354)
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
	pc = 0x82DB5354; continue 'dispatch;
            }
            0x82DB5354 => {
    //   block [0x82DB5354..0x82DB53B4)
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
            }
            0x82DB53B4 => {
    //   block [0x82DB53B4..0x82DB53C0)
	// 82DB53B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DB53B8: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DB53BC: 4BEF4090  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DB53C0 => {
    //   block [0x82DB53C0..0x82DB53CC)
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
    //   block [0x82DB53D0..0x82DB542C)
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
            }
            0x82DB542C => {
    //   block [0x82DB542C..0x82DB5450)
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
	pc = 0x82DB5450; continue 'dispatch;
            }
            0x82DB5450 => {
    //   block [0x82DB5450..0x82DB5484)
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
	pc = 0x82DB5484; continue 'dispatch;
            }
            0x82DB5484 => {
    //   block [0x82DB5484..0x82DB54D8)
	// 82DB5484: 89650031  lbz r11, 0x31(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(49 as u32) ) } as u64;
	// 82DB5488: 8145002C  lwz r10, 0x2c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DB548C: 7D670774  extsb r7, r11
	ctx.r[7].s64 = ctx.r[11].s8 as i64;
	// 82DB5490: 81050028  lwz r8, 0x28(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DB5494: 7D6A21D6  mullw r11, r10, r4
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * ctx.r[4].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
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
            }
            0x82DB54D8 => {
    //   block [0x82DB54D8..0x82DB54EC)
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
	pc = 0x82DB54EC; continue 'dispatch;
            }
            0x82DB54EC => {
    //   block [0x82DB54EC..0x82DB5548)
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
	pc = 0x82DB5548; continue 'dispatch;
            }
            0x82DB5548 => {
    //   block [0x82DB5548..0x82DB56D8)
	// 82DB5548: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82DB554C: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB5550: C00900F0  lfs f0, 0xf0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(240 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB5554: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DB5558: 8969006C  lbz r11, 0x6c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DB555C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DB5560: 99630016  stb r11, 0x16(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 82DB5564: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DB5568: 396B4C30  addi r11, r11, 0x4c30
	ctx.r[11].s64 = ctx.r[11].s64 + 19504;
	pc = 0x82DB56D8; continue 'dispatch;
            }
            0x82DB56D8 => {
    //   block [0x82DB56D8..0x82DB57FC)
	// 82DB56D8: 55671838  slwi r7, r11, 3
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DB56DC: 81490058  lwz r10, 0x58(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DB56E0: 55661838  slwi r6, r11, 3
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DB56E4: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82DB56E8: 7CCB3214  add r6, r11, r6
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82DB56EC: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DB56F0: 392100DF  addi r9, r1, 0xdf
	ctx.r[9].s64 = ctx.r[1].s64 + 223;
	// 82DB56F4: 7D675214  add r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82DB56F8: 54C72036  slwi r7, r6, 4
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DB56FC: 55290030  rlwinm r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82DB5700: 7CE75850  subf r7, r7, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82DB5704: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82DB5708: 7D4A3850  subf r10, r10, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82DB570C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DB5710: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DB5714: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB5718: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DB571C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DB5720: 90EAFFF8  stw r7, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 82DB5724: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB5728: 90EAFFFC  stw r7, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 82DB572C: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB5730: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DB5734: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB5738: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DB573C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DB5740: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DB5744: 4199FFD0  bgt cr6, 0x82db5714
	if ctx.cr[6].gt {
	pc = 0x82DB5714; continue 'dispatch;
	}
	// 82DB5748: 81690018  lwz r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB574C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DB5750: 89090020  lbz r8, 0x20(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB5754: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DB5758: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DB575C: 409A0010  bne cr6, 0x82db576c
	if !ctx.cr[6].eq {
	pc = 0x82DB576C; continue 'dispatch;
	}
	// 82DB5760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB5764: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82DB5768: 4BEF3CEC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82DB576C: 89690021  lbz r11, 0x21(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DB5770: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB5774: 409A0020  bne cr6, 0x82db5794
	if !ctx.cr[6].eq {
	pc = 0x82DB5794; continue 'dispatch;
	}
	// 82DB5778: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB577C: 419A0034  beq cr6, 0x82db57b0
	if ctx.cr[6].eq {
	pc = 0x82DB57B0; continue 'dispatch;
	}
	// 82DB5780: 38A90060  addi r5, r9, 0x60
	ctx.r[5].s64 = ctx.r[9].s64 + 96;
	// 82DB5784: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB5788: 480094F9  bl 0x82dbec80
	ctx.lr = 0x82DB578C;
	sub_82DBEC80(ctx, base);
	// 82DB578C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB5790: 48000024  b 0x82db57b4
	pc = 0x82DB57B4; continue 'dispatch;
	// 82DB5794: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB5798: 419A0018  beq cr6, 0x82db57b0
	if ctx.cr[6].eq {
	pc = 0x82DB57B0; continue 'dispatch;
	}
	// 82DB579C: 38A90030  addi r5, r9, 0x30
	ctx.r[5].s64 = ctx.r[9].s64 + 48;
	// 82DB57A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB57A4: 48006335  bl 0x82dbbad8
	ctx.lr = 0x82DB57A8;
	sub_82DBBAD8(ctx, base);
	// 82DB57A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DB57AC: 48000008  b 0x82db57b4
	pc = 0x82DB57B4; continue 'dispatch;
	// 82DB57B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DB57B4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB57B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB57BC: 419A0034  beq cr6, 0x82db57f0
	if ctx.cr[6].eq {
	pc = 0x82DB57F0; continue 'dispatch;
	}
	// 82DB57C0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB57C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB57C8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DB57CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB57D0: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB57D4: 409A001C  bne cr6, 0x82db57f0
	if !ctx.cr[6].eq {
	pc = 0x82DB57F0; continue 'dispatch;
	}
	// 82DB57D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB57DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB57E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB57E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB57E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB57EC: 4E800421  bctrl
	ctx.lr = 0x82DB57F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB57F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB57F4: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82DB57F8: 4BEF3C5C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DB5818..0x82DB584C)
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
            }
            0x82DB584C => {
    //   block [0x82DB584C..0x82DB58B0)
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
            }
            0x82DB58B0 => {
    //   block [0x82DB58B0..0x82DB58D4)
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
    //   block [0x82DB5BE8..0x82DB5BF8)
	// 82DB5BE8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DB5BEC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DB5BF0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DB5BF4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DB5BF8; continue 'dispatch;
            }
            0x82DB5BF8 => {
    //   block [0x82DB5BF8..0x82DB5C8C)
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
    //   block [0x82DB5C90..0x82DB5CA0)
	// 82DB5C90: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DB5C94: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DB5C98: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DB5C9C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DB5CA0; continue 'dispatch;
            }
            0x82DB5CA0 => {
    //   block [0x82DB5CA0..0x82DB5D4C)
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
    //   block [0x82DB5D58..0x82DB5DA0)
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
	pc = 0x82DB5DA0; continue 'dispatch;
            }
            0x82DB5DA0 => {
    //   block [0x82DB5DA0..0x82DB5DBC)
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
    // ---- function 0x82DB5E38 size=44
    let mut pc: u32 = 0x82DB5E38;
    'dispatch: loop {
        match pc {
            0x82DB5E38 => {
    //   block [0x82DB5E38..0x82DB5E64)
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
    //   block [0x82DB5F18..0x82DB5F60)
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
	pc = 0x82DB5F60; continue 'dispatch;
            }
            0x82DB5F60 => {
    //   block [0x82DB5F60..0x82DB5F9C)
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
	pc = 0x82DB5F9C; continue 'dispatch;
            }
            0x82DB5F9C => {
    //   block [0x82DB5F9C..0x82DB5FA8)
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


pub fn sub_82DB6078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6078 size=72
    let mut pc: u32 = 0x82DB6078;
    'dispatch: loop {
        match pc {
            0x82DB6078 => {
    //   block [0x82DB6078..0x82DB60AC)
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
	pc = 0x82DB60AC; continue 'dispatch;
            }
            0x82DB60AC => {
    //   block [0x82DB60AC..0x82DB60C0)
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
    //   block [0x82DB60C0..0x82DB6108)
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
	pc = 0x82DB6108; continue 'dispatch;
            }
            0x82DB6108 => {
    //   block [0x82DB6108..0x82DB6118)
	// 82DB6108: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DB610C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DB6110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB6114: 4BFA0DFD  bl 0x82d56f10
	ctx.lr = 0x82DB6118;
	sub_82D56F10(ctx, base);
	pc = 0x82DB6118; continue 'dispatch;
            }
            0x82DB6118 => {
    //   block [0x82DB6118..0x82DB6148)
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
	pc = 0x82DB6148; continue 'dispatch;
            }
            0x82DB6148 => {
    //   block [0x82DB6148..0x82DB616C)
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
	pc = 0x82DB616C; continue 'dispatch;
            }
            0x82DB616C => {
    //   block [0x82DB616C..0x82DB618C)
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
	pc = 0x82DB618C; continue 'dispatch;
            }
            0x82DB618C => {
    //   block [0x82DB618C..0x82DB61B0)
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
	pc = 0x82DB61B0; continue 'dispatch;
            }
            0x82DB61B0 => {
    //   block [0x82DB61B0..0x82DB61BC)
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
    //   block [0x82DB61C0..0x82DB6200)
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
	pc = 0x82DB6200; continue 'dispatch;
            }
            0x82DB6200 => {
    //   block [0x82DB6200..0x82DB622C)
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
	pc = 0x82DB622C; continue 'dispatch;
            }
            0x82DB622C => {
    //   block [0x82DB622C..0x82DB6240)
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
    //   block [0x82DB6240..0x82DB62A8)
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
	pc = 0x82DB62A8; continue 'dispatch;
            }
            0x82DB62A8 => {
    //   block [0x82DB62A8..0x82DB62AC)
	// 82DB62A8: D3C10070  stfs f30, 0x70(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	pc = 0x82DB62AC; continue 'dispatch;
            }
            0x82DB62AC => {
    //   block [0x82DB62AC..0x82DB62E0)
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
	pc = 0x82DB62E0; continue 'dispatch;
            }
            0x82DB62E0 => {
    //   block [0x82DB62E0..0x82DB657C)
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
    //   block [0x82DB6580..0x82DB65DC)
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
            }
            0x82DB65DC => {
    //   block [0x82DB65DC..0x82DB6610)
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
    //   block [0x82DB66C8..0x82DB6730)
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
            }
            0x82DB6730 => {
    //   block [0x82DB6730..0x82DB673C)
	// 82DB6730: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82DB6734: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82DB6738: 48000008  b 0x82db6740
	pc = 0x82DB6740; continue 'dispatch;
            }
            0x82DB673C => {
    //   block [0x82DB673C..0x82DB6740)
	// 82DB673C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x82DB6740; continue 'dispatch;
            }
            0x82DB6740 => {
    //   block [0x82DB6740..0x82DB6758)
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
    //   block [0x82DB6878..0x82DB68B4)
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
	pc = 0x82DB68B4; continue 'dispatch;
            }
            0x82DB68B4 => {
    //   block [0x82DB68B4..0x82DB68D8)
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
    // ---- function 0x82DB68D8 size=152
    let mut pc: u32 = 0x82DB68D8;
    'dispatch: loop {
        match pc {
            0x82DB68D8 => {
    //   block [0x82DB68D8..0x82DB6918)
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
	pc = 0x82DB6918; continue 'dispatch;
            }
            0x82DB6918 => {
    //   block [0x82DB6918..0x82DB6970)
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


pub fn sub_82DB6970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB6970 size=284
    let mut pc: u32 = 0x82DB6970;
    'dispatch: loop {
        match pc {
            0x82DB6970 => {
    //   block [0x82DB6970..0x82DB69C0)
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
	pc = 0x82DB69C0; continue 'dispatch;
            }
            0x82DB69C0 => {
    //   block [0x82DB69C0..0x82DB6A8C)
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
    //   block [0x82DB6A90..0x82DB6AE0)
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
	pc = 0x82DB6AE0; continue 'dispatch;
            }
            0x82DB6AE0 => {
    //   block [0x82DB6AE0..0x82DB6B14)
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
	pc = 0x82DB6B14; continue 'dispatch;
            }
            0x82DB6B14 => {
    //   block [0x82DB6B14..0x82DB6B5C)
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
	pc = 0x82DB6B5C; continue 'dispatch;
            }
            0x82DB6B5C => {
    //   block [0x82DB6B5C..0x82DB6B74)
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
    //   block [0x82DB6EE0..0x82DB6EEC)
	// 82DB6EE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB6EE4: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82DB6EE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	pc = 0x82DB6EEC; continue 'dispatch;
            }
            0x82DB6EEC => {
    //   block [0x82DB6EEC..0x82DB6F08)
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
	pc = 0x82DB6F08; continue 'dispatch;
            }
            0x82DB6F08 => {
    //   block [0x82DB6F08..0x82DB6F20)
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
	pc = 0x82DB6F20; continue 'dispatch;
            }
            0x82DB6F20 => {
    //   block [0x82DB6F20..0x82DB6F34)
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
    //   block [0x82DB6FA8..0x82DB6FB4)
	// 82DB6FA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DB6FAC: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82DB6FB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	pc = 0x82DB6FB4; continue 'dispatch;
            }
            0x82DB6FB4 => {
    //   block [0x82DB6FB4..0x82DB6FD0)
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
	pc = 0x82DB6FD0; continue 'dispatch;
            }
            0x82DB6FD0 => {
    //   block [0x82DB6FD0..0x82DB6FE8)
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
	pc = 0x82DB6FE8; continue 'dispatch;
            }
            0x82DB6FE8 => {
    //   block [0x82DB6FE8..0x82DB6FFC)
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
    //   block [0x82DB7000..0x82DB7090)
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
	pc = 0x82DB7090; continue 'dispatch;
            }
            0x82DB7090 => {
    //   block [0x82DB7090..0x82DB70AC)
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
    //   block [0x82DB7110..0x82DB7160)
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
            }
            0x82DB7160 => {
    //   block [0x82DB7160..0x82DB7170)
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
	pc = 0x82DB7170; continue 'dispatch;
            }
            0x82DB7170 => {
    //   block [0x82DB7170..0x82DB7178)
	// 82DB7170: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB7174: 48000068  b 0x82db71dc
	pc = 0x82DB71DC; continue 'dispatch;
            }
            0x82DB7178 => {
    //   block [0x82DB7178..0x82DB7184)
	// 82DB7178: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB717C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DB7180: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	pc = 0x82DB7184; continue 'dispatch;
            }
            0x82DB7184 => {
    //   block [0x82DB7184..0x82DB71DC)
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
	pc = 0x82DB71DC; continue 'dispatch;
            }
            0x82DB71DC => {
    //   block [0x82DB71DC..0x82DB71F8)
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
            }
            0x82DB71F8 => {
    //   block [0x82DB71F8..0x82DB7218)
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
            }
            0x82DB7218 => {
    //   block [0x82DB7218..0x82DB7234)
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
            }
            0x82DB7234 => {
    //   block [0x82DB7234..0x82DB7240)
	// 82DB7234: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB7238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB723C: 419A0014  beq cr6, 0x82db7250
	if ctx.cr[6].eq {
	pc = 0x82DB7250; continue 'dispatch;
	}
	pc = 0x82DB7240; continue 'dispatch;
            }
            0x82DB7240 => {
    //   block [0x82DB7240..0x82DB7250)
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
	pc = 0x82DB7250; continue 'dispatch;
            }
            0x82DB7250 => {
    //   block [0x82DB7250..0x82DB7258)
	// 82DB7250: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB7254: 4BFFFF88  b 0x82db71dc
	pc = 0x82DB71DC; continue 'dispatch;
            }
            0x82DB7258 => {
    //   block [0x82DB7258..0x82DB726C)
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
    //   block [0x82DB7278..0x82DB72AC)
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
            }
            0x82DB72AC => {
    //   block [0x82DB72AC..0x82DB72D0)
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
            }
            0x82DB72D0 => {
    //   block [0x82DB72D0..0x82DB72EC)
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
    //   block [0x82DB76B8..0x82DB7728)
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
            }
            0x82DB7728 => {
    //   block [0x82DB7728..0x82DB7738)
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
	pc = 0x82DB7738; continue 'dispatch;
            }
            0x82DB7738 => {
    //   block [0x82DB7738..0x82DB7774)
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
            }
            0x82DB7774 => {
    //   block [0x82DB7774..0x82DB7784)
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
	pc = 0x82DB7784; continue 'dispatch;
            }
            0x82DB7784 => {
    //   block [0x82DB7784..0x82DB77B4)
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
	pc = 0x82DB77B4; continue 'dispatch;
            }
            0x82DB77B4 => {
    //   block [0x82DB77B4..0x82DB78BC)
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
    //   block [0x82DB8A10..0x82DB8A30)
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
	pc = 0x82DB8A30; continue 'dispatch;
            }
            0x82DB8A30 => {
    //   block [0x82DB8A30..0x82DB8A60)
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
	pc = 0x82DB8A60; continue 'dispatch;
            }
            0x82DB8A60 => {
    //   block [0x82DB8A60..0x82DB8AA4)
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
    // ---- function 0x82DB8AA8 size=92
    let mut pc: u32 = 0x82DB8AA8;
    'dispatch: loop {
        match pc {
            0x82DB8AA8 => {
    //   block [0x82DB8AA8..0x82DB8B04)
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
	// 82DB8AB8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB8ABC: D0030004  stfs f0, 4(r3)
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
    //   block [0x82DB8B78..0x82DB8BE0)
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
	pc = 0x82DB8BE0; continue 'dispatch;
            }
            0x82DB8BE0 => {
    //   block [0x82DB8BE0..0x82DB8BFC)
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


pub fn sub_82DB8C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB8C00 size=148
    let mut pc: u32 = 0x82DB8C00;
    'dispatch: loop {
        match pc {
            0x82DB8C00 => {
    //   block [0x82DB8C00..0x82DB8C68)
	// 82DB8C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB8C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB8C0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB8C14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8C18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB8C1C: 396B09D8  addi r11, r11, 0x9d8
	ctx.r[11].s64 = ctx.r[11].s64 + 2520;
	// 82DB8C20: 394A09B4  addi r10, r10, 0x9b4
	ctx.r[10].s64 = ctx.r[10].s64 + 2484;
	// 82DB8C24: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB8C28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB8C2C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB8C30: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB8C34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB8C38: 419A0030  beq cr6, 0x82db8c68
	if ctx.cr[6].eq {
	pc = 0x82DB8C68; continue 'dispatch;
	}
	// 82DB8C3C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB8C40: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB8C44: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DB8C48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB8C4C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB8C50: 409A0018  bne cr6, 0x82db8c68
	if !ctx.cr[6].eq {
	pc = 0x82DB8C68; continue 'dispatch;
	}
	// 82DB8C54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8C58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB8C5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8C60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8C64: 4E800421  bctrl
	ctx.lr = 0x82DB8C68;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DB8C68 => {
    //   block [0x82DB8C68..0x82DB8C94)
	// 82DB8C68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8C6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DB8C70: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB8C74: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DB8C78: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB8C7C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB8C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB8C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB8C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB8C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB8C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB8C98 size=32
    let mut pc: u32 = 0x82DB8C98;
    'dispatch: loop {
        match pc {
            0x82DB8C98 => {
    //   block [0x82DB8C98..0x82DB8CB8)
	// 82DB8C98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DB8C9C: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB8CA0: C00B001C  lfs f0, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB8CA4: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DB8CA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8CAC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB8CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8CB4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB8CB8 size=612
    let mut pc: u32 = 0x82DB8CB8;
    'dispatch: loop {
        match pc {
            0x82DB8CB8 => {
    //   block [0x82DB8CB8..0x82DB8D40)
	// 82DB8CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8CBC: 4BEF0741  bl 0x82ca93fc
	ctx.lr = 0x82DB8CC0;
	sub_82CA93D0(ctx, base);
	// 82DB8CC0: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 82DB8CC4: 4BEF5015  bl 0x82cadcd8
	ctx.lr = 0x82DB8CC8;
	sub_82CADCA0(ctx, base);
	// 82DB8CC8: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82DB8CCC: 4824DD09  bl 0x830069d4
	ctx.lr = 0x82DB8CD0;
	sub_83006760(ctx, base);
	// 82DB8CD0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8CD4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DB8CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB8CDC: 573DFC7E  rlwinm r29, r25, 0x1f, 0x11, 0x1f
	ctx.r[29].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	// 82DB8CE0: 573C843E  srwi r28, r25, 0x10
	ctx.r[28].u32 = ctx.r[25].u32.wrapping_shr(16);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DB8CE4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DB8CE8: 419A0058  beq cr6, 0x82db8d40
	if ctx.cr[6].eq {
	pc = 0x82DB8D40; continue 'dispatch;
	}
	// 82DB8CEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB8CF0: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB8CF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DB8CF8: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DB8CFC: 392A4CA4  addi r9, r10, 0x4ca4
	ctx.r[9].s64 = ctx.r[10].s64 + 19620;
	// 82DB8D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB8D04: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82DB8D08: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82DB8D0C: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DB8D10: B1050006  sth r8, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DB8D14: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB8D18: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DB8D1C: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DB8D20: 90E5000C  stw r7, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB8D24: B1450014  sth r10, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82DB8D28: 98C50016  stb r6, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[6].u8 ) };
	pc = 0x82DB8D40; continue 'dispatch;
            }
            0x82DB8D40 => {
    //   block [0x82DB8D40..0x82DB8F1C)
	// 82DB8D40: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DB8D44: 7FAB07B4  extsw r11, r29
	ctx.r[11].s64 = ctx.r[29].s32 as i64;
	// 82DB8D48: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB8D4C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DB8D50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB8D54: 3BC30020  addi r30, r3, 0x20
	ctx.r[30].s64 = ctx.r[3].s64 + 32;
	// 82DB8D58: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DB8D5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8D60: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DB8D64: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DB8D68: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DB8D6C: FFC00018  frsp f30, f0
	ctx.f[30].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DB8D70: D3C10060  stfs f30, 0x60(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DB8D74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8D78: 4E800421  bctrl
	ctx.lr = 0x82DB8D7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB8D7C: 7F8A07B4  extsw r10, r28
	ctx.r[10].s64 = ctx.r[28].s32 as i64;
	// 82DB8D80: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB8D84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB8D88: D0210064  stfs f1, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DB8D8C: 3B7C0001  addi r27, r28, 1
	ctx.r[27].s64 = ctx.r[28].s64 + 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8F20 size=20
    let mut pc: u32 = 0x82DB8F20;
    'dispatch: loop {
        match pc {
            0x82DB8F20 => {
    //   block [0x82DB8F20..0x82DB8F34)
	// 82DB8F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB8F24: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DB8F28: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82DB8F2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DB8F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB8F38 size=96
    let mut pc: u32 = 0x82DB8F38;
    'dispatch: loop {
        match pc {
            0x82DB8F38 => {
    //   block [0x82DB8F38..0x82DB8F98)
	// 82DB8F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB8F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB8F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8F48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB8F4C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8F50: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DB8F54: 388B2B50  addi r4, r11, 0x2b50
	ctx.r[4].s64 = ctx.r[11].s64 + 11088;
	// 82DB8F58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB8F5C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB8F64: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB8F68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8F6C: 4E800421  bctrl
	ctx.lr = 0x82DB8F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB8F70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8F74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB8F78: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB8F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8F80: 4E800421  bctrl
	ctx.lr = 0x82DB8F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB8F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB8F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB8F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB8F90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB8F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8F98 size=64
    let mut pc: u32 = 0x82DB8F98;
    'dispatch: loop {
        match pc {
            0x82DB8F98 => {
    //   block [0x82DB8F98..0x82DB8FD8)
	// 82DB8F98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8F9C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82DB8FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB8FA4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB8FA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DB8FAC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DB8FB0: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB8FB4: 394A0218  addi r10, r10, 0x218
	ctx.r[10].s64 = ctx.r[10].s64 + 536;
	// 82DB8FB8: 392901F4  addi r9, r9, 0x1f4
	ctx.r[9].s64 = ctx.r[9].s64 + 500;
	// 82DB8FBC: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DB8FC0: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DB8FC4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB8FC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB8FCC: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DB8FD0: 99030014  stb r8, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82DB8FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB8FD8 size=560
    let mut pc: u32 = 0x82DB8FD8;
    'dispatch: loop {
        match pc {
            0x82DB8FD8 => {
    //   block [0x82DB8FD8..0x82DB902C)
	// 82DB8FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8FDC: 4BEF0415  bl 0x82ca93f0
	ctx.lr = 0x82DB8FE0;
	sub_82CA93D0(ctx, base);
	// 82DB8FE0: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8FE4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8FE8: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DB8FEC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DB8FF0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DB8FF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DB8FF8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DB8FFC: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB9000: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9004: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9008: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB900C: 40980020  bge cr6, 0x82db902c
	if !ctx.cr[6].lt {
	pc = 0x82DB902C; continue 'dispatch;
	}
	// 82DB9010: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB9014: 39292B5C  addi r9, r9, 0x2b5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11100;
	// 82DB9018: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB901C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB9020: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DB9024: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB9028: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DB902C; continue 'dispatch;
            }
            0x82DB902C => {
    //   block [0x82DB902C..0x82DB9070)
	// 82DB902C: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB9030: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 82DB9034: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 82DB9038: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB903C: 7EFABB78  mr r26, r23
	ctx.r[26].u64 = ctx.r[23].u64;
	// 82DB9040: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9044: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DB9048: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB904C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB9050: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9054: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB9058: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB905C: 409A0088  bne cr6, 0x82db90e4
	if !ctx.cr[6].eq {
	pc = 0x82DB90E4; continue 'dispatch;
	}
	// 82DB9060: 4E800421  bctrl
	ctx.lr = 0x82DB9064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9068: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB906C: 419A012C  beq cr6, 0x82db9198
	if ctx.cr[6].eq {
	pc = 0x82DB9198; continue 'dispatch;
	}
            }
            0x82DB9070 => {
    //   block [0x82DB9070..0x82DB90BC)
	// 82DB9070: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9074: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB9078: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB907C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9080: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9088: 4E800421  bctrl
	ctx.lr = 0x82DB908C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB908C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DB9090: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DB9094: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB9098: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82DB909C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB90A0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB90A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB90A8: 4E800421  bctrl
	ctx.lr = 0x82DB90AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB90AC: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82DB90B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB90B4: 419A0008  beq cr6, 0x82db90bc
	if ctx.cr[6].eq {
	pc = 0x82DB90BC; continue 'dispatch;
	}
	// 82DB90B8: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
            }
            0x82DB90BC => {
    //   block [0x82DB90BC..0x82DB90E4)
	// 82DB90BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB90C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB90C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB90C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB90CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB90D0: 4E800421  bctrl
	ctx.lr = 0x82DB90D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB90D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB90D8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB90DC: 409AFF94  bne cr6, 0x82db9070
	if !ctx.cr[6].eq {
	pc = 0x82DB9070; continue 'dispatch;
	}
	// 82DB90E0: 480000B8  b 0x82db9198
	pc = 0x82DB9198; continue 'dispatch;
            }
            0x82DB90E4 => {
    //   block [0x82DB90E4..0x82DB90F4)
	// 82DB90E4: 4E800421  bctrl
	ctx.lr = 0x82DB90E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB90E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB90EC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB90F0: 419A00A8  beq cr6, 0x82db9198
	if ctx.cr[6].eq {
	pc = 0x82DB9198; continue 'dispatch;
	}
            }
            0x82DB90F4 => {
    //   block [0x82DB90F4..0x82DB9174)
	// 82DB90F4: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB90F8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DB90FC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82DB9100: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DB9104: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB9108: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82DB910C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9110: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9118: 4E800421  bctrl
	ctx.lr = 0x82DB911C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB911C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB9124: 419A0050  beq cr6, 0x82db9174
	if ctx.cr[6].eq {
	pc = 0x82DB9174; continue 'dispatch;
	}
	// 82DB9128: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB912C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB9130: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9134: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9138: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB913C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9140: 4E800421  bctrl
	ctx.lr = 0x82DB9144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9144: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DB9148: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DB914C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB9150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB9154: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9158: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB915C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9160: 4E800421  bctrl
	ctx.lr = 0x82DB9164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9164: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB9168: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB916C: 419A0008  beq cr6, 0x82db9174
	if ctx.cr[6].eq {
	pc = 0x82DB9174; continue 'dispatch;
	}
	// 82DB9170: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
            }
            0x82DB9174 => {
    //   block [0x82DB9174..0x82DB9198)
	// 82DB9174: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9178: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB917C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9180: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9188: 4E800421  bctrl
	ctx.lr = 0x82DB918C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB918C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9190: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9194: 409AFF60  bne cr6, 0x82db90f4
	if !ctx.cr[6].eq {
	pc = 0x82DB90F4; continue 'dispatch;
	}
            }
            0x82DB9198 => {
    //   block [0x82DB9198..0x82DB91B8)
	// 82DB9198: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB919C: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 82DB91A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB91A4: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DB91A8: 419A0010  beq cr6, 0x82db91b8
	if ctx.cr[6].eq {
	pc = 0x82DB91B8; continue 'dispatch;
	}
	// 82DB91AC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DB91B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB91B4: 7F4BE12E  stwx r26, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u32) };
	pc = 0x82DB91B8; continue 'dispatch;
            }
            0x82DB91B8 => {
    //   block [0x82DB91B8..0x82DB91E8)
	// 82DB91B8: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB91BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB91C0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB91C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB91C8: 40980020  bge cr6, 0x82db91e8
	if !ctx.cr[6].lt {
	pc = 0x82DB91E8; continue 'dispatch;
	}
	// 82DB91CC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB91D0: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB91D4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB91D8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB91DC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB91E0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB91E4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DB91E8; continue 'dispatch;
            }
            0x82DB91E8 => {
    //   block [0x82DB91E8..0x82DB9208)
	// 82DB91E8: 7D7AB850  subf r11, r26, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[26].s64;
	// 82DB91EC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DB91F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DB91F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DB91F8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DB91FC: 99760000  stb r11, 0(r22)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DB9200: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DB9204: 4BEF023C  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB9208 size=368
    let mut pc: u32 = 0x82DB9208;
    'dispatch: loop {
        match pc {
            0x82DB9208 => {
    //   block [0x82DB9208..0x82DB9260)
	// 82DB9208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB920C: 4BEF01F1  bl 0x82ca93fc
	ctx.lr = 0x82DB9210;
	sub_82CA93D0(ctx, base);
	// 82DB9210: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82DB9214: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB9218: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB921C: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DB9220: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DB9224: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DB9228: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DB922C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DB9230: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DB9234: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9238: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB923C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DB9240: 40980020  bge cr6, 0x82db9260
	if !ctx.cr[6].lt {
	pc = 0x82DB9260; continue 'dispatch;
	}
	// 82DB9244: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DB9248: 39082B6C  addi r8, r8, 0x2b6c
	ctx.r[8].s64 = ctx.r[8].s64 + 11116;
	// 82DB924C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DB9250: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DB9254: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82DB9258: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB925C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82DB9260; continue 'dispatch;
            }
            0x82DB9260 => {
    //   block [0x82DB9260..0x82DB9378)
	// 82DB9260: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB9264: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 82DB9268: 3BC90010  addi r30, r9, 0x10
	ctx.r[30].s64 = ctx.r[9].s64 + 16;
	// 82DB926C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9270: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB9274: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB9278: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB9378 size=276
    let mut pc: u32 = 0x82DB9378;
    'dispatch: loop {
        match pc {
            0x82DB9378 => {
    //   block [0x82DB9378..0x82DB93C4)
	// 82DB9378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB937C: 4BEF0089  bl 0x82ca9404
	ctx.lr = 0x82DB9380;
	sub_82CA93D0(ctx, base);
	// 82DB9380: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DB9384: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB9388: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB938C: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DB9390: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DB9394: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DB9398: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB939C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB93A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB93A4: 40980020  bge cr6, 0x82db93c4
	if !ctx.cr[6].lt {
	pc = 0x82DB93C4; continue 'dispatch;
	}
	// 82DB93A8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB93AC: 39292B8C  addi r9, r9, 0x2b8c
	ctx.r[9].s64 = ctx.r[9].s64 + 11148;
	// 82DB93B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB93B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB93B8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB93BC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB93C0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DB93C4; continue 'dispatch;
            }
            0x82DB93C4 => {
    //   block [0x82DB93C4..0x82DB93F0)
	// 82DB93C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB93C8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB93CC: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 82DB93D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB93D4: C3EB0BE4  lfs f31, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DB93D8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB93DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB93E0: 4E800421  bctrl
	ctx.lr = 0x82DB93E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB93E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB93E8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB93EC: 419A0060  beq cr6, 0x82db944c
	if ctx.cr[6].eq {
	pc = 0x82DB944C; continue 'dispatch;
	}
            }
            0x82DB93F0 => {
    //   block [0x82DB93F0..0x82DB944C)
	// 82DB93F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB93F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DB93F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB93FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9400: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9408: 4E800421  bctrl
	ctx.lr = 0x82DB940C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB940C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9410: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB9414: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB941C: 4E800421  bctrl
	ctx.lr = 0x82DB9420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9420: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DB9424: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9428: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB942C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9430: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9434: FFE00FEE  fsel f31, f0, f31, f1
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[1].f64 };
	// 82DB9438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB943C: 4E800421  bctrl
	ctx.lr = 0x82DB9440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9444: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9448: 409AFFA8  bne cr6, 0x82db93f0
	if !ctx.cr[6].eq {
	pc = 0x82DB93F0; continue 'dispatch;
	}
            }
            0x82DB944C => {
    //   block [0x82DB944C..0x82DB947C)
	// 82DB944C: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DB9450: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9454: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9458: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB945C: 40980020  bge cr6, 0x82db947c
	if !ctx.cr[6].lt {
	pc = 0x82DB947C; continue 'dispatch;
	}
	// 82DB9460: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB9464: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB9468: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB946C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB9470: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB9474: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB9478: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DB947C; continue 'dispatch;
            }
            0x82DB947C => {
    //   block [0x82DB947C..0x82DB948C)
	// 82DB947C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DB9480: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82DB9484: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DB9488: 4BEEFFCC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB9490 size=484
    let mut pc: u32 = 0x82DB9490;
    'dispatch: loop {
        match pc {
            0x82DB9490 => {
    //   block [0x82DB9490..0x82DB94E4)
	// 82DB9490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB9494: 4BEEFF65  bl 0x82ca93f8
	ctx.lr = 0x82DB9498;
	sub_82CA93D0(ctx, base);
	// 82DB9498: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB949C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB94A0: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DB94A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DB94A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DB94AC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DB94B0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DB94B4: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB94B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB94BC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB94C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB94C4: 40980020  bge cr6, 0x82db94e4
	if !ctx.cr[6].lt {
	pc = 0x82DB94E4; continue 'dispatch;
	}
	// 82DB94C8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB94CC: 39292B5C  addi r9, r9, 0x2b5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11100;
	// 82DB94D0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB94D4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB94D8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DB94DC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB94E0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DB94E4; continue 'dispatch;
            }
            0x82DB94E4 => {
    //   block [0x82DB94E4..0x82DB9514)
	// 82DB94E4: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB94E8: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 82DB94EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB94F0: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB94F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB94F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB94FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9500: 409A0088  bne cr6, 0x82db9588
	if !ctx.cr[6].eq {
	pc = 0x82DB9588; continue 'dispatch;
	}
	// 82DB9504: 4E800421  bctrl
	ctx.lr = 0x82DB9508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB950C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9510: 419A012C  beq cr6, 0x82db963c
	if ctx.cr[6].eq {
	pc = 0x82DB963C; continue 'dispatch;
	}
            }
            0x82DB9514 => {
    //   block [0x82DB9514..0x82DB9588)
	// 82DB9514: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9518: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DB951C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9524: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB952C: 4E800421  bctrl
	ctx.lr = 0x82DB9530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9530: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB9534: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82DB9538: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DB953C: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82DB9540: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB9544: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DB9548: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB954C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DB9550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9554: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9558: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB955C: 4E800421  bctrl
	ctx.lr = 0x82DB9560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9560: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9564: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB956C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9574: 4E800421  bctrl
	ctx.lr = 0x82DB9578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB957C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9580: 409AFF94  bne cr6, 0x82db9514
	if !ctx.cr[6].eq {
	pc = 0x82DB9514; continue 'dispatch;
	}
	// 82DB9584: 480000B8  b 0x82db963c
	pc = 0x82DB963C; continue 'dispatch;
            }
            0x82DB9588 => {
    //   block [0x82DB9588..0x82DB9598)
	// 82DB9588: 4E800421  bctrl
	ctx.lr = 0x82DB958C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB958C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9590: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9594: 419A00A8  beq cr6, 0x82db963c
	if ctx.cr[6].eq {
	pc = 0x82DB963C; continue 'dispatch;
	}
            }
            0x82DB9598 => {
    //   block [0x82DB9598..0x82DB9618)
	// 82DB9598: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB959C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DB95A0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82DB95A4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DB95A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB95AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB95B0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB95BC: 4E800421  bctrl
	ctx.lr = 0x82DB95C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB95C0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB95C8: 419A0050  beq cr6, 0x82db9618
	if ctx.cr[6].eq {
	pc = 0x82DB9618; continue 'dispatch;
	}
	// 82DB95CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95D0: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DB95D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB95D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB95DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB95E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB95E4: 4E800421  bctrl
	ctx.lr = 0x82DB95E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB95E8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB95EC: 9381007C  stw r28, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 82DB95F0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DB95F4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82DB95F8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DB95FC: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82DB9600: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB9604: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82DB9608: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB960C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9614: 4E800421  bctrl
	ctx.lr = 0x82DB9618;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DB9618 => {
    //   block [0x82DB9618..0x82DB963C)
	// 82DB9618: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB961C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9620: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9624: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB962C: 4E800421  bctrl
	ctx.lr = 0x82DB9630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9634: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9638: 409AFF60  bne cr6, 0x82db9598
	if !ctx.cr[6].eq {
	pc = 0x82DB9598; continue 'dispatch;
	}
            }
            0x82DB963C => {
    //   block [0x82DB963C..0x82DB966C)
	// 82DB963C: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB9640: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9644: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9648: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB964C: 40980020  bge cr6, 0x82db966c
	if !ctx.cr[6].lt {
	pc = 0x82DB966C; continue 'dispatch;
	}
	// 82DB9650: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB9654: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB9658: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB965C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB9660: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB9664: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB9668: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DB966C; continue 'dispatch;
            }
            0x82DB966C => {
    //   block [0x82DB966C..0x82DB9674)
	// 82DB966C: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82DB9670: 4BEEFDD8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB9678 size=24
    let mut pc: u32 = 0x82DB9678;
    'dispatch: loop {
        match pc {
            0x82DB9678 => {
    //   block [0x82DB9678..0x82DB9690)
	// 82DB9678: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB967C: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB9680: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9684: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB968C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB9690 size=20
    let mut pc: u32 = 0x82DB9690;
    'dispatch: loop {
        match pc {
            0x82DB9690 => {
    //   block [0x82DB9690..0x82DB96A4)
	// 82DB9690: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9694: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9698: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB969C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB96A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB96A8 size=68
    let mut pc: u32 = 0x82DB96A8;
    'dispatch: loop {
        match pc {
            0x82DB96A8 => {
    //   block [0x82DB96A8..0x82DB96EC)
	// 82DB96A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB96AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB96B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB96B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB96B8: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB96BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB96C0: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB96C4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB96C8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB96CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB96D0: 4E800421  bctrl
	ctx.lr = 0x82DB96D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB96D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB96D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB96DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB96E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB96E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB96E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB96F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB96F0 size=204
    let mut pc: u32 = 0x82DB96F0;
    'dispatch: loop {
        match pc {
            0x82DB96F0 => {
    //   block [0x82DB96F0..0x82DB97BC)
	// 82DB96F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB96F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB96F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB96FC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB97C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB97C0 size=84
    let mut pc: u32 = 0x82DB97C0;
    'dispatch: loop {
        match pc {
            0x82DB97C0 => {
    //   block [0x82DB97C0..0x82DB980C)
	// 82DB97C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB97C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB97C8: 396B27DC  addi r11, r11, 0x27dc
	ctx.r[11].s64 = ctx.r[11].s64 + 10204;
	// 82DB97CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB97D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DB97D4: 38E00015  li r7, 0x15
	ctx.r[7].s64 = 21;
	// 82DB97D8: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DB97DC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB97E0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB97E4: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DB97E8: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB97EC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB97F0: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DB97F4: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB97F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB97FC: 419A0010  beq cr6, 0x82db980c
	if ctx.cr[6].eq {
	pc = 0x82DB980C; continue 'dispatch;
	}
	// 82DB9800: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB9804: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB9808: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82DB980C; continue 'dispatch;
            }
            0x82DB980C => {
    //   block [0x82DB980C..0x82DB9814)
	// 82DB980C: 98A30018  stb r5, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u8 ) };
	// 82DB9810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB9818 size=960
    let mut pc: u32 = 0x82DB9818;
    'dispatch: loop {
        match pc {
            0x82DB9818 => {
    //   block [0x82DB9818..0x82DB9BD8)
	// 82DB9818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB981C: 4BEEFBC1  bl 0x82ca93dc
	ctx.lr = 0x82DB9820;
	sub_82CA93D0(ctx, base);
	// 82DB9820: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82DB9824: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82DB9828: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB982C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB9830: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB9BD8 size=1136
    let mut pc: u32 = 0x82DB9BD8;
    'dispatch: loop {
        match pc {
            0x82DB9BD8 => {
    //   block [0x82DB9BD8..0x82DBA048)
	// 82DB9BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB9BDC: 4BEEF805  bl 0x82ca93e0
	ctx.lr = 0x82DB9BE0;
	sub_82CA93D0(ctx, base);
	// 82DB9BE0: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82DB9BE4: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82DB9BE8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB9BEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB9BF0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBA048 size=96
    let mut pc: u32 = 0x82DBA048;
    'dispatch: loop {
        match pc {
            0x82DBA048 => {
    //   block [0x82DBA048..0x82DBA0A8)
	// 82DBA048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBA04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBA050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBA054: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBA058: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBA05C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBA060: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DBA064: 388B2BD0  addi r4, r11, 0x2bd0
	ctx.r[4].s64 = ctx.r[11].s64 + 11216;
	// 82DBA068: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBA06C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBA074: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBA078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBA07C: 4E800421  bctrl
	ctx.lr = 0x82DBA080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBA080: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBA088: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBA08C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBA090: 4E800421  bctrl
	ctx.lr = 0x82DBA094;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBA094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBA098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBA09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBA0A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBA0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA0A8 size=28
    let mut pc: u32 = 0x82DBA0A8;
    'dispatch: loop {
        match pc {
            0x82DBA0A8 => {
    //   block [0x82DBA0A8..0x82DBA0C4)
	// 82DBA0A8: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBA0AC: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DBA0B0: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82DBA0B4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DBA0B8: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBA0BC: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBA0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA0C8 size=20
    let mut pc: u32 = 0x82DBA0C8;
    'dispatch: loop {
        match pc {
            0x82DBA0C8 => {
    //   block [0x82DBA0C8..0x82DBA0DC)
	// 82DBA0C8: 89630017  lbz r11, 0x17(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(23 as u32) ) } as u64;
	// 82DBA0CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBA0D0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBA0D4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBA0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA0E0 size=16
    let mut pc: u32 = 0x82DBA0E0;
    'dispatch: loop {
        match pc {
            0x82DBA0E0 => {
    //   block [0x82DBA0E0..0x82DBA0F0)
	// 82DBA0E0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBA0F0 size=72
    let mut pc: u32 = 0x82DBA0F0;
    'dispatch: loop {
        match pc {
            0x82DBA0F0 => {
    //   block [0x82DBA0F0..0x82DBA138)
	// 82DBA0F0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBA0F4: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82DBA0F8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82DBA0FC: C00A0CA8  lfs f0, 0xca8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3240 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBA100: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA138 size=108
    let mut pc: u32 = 0x82DBA138;
    'dispatch: loop {
        match pc {
            0x82DBA138 => {
    //   block [0x82DBA138..0x82DBA1A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA1A8 size=224
    let mut pc: u32 = 0x82DBA1A8;
    'dispatch: loop {
        match pc {
            0x82DBA1A8 => {
    //   block [0x82DBA1A8..0x82DBA288)
	// 82DBA1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBA1AC: 4BEEF261  bl 0x82ca940c
	ctx.lr = 0x82DBA1B0;
	sub_82CA93D0(ctx, base);
	// 82DBA1B0: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA288 size=180
    let mut pc: u32 = 0x82DBA288;
    'dispatch: loop {
        match pc {
            0x82DBA288 => {
    //   block [0x82DBA288..0x82DBA33C)
	// 82DBA288: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBA28C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBA290: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82DBA294: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 82DBA298: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82DBA29C: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA340 size=208
    let mut pc: u32 = 0x82DBA340;
    'dispatch: loop {
        match pc {
            0x82DBA340 => {
    //   block [0x82DBA340..0x82DBA410)
	// 82DBA340: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA410 size=124
    let mut pc: u32 = 0x82DBA410;
    'dispatch: loop {
        match pc {
            0x82DBA410 => {
    //   block [0x82DBA410..0x82DBA48C)
	// 82DBA410: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82DBA414: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBA418: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82DBA41C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DBA420: 38E30050  addi r7, r3, 0x50
	ctx.r[7].s64 = ctx.r[3].s64 + 80;
	// 82DBA424: 392AB340  addi r9, r10, -0x4cc0
	ctx.r[9].s64 = ctx.r[10].s64 + -19648;
	// 82DBA428: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DBA42C: 390AB36C  addi r8, r10, -0x4c94
	ctx.r[8].s64 = ctx.r[10].s64 + -19604;
	// 82DBA430: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA434: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBA438: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DBA43C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBA440: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DBA444: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DBA448: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA490 size=1272
    let mut pc: u32 = 0x82DBA490;
    'dispatch: loop {
        match pc {
            0x82DBA490 => {
    //   block [0x82DBA490..0x82DBA988)
	// 82DBA490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBA494: 4BEEEF5D  bl 0x82ca93f0
	ctx.lr = 0x82DBA498;
	sub_82CA93D0(ctx, base);
	// 82DBA498: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82DBA49C: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA988 size=724
    let mut pc: u32 = 0x82DBA988;
    'dispatch: loop {
        match pc {
            0x82DBA988 => {
    //   block [0x82DBA988..0x82DBA9CC)
	// 82DBA988: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DBA98C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DBA990: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA994: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DBA998: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBA99C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA9A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBA9A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBA9A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBA9AC: 40980020  bge cr6, 0x82dba9cc
	if !ctx.cr[6].lt {
	pc = 0x82DBA9CC; continue 'dispatch;
	}
	// 82DBA9B0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBA9B4: 39292BDC  addi r9, r9, 0x2bdc
	ctx.r[9].s64 = ctx.r[9].s64 + 11228;
	// 82DBA9B8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBA9BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBA9C0: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DBA9C4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBA9C8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82DBA9CC; continue 'dispatch;
            }
            0x82DBA9CC => {
    //   block [0x82DBA9CC..0x82DBAC5C)
	// 82DBA9CC: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 82DBA9D0: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82DBA9D4: 39240040  addi r9, r4, 0x40
	ctx.r[9].s64 = ctx.r[4].s64 + 64;
	// 82DBA9D8: 38E1FFD0  addi r7, r1, -0x30
	ctx.r[7].s64 = ctx.r[1].s64 + -48;
	// 82DBA9DC: 3881FFD4  addi r4, r1, -0x2c
	ctx.r[4].s64 = ctx.r[1].s64 + -44;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAC60 size=112
    let mut pc: u32 = 0x82DBAC60;
    'dispatch: loop {
        match pc {
            0x82DBAC60 => {
    //   block [0x82DBAC60..0x82DBACD0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBACD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBACD0 size=232
    let mut pc: u32 = 0x82DBACD0;
    'dispatch: loop {
        match pc {
            0x82DBACD0 => {
    //   block [0x82DBACD0..0x82DBACF0)
	// 82DBACD0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBACD4: C1AB0A64  lfs f13, 0xa64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2660 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBACD8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBACDC: 40990014  ble cr6, 0x82dbacf0
	if !ctx.cr[6].gt {
	pc = 0x82DBACF0; continue 'dispatch;
	}
	// 82DBACE0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBACE4: C1AB0AD4  lfs f13, 0xad4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2772 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBACE8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBACEC: 4198FFD8  blt cr6, 0x82dbacc4
	if ctx.cr[6].lt {
		sub_82DBAC60(ctx, base);
		return;
	}
	pc = 0x82DBACF0; continue 'dispatch;
            }
            0x82DBACF0 => {
    //   block [0x82DBACF0..0x82DBADB8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBADB8 size=224
    let mut pc: u32 = 0x82DBADB8;
    'dispatch: loop {
        match pc {
            0x82DBADB8 => {
    //   block [0x82DBADB8..0x82DBAE98)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAEA0 size=8
    let mut pc: u32 = 0x82DBAEA0;
    'dispatch: loop {
        match pc {
            0x82DBAEA0 => {
    //   block [0x82DBAEA0..0x82DBAEA8)
	// 82DBAEA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBAEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAEA8 size=8
    let mut pc: u32 = 0x82DBAEA8;
    'dispatch: loop {
        match pc {
            0x82DBAEA8 => {
    //   block [0x82DBAEA8..0x82DBAEB0)
	// 82DBAEA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBAEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAEB0 size=20
    let mut pc: u32 = 0x82DBAEB0;
    'dispatch: loop {
        match pc {
            0x82DBAEB0 => {
    //   block [0x82DBAEB0..0x82DBAEC4)
	// 82DBAEB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DBAEB4: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DBAEB8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82DBAEBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBAEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBAEC8 size=144
    let mut pc: u32 = 0x82DBAEC8;
    'dispatch: loop {
        match pc {
            0x82DBAEC8 => {
    //   block [0x82DBAEC8..0x82DBAEEC)
	// 82DBAEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBAECC: 4BEEE539  bl 0x82ca9404
	ctx.lr = 0x82DBAED0;
	sub_82CA93D0(ctx, base);
	// 82DBAED0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBAED4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DBAED8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DBAEDC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBAEE0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DBAEE4: 4099006C  ble cr6, 0x82dbaf50
	if !ctx.cr[6].gt {
	pc = 0x82DBAF50; continue 'dispatch;
	}
	// 82DBAEE8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	pc = 0x82DBAEEC; continue 'dispatch;
            }
            0x82DBAEEC => {
    //   block [0x82DBAEEC..0x82DBAF50)
	// 82DBAEEC: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAEF0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DBAEF4: 815B0044  lwz r10, 0x44(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBAEF8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBAEFC: 557CC23E  srwi r28, r11, 8
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DBAF00: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DBAF04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DBAF08: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82DBAF0C: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBAF10: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBAF14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAF18: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBAF1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBAF20: 4E800421  bctrl
	ctx.lr = 0x82DBAF24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBAF24: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBAF28: 578A402E  slwi r10, r28, 8
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBAF2C: 556B0202  rlwinm r11, r11, 0, 8, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBAF30: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DBAF34: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBAF38: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82DBAF3C: 656B3F00  oris r11, r11, 0x3f00
	ctx.r[11].u64 = ctx.r[11].u64 | 1056964608;
	// 82DBAF40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DBAF44: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBAF48: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82DBAF4C: 409AFFA0  bne cr6, 0x82dbaeec
	if !ctx.cr[6].eq {
	pc = 0x82DBAEEC; continue 'dispatch;
	}
            }
            0x82DBAF50 => {
    //   block [0x82DBAF50..0x82DBAF58)
	// 82DBAF50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DBAF54: 4BEEE500  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAF58 size=24
    let mut pc: u32 = 0x82DBAF58;
    'dispatch: loop {
        match pc {
            0x82DBAF58 => {
    //   block [0x82DBAF58..0x82DBAF70)
	// 82DBAF58: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBAF5C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAF60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAF64: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBAF68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBAF6C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBAF70 size=100
    let mut pc: u32 = 0x82DBAF70;
    'dispatch: loop {
        match pc {
            0x82DBAF70 => {
    //   block [0x82DBAF70..0x82DBAF98)
	// 82DBAF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBAF74: 4BEEE495  bl 0x82ca9408
	ctx.lr = 0x82DBAF78;
	sub_82CA93D0(ctx, base);
	// 82DBAF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBAF7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBAF80: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DBAF84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBAF88: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBAF8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBAF90: 40990038  ble cr6, 0x82dbafc8
	if !ctx.cr[6].gt {
	pc = 0x82DBAFC8; continue 'dispatch;
	}
	// 82DBAF94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DBAF98; continue 'dispatch;
            }
            0x82DBAF98 => {
    //   block [0x82DBAF98..0x82DBAFC8)
	// 82DBAF98: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBAF9C: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DBAFA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAFA4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBAFA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBAFAC: 4E800421  bctrl
	ctx.lr = 0x82DBAFB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBAFB0: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBAFB4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBAFB8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DBAFBC: 7F83E214  add r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 82DBAFC0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBAFC4: 4198FFD4  blt cr6, 0x82dbaf98
	if ctx.cr[6].lt {
	pc = 0x82DBAF98; continue 'dispatch;
	}
            }
            0x82DBAFC8 => {
    //   block [0x82DBAFC8..0x82DBAFD4)
	// 82DBAFC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DBAFCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBAFD0: 4BEEE488  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBAFD8 size=136
    let mut pc: u32 = 0x82DBAFD8;
    'dispatch: loop {
        match pc {
            0x82DBAFD8 => {
    //   block [0x82DBAFD8..0x82DBB004)
	// 82DBAFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBAFDC: 4BEEE425  bl 0x82ca9400
	ctx.lr = 0x82DBAFE0;
	sub_82CA93D0(ctx, base);
	// 82DBAFE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBAFE4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DBAFE8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DBAFEC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DBAFF0: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82DBAFF4: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBAFF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBAFFC: 40990058  ble cr6, 0x82dbb054
	if !ctx.cr[6].gt {
	pc = 0x82DBB054; continue 'dispatch;
	}
	// 82DBB000: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	pc = 0x82DBB004; continue 'dispatch;
            }
            0x82DBB004 => {
    //   block [0x82DBB004..0x82DBB054)
	// 82DBB004: 817B0044  lwz r11, 0x44(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB008: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DBB00C: 7FFC582E  lwzx r31, r28, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DBB010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB014: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB018: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBB01C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB020: 4E800421  bctrl
	ctx.lr = 0x82DBB024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB02C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBB030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB034: 4E800421  bctrl
	ctx.lr = 0x82DBB038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB038: 815B0048  lwz r10, 0x48(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB03C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DBB040: 546B2036  slwi r11, r3, 4
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB044: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DBB048: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DBB04C: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DBB050: 4198FFB4  blt cr6, 0x82dbb004
	if ctx.cr[6].lt {
	pc = 0x82DBB004; continue 'dispatch;
	}
            }
            0x82DBB054 => {
    //   block [0x82DBB054..0x82DBB060)
	// 82DBB054: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DBB058: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DBB05C: 4BEEE3F4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB060 size=328
    let mut pc: u32 = 0x82DBB060;
    'dispatch: loop {
        match pc {
            0x82DBB060 => {
    //   block [0x82DBB060..0x82DBB1A8)
	// 82DBB060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB064: 4BEEE3A5  bl 0x82ca9408
	ctx.lr = 0x82DBB068;
	sub_82CA93D0(ctx, base);
	// 82DBB068: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82DBB06C: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB1A8 size=8
    let mut pc: u32 = 0x82DBB1A8;
    'dispatch: loop {
        match pc {
            0x82DBB1A8 => {
    //   block [0x82DBB1A8..0x82DBB1B0)
	// 82DBB1A8: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBB1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB1B0 size=28
    let mut pc: u32 = 0x82DBB1B0;
    'dispatch: loop {
        match pc {
            0x82DBB1B0 => {
    //   block [0x82DBB1B0..0x82DBB1CC)
	// 82DBB1B0: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBB1B4: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82DBB1B8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DBB1BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DBB1C0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82DBB1C4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DBB1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB1D0 size=16
    let mut pc: u32 = 0x82DBB1D0;
    'dispatch: loop {
        match pc {
            0x82DBB1D0 => {
    //   block [0x82DBB1D0..0x82DBB1E0)
	// 82DBB1D0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBB1D4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBB1D8: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBB1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB1E0 size=220
    let mut pc: u32 = 0x82DBB1E0;
    'dispatch: loop {
        match pc {
            0x82DBB1E0 => {
    //   block [0x82DBB1E0..0x82DBB21C)
	// 82DBB1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB1E4: 4BEEE229  bl 0x82ca940c
	ctx.lr = 0x82DBB1E8;
	sub_82CA93D0(ctx, base);
	// 82DBB1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB1EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB1F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB1F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBB1F8: 396B3734  addi r11, r11, 0x3734
	ctx.r[11].s64 = ctx.r[11].s64 + 14132;
	// 82DBB1FC: 394A24EC  addi r10, r10, 0x24ec
	ctx.r[10].s64 = ctx.r[10].s64 + 9452;
	// 82DBB200: 813F0048  lwz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB204: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DBB208: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DBB20C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBB210: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBB214: 4099005C  ble cr6, 0x82dbb270
	if !ctx.cr[6].gt {
	pc = 0x82DBB270; continue 'dispatch;
	}
	// 82DBB218: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DBB21C; continue 'dispatch;
            }
            0x82DBB21C => {
    //   block [0x82DBB21C..0x82DBB25C)
	// 82DBB21C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB220: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB224: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB22C: 419A0030  beq cr6, 0x82dbb25c
	if ctx.cr[6].eq {
	pc = 0x82DBB25C; continue 'dispatch;
	}
	// 82DBB230: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBB234: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBB238: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DBB23C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBB240: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DBB244: 409A0018  bne cr6, 0x82dbb25c
	if !ctx.cr[6].eq {
	pc = 0x82DBB25C; continue 'dispatch;
	}
	// 82DBB248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB24C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DBB250: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB258: 4E800421  bctrl
	ctx.lr = 0x82DBB25C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DBB25C => {
    //   block [0x82DBB25C..0x82DBB270)
	// 82DBB25C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB260: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DBB264: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DBB268: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB26C: 4198FFB0  blt cr6, 0x82dbb21c
	if ctx.cr[6].lt {
	pc = 0x82DBB21C; continue 'dispatch;
	}
	pc = 0x82DBB270; continue 'dispatch;
            }
            0x82DBB270 => {
    //   block [0x82DBB270..0x82DBB29C)
	// 82DBB270: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DBB274: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBB278: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBB27C: 409A0020  bne cr6, 0x82dbb29c
	if !ctx.cr[6].eq {
	pc = 0x82DBB29C; continue 'dispatch;
	}
	// 82DBB280: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB284: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DBB288: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DBB28C: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB290: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DBB294: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBB298: 4BF9A031  bl 0x82d552c8
	ctx.lr = 0x82DBB29C;
	sub_82D552C8(ctx, base);
	pc = 0x82DBB29C; continue 'dispatch;
            }
            0x82DBB29C => {
    //   block [0x82DBB29C..0x82DBB2BC)
	// 82DBB29C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBB2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBB2A4: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DBB2A8: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DBB2AC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DBB2B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBB2B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBB2B8: 4BEEE1A4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB2C0 size=200
    let mut pc: u32 = 0x82DBB2C0;
    'dispatch: loop {
        match pc {
            0x82DBB2C0 => {
    //   block [0x82DBB2C0..0x82DBB364)
	// 82DBB2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB2C4: 4BEEE139  bl 0x82ca93fc
	ctx.lr = 0x82DBB2C8;
	sub_82CA93D0(ctx, base);
	// 82DBB2C8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82DBB2CC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB2D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB2D4: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB2D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DBB2DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DBB2E0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DBB2E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBB2E8: C3EB0BE4  lfs f31, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DBB2EC: 3B230048  addi r25, r3, 0x48
	ctx.r[25].s64 = ctx.r[3].s64 + 72;
	// 82DBB2F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBB2F4: 40990070  ble cr6, 0x82dbb364
	if !ctx.cr[6].gt {
	pc = 0x82DBB364; continue 'dispatch;
	}
	// 82DBB2F8: 3B430044  addi r26, r3, 0x44
	ctx.r[26].s64 = ctx.r[3].s64 + 68;
	// 82DBB2FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DBB300: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB304: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBB308: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DBB30C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB314: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBB318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB31C: 4E800421  bctrl
	ctx.lr = 0x82DBB320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB320: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            0x82DBB364 => {
    //   block [0x82DBB364..0x82DBB388)
	// 82DBB364: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB368: 576A402E  slwi r10, r27, 8
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBB36C: 556B0202  rlwinm r11, r11, 0, 8, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBB370: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBB374: 656B3F00  oris r11, r11, 0x3f00
	ctx.r[11].u64 = ctx.r[11].u64 | 1056964608;
	// 82DBB378: 917C000C  stw r11, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBB37C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DBB380: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82DBB384: 4BEEE0C8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB388 size=168
    let mut pc: u32 = 0x82DBB388;
    'dispatch: loop {
        match pc {
            0x82DBB388 => {
    //   block [0x82DBB388..0x82DBB3C4)
	// 82DBB388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB38C: 4BEEE07D  bl 0x82ca9408
	ctx.lr = 0x82DBB390;
	sub_82CA93D0(ctx, base);
	// 82DBB390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB394: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DBB398: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DBB39C: 3BBC0044  addi r29, r28, 0x44
	ctx.r[29].s64 = ctx.r[28].s64 + 68;
	// 82DBB3A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBB3A4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB3A8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBB3AC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBB3B0: 40980024  bge cr6, 0x82dbb3d4
	if !ctx.cr[6].lt {
	pc = 0x82DBB3D4; continue 'dispatch;
	}
	// 82DBB3B4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB3B8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB3BC: 41980008  blt cr6, 0x82dbb3c4
	if ctx.cr[6].lt {
	pc = 0x82DBB3C4; continue 'dispatch;
	}
	// 82DBB3C0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82DBB3C4; continue 'dispatch;
            }
            0x82DBB3C4 => {
    //   block [0x82DBB3C4..0x82DBB3D4)
	// 82DBB3C4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DBB3C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBB3CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DBB3D0: 4BF9BB41  bl 0x82d56f10
	ctx.lr = 0x82DBB3D4;
	sub_82D56F10(ctx, base);
	pc = 0x82DBB3D4; continue 'dispatch;
            }
            0x82DBB3D4 => {
    //   block [0x82DBB3D4..0x82DBB3F0)
	// 82DBB3D4: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBB3D8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DBB3DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB3E0: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB3E4: D01C0010  stfs f0, 0x10(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBB3E8: 40990040  ble cr6, 0x82dbb428
	if !ctx.cr[6].gt {
	pc = 0x82DBB428; continue 'dispatch;
	}
	// 82DBB3EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82DBB3F0; continue 'dispatch;
            }
            0x82DBB3F0 => {
    //   block [0x82DBB3F0..0x82DBB418)
	// 82DBB3F0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB3F4: 7D2BF02E  lwzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB3F8: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82DBB3FC: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB400: A12A0004  lhz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB404: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DBB408: 419A0010  beq cr6, 0x82dbb418
	if ctx.cr[6].eq {
	pc = 0x82DBB418; continue 'dispatch;
	}
	// 82DBB40C: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBB410: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBB414: B12A0006  sth r9, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	pc = 0x82DBB418; continue 'dispatch;
            }
            0x82DBB418 => {
    //   block [0x82DBB418..0x82DBB428)
	// 82DBB418: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82DBB41C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DBB420: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DBB424: 409AFFCC  bne cr6, 0x82dbb3f0
	if !ctx.cr[6].eq {
	pc = 0x82DBB3F0; continue 'dispatch;
	}
	pc = 0x82DBB428; continue 'dispatch;
            }
            0x82DBB428 => {
    //   block [0x82DBB428..0x82DBB430)
	// 82DBB428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBB42C: 4BEEE02C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB430 size=252
    let mut pc: u32 = 0x82DBB430;
    'dispatch: loop {
        match pc {
            0x82DBB430 => {
    //   block [0x82DBB430..0x82DBB484)
	// 82DBB430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB434: 4BEEDFCD  bl 0x82ca9400
	ctx.lr = 0x82DBB438;
	sub_82CA93D0(ctx, base);
	// 82DBB438: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82DBB43C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB444: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DBB448: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DBB44C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DBB450: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DBB454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB458: 419A002C  beq cr6, 0x82dbb484
	if ctx.cr[6].eq {
	pc = 0x82DBB484; continue 'dispatch;
	}
	// 82DBB45C: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB460: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DBB464: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82DBB468: EC20F82A  fadds f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82DBB46C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82DBB470: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DBB474: 4B4DE97D  bl 0x82299df0
	ctx.lr = 0x82DBB478;
	sub_82299DF0(ctx, base);
	// 82DBB478: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DBB47C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DBB480: 4BEEDFD0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DBB484 => {
    //   block [0x82DBB484..0x82DBB520)
	// 82DBB484: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB488: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DBB48C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DBB490: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DBB494: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB49C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBB4A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB4A4: 4E800421  bctrl
	ctx.lr = 0x82DBB4A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB4A8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB4AC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DBB4B0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DBB4B4: 4099006C  ble cr6, 0x82dbb520
	if !ctx.cr[6].gt {
	pc = 0x82DBB520; continue 'dispatch;
	}
	// 82DBB4B8: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82DBB4BC: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82DBB4C0: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB4C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DBB4C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DBB4CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DBB4D0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB4D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB4D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBB4DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB4E0: 4E800421  bctrl
	ctx.lr = 0x82DBB4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB4E4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            0x82DBB520 => {
    //   block [0x82DBB520..0x82DBB52C)
	// 82DBB520: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DBB524: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DBB528: 4BEEDF28  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB530 size=232
    let mut pc: u32 = 0x82DBB530;
    'dispatch: loop {
        match pc {
            0x82DBB530 => {
    //   block [0x82DBB530..0x82DBB5A8)
	// 82DBB530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB534: 4BEEDED1  bl 0x82ca9404
	ctx.lr = 0x82DBB538;
	sub_82CA93D0(ctx, base);
	// 82DBB538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB53C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBB540: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBB544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB548: 388B2BF8  addi r4, r11, 0x2bf8
	ctx.r[4].s64 = ctx.r[11].s64 + 11256;
	// 82DBB54C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DBB550: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB554: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBB558: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB55C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB564: 4E800421  bctrl
	ctx.lr = 0x82DBB568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB568: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DBB56C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBB570: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBB574: 409A0034  bne cr6, 0x82dbb5a8
	if !ctx.cr[6].eq {
	pc = 0x82DBB5A8; continue 'dispatch;
	}
	// 82DBB578: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB57C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBB580: 80FF0048  lwz r7, 0x48(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB584: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DBB588: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB58C: 388A2BEC  addi r4, r10, 0x2bec
	ctx.r[4].s64 = ctx.r[10].s64 + 11244;
	// 82DBB590: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DBB594: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBB598: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB59C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB5A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB5A4: 4E800421  bctrl
	ctx.lr = 0x82DBB5A8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DBB5A8 => {
    //   block [0x82DBB5A8..0x82DBB5C4)
	// 82DBB5A8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB5AC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DBB5B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBB5B4: 40990048  ble cr6, 0x82dbb5fc
	if !ctx.cr[6].gt {
	pc = 0x82DBB5FC; continue 'dispatch;
	}
	// 82DBB5B8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DBB5BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DBB5C0: 3B6BB3B4  addi r27, r11, -0x4c4c
	ctx.r[27].s64 = ctx.r[11].s64 + -19532;
	pc = 0x82DBB5C4; continue 'dispatch;
            }
            0x82DBB5C4 => {
    //   block [0x82DBB5C4..0x82DBB5FC)
	// 82DBB5C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB5C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBB5CC: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB5D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DBB5D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB5D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB5DC: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBB5E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB5E4: 4E800421  bctrl
	ctx.lr = 0x82DBB5E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB5E8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB5EC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DBB5F0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DBB5F4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB5F8: 4198FFCC  blt cr6, 0x82dbb5c4
	if ctx.cr[6].lt {
	pc = 0x82DBB5C4; continue 'dispatch;
	}
            }
            0x82DBB5FC => {
    //   block [0x82DBB5FC..0x82DBB618)
	// 82DBB5FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB604: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBB608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB60C: 4E800421  bctrl
	ctx.lr = 0x82DBB610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBB614: 4BEEDE40  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB618 size=360
    let mut pc: u32 = 0x82DBB618;
    'dispatch: loop {
        match pc {
            0x82DBB618 => {
    //   block [0x82DBB618..0x82DBB668)
	// 82DBB618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB61C: 4BEEDDD9  bl 0x82ca93f4
	ctx.lr = 0x82DBB620;
	sub_82CA93D0(ctx, base);
	// 82DBB620: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB624: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB628: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DBB62C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DBB630: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DBB634: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DBB638: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DBB63C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB640: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB644: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB648: 40980020  bge cr6, 0x82dbb668
	if !ctx.cr[6].lt {
	pc = 0x82DBB668; continue 'dispatch;
	}
	// 82DBB64C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBB650: 39292C08  addi r9, r9, 0x2c08
	ctx.r[9].s64 = ctx.r[9].s64 + 11272;
	// 82DBB654: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB658: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB65C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBB660: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB664: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBB668; continue 'dispatch;
            }
            0x82DBB668 => {
    //   block [0x82DBB668..0x82DBB6A0)
	// 82DBB668: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DBB66C: 3BC40014  addi r30, r4, 0x14
	ctx.r[30].s64 = ctx.r[4].s64 + 20;
	// 82DBB670: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82DBB674: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBB678: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82DBB67C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB680: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DBB684: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB688: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB690: 4E800421  bctrl
	ctx.lr = 0x82DBB694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB698: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB69C: 419A0074  beq cr6, 0x82dbb710
	if ctx.cr[6].eq {
	pc = 0x82DBB710; continue 'dispatch;
	}
            }
            0x82DBB6A0 => {
    //   block [0x82DBB6A0..0x82DBB6EC)
	// 82DBB6A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB6A4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBB6A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB6AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB6B0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB6B8: 4E800421  bctrl
	ctx.lr = 0x82DBB6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB6BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DBB6C0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DBB6C4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DBB6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBB6CC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB6D0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBB6D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB6D8: 4E800421  bctrl
	ctx.lr = 0x82DBB6DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB6DC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DBB6E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB6E4: 419A0008  beq cr6, 0x82dbb6ec
	if ctx.cr[6].eq {
	pc = 0x82DBB6EC; continue 'dispatch;
	}
	// 82DBB6E8: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
            }
            0x82DBB6EC => {
    //   block [0x82DBB6EC..0x82DBB710)
	// 82DBB6EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB6F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB6F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB6F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB6FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB700: 4E800421  bctrl
	ctx.lr = 0x82DBB704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB708: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB70C: 409AFF94  bne cr6, 0x82dbb6a0
	if !ctx.cr[6].eq {
	pc = 0x82DBB6A0; continue 'dispatch;
	}
            }
            0x82DBB710 => {
    //   block [0x82DBB710..0x82DBB730)
	// 82DBB710: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DBB714: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82DBB718: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBB71C: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DBB720: 419A0010  beq cr6, 0x82dbb730
	if ctx.cr[6].eq {
	pc = 0x82DBB730; continue 'dispatch;
	}
	// 82DBB724: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBB728: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB72C: 7F8BE92E  stwx r28, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	pc = 0x82DBB730; continue 'dispatch;
            }
            0x82DBB730 => {
    //   block [0x82DBB730..0x82DBB774)
	// 82DBB730: 7D7CD050  subf r11, r28, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[28].s64;
	// 82DBB734: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DBB738: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DBB73C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBB740: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB744: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DBB748: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DBB74C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB750: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB754: 40980020  bge cr6, 0x82dbb774
	if !ctx.cr[6].lt {
	pc = 0x82DBB774; continue 'dispatch;
	}
	// 82DBB758: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DBB75C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DBB760: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB764: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB768: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DBB76C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB770: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBB774; continue 'dispatch;
            }
            0x82DBB774 => {
    //   block [0x82DBB774..0x82DBB780)
	// 82DBB774: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DBB778: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82DBB77C: 4BEEDCC8  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB780 size=284
    let mut pc: u32 = 0x82DBB780;
    'dispatch: loop {
        match pc {
            0x82DBB780 => {
    //   block [0x82DBB780..0x82DBB7D0)
	// 82DBB780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB784: 4BEEDC79  bl 0x82ca93fc
	ctx.lr = 0x82DBB788;
	sub_82CA93D0(ctx, base);
	// 82DBB788: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB78C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB790: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DBB794: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DBB798: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DBB79C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DBB7A0: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DBB7A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB7A8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB7AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB7B0: 40980020  bge cr6, 0x82dbb7d0
	if !ctx.cr[6].lt {
	pc = 0x82DBB7D0; continue 'dispatch;
	}
	// 82DBB7B4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBB7B8: 39292B5C  addi r9, r9, 0x2b5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11100;
	// 82DBB7BC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB7C0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB7C4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBB7C8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB7CC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBB7D0; continue 'dispatch;
            }
            0x82DBB7D0 => {
    //   block [0x82DBB7D0..0x82DBB7F4)
	// 82DBB7D0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB7D4: 3BC30014  addi r30, r3, 0x14
	ctx.r[30].s64 = ctx.r[3].s64 + 20;
	// 82DBB7D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB7DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB7E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB7E4: 4E800421  bctrl
	ctx.lr = 0x82DBB7E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB7E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB7EC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB7F0: 419A0074  beq cr6, 0x82dbb864
	if ctx.cr[6].eq {
	pc = 0x82DBB864; continue 'dispatch;
	}
            }
            0x82DBB7F4 => {
    //   block [0x82DBB7F4..0x82DBB864)
	// 82DBB7F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB7F8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBB7FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB800: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB804: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB80C: 4E800421  bctrl
	ctx.lr = 0x82DBB810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB810: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB814: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DBB818: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DBB81C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DBB820: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DBB824: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DBB828: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DBB82C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DBB830: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB834: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DBB838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB83C: 4E800421  bctrl
	ctx.lr = 0x82DBB840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB840: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB848: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB84C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB854: 4E800421  bctrl
	ctx.lr = 0x82DBB858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB85C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB860: 409AFF94  bne cr6, 0x82dbb7f4
	if !ctx.cr[6].eq {
	pc = 0x82DBB7F4; continue 'dispatch;
	}
            }
            0x82DBB864 => {
    //   block [0x82DBB864..0x82DBB894)
	// 82DBB864: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DBB868: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB86C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB870: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB874: 40980020  bge cr6, 0x82dbb894
	if !ctx.cr[6].lt {
	pc = 0x82DBB894; continue 'dispatch;
	}
	// 82DBB878: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DBB87C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DBB880: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB884: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB888: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DBB88C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB890: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBB894; continue 'dispatch;
            }
            0x82DBB894 => {
    //   block [0x82DBB894..0x82DBB89C)
	// 82DBB894: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DBB898: 4BEEDBB4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB8A0 size=160
    let mut pc: u32 = 0x82DBB8A0;
    'dispatch: loop {
        match pc {
            0x82DBB8A0 => {
    //   block [0x82DBB8A0..0x82DBB940)
	// 82DBB8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBB8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBB8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB8B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB8B8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DBB8BC: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82DBB8C0: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB8C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBB8C8: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBB8CC: 394B00C4  addi r10, r11, 0xc4
	ctx.r[10].s64 = ctx.r[11].s64 + 196;
	// 82DBB8D0: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DBB8D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8D8: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DBB8DC: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DBB8E0: 392B3734  addi r9, r11, 0x3734
	ctx.r[9].s64 = ctx.r[11].s64 + 14132;
	// 82DBB8E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8E8: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB8EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8F0: 390B24EC  addi r8, r11, 0x24ec
	ctx.r[8].s64 = ctx.r[11].s64 + 9452;
	// 82DBB8F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBB8F8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBB8FC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBB900: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB904: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DBB908: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82DBB90C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82DBB910: 90FF004C  stw r7, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[7].u32 ) };
	// 82DBB914: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DBB918: 4BFFFA71  bl 0x82dbb388
	ctx.lr = 0x82DBB91C;
	sub_82DBB388(ctx, base);
	// 82DBB91C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DBB920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB924: 4BFFF73D  bl 0x82dbb060
	ctx.lr = 0x82DBB928;
	sub_82DBB060(ctx, base);
	// 82DBB928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB92C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBB930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBB934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBB938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBB93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB940 size=176
    let mut pc: u32 = 0x82DBB940;
    'dispatch: loop {
        match pc {
            0x82DBB940 => {
    //   block [0x82DBB940..0x82DBB978)
	// 82DBB940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB944: 4BEEDAC9  bl 0x82ca940c
	ctx.lr = 0x82DBB948;
	sub_82CA93D0(ctx, base);
	// 82DBB948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB94C: 3BE30008  addi r31, r3, 8
	ctx.r[31].s64 = ctx.r[3].s64 + 8;
	// 82DBB950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBB954: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DBB958: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB95C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB960: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBB964: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB968: 409A0010  bne cr6, 0x82dbb978
	if !ctx.cr[6].eq {
	pc = 0x82DBB978; continue 'dispatch;
	}
	// 82DBB96C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DBB970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB974: 4BF9B625  bl 0x82d56f98
	ctx.lr = 0x82DBB978;
	sub_82D56F98(ctx, base);
	pc = 0x82DBB978; continue 'dispatch;
            }
            0x82DBB978 => {
    //   block [0x82DBB978..0x82DBB9A0)
	// 82DBB978: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB97C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82DBB980: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB984: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82DBB988: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB98C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DBB990: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBB994: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB99C: 419A0014  beq cr6, 0x82dbb9b0
	if ctx.cr[6].eq {
	pc = 0x82DBB9B0; continue 'dispatch;
	}
	pc = 0x82DBB9A0; continue 'dispatch;
            }
            0x82DBB9A0 => {
    //   block [0x82DBB9A0..0x82DBB9B0)
	// 82DBB9A0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DBB9A4: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB9AC: 409AFFF4  bne cr6, 0x82dbb9a0
	if !ctx.cr[6].eq {
	pc = 0x82DBB9A0; continue 'dispatch;
	}
	pc = 0x82DBB9B0; continue 'dispatch;
            }
            0x82DBB9B0 => {
    //   block [0x82DBB9B0..0x82DBB9CC)
	// 82DBB9B0: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBB9B4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DBB9B8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB9BC: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DBB9C0: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB9C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DBB9C8: 419A0014  beq cr6, 0x82dbb9dc
	if ctx.cr[6].eq {
	pc = 0x82DBB9DC; continue 'dispatch;
	}
	pc = 0x82DBB9CC; continue 'dispatch;
            }
            0x82DBB9CC => {
    //   block [0x82DBB9CC..0x82DBB9DC)
	// 82DBB9CC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBB9D0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB9D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DBB9D8: 409AFFF4  bne cr6, 0x82dbb9cc
	if !ctx.cr[6].eq {
	pc = 0x82DBB9CC; continue 'dispatch;
	}
	pc = 0x82DBB9DC; continue 'dispatch;
            }
            0x82DBB9DC => {
    //   block [0x82DBB9DC..0x82DBB9F0)
	// 82DBB9DC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBB9E0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB9E4: 9169000C  stw r11, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBB9E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBB9EC: 4BEEDA70  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB9F0 size=20
    let mut pc: u32 = 0x82DBB9F0;
    'dispatch: loop {
        match pc {
            0x82DBB9F0 => {
    //   block [0x82DBB9F0..0x82DBBA04)
	// 82DBB9F0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBB9F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB9F8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBB9FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBA08 size=144
    let mut pc: u32 = 0x82DBBA08;
    'dispatch: loop {
        match pc {
            0x82DBBA08 => {
    //   block [0x82DBBA08..0x82DBBA98)
	// 82DBBA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBBA10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBBA14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBBA18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBA1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBBA20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBBA24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBBA28: 388B2C30  addi r4, r11, 0x2c30
	ctx.r[4].s64 = ctx.r[11].s64 + 11312;
	// 82DBBA2C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBBA30: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBA34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBBA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBBA3C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBA40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA44: 4E800421  bctrl
	ctx.lr = 0x82DBBA48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBA48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBA4C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DBBA50: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBA54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBBA58: 388BB3B4  addi r4, r11, -0x4c4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19532;
	// 82DBBA5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBBA60: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBBA64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA68: 4E800421  bctrl
	ctx.lr = 0x82DBBA6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBA6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBBA74: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBA78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA7C: 4E800421  bctrl
	ctx.lr = 0x82DBBA80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBA80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBBA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBBA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBBA8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBBA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBBA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBBA98 size=8
    let mut pc: u32 = 0x82DBBA98;
    'dispatch: loop {
        match pc {
            0x82DBBA98 => {
    //   block [0x82DBBA98..0x82DBBAA0)
	// 82DBBA98: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DBBA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBBAA0 size=52
    let mut pc: u32 = 0x82DBBAA0;
    'dispatch: loop {
        match pc {
            0x82DBBAA0 => {
    //   block [0x82DBBAA0..0x82DBBAD4)
	// 82DBBAA0: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBBAD8 size=136
    let mut pc: u32 = 0x82DBBAD8;
    'dispatch: loop {
        match pc {
            0x82DBBAD8 => {
    //   block [0x82DBBAD8..0x82DBBB2C)
	// 82DBBAD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBBADC: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBBAE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBBAE4: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBBAE8: 396B346C  addi r11, r11, 0x346c
	ctx.r[11].s64 = ctx.r[11].s64 + 13420;
	// 82DBBAEC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DBBAF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DBBAF4: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 82DBBAF8: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DBBAFC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBBB00: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DBBB04: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DBBB08: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBBB0C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBBB10: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82DBBB14: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBB18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBBB1C: 419A0010  beq cr6, 0x82dbbb2c
	if ctx.cr[6].eq {
	pc = 0x82DBBB2C; continue 'dispatch;
	}
	// 82DBBB20: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBBB24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBBB28: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82DBBB2C; continue 'dispatch;
            }
            0x82DBBB2C => {
    //   block [0x82DBBB2C..0x82DBBB60)
	// 82DBBB2C: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBB60 size=108
    let mut pc: u32 = 0x82DBBB60;
    'dispatch: loop {
        match pc {
            0x82DBBB60 => {
    //   block [0x82DBBB60..0x82DBBBCC)
	// 82DBBB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBBB68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBBB6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBBB70: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DBBB74: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBB78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBBB7C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DBBB80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBBB84: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82DBBB88: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBBB8C: 4BF9B775  bl 0x82d57300
	ctx.lr = 0x82DBBB90;
	sub_82D57300(ctx, base);
	// 82DBBB90: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBB94: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBBB98: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DBBB9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DBBBA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBBA4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBBBA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBBAC: 4E800421  bctrl
	ctx.lr = 0x82DBBBB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBBB0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DBBBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBBBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBBBBC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DBBBC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBBBC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBBBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBBD0 size=216
    let mut pc: u32 = 0x82DBBBD0;
    'dispatch: loop {
        match pc {
            0x82DBBBD0 => {
    //   block [0x82DBBBD0..0x82DBBCA8)
	// 82DBBBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBBD4: 4BEED835  bl 0x82ca9408
	ctx.lr = 0x82DBBBD8;
	sub_82CA93D0(ctx, base);
	// 82DBBBD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBBDC: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 82DBBBE0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBBE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBBBE8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82DBBBEC: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82DBBBF0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBCA8 size=436
    let mut pc: u32 = 0x82DBBCA8;
    'dispatch: loop {
        match pc {
            0x82DBBCA8 => {
    //   block [0x82DBBCA8..0x82DBBCF4)
	// 82DBBCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBCAC: 4BEED759  bl 0x82ca9404
	ctx.lr = 0x82DBBCB0;
	sub_82CA93D0(ctx, base);
	// 82DBBCB0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBCB4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBCB8: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DBBCBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBBCC0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBBCC4: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DBBCC8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBCCC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBBCD0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBBCD4: 40980020  bge cr6, 0x82dbbcf4
	if !ctx.cr[6].lt {
	pc = 0x82DBBCF4; continue 'dispatch;
	}
	// 82DBBCD8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBBCDC: 39292C40  addi r9, r9, 0x2c40
	ctx.r[9].s64 = ctx.r[9].s64 + 11328;
	// 82DBBCE0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBBCE4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBBCE8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBBCEC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBBCF0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBBCF4; continue 'dispatch;
            }
            0x82DBBCF4 => {
    //   block [0x82DBBCF4..0x82DBBD04)
	// 82DBBCF4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DBBCF8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DBBCFC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DBBD00: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DBBD04; continue 'dispatch;
            }
            0x82DBBD04 => {
    //   block [0x82DBBD04..0x82DBBE5C)
	// 82DBBD04: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBBD08: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBBD0C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DBBD10: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DBBD14: 4200FFF0  bdnz 0x82dbbd04
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBBD04; continue 'dispatch;
	}
	// 82DBBD18: 3BE40020  addi r31, r4, 0x20
	ctx.r[31].s64 = ctx.r[4].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBE60 size=384
    let mut pc: u32 = 0x82DBBE60;
    'dispatch: loop {
        match pc {
            0x82DBBE60 => {
    //   block [0x82DBBE60..0x82DBBEB4)
	// 82DBBE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBE64: 4BEED5A1  bl 0x82ca9404
	ctx.lr = 0x82DBBE68;
	sub_82CA93D0(ctx, base);
	// 82DBBE68: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBE6C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBE70: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DBBE74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBBE78: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DBBE7C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBBE80: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DBBE84: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DBBE88: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBE8C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBBE90: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBBE94: 40980020  bge cr6, 0x82dbbeb4
	if !ctx.cr[6].lt {
	pc = 0x82DBBEB4; continue 'dispatch;
	}
	// 82DBBE98: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBBE9C: 39082C50  addi r8, r8, 0x2c50
	ctx.r[8].s64 = ctx.r[8].s64 + 11344;
	// 82DBBEA0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBBEA4: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBBEA8: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82DBBEAC: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBBEB0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82DBBEB4; continue 'dispatch;
            }
            0x82DBBEB4 => {
    //   block [0x82DBBEB4..0x82DBBEC4)
	// 82DBBEB4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DBBEB8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBBEBC: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82DBBEC0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	pc = 0x82DBBEC4; continue 'dispatch;
            }
            0x82DBBEC4 => {
    //   block [0x82DBBEC4..0x82DBBFE0)
	// 82DBBEC4: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBBEC8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBBECC: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DBBED0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DBBED4: 4200FFF0  bdnz 0x82dbbec4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBBEC4; continue 'dispatch;
	}
	// 82DBBED8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBFE0 size=188
    let mut pc: u32 = 0x82DBBFE0;
    'dispatch: loop {
        match pc {
            0x82DBBFE0 => {
    //   block [0x82DBBFE0..0x82DBC09C)
	// 82DBBFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBFE4: 4BEED425  bl 0x82ca9408
	ctx.lr = 0x82DBBFE8;
	sub_82CA93D0(ctx, base);
	// 82DBBFE8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBFEC: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC0A0 size=252
    let mut pc: u32 = 0x82DBC0A0;
    'dispatch: loop {
        match pc {
            0x82DBC0A0 => {
    //   block [0x82DBC0A0..0x82DBC19C)
	// 82DBC0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC0A4: 4BEED361  bl 0x82ca9404
	ctx.lr = 0x82DBC0A8;
	sub_82CA93D0(ctx, base);
	// 82DBC0A8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC0AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC0B0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBC0B4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DBC0B8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC0BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC0C0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBC0C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC0C8: 4E800421  bctrl
	ctx.lr = 0x82DBC0CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC0CC: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC0D0: 387EFFFF  addi r3, r30, -1
	ctx.r[3].s64 = ctx.r[30].s64 + -1;
	// 82DBC0D4: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DBC0D8: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DBC0DC: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DBC0E0: EBCB0000  ld r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBC0E4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBC0E8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DBC0EC: 547F2036  slwi r31, r3, 4
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DBC0F0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DBC0F4: 7FFFEA14  add r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82DBC0F8: EBA60000  ld r29, 0(r6)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DBC0FC: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DBC100: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DBC104: FBCA0000  std r30, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82DBC108: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82DBC10C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DBC110: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DBC114: EB850000  ld r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DBC118: FBA90000  std r29, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 82DBC11C: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DBC120: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC1A0 size=140
    let mut pc: u32 = 0x82DBC1A0;
    'dispatch: loop {
        match pc {
            0x82DBC1A0 => {
    //   block [0x82DBC1A0..0x82DBC22C)
	// 82DBC1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC1A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBC1AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC1B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBC1B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBC1BC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC1C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC1C4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBC1C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC1CC: 4E800421  bctrl
	ctx.lr = 0x82DBC1D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC1D0: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC230 size=148
    let mut pc: u32 = 0x82DBC230;
    'dispatch: loop {
        match pc {
            0x82DBC230 => {
    //   block [0x82DBC230..0x82DBC2C4)
	// 82DBC230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBC23C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC244: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC248: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBC24C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DBC250: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC258: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBC25C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC260: 4E800421  bctrl
	ctx.lr = 0x82DBC264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC264: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBC268: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC26C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DBC270: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC2C8 size=280
    let mut pc: u32 = 0x82DBC2C8;
    'dispatch: loop {
        match pc {
            0x82DBC2C8 => {
    //   block [0x82DBC2C8..0x82DBC3E0)
	// 82DBC2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC2CC: 4BEED12D  bl 0x82ca93f8
	ctx.lr = 0x82DBC2D0;
	sub_82CA93D0(ctx, base);
	// 82DBC2D0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC2D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC2D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DBC2DC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC2E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC2E4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBC2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC2EC: 4E800421  bctrl
	ctx.lr = 0x82DBC2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC2F0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC2F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBC2F8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DBC2FC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC300: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBC304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC308: 4E800421  bctrl
	ctx.lr = 0x82DBC30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC30C: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC310: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 82DBC314: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DBC318: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82DBC31C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DBC320: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBC324: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DBC328: EB0B0008  ld r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DBC32C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DBC330: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82DBC334: EB650000  ld r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DBC338: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82DBC33C: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DBC340: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBC344: F8690000  std r3, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82DBC348: 7F9EE850  subf r28, r30, r29
	ctx.r[28].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82DBC34C: FB090008  std r24, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82DBC350: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DBC354: EB440000  ld r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DBC358: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DBC35C: FB680000  std r27, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DBC360: F8A80008  std r5, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82DBC364: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC3E8 size=196
    let mut pc: u32 = 0x82DBC3E8;
    'dispatch: loop {
        match pc {
            0x82DBC3E8 => {
    //   block [0x82DBC3E8..0x82DBC420)
	// 82DBC3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC3F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC3F4: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC3F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC3FC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBC400: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBC404: 409A001C  bne cr6, 0x82dbc420
	if !ctx.cr[6].eq {
	pc = 0x82DBC420; continue 'dispatch;
	}
	// 82DBC408: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DBC40C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DBC410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBC41C: 4E800020  blr
	return;
            }
            0x82DBC420 => {
    //   block [0x82DBC420..0x82DBC480)
	// 82DBC420: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC424: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBC428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DBC42C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC430: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBC434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC438: 4E800421  bctrl
	ctx.lr = 0x82DBC43C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC43C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DBC440: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBC444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBC448: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 82DBC44C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82DBC450: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DBC454: C02AB384  lfs f1, -0x4c7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBC458: 4BFFE961  bl 0x82dbadb8
	ctx.lr = 0x82DBC45C;
	sub_82DBADB8(ctx, base);
	// 82DBC45C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBC464: 409A001C  bne cr6, 0x82dbc480
	if !ctx.cr[6].eq {
	pc = 0x82DBC480; continue 'dispatch;
	}
	// 82DBC468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBC46C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DBC470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBC47C: 4E800020  blr
	return;
            }
            0x82DBC480 => {
    //   block [0x82DBC480..0x82DBC4AC)
	// 82DBC480: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC484: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DBC488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC48C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBC490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC494: 4E800421  bctrl
	ctx.lr = 0x82DBC498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC498: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DBC49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC4A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBC4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC4B0 size=204
    let mut pc: u32 = 0x82DBC4B0;
    'dispatch: loop {
        match pc {
            0x82DBC4B0 => {
    //   block [0x82DBC4B0..0x82DBC4E0)
	// 82DBC4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC4B4: 4BEECF49  bl 0x82ca93fc
	ctx.lr = 0x82DBC4B8;
	sub_82CA93D0(ctx, base);
	// 82DBC4B8: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC4BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC4C0: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82DBC4C4: 3F408330  lis r26, -0x7cd0
	ctx.r[26].s64 = -2094006272;
	// 82DBC4C8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBC4CC: 214B0020  subfic r10, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DBC4D0: 7F2B5C30  srw r11, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[25].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBC4D4: 7C9D5430  srw r29, r4, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[4].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBC4D8: 7D7E2038  and r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82DBC4DC: 1F7D0038  mulli r27, r29, 0x38
	ctx.r[27].s32 = ((ctx.r[29].s32 as i64 * 56 as i64) as i32);
	ctx.r[27].s64 = ctx.r[27].s32 as i64;
	pc = 0x82DBC4E0; continue 'dispatch;
            }
            0x82DBC4E0 => {
    //   block [0x82DBC4E0..0x82DBC510)
	// 82DBC4E0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBC4E4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DBC4E8: 7D7B5A14  add r11, r27, r11
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82DBC4EC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBC4F0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBC4F4: 4198001C  blt cr6, 0x82dbc510
	if ctx.cr[6].lt {
	pc = 0x82DBC510; continue 'dispatch;
	}
	// 82DBC4F8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBC4FC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DBC500: 3B7B0038  addi r27, r27, 0x38
	ctx.r[27].s64 = ctx.r[27].s64 + 56;
	// 82DBC504: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DBC508: 40980068  bge cr6, 0x82dbc570
	if !ctx.cr[6].lt {
	pc = 0x82DBC570; continue 'dispatch;
	}
	// 82DBC50C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DBC510; continue 'dispatch;
            }
            0x82DBC510 => {
    //   block [0x82DBC510..0x82DBC570)
	// 82DBC510: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBC514: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBC518: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC520: 216B0020  subfic r11, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DBC524: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBC528: 7FAB5830  slw r11, r29, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[29].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBC52C: 7D7CF378  or r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 | ctx.r[30].u64;
	// 82DBC530: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DBC534: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DBC538: 4E800421  bctrl
	ctx.lr = 0x82DBC53C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC53C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBC540: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBC544: C03AB384  lfs f1, -0x4c7c(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBC548: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 82DBC54C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82DBC550: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DBC554: 4BFFE865  bl 0x82dbadb8
	ctx.lr = 0x82DBC558;
	sub_82DBADB8(ctx, base);
	// 82DBC558: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBC560: 409AFF80  bne cr6, 0x82dbc4e0
	if !ctx.cr[6].eq {
	pc = 0x82DBC4E0; continue 'dispatch;
	}
	// 82DBC564: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DBC568: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DBC56C: 4BEECEE0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DBC570 => {
    //   block [0x82DBC570..0x82DBC57C)
	// 82DBC570: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DBC574: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DBC578: 4BEECED4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC580 size=68
    let mut pc: u32 = 0x82DBC580;
    'dispatch: loop {
        match pc {
            0x82DBC580 => {
    //   block [0x82DBC580..0x82DBC5B0)
	// 82DBC580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC58C: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DBC590: 480009D9  bl 0x82dbcf68
	ctx.lr = 0x82DBC594;
	sub_82DBCF68(ctx, base);
	// 82DBC594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DBC598: 419A0018  beq cr6, 0x82dbc5b0
	if ctx.cr[6].eq {
	pc = 0x82DBC5B0; continue 'dispatch;
	}
	// 82DBC59C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBC5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC5AC: 4E800020  blr
	return;
            }
            0x82DBC5B0 => {
    //   block [0x82DBC5B0..0x82DBC5C4)
	// 82DBC5B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBC5B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBC5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC5C8 size=168
    let mut pc: u32 = 0x82DBC5C8;
    'dispatch: loop {
        match pc {
            0x82DBC5C8 => {
    //   block [0x82DBC5C8..0x82DBC670)
	// 82DBC5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC5D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC5D4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC5D8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBC5DC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DBC5E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC5E4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBC5E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DBC5EC: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC5F0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DBC5F4: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBC5F8: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC5FC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBC600: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DBC604: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DBC608: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC60C: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBC610: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DBC614: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBC618: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DBC61C: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC620: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBC624: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DBC628: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC670 size=704
    let mut pc: u32 = 0x82DBC670;
    'dispatch: loop {
        match pc {
            0x82DBC670 => {
    //   block [0x82DBC670..0x82DBC930)
	// 82DBC670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC674: 4BEECD79  bl 0x82ca93ec
	ctx.lr = 0x82DBC678;
	sub_82CA93D0(ctx, base);
	// 82DBC678: DBC1FF90  stfd f30, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[30].u64 ) };
	// 82DBC67C: DBE1FF98  stfd f31, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82DBC680: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC684: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC688: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82DBC68C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DBC690: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DBC694: 3B3A0010  addi r25, r26, 0x10
	ctx.r[25].s64 = ctx.r[26].s64 + 16;
	// 82DBC698: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DBC69C: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC6A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC6A4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DBC6A8: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DBC6AC: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DBC6B0: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DBC6B4: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DBC6B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC6BC: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DBC6C0: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DBC6C4: C00B0BE4  lfs f0, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC6C8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DBC6CC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DBC6D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DBC6D4: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC940 size=176
    let mut pc: u32 = 0x82DBC940;
    'dispatch: loop {
        match pc {
            0x82DBC940 => {
    //   block [0x82DBC940..0x82DBC9F0)
	// 82DBC940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBC94C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC950: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DBC954: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC958: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DBC95C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DBC960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC964: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBC968: 4BFFC631  bl 0x82db8f98
	ctx.lr = 0x82DBC96C;
	sub_82DB8F98(ctx, base);
	// 82DBC96C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBC970: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC974: 392A254C  addi r9, r10, 0x254c
	ctx.r[9].s64 = ctx.r[10].s64 + 9548;
	// 82DBC978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBC97C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DBC980: 390A2C64  addi r8, r10, 0x2c64
	ctx.r[8].s64 = ctx.r[10].s64 + 11364;
	// 82DBC984: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBC988: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC98C: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC990: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBC994: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DBC998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC99C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DBC9A0: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82DBC9A4: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DBC9A8: 90FF003C  stw r7, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[7].u32 ) };
	// 82DBC9AC: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82DBC9B0: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82DBC9B4: 90FF0048  stw r7, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[7].u32 ) };
	// 82DBC9B8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBC9F0 size=136
    let mut pc: u32 = 0x82DBC9F0;
    'dispatch: loop {
        match pc {
            0x82DBC9F0 => {
    //   block [0x82DBC9F0..0x82DBCA40)
	// 82DBC9F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBC9F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBC9F8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBC9FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DBCA00: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DBCA04: 394A254C  addi r10, r10, 0x254c
	ctx.r[10].s64 = ctx.r[10].s64 + 9548;
	// 82DBCA08: 39292C64  addi r9, r9, 0x2c64
	ctx.r[9].s64 = ctx.r[9].s64 + 11364;
	// 82DBCA0C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82DBCA10: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DBCA14: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DBCA18: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DBCA1C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBCA20: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DBCA24: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBCA28: 419A0044  beq cr6, 0x82dbca6c
	if ctx.cr[6].eq {
	pc = 0x82DBCA6C; continue 'dispatch;
	}
	// 82DBCA2C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCA30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBCA34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCA38: 40990034  ble cr6, 0x82dbca6c
	if !ctx.cr[6].gt {
	pc = 0x82DBCA6C; continue 'dispatch;
	}
	// 82DBCA3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBCA40; continue 'dispatch;
            }
            0x82DBCA40 => {
    //   block [0x82DBCA40..0x82DBCA58)
	// 82DBCA40: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCA44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCA48: 88EB0011  lbz r7, 0x11(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DBCA4C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DBCA50: 409A0008  bne cr6, 0x82dbca58
	if !ctx.cr[6].eq {
	pc = 0x82DBCA58; continue 'dispatch;
	}
	// 82DBCA54: 990B0011  stb r8, 0x11(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(17 as u32), ctx.r[8].u8 ) };
	pc = 0x82DBCA58; continue 'dispatch;
            }
            0x82DBCA58 => {
    //   block [0x82DBCA58..0x82DBCA6C)
	// 82DBCA58: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCA5C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBCA60: 394A0038  addi r10, r10, 0x38
	ctx.r[10].s64 = ctx.r[10].s64 + 56;
	// 82DBCA64: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCA68: 4198FFD8  blt cr6, 0x82dbca40
	if ctx.cr[6].lt {
	pc = 0x82DBCA40; continue 'dispatch;
	}
	pc = 0x82DBCA6C; continue 'dispatch;
            }
            0x82DBCA6C => {
    //   block [0x82DBCA6C..0x82DBCA78)
	// 82DBCA6C: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82DBCA70: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBCA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBCA78 size=364
    let mut pc: u32 = 0x82DBCA78;
    'dispatch: loop {
        match pc {
            0x82DBCA78 => {
    //   block [0x82DBCA78..0x82DBCAAC)
	// 82DBCA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBCA7C: 4BEEC971  bl 0x82ca93ec
	ctx.lr = 0x82DBCA80;
	sub_82CA93D0(ctx, base);
	// 82DBCA80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBCA84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBCA88: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82DBCA8C: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82DBCA90: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBCA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBCA98: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCA9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCAA0: 40990038  ble cr6, 0x82dbcad8
	if !ctx.cr[6].gt {
	pc = 0x82DBCAD8; continue 'dispatch;
	}
	// 82DBCAA4: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82DBCAAC; continue 'dispatch;
            }
            0x82DBCAAC => {
    //   block [0x82DBCAAC..0x82DBCAD8)
	// 82DBCAAC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCAB0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBCAB4: 93EA0034  stw r31, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 82DBCAB8: 811D0038  lwz r8, 0x38(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCABC: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCAC0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DBCAC4: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCAC8: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82DBCACC: 8108001C  lwz r8, 0x1c(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBCAD0: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82DBCAD4: 4198FFD8  blt cr6, 0x82dbcaac
	if ctx.cr[6].lt {
	pc = 0x82DBCAAC; continue 'dispatch;
	}
	pc = 0x82DBCAD8; continue 'dispatch;
            }
            0x82DBCAD8 => {
    //   block [0x82DBCAD8..0x82DBCAFC)
	// 82DBCAD8: 3AFD0040  addi r23, r29, 0x40
	ctx.r[23].s64 = ctx.r[29].s64 + 64;
	// 82DBCADC: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCAE0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCAE4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBCAE8: 40980014  bge cr6, 0x82dbcafc
	if !ctx.cr[6].lt {
	pc = 0x82DBCAFC; continue 'dispatch;
	}
	// 82DBCAEC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DBCAF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBCAF4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DBCAF8: 4BF9A419  bl 0x82d56f10
	ctx.lr = 0x82DBCAFC;
	sub_82D56F10(ctx, base);
	pc = 0x82DBCAFC; continue 'dispatch;
            }
            0x82DBCAFC => {
    //   block [0x82DBCAFC..0x82DBCB1C)
	// 82DBCAFC: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCB00: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCB04: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBCB08: 40980024  bge cr6, 0x82dbcb2c
	if !ctx.cr[6].lt {
	pc = 0x82DBCB2C; continue 'dispatch;
	}
	// 82DBCB0C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBCB10: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCB14: 41980008  blt cr6, 0x82dbcb1c
	if ctx.cr[6].lt {
	pc = 0x82DBCB1C; continue 'dispatch;
	}
	// 82DBCB18: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82DBCB1C; continue 'dispatch;
            }
            0x82DBCB1C => {
    //   block [0x82DBCB1C..0x82DBCB2C)
	// 82DBCB1C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DBCB20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBCB24: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DBCB28: 4BF9A3E9  bl 0x82d56f10
	ctx.lr = 0x82DBCB2C;
	sub_82D56F10(ctx, base);
	pc = 0x82DBCB2C; continue 'dispatch;
            }
            0x82DBCB2C => {
    //   block [0x82DBCB2C..0x82DBCB48)
	// 82DBCB2C: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBCB30: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DBCB34: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCB38: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82DBCB3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCB40: 40990090  ble cr6, 0x82dbcbd0
	if !ctx.cr[6].gt {
	pc = 0x82DBCBD0; continue 'dispatch;
	}
	// 82DBCB44: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	pc = 0x82DBCB48; continue 'dispatch;
            }
            0x82DBCB48 => {
    //   block [0x82DBCB48..0x82DBCB74)
	// 82DBCB48: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCB4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBCB50: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82DBCB54: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBCB58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCB5C: 40990060  ble cr6, 0x82dbcbbc
	if !ctx.cr[6].gt {
	pc = 0x82DBCBBC; continue 'dispatch;
	}
	// 82DBCB60: 3975FFFB  addi r11, r21, -5
	ctx.r[11].s64 = ctx.r[21].s64 + -5;
	// 82DBCB64: 577E083C  slwi r30, r27, 1
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DBCB68: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DBCB6C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBCB70: 69790001  xori r25, r11, 1
	ctx.r[25].u64 = ctx.r[11].u64 ^ 1;
	pc = 0x82DBCB74; continue 'dispatch;
            }
            0x82DBCB74 => {
    //   block [0x82DBCB74..0x82DBCBBC)
	// 82DBCB74: 815D0030  lwz r10, 0x30(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBCB78: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DBCB7C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCB80: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DBCB84: 214A0020  subfic r10, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[10].s64;
	// 82DBCB88: 7F8BF214  add r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DBCB8C: 7F0B5030  slw r11, r24, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[24].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCB90: 7D63FB78  or r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 82DBCB94: 48025EDD  bl 0x82de2a70
	ctx.lr = 0x82DBCB98;
	sub_82DE2A70(ctx, base);
	// 82DBCB98: B07C0000  sth r3, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 82DBCB9C: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCBA0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBCBA4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DBCBA8: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82DBCBAC: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DBCBB0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBCBB4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCBB8: 4198FFBC  blt cr6, 0x82dbcb74
	if ctx.cr[6].lt {
	pc = 0x82DBCB74; continue 'dispatch;
	}
	pc = 0x82DBCBBC; continue 'dispatch;
            }
            0x82DBCBBC => {
    //   block [0x82DBCBBC..0x82DBCBD0)
	// 82DBCBBC: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCBC0: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82DBCBC4: 3B5A0038  addi r26, r26, 0x38
	ctx.r[26].s64 = ctx.r[26].s64 + 56;
	// 82DBCBC8: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCBCC: 4198FF7C  blt cr6, 0x82dbcb48
	if ctx.cr[6].lt {
	pc = 0x82DBCB48; continue 'dispatch;
	}
	pc = 0x82DBCBD0; continue 'dispatch;
            }
            0x82DBCBD0 => {
    //   block [0x82DBCBD0..0x82DBCBE4)
	// 82DBCBD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DBCBD4: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82DBCBD8: 9ABD004C  stb r21, 0x4c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(76 as u32), ctx.r[21].u8 ) };
	// 82DBCBDC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DBCBE0: 4BEEC85C  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBCBE8 size=560
    let mut pc: u32 = 0x82DBCBE8;
    'dispatch: loop {
        match pc {
            0x82DBCBE8 => {
    //   block [0x82DBCBE8..0x82DBCC7C)
	// 82DBCBE8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DBCBEC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DBCBF0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBCBF4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82DBCBF8: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBCBFC: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DBCC00: 7D2B5C30  srw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCC04: 7D662038  and r6, r11, r4
	ctx.r[6].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82DBCC08: 7C8B4430  srw r11, r4, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[4].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCC0C: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 56 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBCC10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCC14: 892B0010  lbz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DBCC18: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBCC1C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82DBCC20: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBCC24: 7D4A3038  and r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[6].u64;
	// 82DBCC28: 409A0054  bne cr6, 0x82dbcc7c
	if !ctx.cr[6].eq {
	pc = 0x82DBCC7C; continue 'dispatch;
	}
	// 82DBCC2C: 69440001  xori r4, r10, 1
	ctx.r[4].u64 = ctx.r[10].u64 ^ 1;
	// 82DBCC30: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBCC34: 7CE931D6  mullw r7, r9, r6
	ctx.r[7].s32 = ((ctx.r[9].s32 as i64 * ctx.r[6].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82DBCC38: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCC3C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCC40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DBCC44: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82DBCC48: 7CE7FA14  add r7, r7, r31
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82DBCC4C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBCC50: 5484083C  slwi r4, r4, 1
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DBCC54: A3E70000  lhz r31, 0(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCC58: 7FCA3A2E  lhzx r30, r10, r7
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DBCC5C: 7C843A2E  lhzx r4, r4, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DBCC60: 7D5F49D6  mullw r10, r31, r9
	ctx.r[10].s32 = ((ctx.r[31].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82DBCC64: 7CFE49D6  mullw r7, r30, r9
	ctx.r[7].s32 = ((ctx.r[30].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82DBCC68: 7D2449D6  mullw r9, r4, r9
	ctx.r[9].s32 = ((ctx.r[4].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82DBCC6C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DBCC70: 7CE74214  add r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82DBCC74: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DBCC78: 48000050  b 0x82dbccc8
	pc = 0x82DBCCC8; continue 'dispatch;
            }
            0x82DBCC7C => {
    //   block [0x82DBCC7C..0x82DBCCC8)
	// 82DBCC7C: 69470001  xori r7, r10, 1
	ctx.r[7].u64 = ctx.r[10].u64 ^ 1;
	// 82DBCC80: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBCC84: 7D0931D6  mullw r8, r9, r6
	ctx.r[8].s32 = ((ctx.r[9].s32 as i64 * ctx.r[6].s32 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82DBCC88: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCC8C: 3BEA0001  addi r31, r10, 1
	ctx.r[31].s64 = ctx.r[10].s64 + 1;
	// 82DBCC90: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DBCC94: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCC98: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82DBCC9C: 57E4103A  slwi r4, r31, 2
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DBCCA0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DBCCA4: 83E80000  lwz r31, 0(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCCA8: 7FC4402E  lwzx r30, r4, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DBCCAC: 7D07402E  lwzx r8, r7, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DBCCB0: 7C9F51D6  mullw r4, r31, r10
	ctx.r[4].s32 = ((ctx.r[31].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 82DBCCB4: 7CFE51D6  mullw r7, r30, r10
	ctx.r[7].s32 = ((ctx.r[30].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82DBCCB8: 7D0851D6  mullw r8, r8, r10
	ctx.r[8].s32 = ((ctx.r[8].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82DBCCBC: 7D444A14  add r10, r4, r9
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[9].u64;
	// 82DBCCC0: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82DBCCC4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	pc = 0x82DBCCC8; continue 'dispatch;
            }
            0x82DBCCC8 => {
    //   block [0x82DBCCC8..0x82DBCD6C)
	// 82DBCCC8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DBCCCC: C1A30014  lfs f13, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBCCD0: C1630018  lfs f11, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DBCCD4: 80830034  lwz r4, 0x34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCCD8: C12A0008  lfs f9, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DBCCDC: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBCCE0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DBCCE4: C1070000  lfs f8, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DBCCE8: C1880C18  lfs f12, 0xc18(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DBCCEC: D181FFCC  stfs f12, -0x34(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 82DBCCF0: D181FFDC  stfs f12, -0x24(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82DBCCF4: D181FFEC  stfs f12, -0x14(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82DBCCF8: C18A0004  lfs f12, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DBCCFC: ED8C0372  fmuls f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DBCD00: D181FFC4  stfs f12, -0x3c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82DBCD04: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DBCD08: ED8902F2  fmuls f12, f9, f11
	ctx.f[12].f64 = (((ctx.f[9].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DBCD0C: C0A90000  lfs f5, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82DBCD10: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBCD14: D181FFC8  stfs f12, -0x38(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 82DBCD18: ED800232  fmuls f12, f0, f8
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[8].f64) as f32) as f64);
	// 82DBCD1C: C0E70004  lfs f7, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82DBCD20: EC000172  fmuls f0, f0, f5
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[5].f64) as f32) as f64);
	// 82DBCD24: C0890004  lfs f4, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82DBCD28: D181FFD0  stfs f12, -0x30(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82DBCD2C: ED870372  fmuls f12, f7, f13
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DBCD30: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82DBCD34: EC040372  fmuls f0, f4, f13
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DBCD38: C0C70008  lfs f6, 8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82DBCD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DBCD40: C0690008  lfs f3, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82DBCD44: D181FFD4  stfs f12, -0x2c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82DBCD48: ED8602F2  fmuls f12, f6, f11
	ctx.f[12].f64 = (((ctx.f[6].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DBCD4C: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82DBCD50: EC0302F2  fmuls f0, f3, f11
	ctx.f[0].f64 = (((ctx.f[3].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DBCD54: D141FFC0  stfs f10, -0x40(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82DBCD58: D181FFD8  stfs f12, -0x28(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82DBCD5C: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82DBCD60: 409A000C  bne cr6, 0x82dbcd6c
	if !ctx.cr[6].eq {
	pc = 0x82DBCD6C; continue 'dispatch;
	}
	// 82DBCD64: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DBCD68: 48000018  b 0x82dbcd80
	pc = 0x82DBCD80; continue 'dispatch;
            }
            0x82DBCD6C => {
    //   block [0x82DBCD6C..0x82DBCD80)
	// 82DBCD6C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCD70: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBCD74: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82DBCD78: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBCD7C: 7D0B522E  lhzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	pc = 0x82DBCD80; continue 'dispatch;
            }
            0x82DBCD80 => {
    //   block [0x82DBCD80..0x82DBCDD8)
	// 82DBCD80: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DBCD84: 419A0054  beq cr6, 0x82dbcdd8
	if ctx.cr[6].eq {
	pc = 0x82DBCDD8; continue 'dispatch;
	}
	// 82DBCD88: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DBCD8C: 8943003C  lbz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBCD90: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DBCD94: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBCD98: 39294CA4  addi r9, r9, 0x4ca4
	ctx.r[9].s64 = ctx.r[9].s64 + 19620;
	// 82DBCD9C: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBCDA0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82DBCDA4: 90E50008  stw r7, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DBCDA8: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DBCDAC: B1050014  sth r8, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[8].u16 ) };
	// 82DBCDB0: 99450016  stb r10, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[10].u8 ) };
	// 82DBCDB4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DBCDB8: B0C50006  sth r6, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DBCDBC: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBCDC0: 9085000C  stw r4, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	pc = 0x82DBCDD8; continue 'dispatch;
            }
            0x82DBCDD8 => {
    //   block [0x82DBCDD8..0x82DBCE18)
	// 82DBCDD8: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82DBCDDC: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 82DBCDE0: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 82DBCDE4: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
	// 82DBCDE8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBCE18 size=160
    let mut pc: u32 = 0x82DBCE18;
    'dispatch: loop {
        match pc {
            0x82DBCE18 => {
    //   block [0x82DBCE18..0x82DBCE54)
	// 82DBCE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBCE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBCE20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBCE24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBCE28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBCE2C: 3BE30034  addi r31, r3, 0x34
	ctx.r[31].s64 = ctx.r[3].s64 + 52;
	// 82DBCE30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBCE34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCE38: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCE3C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCE40: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCE44: 409A0010  bne cr6, 0x82dbce54
	if !ctx.cr[6].eq {
	pc = 0x82DBCE54; continue 'dispatch;
	}
	// 82DBCE48: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DBCE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCE50: 4BF9A149  bl 0x82d56f98
	ctx.lr = 0x82DBCE54;
	sub_82D56F98(ctx, base);
	pc = 0x82DBCE54; continue 'dispatch;
            }
            0x82DBCE54 => {
    //   block [0x82DBCE54..0x82DBCEA0)
	// 82DBCE54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCE58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBCE5C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCE60: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 82DBCE64: 1D4B0038  mulli r10, r11, 0x38
	ctx.r[10].s32 = ((ctx.r[11].s32 as i64 * 56 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82DBCE68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBCE6C: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DBCE70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBCE74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DBCE78: 4BEEC609  bl 0x82ca9480
	ctx.lr = 0x82DBCE7C;
	sub_82CA9480(ctx, base);
	// 82DBCE7C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBCE80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBCE84: 409A001C  bne cr6, 0x82dbcea0
	if !ctx.cr[6].eq {
	pc = 0x82DBCEA0; continue 'dispatch;
	}
	// 82DBCE88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DBCE8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBCE90: 396B4C30  addi r11, r11, 0x4c30
	ctx.r[11].s64 = ctx.r[11].s64 + 19504;
	// 82DBCE94: 915E0030  stw r10, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DBCE98: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DBCE9C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	pc = 0x82DBCEA0; continue 'dispatch;
            }
            0x82DBCEA0 => {
    //   block [0x82DBCEA0..0x82DBCEB8)
	// 82DBCEA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBCEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBCEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBCEAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBCEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBCEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBCEB8 size=176
    let mut pc: u32 = 0x82DBCEB8;
    'dispatch: loop {
        match pc {
            0x82DBCEB8 => {
    //   block [0x82DBCEB8..0x82DBCF3C)
	// 82DBCEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBCEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBCEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBCEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBCEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBCECC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBCED0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBCED4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBCED8: 388B29E8  addi r4, r11, 0x29e8
	ctx.r[4].s64 = ctx.r[11].s64 + 10728;
	// 82DBCEDC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBCEE0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCEE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBCEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCEEC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCEF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBCEF4: 4E800421  bctrl
	ctx.lr = 0x82DBCEF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBCEF8: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBCEFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBCF00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBCF04: 409A0038  bne cr6, 0x82dbcf3c
	if !ctx.cr[6].eq {
	pc = 0x82DBCF3C; continue 'dispatch;
	}
	// 82DBCF08: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCF0C: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCF10: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBCF14: 813E0038  lwz r9, 0x38(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCF18: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCF1C: 1D080038  mulli r8, r8, 0x38
	ctx.r[8].s32 = ((ctx.r[8].s32 as i64 * 56 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82DBCF20: 388B2C84  addi r4, r11, 0x2c84
	ctx.r[4].s64 = ctx.r[11].s64 + 11396;
	// 82DBCF24: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCF28: 1CE90038  mulli r7, r9, 0x38
	ctx.r[7].s32 = ((ctx.r[9].s32 as i64 * 56 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82DBCF2C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBCF30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCF34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBCF38: 4E800421  bctrl
	ctx.lr = 0x82DBCF3C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DBCF3C => {
    //   block [0x82DBCF3C..0x82DBCF68)
	// 82DBCF3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCF44: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBCF48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBCF4C: 4E800421  bctrl
	ctx.lr = 0x82DBCF50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBCF50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBCF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBCF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBCF5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBCF60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBCF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBCF68 size=108
    let mut pc: u32 = 0x82DBCF68;
    'dispatch: loop {
        match pc {
            0x82DBCF68 => {
    //   block [0x82DBCF68..0x82DBCFB4)
	// 82DBCF68: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBCF6C: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCF70: 216A0020  subfic r11, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[10].s64;
	// 82DBCF74: 7C8B5C30  srw r11, r4, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[4].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCF78: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 56 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBCF7C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DBCF80: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBCF84: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DBCF88: 419A004C  beq cr6, 0x82dbcfd4
	if ctx.cr[6].eq {
		sub_82DBCFD4(ctx, base);
		return;
	}
	// 82DBCF8C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DBCF90: 80EB0024  lwz r7, 0x24(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBCF94: 88CB0011  lbz r6, 0x11(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DBCF98: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DBCF9C: 7D0A5430  srw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCFA0: 7D4A2038  and r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 82DBCFA4: 7D4A39D6  mullw r10, r10, r7
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82DBCFA8: 409A000C  bne cr6, 0x82dbcfb4
	if !ctx.cr[6].eq {
	pc = 0x82DBCFB4; continue 'dispatch;
	}
	// 82DBCFAC: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DBCFB0: 48000008  b 0x82dbcfb8
	pc = 0x82DBCFB8; continue 'dispatch;
            }
            0x82DBCFB4 => {
    //   block [0x82DBCFB4..0x82DBCFB8)
	// 82DBCFB4: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	pc = 0x82DBCFB8; continue 'dispatch;
            }
            0x82DBCFB8 => {
    //   block [0x82DBCFB8..0x82DBCFD4)
	// 82DBCFB8: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82DBCFBC: 419A0018  beq cr6, 0x82dbcfd4
	if ctx.cr[6].eq {
		sub_82DBCFD4(ctx, base);
		return;
	}
	// 82DBCFC0: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBCFC4: 812B0028  lwz r9, 0x28(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBCFC8: 7D6851D6  mullw r11, r8, r10
	ctx.r[11].s32 = ((ctx.r[8].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBCFCC: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DBCFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCFD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBCFD4 size=8
    let mut pc: u32 = 0x82DBCFD4;
    'dispatch: loop {
        match pc {
            0x82DBCFD4 => {
    //   block [0x82DBCFD4..0x82DBCFDC)
	// 82DBCFD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBCFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBCFE0 size=4
    let mut pc: u32 = 0x82DBCFE0;
    'dispatch: loop {
        match pc {
            0x82DBCFE0 => {
    //   block [0x82DBCFE0..0x82DBCFE4)
	// 82DBCFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBCFE8 size=84
    let mut pc: u32 = 0x82DBCFE8;
    'dispatch: loop {
        match pc {
            0x82DBCFE8 => {
    //   block [0x82DBCFE8..0x82DBD03C)
	// 82DBCFE8: 7C6B1E70  srawi r11, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 82DBCFEC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82DBCFF0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBCFF4: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82DBCFF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBCFFC: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBD000: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DBD004: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DBD008: C00B0BF8  lfs f0, 0xbf8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD00C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBD010: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DBD014: C00B2CA0  lfs f0, 0x2ca0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11424 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD018: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DBD01C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBD020: D00BB388  stfs f0, -0x4c78(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19576 as u32), tmp.u32 ) };
	// 82DBD024: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD028: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD02C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DBD030: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DBD034: D00BB38C  stfs f0, -0x4c74(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19572 as u32), tmp.u32 ) };
	// 82DBD038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD040 size=8
    let mut pc: u32 = 0x82DBD040;
    'dispatch: loop {
        match pc {
            0x82DBD040 => {
    //   block [0x82DBD040..0x82DBD048)
	// 82DBD040: C0230014  lfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBD044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD048 size=8
    let mut pc: u32 = 0x82DBD048;
    'dispatch: loop {
        match pc {
            0x82DBD048 => {
    //   block [0x82DBD048..0x82DBD050)
	// 82DBD048: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 82DBD04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBD050 size=96
    let mut pc: u32 = 0x82DBD050;
    'dispatch: loop {
        match pc {
            0x82DBD050 => {
    //   block [0x82DBD050..0x82DBD0B0)
	// 82DBD050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBD058: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBD05C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBD060: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBD064: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBD068: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DBD06C: 388B2CA4  addi r4, r11, 0x2ca4
	ctx.r[4].s64 = ctx.r[11].s64 + 11428;
	// 82DBD070: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBD074: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBD078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBD07C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBD080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBD084: 4E800421  bctrl
	ctx.lr = 0x82DBD088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBD088: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBD08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBD090: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBD094: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBD098: 4E800421  bctrl
	ctx.lr = 0x82DBD09C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBD09C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBD0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBD0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBD0A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBD0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD0B0 size=72
    let mut pc: u32 = 0x82DBD0B0;
    'dispatch: loop {
        match pc {
            0x82DBD0B0 => {
    //   block [0x82DBD0B0..0x82DBD0C8)
	// 82DBD0B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD0B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBD0B8: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBD0BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD0C0: C1AA0C80  lfs f13, 0xc80(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3200 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD0C4: C00B0BE8  lfs f0, 0xbe8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82DBD0C8; continue 'dispatch;
            }
            0x82DBD0C8 => {
    //   block [0x82DBD0C8..0x82DBD0F8)
	// 82DBD0C8: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DBD0CC: FD80081E  fctiwz f12, f1
	ctx.f[12].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 82DBD0D0: 7D805FAE  stfiwx f12, 0, r11
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82DBD0D4: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82DBD0D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBD0DC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82DBD0E0: EC21002A  fadds f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DBD0E4: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82DBD0E8: 4198FFE0  blt cr6, 0x82dbd0c8
	if ctx.cr[6].lt {
	pc = 0x82DBD0C8; continue 'dispatch;
	}
	// 82DBD0EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD0F0: C02B0C14  lfs f1, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBD0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD0F8 size=292
    let mut pc: u32 = 0x82DBD0F8;
    'dispatch: loop {
        match pc {
            0x82DBD0F8 => {
    //   block [0x82DBD0F8..0x82DBD21C)
	// 82DBD0F8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DBD0FC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82DBD100: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82DBD104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DBD108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD220 size=32
    let mut pc: u32 = 0x82DBD220;
    'dispatch: loop {
        match pc {
            0x82DBD220 => {
    //   block [0x82DBD220..0x82DBD240)
	// 82DBD220: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD224: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DBD228: D0230014  stfs f1, 0x14(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DBD22C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBD230: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD234: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DBD238: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82DBD23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD240 size=568
    let mut pc: u32 = 0x82DBD240;
    'dispatch: loop {
        match pc {
            0x82DBD240 => {
    //   block [0x82DBD240..0x82DBD478)
	// 82DBD240: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD478 size=276
    let mut pc: u32 = 0x82DBD478;
    'dispatch: loop {
        match pc {
            0x82DBD478 => {
    //   block [0x82DBD478..0x82DBD4F8)
	// 82DBD478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD47C: 4BEEBF89  bl 0x82ca9404
	ctx.lr = 0x82DBD480;
	sub_82CA93D0(ctx, base);
	// 82DBD480: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 82DBD484: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBD488: 41980100  blt cr6, 0x82dbd588
	if ctx.cr[6].lt {
	pc = 0x82DBD588; continue 'dispatch;
	}
	// 82DBD48C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DBD490: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD494: 38A30014  addi r5, r3, 0x14
	ctx.r[5].s64 = ctx.r[3].s64 + 20;
	// 82DBD498: 3BE30050  addi r31, r3, 0x50
	ctx.r[31].s64 = ctx.r[3].s64 + 80;
	// 82DBD49C: 3BC30040  addi r30, r3, 0x40
	ctx.r[30].s64 = ctx.r[3].s64 + 64;
	// 82DBD4A0: C1490C14  lfs f10, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DBD4A4: 3FA08330  lis r29, -0x7cd0
	ctx.r[29].s64 = -2094006272;
	// 82DBD4A8: C16B0BFC  lfs f11, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DBD4AC: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBD4B0: 557C073E  clrlwi r28, r11, 0x1c
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DBD4B4: 557B06F6  rlwinm r27, r11, 0, 0x1b, 0x1b
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBD4B8: 5569CFFE  rlwinm r9, r11, 0x19, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82DBD4BC: 5568D7FE  rlwinm r8, r11, 0x1a, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82DBD4C0: 5567DFFE  rlwinm r7, r11, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBD4C4: FB81FFC8  std r28, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[28].u64 ) };
	// 82DBD4C8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DBD4CC: C801FFC8  lfd f0, -0x38(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DBD4D0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DBD4D4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DBD4D8: EDA0582A  fadds f13, f0, f11
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64;
	// 82DBD4DC: C01DB38C  lfs f0, -0x4c74(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD4E0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBD4E4: EDA0503C  fnmsubs f13, f0, f0, f10
	ctx.f[13].f64 = -(((ctx.f[0].f64 * ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82DBD4E8: EDA0682C  fsqrts f13, f13
	ctx.f[13].f64 = ((ctx.f[13].f64).sqrt() as f32) as f64;
	// 82DBD4EC: 419A000C  beq cr6, 0x82dbd4f8
	if ctx.cr[6].eq {
	pc = 0x82DBD4F8; continue 'dispatch;
	}
	// 82DBD4F0: FD806890  fmr f12, f13
	ctx.f[12].f64 = ctx.f[13].f64;
	// 82DBD4F4: 4800000C  b 0x82dbd500
	pc = 0x82DBD500; continue 'dispatch;
            }
            0x82DBD4F8 => {
    //   block [0x82DBD4F8..0x82DBD500)
	// 82DBD4F8: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 82DBD4FC: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	pc = 0x82DBD500; continue 'dispatch;
            }
            0x82DBD500 => {
    //   block [0x82DBD500..0x82DBD518)
	// 82DBD500: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82DBD504: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DBD508: D181FFC4  stfs f12, -0x3c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82DBD50C: 409A000C  bne cr6, 0x82dbd518
	if !ctx.cr[6].eq {
	pc = 0x82DBD518; continue 'dispatch;
	}
	// 82DBD510: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DBD514: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	pc = 0x82DBD518; continue 'dispatch;
            }
            0x82DBD518 => {
    //   block [0x82DBD518..0x82DBD528)
	// 82DBD518: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DBD51C: 409A000C  bne cr6, 0x82dbd528
	if !ctx.cr[6].eq {
	pc = 0x82DBD528; continue 'dispatch;
	}
	// 82DBD520: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DBD524: D001FFC4  stfs f0, -0x3c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	pc = 0x82DBD528; continue 'dispatch;
            }
            0x82DBD528 => {
    //   block [0x82DBD528..0x82DBD588)
	// 82DBD528: 3901FFC0  addi r8, r1, -0x40
	ctx.r[8].s64 = ctx.r[1].s64 + -64;
	pc = 0x82DBD588; continue 'dispatch;
            }
            0x82DBD588 => {
    //   block [0x82DBD588..0x82DBD58C)
	// 82DBD588: 4BEEBECC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD590 size=56
    let mut pc: u32 = 0x82DBD590;
    'dispatch: loop {
        match pc {
            0x82DBD590 => {
    //   block [0x82DBD590..0x82DBD5C8)
	// 82DBD590: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD594: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DBD598: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82DBD59C: C00B0BFC  lfs f0, 0xbfc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD5A0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD5C8 size=16
    let mut pc: u32 = 0x82DBD5C8;
    'dispatch: loop {
        match pc {
            0x82DBD5C8 => {
    //   block [0x82DBD5C8..0x82DBD5D8)
	// 82DBD5C8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD5D8 size=592
    let mut pc: u32 = 0x82DBD5D8;
    'dispatch: loop {
        match pc {
            0x82DBD5D8 => {
    //   block [0x82DBD5D8..0x82DBD828)
	// 82DBD5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD5DC: 4BEEBE19  bl 0x82ca93f4
	ctx.lr = 0x82DBD5E0;
	sub_82CA93D0(ctx, base);
	// 82DBD5E0: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82DBD5E4: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD5E8: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
	// 82DBD5EC: ED400032  fmuls f10, f0, f0
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBD5F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD5F4: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 82DBD5F8: D181FF90  stfs f12, -0x70(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBD828 size=184
    let mut pc: u32 = 0x82DBD828;
    'dispatch: loop {
        match pc {
            0x82DBD828 => {
    //   block [0x82DBD828..0x82DBD88C)
	// 82DBD828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBD830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBD834: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD838: D0430010  stfs f2, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBD83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBD840: FD600890  fmr f11, f1
	ctx.f[11].f64 = ctx.f[1].f64;
	// 82DBD844: 396B4C14  addi r11, r11, 0x4c14
	ctx.r[11].s64 = ctx.r[11].s64 + 19476;
	// 82DBD848: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBD84C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82DBD850: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DBD854: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 82DBD858: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBD85C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD860: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DBD864: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DBD868: C009B390  lfs f0, -0x4c70(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD86C: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD870: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBD874: 40980018  bge cr6, 0x82dbd88c
	if !ctx.cr[6].lt {
	pc = 0x82DBD88C; continue 'dispatch;
	}
	// 82DBD878: 4BFFF839  bl 0x82dbd0b0
	ctx.lr = 0x82DBD87C;
	sub_82DBD0B0(ctx, base);
	// 82DBD87C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD880: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD884: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DBD888: D009B390  stfs f0, -0x4c70(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-19568 as u32), tmp.u32 ) };
	pc = 0x82DBD88C; continue 'dispatch;
            }
            0x82DBD88C => {
    //   block [0x82DBD88C..0x82DBD8E0)
	// 82DBD88C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBD8E0 size=112
    let mut pc: u32 = 0x82DBD8E0;
    'dispatch: loop {
        match pc {
            0x82DBD8E0 => {
    //   block [0x82DBD8E0..0x82DBD938)
	// 82DBD8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBD8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBD8EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD8F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBD8F4: 396B4C14  addi r11, r11, 0x4c14
	ctx.r[11].s64 = ctx.r[11].s64 + 19476;
	// 82DBD8F8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DBD8FC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DBD900: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DBD904: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBD908: 419A0030  beq cr6, 0x82dbd938
	if ctx.cr[6].eq {
	pc = 0x82DBD938; continue 'dispatch;
	}
	// 82DBD90C: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 82DBD910: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD914: C009B390  lfs f0, -0x4c70(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD918: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD91C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBD920: 40980018  bge cr6, 0x82dbd938
	if !ctx.cr[6].lt {
	pc = 0x82DBD938; continue 'dispatch;
	}
	// 82DBD924: 4BFFF78D  bl 0x82dbd0b0
	ctx.lr = 0x82DBD928;
	sub_82DBD0B0(ctx, base);
	// 82DBD928: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD92C: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD930: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DBD934: D009B390  stfs f0, -0x4c70(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-19568 as u32), tmp.u32 ) };
	pc = 0x82DBD938; continue 'dispatch;
            }
            0x82DBD938 => {
    //   block [0x82DBD938..0x82DBD950)
	// 82DBD938: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82DBD93C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBD940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBD944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBD948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBD94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD950 size=436
    let mut pc: u32 = 0x82DBD950;
    'dispatch: loop {
        match pc {
            0x82DBD950 => {
    //   block [0x82DBD950..0x82DBDB04)
	// 82DBD950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD954: 4BEEBAA5  bl 0x82ca93f8
	ctx.lr = 0x82DBD958;
	sub_82CA93D0(ctx, base);
	// 82DBD958: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82DBD95C: EB640000  ld r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DBD960: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82DBD964: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 82DBD968: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DBD96C: 3901FF60  addi r8, r1, -0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + -160;
	// 82DBD970: 38E1FF70  addi r7, r1, -0x90
	ctx.r[7].s64 = ctx.r[1].s64 + -144;
	// 82DBD974: EB4B0000  ld r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBD978: 38A1FF80  addi r5, r1, -0x80
	ctx.r[5].s64 = ctx.r[1].s64 + -128;
	// 82DBD97C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DBD980: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82DBD984: EB2A0000  ld r25, 0(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DBD988: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DBD98C: FB680000  std r27, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DBD990: 3B81FFA0  addi r28, r1, -0x60
	ctx.r[28].s64 = ctx.r[1].s64 + -96;
	// 82DBD994: F8880008  std r4, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 82DBD998: 3BA30030  addi r29, r3, 0x30
	ctx.r[29].s64 = ctx.r[3].s64 + 48;
	// 82DBD99C: FB470000  std r26, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82DBD9A0: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DBD9A4: 3961FF60  addi r11, r1, -0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + -160;
	// 82DBD9A8: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82DBD9AC: FB250000  std r25, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82DBD9B0: EB090000  ld r24, 0(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DBD9B4: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBDB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBDB08 size=1420
    let mut pc: u32 = 0x82DBDB08;
    'dispatch: loop {
        match pc {
            0x82DBDB08 => {
    //   block [0x82DBDB08..0x82DBDB68)
	// 82DBDB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBDB0C: 4BEEB8F9  bl 0x82ca9404
	ctx.lr = 0x82DBDB10;
	sub_82CA93D0(ctx, base);
	// 82DBDB10: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82DBDB14: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DBDB18: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBDB1C: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBDB20: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DBDB24: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DBDB28: 7FAA4A14  add r29, r10, r9
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DBDB2C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DBDB30: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBDB34: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBDB38: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBDB3C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBDB40: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBDB44: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBDB48: 40980020  bge cr6, 0x82dbdb68
	if !ctx.cr[6].lt {
	pc = 0x82DBDB68; continue 'dispatch;
	}
	// 82DBDB4C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBDB50: 39082CB0  addi r8, r8, 0x2cb0
	ctx.r[8].s64 = ctx.r[8].s64 + 11440;
	// 82DBDB54: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBDB58: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBDB5C: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82DBDB60: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBDB64: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82DBDB68; continue 'dispatch;
            }
            0x82DBDB68 => {
    //   block [0x82DBDB68..0x82DBE094)
	// 82DBDB68: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DBDB6C: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBDB70: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82DBDB74: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBDB78: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DBDB7C: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DBDB80: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82DBDB84: 3B800030  li r28, 0x30
	ctx.r[28].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE098 size=8
    let mut pc: u32 = 0x82DBE098;
    'dispatch: loop {
        match pc {
            0x82DBE098 => {
    //   block [0x82DBE098..0x82DBE0A0)
	// 82DBE098: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82DBE09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE0A0 size=96
    let mut pc: u32 = 0x82DBE0A0;
    'dispatch: loop {
        match pc {
            0x82DBE0A0 => {
    //   block [0x82DBE0A0..0x82DBE100)
	// 82DBE0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE0A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE0AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE0B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBE0B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBE0B8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DBE0BC: 388B2D50  addi r4, r11, 0x2d50
	ctx.r[4].s64 = ctx.r[11].s64 + 11600;
	// 82DBE0C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBE0C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE0C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE0CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE0D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE0D4: 4E800421  bctrl
	ctx.lr = 0x82DBE0D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE0D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE0E0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE0E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE0E8: 4E800421  bctrl
	ctx.lr = 0x82DBE0EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE0EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBE0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBE0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBE0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBE0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE100 size=8
    let mut pc: u32 = 0x82DBE100;
    'dispatch: loop {
        match pc {
            0x82DBE100 => {
    //   block [0x82DBE100..0x82DBE108)
	// 82DBE100: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82DBE104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE108 size=16
    let mut pc: u32 = 0x82DBE108;
    'dispatch: loop {
        match pc {
            0x82DBE108 => {
    //   block [0x82DBE108..0x82DBE118)
	// 82DBE108: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DBE10C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DBE110: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBE114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE118 size=56
    let mut pc: u32 = 0x82DBE118;
    'dispatch: loop {
        match pc {
            0x82DBE118 => {
    //   block [0x82DBE118..0x82DBE150)
	// 82DBE118: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE150 size=8
    let mut pc: u32 = 0x82DBE150;
    'dispatch: loop {
        match pc {
            0x82DBE150 => {
    //   block [0x82DBE150..0x82DBE158)
	// 82DBE150: D003002C  stfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBE154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE158 size=16
    let mut pc: u32 = 0x82DBE158;
    'dispatch: loop {
        match pc {
            0x82DBE158 => {
    //   block [0x82DBE158..0x82DBE168)
	// 82DBE158: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE168 size=156
    let mut pc: u32 = 0x82DBE168;
    'dispatch: loop {
        match pc {
            0x82DBE168 => {
    //   block [0x82DBE168..0x82DBE204)
	// 82DBE168: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE208 size=204
    let mut pc: u32 = 0x82DBE208;
    'dispatch: loop {
        match pc {
            0x82DBE208 => {
    //   block [0x82DBE208..0x82DBE2D4)
	// 82DBE208: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE2D8 size=16
    let mut pc: u32 = 0x82DBE2D8;
    'dispatch: loop {
        match pc {
            0x82DBE2D8 => {
    //   block [0x82DBE2D8..0x82DBE2E8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE2E8 size=80
    let mut pc: u32 = 0x82DBE2E8;
    'dispatch: loop {
        match pc {
            0x82DBE2E8 => {
    //   block [0x82DBE2E8..0x82DBE338)
	// 82DBE2E8: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82DBE2EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBE2F0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82DBE2F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBE2F8: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 82DBE2FC: 392A2CD0  addi r9, r10, 0x2cd0
	ctx.r[9].s64 = ctx.r[10].s64 + 11472;
	// 82DBE300: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE338 size=220
    let mut pc: u32 = 0x82DBE338;
    'dispatch: loop {
        match pc {
            0x82DBE338 => {
    //   block [0x82DBE338..0x82DBE414)
	// 82DBE338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE33C: 4BEEB0C9  bl 0x82ca9404
	ctx.lr = 0x82DBE340;
	sub_82CA93D0(ctx, base);
	// 82DBE340: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBE344: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DBE348: 396B2CD0  addi r11, r11, 0x2cd0
	ctx.r[11].s64 = ctx.r[11].s64 + 11472;
	// 82DBE34C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DBE350: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 82DBE354: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DBE358: C00A0010  lfs f0, 0x10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBE35C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE418 size=120
    let mut pc: u32 = 0x82DBE418;
    'dispatch: loop {
        match pc {
            0x82DBE418 => {
    //   block [0x82DBE418..0x82DBE474)
	// 82DBE418: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBE41C: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBE420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBE424: 396B4D6C  addi r11, r11, 0x4d6c
	ctx.r[11].s64 = ctx.r[11].s64 + 19820;
	// 82DBE428: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBE42C: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82DBE430: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DBE434: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBE438: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DBE43C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DBE440: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE444: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DBE448: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE44C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82DBE450: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBE454: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DBE458: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE45C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82DBE460: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBE464: C1A30024  lfs f13, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBE468: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBE46C: 41980008  blt cr6, 0x82dbe474
	if ctx.cr[6].lt {
	pc = 0x82DBE474; continue 'dispatch;
	}
	// 82DBE470: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	pc = 0x82DBE474; continue 'dispatch;
            }
            0x82DBE474 => {
    //   block [0x82DBE474..0x82DBE488)
	// 82DBE474: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBE478: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBE47C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DBE480: 40980008  bge cr6, 0x82dbe488
	if !ctx.cr[6].lt {
	pc = 0x82DBE488; continue 'dispatch;
	}
	// 82DBE484: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	pc = 0x82DBE488; continue 'dispatch;
            }
            0x82DBE488 => {
    //   block [0x82DBE488..0x82DBE490)
	// 82DBE488: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBE48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE490 size=908
    let mut pc: u32 = 0x82DBE490;
    'dispatch: loop {
        match pc {
            0x82DBE490 => {
    //   block [0x82DBE490..0x82DBE4D0)
	// 82DBE490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE494: 4BEEAF71  bl 0x82ca9404
	ctx.lr = 0x82DBE498;
	sub_82CA93D0(ctx, base);
	// 82DBE498: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE49C: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DBE4A0: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DBE4A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE4A8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE4AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBE4B0: 40980020  bge cr6, 0x82dbe4d0
	if !ctx.cr[6].lt {
	pc = 0x82DBE4D0; continue 'dispatch;
	}
	// 82DBE4B4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBE4B8: 39292D5C  addi r9, r9, 0x2d5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11612;
	// 82DBE4BC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBE4C0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBE4C4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBE4C8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBE4CC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBE4D0; continue 'dispatch;
            }
            0x82DBE4D0 => {
    //   block [0x82DBE4D0..0x82DBE81C)
	// 82DBE4D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBE4D4: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBE4D8: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 82DBE4DC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DBE4E0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE820 size=144
    let mut pc: u32 = 0x82DBE820;
    'dispatch: loop {
        match pc {
            0x82DBE820 => {
    //   block [0x82DBE820..0x82DBE888)
	// 82DBE820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBE82C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBE838: 3BC5FFD0  addi r30, r5, -0x30
	ctx.r[30].s64 = ctx.r[5].s64 + -48;
	// 82DBE83C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DBE840: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE848: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE850: 4E800421  bctrl
	ctx.lr = 0x82DBE854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE854: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBE858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBE85C: 41980038  blt cr6, 0x82dbe894
	if ctx.cr[6].lt {
	pc = 0x82DBE894; continue 'dispatch;
	}
	// 82DBE860: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DBE864: 41990030  bgt cr6, 0x82dbe894
	if ctx.cr[6].gt {
	pc = 0x82DBE894; continue 'dispatch;
	}
	// 82DBE868: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE86C: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82DBE870: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBE874: 409A0014  bne cr6, 0x82dbe888
	if !ctx.cr[6].eq {
	pc = 0x82DBE888; continue 'dispatch;
	}
	// 82DBE878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBE87C: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82DBE880: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82DBE884: 48000014  b 0x82dbe898
	pc = 0x82DBE898; continue 'dispatch;
            }
            0x82DBE888 => {
    //   block [0x82DBE888..0x82DBE894)
	// 82DBE888: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82DBE88C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DBE890: 48000008  b 0x82dbe898
	pc = 0x82DBE898; continue 'dispatch;
            }
            0x82DBE894 => {
    //   block [0x82DBE894..0x82DBE898)
	// 82DBE894: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x82DBE898; continue 'dispatch;
            }
            0x82DBE898 => {
    //   block [0x82DBE898..0x82DBE8B0)
	// 82DBE898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBE89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBE8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBE8A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBE8A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBE8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE8B0 size=20
    let mut pc: u32 = 0x82DBE8B0;
    'dispatch: loop {
        match pc {
            0x82DBE8B0 => {
    //   block [0x82DBE8B0..0x82DBE8C4)
	// 82DBE8B0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE8B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE8B8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBE8BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE8C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE8C8 size=144
    let mut pc: u32 = 0x82DBE8C8;
    'dispatch: loop {
        match pc {
            0x82DBE8C8 => {
    //   block [0x82DBE8C8..0x82DBE958)
	// 82DBE8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBE8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE8DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBE8E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBE8E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBE8E8: 388B2D64  addi r4, r11, 0x2d64
	ctx.r[4].s64 = ctx.r[11].s64 + 11620;
	// 82DBE8EC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBE8F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE8F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBE8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE8FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE904: 4E800421  bctrl
	ctx.lr = 0x82DBE908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE908: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE90C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DBE910: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE914: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBE918: 388BB3B4  addi r4, r11, -0x4c4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19532;
	// 82DBE91C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE920: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE928: 4E800421  bctrl
	ctx.lr = 0x82DBE92C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE92C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE934: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE93C: 4E800421  bctrl
	ctx.lr = 0x82DBE940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBE944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBE948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBE94C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBE950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBE954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE958 size=8
    let mut pc: u32 = 0x82DBE958;
    'dispatch: loop {
        match pc {
            0x82DBE958 => {
    //   block [0x82DBE958..0x82DBE960)
	// 82DBE958: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DBE95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE960 size=104
    let mut pc: u32 = 0x82DBE960;
    'dispatch: loop {
        match pc {
            0x82DBE960 => {
    //   block [0x82DBE960..0x82DBE9C8)
	// 82DBE960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBE96C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBE978: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBE97C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE980: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE984: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE98C: 4E800421  bctrl
	ctx.lr = 0x82DBE990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE990: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE9C8 size=164
    let mut pc: u32 = 0x82DBE9C8;
    'dispatch: loop {
        match pc {
            0x82DBE9C8 => {
    //   block [0x82DBE9C8..0x82DBE9E4)
	// 82DBE9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE9D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE9D4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DBE9D8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DBE9DC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DBE9E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DBE9E4; continue 'dispatch;
            }
            0x82DBE9E4 => {
    //   block [0x82DBE9E4..0x82DBEA6C)
	// 82DBE9E4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBE9E8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBE9EC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DBE9F0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DBE9F4: 4200FFF0  bdnz 0x82dbe9e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBE9E4; continue 'dispatch;
	}
	// 82DBE9F8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEA70 size=92
    let mut pc: u32 = 0x82DBEA70;
    'dispatch: loop {
        match pc {
            0x82DBEA70 => {
    //   block [0x82DBEA70..0x82DBEACC)
	// 82DBEA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEA78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEA7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBEA80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEA84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEA88: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DBEA8C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEA90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEA94: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBEA98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEA9C: 4E800421  bctrl
	ctx.lr = 0x82DBEAA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEAA0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEAD0 size=104
    let mut pc: u32 = 0x82DBEAD0;
    'dispatch: loop {
        match pc {
            0x82DBEAD0 => {
    //   block [0x82DBEAD0..0x82DBEB30)
	// 82DBEAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEAD4: 4BEEA939  bl 0x82ca940c
	ctx.lr = 0x82DBEAD8;
	sub_82CA93D0(ctx, base);
	// 82DBEAD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEAE0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DBEAE4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DBEAE8: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEAEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEAF0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBEAF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEAF8: 4E800421  bctrl
	ctx.lr = 0x82DBEAFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEAFC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DBEB00: 40990030  ble cr6, 0x82dbeb30
	if !ctx.cr[6].gt {
	pc = 0x82DBEB30; continue 'dispatch;
	}
	// 82DBEB04: 393E0020  addi r9, r30, 0x20
	ctx.r[9].s64 = ctx.r[30].s64 + 32;
	// 82DBEB08: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DBEB0C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
            }
            0x82DBEB30 => {
    //   block [0x82DBEB30..0x82DBEB38)
	// 82DBEB30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBEB34: 4BEEA928  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEB38 size=92
    let mut pc: u32 = 0x82DBEB38;
    'dispatch: loop {
        match pc {
            0x82DBEB38 => {
    //   block [0x82DBEB38..0x82DBEB94)
	// 82DBEB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEB40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEB44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBEB48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEB4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEB50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBEB54: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEB58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEB5C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBEB60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEB64: 4E800421  bctrl
	ctx.lr = 0x82DBEB68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEB68: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEB98 size=92
    let mut pc: u32 = 0x82DBEB98;
    'dispatch: loop {
        match pc {
            0x82DBEB98 => {
    //   block [0x82DBEB98..0x82DBEBF4)
	// 82DBEB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEBA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEBA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBEBA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEBAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEBB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBEBB4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEBB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEBBC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBEBC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEBC4: 4E800421  bctrl
	ctx.lr = 0x82DBEBC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEBC8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEBF8 size=136
    let mut pc: u32 = 0x82DBEBF8;
    'dispatch: loop {
        match pc {
            0x82DBEBF8 => {
    //   block [0x82DBEBF8..0x82DBEC74)
	// 82DBEBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEBFC: 4BEEA80D  bl 0x82ca9408
	ctx.lr = 0x82DBEC00;
	sub_82CA93D0(ctx, base);
	// 82DBEC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEC04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBEC08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DBEC0C: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEC10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEC14: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBEC18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEC1C: 4E800421  bctrl
	ctx.lr = 0x82DBEC20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEC20: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEC24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBEC28: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DBEC2C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82DBEC30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEC34: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBEC38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEC3C: 4E800421  bctrl
	ctx.lr = 0x82DBEC40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEC40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBEC44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBEC48: 4099002C  ble cr6, 0x82dbec74
	if !ctx.cr[6].gt {
	pc = 0x82DBEC74; continue 'dispatch;
	}
	// 82DBEC4C: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
            }
            0x82DBEC74 => {
    //   block [0x82DBEC74..0x82DBEC80)
	// 82DBEC74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DBEC78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBEC7C: 4BEEA7DC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBEC80 size=116
    let mut pc: u32 = 0x82DBEC80;
    'dispatch: loop {
        match pc {
            0x82DBEC80 => {
    //   block [0x82DBEC80..0x82DBECD4)
	// 82DBEC80: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBEC84: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBEC88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBEC8C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBEC90: 396B191C  addi r11, r11, 0x191c
	ctx.r[11].s64 = ctx.r[11].s64 + 6428;
	// 82DBEC94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DBEC98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBEC9C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82DBECA0: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DBECA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBECA8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DBECAC: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DBECB0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBECB4: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBECB8: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82DBECBC: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBECC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBECC4: 419A0010  beq cr6, 0x82dbecd4
	if ctx.cr[6].eq {
	pc = 0x82DBECD4; continue 'dispatch;
	}
	// 82DBECC8: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBECCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBECD0: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82DBECD4; continue 'dispatch;
            }
            0x82DBECD4 => {
    //   block [0x82DBECD4..0x82DBECF4)
	// 82DBECD4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBECF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBECF8 size=132
    let mut pc: u32 = 0x82DBECF8;
    'dispatch: loop {
        match pc {
            0x82DBECF8 => {
    //   block [0x82DBECF8..0x82DBED7C)
	// 82DBECF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBECFC: 4BEEA711  bl 0x82ca940c
	ctx.lr = 0x82DBED00;
	sub_82CA93D0(ctx, base);
	// 82DBED00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBED04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBED08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBED0C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBED10: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBED14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBED18: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBED1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBED20: 4E800421  bctrl
	ctx.lr = 0x82DBED24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBED24: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBED80 size=304
    let mut pc: u32 = 0x82DBED80;
    'dispatch: loop {
        match pc {
            0x82DBED80 => {
    //   block [0x82DBED80..0x82DBEDD0)
	// 82DBED80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBED84: 4BEEA685  bl 0x82ca9408
	ctx.lr = 0x82DBED88;
	sub_82CA93D0(ctx, base);
	// 82DBED88: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBED8C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBED90: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DBED94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBED98: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82DBED9C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBEDA0: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DBEDA4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEDA8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBEDAC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBEDB0: 40980020  bge cr6, 0x82dbedd0
	if !ctx.cr[6].lt {
	pc = 0x82DBEDD0; continue 'dispatch;
	}
	// 82DBEDB4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBEDB8: 39082C40  addi r8, r8, 0x2c40
	ctx.r[8].s64 = ctx.r[8].s64 + 11328;
	// 82DBEDBC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBEDC0: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBEDC4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DBEDC8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBEDCC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82DBEDD0; continue 'dispatch;
            }
            0x82DBEDD0 => {
    //   block [0x82DBEDD0..0x82DBEDE0)
	// 82DBEDD0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBEDD4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82DBEDD8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82DBEDDC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	pc = 0x82DBEDE0; continue 'dispatch;
            }
            0x82DBEDE0 => {
    //   block [0x82DBEDE0..0x82DBEEB0)
	// 82DBEDE0: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBEDE4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBEDE8: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DBEDEC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DBEDF0: 4200FFF0  bdnz 0x82dbede0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBEDE0; continue 'dispatch;
	}
	// 82DBEDF4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DBEDF8: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBEEB0 size=320
    let mut pc: u32 = 0x82DBEEB0;
    'dispatch: loop {
        match pc {
            0x82DBEEB0 => {
    //   block [0x82DBEEB0..0x82DBEFE4)
	// 82DBEEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEEB4: 4BEEA549  bl 0x82ca93fc
	ctx.lr = 0x82DBEEB8;
	sub_82CA93D0(ctx, base);
	// 82DBEEB8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEEBC: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DBEEC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBEEC4: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82DBEEC8: 7FC92214  add r30, r9, r4
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82DBEECC: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DBEED0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBEED4: 7D6829D6  mullw r11, r8, r5
	ctx.r[11].s32 = ((ctx.r[8].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBEED8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEEDC: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBEEE0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DBEEE4: 419A0100  beq cr6, 0x82dbefe4
	if ctx.cr[6].eq {
	pc = 0x82DBEFE4; continue 'dispatch;
	}
	// 82DBEEE8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBEEEC: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBEEF0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEEF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBEEF8: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82DBEEFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBEF00: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBEF04: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEF08: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBEF0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DBEF10: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEF14: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DBEF18: 4E800421  bctrl
	ctx.lr = 0x82DBEF1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEF1C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEF20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBEF24: 419A00C0  beq cr6, 0x82dbefe4
	if ctx.cr[6].eq {
	pc = 0x82DBEFE4; continue 'dispatch;
	}
	// 82DBEF28: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEF2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBEF30: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBEF34: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82DBEF38: 394A4C30  addi r10, r10, 0x4c30
	ctx.r[10].s64 = ctx.r[10].s64 + 19504;
	// 82DBEF3C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DBEF40: 38EB0020  addi r7, r11, 0x20
	ctx.r[7].s64 = ctx.r[11].s64 + 32;
            }
            0x82DBEFE4 => {
    //   block [0x82DBEFE4..0x82DBEFF0)
	// 82DBEFE4: C03B0004  lfs f1, 4(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBEFE8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DBEFEC: 4BEEA460  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEFF0 size=300
    let mut pc: u32 = 0x82DBEFF0;
    'dispatch: loop {
        match pc {
            0x82DBEFF0 => {
    //   block [0x82DBEFF0..0x82DBF044)
	// 82DBEFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEFF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEFFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBF000: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF004: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF008: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DBF00C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DBF010: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBF014: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBF018: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF01C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF020: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBF024: 40980020  bge cr6, 0x82dbf044
	if !ctx.cr[6].lt {
	pc = 0x82DBF044; continue 'dispatch;
	}
	// 82DBF028: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBF02C: 39082D74  addi r8, r8, 0x2d74
	ctx.r[8].s64 = ctx.r[8].s64 + 11636;
	// 82DBF030: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBF034: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBF038: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82DBF03C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBF040: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	pc = 0x82DBF044; continue 'dispatch;
            }
            0x82DBF044 => {
    //   block [0x82DBF044..0x82DBF064)
	// 82DBF044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF048: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBF04C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF050: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBF054: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82DBF058: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF05C: 409A0008  bne cr6, 0x82dbf064
	if !ctx.cr[6].eq {
	pc = 0x82DBF064; continue 'dispatch;
	}
	// 82DBF060: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x82DBF064; continue 'dispatch;
            }
            0x82DBF064 => {
    //   block [0x82DBF064..0x82DBF084)
	// 82DBF064: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBF068: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF06C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF070: 419A001C  beq cr6, 0x82dbf08c
	if ctx.cr[6].eq {
	pc = 0x82DBF08C; continue 'dispatch;
	}
	// 82DBF074: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF078: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DBF07C: 409A0008  bne cr6, 0x82dbf084
	if !ctx.cr[6].eq {
	pc = 0x82DBF084; continue 'dispatch;
	}
	// 82DBF080: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x82DBF084; continue 'dispatch;
            }
            0x82DBF084 => {
    //   block [0x82DBF084..0x82DBF08C)
	// 82DBF084: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82DBF088: 48000008  b 0x82dbf090
	pc = 0x82DBF090; continue 'dispatch;
            }
            0x82DBF08C => {
    //   block [0x82DBF08C..0x82DBF090)
	// 82DBF08C: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	pc = 0x82DBF090; continue 'dispatch;
            }
            0x82DBF090 => {
    //   block [0x82DBF090..0x82DBF11C)
	// 82DBF090: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF120 size=276
    let mut pc: u32 = 0x82DBF120;
    'dispatch: loop {
        match pc {
            0x82DBF120 => {
    //   block [0x82DBF120..0x82DBF16C)
	// 82DBF120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF124: 4BEEA2E9  bl 0x82ca940c
	ctx.lr = 0x82DBF128;
	sub_82CA93D0(ctx, base);
	// 82DBF128: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF12C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF130: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DBF134: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DBF138: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBF13C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBF140: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF144: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF148: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DBF14C: 40980020  bge cr6, 0x82dbf16c
	if !ctx.cr[6].lt {
	pc = 0x82DBF16C; continue 'dispatch;
	}
	// 82DBF150: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DBF154: 38842D80  addi r4, r4, 0x2d80
	ctx.r[4].s64 = ctx.r[4].s64 + 11648;
	// 82DBF158: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DBF15C: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82DBF160: 3BAA000C  addi r29, r10, 0xc
	ctx.r[29].s64 = ctx.r[10].s64 + 12;
	// 82DBF164: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DBF168: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	pc = 0x82DBF16C; continue 'dispatch;
            }
            0x82DBF16C => {
    //   block [0x82DBF16C..0x82DBF18C)
	// 82DBF16C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF170: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBF174: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF178: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DBF17C: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82DBF180: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF184: 409A0008  bne cr6, 0x82dbf18c
	if !ctx.cr[6].eq {
	pc = 0x82DBF18C; continue 'dispatch;
	}
	// 82DBF188: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x82DBF18C; continue 'dispatch;
            }
            0x82DBF18C => {
    //   block [0x82DBF18C..0x82DBF1AC)
	// 82DBF18C: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBF190: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF198: 419A001C  beq cr6, 0x82dbf1b4
	if ctx.cr[6].eq {
	pc = 0x82DBF1B4; continue 'dispatch;
	}
	// 82DBF19C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF1A0: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DBF1A4: 409A0008  bne cr6, 0x82dbf1ac
	if !ctx.cr[6].eq {
	pc = 0x82DBF1AC; continue 'dispatch;
	}
	// 82DBF1A8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x82DBF1AC; continue 'dispatch;
            }
            0x82DBF1AC => {
    //   block [0x82DBF1AC..0x82DBF1B4)
	// 82DBF1AC: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82DBF1B0: 48000008  b 0x82dbf1b8
	pc = 0x82DBF1B8; continue 'dispatch;
            }
            0x82DBF1B4 => {
    //   block [0x82DBF1B4..0x82DBF1B8)
	// 82DBF1B4: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	pc = 0x82DBF1B8; continue 'dispatch;
            }
            0x82DBF1B8 => {
    //   block [0x82DBF1B8..0x82DBF234)
	// 82DBF1B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF238 size=472
    let mut pc: u32 = 0x82DBF238;
    'dispatch: loop {
        match pc {
            0x82DBF238 => {
    //   block [0x82DBF238..0x82DBF298)
	// 82DBF238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF23C: 4BEEA1B1  bl 0x82ca93ec
	ctx.lr = 0x82DBF240;
	sub_82CA93D0(ctx, base);
	// 82DBF240: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF244: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF248: 3AA00008  li r21, 8
	ctx.r[21].s64 = 8;
	// 82DBF24C: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82DBF250: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DBF254: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DBF258: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBF25C: 7D7AA82E  lwzx r11, r26, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82DBF260: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBF264: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DBF268: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82DBF26C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF270: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF274: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBF278: 40980020  bge cr6, 0x82dbf298
	if !ctx.cr[6].lt {
	pc = 0x82DBF298; continue 'dispatch;
	}
	// 82DBF27C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBF280: 39292D90  addi r9, r9, 0x2d90
	ctx.r[9].s64 = ctx.r[9].s64 + 11664;
	// 82DBF284: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBF288: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBF28C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBF290: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBF294: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBF298; continue 'dispatch;
            }
            0x82DBF298 => {
    //   block [0x82DBF298..0x82DBF410)
	// 82DBF298: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF410 size=260
    let mut pc: u32 = 0x82DBF410;
    'dispatch: loop {
        match pc {
            0x82DBF410 => {
    //   block [0x82DBF410..0x82DBF458)
	// 82DBF410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF414: 4BEE9FF1  bl 0x82ca9404
	ctx.lr = 0x82DBF418;
	sub_82CA93D0(ctx, base);
	// 82DBF418: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF41C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF420: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82DBF424: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DBF428: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBF42C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF430: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF434: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DBF438: 40980020  bge cr6, 0x82dbf458
	if !ctx.cr[6].lt {
	pc = 0x82DBF458; continue 'dispatch;
	}
	// 82DBF43C: 3FE08203  lis r31, -0x7dfd
	ctx.r[31].s64 = -2113732608;
	// 82DBF440: 3BFF2DA0  addi r31, r31, 0x2da0
	ctx.r[31].s64 = ctx.r[31].s64 + 11680;
	// 82DBF444: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DBF448: 7FEC42E6  mftb r31, 0x10c
	ctx.r[31].u64 = crate::rt::rdtsc_u64();
	// 82DBF44C: 3B64000C  addi r27, r4, 0xc
	ctx.r[27].s64 = ctx.r[4].s64 + 12;
	// 82DBF450: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBF454: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	pc = 0x82DBF458; continue 'dispatch;
            }
            0x82DBF458 => {
    //   block [0x82DBF458..0x82DBF46C)
	// 82DBF458: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DBF45C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBF460: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 82DBF464: 409A0008  bne cr6, 0x82dbf46c
	if !ctx.cr[6].eq {
	pc = 0x82DBF46C; continue 'dispatch;
	}
	// 82DBF468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82DBF46C; continue 'dispatch;
            }
            0x82DBF46C => {
    //   block [0x82DBF46C..0x82DBF490)
	// 82DBF46C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBF470: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DBF474: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF478: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF47C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF480: 419A0010  beq cr6, 0x82dbf490
	if ctx.cr[6].eq {
	pc = 0x82DBF490; continue 'dispatch;
	}
	// 82DBF484: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DBF488: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 82DBF48C: 409A0008  bne cr6, 0x82dbf494
	if !ctx.cr[6].eq {
	pc = 0x82DBF494; continue 'dispatch;
	}
	pc = 0x82DBF490; continue 'dispatch;
            }
            0x82DBF490 => {
    //   block [0x82DBF490..0x82DBF494)
	// 82DBF490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82DBF494; continue 'dispatch;
            }
            0x82DBF494 => {
    //   block [0x82DBF494..0x82DBF514)
	// 82DBF494: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBF498: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBF520 size=236
    let mut pc: u32 = 0x82DBF520;
    'dispatch: loop {
        match pc {
            0x82DBF520 => {
    //   block [0x82DBF520..0x82DBF558)
	// 82DBF520: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBF524: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBF528: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBF52C: 39630100  addi r11, r3, 0x100
	ctx.r[11].s64 = ctx.r[3].s64 + 256;
	// 82DBF530: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBF534: 394A85F4  addi r10, r10, -0x7a0c
	ctx.r[10].s64 = ctx.r[10].s64 + -31244;
	// 82DBF538: 39292DC4  addi r9, r9, 0x2dc4
	ctx.r[9].s64 = ctx.r[9].s64 + 11716;
	// 82DBF53C: 39082DB4  addi r8, r8, 0x2db4
	ctx.r[8].s64 = ctx.r[8].s64 + 11700;
	// 82DBF540: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82DBF544: B0AB0006  sth r5, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82DBF548: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82DBF54C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DBF550: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBF554: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	pc = 0x82DBF558; continue 'dispatch;
            }
            0x82DBF558 => {
    //   block [0x82DBF558..0x82DBF568)
	// 82DBF558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF55C: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF560: 409A0008  bne cr6, 0x82dbf568
	if !ctx.cr[6].eq {
	pc = 0x82DBF568; continue 'dispatch;
	}
	// 82DBF564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF568; continue 'dispatch;
            }
            0x82DBF568 => {
    //   block [0x82DBF568..0x82DBF57C)
	// 82DBF568: 9147FFF8  stw r10, -8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 82DBF56C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF570: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF574: 409A0008  bne cr6, 0x82dbf57c
	if !ctx.cr[6].eq {
	pc = 0x82DBF57C; continue 'dispatch;
	}
	// 82DBF578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF57C; continue 'dispatch;
            }
            0x82DBF57C => {
    //   block [0x82DBF57C..0x82DBF590)
	// 82DBF57C: 9147FFFC  stw r10, -4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 82DBF580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF584: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF588: 409A0008  bne cr6, 0x82dbf590
	if !ctx.cr[6].eq {
	pc = 0x82DBF590; continue 'dispatch;
	}
	// 82DBF58C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF590; continue 'dispatch;
            }
            0x82DBF590 => {
    //   block [0x82DBF590..0x82DBF5A4)
	// 82DBF590: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBF594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF598: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF59C: 409A0008  bne cr6, 0x82dbf5a4
	if !ctx.cr[6].eq {
	pc = 0x82DBF5A4; continue 'dispatch;
	}
	// 82DBF5A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF5A4; continue 'dispatch;
            }
            0x82DBF5A4 => {
    //   block [0x82DBF5A4..0x82DBF5B8)
	// 82DBF5A4: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DBF5A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5AC: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5B0: 409A0008  bne cr6, 0x82dbf5b8
	if !ctx.cr[6].eq {
	pc = 0x82DBF5B8; continue 'dispatch;
	}
	// 82DBF5B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF5B8; continue 'dispatch;
            }
            0x82DBF5B8 => {
    //   block [0x82DBF5B8..0x82DBF5CC)
	// 82DBF5B8: 91470008  stw r10, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DBF5BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5C0: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5C4: 409A0008  bne cr6, 0x82dbf5cc
	if !ctx.cr[6].eq {
	pc = 0x82DBF5CC; continue 'dispatch;
	}
	// 82DBF5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF5CC; continue 'dispatch;
            }
            0x82DBF5CC => {
    //   block [0x82DBF5CC..0x82DBF5E0)
	// 82DBF5CC: 9147000C  stw r10, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DBF5D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5D4: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5D8: 409A0008  bne cr6, 0x82dbf5e0
	if !ctx.cr[6].eq {
	pc = 0x82DBF5E0; continue 'dispatch;
	}
	// 82DBF5DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF5E0; continue 'dispatch;
            }
            0x82DBF5E0 => {
    //   block [0x82DBF5E0..0x82DBF5F4)
	// 82DBF5E0: 91470010  stw r10, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF5E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5E8: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5EC: 409A0008  bne cr6, 0x82dbf5f4
	if !ctx.cr[6].eq {
	pc = 0x82DBF5F4; continue 'dispatch;
	}
	// 82DBF5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DBF5F4; continue 'dispatch;
            }
            0x82DBF5F4 => {
    //   block [0x82DBF5F4..0x82DBF60C)
	// 82DBF5F4: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 82DBF5F8: 91470014  stw r10, 0x14(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBF5FC: 38E70020  addi r7, r7, 0x20
	ctx.r[7].s64 = ctx.r[7].s64 + 32;
	// 82DBF600: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF604: 409AFF54  bne cr6, 0x82dbf558
	if !ctx.cr[6].eq {
	pc = 0x82DBF558; continue 'dispatch;
	}
	// 82DBF608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBF610 size=48
    let mut pc: u32 = 0x82DBF610;
    'dispatch: loop {
        match pc {
            0x82DBF610 => {
    //   block [0x82DBF610..0x82DBF624)
	// 82DBF610: 39230100  addi r9, r3, 0x100
	ctx.r[9].s64 = ctx.r[3].s64 + 256;
	// 82DBF614: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DBF618: 39090008  addi r8, r9, 8
	ctx.r[8].s64 = ctx.r[9].s64 + 8;
	// 82DBF61C: 409A0008  bne cr6, 0x82dbf624
	if !ctx.cr[6].eq {
	pc = 0x82DBF624; continue 'dispatch;
	}
	// 82DBF620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	pc = 0x82DBF624; continue 'dispatch;
            }
            0x82DBF624 => {
    //   block [0x82DBF624..0x82DBF640)
	// 82DBF624: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBF628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBF62C: 396B85F4  addi r11, r11, -0x7a0c
	ctx.r[11].s64 = ctx.r[11].s64 + -31244;
	// 82DBF630: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DBF634: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBF638: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBF63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF640 size=188
    let mut pc: u32 = 0x82DBF640;
    'dispatch: loop {
        match pc {
            0x82DBF640 => {
    //   block [0x82DBF640..0x82DBF664)
	// 82DBF640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF644: 4BEE9DC5  bl 0x82ca9408
	ctx.lr = 0x82DBF648;
	sub_82CA93D0(ctx, base);
	// 82DBF648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF64C: 3B85FFFF  addi r28, r5, -1
	ctx.r[28].s64 = ctx.r[5].s64 + -1;
	// 82DBF650: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBF654: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBF658: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBF65C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DBF660: 41980094  blt cr6, 0x82dbf6f4
	if ctx.cr[6].lt {
	pc = 0x82DBF6F4; continue 'dispatch;
	}
	pc = 0x82DBF664; continue 'dispatch;
            }
            0x82DBF664 => {
    //   block [0x82DBF664..0x82DBF6E4)
	// 82DBF664: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF668: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBF66C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBF674: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF678: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF67C: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DBF680: 88CA0005  lbz r6, 5(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DBF684: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 82DBF688: 7CC90774  extsb r9, r6
	ctx.r[9].s64 = ctx.r[6].s8 as i64;
	// 82DBF68C: 7CA85A14  add r5, r8, r11
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DBF690: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DBF694: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82DBF698: 4E800421  bctrl
	ctx.lr = 0x82DBF69C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBF69C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF6A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF6A4: 419A0040  beq cr6, 0x82dbf6e4
	if ctx.cr[6].eq {
	pc = 0x82DBF6E4; continue 'dispatch;
	}
	// 82DBF6A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF6AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBF6B0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6B4: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6B8: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6BC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DBF6C0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DBF6C4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF6C8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBF6CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF6D0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBF6D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF6D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBF6E0: 4E800421  bctrl
	ctx.lr = 0x82DBF6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DBF6E4 => {
    //   block [0x82DBF6E4..0x82DBF6F4)
	// 82DBF6E4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DBF6E8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DBF6EC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DBF6F0: 4098FF74  bge cr6, 0x82dbf664
	if !ctx.cr[6].lt {
	pc = 0x82DBF664; continue 'dispatch;
	}
	pc = 0x82DBF6F4; continue 'dispatch;
            }
            0x82DBF6F4 => {
    //   block [0x82DBF6F4..0x82DBF6FC)
	// 82DBF6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBF6F8: 4BEE9D60  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF700 size=116
    let mut pc: u32 = 0x82DBF700;
    'dispatch: loop {
        match pc {
            0x82DBF700 => {
    //   block [0x82DBF700..0x82DBF720)
	// 82DBF700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF704: 4BEE9D09  bl 0x82ca940c
	ctx.lr = 0x82DBF708;
	sub_82CA93D0(ctx, base);
	// 82DBF708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF70C: 3BC5FFFF  addi r30, r5, -1
	ctx.r[30].s64 = ctx.r[5].s64 + -1;
	// 82DBF710: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBF714: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBF718: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DBF71C: 41980050  blt cr6, 0x82dbf76c
	if ctx.cr[6].lt {
	pc = 0x82DBF76C; continue 'dispatch;
	}
	pc = 0x82DBF720; continue 'dispatch;
            }
            0x82DBF720 => {
    //   block [0x82DBF720..0x82DBF76C)
	// 82DBF720: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF724: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBF728: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF72C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF730: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF734: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DBF738: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DBF73C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF740: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBF744: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF748: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBF74C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF750: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBF754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBF758: 4E800421  bctrl
	ctx.lr = 0x82DBF75C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBF75C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DBF760: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DBF764: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DBF768: 4098FFB8  bge cr6, 0x82dbf720
	if !ctx.cr[6].lt {
	pc = 0x82DBF720; continue 'dispatch;
	}
            }
            0x82DBF76C => {
    //   block [0x82DBF76C..0x82DBF774)
	// 82DBF76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBF770: 4BEE9CEC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF778 size=1172
    let mut pc: u32 = 0x82DBF778;
    'dispatch: loop {
        match pc {
            0x82DBF778 => {
    //   block [0x82DBF778..0x82DBF7A4)
	// 82DBF778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF77C: 4BEE9C71  bl 0x82ca93ec
	ctx.lr = 0x82DBF780;
	sub_82CA93D0(ctx, base);
	// 82DBF780: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF784: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DBF788: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DBF78C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF790: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF794: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DBF798: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF79C: 41980008  blt cr6, 0x82dbf7a4
	if ctx.cr[6].lt {
	pc = 0x82DBF7A4; continue 'dispatch;
	}
	// 82DBF7A0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x82DBF7A4; continue 'dispatch;
            }
            0x82DBF7A4 => {
    //   block [0x82DBF7A4..0x82DBF7C8)
	// 82DBF7A4: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82DBF7A8: 40980154  bge cr6, 0x82dbf8fc
	if !ctx.cr[6].lt {
	pc = 0x82DBF8FC; continue 'dispatch;
	}
	// 82DBF7AC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DBF7B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBF7B4: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82DBF7B8: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82DBF7BC: 40990104  ble cr6, 0x82dbf8c0
	if !ctx.cr[6].gt {
	pc = 0x82DBF8C0; continue 'dispatch;
	}
	// 82DBF7C0: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82DBF7C4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	pc = 0x82DBF7C8; continue 'dispatch;
            }
            0x82DBF7C8 => {
    //   block [0x82DBF7C8..0x82DBF7E8)
	// 82DBF7C8: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF7CC: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 82DBF7D0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DBF7D4: 4099005C  ble cr6, 0x82dbf830
	if !ctx.cr[6].gt {
	pc = 0x82DBF830; continue 'dispatch;
	}
	// 82DBF7D8: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF7DC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF7E0: 7D054A14  add r8, r5, r9
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82DBF7E4: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82DBF7E8; continue 'dispatch;
            }
            0x82DBF7E8 => {
    //   block [0x82DBF7E8..0x82DBF804)
	// 82DBF7E8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF7EC: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DBF7F0: 409A0014  bne cr6, 0x82dbf804
	if !ctx.cr[6].eq {
	pc = 0x82DBF804; continue 'dispatch;
	}
	// 82DBF7F4: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF7F8: 83A80004  lwz r29, 4(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF7FC: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DBF800: 419A0030  beq cr6, 0x82dbf830
	if ctx.cr[6].eq {
	pc = 0x82DBF830; continue 'dispatch;
	}
	pc = 0x82DBF804; continue 'dispatch;
            }
            0x82DBF804 => {
    //   block [0x82DBF804..0x82DBF81C)
	// 82DBF804: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF808: 7F1E3840  cmplw cr6, r30, r7
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DBF80C: 409A0010  bne cr6, 0x82dbf81c
	if !ctx.cr[6].eq {
	pc = 0x82DBF81C; continue 'dispatch;
	}
	// 82DBF810: 83C80004  lwz r30, 4(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF814: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DBF818: 419A0018  beq cr6, 0x82dbf830
	if ctx.cr[6].eq {
	pc = 0x82DBF830; continue 'dispatch;
	}
	pc = 0x82DBF81C; continue 'dispatch;
            }
            0x82DBF81C => {
    //   block [0x82DBF81C..0x82DBF830)
	// 82DBF81C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF820: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DBF824: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBF828: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DBF82C: 4198FFBC  blt cr6, 0x82dbf7e8
	if ctx.cr[6].lt {
	pc = 0x82DBF7E8; continue 'dispatch;
	}
	pc = 0x82DBF830; continue 'dispatch;
            }
            0x82DBF830 => {
    //   block [0x82DBF830..0x82DBF85C)
	// 82DBF830: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DBF834: 409A0034  bne cr6, 0x82dbf868
	if !ctx.cr[6].eq {
	pc = 0x82DBF868; continue 'dispatch;
	}
	// 82DBF838: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82DBF83C: 419A0020  beq cr6, 0x82dbf85c
	if ctx.cr[6].eq {
	pc = 0x82DBF85C; continue 'dispatch;
	}
	// 82DBF840: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF844: 7D455A14  add r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82DBF848: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82DBF84C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF850: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBF854: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF858: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x82DBF85C; continue 'dispatch;
            }
            0x82DBF85C => {
    //   block [0x82DBF85C..0x82DBF868)
	// 82DBF85C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBF860: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82DBF864: 48000048  b 0x82dbf8ac
	pc = 0x82DBF8AC; continue 'dispatch;
            }
            0x82DBF868 => {
    //   block [0x82DBF868..0x82DBF87C)
	// 82DBF868: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 82DBF86C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF870: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DBF874: 40980038  bge cr6, 0x82dbf8ac
	if !ctx.cr[6].lt {
	pc = 0x82DBF8AC; continue 'dispatch;
	}
	// 82DBF878: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	pc = 0x82DBF87C; continue 'dispatch;
            }
            0x82DBF87C => {
    //   block [0x82DBF87C..0x82DBF8AC)
	// 82DBF87C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF880: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DBF884: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DBF888: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DBF88C: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 82DBF890: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF894: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DBF898: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF89C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBF8A0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF8A4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF8A8: 4198FFD4  blt cr6, 0x82dbf87c
	if ctx.cr[6].lt {
	pc = 0x82DBF87C; continue 'dispatch;
	}
	pc = 0x82DBF8AC; continue 'dispatch;
            }
            0x82DBF8AC => {
    //   block [0x82DBF8AC..0x82DBF8C0)
	// 82DBF8AC: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF8B0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82DBF8B4: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 82DBF8B8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF8BC: 4198FF0C  blt cr6, 0x82dbf7c8
	if ctx.cr[6].lt {
	pc = 0x82DBF7C8; continue 'dispatch;
	}
	pc = 0x82DBF8C0; continue 'dispatch;
            }
            0x82DBF8C0 => {
    //   block [0x82DBF8C0..0x82DBF8E0)
	// 82DBF8C0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBF8C4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBF8C8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBF8CC: 40980024  bge cr6, 0x82dbf8f0
	if !ctx.cr[6].lt {
	pc = 0x82DBF8F0; continue 'dispatch;
	}
	// 82DBF8D0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF8D4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF8D8: 41980008  blt cr6, 0x82dbf8e0
	if ctx.cr[6].lt {
	pc = 0x82DBF8E0; continue 'dispatch;
	}
	// 82DBF8DC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82DBF8E0; continue 'dispatch;
            }
            0x82DBF8E0 => {
    //   block [0x82DBF8E0..0x82DBF8F0)
	// 82DBF8E0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DBF8E4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBF8E8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DBF8EC: 4BF97625  bl 0x82d56f10
	ctx.lr = 0x82DBF8F0;
	sub_82D56F10(ctx, base);
	pc = 0x82DBF8F0; continue 'dispatch;
            }
            0x82DBF8F0 => {
    //   block [0x82DBF8F0..0x82DBF8FC)
	// 82DBF8F0: 93F80004  stw r31, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBF8F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DBF8F8: 4BEE9B44  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DBF8FC => {
    //   block [0x82DBF8FC..0x82DBF938)
	// 82DBF8FC: 4BFA0D15  bl 0x82d60610
	ctx.lr = 0x82DBF900;
	sub_82D60610(ctx, base);
	// 82DBF900: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF904: 3AE00004  li r23, 4
	ctx.r[23].s64 = 4;
	// 82DBF908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBF90C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82DBF910: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DBF914: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBF918: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF91C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBF920: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DBF924: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBF928: 41990010  bgt cr6, 0x82dbf938
	if ctx.cr[6].gt {
	pc = 0x82DBF938; continue 'dispatch;
	}
	// 82DBF92C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82DBF930: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DBF934: 48000018  b 0x82dbf94c
	pc = 0x82DBF94C; continue 'dispatch;
            }
            0x82DBF938 => {
    //   block [0x82DBF938..0x82DBF94C)
	// 82DBF938: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF93C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBF940: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBF944: 4E800421  bctrl
	ctx.lr = 0x82DBF948;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBF948: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
            }
            0x82DBF94C => {
    //   block [0x82DBF94C..0x82DBF974)
	// 82DBF94C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DBF950: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DBF954: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBF958: 4BFA0C71  bl 0x82d605c8
	ctx.lr = 0x82DBF95C;
	sub_82D605C8(ctx, base);
	// 82DBF95C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF960: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DBF964: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBF968: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 82DBF96C: 409900B0  ble cr6, 0x82dbfa1c
	if !ctx.cr[6].gt {
	pc = 0x82DBFA1C; continue 'dispatch;
	}
	// 82DBF970: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	pc = 0x82DBF974; continue 'dispatch;
            }
            0x82DBF974 => {
    //   block [0x82DBF974..0x82DBF99C)
	// 82DBF974: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF978: 7FDF582A  ldx r30, r31, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 82DBF97C: FBC10050  std r30, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u64 ) };
	// 82DBF980: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DBF984: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DBF988: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DBF98C: 40990010  ble cr6, 0x82dbf99c
	if !ctx.cr[6].gt {
	pc = 0x82DBF99C; continue 'dispatch;
	}
	// 82DBF990: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DBF994: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DBF998: EBC10050  ld r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	pc = 0x82DBF99C; continue 'dispatch;
            }
            0x82DBF99C => {
    //   block [0x82DBF99C..0x82DBF9BC)
	// 82DBF99C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBF9A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBF9A4: 4BFA102D  bl 0x82d609d0
	ctx.lr = 0x82DBF9A8;
	sub_82D609D0(ctx, base);
	// 82DBF9A8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DBF9AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBF9B0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF9B4: 40990008  ble cr6, 0x82dbf9bc
	if !ctx.cr[6].gt {
	pc = 0x82DBF9BC; continue 'dispatch;
	}
	// 82DBF9B8: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	pc = 0x82DBF9BC; continue 'dispatch;
            }
            0x82DBF9BC => {
    //   block [0x82DBF9BC..0x82DBF9F0)
	// 82DBF9BC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DBF9C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBF9C4: 419A002C  beq cr6, 0x82dbf9f0
	if ctx.cr[6].eq {
	pc = 0x82DBF9F0; continue 'dispatch;
	}
	// 82DBF9C8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DBF9CC: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DBF9D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBF9D4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF9D8: 7D2B502A  ldx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	// 82DBF9DC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBF9E0: 7D2B512A  stdx r9, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u64) };
	// 82DBF9E4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF9E8: 7EBF592E  stwx r21, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 82DBF9EC: 4800001C  b 0x82dbfa08
	pc = 0x82DBFA08; continue 'dispatch;
            }
            0x82DBF9F0 => {
    //   block [0x82DBF9F0..0x82DBFA08)
	// 82DBF9F0: 57AB402E  slwi r11, r29, 8
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF9F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBF9F8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBF9FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFA00: 61650001  ori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 | 1;
	// 82DBFA04: 4BFA0EED  bl 0x82d608f0
	ctx.lr = 0x82DBFA08;
	sub_82D608F0(ctx, base);
	pc = 0x82DBFA08; continue 'dispatch;
            }
            0x82DBFA08 => {
    //   block [0x82DBFA08..0x82DBFA1C)
	// 82DBFA08: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFA0C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DBFA10: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DBFA14: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFA18: 4198FF5C  blt cr6, 0x82dbf974
	if ctx.cr[6].lt {
	pc = 0x82DBF974; continue 'dispatch;
	}
	pc = 0x82DBFA1C; continue 'dispatch;
            }
            0x82DBFA1C => {
    //   block [0x82DBFA1C..0x82DBFA38)
	// 82DBFA1C: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFA20: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 82DBFA24: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 82DBFA28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBFA2C: 409900E0  ble cr6, 0x82dbfb0c
	if !ctx.cr[6].gt {
	pc = 0x82DBFB0C; continue 'dispatch;
	}
	// 82DBFA30: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 82DBFA34: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	pc = 0x82DBFA38; continue 'dispatch;
            }
            0x82DBFA38 => {
    //   block [0x82DBFA38..0x82DBFA5C)
	// 82DBFA38: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFA3C: 7D7E582A  ldx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) };
	// 82DBFA40: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DBFA44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DBFA48: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DBFA4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DBFA50: 4099000C  ble cr6, 0x82dbfa5c
	if !ctx.cr[6].gt {
	pc = 0x82DBFA5C; continue 'dispatch;
	}
	// 82DBFA54: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DBFA58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	pc = 0x82DBFA5C; continue 'dispatch;
            }
            0x82DBFA5C => {
    //   block [0x82DBFA5C..0x82DBFA80)
	// 82DBFA5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFA60: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DBFA64: 4BFA0F6D  bl 0x82d609d0
	ctx.lr = 0x82DBFA68;
	sub_82D609D0(ctx, base);
	// 82DBFA68: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DBFA6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DBFA70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DBFA74: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DBFA78: 40990008  ble cr6, 0x82dbfa80
	if !ctx.cr[6].gt {
	pc = 0x82DBFA80; continue 'dispatch;
	}
	// 82DBFA7C: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	pc = 0x82DBFA80; continue 'dispatch;
            }
            0x82DBFA80 => {
    //   block [0x82DBFA80..0x82DBFABC)
	// 82DBFA80: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DBFA84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBFA88: 419A004C  beq cr6, 0x82dbfad4
	if ctx.cr[6].eq {
	pc = 0x82DBFAD4; continue 'dispatch;
	}
	// 82DBFA8C: 7D6A2214  add r11, r10, r4
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82DBFA90: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DBFA94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBFA98: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBFA9C: 7D6A482A  ldx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82DBFAA0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DBFAA4: 57E8063E  clrlwi r8, r31, 0x18
	ctx.r[8].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82DBFAA8: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82DBFAAC: 40990010  ble cr6, 0x82dbfabc
	if !ctx.cr[6].gt {
	pc = 0x82DBFABC; continue 'dispatch;
	}
	// 82DBFAB0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBFAB4: 7D6A492A  stdx r11, r10, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u64) };
	// 82DBFAB8: 48000040  b 0x82dbfaf8
	pc = 0x82DBFAF8; continue 'dispatch;
            }
            0x82DBFABC => {
    //   block [0x82DBFABC..0x82DBFAD4)
	// 82DBFABC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFAC0: 4BFA0F71  bl 0x82d60a30
	ctx.lr = 0x82DBFAC4;
	sub_82D60A30(ctx, base);
	// 82DBFAC4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFAC8: 57EAD978  rlwinm r10, r31, 0x1b, 5, 0x1c
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 82DBFACC: 7EAA592E  stwx r21, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 82DBFAD0: 48000028  b 0x82dbfaf8
	pc = 0x82DBFAF8; continue 'dispatch;
            }
            0x82DBFAD4 => {
    //   block [0x82DBFAD4..0x82DBFAF8)
	// 82DBFAD4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFAD8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DBFADC: 7D5E5A14  add r10, r30, r11
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DBFAE0: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82DBFAE4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82DBFAE8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFAEC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBFAF0: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFAF4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x82DBFAF8; continue 'dispatch;
            }
            0x82DBFAF8 => {
    //   block [0x82DBFAF8..0x82DBFB0C)
	// 82DBFAF8: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFAFC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DBFB00: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DBFB04: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFB08: 4198FF30  blt cr6, 0x82dbfa38
	if ctx.cr[6].lt {
	pc = 0x82DBFA38; continue 'dispatch;
	}
	pc = 0x82DBFB0C; continue 'dispatch;
            }
            0x82DBFB0C => {
    //   block [0x82DBFB0C..0x82DBFB2C)
	// 82DBFB0C: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBFB10: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBFB14: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DBFB18: 40980024  bge cr6, 0x82dbfb3c
	if !ctx.cr[6].lt {
	pc = 0x82DBFB3C; continue 'dispatch;
	}
	// 82DBFB1C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBFB20: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFB24: 41980008  blt cr6, 0x82dbfb2c
	if ctx.cr[6].lt {
	pc = 0x82DBFB2C; continue 'dispatch;
	}
	// 82DBFB28: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x82DBFB2C; continue 'dispatch;
            }
            0x82DBFB2C => {
    //   block [0x82DBFB2C..0x82DBFB3C)
	// 82DBFB2C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DBFB30: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBFB34: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DBFB38: 4BF973D9  bl 0x82d56f10
	ctx.lr = 0x82DBFB3C;
	sub_82D56F10(ctx, base);
	pc = 0x82DBFB3C; continue 'dispatch;
            }
            0x82DBFB3C => {
    //   block [0x82DBFB3C..0x82DBFB5C)
	// 82DBFB3C: 93980004  stw r28, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DBFB40: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82DBFB44: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFB48: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82DBFB4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBFB50: 40990050  ble cr6, 0x82dbfba0
	if !ctx.cr[6].gt {
	pc = 0x82DBFBA0; continue 'dispatch;
	}
	// 82DBFB54: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 82DBFB58: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	pc = 0x82DBFB5C; continue 'dispatch;
            }
            0x82DBFB5C => {
    //   block [0x82DBFB5C..0x82DBFB8C)
	// 82DBFB5C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFB60: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DBFB64: 7CC9582E  lwzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DBFB68: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBFB6C: 419A0020  beq cr6, 0x82dbfb8c
	if ctx.cr[6].eq {
	pc = 0x82DBFB8C; continue 'dispatch;
	}
	// 82DBFB70: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFB74: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DBFB78: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBFB7C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82DBFB80: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82DBFB84: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFB88: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x82DBFB8C; continue 'dispatch;
            }
            0x82DBFB8C => {
    //   block [0x82DBFB8C..0x82DBFBA0)
	// 82DBFB8C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFB90: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DBFB94: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DBFB98: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFB9C: 4198FFC0  blt cr6, 0x82dbfb5c
	if ctx.cr[6].lt {
	pc = 0x82DBFB5C; continue 'dispatch;
	}
	pc = 0x82DBFBA0; continue 'dispatch;
            }
            0x82DBFBA0 => {
    //   block [0x82DBFBA0..0x82DBFBC0)
	// 82DBFBA0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBFBA4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBFBA8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBFBAC: 40980024  bge cr6, 0x82dbfbd0
	if !ctx.cr[6].lt {
	pc = 0x82DBFBD0; continue 'dispatch;
	}
	// 82DBFBB0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBFBB4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFBB8: 41980008  blt cr6, 0x82dbfbc0
	if ctx.cr[6].lt {
	pc = 0x82DBFBC0; continue 'dispatch;
	}
	// 82DBFBBC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82DBFBC0; continue 'dispatch;
            }
            0x82DBFBC0 => {
    //   block [0x82DBFBC0..0x82DBFBD0)
	// 82DBFBC0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DBFBC4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBFBC8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DBFBCC: 4BF97345  bl 0x82d56f10
	ctx.lr = 0x82DBFBD0;
	sub_82D56F10(ctx, base);
	pc = 0x82DBFBD0; continue 'dispatch;
            }
            0x82DBFBD0 => {
    //   block [0x82DBFBD0..0x82DBFC04)
	// 82DBFBD0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFBD4: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBFBD8: 4BFA0CE1  bl 0x82d608b8
	ctx.lr = 0x82DBFBDC;
	sub_82D608B8(ctx, base);
	// 82DBFBDC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DBFBE0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBFBE4: 93230020  stw r25, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[25].u32 ) };
	// 82DBFBE8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DBFBEC: 409A0018  bne cr6, 0x82dbfc04
	if !ctx.cr[6].eq {
	pc = 0x82DBFC04; continue 'dispatch;
	}
	// 82DBFBF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFBF4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DBFBF8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBFBFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBFC00: 4E800421  bctrl
	ctx.lr = 0x82DBFC04;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DBFC04 => {
    //   block [0x82DBFC04..0x82DBFC0C)
	// 82DBFC04: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DBFC08: 4BEE9834  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBFC10 size=8
    let mut pc: u32 = 0x82DBFC10;
    'dispatch: loop {
        match pc {
            0x82DBFC10 => {
    //   block [0x82DBFC10..0x82DBFC18)
	// 82DBFC10: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82DBFC14: 48000004  b 0x82dbfc18
	sub_82DBFC18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


