pub fn sub_82D80F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80F70 size=12
    let mut pc: u32 = 0x82D80F70;
    'dispatch: loop {
        match pc {
            0x82D80F70 => {
    //   block [0x82D80F70..0x82D80F7C)
	// 82D80F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D80F74: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D80F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80F80 size=4
    let mut pc: u32 = 0x82D80F80;
    'dispatch: loop {
        match pc {
            0x82D80F80 => {
    //   block [0x82D80F80..0x82D80F84)
	// 82D80F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80F88 size=20
    let mut pc: u32 = 0x82D80F88;
    'dispatch: loop {
        match pc {
            0x82D80F88 => {
    //   block [0x82D80F88..0x82D80F9C)
	// 82D80F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D80F8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D80F90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D80F94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D80F98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80FA0 size=4
    let mut pc: u32 = 0x82D80FA0;
    'dispatch: loop {
        match pc {
            0x82D80FA0 => {
    //   block [0x82D80FA0..0x82D80FA4)
	// 82D80FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80FA8 size=32
    let mut pc: u32 = 0x82D80FA8;
    'dispatch: loop {
        match pc {
            0x82D80FA8 => {
    //   block [0x82D80FA8..0x82D80FC8)
	// 82D80FA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D80FAC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D80FB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D80FB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D80FB8: 396B901C  addi r11, r11, -0x6fe4
	ctx.r[11].s64 = ctx.r[11].s64 + -28644;
	// 82D80FBC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D80FC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D80FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80FC8 size=12
    let mut pc: u32 = 0x82D80FC8;
    'dispatch: loop {
        match pc {
            0x82D80FC8 => {
    //   block [0x82D80FC8..0x82D80FD4)
	// 82D80FC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D80FCC: 386B901C  addi r3, r11, -0x6fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -28644;
	// 82D80FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80FD8 size=4
    let mut pc: u32 = 0x82D80FD8;
    'dispatch: loop {
        match pc {
            0x82D80FD8 => {
    //   block [0x82D80FD8..0x82D80FDC)
	// 82D80FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80FE0 size=20
    let mut pc: u32 = 0x82D80FE0;
    'dispatch: loop {
        match pc {
            0x82D80FE0 => {
    //   block [0x82D80FE0..0x82D80FF4)
	// 82D80FE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D80FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D80FE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D80FEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D80FF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D80FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D80FF8 size=4
    let mut pc: u32 = 0x82D80FF8;
    'dispatch: loop {
        match pc {
            0x82D80FF8 => {
    //   block [0x82D80FF8..0x82D80FFC)
	// 82D80FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81000 size=32
    let mut pc: u32 = 0x82D81000;
    'dispatch: loop {
        match pc {
            0x82D81000 => {
    //   block [0x82D81000..0x82D81020)
	// 82D81000: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81004: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81008: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8100C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81010: 396B90CC  addi r11, r11, -0x6f34
	ctx.r[11].s64 = ctx.r[11].s64 + -28468;
	// 82D81014: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81018: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D8101C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81020 size=12
    let mut pc: u32 = 0x82D81020;
    'dispatch: loop {
        match pc {
            0x82D81020 => {
    //   block [0x82D81020..0x82D8102C)
	// 82D81020: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81024: 386B90CC  addi r3, r11, -0x6f34
	ctx.r[3].s64 = ctx.r[11].s64 + -28468;
	// 82D81028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81030 size=20
    let mut pc: u32 = 0x82D81030;
    'dispatch: loop {
        match pc {
            0x82D81030 => {
    //   block [0x82D81030..0x82D81044)
	// 82D81030: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81034: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81038: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8103C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81040: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81048 size=20
    let mut pc: u32 = 0x82D81048;
    'dispatch: loop {
        match pc {
            0x82D81048 => {
    //   block [0x82D81048..0x82D8105C)
	// 82D81048: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8104C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81050: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81058: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81060 size=32
    let mut pc: u32 = 0x82D81060;
    'dispatch: loop {
        match pc {
            0x82D81060 => {
    //   block [0x82D81060..0x82D81080)
	// 82D81060: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81064: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81068: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D8106C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81070: 396B63AC  addi r11, r11, 0x63ac
	ctx.r[11].s64 = ctx.r[11].s64 + 25516;
	// 82D81074: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81078: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D8107C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81080 size=12
    let mut pc: u32 = 0x82D81080;
    'dispatch: loop {
        match pc {
            0x82D81080 => {
    //   block [0x82D81080..0x82D8108C)
	// 82D81080: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81084: 386B63AC  addi r3, r11, 0x63ac
	ctx.r[3].s64 = ctx.r[11].s64 + 25516;
	// 82D81088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81090 size=32
    let mut pc: u32 = 0x82D81090;
    'dispatch: loop {
        match pc {
            0x82D81090 => {
    //   block [0x82D81090..0x82D810B0)
	// 82D81090: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81094: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81098: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8109C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D810A0: 396B915C  addi r11, r11, -0x6ea4
	ctx.r[11].s64 = ctx.r[11].s64 + -28324;
	// 82D810A4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D810A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D810AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D810B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D810B0 size=12
    let mut pc: u32 = 0x82D810B0;
    'dispatch: loop {
        match pc {
            0x82D810B0 => {
    //   block [0x82D810B0..0x82D810BC)
	// 82D810B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D810B4: 386B915C  addi r3, r11, -0x6ea4
	ctx.r[3].s64 = ctx.r[11].s64 + -28324;
	// 82D810B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D810C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D810C0 size=100
    let mut pc: u32 = 0x82D810C0;
    'dispatch: loop {
        match pc {
            0x82D810C0 => {
    //   block [0x82D810C0..0x82D81108)
	// 82D810C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D810C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D810C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D810CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D810D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D810D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D810D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D810DC: 4801AC75  bl 0x82d9bd50
	ctx.lr = 0x82D810E0;
	sub_82D9BD50(ctx, base);
	// 82D810E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D810E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D810E8: 419A0020  beq cr6, 0x82d81108
	if ctx.cr[6].eq {
	pc = 0x82D81108; continue 'dispatch;
	}
	// 82D810EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D810F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D810F4: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82D810F8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D810FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D81100: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D81104: 4BFD41C5  bl 0x82d552c8
	ctx.lr = 0x82D81108;
	sub_82D552C8(ctx, base);
	pc = 0x82D81108; continue 'dispatch;
            }
            0x82D81108 => {
    //   block [0x82D81108..0x82D81124)
	// 82D81108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D8110C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D81110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81118: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D8111C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81128 size=4
    let mut pc: u32 = 0x82D81128;
    'dispatch: loop {
        match pc {
            0x82D81128 => {
    //   block [0x82D81128..0x82D8112C)
	// 82D81128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81160 size=44
    let mut pc: u32 = 0x82D81160;
    'dispatch: loop {
        match pc {
            0x82D81160 => {
    //   block [0x82D81160..0x82D8118C)
	// 82D81160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81168: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8116C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D81174: 4801C05D  bl 0x82d9d1d0
	ctx.lr = 0x82D81178;
	sub_82D9D1D0(ctx, base);
	// 82D81178: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D8117C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D81180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81190 size=4
    let mut pc: u32 = 0x82D81190;
    'dispatch: loop {
        match pc {
            0x82D81190 => {
    //   block [0x82D81190..0x82D81194)
	// 82D81190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81198 size=20
    let mut pc: u32 = 0x82D81198;
    'dispatch: loop {
        match pc {
            0x82D81198 => {
    //   block [0x82D81198..0x82D811AC)
	// 82D81198: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8119C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D811A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D811A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D811A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D811B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D811B0 size=4
    let mut pc: u32 = 0x82D811B0;
    'dispatch: loop {
        match pc {
            0x82D811B0 => {
    //   block [0x82D811B0..0x82D811B4)
	// 82D811B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D811B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D811B8 size=32
    let mut pc: u32 = 0x82D811B8;
    'dispatch: loop {
        match pc {
            0x82D811B8 => {
    //   block [0x82D811B8..0x82D811D8)
	// 82D811B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D811BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D811C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D811C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D811C8: 396B9784  addi r11, r11, -0x687c
	ctx.r[11].s64 = ctx.r[11].s64 + -26748;
	// 82D811CC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D811D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D811D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D811D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D811D8 size=12
    let mut pc: u32 = 0x82D811D8;
    'dispatch: loop {
        match pc {
            0x82D811D8 => {
    //   block [0x82D811D8..0x82D811E4)
	// 82D811D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D811DC: 386B9784  addi r3, r11, -0x687c
	ctx.r[3].s64 = ctx.r[11].s64 + -26748;
	// 82D811E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D811E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D811E8 size=100
    let mut pc: u32 = 0x82D811E8;
    'dispatch: loop {
        match pc {
            0x82D811E8 => {
    //   block [0x82D811E8..0x82D81230)
	// 82D811E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D811EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D811F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D811F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D811F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D811FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D81200: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D81204: 4800A20D  bl 0x82d8b410
	ctx.lr = 0x82D81208;
	sub_82D8B410(ctx, base);
	// 82D81208: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D8120C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D81210: 419A0020  beq cr6, 0x82d81230
	if ctx.cr[6].eq {
	pc = 0x82D81230; continue 'dispatch;
	}
	// 82D81214: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81218: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D8121C: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D81220: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D81224: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D81228: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8122C: 4BFD409D  bl 0x82d552c8
	ctx.lr = 0x82D81230;
	sub_82D552C8(ctx, base);
	pc = 0x82D81230; continue 'dispatch;
            }
            0x82D81230 => {
    //   block [0x82D81230..0x82D8124C)
	// 82D81230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81234: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D81238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8123C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81240: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D81244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81250 size=4
    let mut pc: u32 = 0x82D81250;
    'dispatch: loop {
        match pc {
            0x82D81250 => {
    //   block [0x82D81250..0x82D81254)
	// 82D81250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81258 size=4
    let mut pc: u32 = 0x82D81258;
    'dispatch: loop {
        match pc {
            0x82D81258 => {
    //   block [0x82D81258..0x82D8125C)
	// 82D81258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81260 size=20
    let mut pc: u32 = 0x82D81260;
    'dispatch: loop {
        match pc {
            0x82D81260 => {
    //   block [0x82D81260..0x82D81274)
	// 82D81260: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81268: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8126C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81270: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81278 size=16
    let mut pc: u32 = 0x82D81278;
    'dispatch: loop {
        match pc {
            0x82D81278 => {
    //   block [0x82D81278..0x82D81288)
	// 82D81278: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D8127C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81280: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81284: 480048AC  b 0x82d85b30
	sub_82D85B30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81288 size=4
    let mut pc: u32 = 0x82D81288;
    'dispatch: loop {
        match pc {
            0x82D81288 => {
    //   block [0x82D81288..0x82D8128C)
	// 82D81288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81290 size=44
    let mut pc: u32 = 0x82D81290;
    'dispatch: loop {
        match pc {
            0x82D81290 => {
    //   block [0x82D81290..0x82D812BC)
	// 82D81290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81298: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8129C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D812A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D812A4: 4800488D  bl 0x82d85b30
	ctx.lr = 0x82D812A8;
	sub_82D85B30(ctx, base);
	// 82D812A8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D812AC: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 82D812B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D812B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D812B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D812C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D812C8 size=128
    let mut pc: u32 = 0x82D812C8;
    'dispatch: loop {
        match pc {
            0x82D812C8 => {
    //   block [0x82D812C8..0x82D81308)
	// 82D812C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D812CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D812D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D812D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D812D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D812DC: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 82D812E0: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D812E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D812E8: 409A0020  bne cr6, 0x82d81308
	if !ctx.cr[6].eq {
	pc = 0x82D81308; continue 'dispatch;
	}
	// 82D812EC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D812F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D812F4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D812F8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D812FC: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D81300: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D81304: 4BFD3FC5  bl 0x82d552c8
	ctx.lr = 0x82D81308;
	sub_82D552C8(ctx, base);
	pc = 0x82D81308; continue 'dispatch;
            }
            0x82D81308 => {
    //   block [0x82D81308..0x82D81334)
	// 82D81308: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D8130C: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D81310: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D81314: 409A0020  bne cr6, 0x82d81334
	if !ctx.cr[6].eq {
	pc = 0x82D81334; continue 'dispatch;
	}
	// 82D81318: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8131C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D81320: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D81324: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81328: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D8132C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D81330: 4BFD3F99  bl 0x82d552c8
	ctx.lr = 0x82D81334;
	sub_82D552C8(ctx, base);
	pc = 0x82D81334; continue 'dispatch;
            }
            0x82D81334 => {
    //   block [0x82D81334..0x82D81348)
	// 82D81334: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D81338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8133C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81348 size=152
    let mut pc: u32 = 0x82D81348;
    'dispatch: loop {
        match pc {
            0x82D81348 => {
    //   block [0x82D81348..0x82D813AC)
	// 82D81348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8134C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D81354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D81358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8135C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D81360: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D81364: 4BFFFF65  bl 0x82d812c8
	ctx.lr = 0x82D81368;
	sub_82D812C8(ctx, base);
	// 82D81368: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D8136C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D81370: 419A0054  beq cr6, 0x82d813c4
	if ctx.cr[6].eq {
	pc = 0x82D813C4; continue 'dispatch;
	}
	// 82D81374: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D81378: 419A004C  beq cr6, 0x82d813c4
	if ctx.cr[6].eq {
	pc = 0x82D813C4; continue 'dispatch;
	}
	// 82D8137C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81380: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D81384: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D81388: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D8138C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D81390: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D81394: 41980018  blt cr6, 0x82d813ac
	if ctx.cr[6].lt {
	pc = 0x82D813AC; continue 'dispatch;
	}
	// 82D81398: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D8139C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82D813A0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D813A4: 4BFD3D85  bl 0x82d55128
	ctx.lr = 0x82D813A8;
	sub_82D55128(ctx, base);
	// 82D813A8: 4800001C  b 0x82d813c4
	pc = 0x82D813C4; continue 'dispatch;
            }
            0x82D813AC => {
    //   block [0x82D813AC..0x82D813C4)
	// 82D813AC: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D813B0: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D813B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D813B8: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82D813BC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D813C0: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	pc = 0x82D813C4; continue 'dispatch;
            }
            0x82D813C4 => {
    //   block [0x82D813C4..0x82D813E0)
	// 82D813C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D813C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D813CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D813D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D813D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D813D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D813DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D813E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D813E0 size=20
    let mut pc: u32 = 0x82D813E0;
    'dispatch: loop {
        match pc {
            0x82D813E0 => {
    //   block [0x82D813E0..0x82D813F4)
	// 82D813E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D813E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D813E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D813EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D813F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D813F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D813F8 size=32
    let mut pc: u32 = 0x82D813F8;
    'dispatch: loop {
        match pc {
            0x82D813F8 => {
    //   block [0x82D813F8..0x82D81418)
	// 82D813F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D813FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81400: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81404: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81408: 396B9F2C  addi r11, r11, -0x60d4
	ctx.r[11].s64 = ctx.r[11].s64 + -24788;
	// 82D8140C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81410: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D81414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81418 size=12
    let mut pc: u32 = 0x82D81418;
    'dispatch: loop {
        match pc {
            0x82D81418 => {
    //   block [0x82D81418..0x82D81424)
	// 82D81418: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8141C: 386B9F2C  addi r3, r11, -0x60d4
	ctx.r[3].s64 = ctx.r[11].s64 + -24788;
	// 82D81420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81428 size=140
    let mut pc: u32 = 0x82D81428;
    'dispatch: loop {
        match pc {
            0x82D81428 => {
    //   block [0x82D81428..0x82D81468)
	// 82D81428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8142C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81430: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D81434: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D8143C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D81440: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D81444: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D81448: 409A0020  bne cr6, 0x82d81468
	if !ctx.cr[6].eq {
	pc = 0x82D81468; continue 'dispatch;
	}
	// 82D8144C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81450: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D81454: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D81458: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D8145C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D81460: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D81464: 4BFD3E65  bl 0x82d552c8
	ctx.lr = 0x82D81468;
	sub_82D552C8(ctx, base);
	pc = 0x82D81468; continue 'dispatch;
            }
            0x82D81468 => {
    //   block [0x82D81468..0x82D81494)
	// 82D81468: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D8146C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D81470: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D81474: 409A0020  bne cr6, 0x82d81494
	if !ctx.cr[6].eq {
	pc = 0x82D81494; continue 'dispatch;
	}
	// 82D81478: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8147C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D81480: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D81484: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D81488: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D8148C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D81490: 4BFD3E39  bl 0x82d552c8
	ctx.lr = 0x82D81494;
	sub_82D552C8(ctx, base);
	pc = 0x82D81494; continue 'dispatch;
            }
            0x82D81494 => {
    //   block [0x82D81494..0x82D814B4)
	// 82D81494: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D81498: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D8149C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D814A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D814A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D814A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D814AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D814B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D814B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D814B8 size=100
    let mut pc: u32 = 0x82D814B8;
    'dispatch: loop {
        match pc {
            0x82D814B8 => {
    //   block [0x82D814B8..0x82D81500)
	// 82D814B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D814BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D814C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D814C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D814C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D814CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D814D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D814D4: 4BFFFF55  bl 0x82d81428
	ctx.lr = 0x82D814D8;
	sub_82D81428(ctx, base);
	// 82D814D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D814DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D814E0: 419A0020  beq cr6, 0x82d81500
	if ctx.cr[6].eq {
	pc = 0x82D81500; continue 'dispatch;
	}
	// 82D814E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D814E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D814EC: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D814F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D814F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D814F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D814FC: 4BFD3DCD  bl 0x82d552c8
	ctx.lr = 0x82D81500;
	sub_82D552C8(ctx, base);
	pc = 0x82D81500; continue 'dispatch;
            }
            0x82D81500 => {
    //   block [0x82D81500..0x82D8151C)
	// 82D81500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D81508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8150C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81510: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D81514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81550 size=44
    let mut pc: u32 = 0x82D81550;
    'dispatch: loop {
        match pc {
            0x82D81550 => {
    //   block [0x82D81550..0x82D8157C)
	// 82D81550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8155C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D81564: 4801CDC5  bl 0x82d9e328
	ctx.lr = 0x82D81568;
	sub_82D9E328(ctx, base);
	// 82D81568: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D8156C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D81570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81580 size=4
    let mut pc: u32 = 0x82D81580;
    'dispatch: loop {
        match pc {
            0x82D81580 => {
    //   block [0x82D81580..0x82D81584)
	// 82D81580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D815A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D815A8 size=216
    let mut pc: u32 = 0x82D815A8;
    'dispatch: loop {
        match pc {
            0x82D815A8 => {
    //   block [0x82D815A8..0x82D815E8)
	// 82D815A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D815AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D815B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D815B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D815B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D815BC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D815C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D815C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D815C8: 409A0020  bne cr6, 0x82d815e8
	if !ctx.cr[6].eq {
	pc = 0x82D815E8; continue 'dispatch;
	}
	// 82D815CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D815D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D815D4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D815D8: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D815DC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D815E0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D815E4: 4BFD3CE5  bl 0x82d552c8
	ctx.lr = 0x82D815E8;
	sub_82D552C8(ctx, base);
	pc = 0x82D815E8; continue 'dispatch;
            }
            0x82D815E8 => {
    //   block [0x82D815E8..0x82D81614)
	// 82D815E8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D815EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D815F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D815F4: 409A0020  bne cr6, 0x82d81614
	if !ctx.cr[6].eq {
	pc = 0x82D81614; continue 'dispatch;
	}
	// 82D815F8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D815FC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D81600: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D81604: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D81608: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D8160C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D81610: 4BFD3CB9  bl 0x82d552c8
	ctx.lr = 0x82D81614;
	sub_82D552C8(ctx, base);
	pc = 0x82D81614; continue 'dispatch;
            }
            0x82D81614 => {
    //   block [0x82D81614..0x82D81640)
	// 82D81614: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D81618: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D8161C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D81620: 409A0020  bne cr6, 0x82d81640
	if !ctx.cr[6].eq {
	pc = 0x82D81640; continue 'dispatch;
	}
	// 82D81624: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81628: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D8162C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D81630: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D81634: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D81638: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D8163C: 4BFD3C8D  bl 0x82d552c8
	ctx.lr = 0x82D81640;
	sub_82D552C8(ctx, base);
	pc = 0x82D81640; continue 'dispatch;
            }
            0x82D81640 => {
    //   block [0x82D81640..0x82D8166C)
	// 82D81640: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D81644: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D81648: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D8164C: 409A0020  bne cr6, 0x82d8166c
	if !ctx.cr[6].eq {
	pc = 0x82D8166C; continue 'dispatch;
	}
	// 82D81650: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81654: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D81658: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D8165C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D81660: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D81664: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D81668: 4BFD3C61  bl 0x82d552c8
	ctx.lr = 0x82D8166C;
	sub_82D552C8(ctx, base);
	pc = 0x82D8166C; continue 'dispatch;
            }
            0x82D8166C => {
    //   block [0x82D8166C..0x82D81680)
	// 82D8166C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D81670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D8167C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81680 size=20
    let mut pc: u32 = 0x82D81680;
    'dispatch: loop {
        match pc {
            0x82D81680 => {
    //   block [0x82D81680..0x82D81694)
	// 82D81680: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81684: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81688: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8168C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81690: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81698 size=32
    let mut pc: u32 = 0x82D81698;
    'dispatch: loop {
        match pc {
            0x82D81698 => {
    //   block [0x82D81698..0x82D816B8)
	// 82D81698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8169C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D816A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D816A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D816A8: 396BA544  addi r11, r11, -0x5abc
	ctx.r[11].s64 = ctx.r[11].s64 + -23228;
	// 82D816AC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D816B0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D816B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D816B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D816B8 size=12
    let mut pc: u32 = 0x82D816B8;
    'dispatch: loop {
        match pc {
            0x82D816B8 => {
    //   block [0x82D816B8..0x82D816C4)
	// 82D816B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D816BC: 386BA544  addi r3, r11, -0x5abc
	ctx.r[3].s64 = ctx.r[11].s64 + -23228;
	// 82D816C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D816C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D816C8 size=100
    let mut pc: u32 = 0x82D816C8;
    'dispatch: loop {
        match pc {
            0x82D816C8 => {
    //   block [0x82D816C8..0x82D81710)
	// 82D816C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D816CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D816D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D816D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D816D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D816DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D816E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D816E4: 480017E5  bl 0x82d82ec8
	ctx.lr = 0x82D816E8;
	sub_82D82EC8(ctx, base);
	// 82D816E8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D816EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D816F0: 419A0020  beq cr6, 0x82d81710
	if ctx.cr[6].eq {
	pc = 0x82D81710; continue 'dispatch;
	}
	// 82D816F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D816F8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D816FC: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 82D81700: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D81704: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D81708: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8170C: 4BFD3BBD  bl 0x82d552c8
	ctx.lr = 0x82D81710;
	sub_82D552C8(ctx, base);
	pc = 0x82D81710; continue 'dispatch;
            }
            0x82D81710 => {
    //   block [0x82D81710..0x82D8172C)
	// 82D81710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D81718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8171C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D81724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81730 size=20
    let mut pc: u32 = 0x82D81730;
    'dispatch: loop {
        match pc {
            0x82D81730 => {
    //   block [0x82D81730..0x82D81744)
	// 82D81730: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81734: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81738: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8173C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81740: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81748 size=44
    let mut pc: u32 = 0x82D81748;
    'dispatch: loop {
        match pc {
            0x82D81748 => {
    //   block [0x82D81748..0x82D81774)
	// 82D81748: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D8174C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D81750: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81754: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82D81758: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D8175C: 394AA6A8  addi r10, r10, -0x5958
	ctx.r[10].s64 = ctx.r[10].s64 + -22872;
	// 82D81760: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82D81764: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D81768: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D8176C: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D81770: 4801CE30  b 0x82d9e5a0
	sub_82D9E5A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81774 size=4
    let mut pc: u32 = 0x82D81774;
    'dispatch: loop {
        match pc {
            0x82D81774 => {
    //   block [0x82D81774..0x82D81778)
	// 82D81774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81778 size=64
    let mut pc: u32 = 0x82D81778;
    'dispatch: loop {
        match pc {
            0x82D81778 => {
    //   block [0x82D81778..0x82D817B8)
	// 82D81778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8177C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81780: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81784: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81788: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D8178C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81790: 396BA6A8  addi r11, r11, -0x5958
	ctx.r[11].s64 = ctx.r[11].s64 + -22872;
	// 82D81794: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82D81798: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 82D8179C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82D817A0: 4801CE01  bl 0x82d9e5a0
	ctx.lr = 0x82D817A4;
	sub_82D9E5A0(ctx, base);
	// 82D817A4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D817A8: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82D817AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D817B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D817B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D817B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D817B8 size=100
    let mut pc: u32 = 0x82D817B8;
    'dispatch: loop {
        match pc {
            0x82D817B8 => {
    //   block [0x82D817B8..0x82D81800)
	// 82D817B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D817BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D817C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D817C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D817C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D817CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D817D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D817D4: 4801D415  bl 0x82d9ebe8
	ctx.lr = 0x82D817D8;
	sub_82D9EBE8(ctx, base);
	// 82D817D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D817DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D817E0: 419A0020  beq cr6, 0x82d81800
	if ctx.cr[6].eq {
	pc = 0x82D81800; continue 'dispatch;
	}
	// 82D817E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D817E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D817EC: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D817F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D817F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D817F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D817FC: 4BFD3ACD  bl 0x82d552c8
	ctx.lr = 0x82D81800;
	sub_82D552C8(ctx, base);
	pc = 0x82D81800; continue 'dispatch;
            }
            0x82D81800 => {
    //   block [0x82D81800..0x82D8181C)
	// 82D81800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D81808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8180C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81810: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D81814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81820 size=4
    let mut pc: u32 = 0x82D81820;
    'dispatch: loop {
        match pc {
            0x82D81820 => {
    //   block [0x82D81820..0x82D81824)
	// 82D81820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81828 size=20
    let mut pc: u32 = 0x82D81828;
    'dispatch: loop {
        match pc {
            0x82D81828 => {
    //   block [0x82D81828..0x82D8183C)
	// 82D81828: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8182C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81830: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81838: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81840 size=4
    let mut pc: u32 = 0x82D81840;
    'dispatch: loop {
        match pc {
            0x82D81840 => {
    //   block [0x82D81840..0x82D81844)
	// 82D81840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81848 size=32
    let mut pc: u32 = 0x82D81848;
    'dispatch: loop {
        match pc {
            0x82D81848 => {
    //   block [0x82D81848..0x82D81868)
	// 82D81848: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8184C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81850: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81854: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81858: 396BA7BC  addi r11, r11, -0x5844
	ctx.r[11].s64 = ctx.r[11].s64 + -22596;
	// 82D8185C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81860: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D81864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81868 size=12
    let mut pc: u32 = 0x82D81868;
    'dispatch: loop {
        match pc {
            0x82D81868 => {
    //   block [0x82D81868..0x82D81874)
	// 82D81868: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8186C: 386BA7BC  addi r3, r11, -0x5844
	ctx.r[3].s64 = ctx.r[11].s64 + -22596;
	// 82D81870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81878 size=96
    let mut pc: u32 = 0x82D81878;
    'dispatch: loop {
        match pc {
            0x82D81878 => {
    //   block [0x82D81878..0x82D818C0)
	// 82D81878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8187C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D81884: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81888: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D8188C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D81890: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D81894: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82D81898: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D8189C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D818A0: 419A0020  beq cr6, 0x82d818c0
	if ctx.cr[6].eq {
	pc = 0x82D818C0; continue 'dispatch;
	}
	// 82D818A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D818A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D818AC: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D818B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D818B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D818B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D818BC: 4BFD3A0D  bl 0x82d552c8
	ctx.lr = 0x82D818C0;
	sub_82D552C8(ctx, base);
	pc = 0x82D818C0; continue 'dispatch;
            }
            0x82D818C0 => {
    //   block [0x82D818C0..0x82D818D8)
	// 82D818C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D818C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D818C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D818CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D818D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D818D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D818D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D818D8 size=340
    let mut pc: u32 = 0x82D818D8;
    'dispatch: loop {
        match pc {
            0x82D818D8 => {
    //   block [0x82D818D8..0x82D81920)
	// 82D818D8: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82D818DC: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82D818E0: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	// 82D818E4: 3D8082D8  lis r12, -0x7d28
	ctx.r[12].s64 = -2099773440;
	// 82D818E8: 398C18FC  addi r12, r12, 0x18fc
	ctx.r[12].s64 = ctx.r[12].s64 + 6396;
	// 82D818EC: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D818F0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D818F4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D818F8: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82D81920; continue 'dispatch;
		},
		1 => {
	pc = 0x82D81944; continue 'dispatch;
		},
		2 => {
	pc = 0x82D81968; continue 'dispatch;
		},
		3 => {
	pc = 0x82D81970; continue 'dispatch;
		},
		4 => {
	pc = 0x82D81994; continue 'dispatch;
		},
		5 => {
	pc = 0x82D819B8; continue 'dispatch;
		},
		6 => {
	pc = 0x82D819C0; continue 'dispatch;
		},
		7 => {
	pc = 0x82D819E4; continue 'dispatch;
		},
		8 => {
	pc = 0x82D81A08; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D818FC: 82D81920  lwz r22, 0x1920(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6432 as u32) ) } as u64;
	// 82D81900: 82D81944  lwz r22, 0x1944(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6468 as u32) ) } as u64;
	// 82D81904: 82D81968  lwz r22, 0x1968(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6504 as u32) ) } as u64;
	// 82D81908: 82D81970  lwz r22, 0x1970(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6512 as u32) ) } as u64;
	// 82D8190C: 82D81994  lwz r22, 0x1994(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6548 as u32) ) } as u64;
	// 82D81910: 82D819B8  lwz r22, 0x19b8(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6584 as u32) ) } as u64;
	// 82D81914: 82D819C0  lwz r22, 0x19c0(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6592 as u32) ) } as u64;
	// 82D81918: 82D819E4  lwz r22, 0x19e4(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6628 as u32) ) } as u64;
	// 82D8191C: 82D81A08  lwz r22, 0x1a08(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6664 as u32) ) } as u64;
            }
            0x82D81920 => {
    //   block [0x82D81920..0x82D81944)
	// 82D81920: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81924: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D81928: C00B0B24  lfs f0, 0xb24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D8192C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81930: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82D81934: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D81938: C1AB0B6C  lfs f13, 0xb6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2924 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D8193C: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82D81940: 4E800020  blr
	return;
            }
            0x82D81944 => {
    //   block [0x82D81944..0x82D81968)
	// 82D81944: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81948: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D8194C: C00B0B0C  lfs f0, 0xb0c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2828 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D81950: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81954: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82D81958: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D8195C: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81960: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82D81964: 4E800020  blr
	return;
            }
            0x82D81968 => {
    //   block [0x82D81968..0x82D81970)
	// 82D81968: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D8196C: 480000A0  b 0x82d81a0c
	pc = 0x82D81A0C; continue 'dispatch;
            }
            0x82D81970 => {
    //   block [0x82D81970..0x82D81994)
	// 82D81970: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81974: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D81978: C00B0B24  lfs f0, 0xb24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D8197C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81980: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82D81984: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D81988: C1AB0B6C  lfs f13, 0xb6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2924 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D8198C: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82D81990: 4E800020  blr
	return;
            }
            0x82D81994 => {
    //   block [0x82D81994..0x82D819B8)
	// 82D81994: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81998: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D8199C: C00B0B0C  lfs f0, 0xb0c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2828 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D819A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D819A4: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82D819A8: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D819AC: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D819B0: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82D819B4: 4E800020  blr
	return;
            }
            0x82D819B8 => {
    //   block [0x82D819B8..0x82D819C0)
	// 82D819B8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D819BC: 48000050  b 0x82d81a0c
	pc = 0x82D81A0C; continue 'dispatch;
            }
            0x82D819C0 => {
    //   block [0x82D819C0..0x82D819E4)
	// 82D819C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D819C4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82D819C8: C00B0B24  lfs f0, 0xb24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D819CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D819D0: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82D819D4: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D819D8: C1AB0B6C  lfs f13, 0xb6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2924 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D819DC: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82D819E0: 4E800020  blr
	return;
            }
            0x82D819E4 => {
    //   block [0x82D819E4..0x82D81A08)
	// 82D819E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D819E8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82D819EC: C00B0B0C  lfs f0, 0xb0c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2828 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D819F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D819F4: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82D819F8: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D819FC: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81A00: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82D81A04: 4E800020  blr
	return;
            }
            0x82D81A08 => {
    //   block [0x82D81A08..0x82D81A0C)
	// 82D81A08: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	pc = 0x82D81A0C; continue 'dispatch;
            }
            0x82D81A0C => {
    //   block [0x82D81A0C..0x82D81A2C)
	// 82D81A0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81A10: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D81A14: C00B0B6C  lfs f0, 0xb6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2924 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D81A18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81A1C: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82D81A20: C1AB0C80  lfs f13, 0xc80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3200 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81A24: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82D81A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D81A30 size=436
    let mut pc: u32 = 0x82D81A30;
    'dispatch: loop {
        match pc {
            0x82D81A30 => {
    //   block [0x82D81A30..0x82D81BE4)
	// 82D81A30: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82D81A34: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D81A38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81A3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81A40: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82D81A44: 38C00400  li r6, 0x400
	ctx.r[6].s64 = 1024;
	// 82D81A48: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82D81A4C: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D81A50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81A54: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82D81A58: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82D81A5C: 392BA8EC  addi r9, r11, -0x5714
	ctx.r[9].s64 = ctx.r[11].s64 + -22292;
	// 82D81A60: D001FFC8  stfs f0, -0x38(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 82D81A64: D001FFCC  stfs f0, -0x34(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 82D81A68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81A6C: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82D81A70: 388000FA  li r4, 0xfa
	ctx.r[4].s64 = 250;
	// 82D81A74: D001FFEC  stfs f0, -0x14(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82D81A78: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 82D81A7C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81A80: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82D81A84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D81A88: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81A8C: C1AB0E34  lfs f13, 0xe34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3636 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81A90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81A94: D1A1FFC4  stfs f13, -0x3c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82D81A98: C1890B0C  lfs f12, 0xb0c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2828 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D81A9C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81AA0: C1ABA8E4  lfs f13, -0x571c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22300 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81AA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81AA8: D1A1FFD0  stfs f13, -0x30(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82D81AAC: D1A1FFD4  stfs f13, -0x2c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82D81AB0: D1A1FFD8  stfs f13, -0x28(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82D81AB4: C0090C14  lfs f0, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D81AB8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81ABC: C1AB0B10  lfs f13, 0xb10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2832 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D81AC4: D1A1FFE0  stfs f13, -0x20(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82D81AC8: D1A1FFE4  stfs f13, -0x1c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82D81ACC: D1A1FFE8  stfs f13, -0x18(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82D81AD0: C1690C64  lfs f11, 0xc64(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3172 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D81AD4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82D81AD8: C149A8E0  lfs f10, -0x5720(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-22304 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D81ADC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82D81AE0: C129A8DC  lfs f9, -0x5724(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-22308 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D81AE4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81AE8: C1A90BF8  lfs f13, 0xbf8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3064 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81AEC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81AF0: C1090A94  lfs f8, 0xa94(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2708 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82D81AF4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81AF8: C0E90B80  lfs f7, 0xb80(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2944 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82D81AFC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81B00: C0C90BA0  lfs f6, 0xba0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2976 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82D81B04: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81B08: C0A90B8C  lfs f5, 0xb8c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2956 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82D81B0C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81B10: C0890BE8  lfs f4, 0xbe8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3048 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82D81B14: 3921FFC0  addi r9, r1, -0x40
	ctx.r[9].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D81BE8 size=68
    let mut pc: u32 = 0x82D81BE8;
    'dispatch: loop {
        match pc {
            0x82D81BE8 => {
    //   block [0x82D81BE8..0x82D81C2C)
	// 82D81BE8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D81BEC: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82D81BF0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D81BF4: C00A0C8C  lfs f0, 0xc8c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D81BF8: 39430040  addi r10, r3, 0x40
	ctx.r[10].s64 = ctx.r[3].s64 + 64;
	// 82D81BFC: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D81C00: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D81C30 size=64
    let mut pc: u32 = 0x82D81C30;
    'dispatch: loop {
        match pc {
            0x82D81C30 => {
    //   block [0x82D81C30..0x82D81C70)
	// 82D81C30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81C34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81C38: 396BA8EC  addi r11, r11, -0x5714
	ctx.r[11].s64 = ctx.r[11].s64 + -22292;
	// 82D81C3C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D81C40: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81C44: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D81C48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81C4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81C50: C1A30024  lfs f13, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D81C54: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D81C58: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82D81C5C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82D81C60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D81C64: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D81C68: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D81C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81CA0 size=44
    let mut pc: u32 = 0x82D81CA0;
    'dispatch: loop {
        match pc {
            0x82D81CA0 => {
    //   block [0x82D81CA0..0x82D81CCC)
	// 82D81CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81CA8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81CAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81CB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D81CB4: 4800001D  bl 0x82d81cd0
	ctx.lr = 0x82D81CB8;
	sub_82D81CD0(ctx, base);
	// 82D81CB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D81CBC: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82D81CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81CD0 size=104
    let mut pc: u32 = 0x82D81CD0;
    'dispatch: loop {
        match pc {
            0x82D81CD0 => {
    //   block [0x82D81CD0..0x82D81D38)
	// 82D81CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81CD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D81CDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81CE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D81CE4: 4BFFEB1D  bl 0x82d80800
	ctx.lr = 0x82D81CE8;
	sub_82D80800(ctx, base);
	// 82D81CE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81CEC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D81CF0: 392BA94C  addi r9, r11, -0x56b4
	ctx.r[9].s64 = ctx.r[11].s64 + -22196;
	// 82D81CF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D81CF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81CFC: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82D81D00: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82D81D04: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82D81D08: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82D81D0C: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82D81D10: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82D81D14: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D81D18: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82D81D1C: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82D81D20: 915F00C8  stw r10, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 82D81D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D81D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81D30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81D38 size=100
    let mut pc: u32 = 0x82D81D38;
    'dispatch: loop {
        match pc {
            0x82D81D38 => {
    //   block [0x82D81D38..0x82D81D80)
	// 82D81D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D81D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D81D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D81D50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D81D54: 4801DF85  bl 0x82d9fcd8
	ctx.lr = 0x82D81D58;
	sub_82D9FCD8(ctx, base);
	// 82D81D58: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D81D5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D81D60: 419A0020  beq cr6, 0x82d81d80
	if ctx.cr[6].eq {
	pc = 0x82D81D80; continue 'dispatch;
	}
	// 82D81D64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81D68: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D81D6C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82D81D70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D81D74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D81D78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D81D7C: 4BFD354D  bl 0x82d552c8
	ctx.lr = 0x82D81D80;
	sub_82D552C8(ctx, base);
	pc = 0x82D81D80; continue 'dispatch;
            }
            0x82D81D80 => {
    //   block [0x82D81D80..0x82D81D9C)
	// 82D81D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81D84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D81D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81D90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D81D94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81DA0 size=20
    let mut pc: u32 = 0x82D81DA0;
    'dispatch: loop {
        match pc {
            0x82D81DA0 => {
    //   block [0x82D81DA0..0x82D81DB4)
	// 82D81DA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81DA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81DA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81DAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81DB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81DB8 size=32
    let mut pc: u32 = 0x82D81DB8;
    'dispatch: loop {
        match pc {
            0x82D81DB8 => {
    //   block [0x82D81DB8..0x82D81DD8)
	// 82D81DB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81DBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81DC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81DC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81DC8: 396BAA5C  addi r11, r11, -0x55a4
	ctx.r[11].s64 = ctx.r[11].s64 + -21924;
	// 82D81DCC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81DD0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D81DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81DD8 size=12
    let mut pc: u32 = 0x82D81DD8;
    'dispatch: loop {
        match pc {
            0x82D81DD8 => {
    //   block [0x82D81DD8..0x82D81DE4)
	// 82D81DD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81DDC: 386BAA5C  addi r3, r11, -0x55a4
	ctx.r[3].s64 = ctx.r[11].s64 + -21924;
	// 82D81DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81DE8 size=4
    let mut pc: u32 = 0x82D81DE8;
    'dispatch: loop {
        match pc {
            0x82D81DE8 => {
    //   block [0x82D81DE8..0x82D81DEC)
	// 82D81DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81DF0 size=16
    let mut pc: u32 = 0x82D81DF0;
    'dispatch: loop {
        match pc {
            0x82D81DF0 => {
    //   block [0x82D81DF0..0x82D81E00)
	// 82D81DF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D81DF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81DF8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81DFC: 4801E8F4  b 0x82da06f0
	sub_82DA06F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81E00 size=4
    let mut pc: u32 = 0x82D81E00;
    'dispatch: loop {
        match pc {
            0x82D81E00 => {
    //   block [0x82D81E00..0x82D81E04)
	// 82D81E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81E08 size=20
    let mut pc: u32 = 0x82D81E08;
    'dispatch: loop {
        match pc {
            0x82D81E08 => {
    //   block [0x82D81E08..0x82D81E1C)
	// 82D81E08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81E0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81E10: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81E14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81E18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81E20 size=44
    let mut pc: u32 = 0x82D81E20;
    'dispatch: loop {
        match pc {
            0x82D81E20 => {
    //   block [0x82D81E20..0x82D81E4C)
	// 82D81E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81E28: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81E2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81E30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D81E34: 4801E8BD  bl 0x82da06f0
	ctx.lr = 0x82D81E38;
	sub_82DA06F0(ctx, base);
	// 82D81E38: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D81E3C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D81E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81E50 size=4
    let mut pc: u32 = 0x82D81E50;
    'dispatch: loop {
        match pc {
            0x82D81E50 => {
    //   block [0x82D81E50..0x82D81E54)
	// 82D81E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81E58 size=4
    let mut pc: u32 = 0x82D81E58;
    'dispatch: loop {
        match pc {
            0x82D81E58 => {
    //   block [0x82D81E58..0x82D81E5C)
	// 82D81E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81E60 size=20
    let mut pc: u32 = 0x82D81E60;
    'dispatch: loop {
        match pc {
            0x82D81E60 => {
    //   block [0x82D81E60..0x82D81E74)
	// 82D81E60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81E64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81E68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81E70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81E78 size=4
    let mut pc: u32 = 0x82D81E78;
    'dispatch: loop {
        match pc {
            0x82D81E78 => {
    //   block [0x82D81E78..0x82D81E7C)
	// 82D81E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81E80 size=32
    let mut pc: u32 = 0x82D81E80;
    'dispatch: loop {
        match pc {
            0x82D81E80 => {
    //   block [0x82D81E80..0x82D81EA0)
	// 82D81E80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81E84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81E88: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81E8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81E90: 396BAF0C  addi r11, r11, -0x50f4
	ctx.r[11].s64 = ctx.r[11].s64 + -20724;
	// 82D81E94: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81E98: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D81E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81EA0 size=12
    let mut pc: u32 = 0x82D81EA0;
    'dispatch: loop {
        match pc {
            0x82D81EA0 => {
    //   block [0x82D81EA0..0x82D81EAC)
	// 82D81EA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81EA4: 386BAF0C  addi r3, r11, -0x50f4
	ctx.r[3].s64 = ctx.r[11].s64 + -20724;
	// 82D81EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81EB0 size=100
    let mut pc: u32 = 0x82D81EB0;
    'dispatch: loop {
        match pc {
            0x82D81EB0 => {
    //   block [0x82D81EB0..0x82D81EF8)
	// 82D81EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D81EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D81EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D81EC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D81ECC: 4801F5F5  bl 0x82da14c0
	ctx.lr = 0x82D81ED0;
	sub_82DA14C0(ctx, base);
	// 82D81ED0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D81ED4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D81ED8: 419A0020  beq cr6, 0x82d81ef8
	if ctx.cr[6].eq {
	pc = 0x82D81EF8; continue 'dispatch;
	}
	// 82D81EDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81EE0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D81EE4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D81EE8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D81EEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D81EF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D81EF4: 4BFD33D5  bl 0x82d552c8
	ctx.lr = 0x82D81EF8;
	sub_82D552C8(ctx, base);
	pc = 0x82D81EF8; continue 'dispatch;
            }
            0x82D81EF8 => {
    //   block [0x82D81EF8..0x82D81F14)
	// 82D81EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81EFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D81F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81F08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D81F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81F18 size=20
    let mut pc: u32 = 0x82D81F18;
    'dispatch: loop {
        match pc {
            0x82D81F18 => {
    //   block [0x82D81F18..0x82D81F2C)
	// 82D81F18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81F1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81F20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81F24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81F28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81F30 size=32
    let mut pc: u32 = 0x82D81F30;
    'dispatch: loop {
        match pc {
            0x82D81F30 => {
    //   block [0x82D81F30..0x82D81F50)
	// 82D81F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81F34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81F38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81F3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81F40: 396BB13C  addi r11, r11, -0x4ec4
	ctx.r[11].s64 = ctx.r[11].s64 + -20164;
	// 82D81F44: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D81F48: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D81F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81F50 size=12
    let mut pc: u32 = 0x82D81F50;
    'dispatch: loop {
        match pc {
            0x82D81F50 => {
    //   block [0x82D81F50..0x82D81F5C)
	// 82D81F50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81F54: 386BB13C  addi r3, r11, -0x4ec4
	ctx.r[3].s64 = ctx.r[11].s64 + -20164;
	// 82D81F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D81F60 size=96
    let mut pc: u32 = 0x82D81F60;
    'dispatch: loop {
        match pc {
            0x82D81F60 => {
    //   block [0x82D81F60..0x82D81FA8)
	// 82D81F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D81F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D81F68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D81F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D81F70: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D81F74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D81F78: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D81F7C: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82D81F80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D81F84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D81F88: 419A0020  beq cr6, 0x82d81fa8
	if ctx.cr[6].eq {
	pc = 0x82D81FA8; continue 'dispatch;
	}
	// 82D81F8C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81F90: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D81F94: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82D81F98: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D81F9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D81FA0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D81FA4: 4BFD3325  bl 0x82d552c8
	ctx.lr = 0x82D81FA8;
	sub_82D552C8(ctx, base);
	pc = 0x82D81FA8; continue 'dispatch;
            }
            0x82D81FA8 => {
    //   block [0x82D81FA8..0x82D81FC0)
	// 82D81FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D81FAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D81FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D81FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D81FB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D81FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81FC0 size=4
    let mut pc: u32 = 0x82D81FC0;
    'dispatch: loop {
        match pc {
            0x82D81FC0 => {
    //   block [0x82D81FC0..0x82D81FC4)
	// 82D81FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81FC8 size=20
    let mut pc: u32 = 0x82D81FC8;
    'dispatch: loop {
        match pc {
            0x82D81FC8 => {
    //   block [0x82D81FC8..0x82D81FDC)
	// 82D81FC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81FCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D81FD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D81FD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D81FD8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81FE0 size=4
    let mut pc: u32 = 0x82D81FE0;
    'dispatch: loop {
        match pc {
            0x82D81FE0 => {
    //   block [0x82D81FE0..0x82D81FE4)
	// 82D81FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D81FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D81FE8 size=32
    let mut pc: u32 = 0x82D81FE8;
    'dispatch: loop {
        match pc {
            0x82D81FE8 => {
    //   block [0x82D81FE8..0x82D82008)
	// 82D81FE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D81FEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D81FF0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D81FF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D81FF8: 396BB214  addi r11, r11, -0x4dec
	ctx.r[11].s64 = ctx.r[11].s64 + -19948;
	// 82D81FFC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82000: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82008 size=12
    let mut pc: u32 = 0x82D82008;
    'dispatch: loop {
        match pc {
            0x82D82008 => {
    //   block [0x82D82008..0x82D82014)
	// 82D82008: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8200C: 386BB214  addi r3, r11, -0x4dec
	ctx.r[3].s64 = ctx.r[11].s64 + -19948;
	// 82D82010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82018 size=20
    let mut pc: u32 = 0x82D82018;
    'dispatch: loop {
        match pc {
            0x82D82018 => {
    //   block [0x82D82018..0x82D8202C)
	// 82D82018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8201C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82020: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82028: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82030 size=16
    let mut pc: u32 = 0x82D82030;
    'dispatch: loop {
        match pc {
            0x82D82030 => {
    //   block [0x82D82030..0x82D82040)
	// 82D82030: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D82034: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82038: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D8203C: 4802003C  b 0x82da2078
	sub_82DA2078(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82040 size=4
    let mut pc: u32 = 0x82D82040;
    'dispatch: loop {
        match pc {
            0x82D82040 => {
    //   block [0x82D82040..0x82D82044)
	// 82D82040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82048 size=44
    let mut pc: u32 = 0x82D82048;
    'dispatch: loop {
        match pc {
            0x82D82048 => {
    //   block [0x82D82048..0x82D82074)
	// 82D82048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8204C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82050: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82054: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82058: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D8205C: 4802001D  bl 0x82da2078
	ctx.lr = 0x82D82060;
	sub_82DA2078(ctx, base);
	// 82D82060: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D82064: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D82068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8206C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82078 size=4
    let mut pc: u32 = 0x82D82078;
    'dispatch: loop {
        match pc {
            0x82D82078 => {
    //   block [0x82D82078..0x82D8207C)
	// 82D82078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82080 size=20
    let mut pc: u32 = 0x82D82080;
    'dispatch: loop {
        match pc {
            0x82D82080 => {
    //   block [0x82D82080..0x82D82094)
	// 82D82080: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82084: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82088: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8208C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82090: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82098 size=4
    let mut pc: u32 = 0x82D82098;
    'dispatch: loop {
        match pc {
            0x82D82098 => {
    //   block [0x82D82098..0x82D8209C)
	// 82D82098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D820A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D820A0 size=52
    let mut pc: u32 = 0x82D820A0;
    'dispatch: loop {
        match pc {
            0x82D820A0 => {
    //   block [0x82D820A0..0x82D820D4)
	// 82D820A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D820A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D820A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D820AC: 89230116  lbz r9, 0x116(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(278 as u32) ) } as u64;
	// 82D820B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D820B4: 396BB584  addi r11, r11, -0x4a7c
	ctx.r[11].s64 = ctx.r[11].s64 + -19068;
	// 82D820B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D820BC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D820C0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D820C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D820C8: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82D820CC: 99630116  stb r11, 0x116(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(278 as u32), ctx.r[11].u8 ) };
	// 82D820D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D820D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D820D8 size=12
    let mut pc: u32 = 0x82D820D8;
    'dispatch: loop {
        match pc {
            0x82D820D8 => {
    //   block [0x82D820D8..0x82D820E4)
	// 82D820D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D820DC: 386BB584  addi r3, r11, -0x4a7c
	ctx.r[3].s64 = ctx.r[11].s64 + -19068;
	// 82D820E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D820E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D820E8 size=100
    let mut pc: u32 = 0x82D820E8;
    'dispatch: loop {
        match pc {
            0x82D820E8 => {
    //   block [0x82D820E8..0x82D82130)
	// 82D820E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D820EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D820F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D820F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D820F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D820FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82100: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D82104: 48004C95  bl 0x82d86d98
	ctx.lr = 0x82D82108;
	sub_82D86D98(ctx, base);
	// 82D82108: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D8210C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82110: 419A0020  beq cr6, 0x82d82130
	if ctx.cr[6].eq {
	pc = 0x82D82130; continue 'dispatch;
	}
	// 82D82114: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82118: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D8211C: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D82120: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82124: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D82128: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8212C: 4BFD319D  bl 0x82d552c8
	ctx.lr = 0x82D82130;
	sub_82D552C8(ctx, base);
	pc = 0x82D82130; continue 'dispatch;
            }
            0x82D82130 => {
    //   block [0x82D82130..0x82D8214C)
	// 82D82130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D82134: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8213C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82180 size=44
    let mut pc: u32 = 0x82D82180;
    'dispatch: loop {
        match pc {
            0x82D82180 => {
    //   block [0x82D82180..0x82D821AC)
	// 82D82180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82188: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8218C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82190: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D82194: 4800001D  bl 0x82d821b0
	ctx.lr = 0x82D82198;
	sub_82D821B0(ctx, base);
	// 82D82198: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D8219C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82D821A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D821A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D821A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D821B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D821B0 size=104
    let mut pc: u32 = 0x82D821B0;
    'dispatch: loop {
        match pc {
            0x82D821B0 => {
    //   block [0x82D821B0..0x82D82218)
	// 82D821B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D821B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D821B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D821BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D821C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D821C4: 4BFFE63D  bl 0x82d80800
	ctx.lr = 0x82D821C8;
	sub_82D80800(ctx, base);
	// 82D821C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D821CC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D821D0: 392B42BC  addi r9, r11, 0x42bc
	ctx.r[9].s64 = ctx.r[11].s64 + 17084;
	// 82D821D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D821D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D821DC: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82D821E0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82D821E4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82D821E8: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82D821EC: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82D821F0: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82D821F4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D821F8: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82D821FC: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82D82200: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 82D82204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D82208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8220C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82210: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82218 size=100
    let mut pc: u32 = 0x82D82218;
    'dispatch: loop {
        match pc {
            0x82D82218 => {
    //   block [0x82D82218..0x82D82260)
	// 82D82218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D82224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D82228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8222C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82230: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D82234: 480206F5  bl 0x82da2928
	ctx.lr = 0x82D82238;
	sub_82DA2928(ctx, base);
	// 82D82238: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D8223C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82240: 419A0020  beq cr6, 0x82d82260
	if ctx.cr[6].eq {
	pc = 0x82D82260; continue 'dispatch;
	}
	// 82D82244: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82248: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D8224C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82D82250: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82254: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D82258: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8225C: 4BFD306D  bl 0x82d552c8
	ctx.lr = 0x82D82260;
	sub_82D552C8(ctx, base);
	pc = 0x82D82260; continue 'dispatch;
            }
            0x82D82260 => {
    //   block [0x82D82260..0x82D8227C)
	// 82D82260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D82264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8226C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82270: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82280 size=4
    let mut pc: u32 = 0x82D82280;
    'dispatch: loop {
        match pc {
            0x82D82280 => {
    //   block [0x82D82280..0x82D82284)
	// 82D82280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82288 size=20
    let mut pc: u32 = 0x82D82288;
    'dispatch: loop {
        match pc {
            0x82D82288 => {
    //   block [0x82D82288..0x82D8229C)
	// 82D82288: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8228C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82290: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82298: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D822A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D822A0 size=4
    let mut pc: u32 = 0x82D822A0;
    'dispatch: loop {
        match pc {
            0x82D822A0 => {
    //   block [0x82D822A0..0x82D822A4)
	// 82D822A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D822A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D822A8 size=52
    let mut pc: u32 = 0x82D822A8;
    'dispatch: loop {
        match pc {
            0x82D822A8 => {
    //   block [0x82D822A8..0x82D822DC)
	// 82D822A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D822AC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D822B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D822B4: 8923009A  lbz r9, 0x9a(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(154 as u32) ) } as u64;
	// 82D822B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D822BC: 396BB6E4  addi r11, r11, -0x491c
	ctx.r[11].s64 = ctx.r[11].s64 + -18716;
	// 82D822C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D822C4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D822C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D822CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D822D0: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82D822D4: 9963009A  stb r11, 0x9a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 82D822D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D822E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D822E0 size=12
    let mut pc: u32 = 0x82D822E0;
    'dispatch: loop {
        match pc {
            0x82D822E0 => {
    //   block [0x82D822E0..0x82D822EC)
	// 82D822E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D822E4: 386BB6E4  addi r3, r11, -0x491c
	ctx.r[3].s64 = ctx.r[11].s64 + -18716;
	// 82D822E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D822F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D822F0 size=4
    let mut pc: u32 = 0x82D822F0;
    'dispatch: loop {
        match pc {
            0x82D822F0 => {
    //   block [0x82D822F0..0x82D822F4)
	// 82D822F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D822F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D822F8 size=4
    let mut pc: u32 = 0x82D822F8;
    'dispatch: loop {
        match pc {
            0x82D822F8 => {
    //   block [0x82D822F8..0x82D822FC)
	// 82D822F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82300 size=20
    let mut pc: u32 = 0x82D82300;
    'dispatch: loop {
        match pc {
            0x82D82300 => {
    //   block [0x82D82300..0x82D82314)
	// 82D82300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82304: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82308: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8230C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82310: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82318 size=32
    let mut pc: u32 = 0x82D82318;
    'dispatch: loop {
        match pc {
            0x82D82318 => {
    //   block [0x82D82318..0x82D82338)
	// 82D82318: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8231C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82320: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82328: 396BB8F0  addi r11, r11, -0x4710
	ctx.r[11].s64 = ctx.r[11].s64 + -18192;
	// 82D8232C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82330: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82338 size=12
    let mut pc: u32 = 0x82D82338;
    'dispatch: loop {
        match pc {
            0x82D82338 => {
    //   block [0x82D82338..0x82D82344)
	// 82D82338: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8233C: 386BB8F0  addi r3, r11, -0x4710
	ctx.r[3].s64 = ctx.r[11].s64 + -18192;
	// 82D82340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82348 size=20
    let mut pc: u32 = 0x82D82348;
    'dispatch: loop {
        match pc {
            0x82D82348 => {
    //   block [0x82D82348..0x82D8235C)
	// 82D82348: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8234C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82350: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82358: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82360 size=68
    let mut pc: u32 = 0x82D82360;
    'dispatch: loop {
        match pc {
            0x82D82360 => {
    //   block [0x82D82360..0x82D82390)
	// 82D82360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D8236C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82374: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D82378: 419A0018  beq cr6, 0x82d82390
	if ctx.cr[6].eq {
	pc = 0x82D82390; continue 'dispatch;
	}
	// 82D8237C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D82380: 480037B1  bl 0x82d85b30
	ctx.lr = 0x82D82384;
	sub_82D85B30(ctx, base);
	// 82D82384: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82388: 396BB944  addi r11, r11, -0x46bc
	ctx.r[11].s64 = ctx.r[11].s64 + -18108;
	// 82D8238C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82D82390; continue 'dispatch;
            }
            0x82D82390 => {
    //   block [0x82D82390..0x82D823A4)
	// 82D82390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D82394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D8239C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D823A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D823A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D823A8 size=48
    let mut pc: u32 = 0x82D823A8;
    'dispatch: loop {
        match pc {
            0x82D823A8 => {
    //   block [0x82D823A8..0x82D823D8)
	// 82D823A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D823AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D823B0: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D823B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D823B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D823BC: 48003775  bl 0x82d85b30
	ctx.lr = 0x82D823C0;
	sub_82D85B30(ctx, base);
	// 82D823C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D823C4: 386BB944  addi r3, r11, -0x46bc
	ctx.r[3].s64 = ctx.r[11].s64 + -18108;
	// 82D823C8: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 82D823CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D823D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D823D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D823D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D823D8 size=100
    let mut pc: u32 = 0x82D823D8;
    'dispatch: loop {
        match pc {
            0x82D823D8 => {
    //   block [0x82D823D8..0x82D82420)
	// 82D823D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D823DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D823E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D823E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D823E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D823EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D823F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D823F4: 48001A35  bl 0x82d83e28
	ctx.lr = 0x82D823F8;
	sub_82D83E28(ctx, base);
	// 82D823F8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D823FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82400: 419A0020  beq cr6, 0x82d82420
	if ctx.cr[6].eq {
	pc = 0x82D82420; continue 'dispatch;
	}
	// 82D82404: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82408: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D8240C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82D82410: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82414: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D82418: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8241C: 4BFD2EAD  bl 0x82d552c8
	ctx.lr = 0x82D82420;
	sub_82D552C8(ctx, base);
	pc = 0x82D82420; continue 'dispatch;
            }
            0x82D82420 => {
    //   block [0x82D82420..0x82D8243C)
	// 82D82420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D82424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8242C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82430: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82440 size=20
    let mut pc: u32 = 0x82D82440;
    'dispatch: loop {
        match pc {
            0x82D82440 => {
    //   block [0x82D82440..0x82D82454)
	// 82D82440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82448: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8244C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82450: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82458 size=32
    let mut pc: u32 = 0x82D82458;
    'dispatch: loop {
        match pc {
            0x82D82458 => {
    //   block [0x82D82458..0x82D82478)
	// 82D82458: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8245C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82460: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82464: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82468: 396BB974  addi r11, r11, -0x468c
	ctx.r[11].s64 = ctx.r[11].s64 + -18060;
	// 82D8246C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82470: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82478 size=12
    let mut pc: u32 = 0x82D82478;
    'dispatch: loop {
        match pc {
            0x82D82478 => {
    //   block [0x82D82478..0x82D82484)
	// 82D82478: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8247C: 386BB974  addi r3, r11, -0x468c
	ctx.r[3].s64 = ctx.r[11].s64 + -18060;
	// 82D82480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82488 size=4
    let mut pc: u32 = 0x82D82488;
    'dispatch: loop {
        match pc {
            0x82D82488 => {
    //   block [0x82D82488..0x82D8248C)
	// 82D82488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82490 size=20
    let mut pc: u32 = 0x82D82490;
    'dispatch: loop {
        match pc {
            0x82D82490 => {
    //   block [0x82D82490..0x82D824A4)
	// 82D82490: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82494: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82498: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8249C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D824A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D824A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D824A8 size=4
    let mut pc: u32 = 0x82D824A8;
    'dispatch: loop {
        match pc {
            0x82D824A8 => {
    //   block [0x82D824A8..0x82D824AC)
	// 82D824A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D824B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D824B0 size=32
    let mut pc: u32 = 0x82D824B0;
    'dispatch: loop {
        match pc {
            0x82D824B0 => {
    //   block [0x82D824B0..0x82D824D0)
	// 82D824B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D824B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D824B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D824BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D824C0: 396BBA38  addi r11, r11, -0x45c8
	ctx.r[11].s64 = ctx.r[11].s64 + -17864;
	// 82D824C4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D824C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D824CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D824D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D824D0 size=12
    let mut pc: u32 = 0x82D824D0;
    'dispatch: loop {
        match pc {
            0x82D824D0 => {
    //   block [0x82D824D0..0x82D824DC)
	// 82D824D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D824D4: 386BBA38  addi r3, r11, -0x45c8
	ctx.r[3].s64 = ctx.r[11].s64 + -17864;
	// 82D824D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D824E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D824E0 size=20
    let mut pc: u32 = 0x82D824E0;
    'dispatch: loop {
        match pc {
            0x82D824E0 => {
    //   block [0x82D824E0..0x82D824F4)
	// 82D824E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D824E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D824E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D824EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D824F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D824F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D824F8 size=32
    let mut pc: u32 = 0x82D824F8;
    'dispatch: loop {
        match pc {
            0x82D824F8 => {
    //   block [0x82D824F8..0x82D82518)
	// 82D824F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D824FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82500: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82504: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82508: 396BBB14  addi r11, r11, -0x44ec
	ctx.r[11].s64 = ctx.r[11].s64 + -17644;
	// 82D8250C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82510: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82518 size=12
    let mut pc: u32 = 0x82D82518;
    'dispatch: loop {
        match pc {
            0x82D82518 => {
    //   block [0x82D82518..0x82D82524)
	// 82D82518: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8251C: 386BBB14  addi r3, r11, -0x44ec
	ctx.r[3].s64 = ctx.r[11].s64 + -17644;
	// 82D82520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82528 size=100
    let mut pc: u32 = 0x82D82528;
    'dispatch: loop {
        match pc {
            0x82D82528 => {
    //   block [0x82D82528..0x82D82570)
	// 82D82528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D82534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D82538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8253C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82540: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D82544: 48004B45  bl 0x82d87088
	ctx.lr = 0x82D82548;
	sub_82D87088(ctx, base);
	// 82D82548: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D8254C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82550: 419A0020  beq cr6, 0x82d82570
	if ctx.cr[6].eq {
	pc = 0x82D82570; continue 'dispatch;
	}
	// 82D82554: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82558: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D8255C: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D82560: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82564: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D82568: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8256C: 4BFD2D5D  bl 0x82d552c8
	ctx.lr = 0x82D82570;
	sub_82D552C8(ctx, base);
	pc = 0x82D82570; continue 'dispatch;
            }
            0x82D82570 => {
    //   block [0x82D82570..0x82D8258C)
	// 82D82570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D82574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8257C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82590 size=100
    let mut pc: u32 = 0x82D82590;
    'dispatch: loop {
        match pc {
            0x82D82590 => {
    //   block [0x82D82590..0x82D825D8)
	// 82D82590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D8259C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D825A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D825A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D825A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D825AC: 48009855  bl 0x82d8be00
	ctx.lr = 0x82D825B0;
	sub_82D8BE00(ctx, base);
	// 82D825B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D825B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D825B8: 419A0020  beq cr6, 0x82d825d8
	if ctx.cr[6].eq {
	pc = 0x82D825D8; continue 'dispatch;
	}
	// 82D825BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D825C0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D825C4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82D825C8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D825CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D825D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D825D4: 4BFD2CF5  bl 0x82d552c8
	ctx.lr = 0x82D825D8;
	sub_82D552C8(ctx, base);
	pc = 0x82D825D8; continue 'dispatch;
            }
            0x82D825D8 => {
    //   block [0x82D825D8..0x82D825F4)
	// 82D825D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D825DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D825E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D825E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D825E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D825EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D825F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D825F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D825F8 size=4
    let mut pc: u32 = 0x82D825F8;
    'dispatch: loop {
        match pc {
            0x82D825F8 => {
    //   block [0x82D825F8..0x82D825FC)
	// 82D825F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82630 size=44
    let mut pc: u32 = 0x82D82630;
    'dispatch: loop {
        match pc {
            0x82D82630 => {
    //   block [0x82D82630..0x82D8265C)
	// 82D82630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82638: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8263C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82640: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D82644: 4800001D  bl 0x82d82660
	ctx.lr = 0x82D82648;
	sub_82D82660(ctx, base);
	// 82D82648: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D8264C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82D82650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82660 size=104
    let mut pc: u32 = 0x82D82660;
    'dispatch: loop {
        match pc {
            0x82D82660 => {
    //   block [0x82D82660..0x82D826C8)
	// 82D82660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82668: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D8266C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82674: 4BFFE18D  bl 0x82d80800
	ctx.lr = 0x82D82678;
	sub_82D80800(ctx, base);
	// 82D82678: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D8267C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D82680: 392B4C5C  addi r9, r11, 0x4c5c
	ctx.r[9].s64 = ctx.r[11].s64 + 19548;
	// 82D82684: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D82688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D8268C: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82D82690: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82D82694: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82D82698: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82D8269C: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82D826A0: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82D826A4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D826A8: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82D826AC: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82D826B0: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 82D826B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D826B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D826BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D826C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D826C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D826C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D826C8 size=96
    let mut pc: u32 = 0x82D826C8;
    'dispatch: loop {
        match pc {
            0x82D826C8 => {
    //   block [0x82D826C8..0x82D82710)
	// 82D826C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D826CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D826D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D826D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D826D8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D826DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D826E0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D826E4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82D826E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D826EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D826F0: 419A0020  beq cr6, 0x82d82710
	if ctx.cr[6].eq {
	pc = 0x82D82710; continue 'dispatch;
	}
	// 82D826F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D826F8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D826FC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82D82700: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82704: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D82708: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8270C: 4BFD2BBD  bl 0x82d552c8
	ctx.lr = 0x82D82710;
	sub_82D552C8(ctx, base);
	pc = 0x82D82710; continue 'dispatch;
            }
            0x82D82710 => {
    //   block [0x82D82710..0x82D82728)
	// 82D82710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D82714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D82718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8271C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82728 size=32
    let mut pc: u32 = 0x82D82728;
    'dispatch: loop {
        match pc {
            0x82D82728 => {
    //   block [0x82D82728..0x82D82748)
	// 82D82728: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8272C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82730: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82734: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82738: 396BBBEC  addi r11, r11, -0x4414
	ctx.r[11].s64 = ctx.r[11].s64 + -17428;
	// 82D8273C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82740: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82748 size=20
    let mut pc: u32 = 0x82D82748;
    'dispatch: loop {
        match pc {
            0x82D82748 => {
    //   block [0x82D82748..0x82D8275C)
	// 82D82748: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8274C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82750: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82758: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82760 size=12
    let mut pc: u32 = 0x82D82760;
    'dispatch: loop {
        match pc {
            0x82D82760 => {
    //   block [0x82D82760..0x82D8276C)
	// 82D82760: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82764: 386BBBEC  addi r3, r11, -0x4414
	ctx.r[3].s64 = ctx.r[11].s64 + -17428;
	// 82D82768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82770 size=20
    let mut pc: u32 = 0x82D82770;
    'dispatch: loop {
        match pc {
            0x82D82770 => {
    //   block [0x82D82770..0x82D82784)
	// 82D82770: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82774: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82778: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8277C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82780: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82788 size=32
    let mut pc: u32 = 0x82D82788;
    'dispatch: loop {
        match pc {
            0x82D82788 => {
    //   block [0x82D82788..0x82D827A8)
	// 82D82788: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8278C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82790: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82794: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82798: 396BBDE4  addi r11, r11, -0x421c
	ctx.r[11].s64 = ctx.r[11].s64 + -16924;
	// 82D8279C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D827A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D827A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D827A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D827A8 size=12
    let mut pc: u32 = 0x82D827A8;
    'dispatch: loop {
        match pc {
            0x82D827A8 => {
    //   block [0x82D827A8..0x82D827B4)
	// 82D827A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D827AC: 386BBDE4  addi r3, r11, -0x421c
	ctx.r[3].s64 = ctx.r[11].s64 + -16924;
	// 82D827B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D827B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D827B8 size=100
    let mut pc: u32 = 0x82D827B8;
    'dispatch: loop {
        match pc {
            0x82D827B8 => {
    //   block [0x82D827B8..0x82D82800)
	// 82D827B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D827BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D827C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D827C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D827C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D827CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D827D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D827D4: 4800B0A5  bl 0x82d8d878
	ctx.lr = 0x82D827D8;
	sub_82D8D878(ctx, base);
	// 82D827D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D827DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D827E0: 419A0020  beq cr6, 0x82d82800
	if ctx.cr[6].eq {
	pc = 0x82D82800; continue 'dispatch;
	}
	// 82D827E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D827E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D827EC: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82D827F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D827F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D827F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D827FC: 4BFD2ACD  bl 0x82d552c8
	ctx.lr = 0x82D82800;
	sub_82D552C8(ctx, base);
	pc = 0x82D82800; continue 'dispatch;
            }
            0x82D82800 => {
    //   block [0x82D82800..0x82D8281C)
	// 82D82800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D82804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8280C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82810: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82820 size=20
    let mut pc: u32 = 0x82D82820;
    'dispatch: loop {
        match pc {
            0x82D82820 => {
    //   block [0x82D82820..0x82D82834)
	// 82D82820: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82824: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82828: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8282C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82830: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82838 size=32
    let mut pc: u32 = 0x82D82838;
    'dispatch: loop {
        match pc {
            0x82D82838 => {
    //   block [0x82D82838..0x82D82858)
	// 82D82838: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8283C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82840: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82844: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82848: 396BBAF4  addi r11, r11, -0x450c
	ctx.r[11].s64 = ctx.r[11].s64 + -17676;
	// 82D8284C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82850: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82858 size=12
    let mut pc: u32 = 0x82D82858;
    'dispatch: loop {
        match pc {
            0x82D82858 => {
    //   block [0x82D82858..0x82D82864)
	// 82D82858: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8285C: 386BBAF4  addi r3, r11, -0x450c
	ctx.r[3].s64 = ctx.r[11].s64 + -17676;
	// 82D82860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82868 size=4
    let mut pc: u32 = 0x82D82868;
    'dispatch: loop {
        match pc {
            0x82D82868 => {
    //   block [0x82D82868..0x82D8286C)
	// 82D82868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82870 size=20
    let mut pc: u32 = 0x82D82870;
    'dispatch: loop {
        match pc {
            0x82D82870 => {
    //   block [0x82D82870..0x82D82884)
	// 82D82870: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82874: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82878: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8287C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82880: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82888 size=32
    let mut pc: u32 = 0x82D82888;
    'dispatch: loop {
        match pc {
            0x82D82888 => {
    //   block [0x82D82888..0x82D828A8)
	// 82D82888: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8288C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82890: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82894: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82898: 396BC234  addi r11, r11, -0x3dcc
	ctx.r[11].s64 = ctx.r[11].s64 + -15820;
	// 82D8289C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D828A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D828A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D828A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D828A8 size=12
    let mut pc: u32 = 0x82D828A8;
    'dispatch: loop {
        match pc {
            0x82D828A8 => {
    //   block [0x82D828A8..0x82D828B4)
	// 82D828A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D828AC: 386BC234  addi r3, r11, -0x3dcc
	ctx.r[3].s64 = ctx.r[11].s64 + -15820;
	// 82D828B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D828B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D828B8 size=96
    let mut pc: u32 = 0x82D828B8;
    'dispatch: loop {
        match pc {
            0x82D828B8 => {
    //   block [0x82D828B8..0x82D82900)
	// 82D828B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D828BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D828C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D828C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D828C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D828CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D828D0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D828D4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82D828D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D828DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D828E0: 419A0020  beq cr6, 0x82d82900
	if ctx.cr[6].eq {
	pc = 0x82D82900; continue 'dispatch;
	}
	// 82D828E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D828E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D828EC: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 82D828F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D828F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D828F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D828FC: 4BFD29CD  bl 0x82d552c8
	ctx.lr = 0x82D82900;
	sub_82D552C8(ctx, base);
	pc = 0x82D82900; continue 'dispatch;
            }
            0x82D82900 => {
    //   block [0x82D82900..0x82D82918)
	// 82D82900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D82904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D82908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8290C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82918 size=20
    let mut pc: u32 = 0x82D82918;
    'dispatch: loop {
        match pc {
            0x82D82918 => {
    //   block [0x82D82918..0x82D8292C)
	// 82D82918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8291C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82920: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82928: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82930 size=32
    let mut pc: u32 = 0x82D82930;
    'dispatch: loop {
        match pc {
            0x82D82930 => {
    //   block [0x82D82930..0x82D82950)
	// 82D82930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82934: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82938: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8293C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82940: 396BC29C  addi r11, r11, -0x3d64
	ctx.r[11].s64 = ctx.r[11].s64 + -15716;
	// 82D82944: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82948: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D8294C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82950 size=12
    let mut pc: u32 = 0x82D82950;
    'dispatch: loop {
        match pc {
            0x82D82950 => {
    //   block [0x82D82950..0x82D8295C)
	// 82D82950: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82954: 386BC29C  addi r3, r11, -0x3d64
	ctx.r[3].s64 = ctx.r[11].s64 + -15716;
	// 82D82958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82960 size=20
    let mut pc: u32 = 0x82D82960;
    'dispatch: loop {
        match pc {
            0x82D82960 => {
    //   block [0x82D82960..0x82D82974)
	// 82D82960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82964: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82968: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8296C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82970: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82978 size=32
    let mut pc: u32 = 0x82D82978;
    'dispatch: loop {
        match pc {
            0x82D82978 => {
    //   block [0x82D82978..0x82D82998)
	// 82D82978: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8297C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82980: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82988: 396BC314  addi r11, r11, -0x3cec
	ctx.r[11].s64 = ctx.r[11].s64 + -15596;
	// 82D8298C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82990: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82998 size=12
    let mut pc: u32 = 0x82D82998;
    'dispatch: loop {
        match pc {
            0x82D82998 => {
    //   block [0x82D82998..0x82D829A4)
	// 82D82998: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D8299C: 386BC314  addi r3, r11, -0x3cec
	ctx.r[3].s64 = ctx.r[11].s64 + -15596;
	// 82D829A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D829A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D829A8 size=20
    let mut pc: u32 = 0x82D829A8;
    'dispatch: loop {
        match pc {
            0x82D829A8 => {
    //   block [0x82D829A8..0x82D829BC)
	// 82D829A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D829AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D829B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D829B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D829B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D829C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D829C0 size=32
    let mut pc: u32 = 0x82D829C0;
    'dispatch: loop {
        match pc {
            0x82D829C0 => {
    //   block [0x82D829C0..0x82D829E0)
	// 82D829C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D829C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D829C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D829CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D829D0: 396BC39C  addi r11, r11, -0x3c64
	ctx.r[11].s64 = ctx.r[11].s64 + -15460;
	// 82D829D4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D829D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D829DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D829E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D829E0 size=12
    let mut pc: u32 = 0x82D829E0;
    'dispatch: loop {
        match pc {
            0x82D829E0 => {
    //   block [0x82D829E0..0x82D829EC)
	// 82D829E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D829E4: 386BC39C  addi r3, r11, -0x3c64
	ctx.r[3].s64 = ctx.r[11].s64 + -15460;
	// 82D829E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D829F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D829F0 size=20
    let mut pc: u32 = 0x82D829F0;
    'dispatch: loop {
        match pc {
            0x82D829F0 => {
    //   block [0x82D829F0..0x82D82A04)
	// 82D829F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D829F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D829F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D829FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82A00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82A08 size=32
    let mut pc: u32 = 0x82D82A08;
    'dispatch: loop {
        match pc {
            0x82D82A08 => {
    //   block [0x82D82A08..0x82D82A28)
	// 82D82A08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82A0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82A10: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82A14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82A18: 396BC3D4  addi r11, r11, -0x3c2c
	ctx.r[11].s64 = ctx.r[11].s64 + -15404;
	// 82D82A1C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82A20: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82A28 size=12
    let mut pc: u32 = 0x82D82A28;
    'dispatch: loop {
        match pc {
            0x82D82A28 => {
    //   block [0x82D82A28..0x82D82A34)
	// 82D82A28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82A2C: 386BC3D4  addi r3, r11, -0x3c2c
	ctx.r[3].s64 = ctx.r[11].s64 + -15404;
	// 82D82A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82A38 size=4
    let mut pc: u32 = 0x82D82A38;
    'dispatch: loop {
        match pc {
            0x82D82A38 => {
    //   block [0x82D82A38..0x82D82A3C)
	// 82D82A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82A40 size=16
    let mut pc: u32 = 0x82D82A40;
    'dispatch: loop {
        match pc {
            0x82D82A40 => {
    //   block [0x82D82A40..0x82D82A50)
	// 82D82A40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D82A44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82A48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82A4C: 48008D0C  b 0x82d8b758
	sub_82D8B758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82A50 size=4
    let mut pc: u32 = 0x82D82A50;
    'dispatch: loop {
        match pc {
            0x82D82A50 => {
    //   block [0x82D82A50..0x82D82A54)
	// 82D82A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82A58 size=20
    let mut pc: u32 = 0x82D82A58;
    'dispatch: loop {
        match pc {
            0x82D82A58 => {
    //   block [0x82D82A58..0x82D82A6C)
	// 82D82A58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82A5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82A60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82A64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82A68: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82A70 size=44
    let mut pc: u32 = 0x82D82A70;
    'dispatch: loop {
        match pc {
            0x82D82A70 => {
    //   block [0x82D82A70..0x82D82A9C)
	// 82D82A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82A78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82A7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82A80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D82A84: 48008CD5  bl 0x82d8b758
	ctx.lr = 0x82D82A88;
	sub_82D8B758(ctx, base);
	// 82D82A88: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D82A8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D82A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82AA0 size=20
    let mut pc: u32 = 0x82D82AA0;
    'dispatch: loop {
        match pc {
            0x82D82AA0 => {
    //   block [0x82D82AA0..0x82D82AB4)
	// 82D82AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82AA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82AA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82AAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82AB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82AB8 size=32
    let mut pc: u32 = 0x82D82AB8;
    'dispatch: loop {
        match pc {
            0x82D82AB8 => {
    //   block [0x82D82AB8..0x82D82AD8)
	// 82D82AB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82ABC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82AC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82AC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82AC8: 396BC564  addi r11, r11, -0x3a9c
	ctx.r[11].s64 = ctx.r[11].s64 + -15004;
	// 82D82ACC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82AD0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82AD8 size=12
    let mut pc: u32 = 0x82D82AD8;
    'dispatch: loop {
        match pc {
            0x82D82AD8 => {
    //   block [0x82D82AD8..0x82D82AE4)
	// 82D82AD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82ADC: 386BC564  addi r3, r11, -0x3a9c
	ctx.r[3].s64 = ctx.r[11].s64 + -15004;
	// 82D82AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82AE8 size=4
    let mut pc: u32 = 0x82D82AE8;
    'dispatch: loop {
        match pc {
            0x82D82AE8 => {
    //   block [0x82D82AE8..0x82D82AEC)
	// 82D82AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82AF0 size=20
    let mut pc: u32 = 0x82D82AF0;
    'dispatch: loop {
        match pc {
            0x82D82AF0 => {
    //   block [0x82D82AF0..0x82D82B04)
	// 82D82AF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82AF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82AF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82B00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B08 size=4
    let mut pc: u32 = 0x82D82B08;
    'dispatch: loop {
        match pc {
            0x82D82B08 => {
    //   block [0x82D82B08..0x82D82B0C)
	// 82D82B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B10 size=32
    let mut pc: u32 = 0x82D82B10;
    'dispatch: loop {
        match pc {
            0x82D82B10 => {
    //   block [0x82D82B10..0x82D82B30)
	// 82D82B10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82B14: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82B18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82B1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82B20: 396BC630  addi r11, r11, -0x39d0
	ctx.r[11].s64 = ctx.r[11].s64 + -14800;
	// 82D82B24: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82B28: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B30 size=12
    let mut pc: u32 = 0x82D82B30;
    'dispatch: loop {
        match pc {
            0x82D82B30 => {
    //   block [0x82D82B30..0x82D82B3C)
	// 82D82B30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82B34: 386BC630  addi r3, r11, -0x39d0
	ctx.r[3].s64 = ctx.r[11].s64 + -14800;
	// 82D82B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B40 size=4
    let mut pc: u32 = 0x82D82B40;
    'dispatch: loop {
        match pc {
            0x82D82B40 => {
    //   block [0x82D82B40..0x82D82B44)
	// 82D82B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B48 size=20
    let mut pc: u32 = 0x82D82B48;
    'dispatch: loop {
        match pc {
            0x82D82B48 => {
    //   block [0x82D82B48..0x82D82B5C)
	// 82D82B48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82B4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82B50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82B54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82B58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B60 size=4
    let mut pc: u32 = 0x82D82B60;
    'dispatch: loop {
        match pc {
            0x82D82B60 => {
    //   block [0x82D82B60..0x82D82B64)
	// 82D82B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B68 size=32
    let mut pc: u32 = 0x82D82B68;
    'dispatch: loop {
        match pc {
            0x82D82B68 => {
    //   block [0x82D82B68..0x82D82B88)
	// 82D82B68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82B6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82B70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82B74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82B78: 396BC874  addi r11, r11, -0x378c
	ctx.r[11].s64 = ctx.r[11].s64 + -14220;
	// 82D82B7C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82B80: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B88 size=12
    let mut pc: u32 = 0x82D82B88;
    'dispatch: loop {
        match pc {
            0x82D82B88 => {
    //   block [0x82D82B88..0x82D82B94)
	// 82D82B88: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82B8C: 386BC874  addi r3, r11, -0x378c
	ctx.r[3].s64 = ctx.r[11].s64 + -14220;
	// 82D82B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82B98 size=20
    let mut pc: u32 = 0x82D82B98;
    'dispatch: loop {
        match pc {
            0x82D82B98 => {
    //   block [0x82D82B98..0x82D82BAC)
	// 82D82B98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82B9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82BA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82BA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82BA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82BB0 size=32
    let mut pc: u32 = 0x82D82BB0;
    'dispatch: loop {
        match pc {
            0x82D82BB0 => {
    //   block [0x82D82BB0..0x82D82BD0)
	// 82D82BB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82BB4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82BB8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82BBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82BC0: 396BC8DC  addi r11, r11, -0x3724
	ctx.r[11].s64 = ctx.r[11].s64 + -14116;
	// 82D82BC4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82BC8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82BD0 size=12
    let mut pc: u32 = 0x82D82BD0;
    'dispatch: loop {
        match pc {
            0x82D82BD0 => {
    //   block [0x82D82BD0..0x82D82BDC)
	// 82D82BD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82BD4: 386BC8DC  addi r3, r11, -0x3724
	ctx.r[3].s64 = ctx.r[11].s64 + -14116;
	// 82D82BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82BE0 size=20
    let mut pc: u32 = 0x82D82BE0;
    'dispatch: loop {
        match pc {
            0x82D82BE0 => {
    //   block [0x82D82BE0..0x82D82BF4)
	// 82D82BE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82BE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82BE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82BF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82BF8 size=32
    let mut pc: u32 = 0x82D82BF8;
    'dispatch: loop {
        match pc {
            0x82D82BF8 => {
    //   block [0x82D82BF8..0x82D82C18)
	// 82D82BF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82BFC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82C00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82C04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D82C08: 396BC9DC  addi r11, r11, -0x3624
	ctx.r[11].s64 = ctx.r[11].s64 + -13860;
	// 82D82C0C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D82C10: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D82C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82C18 size=12
    let mut pc: u32 = 0x82D82C18;
    'dispatch: loop {
        match pc {
            0x82D82C18 => {
    //   block [0x82D82C18..0x82D82C24)
	// 82D82C18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82C1C: 386BC9DC  addi r3, r11, -0x3624
	ctx.r[3].s64 = ctx.r[11].s64 + -13860;
	// 82D82C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82C28 size=16
    let mut pc: u32 = 0x82D82C28;
    'dispatch: loop {
        match pc {
            0x82D82C28 => {
    //   block [0x82D82C28..0x82D82C38)
	// 82D82C28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D82C2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D82C30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82D82C34: 4BFF8D34  b 0x82d7b968
	sub_82D7B968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82C38 size=4
    let mut pc: u32 = 0x82D82C38;
    'dispatch: loop {
        match pc {
            0x82D82C38 => {
    //   block [0x82D82C38..0x82D82C3C)
	// 82D82C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82C40 size=20
    let mut pc: u32 = 0x82D82C40;
    'dispatch: loop {
        match pc {
            0x82D82C40 => {
    //   block [0x82D82C40..0x82D82C54)
	// 82D82C40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82C44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82C48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82C4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82C50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82C58 size=44
    let mut pc: u32 = 0x82D82C58;
    'dispatch: loop {
        match pc {
            0x82D82C58 => {
    //   block [0x82D82C58..0x82D82C84)
	// 82D82C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82C60: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82C64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D82C68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D82C6C: 4BFF8CFD  bl 0x82d7b968
	ctx.lr = 0x82D82C70;
	sub_82D7B968(ctx, base);
	// 82D82C70: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D82C74: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 82D82C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82C88 size=92
    let mut pc: u32 = 0x82D82C88;
    'dispatch: loop {
        match pc {
            0x82D82C88 => {
    //   block [0x82D82C88..0x82D82CE4)
	// 82D82C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82C90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D82C94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D82C98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82C9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82CA0: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D82CA4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D82CA8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D82CAC: 4BFFD735  bl 0x82d803e0
	ctx.lr = 0x82D82CB0;
	sub_82D803E0(ctx, base);
	// 82D82CB0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D82CB4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D82CB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D82CBC: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D82CC0: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D82CC4: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82CC8: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82D82CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82CE8 size=92
    let mut pc: u32 = 0x82D82CE8;
    'dispatch: loop {
        match pc {
            0x82D82CE8 => {
    //   block [0x82D82CE8..0x82D82D44)
	// 82D82CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D82CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D82CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82D00: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D82D04: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D82D08: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82D0C: 4BFFD6D5  bl 0x82d803e0
	ctx.lr = 0x82D82D10;
	sub_82D803E0(ctx, base);
	// 82D82D10: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D82D14: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D82D18: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D82D1C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D82D20: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82D82D24: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82D28: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82D82D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82D38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82D48 size=144
    let mut pc: u32 = 0x82D82D48;
    'dispatch: loop {
        match pc {
            0x82D82D48 => {
    //   block [0x82D82D48..0x82D82DA4)
	// 82D82D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82D50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D82D54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D82D58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82D5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82D60: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D82D64: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D82D68: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D82D6C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82D70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82D74: 419A0030  beq cr6, 0x82d82da4
	if ctx.cr[6].eq {
	pc = 0x82D82DA4; continue 'dispatch;
	}
	// 82D82D78: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D82D7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D82D80: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D82D84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D82D88: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D82D8C: 409A0018  bne cr6, 0x82d82da4
	if !ctx.cr[6].eq {
	pc = 0x82D82DA4; continue 'dispatch;
	}
	// 82D82D90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82D94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D82D98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82D9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82DA0: 4E800421  bctrl
	ctx.lr = 0x82D82DA4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D82DA4 => {
    //   block [0x82D82DA4..0x82D82DD8)
	// 82D82DA4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D82DA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D82DAC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D82DB0: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D82DB4: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82D82DB8: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82DBC: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82D82DC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82DCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82DD8 size=144
    let mut pc: u32 = 0x82D82DD8;
    'dispatch: loop {
        match pc {
            0x82D82DD8 => {
    //   block [0x82D82DD8..0x82D82E34)
	// 82D82DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D82DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D82DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D82DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D82DF0: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D82DF4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D82DF8: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82DFC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82E00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82E04: 419A0030  beq cr6, 0x82d82e34
	if ctx.cr[6].eq {
	pc = 0x82D82E34; continue 'dispatch;
	}
	// 82D82E08: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D82E0C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D82E10: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D82E14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D82E18: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D82E1C: 409A0018  bne cr6, 0x82d82e34
	if !ctx.cr[6].eq {
	pc = 0x82D82E34; continue 'dispatch;
	}
	// 82D82E20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82E24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D82E28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82E2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82E30: 4E800421  bctrl
	ctx.lr = 0x82D82E34;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D82E34 => {
    //   block [0x82D82E34..0x82D82E68)
	// 82D82E34: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D82E38: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D82E3C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D82E40: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D82E44: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82D82E48: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82E4C: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82D82E50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D82E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D82E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D82E5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D82E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D82E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D82E68 size=92
    let mut pc: u32 = 0x82D82E68;
    'dispatch: loop {
        match pc {
            0x82D82E68 => {
    //   block [0x82D82E68..0x82D82EC4)
	// 82D82E68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D82E6C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82D82E70: 392BA544  addi r9, r11, -0x5abc
	ctx.r[9].s64 = ctx.r[11].s64 + -23228;
	// 82D82E74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D82E78: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D82E7C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82D82E80: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D82E84: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D82E88: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D82E8C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82D82E90: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D82E94: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D82E98: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82D82E9C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D82EA0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82D82EA4: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82D82EA8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82D82EAC: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82D82EB0: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82D82EB4: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82D82EB8: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82D82EBC: 99030040  stb r8, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[8].u8 ) };
	// 82D82EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D82EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D82EC8 size=568
    let mut pc: u32 = 0x82D82EC8;
    'dispatch: loop {
        match pc {
            0x82D82EC8 => {
    //   block [0x82D82EC8..0x82D82EF8)
	// 82D82EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D82ECC: 4BF26541  bl 0x82ca940c
	ctx.lr = 0x82D82ED0;
	sub_82CA93D0(ctx, base);
	// 82D82ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D82ED4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D82ED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82D82EDC: 394AA544  addi r10, r10, -0x5abc
	ctx.r[10].s64 = ctx.r[10].s64 + -23228;
	// 82D82EE0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D82EE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D82EE8: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D82EEC: 40990034  ble cr6, 0x82d82f20
	if !ctx.cr[6].gt {
	pc = 0x82D82F20; continue 'dispatch;
	}
	// 82D82EF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D82EF4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	pc = 0x82D82EF8; continue 'dispatch;
            }
            0x82D82EF8 => {
    //   block [0x82D82EF8..0x82D82F10)
	// 82D82EF8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D82EFC: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82F00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D82F04: 419A000C  beq cr6, 0x82d82f10
	if ctx.cr[6].eq {
	pc = 0x82D82F10; continue 'dispatch;
	}
	// 82D82F08: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D82F0C: 4BFFD4D5  bl 0x82d803e0
	ctx.lr = 0x82D82F10;
	sub_82D803E0(ctx, base);
	pc = 0x82D82F10; continue 'dispatch;
            }
            0x82D82F10 => {
    //   block [0x82D82F10..0x82D82F20)
	// 82D82F10: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D82F14: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D82F18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D82F1C: 409AFFDC  bne cr6, 0x82d82ef8
	if !ctx.cr[6].eq {
	pc = 0x82D82EF8; continue 'dispatch;
	}
	pc = 0x82D82F20; continue 'dispatch;
            }
            0x82D82F20 => {
    //   block [0x82D82F20..0x82D82F34)
	// 82D82F20: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D82F24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D82F28: 40990034  ble cr6, 0x82d82f5c
	if !ctx.cr[6].gt {
	pc = 0x82D82F5C; continue 'dispatch;
	}
	// 82D82F2C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D82F30: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	pc = 0x82D82F34; continue 'dispatch;
            }
            0x82D82F34 => {
    //   block [0x82D82F34..0x82D82F4C)
	// 82D82F34: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D82F38: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82F3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D82F40: 419A000C  beq cr6, 0x82d82f4c
	if ctx.cr[6].eq {
	pc = 0x82D82F4C; continue 'dispatch;
	}
	// 82D82F44: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D82F48: 4BFFD499  bl 0x82d803e0
	ctx.lr = 0x82D82F4C;
	sub_82D803E0(ctx, base);
	pc = 0x82D82F4C; continue 'dispatch;
            }
            0x82D82F4C => {
    //   block [0x82D82F4C..0x82D82F5C)
	// 82D82F4C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D82F50: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D82F54: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D82F58: 409AFFDC  bne cr6, 0x82d82f34
	if !ctx.cr[6].eq {
	pc = 0x82D82F34; continue 'dispatch;
	}
	pc = 0x82D82F5C; continue 'dispatch;
            }
            0x82D82F5C => {
    //   block [0x82D82F5C..0x82D82F70)
	// 82D82F5C: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D82F60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D82F64: 40990068  ble cr6, 0x82d82fcc
	if !ctx.cr[6].gt {
	pc = 0x82D82FCC; continue 'dispatch;
	}
	// 82D82F68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D82F6C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	pc = 0x82D82F70; continue 'dispatch;
            }
            0x82D82F70 => {
    //   block [0x82D82F70..0x82D82FBC)
	// 82D82F70: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D82F74: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D82F78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D82F7C: 419A0040  beq cr6, 0x82d82fbc
	if ctx.cr[6].eq {
	pc = 0x82D82FBC; continue 'dispatch;
	}
	// 82D82F80: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D82F84: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82F88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82F8C: 419A0030  beq cr6, 0x82d82fbc
	if ctx.cr[6].eq {
	pc = 0x82D82FBC; continue 'dispatch;
	}
	// 82D82F90: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D82F94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D82F98: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D82F9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D82FA0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D82FA4: 409A0018  bne cr6, 0x82d82fbc
	if !ctx.cr[6].eq {
	pc = 0x82D82FBC; continue 'dispatch;
	}
	// 82D82FA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82FAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D82FB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D82FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D82FB8: 4E800421  bctrl
	ctx.lr = 0x82D82FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D82FBC => {
    //   block [0x82D82FBC..0x82D82FCC)
	// 82D82FBC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D82FC0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D82FC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D82FC8: 409AFFA8  bne cr6, 0x82d82f70
	if !ctx.cr[6].eq {
	pc = 0x82D82F70; continue 'dispatch;
	}
	pc = 0x82D82FCC; continue 'dispatch;
            }
            0x82D82FCC => {
    //   block [0x82D82FCC..0x82D82FE0)
	// 82D82FCC: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D82FD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D82FD4: 40990068  ble cr6, 0x82d8303c
	if !ctx.cr[6].gt {
	pc = 0x82D8303C; continue 'dispatch;
	}
	// 82D82FD8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D82FDC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	pc = 0x82D82FE0; continue 'dispatch;
            }
            0x82D82FE0 => {
    //   block [0x82D82FE0..0x82D8302C)
	// 82D82FE0: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D82FE4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82D82FE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D82FEC: 419A0040  beq cr6, 0x82d8302c
	if ctx.cr[6].eq {
	pc = 0x82D8302C; continue 'dispatch;
	}
	// 82D82FF0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D82FF4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D82FF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D82FFC: 419A0030  beq cr6, 0x82d8302c
	if ctx.cr[6].eq {
	pc = 0x82D8302C; continue 'dispatch;
	}
	// 82D83000: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D83004: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D83008: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D8300C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83010: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D83014: 409A0018  bne cr6, 0x82d8302c
	if !ctx.cr[6].eq {
	pc = 0x82D8302C; continue 'dispatch;
	}
	// 82D83018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8301C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D83020: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83028: 4E800421  bctrl
	ctx.lr = 0x82D8302C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D8302C => {
    //   block [0x82D8302C..0x82D8303C)
	// 82D8302C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D83030: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D83034: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D83038: 409AFFA8  bne cr6, 0x82d82fe0
	if !ctx.cr[6].eq {
	pc = 0x82D82FE0; continue 'dispatch;
	}
	pc = 0x82D8303C; continue 'dispatch;
            }
            0x82D8303C => {
    //   block [0x82D8303C..0x82D83068)
	// 82D8303C: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D83040: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83044: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83048: 409A0020  bne cr6, 0x82d83068
	if !ctx.cr[6].eq {
	pc = 0x82D83068; continue 'dispatch;
	}
	// 82D8304C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83050: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D83054: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D83058: 809D002C  lwz r4, 0x2c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D8305C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D83060: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D83064: 4BFD2265  bl 0x82d552c8
	ctx.lr = 0x82D83068;
	sub_82D552C8(ctx, base);
	pc = 0x82D83068; continue 'dispatch;
            }
            0x82D83068 => {
    //   block [0x82D83068..0x82D83094)
	// 82D83068: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D8306C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83070: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83074: 409A0020  bne cr6, 0x82d83094
	if !ctx.cr[6].eq {
	pc = 0x82D83094; continue 'dispatch;
	}
	// 82D83078: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8307C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D83080: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D83084: 809D0020  lwz r4, 0x20(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D83088: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D8308C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D83090: 4BFD2239  bl 0x82d552c8
	ctx.lr = 0x82D83094;
	sub_82D552C8(ctx, base);
	pc = 0x82D83094; continue 'dispatch;
            }
            0x82D83094 => {
    //   block [0x82D83094..0x82D830C0)
	// 82D83094: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D83098: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D8309C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D830A0: 409A0020  bne cr6, 0x82d830c0
	if !ctx.cr[6].eq {
	pc = 0x82D830C0; continue 'dispatch;
	}
	// 82D830A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D830A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D830AC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D830B0: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D830B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D830B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D830BC: 4BFD220D  bl 0x82d552c8
	ctx.lr = 0x82D830C0;
	sub_82D552C8(ctx, base);
	pc = 0x82D830C0; continue 'dispatch;
            }
            0x82D830C0 => {
    //   block [0x82D830C0..0x82D830EC)
	// 82D830C0: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D830C4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D830C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D830CC: 409A0020  bne cr6, 0x82d830ec
	if !ctx.cr[6].eq {
	pc = 0x82D830EC; continue 'dispatch;
	}
	// 82D830D0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D830D4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D830D8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D830DC: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D830E0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D830E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D830E8: 4BFD21E1  bl 0x82d552c8
	ctx.lr = 0x82D830EC;
	sub_82D552C8(ctx, base);
	pc = 0x82D830EC; continue 'dispatch;
            }
            0x82D830EC => {
    //   block [0x82D830EC..0x82D83100)
	// 82D830EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D830F0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D830F4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D830F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D830FC: 4BF26360  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D83100 size=740
    let mut pc: u32 = 0x82D83100;
    'dispatch: loop {
        match pc {
            0x82D83100 => {
    //   block [0x82D83100..0x82D83158)
	// 82D83100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D83104: 4BF26305  bl 0x82ca9408
	ctx.lr = 0x82D83108;
	sub_82CA93D0(ctx, base);
	// 82D83108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8310C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D83110: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D83114: 3BFD0008  addi r31, r29, 8
	ctx.r[31].s64 = ctx.r[29].s64 + 8;
	// 82D83118: 3BDC0008  addi r30, r28, 8
	ctx.r[30].s64 = ctx.r[28].s64 + 8;
	// 82D8311C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83120: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83124: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D83128: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D8312C: 40980060  bge cr6, 0x82d8318c
	if !ctx.cr[6].lt {
	pc = 0x82D8318C; continue 'dispatch;
	}
	// 82D83130: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83134: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83138: 409A0020  bne cr6, 0x82d83158
	if !ctx.cr[6].eq {
	pc = 0x82D83158; continue 'dispatch;
	}
	// 82D8313C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83140: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D83144: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D83148: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8314C: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D83150: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83154: 4BFD2175  bl 0x82d552c8
	ctx.lr = 0x82D83158;
	sub_82D552C8(ctx, base);
	pc = 0x82D83158; continue 'dispatch;
            }
            0x82D83158 => {
    //   block [0x82D83158..0x82D8318C)
	// 82D83158: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8315C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D83160: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83164: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D83168: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D8316C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83170: 4BFD20D9  bl 0x82d55248
	ctx.lr = 0x82D83174;
	sub_82D55248(ctx, base);
	// 82D83174: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83178: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D8317C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83180: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83184: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82D83188: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82D8318C; continue 'dispatch;
            }
            0x82D8318C => {
    //   block [0x82D8318C..0x82D831A8)
	// 82D8318C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83190: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83194: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83198: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D8319C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D831A0: 40990020  ble cr6, 0x82d831c0
	if !ctx.cr[6].gt {
	pc = 0x82D831C0; continue 'dispatch;
	}
	// 82D831A4: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	pc = 0x82D831A8; continue 'dispatch;
            }
            0x82D831A8 => {
    //   block [0x82D831A8..0x82D831C0)
	// 82D831A8: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D831AC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D831B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D831B4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D831B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D831BC: 409AFFEC  bne cr6, 0x82d831a8
	if !ctx.cr[6].eq {
	pc = 0x82D831A8; continue 'dispatch;
	}
	pc = 0x82D831C0; continue 'dispatch;
            }
            0x82D831C0 => {
    //   block [0x82D831C0..0x82D83204)
	// 82D831C0: 3BFD002C  addi r31, r29, 0x2c
	ctx.r[31].s64 = ctx.r[29].s64 + 44;
	// 82D831C4: 3BDC002C  addi r30, r28, 0x2c
	ctx.r[30].s64 = ctx.r[28].s64 + 44;
	// 82D831C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D831CC: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D831D0: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D831D4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D831D8: 40980060  bge cr6, 0x82d83238
	if !ctx.cr[6].lt {
	pc = 0x82D83238; continue 'dispatch;
	}
	// 82D831DC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D831E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D831E4: 409A0020  bne cr6, 0x82d83204
	if !ctx.cr[6].eq {
	pc = 0x82D83204; continue 'dispatch;
	}
	// 82D831E8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D831EC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D831F0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D831F4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D831F8: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D831FC: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83200: 4BFD20C9  bl 0x82d552c8
	ctx.lr = 0x82D83204;
	sub_82D552C8(ctx, base);
	pc = 0x82D83204; continue 'dispatch;
            }
            0x82D83204 => {
    //   block [0x82D83204..0x82D83238)
	// 82D83204: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83208: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D8320C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83210: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D83214: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D83218: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D8321C: 4BFD202D  bl 0x82d55248
	ctx.lr = 0x82D83220;
	sub_82D55248(ctx, base);
	// 82D83220: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83224: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D83228: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D8322C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83230: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82D83234: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82D83238; continue 'dispatch;
            }
            0x82D83238 => {
    //   block [0x82D83238..0x82D83254)
	// 82D83238: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D8323C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83240: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83244: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D83248: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8324C: 40990020  ble cr6, 0x82d8326c
	if !ctx.cr[6].gt {
	pc = 0x82D8326C; continue 'dispatch;
	}
	// 82D83250: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	pc = 0x82D83254; continue 'dispatch;
            }
            0x82D83254 => {
    //   block [0x82D83254..0x82D8326C)
	// 82D83254: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83258: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D8325C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D83260: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D83264: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D83268: 409AFFEC  bne cr6, 0x82d83254
	if !ctx.cr[6].eq {
	pc = 0x82D83254; continue 'dispatch;
	}
	pc = 0x82D8326C; continue 'dispatch;
            }
            0x82D8326C => {
    //   block [0x82D8326C..0x82D832B0)
	// 82D8326C: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 82D83270: 3BDC0014  addi r30, r28, 0x14
	ctx.r[30].s64 = ctx.r[28].s64 + 20;
	// 82D83274: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83278: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D8327C: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D83280: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D83284: 40980060  bge cr6, 0x82d832e4
	if !ctx.cr[6].lt {
	pc = 0x82D832E4; continue 'dispatch;
	}
	// 82D83288: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D8328C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83290: 409A0020  bne cr6, 0x82d832b0
	if !ctx.cr[6].eq {
	pc = 0x82D832B0; continue 'dispatch;
	}
	// 82D83294: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83298: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D8329C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D832A0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D832A4: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D832A8: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D832AC: 4BFD201D  bl 0x82d552c8
	ctx.lr = 0x82D832B0;
	sub_82D552C8(ctx, base);
	pc = 0x82D832B0; continue 'dispatch;
            }
            0x82D832B0 => {
    //   block [0x82D832B0..0x82D832E4)
	// 82D832B0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D832B4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D832B8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D832BC: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D832C0: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D832C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D832C8: 4BFD1F81  bl 0x82d55248
	ctx.lr = 0x82D832CC;
	sub_82D55248(ctx, base);
	// 82D832CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D832D0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D832D4: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D832D8: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D832DC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82D832E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82D832E4; continue 'dispatch;
            }
            0x82D832E4 => {
    //   block [0x82D832E4..0x82D83300)
	// 82D832E4: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D832E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D832EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D832F0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D832F4: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D832F8: 40990020  ble cr6, 0x82d83318
	if !ctx.cr[6].gt {
	pc = 0x82D83318; continue 'dispatch;
	}
	// 82D832FC: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	pc = 0x82D83300; continue 'dispatch;
            }
            0x82D83300 => {
    //   block [0x82D83300..0x82D83318)
	// 82D83300: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83304: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D83308: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D8330C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D83310: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D83314: 409AFFEC  bne cr6, 0x82d83300
	if !ctx.cr[6].eq {
	pc = 0x82D83300; continue 'dispatch;
	}
	pc = 0x82D83318; continue 'dispatch;
            }
            0x82D83318 => {
    //   block [0x82D83318..0x82D8335C)
	// 82D83318: 3BFD0020  addi r31, r29, 0x20
	ctx.r[31].s64 = ctx.r[29].s64 + 32;
	// 82D8331C: 3BDC0020  addi r30, r28, 0x20
	ctx.r[30].s64 = ctx.r[28].s64 + 32;
	// 82D83320: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83324: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83328: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D8332C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D83330: 40980060  bge cr6, 0x82d83390
	if !ctx.cr[6].lt {
	pc = 0x82D83390; continue 'dispatch;
	}
	// 82D83334: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83338: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D8333C: 409A0020  bne cr6, 0x82d8335c
	if !ctx.cr[6].eq {
	pc = 0x82D8335C; continue 'dispatch;
	}
	// 82D83340: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83344: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D83348: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D8334C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83350: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D83354: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83358: 4BFD1F71  bl 0x82d552c8
	ctx.lr = 0x82D8335C;
	sub_82D552C8(ctx, base);
	pc = 0x82D8335C; continue 'dispatch;
            }
            0x82D8335C => {
    //   block [0x82D8335C..0x82D83390)
	// 82D8335C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83360: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D83364: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83368: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D8336C: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D83370: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83374: 4BFD1ED5  bl 0x82d55248
	ctx.lr = 0x82D83378;
	sub_82D55248(ctx, base);
	// 82D83378: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D8337C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D83380: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83384: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83388: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82D8338C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x82D83390; continue 'dispatch;
            }
            0x82D83390 => {
    //   block [0x82D83390..0x82D833AC)
	// 82D83390: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83394: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83398: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D8339C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D833A0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D833A4: 40990020  ble cr6, 0x82d833c4
	if !ctx.cr[6].gt {
	pc = 0x82D833C4; continue 'dispatch;
	}
	// 82D833A8: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	pc = 0x82D833AC; continue 'dispatch;
            }
            0x82D833AC => {
    //   block [0x82D833AC..0x82D833C4)
	// 82D833AC: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D833B0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D833B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D833B8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D833BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D833C0: 409AFFEC  bne cr6, 0x82d833ac
	if !ctx.cr[6].eq {
	pc = 0x82D833AC; continue 'dispatch;
	}
	pc = 0x82D833C4; continue 'dispatch;
            }
            0x82D833C4 => {
    //   block [0x82D833C4..0x82D833E4)
	// 82D833C4: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D833C8: 917D0038  stw r11, 0x38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82D833CC: 817C003C  lwz r11, 0x3c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D833D0: 917D003C  stw r11, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82D833D4: 897C0040  lbz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D833D8: 997D0040  stb r11, 0x40(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82D833DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D833E0: 4BF26078  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D833E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D833E8 size=132
    let mut pc: u32 = 0x82D833E8;
    'dispatch: loop {
        match pc {
            0x82D833E8 => {
    //   block [0x82D833E8..0x82D83438)
	// 82D833E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D833EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D833F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D833F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D833F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D833FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D83400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D83404: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D83408: 419A004C  beq cr6, 0x82d83454
	if ctx.cr[6].eq {
	pc = 0x82D83454; continue 'dispatch;
	}
	// 82D8340C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D83410: 4BFFCFB1  bl 0x82d803c0
	ctx.lr = 0x82D83414;
	sub_82D803C0(ctx, base);
	// 82D83414: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82D83418: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D8341C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83420: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D83424: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83428: 409A0010  bne cr6, 0x82d83438
	if !ctx.cr[6].eq {
	pc = 0x82D83438; continue 'dispatch;
	}
	// 82D8342C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D83430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D83434: 4BFD3B65  bl 0x82d56f98
	ctx.lr = 0x82D83438;
	sub_82D56F98(ctx, base);
	pc = 0x82D83438; continue 'dispatch;
            }
            0x82D83438 => {
    //   block [0x82D83438..0x82D83454)
	// 82D83438: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D8343C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83440: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D83444: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82D83448: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D8344C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D83450: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82D83454; continue 'dispatch;
            }
            0x82D83454 => {
    //   block [0x82D83454..0x82D8346C)
	// 82D83454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D83458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8345C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D83460: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D83464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D83468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D83470 size=132
    let mut pc: u32 = 0x82D83470;
    'dispatch: loop {
        match pc {
            0x82D83470 => {
    //   block [0x82D83470..0x82D834C0)
	// 82D83470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D83474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D83478: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D8347C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D83480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D83484: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D83488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D8348C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D83490: 419A004C  beq cr6, 0x82d834dc
	if ctx.cr[6].eq {
	pc = 0x82D834DC; continue 'dispatch;
	}
	// 82D83494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D83498: 4BFFCF29  bl 0x82d803c0
	ctx.lr = 0x82D8349C;
	sub_82D803C0(ctx, base);
	// 82D8349C: 3BFF002C  addi r31, r31, 0x2c
	ctx.r[31].s64 = ctx.r[31].s64 + 44;
	// 82D834A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D834A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D834A8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D834AC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D834B0: 409A0010  bne cr6, 0x82d834c0
	if !ctx.cr[6].eq {
	pc = 0x82D834C0; continue 'dispatch;
	}
	// 82D834B4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D834B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D834BC: 4BFD3ADD  bl 0x82d56f98
	ctx.lr = 0x82D834C0;
	sub_82D56F98(ctx, base);
	pc = 0x82D834C0; continue 'dispatch;
            }
            0x82D834C0 => {
    //   block [0x82D834C0..0x82D834DC)
	// 82D834C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D834C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D834C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D834CC: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82D834D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D834D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D834D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82D834DC; continue 'dispatch;
            }
            0x82D834DC => {
    //   block [0x82D834DC..0x82D834F4)
	// 82D834DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D834E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D834E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D834E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D834EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D834F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D834F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D834F8 size=144
    let mut pc: u32 = 0x82D834F8;
    'dispatch: loop {
        match pc {
            0x82D834F8 => {
    //   block [0x82D834F8..0x82D83530)
	// 82D834F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D834FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D83500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D83504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D83508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8350C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D83510: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D83514: 419A005C  beq cr6, 0x82d83570
	if ctx.cr[6].eq {
	pc = 0x82D83570; continue 'dispatch;
	}
	// 82D83518: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D8351C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D83520: 419A0010  beq cr6, 0x82d83530
	if ctx.cr[6].eq {
	pc = 0x82D83530; continue 'dispatch;
	}
	// 82D83524: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D83528: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D8352C: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82D83530; continue 'dispatch;
            }
            0x82D83530 => {
    //   block [0x82D83530..0x82D83554)
	// 82D83530: 3BE30014  addi r31, r3, 0x14
	ctx.r[31].s64 = ctx.r[3].s64 + 20;
	// 82D83534: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83538: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D8353C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D83540: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83544: 409A0010  bne cr6, 0x82d83554
	if !ctx.cr[6].eq {
	pc = 0x82D83554; continue 'dispatch;
	}
	// 82D83548: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D8354C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D83550: 4BFD3A49  bl 0x82d56f98
	ctx.lr = 0x82D83554;
	sub_82D56F98(ctx, base);
	pc = 0x82D83554; continue 'dispatch;
            }
            0x82D83554 => {
    //   block [0x82D83554..0x82D83570)
	// 82D83554: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83558: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8355C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D83560: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82D83564: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83568: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D8356C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82D83570; continue 'dispatch;
            }
            0x82D83570 => {
    //   block [0x82D83570..0x82D83588)
	// 82D83570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D83574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D83578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D8357C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D83580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D83584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D83588 size=144
    let mut pc: u32 = 0x82D83588;
    'dispatch: loop {
        match pc {
            0x82D83588 => {
    //   block [0x82D83588..0x82D835C0)
	// 82D83588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8358C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D83590: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D83594: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D83598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8359C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D835A0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D835A4: 419A005C  beq cr6, 0x82d83600
	if ctx.cr[6].eq {
	pc = 0x82D83600; continue 'dispatch;
	}
	// 82D835A8: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D835AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D835B0: 419A0010  beq cr6, 0x82d835c0
	if ctx.cr[6].eq {
	pc = 0x82D835C0; continue 'dispatch;
	}
	// 82D835B4: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D835B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D835BC: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82D835C0; continue 'dispatch;
            }
            0x82D835C0 => {
    //   block [0x82D835C0..0x82D835E4)
	// 82D835C0: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 82D835C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D835C8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D835CC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D835D0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D835D4: 409A0010  bne cr6, 0x82d835e4
	if !ctx.cr[6].eq {
	pc = 0x82D835E4; continue 'dispatch;
	}
	// 82D835D8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D835DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D835E0: 4BFD39B9  bl 0x82d56f98
	ctx.lr = 0x82D835E4;
	sub_82D56F98(ctx, base);
	pc = 0x82D835E4; continue 'dispatch;
            }
            0x82D835E4 => {
    //   block [0x82D835E4..0x82D83600)
	// 82D835E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D835E8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D835EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D835F0: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82D835F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D835F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D835FC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x82D83600; continue 'dispatch;
            }
            0x82D83600 => {
    //   block [0x82D83600..0x82D83618)
	// 82D83600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D83604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D83608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D8360C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D83610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D83614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D83618 size=1440
    let mut pc: u32 = 0x82D83618;
    'dispatch: loop {
        match pc {
            0x82D83618 => {
    //   block [0x82D83618..0x82D836F0)
	// 82D83618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8361C: 4BF25DC9  bl 0x82ca93e4
	ctx.lr = 0x82D83620;
	sub_82CA93D0(ctx, base);
	// 82D83620: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D83624: 826D0000  lwz r19, 0(r13)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83628: 3A800004  li r20, 4
	ctx.r[20].s64 = 4;
	// 82D8362C: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82D83630: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82D83634: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D83638: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82D8363C: 4BFD1C0D  bl 0x82d55248
	ctx.lr = 0x82D83640;
	sub_82D55248(ctx, base);
	// 82D83640: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D83644: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D83648: 396BA544  addi r11, r11, -0x5abc
	ctx.r[11].s64 = ctx.r[11].s64 + -23228;
	// 82D8364C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D83650: 39200044  li r9, 0x44
	ctx.r[9].s64 = 68;
	// 82D83654: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82D83658: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 82D8365C: 3B190008  addi r24, r25, 8
	ctx.r[24].s64 = ctx.r[25].s64 + 8;
	// 82D83660: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D83664: 3B990014  addi r28, r25, 0x14
	ctx.r[28].s64 = ctx.r[25].s64 + 20;
	// 82D83668: B1590006  sth r10, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D8366C: 3AB90020  addi r21, r25, 0x20
	ctx.r[21].s64 = ctx.r[25].s64 + 32;
	// 82D83670: B1390004  sth r9, 4(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82D83674: 3AF9002C  addi r23, r25, 0x2c
	ctx.r[23].s64 = ctx.r[25].s64 + 44;
	// 82D83678: 93580000  stw r26, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D8367C: 93580004  stw r26, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82D83680: 92D80008  stw r22, 8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82D83684: 935C0000  stw r26, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D83688: 935C0004  stw r26, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82D8368C: 92DC0008  stw r22, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82D83690: 93550000  stw r26, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D83694: 93550004  stw r26, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82D83698: 92D50008  stw r22, 8(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82D8369C: 93570000  stw r26, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D836A0: 93570004  stw r26, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82D836A4: 92D70008  stw r22, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82D836A8: 93590038  stw r26, 0x38(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 82D836AC: 9359003C  stw r26, 0x3c(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(60 as u32), ctx.r[26].u32 ) };
	// 82D836B0: 99590040  stb r10, 0x40(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 82D836B4: 817B0038  lwz r11, 0x38(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D836B8: 91790038  stw r11, 0x38(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82D836BC: 817B003C  lwz r11, 0x3c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D836C0: 9179003C  stw r11, 0x3c(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82D836C4: 897B0040  lbz r11, 0x40(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D836C8: 99790040  stb r11, 0x40(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82D836CC: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D836D0: 83FB000C  lwz r31, 0xc(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D836D4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D836D8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D836DC: 40980024  bge cr6, 0x82d83700
	if !ctx.cr[6].lt {
	pc = 0x82D83700; continue 'dispatch;
	}
	// 82D836E0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D836E4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D836E8: 41980008  blt cr6, 0x82d836f0
	if ctx.cr[6].lt {
	pc = 0x82D836F0; continue 'dispatch;
	}
	// 82D836EC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82D836F0; continue 'dispatch;
            }
            0x82D836F0 => {
    //   block [0x82D836F0..0x82D83700)
	// 82D836F0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D836F4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D836F8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82D836FC: 4BFD3815  bl 0x82d56f10
	ctx.lr = 0x82D83700;
	sub_82D56F10(ctx, base);
	pc = 0x82D83700; continue 'dispatch;
            }
            0x82D83700 => {
    //   block [0x82D83700..0x82D83728)
	// 82D83700: 93F80004  stw r31, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D83704: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83708: 83FB0030  lwz r31, 0x30(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D8370C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D83710: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D83714: 40980024  bge cr6, 0x82d83738
	if !ctx.cr[6].lt {
	pc = 0x82D83738; continue 'dispatch;
	}
	// 82D83718: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D8371C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83720: 41980008  blt cr6, 0x82d83728
	if ctx.cr[6].lt {
	pc = 0x82D83728; continue 'dispatch;
	}
	// 82D83724: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82D83728; continue 'dispatch;
            }
            0x82D83728 => {
    //   block [0x82D83728..0x82D83738)
	// 82D83728: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D8372C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D83730: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82D83734: 4BFD37DD  bl 0x82d56f10
	ctx.lr = 0x82D83738;
	sub_82D56F10(ctx, base);
	pc = 0x82D83738; continue 'dispatch;
            }
            0x82D83738 => {
    //   block [0x82D83738..0x82D83760)
	// 82D83738: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D8373C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83740: 83FB0018  lwz r31, 0x18(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D83744: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D83748: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D8374C: 40980024  bge cr6, 0x82d83770
	if !ctx.cr[6].lt {
	pc = 0x82D83770; continue 'dispatch;
	}
	// 82D83750: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D83754: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83758: 41980008  blt cr6, 0x82d83760
	if ctx.cr[6].lt {
	pc = 0x82D83760; continue 'dispatch;
	}
	// 82D8375C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82D83760; continue 'dispatch;
            }
            0x82D83760 => {
    //   block [0x82D83760..0x82D83770)
	// 82D83760: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D83764: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D83768: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D8376C: 4BFD37A5  bl 0x82d56f10
	ctx.lr = 0x82D83770;
	sub_82D56F10(ctx, base);
	pc = 0x82D83770; continue 'dispatch;
            }
            0x82D83770 => {
    //   block [0x82D83770..0x82D83798)
	// 82D83770: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D83774: 81750008  lwz r11, 8(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83778: 83FB0024  lwz r31, 0x24(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D8377C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D83780: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D83784: 40980024  bge cr6, 0x82d837a8
	if !ctx.cr[6].lt {
	pc = 0x82D837A8; continue 'dispatch;
	}
	// 82D83788: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D8378C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83790: 41980008  blt cr6, 0x82d83798
	if ctx.cr[6].lt {
	pc = 0x82D83798; continue 'dispatch;
	}
	// 82D83794: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82D83798; continue 'dispatch;
            }
            0x82D83798 => {
    //   block [0x82D83798..0x82D837A8)
	// 82D83798: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D8379C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D837A0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82D837A4: 4BFD376D  bl 0x82d56f10
	ctx.lr = 0x82D837A8;
	sub_82D56F10(ctx, base);
	pc = 0x82D837A8; continue 'dispatch;
            }
            0x82D837A8 => {
    //   block [0x82D837A8..0x82D837C0)
	// 82D837A8: 93F50004  stw r31, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D837AC: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82D837B0: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D837B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D837B8: 40990040  ble cr6, 0x82d837f8
	if !ctx.cr[6].gt {
	pc = 0x82D837F8; continue 'dispatch;
	}
	// 82D837BC: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	pc = 0x82D837C0; continue 'dispatch;
            }
            0x82D837C0 => {
    //   block [0x82D837C0..0x82D837F8)
	// 82D837C0: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D837C4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D837C8: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82D837CC: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82D837D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D837D4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D837D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D837DC: 4E800421  bctrl
	ctx.lr = 0x82D837E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D837E0: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D837E4: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D837E8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82D837EC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D837F0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D837F4: 4198FFCC  blt cr6, 0x82d837c0
	if ctx.cr[6].lt {
	pc = 0x82D837C0; continue 'dispatch;
	}
            }
            0x82D837F8 => {
    //   block [0x82D837F8..0x82D8380C)
	// 82D837F8: 817B0030  lwz r11, 0x30(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D837FC: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82D83800: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83804: 40990040  ble cr6, 0x82d83844
	if !ctx.cr[6].gt {
	pc = 0x82D83844; continue 'dispatch;
	}
	// 82D83808: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	pc = 0x82D8380C; continue 'dispatch;
            }
            0x82D8380C => {
    //   block [0x82D8380C..0x82D83844)
	// 82D8380C: 815B002C  lwz r10, 0x2c(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D83810: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83814: 7FBF5A14  add r29, r31, r11
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D83818: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82D8381C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83820: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D83824: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83828: 4E800421  bctrl
	ctx.lr = 0x82D8382C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D8382C: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D83830: 817B0030  lwz r11, 0x30(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D83834: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82D83838: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D8383C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83840: 4198FFCC  blt cr6, 0x82d8380c
	if ctx.cr[6].lt {
	pc = 0x82D8380C; continue 'dispatch;
	}
            }
            0x82D83844 => {
    //   block [0x82D83844..0x82D83858)
	// 82D83844: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D83848: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82D8384C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83850: 409900BC  ble cr6, 0x82d8390c
	if !ctx.cr[6].gt {
	pc = 0x82D8390C; continue 'dispatch;
	}
	// 82D83854: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	pc = 0x82D83858; continue 'dispatch;
            }
            0x82D83858 => {
    //   block [0x82D83858..0x82D83878)
	// 82D83858: 815B0014  lwz r10, 0x14(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D8385C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82D83860: 813B000C  lwz r9, 0xc(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D83864: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D83868: 7C7E502E  lwzx r3, r30, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D8386C: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D83870: 40990024  ble cr6, 0x82d83894
	if !ctx.cr[6].gt {
	pc = 0x82D83894; continue 'dispatch;
	}
	// 82D83874: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82D83878; continue 'dispatch;
            }
            0x82D83878 => {
    //   block [0x82D83878..0x82D83894)
	// 82D83878: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8387C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D83880: 419A00BC  beq cr6, 0x82d8393c
	if ctx.cr[6].eq {
	pc = 0x82D8393C; continue 'dispatch;
	}
	// 82D83884: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D83888: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D8388C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D83890: 4198FFE8  blt cr6, 0x82d83878
	if ctx.cr[6].lt {
	pc = 0x82D83878; continue 'dispatch;
	}
	pc = 0x82D83894; continue 'dispatch;
            }
            0x82D83894 => {
    //   block [0x82D83894..0x82D83898)
	// 82D83894: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	pc = 0x82D83898; continue 'dispatch;
            }
            0x82D83898 => {
    //   block [0x82D83898..0x82D838B0)
	// 82D83898: 813B000C  lwz r9, 0xc(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D8389C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82D838A0: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D838A4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D838A8: 40990024  ble cr6, 0x82d838cc
	if !ctx.cr[6].gt {
	pc = 0x82D838CC; continue 'dispatch;
	}
	// 82D838AC: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	pc = 0x82D838B0; continue 'dispatch;
            }
            0x82D838B0 => {
    //   block [0x82D838B0..0x82D838CC)
	// 82D838B0: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D838B4: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D838B8: 419A008C  beq cr6, 0x82d83944
	if ctx.cr[6].eq {
	pc = 0x82D83944; continue 'dispatch;
	}
	// 82D838BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D838C0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D838C4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D838C8: 4198FFE8  blt cr6, 0x82d838b0
	if ctx.cr[6].lt {
	pc = 0x82D838B0; continue 'dispatch;
	}
	pc = 0x82D838CC; continue 'dispatch;
            }
            0x82D838CC => {
    //   block [0x82D838CC..0x82D838D0)
	// 82D838CC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	pc = 0x82D838D0; continue 'dispatch;
            }
            0x82D838D0 => {
    //   block [0x82D838D0..0x82D8390C)
	// 82D838D0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D838D4: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D838D8: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D838DC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D838E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D838E4: 7FEAF214  add r31, r10, r30
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82D838E8: 7CA8582E  lwzx r5, r8, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D838EC: 7C89582E  lwzx r4, r9, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D838F0: 48003961  bl 0x82d87250
	ctx.lr = 0x82D838F4;
	sub_82D87250(ctx, base);
	// 82D838F4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D838F8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D838FC: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D83900: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82D83904: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83908: 4198FF50  blt cr6, 0x82d83858
	if ctx.cr[6].lt {
	pc = 0x82D83858; continue 'dispatch;
	}
	pc = 0x82D8390C; continue 'dispatch;
            }
            0x82D8390C => {
    //   block [0x82D8390C..0x82D83920)
	// 82D8390C: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D83910: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82D83914: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83918: 40990294  ble cr6, 0x82d83bac
	if !ctx.cr[6].gt {
	pc = 0x82D83BAC; continue 'dispatch;
	}
	// 82D8391C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	pc = 0x82D83920; continue 'dispatch;
            }
            0x82D83920 => {
    //   block [0x82D83920..0x82D8393C)
	// 82D83920: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D83924: 7FDD582E  lwzx r30, r29, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D83928: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D8392C: 409A0020  bne cr6, 0x82d8394c
	if !ctx.cr[6].eq {
	pc = 0x82D8394C; continue 'dispatch;
	}
	// 82D83930: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83934: 7F5D592E  stwx r26, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[26].u32) };
	// 82D83938: 48000260  b 0x82d83b98
	pc = 0x82D83B98; continue 'dispatch;
            }
            0x82D8393C => {
    //   block [0x82D8393C..0x82D83944)
	// 82D8393C: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 82D83940: 4BFFFF58  b 0x82d83898
	pc = 0x82D83898; continue 'dispatch;
            }
            0x82D83944 => {
    //   block [0x82D83944..0x82D8394C)
	// 82D83944: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D83948: 4BFFFF88  b 0x82d838d0
	pc = 0x82D838D0; continue 'dispatch;
            }
            0x82D8394C => {
    //   block [0x82D8394C..0x82D83994)
	// 82D8394C: 93410080  stw r26, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[26].u32 ) };
	// 82D83950: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82D83954: 93410084  stw r26, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[26].u32 ) };
	// 82D83958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D8395C: 92C10088  stw r22, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[22].u32 ) };
	// 82D83960: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83964: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D83968: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D8396C: 4E800421  bctrl
	ctx.lr = 0x82D83970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D83970: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D83974: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 82D83978: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82D8397C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D83980: 92C10068  stw r22, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[22].u32 ) };
	// 82D83984: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83988: 4099001C  ble cr6, 0x82d839a4
	if !ctx.cr[6].gt {
	pc = 0x82D839A4; continue 'dispatch;
	}
	// 82D8398C: 40980008  bge cr6, 0x82d83994
	if !ctx.cr[6].lt {
	pc = 0x82D83994; continue 'dispatch;
	}
	// 82D83990: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
            }
            0x82D83994 => {
    //   block [0x82D83994..0x82D839A4)
	// 82D83994: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D83998: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D8399C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D839A0: 4BFD3571  bl 0x82d56f10
	ctx.lr = 0x82D839A4;
	sub_82D56F10(ctx, base);
	pc = 0x82D839A4; continue 'dispatch;
            }
            0x82D839A4 => {
    //   block [0x82D839A4..0x82D839B8)
	// 82D839A4: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82D839A8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D839AC: 4099006C  ble cr6, 0x82d83a18
	if !ctx.cr[6].gt {
	pc = 0x82D83A18; continue 'dispatch;
	}
	// 82D839B0: 38DB0008  addi r6, r27, 8
	ctx.r[6].s64 = ctx.r[27].s64 + 8;
	// 82D839B4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	pc = 0x82D839B8; continue 'dispatch;
            }
            0x82D839B8 => {
    //   block [0x82D839B8..0x82D839D4)
	// 82D839B8: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D839BC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82D839C0: 81260004  lwz r9, 4(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D839C4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D839C8: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D839CC: 40990024  ble cr6, 0x82d839f0
	if !ctx.cr[6].gt {
	pc = 0x82D839F0; continue 'dispatch;
	}
	// 82D839D0: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82D839D4; continue 'dispatch;
            }
            0x82D839D4 => {
    //   block [0x82D839D4..0x82D839F0)
	// 82D839D4: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D839D8: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82D839DC: 419A0018  beq cr6, 0x82d839f4
	if ctx.cr[6].eq {
	pc = 0x82D839F4; continue 'dispatch;
	}
	// 82D839E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D839E4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D839E8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D839EC: 4198FFE8  blt cr6, 0x82d839d4
	if ctx.cr[6].lt {
	pc = 0x82D839D4; continue 'dispatch;
	}
	pc = 0x82D839F0; continue 'dispatch;
            }
            0x82D839F0 => {
    //   block [0x82D839F0..0x82D839F4)
	// 82D839F0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82D839F4; continue 'dispatch;
            }
            0x82D839F4 => {
    //   block [0x82D839F4..0x82D83A18)
	// 82D839F4: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D839F8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D839FC: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82D83A00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D83A04: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D83A08: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D83A0C: 7D68512E  stwx r11, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82D83A10: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82D83A14: 409AFFA4  bne cr6, 0x82d839b8
	if !ctx.cr[6].eq {
	pc = 0x82D839B8; continue 'dispatch;
	}
	pc = 0x82D83A18; continue 'dispatch;
            }
            0x82D83A18 => {
    //   block [0x82D83A18..0x82D83A60)
	// 82D83A18: 93410070  stw r26, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[26].u32 ) };
	// 82D83A1C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82D83A20: 93410074  stw r26, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[26].u32 ) };
	// 82D83A24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D83A28: 92C10078  stw r22, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[22].u32 ) };
	// 82D83A2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83A30: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D83A34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83A38: 4E800421  bctrl
	ctx.lr = 0x82D83A3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D83A3C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D83A40: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82D83A44: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 82D83A48: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D83A4C: 92C10058  stw r22, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[22].u32 ) };
	// 82D83A50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D83A54: 4099001C  ble cr6, 0x82d83a70
	if !ctx.cr[6].gt {
	pc = 0x82D83A70; continue 'dispatch;
	}
	// 82D83A58: 40980008  bge cr6, 0x82d83a60
	if !ctx.cr[6].lt {
	pc = 0x82D83A60; continue 'dispatch;
	}
	// 82D83A5C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
            }
            0x82D83A60 => {
    //   block [0x82D83A60..0x82D83A70)
	// 82D83A60: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D83A64: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D83A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D83A6C: 4BFD34A5  bl 0x82d56f10
	ctx.lr = 0x82D83A70;
	sub_82D56F10(ctx, base);
	pc = 0x82D83A70; continue 'dispatch;
            }
            0x82D83A70 => {
    //   block [0x82D83A70..0x82D83A84)
	// 82D83A70: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D83A74: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D83A78: 4099006C  ble cr6, 0x82d83ae4
	if !ctx.cr[6].gt {
	pc = 0x82D83AE4; continue 'dispatch;
	}
	// 82D83A7C: 38DB002C  addi r6, r27, 0x2c
	ctx.r[6].s64 = ctx.r[27].s64 + 44;
	// 82D83A80: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	pc = 0x82D83A84; continue 'dispatch;
            }
            0x82D83A84 => {
    //   block [0x82D83A84..0x82D83AA0)
	// 82D83A84: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D83A88: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82D83A8C: 81260004  lwz r9, 4(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D83A90: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D83A94: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D83A98: 40990024  ble cr6, 0x82d83abc
	if !ctx.cr[6].gt {
	pc = 0x82D83ABC; continue 'dispatch;
	}
	// 82D83A9C: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82D83AA0; continue 'dispatch;
            }
            0x82D83AA0 => {
    //   block [0x82D83AA0..0x82D83ABC)
	// 82D83AA0: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83AA4: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82D83AA8: 419A0018  beq cr6, 0x82d83ac0
	if ctx.cr[6].eq {
	pc = 0x82D83AC0; continue 'dispatch;
	}
	// 82D83AAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D83AB0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D83AB4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D83AB8: 4198FFE8  blt cr6, 0x82d83aa0
	if ctx.cr[6].lt {
	pc = 0x82D83AA0; continue 'dispatch;
	}
	pc = 0x82D83ABC; continue 'dispatch;
            }
            0x82D83ABC => {
    //   block [0x82D83ABC..0x82D83AC0)
	// 82D83ABC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82D83AC0; continue 'dispatch;
            }
            0x82D83AC0 => {
    //   block [0x82D83AC0..0x82D83AE4)
	// 82D83AC0: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83AC4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D83AC8: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82D83ACC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D83AD0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D83AD4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D83AD8: 7D68512E  stwx r11, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82D83ADC: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82D83AE0: 409AFFA4  bne cr6, 0x82d83a84
	if !ctx.cr[6].eq {
	pc = 0x82D83A84; continue 'dispatch;
	}
	pc = 0x82D83AE4; continue 'dispatch;
            }
            0x82D83AE4 => {
    //   block [0x82D83AE4..0x82D83B2C)
	// 82D83AE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83AE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D83AEC: 83F50000  lwz r31, 0(r21)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83AF0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82D83AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D83AF8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D83AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83B00: 4E800421  bctrl
	ctx.lr = 0x82D83B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D83B04: 7C7DF92E  stwx r3, r29, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.r[3].u32) };
	// 82D83B08: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D83B0C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83B10: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83B14: 409A0018  bne cr6, 0x82d83b2c
	if !ctx.cr[6].eq {
	pc = 0x82D83B2C; continue 'dispatch;
	}
	// 82D83B18: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D83B1C: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82D83B20: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D83B24: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D83B28: 4BFD17A1  bl 0x82d552c8
	ctx.lr = 0x82D83B2C;
	sub_82D552C8(ctx, base);
            }
            0x82D83B2C => {
    //   block [0x82D83B2C..0x82D83B50)
	// 82D83B2C: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D83B30: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83B34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83B38: 409A0018  bne cr6, 0x82d83b50
	if !ctx.cr[6].eq {
	pc = 0x82D83B50; continue 'dispatch;
	}
	// 82D83B3C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D83B40: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82D83B44: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D83B48: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D83B4C: 4BFD177D  bl 0x82d552c8
	ctx.lr = 0x82D83B50;
	sub_82D552C8(ctx, base);
	pc = 0x82D83B50; continue 'dispatch;
            }
            0x82D83B50 => {
    //   block [0x82D83B50..0x82D83B74)
	// 82D83B50: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D83B54: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83B58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83B5C: 409A0018  bne cr6, 0x82d83b74
	if !ctx.cr[6].eq {
	pc = 0x82D83B74; continue 'dispatch;
	}
	// 82D83B60: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D83B64: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82D83B68: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D83B6C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D83B70: 4BFD1759  bl 0x82d552c8
	ctx.lr = 0x82D83B74;
	sub_82D552C8(ctx, base);
	pc = 0x82D83B74; continue 'dispatch;
            }
            0x82D83B74 => {
    //   block [0x82D83B74..0x82D83B98)
	// 82D83B74: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82D83B78: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D83B7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D83B80: 409A0018  bne cr6, 0x82d83b98
	if !ctx.cr[6].eq {
	pc = 0x82D83B98; continue 'dispatch;
	}
	// 82D83B84: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D83B88: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82D83B8C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D83B90: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D83B94: 4BFD1735  bl 0x82d552c8
	ctx.lr = 0x82D83B98;
	sub_82D552C8(ctx, base);
	pc = 0x82D83B98; continue 'dispatch;
            }
            0x82D83B98 => {
    //   block [0x82D83B98..0x82D83BAC)
	// 82D83B98: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D83B9C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82D83BA0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82D83BA4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D83BA8: 4198FD78  blt cr6, 0x82d83920
	if ctx.cr[6].lt {
	pc = 0x82D83920; continue 'dispatch;
	}
	pc = 0x82D83BAC; continue 'dispatch;
            }
            0x82D83BAC => {
    //   block [0x82D83BAC..0x82D83BB8)
	// 82D83BAC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82D83BB0: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82D83BB4: 4BF25880  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D83BE8 size=44
    let mut pc: u32 = 0x82D83BE8;
    'dispatch: loop {
        match pc {
            0x82D83BE8 => {
    //   block [0x82D83BE8..0x82D83C14)
	// 82D83BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D83BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D83BF0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D83BF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D83BF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D83BFC: 4BFFE035  bl 0x82d81c30
	ctx.lr = 0x82D83C00;
	sub_82D81C30(ctx, base);
	// 82D83C00: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D83C04: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82D83C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D83C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D83C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83C18 size=288
    let mut pc: u32 = 0x82D83C18;
    'dispatch: loop {
        match pc {
            0x82D83C18 => {
    //   block [0x82D83C18..0x82D83D38)
	// 82D83C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D83C1C: 4BF257ED  bl 0x82ca9408
	ctx.lr = 0x82D83C20;
	sub_82CA93D0(ctx, base);
	// 82D83C20: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82D83C24: 3BC00020  li r30, 0x20
	ctx.r[30].s64 = 32;
	// 82D83C28: 39030030  addi r8, r3, 0x30
	ctx.r[8].s64 = ctx.r[3].s64 + 48;
	// 82D83C2C: 3961FFC0  addi r11, r1, -0x40
	ctx.r[11].s64 = ctx.r[1].s64 + -64;
	// 82D83C30: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D83D38 size=80
    let mut pc: u32 = 0x82D83D38;
    'dispatch: loop {
        match pc {
            0x82D83D38 => {
    //   block [0x82D83D38..0x82D83D88)
	// 82D83D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D83D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D83D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D83D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D83D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D83D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D83D50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D83D54: 48001755  bl 0x82d854a8
	ctx.lr = 0x82D83D58;
	sub_82D854A8(ctx, base);
	// 82D83D58: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D83D5C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D83D60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D83D64: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D83D68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83D6C: 4E800421  bctrl
	ctx.lr = 0x82D83D70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D83D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D83D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D83D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D83D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D83D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D83D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D83D88 size=80
    let mut pc: u32 = 0x82D83D88;
    'dispatch: loop {
        match pc {
            0x82D83D88 => {
    //   block [0x82D83D88..0x82D83DD8)
	// 82D83D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D83D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D83D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D83D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D83D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D83D9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D83DA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D83DA4: 48001705  bl 0x82d854a8
	ctx.lr = 0x82D83DA8;
	sub_82D854A8(ctx, base);
	// 82D83DA8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D83DAC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D83DB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D83DB4: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D83DB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83DBC: 4E800421  bctrl
	ctx.lr = 0x82D83DC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D83DC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D83DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D83DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D83DCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D83DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D83DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D83DD8 size=64
    let mut pc: u32 = 0x82D83DD8;
    'dispatch: loop {
        match pc {
            0x82D83DD8 => {
    //   block [0x82D83DD8..0x82D83E18)
	// 82D83DD8: C1A50000  lfs f13, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D83DDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D83DE0: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D83DE4: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82D83DE8: C1850008  lfs f12, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D83DEC: FC0B682E  fsel f0, f11, f0, f13
	ctx.f[0].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82D83DF0: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82D83DF4: FC0D032E  fsel f0, f13, f12, f0
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 82D83DF8: C1AB0BFC  lfs f13, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D83DFC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82D83E00: 40980018  bge cr6, 0x82d83e18
	if !ctx.cr[6].lt {
		sub_82D83E18(ctx, base);
		return;
	}
	// 82D83E04: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D83E08: C1AB0A94  lfs f13, 0xa94(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D83E0C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D83E10: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82D83E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D83E18 size=16
    let mut pc: u32 = 0x82D83E18;
    'dispatch: loop {
        match pc {
            0x82D83E18 => {
    //   block [0x82D83E18..0x82D83E28)
	// 82D83E18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D83E1C: C00B0BF8  lfs f0, 0xbf8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D83E20: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82D83E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83E28 size=24
    let mut pc: u32 = 0x82D83E28;
    'dispatch: loop {
        match pc {
            0x82D83E28 => {
    //   block [0x82D83E28..0x82D83E40)
	// 82D83E28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D83E2C: 396BB944  addi r11, r11, -0x46bc
	ctx.r[11].s64 = ctx.r[11].s64 + -18108;
	// 82D83E30: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D83E34: 48002644  b 0x82d86478
	sub_82D86478(ctx, base);
	return;
	// 82D83E38: 386300E0  addi r3, r3, 0xe0
	ctx.r[3].s64 = ctx.r[3].s64 + 224;
	// 82D83E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83E40 size=60
    let mut pc: u32 = 0x82D83E40;
    'dispatch: loop {
        match pc {
            0x82D83E40 => {
    //   block [0x82D83E40..0x82D83E7C)
	// 82D83E40: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82D83E44: 419A004C  beq cr6, 0x82d83e90
	if ctx.cr[6].eq {
		sub_82D83E90(ctx, base);
		return;
	}
	// 82D83E48: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82D83E4C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82D83E50: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D83E54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D83E58: 80A300C0  lwz r5, 0xc0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D83E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D83E60: 419A001C  beq cr6, 0x82d83e7c
	if ctx.cr[6].eq {
		sub_82D83E7C(ctx, base);
		return;
	}
	// 82D83E64: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D83E68: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82D83E6C: 890B02C7  lbz r8, 0x2c7(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(711 as u32) ) } as u64;
	// 82D83E70: 88EB02C6  lbz r7, 0x2c6(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(710 as u32) ) } as u64;
	// 82D83E74: 88CB02C5  lbz r6, 0x2c5(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(709 as u32) ) } as u64;
	// 82D83E78: 4B8D9CB0  b 0x8265db28
	sub_8265DB28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83E7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83E7C size=20
    let mut pc: u32 = 0x82D83E7C;
    'dispatch: loop {
        match pc {
            0x82D83E7C => {
    //   block [0x82D83E7C..0x82D83E90)
	// 82D83E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D83E80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D83E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D83E88: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82D83E8C: 4B8D9C9C  b 0x8265db28
	sub_8265DB28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83E90 size=28
    let mut pc: u32 = 0x82D83E90;
    'dispatch: loop {
        match pc {
            0x82D83E90 => {
    //   block [0x82D83E90..0x82D83EAC)
	// 82D83E90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D83E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D83E98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D83E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D83EA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D83EA4: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82D83EA8: 4B8D9C80  b 0x8265db28
	sub_8265DB28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83EAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83EAC size=4
    let mut pc: u32 = 0x82D83EAC;
    'dispatch: loop {
        match pc {
            0x82D83EAC => {
    //   block [0x82D83EAC..0x82D83EB0)
	// 82D83EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83EB0 size=28
    let mut pc: u32 = 0x82D83EB0;
    'dispatch: loop {
        match pc {
            0x82D83EB0 => {
    //   block [0x82D83EB0..0x82D83ECC)
	// 82D83EB0: 896300D9  lbz r11, 0xd9(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(217 as u32) ) } as u64;
	// 82D83EB4: 396BFF01  addi r11, r11, -0xff
	ctx.r[11].s64 = ctx.r[11].s64 + -255;
	// 82D83EB8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D83EBC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D83EC0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D83EC4: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82D83EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83ED0 size=24
    let mut pc: u32 = 0x82D83ED0;
    'dispatch: loop {
        match pc {
            0x82D83ED0 => {
    //   block [0x82D83ED0..0x82D83EE8)
	// 82D83ED0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D83ED4: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 82D83ED8: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D83EDC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D83EE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83EE4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83EE8 size=24
    let mut pc: u32 = 0x82D83EE8;
    'dispatch: loop {
        match pc {
            0x82D83EE8 => {
    //   block [0x82D83EE8..0x82D83F00)
	// 82D83EE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D83EEC: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 82D83EF0: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D83EF4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D83EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83EFC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D83F00 size=24
    let mut pc: u32 = 0x82D83F00;
    'dispatch: loop {
        match pc {
            0x82D83F00 => {
    //   block [0x82D83F00..0x82D83F18)
	// 82D83F00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D83F04: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 82D83F08: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D83F0C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D83F10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83F14: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D83F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D83F30 size=264
    let mut pc: u32 = 0x82D83F30;
    'dispatch: loop {
        match pc {
            0x82D83F30 => {
    //   block [0x82D83F30..0x82D84038)
	// 82D83F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D83F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D83F38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D83F3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D83F40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D83F44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D83F48: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82D83F4C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D83F50: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D83F54: 388B6E60  addi r4, r11, 0x6e60
	ctx.r[4].s64 = ctx.r[11].s64 + 28256;
	// 82D83F58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D83F5C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D83F60: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82D83F64: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D83F68: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D83F6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D83F70: 4E800421  bctrl
	ctx.lr = 0x82D83F74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D83F74: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82D83F78: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D83F7C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82D83F80: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82D83F84: 39000160  li r8, 0x160
	ctx.r[8].s64 = 352;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D84038 size=620
    let mut pc: u32 = 0x82D84038;
    'dispatch: loop {
        match pc {
            0x82D84038 => {
    //   block [0x82D84038..0x82D840A8)
	// 82D84038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8403C: 4BF253CD  bl 0x82ca9408
	ctx.lr = 0x82D84040;
	sub_82CA93D0(ctx, base);
	// 82D84040: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82D84044: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84048: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D8404C: D04100C4  stfs f2, 0xc4(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82D84050: D06100CC  stfs f3, 0xcc(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82D84054: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D84058: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 82D8405C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82D84060: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82D84064: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82D84068: 419901AC  bgt cr6, 0x82d84214
	if ctx.cr[6].gt {
	pc = 0x82D84214; continue 'dispatch;
	}
	// 82D8406C: 3D8082D8  lis r12, -0x7d28
	ctx.r[12].s64 = -2099773440;
	// 82D84070: 398C4084  addi r12, r12, 0x4084
	ctx.r[12].s64 = ctx.r[12].s64 + 16516;
	// 82D84074: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D84078: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D8407C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D84080: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82D84150; continue 'dispatch;
		},
		1 => {
	pc = 0x82D840A8; continue 'dispatch;
		},
		2 => {
	pc = 0x82D84108; continue 'dispatch;
		},
		3 => {
	pc = 0x82D840D8; continue 'dispatch;
		},
		4 => {
	pc = 0x82D84138; continue 'dispatch;
		},
		5 => {
	pc = 0x82D841BC; continue 'dispatch;
		},
		6 => {
	pc = 0x82D84214; continue 'dispatch;
		},
		7 => {
	pc = 0x82D840F0; continue 'dispatch;
		},
		8 => {
	pc = 0x82D841E4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D84084: 82D84150  lwz r22, 0x4150(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16720 as u32) ) } as u64;
	// 82D84088: 82D840A8  lwz r22, 0x40a8(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16552 as u32) ) } as u64;
	// 82D8408C: 82D84108  lwz r22, 0x4108(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16648 as u32) ) } as u64;
	// 82D84090: 82D840D8  lwz r22, 0x40d8(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16600 as u32) ) } as u64;
	// 82D84094: 82D84138  lwz r22, 0x4138(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16696 as u32) ) } as u64;
	// 82D84098: 82D841BC  lwz r22, 0x41bc(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16828 as u32) ) } as u64;
	// 82D8409C: 82D84214  lwz r22, 0x4214(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16916 as u32) ) } as u64;
	// 82D840A0: 82D840F0  lwz r22, 0x40f0(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16624 as u32) ) } as u64;
	// 82D840A4: 82D841E4  lwz r22, 0x41e4(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16868 as u32) ) } as u64;
            }
            0x82D840A8 => {
    //   block [0x82D840A8..0x82D840D8)
	// 82D840A8: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D840AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D840B0: 419A017C  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D840B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D840B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D840BC: 480069E5  bl 0x82d8aaa0
	ctx.lr = 0x82D840C0;
	sub_82D8AAA0(ctx, base);
	// 82D840C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D840C4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D840C8: 396BC29C  addi r11, r11, -0x3d64
	ctx.r[11].s64 = ctx.r[11].s64 + -15716;
	// 82D840CC: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82D840D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D840D4: 4800015C  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D840D8 => {
    //   block [0x82D840D8..0x82D840F0)
	// 82D840D8: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D840DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D840E0: 419A014C  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D840E4: 4801D845  bl 0x82da1928
	ctx.lr = 0x82D840E8;
	sub_82DA1928(ctx, base);
	// 82D840E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D840EC: 48000144  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D840F0 => {
    //   block [0x82D840F0..0x82D84108)
	// 82D840F0: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D840F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D840F8: 419A0134  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D840FC: 48020DAD  bl 0x82da4ea8
	ctx.lr = 0x82D84100;
	sub_82DA4EA8(ctx, base);
	// 82D84100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84104: 4800012C  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D84108 => {
    //   block [0x82D84108..0x82D84138)
	// 82D84108: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D8410C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D84110: 419A011C  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D84114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D84118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D8411C: 48006985  bl 0x82d8aaa0
	ctx.lr = 0x82D84120;
	sub_82D8AAA0(ctx, base);
	// 82D84120: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D84124: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82D84128: 396BC564  addi r11, r11, -0x3a9c
	ctx.r[11].s64 = ctx.r[11].s64 + -15004;
	// 82D8412C: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82D84130: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D84134: 480000FC  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D84138 => {
    //   block [0x82D84138..0x82D84150)
	// 82D84138: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D8413C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D84140: 419A00EC  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D84144: 48020D1D  bl 0x82da4e60
	ctx.lr = 0x82D84148;
	sub_82DA4E60(ctx, base);
	// 82D84148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D8414C: 480000E4  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D84150 => {
    //   block [0x82D84150..0x82D841BC)
	// 82D84150: C19E0028  lfs f12, 0x28(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D84154: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D84158: C1BE0014  lfs f13, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D8415C: ED6D6028  fsubs f11, f13, f12
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82D84160: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84164: FD4B636E  fsel f10, f11, f13, f12
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 82D84168: FDAB6B2E  fsel f13, f11, f12, f13
	ctx.f[13].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[13].f64 };
	// 82D8416C: ED805028  fsubs f12, f0, f10
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82D84170: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D84174: FD8C502E  fsel f12, f12, f0, f10
	ctx.f[12].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[10].f64 };
	// 82D84178: FDAB036E  fsel f13, f11, f13, f0
	ctx.f[13].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82D8417C: C00B0B30  lfs f0, 0xb30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84180: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D84184: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82D84188: 4099FF50  ble cr6, 0x82d840d8
	if !ctx.cr[6].gt {
	pc = 0x82D840D8; continue 'dispatch;
	}
	// 82D8418C: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D84190: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D84194: 419A0098  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D84198: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D8419C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D841A0: 48006901  bl 0x82d8aaa0
	ctx.lr = 0x82D841A4;
	sub_82D8AAA0(ctx, base);
	// 82D841A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D841A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D841AC: 396BC29C  addi r11, r11, -0x3d64
	ctx.r[11].s64 = ctx.r[11].s64 + -15716;
	// 82D841B0: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82D841B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D841B8: 48000078  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D841BC => {
    //   block [0x82D841BC..0x82D841E4)
	// 82D841BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D841C0: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D841C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D841C8: C00B0A48  lfs f0, 0xa48(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2632 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D841CC: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82D841D0: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82D841D4: 419A0058  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D841D8: 48017B11  bl 0x82d9bce8
	ctx.lr = 0x82D841DC;
	sub_82D9BCE8(ctx, base);
	// 82D841DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D841E0: 48000050  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D841E4 => {
    //   block [0x82D841E4..0x82D84214)
	// 82D841E4: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D841E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D841EC: 419A0040  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D841F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D841F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D841F8: 480068A9  bl 0x82d8aaa0
	ctx.lr = 0x82D841FC;
	sub_82D8AAA0(ctx, base);
	// 82D841FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D84200: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 82D84204: 396BB974  addi r11, r11, -0x468c
	ctx.r[11].s64 = ctx.r[11].s64 + -18060;
	// 82D84208: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82D8420C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D84210: 48000020  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D84214 => {
    //   block [0x82D84214..0x82D8422C)
	// 82D84214: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D84218: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8421C: 419A0010  beq cr6, 0x82d8422c
	if ctx.cr[6].eq {
	pc = 0x82D8422C; continue 'dispatch;
	}
	// 82D84220: 48017C71  bl 0x82d9be90
	ctx.lr = 0x82D84224;
	sub_82D9BE90(ctx, base);
	// 82D84224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84228: 48000008  b 0x82d84230
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D8422C => {
    //   block [0x82D8422C..0x82D84230)
	// 82D8422C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82D84230; continue 'dispatch;
            }
            0x82D84230 => {
    //   block [0x82D84230..0x82D84280)
	// 82D84230: 2F1D0006  cmpwi cr6, r29, 6
	ctx.cr[6].compare_i32(ctx.r[29].s32, 6, &mut ctx.xer);
	// 82D84234: 419A004C  beq cr6, 0x82d84280
	if ctx.cr[6].eq {
	pc = 0x82D84280; continue 'dispatch;
	}
	// 82D84238: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8423C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D84240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84244: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D84248: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D8424C: 4E800421  bctrl
	ctx.lr = 0x82D84250;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84250: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84254: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D84258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D8425C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D84260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84264: 4E800421  bctrl
	ctx.lr = 0x82D84268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8426C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D84270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84274: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D84278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D8427C: 4E800421  bctrl
	ctx.lr = 0x82D84280;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84280 => {
    //   block [0x82D84280..0x82D842A4)
	// 82D84280: 388100C4  addi r4, r1, 0xc4
	ctx.r[4].s64 = ctx.r[1].s64 + 196;
	// 82D84284: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 82D84288: 4B7DC089  bl 0x82560310
	ctx.lr = 0x82D8428C;
	sub_82560310(ctx, base);
	// 82D8428C: 388100CC  addi r4, r1, 0xcc
	ctx.r[4].s64 = ctx.r[1].s64 + 204;
	// 82D84290: 387F00BD  addi r3, r31, 0xbd
	ctx.r[3].s64 = ctx.r[31].s64 + 189;
	// 82D84294: 4B7DC07D  bl 0x82560310
	ctx.lr = 0x82D84298;
	sub_82560310(ctx, base);
	// 82D84298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D8429C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82D842A0: 4BF251B8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D842A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D842A8 size=608
    let mut pc: u32 = 0x82D842A8;
    'dispatch: loop {
        match pc {
            0x82D842A8 => {
    //   block [0x82D842A8..0x82D84324)
	// 82D842A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D842AC: 4BF25161  bl 0x82ca940c
	ctx.lr = 0x82D842B0;
	sub_82CA93D0(ctx, base);
	// 82D842B0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82D842B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D842B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D842BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D842C0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D842C4: 48001755  bl 0x82d85a18
	ctx.lr = 0x82D842C8;
	sub_82D85A18(ctx, base);
	// 82D842C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D842CC: 396BB944  addi r11, r11, -0x46bc
	ctx.r[11].s64 = ctx.r[11].s64 + -18108;
	// 82D842D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D842D4: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D842D8: 997F0080  stb r11, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82D842DC: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82D842E0: B17F0096  sth r11, 0x96(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(150 as u32), ctx.r[11].u16 ) };
	// 82D842E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D842E8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82D842EC: 897E00B0  lbz r11, 0xb0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82D842F0: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82D842F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D842F8: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82D842FC: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82D84300: 409A006C  bne cr6, 0x82d8436c
	if !ctx.cr[6].eq {
	pc = 0x82D8436C; continue 'dispatch;
	}
	// 82D84304: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8430C: 419A0018  beq cr6, 0x82d84324
	if ctx.cr[6].eq {
	pc = 0x82D84324; continue 'dispatch;
	}
	// 82D84310: 38BE0020  addi r5, r30, 0x20
	ctx.r[5].s64 = ctx.r[30].s64 + 32;
	// 82D84314: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82D84318: 48017B79  bl 0x82d9be90
	ctx.lr = 0x82D8431C;
	sub_82D9BE90(ctx, base);
	// 82D8431C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D84320: 48000008  b 0x82d84328
	pc = 0x82D84328; continue 'dispatch;
            }
            0x82D84324 => {
    //   block [0x82D84324..0x82D84328)
	// 82D84324: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82D84328; continue 'dispatch;
            }
            0x82D84328 => {
    //   block [0x82D84328..0x82D8436C)
	// 82D84328: 389E00A4  addi r4, r30, 0xa4
	ctx.r[4].s64 = ctx.r[30].s64 + 164;
	// 82D8432C: 387D00BC  addi r3, r29, 0xbc
	ctx.r[3].s64 = ctx.r[29].s64 + 188;
	// 82D84330: 4B7DBFE1  bl 0x82560310
	ctx.lr = 0x82D84334;
	sub_82560310(ctx, base);
	// 82D84334: 389E00A8  addi r4, r30, 0xa8
	ctx.r[4].s64 = ctx.r[30].s64 + 168;
	// 82D84338: 387D00BD  addi r3, r29, 0xbd
	ctx.r[3].s64 = ctx.r[29].s64 + 189;
	// 82D8433C: 4B7DBFD5  bl 0x82560310
	ctx.lr = 0x82D84340;
	sub_82560310(ctx, base);
	// 82D84340: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D84344: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D84348: 480066C1  bl 0x82d8aa08
	ctx.lr = 0x82D8434C;
	sub_82D8AA08(ctx, base);
	// 82D8434C: 397F00E0  addi r11, r31, 0xe0
	ctx.r[11].s64 = ctx.r[31].s64 + 224;
	// 82D84350: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D84354: C01E00AC  lfs f0, 0xac(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84358: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82D8435C: 419900A4  bgt cr6, 0x82d84400
	if ctx.cr[6].gt {
	pc = 0x82D84400; continue 'dispatch;
	}
	// 82D84360: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D84364: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84368: 48000098  b 0x82d84400
	pc = 0x82D84400; continue 'dispatch;
            }
            0x82D8436C => {
    //   block [0x82D8436C..0x82D84400)
	// 82D8436C: 3BBF00D0  addi r29, r31, 0xd0
	ctx.r[29].s64 = ctx.r[31].s64 + 208;
	// 82D84370: C07E00A8  lfs f3, 0xa8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82D84374: 391E0080  addi r8, r30, 0x80
	ctx.r[8].s64 = ctx.r[30].s64 + 128;
	// 82D84378: C05E00A4  lfs f2, 0xa4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82D8437C: 38FE0050  addi r7, r30, 0x50
	ctx.r[7].s64 = ctx.r[30].s64 + 80;
	// 82D84380: C03E0090  lfs f1, 0x90(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D84384: 38BE0020  addi r5, r30, 0x20
	ctx.r[5].s64 = ctx.r[30].s64 + 32;
	// 82D84388: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82D8438C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82D84390: 4BFFFCA9  bl 0x82d84038
	ctx.lr = 0x82D84394;
	sub_82D84038(ctx, base);
	// 82D84394: 897E00B2  lbz r11, 0xb2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(178 as u32) ) } as u64;
	// 82D84398: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D8439C: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82D843A0: 48006669  bl 0x82d8aa08
	ctx.lr = 0x82D843A4;
	sub_82D8AA08(ctx, base);
	// 82D843A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D843A8: 48001101  bl 0x82d854a8
	ctx.lr = 0x82D843AC;
	sub_82D854A8(ctx, base);
	// 82D843AC: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D843B0: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 82D843B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D843B8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D843BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D843C0: 4E800421  bctrl
	ctx.lr = 0x82D843C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D843C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D843C8: 480010E1  bl 0x82d854a8
	ctx.lr = 0x82D843CC;
	sub_82D854A8(ctx, base);
	// 82D843CC: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D843D0: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 82D843D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D843D8: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D843DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D843E0: 4E800421  bctrl
	ctx.lr = 0x82D843E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D843E4: 397F00E0  addi r11, r31, 0xe0
	ctx.r[11].s64 = ctx.r[31].s64 + 224;
	// 82D843E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D843EC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D843F0: 897E00B1  lbz r11, 0xb1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(177 as u32) ) } as u64;
	// 82D843F4: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82D843F8: 4BFFFA49  bl 0x82d83e40
	ctx.lr = 0x82D843FC;
	sub_82D83E40(ctx, base);
	// 82D843FC: C01E00AC  lfs f0, 0xac(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            0x82D84400 => {
    //   block [0x82D84400..0x82D84474)
	// 82D84400: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D84404: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D84408: C01E0094  lfs f0, 0x94(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D8440C: D01F0184  stfs f0, 0x184(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 82D84410: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D84414: C01E0098  lfs f0, 0x98(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84418: D01F0188  stfs f0, 0x188(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), tmp.u32 ) };
	// 82D8441C: 419A0064  beq cr6, 0x82d84480
	if ctx.cr[6].eq {
	pc = 0x82D84480; continue 'dispatch;
	}
	// 82D84420: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D84424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84428: 4BFFFB09  bl 0x82d83f30
	ctx.lr = 0x82D8442C;
	sub_82D83F30(ctx, base);
	// 82D8442C: C01F0058  lfs f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84430: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82D84434: 4199004C  bgt cr6, 0x82d84480
	if ctx.cr[6].gt {
	pc = 0x82D84480; continue 'dispatch;
	}
	// 82D84438: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D8443C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D84440: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84444: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D84448: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82D8444C: C1A10068  lfs f13, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D84450: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D84454: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82D84458: C1AB0BFC  lfs f13, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D8445C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82D84460: 40980014  bge cr6, 0x82d84474
	if !ctx.cr[6].lt {
	pc = 0x82D84474; continue 'dispatch;
	}
	// 82D84464: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D84468: C1AB0A94  lfs f13, 0xa94(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D8446C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D84470: 4800000C  b 0x82d8447c
	pc = 0x82D8447C; continue 'dispatch;
            }
            0x82D84474 => {
    //   block [0x82D84474..0x82D8447C)
	// 82D84474: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D84478: C00B0BF8  lfs f0, 0xbf8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82D8447C; continue 'dispatch;
            }
            0x82D8447C => {
    //   block [0x82D8447C..0x82D84480)
	// 82D8447C: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x82D84480; continue 'dispatch;
            }
            0x82D84480 => {
    //   block [0x82D84480..0x82D84494)
	// 82D84480: 897E00B3  lbz r11, 0xb3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(179 as u32) ) } as u64;
	// 82D84484: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D84488: 419A000C  beq cr6, 0x82d84494
	if ctx.cr[6].eq {
	pc = 0x82D84494; continue 'dispatch;
	}
	// 82D8448C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D84490: 4800002C  b 0x82d844bc
	pc = 0x82D844BC; continue 'dispatch;
            }
            0x82D84494 => {
    //   block [0x82D84494..0x82D844A8)
	// 82D84494: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D84498: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82D8449C: 409A000C  bne cr6, 0x82d844a8
	if !ctx.cr[6].eq {
	pc = 0x82D844A8; continue 'dispatch;
	}
	// 82D844A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D844A4: 48000018  b 0x82d844bc
	pc = 0x82D844BC; continue 'dispatch;
            }
            0x82D844A8 => {
    //   block [0x82D844A8..0x82D844BC)
	// 82D844A8: 897E00B0  lbz r11, 0xb0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82D844AC: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82D844B0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82D844B4: 419A0008  beq cr6, 0x82d844bc
	if ctx.cr[6].eq {
	pc = 0x82D844BC; continue 'dispatch;
	}
	// 82D844B8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	pc = 0x82D844BC; continue 'dispatch;
            }
            0x82D844BC => {
    //   block [0x82D844BC..0x82D844E8)
	// 82D844BC: B17F002A  sth r11, 0x2a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 82D844C0: 897E00B5  lbz r11, 0xb5(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(181 as u32) ) } as u64;
	// 82D844C4: 997F00BD  stb r11, 0xbd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(189 as u32), ctx.r[11].u8 ) };
	// 82D844C8: 897E00B4  lbz r11, 0xb4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 82D844CC: 997F00BC  stb r11, 0xbc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[11].u8 ) };
	// 82D844D0: 897E00B6  lbz r11, 0xb6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(182 as u32) ) } as u64;
	// 82D844D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D844D8: 419A0010  beq cr6, 0x82d844e8
	if ctx.cr[6].eq {
	pc = 0x82D844E8; continue 'dispatch;
	}
	// 82D844DC: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82D844E0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82D844E4: 997F0021  stb r11, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	pc = 0x82D844E8; continue 'dispatch;
            }
            0x82D844E8 => {
    //   block [0x82D844E8..0x82D84508)
	// 82D844E8: C01E009C  lfs f0, 0x9c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D844EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D844F0: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82D844F4: C01E00A0  lfs f0, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D844F8: D01F0088  stfs f0, 0x88(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82D844FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D84500: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82D84504: 4BF24F58  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84508 size=120
    let mut pc: u32 = 0x82D84508;
    'dispatch: loop {
        match pc {
            0x82D84508 => {
    //   block [0x82D84508..0x82D84564)
	// 82D84508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8450C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84514: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D84518: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D8451C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84520: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D84524: 419A0040  beq cr6, 0x82d84564
	if ctx.cr[6].eq {
	pc = 0x82D84564; continue 'dispatch;
	}
	// 82D84528: 81230084  lwz r9, 0x84(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D8452C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D84530: 419A0034  beq cr6, 0x82d84564
	if ctx.cr[6].eq {
	pc = 0x82D84564; continue 'dispatch;
	}
	// 82D84534: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D84538: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D8453C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D84540: 99410058  stb r10, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u8 ) };
	// 82D84544: 98A10059  stb r5, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[5].u8 ) };
	// 82D84548: 98C1005A  stb r6, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[6].u8 ) };
	// 82D8454C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82D84550: 4BFF3511  bl 0x82d77a60
	ctx.lr = 0x82D84554;
	sub_82D77A60(ctx, base);
	// 82D84554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D84558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8455C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84560: 4E800020  blr
	return;
            }
            0x82D84564 => {
    //   block [0x82D84564..0x82D84580)
	// 82D84564: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82D84568: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D8456C: 48014B05  bl 0x82d99070
	ctx.lr = 0x82D84570;
	sub_82D99070(ctx, base);
	// 82D84570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D84574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D8457C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D84580 size=496
    let mut pc: u32 = 0x82D84580;
    'dispatch: loop {
        match pc {
            0x82D84580 => {
    //   block [0x82D84580..0x82D845D0)
	// 82D84580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84584: 4BF24E89  bl 0x82ca940c
	ctx.lr = 0x82D84588;
	sub_82CA93D0(ctx, base);
	// 82D84588: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8458C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84590: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D84594: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84598: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8459C: 419A004C  beq cr6, 0x82d845e8
	if ctx.cr[6].eq {
	pc = 0x82D845E8; continue 'dispatch;
	}
	// 82D845A0: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D845A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D845A8: 419A0028  beq cr6, 0x82d845d0
	if ctx.cr[6].eq {
	pc = 0x82D845D0; continue 'dispatch;
	}
	// 82D845AC: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82D845B0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D845B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D845B8: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82D845BC: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D845C0: 4BFF34A1  bl 0x82d77a60
	ctx.lr = 0x82D845C4;
	sub_82D77A60(ctx, base);
	// 82D845C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D845C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D845CC: 4BF24E90  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82D845D0 => {
    //   block [0x82D845D0..0x82D845E8)
	// 82D845D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D845D4: 419A0014  beq cr6, 0x82d845e8
	if ctx.cr[6].eq {
	pc = 0x82D845E8; continue 'dispatch;
	}
	// 82D845D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D845DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D845E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D845E4: 48000CB5  bl 0x82d85298
	ctx.lr = 0x82D845E8;
	sub_82D85298(ctx, base);
	pc = 0x82D845E8; continue 'dispatch;
            }
            0x82D845E8 => {
    //   block [0x82D845E8..0x82D8460C)
	// 82D845E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D845EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D845F0: 419A001C  beq cr6, 0x82d8460c
	if ctx.cr[6].eq {
	pc = 0x82D8460C; continue 'dispatch;
	}
	// 82D845F4: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D845F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D845FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D84600: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82D84604: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84608: 480135E9  bl 0x82d97bf0
	ctx.lr = 0x82D8460C;
	sub_82D97BF0(ctx, base);
	pc = 0x82D8460C; continue 'dispatch;
            }
            0x82D8460C => {
    //   block [0x82D8460C..0x82D8462C)
	// 82D8460C: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D84610: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82D84614: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D84618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D8461C: 419A0010  beq cr6, 0x82d8462c
	if ctx.cr[6].eq {
	pc = 0x82D8462C; continue 'dispatch;
	}
	// 82D84620: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D84624: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D84628: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82D8462C; continue 'dispatch;
            }
            0x82D8462C => {
    //   block [0x82D8462C..0x82D84670)
	// 82D8462C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D84630: 419A0040  beq cr6, 0x82d84670
	if ctx.cr[6].eq {
	pc = 0x82D84670; continue 'dispatch;
	}
	// 82D84634: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D84638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D8463C: 419A0034  beq cr6, 0x82d84670
	if ctx.cr[6].eq {
	pc = 0x82D84670; continue 'dispatch;
	}
	// 82D84640: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D84644: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D84648: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D8464C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84650: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D84654: 409A001C  bne cr6, 0x82d84670
	if !ctx.cr[6].eq {
	pc = 0x82D84670; continue 'dispatch;
	}
	// 82D84658: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8465C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D84660: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D84664: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D8466C: 4E800421  bctrl
	ctx.lr = 0x82D84670;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84670 => {
    //   block [0x82D84670..0x82D846A8)
	// 82D84670: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D84674: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D84678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D8467C: 4BFFF8B5  bl 0x82d83f30
	ctx.lr = 0x82D84680;
	sub_82D83F30(ctx, base);
	// 82D84680: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D84684: 419A0024  beq cr6, 0x82d846a8
	if ctx.cr[6].eq {
	pc = 0x82D846A8; continue 'dispatch;
	}
	// 82D84688: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D8468C: C1BF0058  lfs f13, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D84690: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84694: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82D84698: 419A0010  beq cr6, 0x82d846a8
	if ctx.cr[6].eq {
	pc = 0x82D846A8; continue 'dispatch;
	}
	// 82D8469C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D846A0: C00B0EE0  lfs f0, 0xee0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D846A4: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x82D846A8; continue 'dispatch;
            }
            0x82D846A8 => {
    //   block [0x82D846A8..0x82D846F8)
	// 82D846A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D846AC: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D846B0: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D846B4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82D846B8: 4199004C  bgt cr6, 0x82d84704
	if ctx.cr[6].gt {
	pc = 0x82D84704; continue 'dispatch;
	}
	// 82D846BC: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D846C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D846C4: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D846C8: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D846CC: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82D846D0: C1A10068  lfs f13, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D846D4: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D846D8: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82D846DC: C1AB0BFC  lfs f13, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D846E0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82D846E4: 40980014  bge cr6, 0x82d846f8
	if !ctx.cr[6].lt {
	pc = 0x82D846F8; continue 'dispatch;
	}
	// 82D846E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D846EC: C1AB0A94  lfs f13, 0xa94(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D846F0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D846F4: 4800000C  b 0x82d84700
	pc = 0x82D84700; continue 'dispatch;
            }
            0x82D846F8 => {
    //   block [0x82D846F8..0x82D84700)
	// 82D846F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D846FC: C00B0BF8  lfs f0, 0xbf8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82D84700; continue 'dispatch;
            }
            0x82D84700 => {
    //   block [0x82D84700..0x82D84704)
	// 82D84700: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x82D84704; continue 'dispatch;
            }
            0x82D84704 => {
    //   block [0x82D84704..0x82D84718)
	// 82D84704: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84708: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D8470C: 419A000C  beq cr6, 0x82d84718
	if ctx.cr[6].eq {
	pc = 0x82D84718; continue 'dispatch;
	}
	// 82D84710: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D84714: 4800F685  bl 0x82d93d98
	ctx.lr = 0x82D84718;
	sub_82D93D98(ctx, base);
	pc = 0x82D84718; continue 'dispatch;
            }
            0x82D84718 => {
    //   block [0x82D84718..0x82D84764)
	// 82D84718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D8471C: 4800ECBD  bl 0x82d933d8
	ctx.lr = 0x82D84720;
	sub_82D933D8(ctx, base);
	// 82D84720: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84724: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D84728: 419A003C  beq cr6, 0x82d84764
	if ctx.cr[6].eq {
	pc = 0x82D84764; continue 'dispatch;
	}
	// 82D8472C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D84730: 48013211  bl 0x82d97940
	ctx.lr = 0x82D84734;
	sub_82D97940(ctx, base);
	// 82D84734: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84738: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D8473C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84740: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82D84744: 40820020  bne 0x82d84764
	if !ctx.cr[0].eq {
	pc = 0x82D84764; continue 'dispatch;
	}
	// 82D84748: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D8474C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84750: 419A0014  beq cr6, 0x82d84764
	if ctx.cr[6].eq {
	pc = 0x82D84764; continue 'dispatch;
	}
	// 82D84754: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D84758: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D8475C: 409A0008  bne cr6, 0x82d84764
	if !ctx.cr[6].eq {
	pc = 0x82D84764; continue 'dispatch;
	}
	// 82D84760: 4BFF32E9  bl 0x82d77a48
	ctx.lr = 0x82D84764;
	sub_82D77A48(ctx, base);
	pc = 0x82D84764; continue 'dispatch;
            }
            0x82D84764 => {
    //   block [0x82D84764..0x82D84770)
	// 82D84764: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D84768: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D8476C: 4BF24CF0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84770 size=76
    let mut pc: u32 = 0x82D84770;
    'dispatch: loop {
        match pc {
            0x82D84770 => {
    //   block [0x82D84770..0x82D847BC)
	// 82D84770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D8477C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84784: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84788: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D8478C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D84790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84794: 4E800421  bctrl
	ctx.lr = 0x82D84798;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84798: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D8479C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D847A0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D847A4: 4BFFF78D  bl 0x82d83f30
	ctx.lr = 0x82D847A8;
	sub_82D83F30(ctx, base);
	// 82D847A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D847AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D847B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D847B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D847B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D847C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D847C0 size=28
    let mut pc: u32 = 0x82D847C0;
    'dispatch: loop {
        match pc {
            0x82D847C0 => {
    //   block [0x82D847C0..0x82D847DC)
	// 82D847C0: 896300D8  lbz r11, 0xd8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D847C4: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82D847C8: 419A0014  beq cr6, 0x82d847dc
	if ctx.cr[6].eq {
		sub_82D847DC(ctx, base);
		return;
	}
	// 82D847CC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82D847D0: 419A000C  beq cr6, 0x82d847dc
	if ctx.cr[6].eq {
		sub_82D847DC(ctx, base);
		return;
	}
	// 82D847D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D847D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D847DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D847DC size=8
    let mut pc: u32 = 0x82D847DC;
    'dispatch: loop {
        match pc {
            0x82D847DC => {
    //   block [0x82D847DC..0x82D847E4)
	// 82D847DC: 806301E8  lwz r3, 0x1e8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(488 as u32) ) } as u64;
	// 82D847E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D847E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D847E8 size=28
    let mut pc: u32 = 0x82D847E8;
    'dispatch: loop {
        match pc {
            0x82D847E8 => {
    //   block [0x82D847E8..0x82D84804)
	// 82D847E8: 896300D8  lbz r11, 0xd8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D847EC: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82D847F0: 419A0014  beq cr6, 0x82d84804
	if ctx.cr[6].eq {
		sub_82D84804(ctx, base);
		return;
	}
	// 82D847F4: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82D847F8: 419A000C  beq cr6, 0x82d84804
	if ctx.cr[6].eq {
		sub_82D84804(ctx, base);
		return;
	}
	// 82D847FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D84800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D84804 size=8
    let mut pc: u32 = 0x82D84804;
    'dispatch: loop {
        match pc {
            0x82D84804 => {
    //   block [0x82D84804..0x82D8480C)
	// 82D84804: 806301E8  lwz r3, 0x1e8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(488 as u32) ) } as u64;
	// 82D84808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84810 size=304
    let mut pc: u32 = 0x82D84810;
    'dispatch: loop {
        match pc {
            0x82D84810 => {
    //   block [0x82D84810..0x82D8485C)
	// 82D84810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D8481C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D84820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D84828: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D8482C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D84830: 419A00F8  beq cr6, 0x82d84928
	if ctx.cr[6].eq {
	pc = 0x82D84928; continue 'dispatch;
	}
	// 82D84834: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D84838: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D8483C: 419A0020  beq cr6, 0x82d8485c
	if ctx.cr[6].eq {
	pc = 0x82D8485C; continue 'dispatch;
	}
	// 82D84840: 39600015  li r11, 0x15
	ctx.r[11].s64 = 21;
	// 82D84844: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82D84848: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82D8484C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84850: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82D84854: 4BFF320D  bl 0x82d77a60
	ctx.lr = 0x82D84858;
	sub_82D77A60(ctx, base);
	// 82D84858: 480000D0  b 0x82d84928
	pc = 0x82D84928; continue 'dispatch;
            }
            0x82D8485C => {
    //   block [0x82D8485C..0x82D848B4)
	// 82D8485C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D84860: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D84868: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82D8486C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D84870: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82D84874: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D84878: 915F0084  stw r10, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82D8487C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82D84880: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84884: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82D84888: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8488C: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D84890: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84894: 4E800421  bctrl
	ctx.lr = 0x82D84898;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84898: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D8489C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D848A0: 419A0014  beq cr6, 0x82d848b4
	if ctx.cr[6].eq {
	pc = 0x82D848B4; continue 'dispatch;
	}
	// 82D848A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D848A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D848AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82D848B0: 4800A689  bl 0x82d8ef38
	ctx.lr = 0x82D848B4;
	sub_82D8EF38(ctx, base);
            }
            0x82D848B4 => {
    //   block [0x82D848B4..0x82D848EC)
	// 82D848B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D848B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D848BC: 4800099D  bl 0x82d85258
	ctx.lr = 0x82D848C0;
	sub_82D85258(ctx, base);
	// 82D848C0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D848C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D848C8: 409A0030  bne cr6, 0x82d848f8
	if !ctx.cr[6].eq {
	pc = 0x82D848F8; continue 'dispatch;
	}
	// 82D848CC: 897F00C6  lbz r11, 0xc6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(198 as u32) ) } as u64;
	// 82D848D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D848D4: 419A0018  beq cr6, 0x82d848ec
	if ctx.cr[6].eq {
	pc = 0x82D848EC; continue 'dispatch;
	}
	// 82D848D8: 897E00D8  lbz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D848DC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82D848E0: 419A000C  beq cr6, 0x82d848ec
	if ctx.cr[6].eq {
	pc = 0x82D848EC; continue 'dispatch;
	}
	// 82D848E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D848E8: 48000BC1  bl 0x82d854a8
	ctx.lr = 0x82D848EC;
	sub_82D854A8(ctx, base);
	pc = 0x82D848EC; continue 'dispatch;
            }
            0x82D848EC => {
    //   block [0x82D848EC..0x82D848F8)
	// 82D848EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D848F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D848F4: 480107FD  bl 0x82d950f0
	ctx.lr = 0x82D848F8;
	sub_82D950F0(ctx, base);
	pc = 0x82D848F8; continue 'dispatch;
            }
            0x82D848F8 => {
    //   block [0x82D848F8..0x82D84928)
	// 82D848F8: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D848FC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84900: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82D84904: 40820024  bne 0x82d84928
	if !ctx.cr[0].eq {
	pc = 0x82D84928; continue 'dispatch;
	}
	// 82D84908: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D8490C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84910: 419A0018  beq cr6, 0x82d84928
	if ctx.cr[6].eq {
	pc = 0x82D84928; continue 'dispatch;
	}
	// 82D84914: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D84918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D8491C: 409A000C  bne cr6, 0x82d84928
	if !ctx.cr[6].eq {
	pc = 0x82D84928; continue 'dispatch;
	}
	// 82D84920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84924: 4BFF3125  bl 0x82d77a48
	ctx.lr = 0x82D84928;
	sub_82D77A48(ctx, base);
	pc = 0x82D84928; continue 'dispatch;
            }
            0x82D84928 => {
    //   block [0x82D84928..0x82D84940)
	// 82D84928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D8492C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84934: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D84938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D8493C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84940 size=68
    let mut pc: u32 = 0x82D84940;
    'dispatch: loop {
        match pc {
            0x82D84940 => {
    //   block [0x82D84940..0x82D84984)
	// 82D84940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84948: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D8494C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84954: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84958: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D8495C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D84960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84964: 4E800421  bctrl
	ctx.lr = 0x82D84968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D8496C: 4BFFFEA5  bl 0x82d84810
	ctx.lr = 0x82D84970;
	sub_82D84810(ctx, base);
	// 82D84970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D84974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D8497C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D84980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84988 size=68
    let mut pc: u32 = 0x82D84988;
    'dispatch: loop {
        match pc {
            0x82D84988 => {
    //   block [0x82D84988..0x82D849CC)
	// 82D84988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D84994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D8499C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D849A0: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D849A4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D849A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D849AC: 4E800421  bctrl
	ctx.lr = 0x82D849B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D849B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D849B4: 4BFFFE5D  bl 0x82d84810
	ctx.lr = 0x82D849B8;
	sub_82D84810(ctx, base);
	// 82D849B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D849BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D849C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D849C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D849C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D849D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D849D0 size=68
    let mut pc: u32 = 0x82D849D0;
    'dispatch: loop {
        match pc {
            0x82D849D0 => {
    //   block [0x82D849D0..0x82D84A14)
	// 82D849D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D849D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D849D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D849DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D849E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D849E4: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D849E8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D849EC: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D849F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D849F4: 4E800421  bctrl
	ctx.lr = 0x82D849F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D849F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D849FC: 4BFFFE15  bl 0x82d84810
	ctx.lr = 0x82D84A00;
	sub_82D84810(ctx, base);
	// 82D84A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D84A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D84A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84A18 size=68
    let mut pc: u32 = 0x82D84A18;
    'dispatch: loop {
        match pc {
            0x82D84A18 => {
    //   block [0x82D84A18..0x82D84A5C)
	// 82D84A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D84A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84A2C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84A30: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D84A34: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D84A38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84A3C: 4E800421  bctrl
	ctx.lr = 0x82D84A40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84A40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84A44: 4BFFFDCD  bl 0x82d84810
	ctx.lr = 0x82D84A48;
	sub_82D84810(ctx, base);
	// 82D84A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D84A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D84A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84A60 size=176
    let mut pc: u32 = 0x82D84A60;
    'dispatch: loop {
        match pc {
            0x82D84A60 => {
    //   block [0x82D84A60..0x82D84AE4)
	// 82D84A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84A64: 4BF249A9  bl 0x82ca940c
	ctx.lr = 0x82D84A68;
	sub_82CA93D0(ctx, base);
	// 82D84A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84A70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D84A74: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D84A78: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84A7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84A80: 419A0064  beq cr6, 0x82d84ae4
	if ctx.cr[6].eq {
	pc = 0x82D84AE4; continue 'dispatch;
	}
	// 82D84A84: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D84A88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84A8C: 419A0058  beq cr6, 0x82d84ae4
	if ctx.cr[6].eq {
	pc = 0x82D84AE4; continue 'dispatch;
	}
	// 82D84A90: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84A94: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D84A98: 39200017  li r9, 0x17
	ctx.r[9].s64 = 23;
	// 82D84A9C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D84AA0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D84AA4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82D84AA8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D84AAC: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82D84AB0: 4BFD0799  bl 0x82d55248
	ctx.lr = 0x82D84AB4;
	sub_82D55248(ctx, base);
	pc = 0x82D84AE4; continue 'dispatch;
            }
            0x82D84AE4 => {
    //   block [0x82D84AE4..0x82D84B10)
	// 82D84AE4: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D84AE8: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84AEC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D84AF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D84AF4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D84AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84AFC: 4E800421  bctrl
	ctx.lr = 0x82D84B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84B04: 4BFFFD0D  bl 0x82d84810
	ctx.lr = 0x82D84B08;
	sub_82D84810(ctx, base);
	// 82D84B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D84B0C: 4BF24950  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D84B10 size=156
    let mut pc: u32 = 0x82D84B10;
    'dispatch: loop {
        match pc {
            0x82D84B10 => {
    //   block [0x82D84B10..0x82D84B74)
	// 82D84B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84B18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D84B1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D84B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84B24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84B28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D84B2C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84B30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D84B34: 419A0040  beq cr6, 0x82d84b74
	if ctx.cr[6].eq {
	pc = 0x82D84B74; continue 'dispatch;
	}
	// 82D84B38: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D84B3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84B40: 419A0034  beq cr6, 0x82d84b74
	if ctx.cr[6].eq {
	pc = 0x82D84B74; continue 'dispatch;
	}
	// 82D84B44: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82D84B48: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84B4C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D84B50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D84B54: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84B58: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D84B5C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82D84B60: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84B64: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82D84B68: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D84B6C: 4BFF2EF5  bl 0x82d77a60
	ctx.lr = 0x82D84B70;
	sub_82D77A60(ctx, base);
	// 82D84B70: 48000024  b 0x82d84b94
	pc = 0x82D84B94; continue 'dispatch;
            }
            0x82D84B74 => {
    //   block [0x82D84B74..0x82D84B94)
	// 82D84B74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84B78: 48000931  bl 0x82d854a8
	ctx.lr = 0x82D84B7C;
	sub_82D854A8(ctx, base);
	// 82D84B7C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D84B80: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84B84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D84B88: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D84B8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84B90: 4E800421  bctrl
	ctx.lr = 0x82D84B94;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84B94 => {
    //   block [0x82D84B94..0x82D84BAC)
	// 82D84B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D84B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84BA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D84BA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D84BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D84BB0 size=156
    let mut pc: u32 = 0x82D84BB0;
    'dispatch: loop {
        match pc {
            0x82D84BB0 => {
    //   block [0x82D84BB0..0x82D84C14)
	// 82D84BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D84BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D84BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84BC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84BC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D84BCC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84BD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D84BD4: 419A0040  beq cr6, 0x82d84c14
	if ctx.cr[6].eq {
	pc = 0x82D84C14; continue 'dispatch;
	}
	// 82D84BD8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D84BDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84BE0: 419A0034  beq cr6, 0x82d84c14
	if ctx.cr[6].eq {
	pc = 0x82D84C14; continue 'dispatch;
	}
	// 82D84BE4: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 82D84BE8: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84BEC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D84BF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D84BF4: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84BF8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D84BFC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82D84C00: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84C04: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82D84C08: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D84C0C: 4BFF2E55  bl 0x82d77a60
	ctx.lr = 0x82D84C10;
	sub_82D77A60(ctx, base);
	// 82D84C10: 48000024  b 0x82d84c34
	pc = 0x82D84C34; continue 'dispatch;
            }
            0x82D84C14 => {
    //   block [0x82D84C14..0x82D84C34)
	// 82D84C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84C18: 48000891  bl 0x82d854a8
	ctx.lr = 0x82D84C1C;
	sub_82D854A8(ctx, base);
	// 82D84C1C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D84C20: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84C24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D84C28: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D84C2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84C30: 4E800421  bctrl
	ctx.lr = 0x82D84C34;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84C34 => {
    //   block [0x82D84C34..0x82D84C4C)
	// 82D84C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D84C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84C40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D84C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D84C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D84C50 size=156
    let mut pc: u32 = 0x82D84C50;
    'dispatch: loop {
        match pc {
            0x82D84C50 => {
    //   block [0x82D84C50..0x82D84CB4)
	// 82D84C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D84C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D84C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84C68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D84C6C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84C70: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D84C74: 419A0040  beq cr6, 0x82d84cb4
	if ctx.cr[6].eq {
	pc = 0x82D84CB4; continue 'dispatch;
	}
	// 82D84C78: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D84C7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84C80: 419A0034  beq cr6, 0x82d84cb4
	if ctx.cr[6].eq {
	pc = 0x82D84CB4; continue 'dispatch;
	}
	// 82D84C84: 3960001A  li r11, 0x1a
	ctx.r[11].s64 = 26;
	// 82D84C88: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84C8C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D84C90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D84C94: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84C98: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D84C9C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82D84CA0: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84CA4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82D84CA8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D84CAC: 4BFF2DB5  bl 0x82d77a60
	ctx.lr = 0x82D84CB0;
	sub_82D77A60(ctx, base);
	// 82D84CB0: 48000024  b 0x82d84cd4
	pc = 0x82D84CD4; continue 'dispatch;
            }
            0x82D84CB4 => {
    //   block [0x82D84CB4..0x82D84CD4)
	// 82D84CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84CB8: 480007F1  bl 0x82d854a8
	ctx.lr = 0x82D84CBC;
	sub_82D854A8(ctx, base);
	// 82D84CBC: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D84CC0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84CC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D84CC8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D84CCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84CD0: 4E800421  bctrl
	ctx.lr = 0x82D84CD4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84CD4 => {
    //   block [0x82D84CD4..0x82D84CEC)
	// 82D84CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D84CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84CE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D84CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D84CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84CF0 size=176
    let mut pc: u32 = 0x82D84CF0;
    'dispatch: loop {
        match pc {
            0x82D84CF0 => {
    //   block [0x82D84CF0..0x82D84D74)
	// 82D84CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84CF4: 4BF24719  bl 0x82ca940c
	ctx.lr = 0x82D84CF8;
	sub_82CA93D0(ctx, base);
	// 82D84CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84D00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D84D04: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D84D08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84D0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84D10: 419A0064  beq cr6, 0x82d84d74
	if ctx.cr[6].eq {
	pc = 0x82D84D74; continue 'dispatch;
	}
	// 82D84D14: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D84D18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84D1C: 419A0058  beq cr6, 0x82d84d74
	if ctx.cr[6].eq {
	pc = 0x82D84D74; continue 'dispatch;
	}
	// 82D84D20: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84D24: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D84D28: 3920001B  li r9, 0x1b
	ctx.r[9].s64 = 27;
	// 82D84D2C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D84D30: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D84D34: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82D84D38: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D84D3C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82D84D40: 4BFD0509  bl 0x82d55248
	ctx.lr = 0x82D84D44;
	sub_82D55248(ctx, base);
	pc = 0x82D84D74; continue 'dispatch;
            }
            0x82D84D74 => {
    //   block [0x82D84D74..0x82D84DA0)
	// 82D84D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84D78: 48000731  bl 0x82d854a8
	ctx.lr = 0x82D84D7C;
	sub_82D854A8(ctx, base);
	// 82D84D7C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D84D80: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84D84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D84D88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D84D8C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D84D90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84D94: 4E800421  bctrl
	ctx.lr = 0x82D84D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D84D9C: 4BF246C0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D84DA0 size=156
    let mut pc: u32 = 0x82D84DA0;
    'dispatch: loop {
        match pc {
            0x82D84DA0 => {
    //   block [0x82D84DA0..0x82D84E04)
	// 82D84DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D84DA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D84DAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D84DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84DB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84DB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D84DBC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D84DC0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D84DC4: 419A0040  beq cr6, 0x82d84e04
	if ctx.cr[6].eq {
	pc = 0x82D84E04; continue 'dispatch;
	}
	// 82D84DC8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D84DCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84DD0: 419A0034  beq cr6, 0x82d84e04
	if ctx.cr[6].eq {
	pc = 0x82D84E04; continue 'dispatch;
	}
	// 82D84DD4: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82D84DD8: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84DDC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D84DE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D84DE4: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84DE8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D84DEC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82D84DF0: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84DF4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82D84DF8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D84DFC: 4BFF2C65  bl 0x82d77a60
	ctx.lr = 0x82D84E00;
	sub_82D77A60(ctx, base);
	// 82D84E00: 48000024  b 0x82d84e24
	pc = 0x82D84E24; continue 'dispatch;
            }
            0x82D84E04 => {
    //   block [0x82D84E04..0x82D84E24)
	// 82D84E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84E08: 480006A1  bl 0x82d854a8
	ctx.lr = 0x82D84E0C;
	sub_82D854A8(ctx, base);
	// 82D84E0C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D84E10: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D84E14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D84E18: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D84E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84E20: 4E800421  bctrl
	ctx.lr = 0x82D84E24;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84E24 => {
    //   block [0x82D84E24..0x82D84E3C)
	// 82D84E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D84E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D84E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D84E30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D84E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D84E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D84E40 size=236
    let mut pc: u32 = 0x82D84E40;
    'dispatch: loop {
        match pc {
            0x82D84E40 => {
    //   block [0x82D84E40..0x82D84E78)
	// 82D84E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84E44: 4BF245C9  bl 0x82ca940c
	ctx.lr = 0x82D84E48;
	sub_82CA93D0(ctx, base);
	// 82D84E48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84E4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D84E50: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D84E54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D84E58: 419A00CC  beq cr6, 0x82d84f24
	if ctx.cr[6].eq {
	pc = 0x82D84F24; continue 'dispatch;
	}
	// 82D84E5C: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D84E60: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D84E64: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82D84E68: 419A0010  beq cr6, 0x82d84e78
	if ctx.cr[6].eq {
	pc = 0x82D84E78; continue 'dispatch;
	}
	// 82D84E6C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82D84E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D84E74: 409A0008  bne cr6, 0x82d84e7c
	if !ctx.cr[6].eq {
	pc = 0x82D84E7C; continue 'dispatch;
	}
	pc = 0x82D84E78; continue 'dispatch;
            }
            0x82D84E78 => {
    //   block [0x82D84E78..0x82D84E7C)
	// 82D84E78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82D84E7C; continue 'dispatch;
            }
            0x82D84E7C => {
    //   block [0x82D84E7C..0x82D84E98)
	// 82D84E7C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D84E80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D84E84: 409A005C  bne cr6, 0x82d84ee0
	if !ctx.cr[6].eq {
	pc = 0x82D84EE0; continue 'dispatch;
	}
	// 82D84E88: 2F1E000B  cmpwi cr6, r30, 0xb
	ctx.cr[6].compare_i32(ctx.r[30].s32, 11, &mut ctx.xer);
	// 82D84E8C: 419A000C  beq cr6, 0x82d84e98
	if ctx.cr[6].eq {
	pc = 0x82D84E98; continue 'dispatch;
	}
	// 82D84E90: 2F1E0015  cmpwi cr6, r30, 0x15
	ctx.cr[6].compare_i32(ctx.r[30].s32, 21, &mut ctx.xer);
	// 82D84E94: 409A0028  bne cr6, 0x82d84ebc
	if !ctx.cr[6].eq {
	pc = 0x82D84EBC; continue 'dispatch;
	}
	pc = 0x82D84E98; continue 'dispatch;
            }
            0x82D84E98 => {
    //   block [0x82D84E98..0x82D84EBC)
	// 82D84E98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D84EA0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D84EA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84EA8: 4E800421  bctrl
	ctx.lr = 0x82D84EAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84EAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84EB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D84EB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84EB8: 4E800421  bctrl
	ctx.lr = 0x82D84EBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84EBC => {
    //   block [0x82D84EBC..0x82D84EE0)
	// 82D84EBC: 2F1E000C  cmpwi cr6, r30, 0xc
	ctx.cr[6].compare_i32(ctx.r[30].s32, 12, &mut ctx.xer);
	// 82D84EC0: 409A0020  bne cr6, 0x82d84ee0
	if !ctx.cr[6].eq {
	pc = 0x82D84EE0; continue 'dispatch;
	}
	// 82D84EC4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D84EC8: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82D84ECC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D84ED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84ED4: 4E800421  bctrl
	ctx.lr = 0x82D84ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84ED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D84EDC: 4BF24580  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82D84EE0 => {
    //   block [0x82D84EE0..0x82D84F08)
	// 82D84EE0: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82D84EE4: 409A0024  bne cr6, 0x82d84f08
	if !ctx.cr[6].eq {
	pc = 0x82D84F08; continue 'dispatch;
	}
	// 82D84EE8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D84EEC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82D84EF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D84EF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D84EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84EFC: 4E800421  bctrl
	ctx.lr = 0x82D84F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D84F00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D84F04: 4BF24558  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82D84F08 => {
    //   block [0x82D84F08..0x82D84F24)
	// 82D84F08: 2F1E000A  cmpwi cr6, r30, 0xa
	ctx.cr[6].compare_i32(ctx.r[30].s32, 10, &mut ctx.xer);
	// 82D84F0C: 409A0018  bne cr6, 0x82d84f24
	if !ctx.cr[6].eq {
	pc = 0x82D84F24; continue 'dispatch;
	}
	// 82D84F10: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D84F14: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82D84F18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D84F1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D84F20: 4E800421  bctrl
	ctx.lr = 0x82D84F24;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82D84F24 => {
    //   block [0x82D84F24..0x82D84F2C)
	// 82D84F24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D84F28: 4BF24534  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D84F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D84F30 size=340
    let mut pc: u32 = 0x82D84F30;
    'dispatch: loop {
        match pc {
            0x82D84F30 => {
    //   block [0x82D84F30..0x82D85084)
	// 82D84F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D84F34: 4BF244D9  bl 0x82ca940c
	ctx.lr = 0x82D84F38;
	sub_82CA93D0(ctx, base);
	// 82D84F38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D84F3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D84F40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D84F44: 394001A0  li r10, 0x1a0
	ctx.r[10].s64 = 416;
	// 82D84F48: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82D84F4C: 390001B0  li r8, 0x1b0
	ctx.r[8].s64 = 432;
	// 82D84F50: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82D84F54: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82D84F58: 3BBF00D0  addi r29, r31, 0xd0
	ctx.r[29].s64 = ctx.r[31].s64 + 208;
	// 82D84F5C: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D84F60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D84F64: 997E00B6  stb r11, 0xb6(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(182 as u32), ctx.r[11].u8 ) };
	// 82D84F68: 897F00BC  lbz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82D84F6C: 997E00B4  stb r11, 0xb4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[11].u8 ) };
	// 82D84F70: 897F00BD  lbz r11, 0xbd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(189 as u32) ) } as u64;
	// 82D84F74: 997E00B5  stb r11, 0xb5(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(181 as u32), ctx.r[11].u8 ) };
	// 82D84F78: A17F0096  lhz r11, 0x96(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(150 as u32) ) } as u64;
	// 82D84F7C: B17E000A  sth r11, 0xa(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82D84F80: 897F00D9  lbz r11, 0xd9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(217 as u32) ) } as u64;
	// 82D84F84: 396BFF01  addi r11, r11, -0xff
	ctx.r[11].s64 = ctx.r[11].s64 + -255;
	// 82D84F88: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D84F8C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D84F90: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D84F94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D84F98: 997E00B1  stb r11, 0xb1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(177 as u32), ctx.r[11].u8 ) };
	// 82D84F9C: C01F0084  lfs f0, 0x84(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84FA0: D01E009C  stfs f0, 0x9c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82D84FA4: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D84FA8: 997E0008  stb r11, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82D84FAC: C01F0088  lfs f0, 0x88(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84FB0: D01E00A0  stfs f0, 0xa0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82D84FB4: C01F0184  lfs f0, 0x184(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84FB8: D01E0094  stfs f0, 0x94(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82D84FBC: C01F0188  lfs f0, 0x188(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D84FC0: D01E0098  stfs f0, 0x98(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D85088 size=352
    let mut pc: u32 = 0x82D85088;
    'dispatch: loop {
        match pc {
            0x82D85088 => {
    //   block [0x82D85088..0x82D850F4)
	// 82D85088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8508C: 4BF2437D  bl 0x82ca9408
	ctx.lr = 0x82D85090;
	sub_82CA93D0(ctx, base);
	// 82D85090: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D85094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D85098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D8509C: 4BFFEB7D  bl 0x82d83c18
	ctx.lr = 0x82D850A0;
	sub_82D83C18(ctx, base);
	// 82D850A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D850A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D850A8: 4BFFFE89  bl 0x82d84f30
	ctx.lr = 0x82D850AC;
	sub_82D84F30(ctx, base);
	// 82D850AC: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D850B0: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D850B4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82D850B8: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 82D850BC: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D850C0: 4BFD0189  bl 0x82d55248
	ctx.lr = 0x82D850C4;
	sub_82D55248(ctx, base);
	// 82D850C4: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 82D850C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D850CC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D850D0: 4BFFF1D9  bl 0x82d842a8
	ctx.lr = 0x82D850D4;
	sub_82D842A8(ctx, base);
	// 82D850D4: 39600012  li r11, 0x12
	ctx.r[11].s64 = 18;
	// 82D850D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D850DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D850E0: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 82D850E4: 7D3EF850  subf r9, r30, r31
	ctx.r[9].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 82D850E8: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82D850EC: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82D850F0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	pc = 0x82D850F4; continue 'dispatch;
            }
            0x82D850F4 => {
    //   block [0x82D850F4..0x82D85170)
	// 82D850F4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D850F8: 910AFFF8  stw r8, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	// 82D850FC: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D85100: 910AFFFC  stw r8, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 82D85104: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D85108: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D8510C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D85110: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D85114: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D85118: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D8511C: 4200FFD8  bdnz 0x82d850f4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82D850F4; continue 'dispatch;
	}
	// 82D85120: 817F01E8  lwz r11, 0x1e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82D85124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D85128: 419A007C  beq cr6, 0x82d851a4
	if ctx.cr[6].eq {
	pc = 0x82D851A4; continue 'dispatch;
	}
	// 82D8512C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82D85130: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D85134: 38800120  li r4, 0x120
	ctx.r[4].s64 = 288;
	// 82D85138: 4BFD0111  bl 0x82d55248
	ctx.lr = 0x82D8513C;
	sub_82D55248(ctx, base);
	// 82D8513C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82D85140: 39400120  li r10, 0x120
	ctx.r[10].s64 = 288;
	// 82D85144: 396B915C  addi r11, r11, -0x6ea4
	ctx.r[11].s64 = ctx.r[11].s64 + -28324;
	// 82D85148: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82D8514C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D85150: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82D85154: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D85158: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D8515C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D85160: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82D85164: 90E30118  stw r7, 0x118(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(280 as u32), ctx.r[7].u32 ) };
	// 82D85168: 907E01E8  stw r3, 0x1e8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(488 as u32), ctx.r[3].u32 ) };
	// 82D8516C: 817F01E8  lwz r11, 0x1e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	pc = 0x82D85170; continue 'dispatch;
            }
            0x82D85170 => {
    //   block [0x82D85170..0x82D851A4)
	// 82D85170: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D85174: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D85178: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D8517C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D85180: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D85184: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D85188: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D8518C: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82D85190: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D85194: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D85198: 910A000C  stw r8, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D8519C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D851A0: 4199FFD0  bgt cr6, 0x82d85170
	if ctx.cr[6].gt {
	pc = 0x82D85170; continue 'dispatch;
	}
	pc = 0x82D851A4; continue 'dispatch;
            }
            0x82D851A4 => {
    //   block [0x82D851A4..0x82D851E8)
	// 82D851A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D851A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D851AC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D851B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D851B4: 4E800421  bctrl
	ctx.lr = 0x82D851B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D851B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D851BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D851C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D851C4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D851C8: 4BFFB041  bl 0x82d80208
	ctx.lr = 0x82D851CC;
	sub_82D80208(ctx, base);
	// 82D851CC: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D851D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D851D4: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82D851D8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D851DC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D851E0: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82D851E4: 4BF24274  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D851E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D851E8 size=72
    let mut pc: u32 = 0x82D851E8;
    'dispatch: loop {
        match pc {
            0x82D851E8 => {
    //   block [0x82D851E8..0x82D85220)
	// 82D851E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D851EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D851F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D851F4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D851F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D851FC: 419A0024  beq cr6, 0x82d85220
	if ctx.cr[6].eq {
	pc = 0x82D85220; continue 'dispatch;
	}
	// 82D85200: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D85204: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D85208: 419A0018  beq cr6, 0x82d85220
	if ctx.cr[6].eq {
	pc = 0x82D85220; continue 'dispatch;
	}
	// 82D8520C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D85210: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82D85214: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D85218: 806B006C  lwz r3, 0x6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82D8521C: 4801FD0D  bl 0x82da4f28
	ctx.lr = 0x82D85220;
	sub_82DA4F28(ctx, base);
	pc = 0x82D85220; continue 'dispatch;
            }
            0x82D85220 => {
    //   block [0x82D85220..0x82D85230)
	// 82D85220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D85224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D85228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D8522C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D85258 size=20
    let mut pc: u32 = 0x82D85258;
    'dispatch: loop {
        match pc {
            0x82D85258 => {
    //   block [0x82D85258..0x82D8526C)
	// 82D85258: 816400B8  lwz r11, 0xb8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D8525C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D85260: 409A000C  bne cr6, 0x82d8526c
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82D8526C);
		return;
	}
	// 82D85264: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D85268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D85298 size=296
    let mut pc: u32 = 0x82D85298;
    'dispatch: loop {
        match pc {
            0x82D85298 => {
    //   block [0x82D85298..0x82D852E4)
	// 82D85298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8529C: 4BF2416D  bl 0x82ca9408
	ctx.lr = 0x82D852A0;
	sub_82CA93D0(ctx, base);
	// 82D852A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D852A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D852A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D852AC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D852B0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82D852B4: 809D0054  lwz r4, 0x54(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D852B8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D852BC: 419A0028  beq cr6, 0x82d852e4
	if ctx.cr[6].eq {
	pc = 0x82D852E4; continue 'dispatch;
	}
	// 82D852C0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D852C4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D852C8: A13D0050  lhz r9, 0x50(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D852CC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82D852D0: 5525283E  rotlwi r5, r9, 5
	ctx.r[5].u64 = ((ctx.r[9].u32).rotate_left(5)) as u64;
	// 82D852D4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D852D8: 4BFCFFF1  bl 0x82d552c8
	ctx.lr = 0x82D852DC;
	sub_82D552C8(ctx, base);
	// 82D852DC: B39D0050  sth r28, 0x50(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(80 as u32), ctx.r[28].u16 ) };
	// 82D852E0: 939D0054  stw r28, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	pc = 0x82D852E4; continue 'dispatch;
            }
            0x82D852E4 => {
    //   block [0x82D852E4..0x82D85330)
	// 82D852E4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D852E8: 419A00D0  beq cr6, 0x82d853b8
	if ctx.cr[6].eq {
	pc = 0x82D853B8; continue 'dispatch;
	}
	// 82D852EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D852F0: 419A00C8  beq cr6, 0x82d853b8
	if ctx.cr[6].eq {
	pc = 0x82D853B8; continue 'dispatch;
	}
	// 82D852F4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D852F8: 815E0074  lwz r10, 0x74(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D852FC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82D85300: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 82D85304: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D85308: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D8530C: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D85310: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D85314: 419A001C  beq cr6, 0x82d85330
	if ctx.cr[6].eq {
	pc = 0x82D85330; continue 'dispatch;
	}
	// 82D85318: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8531C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D85320: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D85324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D85328: 4E800421  bctrl
	ctx.lr = 0x82D8532C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D8532C: 48000028  b 0x82d85354
	pc = 0x82D85354; continue 'dispatch;
            }
            0x82D85330 => {
    //   block [0x82D85330..0x82D85354)
	// 82D85330: 556B0294  rlwinm r11, r11, 0, 0xa, 0xa
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D85334: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D85338: 419A0080  beq cr6, 0x82d853b8
	if ctx.cr[6].eq {
	pc = 0x82D853B8; continue 'dispatch;
	}
	// 82D8533C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D85340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D85344: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D85348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D8534C: 4E800421  bctrl
	ctx.lr = 0x82D85350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D85350: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
            }
            0x82D85354 => {
    //   block [0x82D85354..0x82D8536C)
	// 82D85354: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D85358: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82D8535C: 419A0010  beq cr6, 0x82d8536c
	if ctx.cr[6].eq {
	pc = 0x82D8536C; continue 'dispatch;
	}
	// 82D85360: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82D85364: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82D85368: 409A0008  bne cr6, 0x82d85370
	if !ctx.cr[6].eq {
	pc = 0x82D85370; continue 'dispatch;
	}
	pc = 0x82D8536C; continue 'dispatch;
            }
            0x82D8536C => {
    //   block [0x82D8536C..0x82D85370)
	// 82D8536C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82D85370; continue 'dispatch;
            }
            0x82D85370 => {
    //   block [0x82D85370..0x82D85388)
	// 82D85370: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D85374: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D85378: 419A0010  beq cr6, 0x82d85388
	if ctx.cr[6].eq {
	pc = 0x82D85388; continue 'dispatch;
	}
	// 82D8537C: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82D85380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D85384: 409A0034  bne cr6, 0x82d853b8
	if !ctx.cr[6].eq {
	pc = 0x82D853B8; continue 'dispatch;
	}
	pc = 0x82D85388; continue 'dispatch;
            }
            0x82D85388 => {
    //   block [0x82D85388..0x82D853B8)
	// 82D85388: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8538C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D85390: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D85394: 4E800421  bctrl
	ctx.lr = 0x82D85398;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D85398: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D8539C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D853A0: B07D0050  sth r3, 0x50(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(80 as u32), ctx.r[3].u16 ) };
	// 82D853A4: 54642834  slwi r4, r3, 5
	ctx.r[4].u32 = ctx.r[3].u32.wrapping_shl(5);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D853A8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82D853AC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D853B0: 4BFCFE99  bl 0x82d55248
	ctx.lr = 0x82D853B4;
	sub_82D55248(ctx, base);
	// 82D853B4: 907D0054  stw r3, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
            }
            0x82D853B8 => {
    //   block [0x82D853B8..0x82D853C0)
	// 82D853B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D853BC: 4BF2409C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D853C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D853C0 size=76
    let mut pc: u32 = 0x82D853C0;
    'dispatch: loop {
        match pc {
            0x82D853C0 => {
    //   block [0x82D853C0..0x82D853D8)
	// 82D853C0: 810300CC  lwz r8, 0xcc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82D853C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D853C8: A1280004  lhz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D853CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D853D0: 40990024  ble cr6, 0x82d853f4
	if !ctx.cr[6].gt {
	pc = 0x82D853F4; continue 'dispatch;
	}
	// 82D853D4: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82D853D8; continue 'dispatch;
            }
            0x82D853D8 => {
    //   block [0x82D853D8..0x82D853F4)
	// 82D853D8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D853DC: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D853E0: 419A0018  beq cr6, 0x82d853f8
	if ctx.cr[6].eq {
	pc = 0x82D853F8; continue 'dispatch;
	}
	// 82D853E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D853E8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D853EC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D853F0: 4198FFE8  blt cr6, 0x82d853d8
	if ctx.cr[6].lt {
	pc = 0x82D853D8; continue 'dispatch;
	}
	pc = 0x82D853F4; continue 'dispatch;
            }
            0x82D853F4 => {
    //   block [0x82D853F4..0x82D853F8)
	// 82D853F4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82D853F8; continue 'dispatch;
            }
            0x82D853F8 => {
    //   block [0x82D853F8..0x82D8540C)
	// 82D853F8: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D853FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D85400: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D85404: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82D85408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D85410 size=72
    let mut pc: u32 = 0x82D85410;
    'dispatch: loop {
        match pc {
            0x82D85410 => {
    //   block [0x82D85410..0x82D85424)
	// 82D85410: A12301F4  lhz r9, 0x1f4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(500 as u32) ) } as u64;
	// 82D85414: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D85418: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D8541C: 40990024  ble cr6, 0x82d85440
	if !ctx.cr[6].gt {
	pc = 0x82D85440; continue 'dispatch;
	}
	// 82D85420: 814301F0  lwz r10, 0x1f0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(496 as u32) ) } as u64;
	pc = 0x82D85424; continue 'dispatch;
            }
            0x82D85424 => {
    //   block [0x82D85424..0x82D85440)
	// 82D85424: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D85428: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D8542C: 419A0018  beq cr6, 0x82d85444
	if ctx.cr[6].eq {
	pc = 0x82D85444; continue 'dispatch;
	}
	// 82D85430: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D85434: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D85438: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D8543C: 4198FFE8  blt cr6, 0x82d85424
	if ctx.cr[6].lt {
	pc = 0x82D85424; continue 'dispatch;
	}
	pc = 0x82D85440; continue 'dispatch;
            }
            0x82D85440 => {
    //   block [0x82D85440..0x82D85444)
	// 82D85440: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82D85444; continue 'dispatch;
            }
            0x82D85444 => {
    //   block [0x82D85444..0x82D85458)
	// 82D85444: 814301F0  lwz r10, 0x1f0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(496 as u32) ) } as u64;
	// 82D85448: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D8544C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D85450: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82D85454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D85458 size=64
    let mut pc: u32 = 0x82D85458;
    'dispatch: loop {
        match pc {
            0x82D85458 => {
    //   block [0x82D85458..0x82D85474)
	// 82D85458: 81230060  lwz r9, 0x60(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D8545C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D85460: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D85464: 4099002C  ble cr6, 0x82d85490
	if !ctx.cr[6].gt {
	pc = 0x82D85490; continue 'dispatch;
	}
	// 82D85468: 80E3005C  lwz r7, 0x5c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D8546C: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 82D85470: 39670004  addi r11, r7, 4
	ctx.r[11].s64 = ctx.r[7].s64 + 4;
	pc = 0x82D85474; continue 'dispatch;
            }
            0x82D85474 => {
    //   block [0x82D85474..0x82D85490)
	// 82D85474: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D85478: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D8547C: 419A001C  beq cr6, 0x82d85498
	if ctx.cr[6].eq {
		sub_82D85498(ctx, base);
		return;
	}
	// 82D85480: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D85484: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82D85488: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D8548C: 4198FFE8  blt cr6, 0x82d85474
	if ctx.cr[6].lt {
	pc = 0x82D85474; continue 'dispatch;
	}
	pc = 0x82D85490; continue 'dispatch;
            }
            0x82D85490 => {
    //   block [0x82D85490..0x82D85498)
	// 82D85490: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D85494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D85498 size=16
    let mut pc: u32 = 0x82D85498;
    'dispatch: loop {
        match pc {
            0x82D85498 => {
    //   block [0x82D85498..0x82D854A8)
	// 82D85498: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D8549C: 7D6B382E  lwzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D854A0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D854A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D854A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D854A8 size=144
    let mut pc: u32 = 0x82D854A8;
    'dispatch: loop {
        match pc {
            0x82D854A8 => {
    //   block [0x82D854A8..0x82D854D4)
	// 82D854A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D854AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D854B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D854B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D854B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D854BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D854C0: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D854C4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D854C8: 409A000C  bne cr6, 0x82d854d4
	if !ctx.cr[6].eq {
	pc = 0x82D854D4; continue 'dispatch;
	}
	// 82D854CC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82D854D0: 4800000C  b 0x82d854dc
	pc = 0x82D854DC; continue 'dispatch;
            }
            0x82D854D4 => {
    //   block [0x82D854D4..0x82D854DC)
	// 82D854D4: 89640028  lbz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D854D8: 556BE7BE  rlwinm r11, r11, 0x1c, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	pc = 0x82D854DC; continue 'dispatch;
            }
            0x82D854DC => {
    //   block [0x82D854DC..0x82D85524)
	// 82D854DC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D854E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D854E4: 409A0040  bne cr6, 0x82d85524
	if !ctx.cr[6].eq {
	pc = 0x82D85524; continue 'dispatch;
	}
	// 82D854E8: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D854EC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82D854F0: 419A0034  beq cr6, 0x82d85524
	if ctx.cr[6].eq {
	pc = 0x82D85524; continue 'dispatch;
	}
	// 82D854F4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D854F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D854FC: 419A0028  beq cr6, 0x82d85524
	if ctx.cr[6].eq {
	pc = 0x82D85524; continue 'dispatch;
	}
	// 82D85500: B15F00DA  sth r10, 0xda(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(218 as u32), ctx.r[10].u16 ) };
	// 82D85504: B15F00DC  sth r10, 0xdc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[10].u16 ) };
	// 82D85508: 48010CD9  bl 0x82d961e0
	ctx.lr = 0x82D8550C;
	sub_82D961E0(ctx, base);
	// 82D8550C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D85510: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82D85514: 88CB02C7  lbz r6, 0x2c7(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(711 as u32) ) } as u64;
	// 82D85518: 88AB02C6  lbz r5, 0x2c6(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(710 as u32) ) } as u64;
	// 82D8551C: 888B02C5  lbz r4, 0x2c5(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(709 as u32) ) } as u64;
	// 82D85520: 4B8D8651  bl 0x8265db70
	ctx.lr = 0x82D85524;
	sub_8265DB70(ctx, base);
	pc = 0x82D85524; continue 'dispatch;
            }
            0x82D85524 => {
    //   block [0x82D85524..0x82D85538)
	// 82D85524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D85528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8552C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D85530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D85534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D85538 size=260
    let mut pc: u32 = 0x82D85538;
    'dispatch: loop {
        match pc {
            0x82D85538 => {
    //   block [0x82D85538..0x82D85564)
	// 82D85538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D8553C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D85540: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D85544: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D85548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8554C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D85550: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D85554: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D85558: 409A000C  bne cr6, 0x82d85564
	if !ctx.cr[6].eq {
	pc = 0x82D85564; continue 'dispatch;
	}
	// 82D8555C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D85560: 4800000C  b 0x82d8556c
	pc = 0x82D8556C; continue 'dispatch;
            }
            0x82D85564 => {
    //   block [0x82D85564..0x82D8556C)
	// 82D85564: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D85568: 554AE7BE  rlwinm r10, r10, 0x1c, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	pc = 0x82D8556C; continue 'dispatch;
            }
            0x82D8556C => {
    //   block [0x82D8556C..0x82D85594)
	// 82D8556C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D85570: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D85574: 419A00B0  beq cr6, 0x82d85624
	if ctx.cr[6].eq {
	pc = 0x82D85624; continue 'dispatch;
	}
	// 82D85578: 896B0027  lbz r11, 0x27(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(39 as u32) ) } as u64;
	// 82D8557C: 556A06B6  rlwinm r10, r11, 0, 0x1a, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D85580: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D85584: 409A0010  bne cr6, 0x82d85594
	if !ctx.cr[6].eq {
	pc = 0x82D85594; continue 'dispatch;
	}
	// 82D85588: 556B0032  rlwinm r11, r11, 0, 0, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D8558C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D85590: 419A004C  beq cr6, 0x82d855dc
	if ctx.cr[6].eq {
	pc = 0x82D855DC; continue 'dispatch;
	}
	pc = 0x82D85594; continue 'dispatch;
            }
            0x82D85594 => {
    //   block [0x82D85594..0x82D855DC)
	// 82D85594: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D85598: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D8559C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D855A0: 83CB00A4  lwz r30, 0xa4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82D855A4: 912B00A4  stw r9, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[9].u32 ) };
	// 82D855A8: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D855AC: 892B0027  lbz r9, 0x27(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(39 as u32) ) } as u64;
	// 82D855B0: 712900CF  andi. r9, r9, 0xcf
	ctx.r[9].u64 = ctx.r[9].u64 & 207;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D855B4: 992B0027  stb r9, 0x27(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(39 as u32), ctx.r[9].u8 ) };
	// 82D855B8: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D855BC: 892B0027  lbz r9, 0x27(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(39 as u32) ) } as u64;
	// 82D855C0: 51493032  rlwimi r9, r10, 6, 0, 0x19
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[9].u64 & 0xFFFFFFFF0000003F);
	// 82D855C4: 992B0027  stb r9, 0x27(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(39 as u32), ctx.r[9].u8 ) };
	// 82D855C8: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D855CC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D855D0: 48013719  bl 0x82d98ce8
	ctx.lr = 0x82D855D4;
	sub_82D98CE8(ctx, base);
	// 82D855D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D855D8: 93CB00A4  stw r30, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	pc = 0x82D855DC; continue 'dispatch;
            }
            0x82D855DC => {
    //   block [0x82D855DC..0x82D855FC)
	// 82D855DC: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D855E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D855E4: 81440050  lwz r10, 0x50(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D855E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D855EC: 40990030  ble cr6, 0x82d8561c
	if !ctx.cr[6].gt {
	pc = 0x82D8561C; continue 'dispatch;
	}
	// 82D855F0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D855F4: 8144004C  lwz r10, 0x4c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D855F8: C0090C64  lfs f0, 0xc64(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82D855FC; continue 'dispatch;
            }
            0x82D855FC => {
    //   block [0x82D855FC..0x82D8561C)
	// 82D855FC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D85600: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D85604: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D85608: D00901CC  stfs f0, 0x1cc(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(460 as u32), tmp.u32 ) };
	// 82D8560C: D00901DC  stfs f0, 0x1dc(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(476 as u32), tmp.u32 ) };
	// 82D85610: 81240050  lwz r9, 0x50(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D85614: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D85618: 4198FFE4  blt cr6, 0x82d855fc
	if ctx.cr[6].lt {
	pc = 0x82D855FC; continue 'dispatch;
	}
	pc = 0x82D8561C; continue 'dispatch;
            }
            0x82D8561C => {
    //   block [0x82D8561C..0x82D85624)
	// 82D8561C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D85620: 48010A99  bl 0x82d960b8
	ctx.lr = 0x82D85624;
	sub_82D960B8(ctx, base);
	pc = 0x82D85624; continue 'dispatch;
            }
            0x82D85624 => {
    //   block [0x82D85624..0x82D8563C)
	// 82D85624: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D85628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8562C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D85630: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D85634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D85638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D85640 size=100
    let mut pc: u32 = 0x82D85640;
    'dispatch: loop {
        match pc {
            0x82D85640 => {
    //   block [0x82D85640..0x82D8568C)
	// 82D85640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D85644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D85648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D8564C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D85650: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D85654: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D85658: 419A0034  beq cr6, 0x82d8568c
	if ctx.cr[6].eq {
	pc = 0x82D8568C; continue 'dispatch;
	}
	// 82D8565C: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D85660: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D85664: 419A0028  beq cr6, 0x82d8568c
	if ctx.cr[6].eq {
	pc = 0x82D8568C; continue 'dispatch;
	}
	// 82D85668: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82D8566C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D85670: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D85674: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82D85678: 4BFF23E9  bl 0x82d77a60
	ctx.lr = 0x82D8567C;
	sub_82D77A60(ctx, base);
	// 82D8567C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D85680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D85684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D85688: 4E800020  blr
	return;
            }
            0x82D8568C => {
    //   block [0x82D8568C..0x82D856A4)
	// 82D8568C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D85690: 4BFFFE19  bl 0x82d854a8
	ctx.lr = 0x82D85694;
	sub_82D854A8(ctx, base);
	// 82D85694: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D85698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D8569C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D856A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D856A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D856A8 size=100
    let mut pc: u32 = 0x82D856A8;
    'dispatch: loop {
        match pc {
            0x82D856A8 => {
    //   block [0x82D856A8..0x82D856F4)
	// 82D856A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D856AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D856B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D856B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D856B8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D856BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D856C0: 419A0034  beq cr6, 0x82d856f4
	if ctx.cr[6].eq {
	pc = 0x82D856F4; continue 'dispatch;
	}
	// 82D856C4: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D856C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D856CC: 419A0028  beq cr6, 0x82d856f4
	if ctx.cr[6].eq {
	pc = 0x82D856F4; continue 'dispatch;
	}
	// 82D856D0: 39400021  li r10, 0x21
	ctx.r[10].s64 = 33;
	// 82D856D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D856D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D856DC: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82D856E0: 4BFF2381  bl 0x82d77a60
	ctx.lr = 0x82D856E4;
	sub_82D77A60(ctx, base);
	// 82D856E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D856E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D856EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D856F0: 4E800020  blr
	return;
            }
            0x82D856F4 => {
    //   block [0x82D856F4..0x82D8570C)
	// 82D856F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D856F8: 4BFFFE41  bl 0x82d85538
	ctx.lr = 0x82D856FC;
	sub_82D85538(ctx, base);
	// 82D856FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D85700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D85704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D85708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D85710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D85710 size=16
    let mut pc: u32 = 0x82D85710;
    'dispatch: loop {
        match pc {
            0x82D85710 => {
    //   block [0x82D85710..0x82D85720)
	// 82D85710: A163009C  lhz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82D85714: 814300A4  lwz r10, 0xa4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82D85718: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D8571C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


