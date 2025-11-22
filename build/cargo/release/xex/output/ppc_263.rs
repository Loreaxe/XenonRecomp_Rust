pub fn sub_832A7B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B18 size=4
    let mut pc: u32 = 0x832A7B18;
    'dispatch: loop {
        match pc {
            0x832A7B18 => {
    //   block [0x832A7B18..0x832A7B1C)
	// 832A7B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B20 size=12
    let mut pc: u32 = 0x832A7B20;
    'dispatch: loop {
        match pc {
            0x832A7B20 => {
    //   block [0x832A7B20..0x832A7B2C)
	// 832A7B20: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B24: 386B76A0  addi r3, r11, 0x76a0
	ctx.r[3].s64 = ctx.r[11].s64 + 30368;
	// 832A7B28: 4AF6D2B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B30 size=12
    let mut pc: u32 = 0x832A7B30;
    'dispatch: loop {
        match pc {
            0x832A7B30 => {
    //   block [0x832A7B30..0x832A7B3C)
	// 832A7B30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B34: 386B76A4  addi r3, r11, 0x76a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30372;
	// 832A7B38: 4AF6D2A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B40 size=12
    let mut pc: u32 = 0x832A7B40;
    'dispatch: loop {
        match pc {
            0x832A7B40 => {
    //   block [0x832A7B40..0x832A7B4C)
	// 832A7B40: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B44: 386B76A8  addi r3, r11, 0x76a8
	ctx.r[3].s64 = ctx.r[11].s64 + 30376;
	// 832A7B48: 4AF6D290  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7B50 size=100
    let mut pc: u32 = 0x832A7B50;
    'dispatch: loop {
        match pc {
            0x832A7B50 => {
    //   block [0x832A7B50..0x832A7BB4)
	// 832A7B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7B60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B64: 3BEB76AC  addi r31, r11, 0x76ac
	ctx.r[31].s64 = ctx.r[11].s64 + 30380;
	// 832A7B68: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7B6C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7B70: 419A0018  beq cr6, 0x832a7b88
	if ctx.cr[6].eq {
	pc = 0x832A7B88; continue 'dispatch;
	}
	// 832A7B74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7B78: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7B7C: 4B7EA385  bl 0x82a91f00
	ctx.lr = 0x832A7B80;
	sub_82A91F00(ctx, base);
	// 832A7B80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7B84: 4AF741B5  bl 0x8221bd38
	ctx.lr = 0x832A7B88;
	sub_8221BD38(ctx, base);
	// 832A7B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7B90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7B94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A7B98: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A7B9C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A7BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7BB8 size=12
    let mut pc: u32 = 0x832A7BB8;
    'dispatch: loop {
        match pc {
            0x832A7BB8 => {
    //   block [0x832A7BB8..0x832A7BC4)
	// 832A7BB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7BBC: 386B76BC  addi r3, r11, 0x76bc
	ctx.r[3].s64 = ctx.r[11].s64 + 30396;
	// 832A7BC0: 4AF6D218  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7BC8 size=104
    let mut pc: u32 = 0x832A7BC8;
    'dispatch: loop {
        match pc {
            0x832A7BC8 => {
    //   block [0x832A7BC8..0x832A7C30)
	// 832A7BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7BCC: 4BA0183D  bl 0x82ca9408
	ctx.lr = 0x832A7BD0;
	sub_82CA93D0(ctx, base);
	// 832A7BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7BD4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7BD8: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A7BDC: 396B76C0  addi r11, r11, 0x76c0
	ctx.r[11].s64 = ctx.r[11].s64 + 30400;
	// 832A7BE0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7BE4: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A7BE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7BEC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A7BF0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7BF8: 4AF1EB71  bl 0x821c6768
	ctx.lr = 0x832A7BFC;
	sub_821C6768(ctx, base);
	// 832A7BFC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A7C00: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7C04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C08: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7C0C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7C10: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7C14: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C18: 4082FFE8  bne 0x832a7c00
	if !ctx.cr[0].eq {
	pc = 0x832A7C00; continue 'dispatch;
	}
	// 832A7C1C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7C20: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7C24: 4080FFCC  bge 0x832a7bf0
	if !ctx.cr[0].lt {
	pc = 0x832A7BF0; continue 'dispatch;
	}
	// 832A7C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7C2C: 4BA0182C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7C30 size=104
    let mut pc: u32 = 0x832A7C30;
    'dispatch: loop {
        match pc {
            0x832A7C30 => {
    //   block [0x832A7C30..0x832A7C98)
	// 832A7C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7C34: 4BA017D5  bl 0x82ca9408
	ctx.lr = 0x832A7C38;
	sub_82CA93D0(ctx, base);
	// 832A7C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7C3C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C40: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A7C44: 396B7700  addi r11, r11, 0x7700
	ctx.r[11].s64 = ctx.r[11].s64 + 30464;
	// 832A7C48: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7C4C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A7C50: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C54: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A7C58: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7C60: 4AF1EB09  bl 0x821c6768
	ctx.lr = 0x832A7C64;
	sub_821C6768(ctx, base);
	// 832A7C64: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A7C68: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7C6C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C70: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7C74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7C78: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7C7C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C80: 4082FFE8  bne 0x832a7c68
	if !ctx.cr[0].eq {
	pc = 0x832A7C68; continue 'dispatch;
	}
	// 832A7C84: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7C88: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7C8C: 4080FFCC  bge 0x832a7c58
	if !ctx.cr[0].lt {
	pc = 0x832A7C58; continue 'dispatch;
	}
	// 832A7C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7C94: 4BA017C4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7C98 size=12
    let mut pc: u32 = 0x832A7C98;
    'dispatch: loop {
        match pc {
            0x832A7C98 => {
    //   block [0x832A7C98..0x832A7CA4)
	// 832A7C98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C9C: 386B7794  addi r3, r11, 0x7794
	ctx.r[3].s64 = ctx.r[11].s64 + 30612;
	// 832A7CA0: 4AF6D138  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CA8 size=12
    let mut pc: u32 = 0x832A7CA8;
    'dispatch: loop {
        match pc {
            0x832A7CA8 => {
    //   block [0x832A7CA8..0x832A7CB4)
	// 832A7CA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CAC: 386B7798  addi r3, r11, 0x7798
	ctx.r[3].s64 = ctx.r[11].s64 + 30616;
	// 832A7CB0: 4AF6D128  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CB8 size=12
    let mut pc: u32 = 0x832A7CB8;
    'dispatch: loop {
        match pc {
            0x832A7CB8 => {
    //   block [0x832A7CB8..0x832A7CC4)
	// 832A7CB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CBC: 386B779C  addi r3, r11, 0x779c
	ctx.r[3].s64 = ctx.r[11].s64 + 30620;
	// 832A7CC0: 4AF6D118  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CC8 size=12
    let mut pc: u32 = 0x832A7CC8;
    'dispatch: loop {
        match pc {
            0x832A7CC8 => {
    //   block [0x832A7CC8..0x832A7CD4)
	// 832A7CC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CCC: 386B77A0  addi r3, r11, 0x77a0
	ctx.r[3].s64 = ctx.r[11].s64 + 30624;
	// 832A7CD0: 4AF6D108  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CD8 size=12
    let mut pc: u32 = 0x832A7CD8;
    'dispatch: loop {
        match pc {
            0x832A7CD8 => {
    //   block [0x832A7CD8..0x832A7CE4)
	// 832A7CD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CDC: 386B77A4  addi r3, r11, 0x77a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30628;
	// 832A7CE0: 4AF6D0F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CE8 size=12
    let mut pc: u32 = 0x832A7CE8;
    'dispatch: loop {
        match pc {
            0x832A7CE8 => {
    //   block [0x832A7CE8..0x832A7CF4)
	// 832A7CE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CEC: 386B77A8  addi r3, r11, 0x77a8
	ctx.r[3].s64 = ctx.r[11].s64 + 30632;
	// 832A7CF0: 4AF6D0E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CF8 size=12
    let mut pc: u32 = 0x832A7CF8;
    'dispatch: loop {
        match pc {
            0x832A7CF8 => {
    //   block [0x832A7CF8..0x832A7D04)
	// 832A7CF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CFC: 386B77AC  addi r3, r11, 0x77ac
	ctx.r[3].s64 = ctx.r[11].s64 + 30636;
	// 832A7D00: 4AF6D0D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D08 size=12
    let mut pc: u32 = 0x832A7D08;
    'dispatch: loop {
        match pc {
            0x832A7D08 => {
    //   block [0x832A7D08..0x832A7D14)
	// 832A7D08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D0C: 386B77B0  addi r3, r11, 0x77b0
	ctx.r[3].s64 = ctx.r[11].s64 + 30640;
	// 832A7D10: 4AF6D0C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D18 size=12
    let mut pc: u32 = 0x832A7D18;
    'dispatch: loop {
        match pc {
            0x832A7D18 => {
    //   block [0x832A7D18..0x832A7D24)
	// 832A7D18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D1C: 386B77B4  addi r3, r11, 0x77b4
	ctx.r[3].s64 = ctx.r[11].s64 + 30644;
	// 832A7D20: 4AF6D0B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D28 size=12
    let mut pc: u32 = 0x832A7D28;
    'dispatch: loop {
        match pc {
            0x832A7D28 => {
    //   block [0x832A7D28..0x832A7D34)
	// 832A7D28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D2C: 386B77B8  addi r3, r11, 0x77b8
	ctx.r[3].s64 = ctx.r[11].s64 + 30648;
	// 832A7D30: 4AF6D0A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7D38 size=88
    let mut pc: u32 = 0x832A7D38;
    'dispatch: loop {
        match pc {
            0x832A7D38 => {
    //   block [0x832A7D38..0x832A7D90)
	// 832A7D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7D48: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A7D4C: 3BEB2144  addi r31, r11, 0x2144
	ctx.r[31].s64 = ctx.r[11].s64 + 8516;
	// 832A7D50: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832A7D54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7D58: 419A0014  beq cr6, 0x832a7d6c
	if ctx.cr[6].eq {
	pc = 0x832A7D6C; continue 'dispatch;
	}
	// 832A7D5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7D60: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7D64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7D68: 4E800421  bctrl
	ctx.lr = 0x832A7D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7D6C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832A7D70: 4B0CE171  bl 0x82375ee0
	ctx.lr = 0x832A7D74;
	sub_82375EE0(ctx, base);
	// 832A7D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7D78: 4B0CD309  bl 0x82375080
	ctx.lr = 0x832A7D7C;
	sub_82375080(ctx, base);
	// 832A7D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D90 size=12
    let mut pc: u32 = 0x832A7D90;
    'dispatch: loop {
        match pc {
            0x832A7D90 => {
    //   block [0x832A7D90..0x832A7D9C)
	// 832A7D90: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D94: 386B77C0  addi r3, r11, 0x77c0
	ctx.r[3].s64 = ctx.r[11].s64 + 30656;
	// 832A7D98: 4AF6D040  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DA0 size=12
    let mut pc: u32 = 0x832A7DA0;
    'dispatch: loop {
        match pc {
            0x832A7DA0 => {
    //   block [0x832A7DA0..0x832A7DAC)
	// 832A7DA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DA4: 386B77C4  addi r3, r11, 0x77c4
	ctx.r[3].s64 = ctx.r[11].s64 + 30660;
	// 832A7DA8: 4AF6D030  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DB0 size=12
    let mut pc: u32 = 0x832A7DB0;
    'dispatch: loop {
        match pc {
            0x832A7DB0 => {
    //   block [0x832A7DB0..0x832A7DBC)
	// 832A7DB0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DB4: 386B77C8  addi r3, r11, 0x77c8
	ctx.r[3].s64 = ctx.r[11].s64 + 30664;
	// 832A7DB8: 4AF6D020  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DC0 size=12
    let mut pc: u32 = 0x832A7DC0;
    'dispatch: loop {
        match pc {
            0x832A7DC0 => {
    //   block [0x832A7DC0..0x832A7DCC)
	// 832A7DC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DC4: 386B77CC  addi r3, r11, 0x77cc
	ctx.r[3].s64 = ctx.r[11].s64 + 30668;
	// 832A7DC8: 4AF6D010  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DD0 size=12
    let mut pc: u32 = 0x832A7DD0;
    'dispatch: loop {
        match pc {
            0x832A7DD0 => {
    //   block [0x832A7DD0..0x832A7DDC)
	// 832A7DD0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DD4: 386B77D0  addi r3, r11, 0x77d0
	ctx.r[3].s64 = ctx.r[11].s64 + 30672;
	// 832A7DD8: 4AF6D000  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DE0 size=12
    let mut pc: u32 = 0x832A7DE0;
    'dispatch: loop {
        match pc {
            0x832A7DE0 => {
    //   block [0x832A7DE0..0x832A7DEC)
	// 832A7DE0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DE4: 386B77D4  addi r3, r11, 0x77d4
	ctx.r[3].s64 = ctx.r[11].s64 + 30676;
	// 832A7DE8: 4AF6CFF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DF0 size=12
    let mut pc: u32 = 0x832A7DF0;
    'dispatch: loop {
        match pc {
            0x832A7DF0 => {
    //   block [0x832A7DF0..0x832A7DFC)
	// 832A7DF0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DF4: 386B77D8  addi r3, r11, 0x77d8
	ctx.r[3].s64 = ctx.r[11].s64 + 30680;
	// 832A7DF8: 4AF6CFE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E00 size=12
    let mut pc: u32 = 0x832A7E00;
    'dispatch: loop {
        match pc {
            0x832A7E00 => {
    //   block [0x832A7E00..0x832A7E0C)
	// 832A7E00: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7E04: 386B77DC  addi r3, r11, 0x77dc
	ctx.r[3].s64 = ctx.r[11].s64 + 30684;
	// 832A7E08: 4AF6CFD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E10 size=4
    let mut pc: u32 = 0x832A7E10;
    'dispatch: loop {
        match pc {
            0x832A7E10 => {
    //   block [0x832A7E10..0x832A7E14)
	// 832A7E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E18 size=4
    let mut pc: u32 = 0x832A7E18;
    'dispatch: loop {
        match pc {
            0x832A7E18 => {
    //   block [0x832A7E18..0x832A7E1C)
	// 832A7E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E20 size=4
    let mut pc: u32 = 0x832A7E20;
    'dispatch: loop {
        match pc {
            0x832A7E20 => {
    //   block [0x832A7E20..0x832A7E24)
	// 832A7E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E28 size=4
    let mut pc: u32 = 0x832A7E28;
    'dispatch: loop {
        match pc {
            0x832A7E28 => {
    //   block [0x832A7E28..0x832A7E2C)
	// 832A7E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E30 size=12
    let mut pc: u32 = 0x832A7E30;
    'dispatch: loop {
        match pc {
            0x832A7E30 => {
    //   block [0x832A7E30..0x832A7E3C)
	// 832A7E30: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832A7E34: 806B76BC  lwz r3, 0x76bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30396 as u32) ) } as u64;
	// 832A7E38: 4AF73F00  b 0x8221bd38
	sub_8221BD38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E40 size=4
    let mut pc: u32 = 0x832A7E40;
    'dispatch: loop {
        match pc {
            0x832A7E40 => {
    //   block [0x832A7E40..0x832A7E44)
	// 832A7E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7E48 size=172
    let mut pc: u32 = 0x832A7E48;
    'dispatch: loop {
        match pc {
            0x832A7E48 => {
    //   block [0x832A7E48..0x832A7EF4)
	// 832A7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7E58: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A7E5C: 3BEB3978  addi r31, r11, 0x3978
	ctx.r[31].s64 = ctx.r[11].s64 + 14712;
	// 832A7E60: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 832A7E64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7E68: 419A0014  beq cr6, 0x832a7e7c
	if ctx.cr[6].eq {
	pc = 0x832A7E7C; continue 'dispatch;
	}
	// 832A7E6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7E70: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7E74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7E78: 4E800421  bctrl
	ctx.lr = 0x832A7E7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7E7C: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 832A7E80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7E84: 419A0014  beq cr6, 0x832a7e98
	if ctx.cr[6].eq {
	pc = 0x832A7E98; continue 'dispatch;
	}
	// 832A7E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7E8C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7E90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7E94: 4E800421  bctrl
	ctx.lr = 0x832A7E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7E98: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832A7E9C: 4B0D8285  bl 0x82380120
	ctx.lr = 0x832A7EA0;
	sub_82380120(ctx, base);
	// 832A7EA0: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 832A7EA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7EA8: 419A0018  beq cr6, 0x832a7ec0
	if ctx.cr[6].eq {
	pc = 0x832A7EC0; continue 'dispatch;
	}
	// 832A7EAC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832A7EB0: 80BF002C  lwz r5, 0x2c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 832A7EB4: 4B0D889D  bl 0x82380750
	ctx.lr = 0x832A7EB8;
	sub_82380750(ctx, base);
	// 832A7EB8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 832A7EBC: 4AF73E7D  bl 0x8221bd38
	ctx.lr = 0x832A7EC0;
	sub_8221BD38(ctx, base);
	// 832A7EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7EC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7EC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7ECC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 832A7ED0: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 832A7ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7ED8: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 832A7EDC: 4B9CADE5  bl 0x82c72cc0
	ctx.lr = 0x832A7EE0;
	sub_82C72CC0(ctx, base);
	// 832A7EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7EF8 size=220
    let mut pc: u32 = 0x832A7EF8;
    'dispatch: loop {
        match pc {
            0x832A7EF8 => {
    //   block [0x832A7EF8..0x832A7FD4)
	// 832A7EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7EFC: 4BA0150D  bl 0x82ca9408
	ctx.lr = 0x832A7F00;
	sub_82CA93D0(ctx, base);
	// 832A7F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7F04: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7F08: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 832A7F0C: 396B7810  addi r11, r11, 0x7810
	ctx.r[11].s64 = ctx.r[11].s64 + 30736;
	// 832A7F10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7F14: 3BEB0088  addi r31, r11, 0x88
	ctx.r[31].s64 = ctx.r[11].s64 + 136;
	// 832A7F18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7F1C: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	// 832A7F20: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832A7F24: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7F28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7F2C: 419A0014  beq cr6, 0x832a7f40
	if ctx.cr[6].eq {
	pc = 0x832A7F40; continue 'dispatch;
	}
	// 832A7F30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7F34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7F38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7F3C: 4E800421  bctrl
	ctx.lr = 0x832A7F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7F40: 387FFFFC  addi r3, r31, -4
	ctx.r[3].s64 = ctx.r[31].s64 + -4;
	// 832A7F44: 4AF1E825  bl 0x821c6768
	ctx.lr = 0x832A7F48;
	sub_821C6768(ctx, base);
	// 832A7F48: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832A7F4C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7F50: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F54: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7F58: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7F5C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7F60: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F64: 4082FFE8  bne 0x832a7f4c
	if !ctx.cr[0].eq {
	pc = 0x832A7F4C; continue 'dispatch;
	}
	// 832A7F68: 939FFFFC  stw r28, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[28].u32 ) };
	// 832A7F6C: 387FFFF4  addi r3, r31, -0xc
	ctx.r[3].s64 = ctx.r[31].s64 + -12;
	// 832A7F70: 4AF1E7F9  bl 0x821c6768
	ctx.lr = 0x832A7F74;
	sub_821C6768(ctx, base);
	// 832A7F74: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 832A7F78: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832A7F7C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F80: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832A7F84: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832A7F88: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7F8C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F90: 4082FFE8  bne 0x832a7f78
	if !ctx.cr[0].eq {
	pc = 0x832A7F78; continue 'dispatch;
	}
	// 832A7F94: 939FFFF4  stw r28, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[28].u32 ) };
	// 832A7F98: 387FFFF0  addi r3, r31, -0x10
	ctx.r[3].s64 = ctx.r[31].s64 + -16;
	// 832A7F9C: 4AF1E7CD  bl 0x821c6768
	ctx.lr = 0x832A7FA0;
	sub_821C6768(ctx, base);
	// 832A7FA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832A7FA4: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 832A7FA8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7FAC: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 832A7FB0: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 832A7FB4: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7FB8: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7FBC: 4082FFE8  bne 0x832a7fa4
	if !ctx.cr[0].eq {
	pc = 0x832A7FA4; continue 'dispatch;
	}
	// 832A7FC0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832A7FC4: 939FFFF0  stw r28, -0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16 as u32), ctx.r[28].u32 ) };
	// 832A7FC8: 4080FF58  bge 0x832a7f20
	if !ctx.cr[0].lt {
	pc = 0x832A7F20; continue 'dispatch;
	}
	// 832A7FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7FD0: 4BA01488  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7FD8 size=120
    let mut pc: u32 = 0x832A7FD8;
    'dispatch: loop {
        match pc {
            0x832A7FD8 => {
    //   block [0x832A7FD8..0x832A8050)
	// 832A7FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7FE8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832A7FEC: 3BEB9930  addi r31, r11, -0x66d0
	ctx.r[31].s64 = ctx.r[11].s64 + -26320;
	// 832A7FF0: 806B9930  lwz r3, -0x66d0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26320 as u32) ) } as u64;
	// 832A7FF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7FF8: 419A0034  beq cr6, 0x832a802c
	if ctx.cr[6].eq {
	pc = 0x832A802C; continue 'dispatch;
	}
	// 832A7FFC: 4AF73D3D  bl 0x8221bd38
	ctx.lr = 0x832A8000;
	sub_8221BD38(ctx, base);
	// 832A8000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8008: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 832A800C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A8010: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A8014: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832A8018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A801C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8028: 4E800020  blr
	return;
	// 832A802C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8030: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 832A8034: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8038: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A803C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A804C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8050 size=12
    let mut pc: u32 = 0x832A8050;
    'dispatch: loop {
        match pc {
            0x832A8050 => {
    //   block [0x832A8050..0x832A805C)
	// 832A8050: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8054: 386B7888  addi r3, r11, 0x7888
	ctx.r[3].s64 = ctx.r[11].s64 + 30856;
	// 832A8058: 4AF6CD80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8060 size=12
    let mut pc: u32 = 0x832A8060;
    'dispatch: loop {
        match pc {
            0x832A8060 => {
    //   block [0x832A8060..0x832A806C)
	// 832A8060: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8064: 386B788C  addi r3, r11, 0x788c
	ctx.r[3].s64 = ctx.r[11].s64 + 30860;
	// 832A8068: 4AF6CD70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8070 size=12
    let mut pc: u32 = 0x832A8070;
    'dispatch: loop {
        match pc {
            0x832A8070 => {
    //   block [0x832A8070..0x832A807C)
	// 832A8070: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8074: 386B7890  addi r3, r11, 0x7890
	ctx.r[3].s64 = ctx.r[11].s64 + 30864;
	// 832A8078: 4AF6CD60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8080 size=12
    let mut pc: u32 = 0x832A8080;
    'dispatch: loop {
        match pc {
            0x832A8080 => {
    //   block [0x832A8080..0x832A808C)
	// 832A8080: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8084: 386B7894  addi r3, r11, 0x7894
	ctx.r[3].s64 = ctx.r[11].s64 + 30868;
	// 832A8088: 4AF6CD50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8090 size=12
    let mut pc: u32 = 0x832A8090;
    'dispatch: loop {
        match pc {
            0x832A8090 => {
    //   block [0x832A8090..0x832A809C)
	// 832A8090: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8094: 386B7898  addi r3, r11, 0x7898
	ctx.r[3].s64 = ctx.r[11].s64 + 30872;
	// 832A8098: 4AF6CD40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80A0 size=12
    let mut pc: u32 = 0x832A80A0;
    'dispatch: loop {
        match pc {
            0x832A80A0 => {
    //   block [0x832A80A0..0x832A80AC)
	// 832A80A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80A4: 386B789C  addi r3, r11, 0x789c
	ctx.r[3].s64 = ctx.r[11].s64 + 30876;
	// 832A80A8: 4AF6CD30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80B0 size=12
    let mut pc: u32 = 0x832A80B0;
    'dispatch: loop {
        match pc {
            0x832A80B0 => {
    //   block [0x832A80B0..0x832A80BC)
	// 832A80B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80B4: 386B78A0  addi r3, r11, 0x78a0
	ctx.r[3].s64 = ctx.r[11].s64 + 30880;
	// 832A80B8: 4AF6CD20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80C0 size=12
    let mut pc: u32 = 0x832A80C0;
    'dispatch: loop {
        match pc {
            0x832A80C0 => {
    //   block [0x832A80C0..0x832A80CC)
	// 832A80C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80C4: 386B78A4  addi r3, r11, 0x78a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30884;
	// 832A80C8: 4AF6CD10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80D0 size=12
    let mut pc: u32 = 0x832A80D0;
    'dispatch: loop {
        match pc {
            0x832A80D0 => {
    //   block [0x832A80D0..0x832A80DC)
	// 832A80D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80D4: 386B78A8  addi r3, r11, 0x78a8
	ctx.r[3].s64 = ctx.r[11].s64 + 30888;
	// 832A80D8: 4AF6CD00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80E0 size=12
    let mut pc: u32 = 0x832A80E0;
    'dispatch: loop {
        match pc {
            0x832A80E0 => {
    //   block [0x832A80E0..0x832A80EC)
	// 832A80E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80E4: 386B78AC  addi r3, r11, 0x78ac
	ctx.r[3].s64 = ctx.r[11].s64 + 30892;
	// 832A80E8: 4AF6CCF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80F0 size=12
    let mut pc: u32 = 0x832A80F0;
    'dispatch: loop {
        match pc {
            0x832A80F0 => {
    //   block [0x832A80F0..0x832A80FC)
	// 832A80F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80F4: 386B78B0  addi r3, r11, 0x78b0
	ctx.r[3].s64 = ctx.r[11].s64 + 30896;
	// 832A80F8: 4AF6CCE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8100 size=12
    let mut pc: u32 = 0x832A8100;
    'dispatch: loop {
        match pc {
            0x832A8100 => {
    //   block [0x832A8100..0x832A810C)
	// 832A8100: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8104: 386B78B4  addi r3, r11, 0x78b4
	ctx.r[3].s64 = ctx.r[11].s64 + 30900;
	// 832A8108: 4AF6CCD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8110 size=12
    let mut pc: u32 = 0x832A8110;
    'dispatch: loop {
        match pc {
            0x832A8110 => {
    //   block [0x832A8110..0x832A811C)
	// 832A8110: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8114: 386B78B8  addi r3, r11, 0x78b8
	ctx.r[3].s64 = ctx.r[11].s64 + 30904;
	// 832A8118: 4AF6CCC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8120 size=12
    let mut pc: u32 = 0x832A8120;
    'dispatch: loop {
        match pc {
            0x832A8120 => {
    //   block [0x832A8120..0x832A812C)
	// 832A8120: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8124: 386B78BC  addi r3, r11, 0x78bc
	ctx.r[3].s64 = ctx.r[11].s64 + 30908;
	// 832A8128: 4AF6CCB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8130 size=12
    let mut pc: u32 = 0x832A8130;
    'dispatch: loop {
        match pc {
            0x832A8130 => {
    //   block [0x832A8130..0x832A813C)
	// 832A8130: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8134: 386B78C0  addi r3, r11, 0x78c0
	ctx.r[3].s64 = ctx.r[11].s64 + 30912;
	// 832A8138: 4AF6CCA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8140 size=12
    let mut pc: u32 = 0x832A8140;
    'dispatch: loop {
        match pc {
            0x832A8140 => {
    //   block [0x832A8140..0x832A814C)
	// 832A8140: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8144: 386B78C4  addi r3, r11, 0x78c4
	ctx.r[3].s64 = ctx.r[11].s64 + 30916;
	// 832A8148: 4AF6CC90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8150 size=12
    let mut pc: u32 = 0x832A8150;
    'dispatch: loop {
        match pc {
            0x832A8150 => {
    //   block [0x832A8150..0x832A815C)
	// 832A8150: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8154: 386B78C8  addi r3, r11, 0x78c8
	ctx.r[3].s64 = ctx.r[11].s64 + 30920;
	// 832A8158: 4AF6CC80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8160 size=12
    let mut pc: u32 = 0x832A8160;
    'dispatch: loop {
        match pc {
            0x832A8160 => {
    //   block [0x832A8160..0x832A816C)
	// 832A8160: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8164: 386B78CC  addi r3, r11, 0x78cc
	ctx.r[3].s64 = ctx.r[11].s64 + 30924;
	// 832A8168: 4AF6CC70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8170 size=12
    let mut pc: u32 = 0x832A8170;
    'dispatch: loop {
        match pc {
            0x832A8170 => {
    //   block [0x832A8170..0x832A817C)
	// 832A8170: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8174: 386B78D0  addi r3, r11, 0x78d0
	ctx.r[3].s64 = ctx.r[11].s64 + 30928;
	// 832A8178: 4AF6CC60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8180 size=12
    let mut pc: u32 = 0x832A8180;
    'dispatch: loop {
        match pc {
            0x832A8180 => {
    //   block [0x832A8180..0x832A818C)
	// 832A8180: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8184: 386B78D4  addi r3, r11, 0x78d4
	ctx.r[3].s64 = ctx.r[11].s64 + 30932;
	// 832A8188: 4AF6CC50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8190 size=12
    let mut pc: u32 = 0x832A8190;
    'dispatch: loop {
        match pc {
            0x832A8190 => {
    //   block [0x832A8190..0x832A819C)
	// 832A8190: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8194: 386B78D8  addi r3, r11, 0x78d8
	ctx.r[3].s64 = ctx.r[11].s64 + 30936;
	// 832A8198: 4AF6CC40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A81A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A81A0 size=12
    let mut pc: u32 = 0x832A81A0;
    'dispatch: loop {
        match pc {
            0x832A81A0 => {
    //   block [0x832A81A0..0x832A81AC)
	// 832A81A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A81A4: 386B78DC  addi r3, r11, 0x78dc
	ctx.r[3].s64 = ctx.r[11].s64 + 30940;
	// 832A81A8: 4AF6CC30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A81B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A81B0 size=12
    let mut pc: u32 = 0x832A81B0;
    'dispatch: loop {
        match pc {
            0x832A81B0 => {
    //   block [0x832A81B0..0x832A81BC)
	// 832A81B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A81B4: 386B78E0  addi r3, r11, 0x78e0
	ctx.r[3].s64 = ctx.r[11].s64 + 30944;
	// 832A81B8: 4AF6CC20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A81C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A81C0 size=12
    let mut pc: u32 = 0x832A81C0;
    'dispatch: loop {
        match pc {
            0x832A81C0 => {
    //   block [0x832A81C0..0x832A81CC)
	// 832A81C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A81C4: 386B78E4  addi r3, r11, 0x78e4
	ctx.r[3].s64 = ctx.r[11].s64 + 30948;
	// 832A81C8: 4AF6CC10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A81D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A81D0 size=12
    let mut pc: u32 = 0x832A81D0;
    'dispatch: loop {
        match pc {
            0x832A81D0 => {
    //   block [0x832A81D0..0x832A81DC)
	// 832A81D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A81D4: 386B78E8  addi r3, r11, 0x78e8
	ctx.r[3].s64 = ctx.r[11].s64 + 30952;
	// 832A81D8: 4AF6CC00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A81E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A81E0 size=12
    let mut pc: u32 = 0x832A81E0;
    'dispatch: loop {
        match pc {
            0x832A81E0 => {
    //   block [0x832A81E0..0x832A81EC)
	// 832A81E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A81E4: 386B78EC  addi r3, r11, 0x78ec
	ctx.r[3].s64 = ctx.r[11].s64 + 30956;
	// 832A81E8: 4AF6CBF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A81F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A81F0 size=12
    let mut pc: u32 = 0x832A81F0;
    'dispatch: loop {
        match pc {
            0x832A81F0 => {
    //   block [0x832A81F0..0x832A81FC)
	// 832A81F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A81F4: 386B78F0  addi r3, r11, 0x78f0
	ctx.r[3].s64 = ctx.r[11].s64 + 30960;
	// 832A81F8: 4AF6CBE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8200 size=12
    let mut pc: u32 = 0x832A8200;
    'dispatch: loop {
        match pc {
            0x832A8200 => {
    //   block [0x832A8200..0x832A820C)
	// 832A8200: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8204: 386B78F4  addi r3, r11, 0x78f4
	ctx.r[3].s64 = ctx.r[11].s64 + 30964;
	// 832A8208: 4AF6CBD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8210 size=12
    let mut pc: u32 = 0x832A8210;
    'dispatch: loop {
        match pc {
            0x832A8210 => {
    //   block [0x832A8210..0x832A821C)
	// 832A8210: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8214: 386B78F8  addi r3, r11, 0x78f8
	ctx.r[3].s64 = ctx.r[11].s64 + 30968;
	// 832A8218: 4AF6CBC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8220 size=12
    let mut pc: u32 = 0x832A8220;
    'dispatch: loop {
        match pc {
            0x832A8220 => {
    //   block [0x832A8220..0x832A822C)
	// 832A8220: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8224: 386B78FC  addi r3, r11, 0x78fc
	ctx.r[3].s64 = ctx.r[11].s64 + 30972;
	// 832A8228: 4AF6CBB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8230 size=12
    let mut pc: u32 = 0x832A8230;
    'dispatch: loop {
        match pc {
            0x832A8230 => {
    //   block [0x832A8230..0x832A823C)
	// 832A8230: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8234: 386B7900  addi r3, r11, 0x7900
	ctx.r[3].s64 = ctx.r[11].s64 + 30976;
	// 832A8238: 4AF6CBA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8240 size=192
    let mut pc: u32 = 0x832A8240;
    'dispatch: loop {
        match pc {
            0x832A8240 => {
    //   block [0x832A8240..0x832A8300)
	// 832A8240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8244: 4BA011BD  bl 0x82ca9400
	ctx.lr = 0x832A8248;
	sub_82CA93D0(ctx, base);
	// 832A8248: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A824C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8250: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 832A8254: 3BAB7904  addi r29, r11, 0x7904
	ctx.r[29].s64 = ctx.r[11].s64 + 30980;
	// 832A8258: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 832A825C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8260: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A8264: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A8268: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A826C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8270: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8274: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832A8278: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A827C: 419A0070  beq cr6, 0x832a82ec
	if ctx.cr[6].eq {
	pc = 0x832A82EC; continue 'dispatch;
	}
	// 832A8280: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832A8284: 3B4B0B7C  addi r26, r11, 0xb7c
	ctx.r[26].s64 = ctx.r[11].s64 + 2940;
	// 832A8288: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832A828C: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 832A8290: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A8294: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 832A8298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832A829C: 419A0034  beq cr6, 0x832a82d0
	if ctx.cr[6].eq {
	pc = 0x832A82D0; continue 'dispatch;
	}
	// 832A82A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A82A4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 832A82A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A82AC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A82B0: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A82B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 832A82B8: 409A0014  bne cr6, 0x832a82cc
	if !ctx.cr[6].eq {
	pc = 0x832A82CC; continue 'dispatch;
	}
	// 832A82BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A82C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832A82C4: 409A0008  bne cr6, 0x832a82cc
	if !ctx.cr[6].eq {
	pc = 0x832A82CC; continue 'dispatch;
	}
	// 832A82C8: 4AF73A71  bl 0x8221bd38
	ctx.lr = 0x832A82CC;
	sub_8221BD38(ctx, base);
	// 832A82CC: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 832A82D0: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 832A82D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A82D8: 4AF73A61  bl 0x8221bd38
	ctx.lr = 0x832A82DC;
	sub_8221BD38(ctx, base);
	// 832A82DC: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A82E0: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 832A82E4: 7F1C1840  cmplw cr6, r28, r3
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832A82E8: 409AFFA0  bne cr6, 0x832a8288
	if !ctx.cr[6].eq {
	pc = 0x832A8288; continue 'dispatch;
	}
	// 832A82EC: 4AF73A4D  bl 0x8221bd38
	ctx.lr = 0x832A82F0;
	sub_8221BD38(ctx, base);
	// 832A82F0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 832A82F4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A82F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 832A82FC: 4BA01154  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8300 size=84
    let mut pc: u32 = 0x832A8300;
    'dispatch: loop {
        match pc {
            0x832A8300 => {
    //   block [0x832A8300..0x832A8354)
	// 832A8300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A830C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8310: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8314: 3BEB7910  addi r31, r11, 0x7910
	ctx.r[31].s64 = ctx.r[11].s64 + 30992;
	// 832A8318: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A831C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A8320: 419A0008  beq cr6, 0x832a8328
	if ctx.cr[6].eq {
	pc = 0x832A8328; continue 'dispatch;
	}
	// 832A8324: 4AF73A15  bl 0x8221bd38
	ctx.lr = 0x832A8328;
	sub_8221BD38(ctx, base);
	// 832A8328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A832C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8330: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A8334: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8338: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A833C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A8340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A834C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8358 size=12
    let mut pc: u32 = 0x832A8358;
    'dispatch: loop {
        match pc {
            0x832A8358 => {
    //   block [0x832A8358..0x832A8364)
	// 832A8358: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A835C: 386B7924  addi r3, r11, 0x7924
	ctx.r[3].s64 = ctx.r[11].s64 + 31012;
	// 832A8360: 4AF6CA78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8368 size=12
    let mut pc: u32 = 0x832A8368;
    'dispatch: loop {
        match pc {
            0x832A8368 => {
    //   block [0x832A8368..0x832A8374)
	// 832A8368: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A836C: 386B7928  addi r3, r11, 0x7928
	ctx.r[3].s64 = ctx.r[11].s64 + 31016;
	// 832A8370: 4AF6CA68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8378 size=12
    let mut pc: u32 = 0x832A8378;
    'dispatch: loop {
        match pc {
            0x832A8378 => {
    //   block [0x832A8378..0x832A8384)
	// 832A8378: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A837C: 386B792C  addi r3, r11, 0x792c
	ctx.r[3].s64 = ctx.r[11].s64 + 31020;
	// 832A8380: 4B0E6D80  b 0x8238f100
	sub_8238F100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8388 size=12
    let mut pc: u32 = 0x832A8388;
    'dispatch: loop {
        match pc {
            0x832A8388 => {
    //   block [0x832A8388..0x832A8394)
	// 832A8388: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A838C: 386B7938  addi r3, r11, 0x7938
	ctx.r[3].s64 = ctx.r[11].s64 + 31032;
	// 832A8390: 4AF6CA48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8398 size=12
    let mut pc: u32 = 0x832A8398;
    'dispatch: loop {
        match pc {
            0x832A8398 => {
    //   block [0x832A8398..0x832A83A4)
	// 832A8398: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A839C: 386B793C  addi r3, r11, 0x793c
	ctx.r[3].s64 = ctx.r[11].s64 + 31036;
	// 832A83A0: 4AF6CA38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A83A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A83A8 size=12
    let mut pc: u32 = 0x832A83A8;
    'dispatch: loop {
        match pc {
            0x832A83A8 => {
    //   block [0x832A83A8..0x832A83B4)
	// 832A83A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A83AC: 386B7940  addi r3, r11, 0x7940
	ctx.r[3].s64 = ctx.r[11].s64 + 31040;
	// 832A83B0: 4AF6CA28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A83B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A83B8 size=12
    let mut pc: u32 = 0x832A83B8;
    'dispatch: loop {
        match pc {
            0x832A83B8 => {
    //   block [0x832A83B8..0x832A83C4)
	// 832A83B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A83BC: 386B7944  addi r3, r11, 0x7944
	ctx.r[3].s64 = ctx.r[11].s64 + 31044;
	// 832A83C0: 4AF6CA18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A83C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A83C8 size=12
    let mut pc: u32 = 0x832A83C8;
    'dispatch: loop {
        match pc {
            0x832A83C8 => {
    //   block [0x832A83C8..0x832A83D4)
	// 832A83C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A83CC: 386B7948  addi r3, r11, 0x7948
	ctx.r[3].s64 = ctx.r[11].s64 + 31048;
	// 832A83D0: 4AF6CA08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A83D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A83D8 size=12
    let mut pc: u32 = 0x832A83D8;
    'dispatch: loop {
        match pc {
            0x832A83D8 => {
    //   block [0x832A83D8..0x832A83E4)
	// 832A83D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A83DC: 386B794C  addi r3, r11, 0x794c
	ctx.r[3].s64 = ctx.r[11].s64 + 31052;
	// 832A83E0: 4AF6C9F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A83E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A83E8 size=12
    let mut pc: u32 = 0x832A83E8;
    'dispatch: loop {
        match pc {
            0x832A83E8 => {
    //   block [0x832A83E8..0x832A83F4)
	// 832A83E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A83EC: 386B7950  addi r3, r11, 0x7950
	ctx.r[3].s64 = ctx.r[11].s64 + 31056;
	// 832A83F0: 4AF6C9E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A83F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A83F8 size=12
    let mut pc: u32 = 0x832A83F8;
    'dispatch: loop {
        match pc {
            0x832A83F8 => {
    //   block [0x832A83F8..0x832A8404)
	// 832A83F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A83FC: 386B7954  addi r3, r11, 0x7954
	ctx.r[3].s64 = ctx.r[11].s64 + 31060;
	// 832A8400: 4AF6C9D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8408 size=12
    let mut pc: u32 = 0x832A8408;
    'dispatch: loop {
        match pc {
            0x832A8408 => {
    //   block [0x832A8408..0x832A8414)
	// 832A8408: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A840C: 386B7958  addi r3, r11, 0x7958
	ctx.r[3].s64 = ctx.r[11].s64 + 31064;
	// 832A8410: 4AF6C9C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8418 size=12
    let mut pc: u32 = 0x832A8418;
    'dispatch: loop {
        match pc {
            0x832A8418 => {
    //   block [0x832A8418..0x832A8424)
	// 832A8418: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A841C: 386B795C  addi r3, r11, 0x795c
	ctx.r[3].s64 = ctx.r[11].s64 + 31068;
	// 832A8420: 4AF6C9B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8428 size=12
    let mut pc: u32 = 0x832A8428;
    'dispatch: loop {
        match pc {
            0x832A8428 => {
    //   block [0x832A8428..0x832A8434)
	// 832A8428: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A842C: 386B7968  addi r3, r11, 0x7968
	ctx.r[3].s64 = ctx.r[11].s64 + 31080;
	// 832A8430: 4B392258  b 0x8263a688
	sub_8263A688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8438 size=12
    let mut pc: u32 = 0x832A8438;
    'dispatch: loop {
        match pc {
            0x832A8438 => {
    //   block [0x832A8438..0x832A8444)
	// 832A8438: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A843C: 386B7974  addi r3, r11, 0x7974
	ctx.r[3].s64 = ctx.r[11].s64 + 31092;
	// 832A8440: 4B392248  b 0x8263a688
	sub_8263A688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8448 size=12
    let mut pc: u32 = 0x832A8448;
    'dispatch: loop {
        match pc {
            0x832A8448 => {
    //   block [0x832A8448..0x832A8454)
	// 832A8448: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A844C: 386B7980  addi r3, r11, 0x7980
	ctx.r[3].s64 = ctx.r[11].s64 + 31104;
	// 832A8450: 4AF6C988  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8458 size=12
    let mut pc: u32 = 0x832A8458;
    'dispatch: loop {
        match pc {
            0x832A8458 => {
    //   block [0x832A8458..0x832A8464)
	// 832A8458: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A845C: 386B7984  addi r3, r11, 0x7984
	ctx.r[3].s64 = ctx.r[11].s64 + 31108;
	// 832A8460: 4AF6C978  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8468 size=12
    let mut pc: u32 = 0x832A8468;
    'dispatch: loop {
        match pc {
            0x832A8468 => {
    //   block [0x832A8468..0x832A8474)
	// 832A8468: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A846C: 386B7988  addi r3, r11, 0x7988
	ctx.r[3].s64 = ctx.r[11].s64 + 31112;
	// 832A8470: 4AF6C968  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8478 size=12
    let mut pc: u32 = 0x832A8478;
    'dispatch: loop {
        match pc {
            0x832A8478 => {
    //   block [0x832A8478..0x832A8484)
	// 832A8478: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A847C: 386B798C  addi r3, r11, 0x798c
	ctx.r[3].s64 = ctx.r[11].s64 + 31116;
	// 832A8480: 4AF6C958  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8488 size=12
    let mut pc: u32 = 0x832A8488;
    'dispatch: loop {
        match pc {
            0x832A8488 => {
    //   block [0x832A8488..0x832A8494)
	// 832A8488: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A848C: 386B7990  addi r3, r11, 0x7990
	ctx.r[3].s64 = ctx.r[11].s64 + 31120;
	// 832A8490: 4AF6C948  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8498 size=12
    let mut pc: u32 = 0x832A8498;
    'dispatch: loop {
        match pc {
            0x832A8498 => {
    //   block [0x832A8498..0x832A84A4)
	// 832A8498: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A849C: 386B79A8  addi r3, r11, 0x79a8
	ctx.r[3].s64 = ctx.r[11].s64 + 31144;
	// 832A84A0: 4AF0F678  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A84A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A84A8 size=104
    let mut pc: u32 = 0x832A84A8;
    'dispatch: loop {
        match pc {
            0x832A84A8 => {
    //   block [0x832A84A8..0x832A8510)
	// 832A84A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A84AC: 4BA00F5D  bl 0x82ca9408
	ctx.lr = 0x832A84B0;
	sub_82CA93D0(ctx, base);
	// 832A84B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A84B4: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832A84B8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832A84BC: 396B6E24  addi r11, r11, 0x6e24
	ctx.r[11].s64 = ctx.r[11].s64 + 28196;
	// 832A84C0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A84C4: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 832A84C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A84CC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A84D0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A84D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A84D8: 4AF1E291  bl 0x821c6768
	ctx.lr = 0x832A84DC;
	sub_821C6768(ctx, base);
	// 832A84DC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A84E0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A84E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A84E8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A84EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A84F0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A84F4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A84F8: 4082FFE8  bne 0x832a84e0
	if !ctx.cr[0].eq {
	pc = 0x832A84E0; continue 'dispatch;
	}
	// 832A84FC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A8500: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A8504: 4080FFCC  bge 0x832a84d0
	if !ctx.cr[0].lt {
	pc = 0x832A84D0; continue 'dispatch;
	}
	// 832A8508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A850C: 4BA00F4C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8510 size=12
    let mut pc: u32 = 0x832A8510;
    'dispatch: loop {
        match pc {
            0x832A8510 => {
    //   block [0x832A8510..0x832A851C)
	// 832A8510: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8514: 386B79B0  addi r3, r11, 0x79b0
	ctx.r[3].s64 = ctx.r[11].s64 + 31152;
	// 832A8518: 4AF6C8C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8520 size=12
    let mut pc: u32 = 0x832A8520;
    'dispatch: loop {
        match pc {
            0x832A8520 => {
    //   block [0x832A8520..0x832A852C)
	// 832A8520: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8524: 386B79B4  addi r3, r11, 0x79b4
	ctx.r[3].s64 = ctx.r[11].s64 + 31156;
	// 832A8528: 4AF6C8B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8530 size=12
    let mut pc: u32 = 0x832A8530;
    'dispatch: loop {
        match pc {
            0x832A8530 => {
    //   block [0x832A8530..0x832A853C)
	// 832A8530: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8534: 386B79B8  addi r3, r11, 0x79b8
	ctx.r[3].s64 = ctx.r[11].s64 + 31160;
	// 832A8538: 4AF6C8A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8540 size=12
    let mut pc: u32 = 0x832A8540;
    'dispatch: loop {
        match pc {
            0x832A8540 => {
    //   block [0x832A8540..0x832A854C)
	// 832A8540: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8544: 386B79BC  addi r3, r11, 0x79bc
	ctx.r[3].s64 = ctx.r[11].s64 + 31164;
	// 832A8548: 4AF6C890  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8550 size=12
    let mut pc: u32 = 0x832A8550;
    'dispatch: loop {
        match pc {
            0x832A8550 => {
    //   block [0x832A8550..0x832A855C)
	// 832A8550: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8554: 386B79C0  addi r3, r11, 0x79c0
	ctx.r[3].s64 = ctx.r[11].s64 + 31168;
	// 832A8558: 4AF6C880  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8560 size=68
    let mut pc: u32 = 0x832A8560;
    'dispatch: loop {
        match pc {
            0x832A8560 => {
    //   block [0x832A8560..0x832A85A4)
	// 832A8560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A856C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8570: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8574: 3BEB79C4  addi r31, r11, 0x79c4
	ctx.r[31].s64 = ctx.r[11].s64 + 31172;
	// 832A8578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A857C: 4B07A26D  bl 0x823227e8
	ctx.lr = 0x832A8580;
	sub_823227E8(ctx, base);
	// 832A8580: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8584: 4AF737B5  bl 0x8221bd38
	ctx.lr = 0x832A8588;
	sub_8221BD38(ctx, base);
	// 832A8588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A858C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A859C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A85A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A85A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A85A8 size=84
    let mut pc: u32 = 0x832A85A8;
    'dispatch: loop {
        match pc {
            0x832A85A8 => {
    //   block [0x832A85A8..0x832A85FC)
	// 832A85A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A85AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A85B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A85B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A85B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A85BC: 3BEB79D0  addi r31, r11, 0x79d0
	ctx.r[31].s64 = ctx.r[11].s64 + 31184;
	// 832A85C0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A85C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A85C8: 419A0008  beq cr6, 0x832a85d0
	if ctx.cr[6].eq {
	pc = 0x832A85D0; continue 'dispatch;
	}
	// 832A85CC: 4AF7376D  bl 0x8221bd38
	ctx.lr = 0x832A85D0;
	sub_8221BD38(ctx, base);
	// 832A85D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A85D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A85D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A85DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A85E0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A85E4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A85E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A85EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A85F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A85F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A85F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8600 size=12
    let mut pc: u32 = 0x832A8600;
    'dispatch: loop {
        match pc {
            0x832A8600 => {
    //   block [0x832A8600..0x832A860C)
	// 832A8600: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8604: 386B79E0  addi r3, r11, 0x79e0
	ctx.r[3].s64 = ctx.r[11].s64 + 31200;
	// 832A8608: 4AF6C7D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8610 size=12
    let mut pc: u32 = 0x832A8610;
    'dispatch: loop {
        match pc {
            0x832A8610 => {
    //   block [0x832A8610..0x832A861C)
	// 832A8610: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8614: 386B79E4  addi r3, r11, 0x79e4
	ctx.r[3].s64 = ctx.r[11].s64 + 31204;
	// 832A8618: 4AF6C7C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8620 size=12
    let mut pc: u32 = 0x832A8620;
    'dispatch: loop {
        match pc {
            0x832A8620 => {
    //   block [0x832A8620..0x832A862C)
	// 832A8620: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8624: 386B79E8  addi r3, r11, 0x79e8
	ctx.r[3].s64 = ctx.r[11].s64 + 31208;
	// 832A8628: 4B072530  b 0x8231ab58
	sub_8231AB58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8630 size=12
    let mut pc: u32 = 0x832A8630;
    'dispatch: loop {
        match pc {
            0x832A8630 => {
    //   block [0x832A8630..0x832A863C)
	// 832A8630: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8634: 386B79F8  addi r3, r11, 0x79f8
	ctx.r[3].s64 = ctx.r[11].s64 + 31224;
	// 832A8638: 4AF6C7A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8640 size=12
    let mut pc: u32 = 0x832A8640;
    'dispatch: loop {
        match pc {
            0x832A8640 => {
    //   block [0x832A8640..0x832A864C)
	// 832A8640: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8644: 386B79FC  addi r3, r11, 0x79fc
	ctx.r[3].s64 = ctx.r[11].s64 + 31228;
	// 832A8648: 4AF6C790  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8650 size=12
    let mut pc: u32 = 0x832A8650;
    'dispatch: loop {
        match pc {
            0x832A8650 => {
    //   block [0x832A8650..0x832A865C)
	// 832A8650: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8654: 386B7A10  addi r3, r11, 0x7a10
	ctx.r[3].s64 = ctx.r[11].s64 + 31248;
	// 832A8658: 4AF6C780  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8660 size=12
    let mut pc: u32 = 0x832A8660;
    'dispatch: loop {
        match pc {
            0x832A8660 => {
    //   block [0x832A8660..0x832A866C)
	// 832A8660: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8664: 386B7A14  addi r3, r11, 0x7a14
	ctx.r[3].s64 = ctx.r[11].s64 + 31252;
	// 832A8668: 4AF6C770  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8670 size=12
    let mut pc: u32 = 0x832A8670;
    'dispatch: loop {
        match pc {
            0x832A8670 => {
    //   block [0x832A8670..0x832A867C)
	// 832A8670: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8674: 386B7A18  addi r3, r11, 0x7a18
	ctx.r[3].s64 = ctx.r[11].s64 + 31256;
	// 832A8678: 4AF6C760  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8680 size=84
    let mut pc: u32 = 0x832A8680;
    'dispatch: loop {
        match pc {
            0x832A8680 => {
    //   block [0x832A8680..0x832A86D4)
	// 832A8680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A868C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8690: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A8694: 3BEB3598  addi r31, r11, 0x3598
	ctx.r[31].s64 = ctx.r[11].s64 + 13720;
	// 832A8698: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 832A869C: 4B11A435  bl 0x823c2ad0
	ctx.lr = 0x832A86A0;
	sub_823C2AD0(ctx, base);
	// 832A86A0: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 832A86A4: 4AF73695  bl 0x8221bd38
	ctx.lr = 0x832A86A8;
	sub_8221BD38(ctx, base);
	// 832A86A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A86AC: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 832A86B0: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A86B4: 4AF6C725  bl 0x82214dd8
	ctx.lr = 0x832A86B8;
	sub_82214DD8(ctx, base);
	// 832A86B8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832A86BC: 4AF6C71D  bl 0x82214dd8
	ctx.lr = 0x832A86C0;
	sub_82214DD8(ctx, base);
	// 832A86C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A86C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A86C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A86CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A86D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A86D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A86D8 size=12
    let mut pc: u32 = 0x832A86D8;
    'dispatch: loop {
        match pc {
            0x832A86D8 => {
    //   block [0x832A86D8..0x832A86E4)
	// 832A86D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A86DC: 386B7A1C  addi r3, r11, 0x7a1c
	ctx.r[3].s64 = ctx.r[11].s64 + 31260;
	// 832A86E0: 4AF6C6F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A86E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A86E8 size=12
    let mut pc: u32 = 0x832A86E8;
    'dispatch: loop {
        match pc {
            0x832A86E8 => {
    //   block [0x832A86E8..0x832A86F4)
	// 832A86E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A86EC: 386B7A20  addi r3, r11, 0x7a20
	ctx.r[3].s64 = ctx.r[11].s64 + 31264;
	// 832A86F0: 4AF6C6E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A86F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A86F8 size=12
    let mut pc: u32 = 0x832A86F8;
    'dispatch: loop {
        match pc {
            0x832A86F8 => {
    //   block [0x832A86F8..0x832A8704)
	// 832A86F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A86FC: 386B7A24  addi r3, r11, 0x7a24
	ctx.r[3].s64 = ctx.r[11].s64 + 31268;
	// 832A8700: 4AF6C6D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8708 size=12
    let mut pc: u32 = 0x832A8708;
    'dispatch: loop {
        match pc {
            0x832A8708 => {
    //   block [0x832A8708..0x832A8714)
	// 832A8708: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A870C: 386B7A28  addi r3, r11, 0x7a28
	ctx.r[3].s64 = ctx.r[11].s64 + 31272;
	// 832A8710: 4AF6C6C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8718 size=104
    let mut pc: u32 = 0x832A8718;
    'dispatch: loop {
        match pc {
            0x832A8718 => {
    //   block [0x832A8718..0x832A8780)
	// 832A8718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A871C: 4BA00CED  bl 0x82ca9408
	ctx.lr = 0x832A8720;
	sub_82CA93D0(ctx, base);
	// 832A8720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8724: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8728: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A872C: 396B7A80  addi r11, r11, 0x7a80
	ctx.r[11].s64 = ctx.r[11].s64 + 31360;
	// 832A8730: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A8734: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A8738: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A873C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A8740: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A8744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A8748: 4AF1E021  bl 0x821c6768
	ctx.lr = 0x832A874C;
	sub_821C6768(ctx, base);
	// 832A874C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A8750: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A8754: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A8758: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A875C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A8760: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A8764: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A8768: 4082FFE8  bne 0x832a8750
	if !ctx.cr[0].eq {
	pc = 0x832A8750; continue 'dispatch;
	}
	// 832A876C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A8770: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A8774: 4080FFCC  bge 0x832a8740
	if !ctx.cr[0].lt {
	pc = 0x832A8740; continue 'dispatch;
	}
	// 832A8778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A877C: 4BA00CDC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8780 size=104
    let mut pc: u32 = 0x832A8780;
    'dispatch: loop {
        match pc {
            0x832A8780 => {
    //   block [0x832A8780..0x832A87E8)
	// 832A8780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8784: 4BA00C85  bl 0x82ca9408
	ctx.lr = 0x832A8788;
	sub_82CA93D0(ctx, base);
	// 832A8788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A878C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8790: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A8794: 396B7AC0  addi r11, r11, 0x7ac0
	ctx.r[11].s64 = ctx.r[11].s64 + 31424;
	// 832A8798: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A879C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A87A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A87A4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A87A8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A87AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A87B0: 4AF1DFB9  bl 0x821c6768
	ctx.lr = 0x832A87B4;
	sub_821C6768(ctx, base);
	// 832A87B4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A87B8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A87BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A87C0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A87C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A87C8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A87CC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A87D0: 4082FFE8  bne 0x832a87b8
	if !ctx.cr[0].eq {
	pc = 0x832A87B8; continue 'dispatch;
	}
	// 832A87D4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A87D8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A87DC: 4080FFCC  bge 0x832a87a8
	if !ctx.cr[0].lt {
	pc = 0x832A87A8; continue 'dispatch;
	}
	// 832A87E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A87E4: 4BA00C74  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A87E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A87E8 size=12
    let mut pc: u32 = 0x832A87E8;
    'dispatch: loop {
        match pc {
            0x832A87E8 => {
    //   block [0x832A87E8..0x832A87F4)
	// 832A87E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A87EC: 386B7B00  addi r3, r11, 0x7b00
	ctx.r[3].s64 = ctx.r[11].s64 + 31488;
	// 832A87F0: 4AF6C5E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A87F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A87F8 size=12
    let mut pc: u32 = 0x832A87F8;
    'dispatch: loop {
        match pc {
            0x832A87F8 => {
    //   block [0x832A87F8..0x832A8804)
	// 832A87F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A87FC: 386B7B04  addi r3, r11, 0x7b04
	ctx.r[3].s64 = ctx.r[11].s64 + 31492;
	// 832A8800: 4AF6C5D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8808 size=12
    let mut pc: u32 = 0x832A8808;
    'dispatch: loop {
        match pc {
            0x832A8808 => {
    //   block [0x832A8808..0x832A8814)
	// 832A8808: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A880C: 386B7B08  addi r3, r11, 0x7b08
	ctx.r[3].s64 = ctx.r[11].s64 + 31496;
	// 832A8810: 4AF6C5C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8818 size=12
    let mut pc: u32 = 0x832A8818;
    'dispatch: loop {
        match pc {
            0x832A8818 => {
    //   block [0x832A8818..0x832A8824)
	// 832A8818: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A881C: 386B7B60  addi r3, r11, 0x7b60
	ctx.r[3].s64 = ctx.r[11].s64 + 31584;
	// 832A8820: 4AF6C5B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8828 size=12
    let mut pc: u32 = 0x832A8828;
    'dispatch: loop {
        match pc {
            0x832A8828 => {
    //   block [0x832A8828..0x832A8834)
	// 832A8828: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A882C: 386B7B64  addi r3, r11, 0x7b64
	ctx.r[3].s64 = ctx.r[11].s64 + 31588;
	// 832A8830: 4AF6C5A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8838 size=12
    let mut pc: u32 = 0x832A8838;
    'dispatch: loop {
        match pc {
            0x832A8838 => {
    //   block [0x832A8838..0x832A8844)
	// 832A8838: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A883C: 386B7B68  addi r3, r11, 0x7b68
	ctx.r[3].s64 = ctx.r[11].s64 + 31592;
	// 832A8840: 4AF6C598  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8848 size=12
    let mut pc: u32 = 0x832A8848;
    'dispatch: loop {
        match pc {
            0x832A8848 => {
    //   block [0x832A8848..0x832A8854)
	// 832A8848: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A884C: 386B7B84  addi r3, r11, 0x7b84
	ctx.r[3].s64 = ctx.r[11].s64 + 31620;
	// 832A8850: 4AF6C588  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8858 size=12
    let mut pc: u32 = 0x832A8858;
    'dispatch: loop {
        match pc {
            0x832A8858 => {
    //   block [0x832A8858..0x832A8864)
	// 832A8858: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A885C: 386B7B88  addi r3, r11, 0x7b88
	ctx.r[3].s64 = ctx.r[11].s64 + 31624;
	// 832A8860: 4AF6C578  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8868 size=12
    let mut pc: u32 = 0x832A8868;
    'dispatch: loop {
        match pc {
            0x832A8868 => {
    //   block [0x832A8868..0x832A8874)
	// 832A8868: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A886C: 386B7B8C  addi r3, r11, 0x7b8c
	ctx.r[3].s64 = ctx.r[11].s64 + 31628;
	// 832A8870: 4AF6C568  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8878 size=12
    let mut pc: u32 = 0x832A8878;
    'dispatch: loop {
        match pc {
            0x832A8878 => {
    //   block [0x832A8878..0x832A8884)
	// 832A8878: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A887C: 386B7B90  addi r3, r11, 0x7b90
	ctx.r[3].s64 = ctx.r[11].s64 + 31632;
	// 832A8880: 4AF6C558  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8888 size=12
    let mut pc: u32 = 0x832A8888;
    'dispatch: loop {
        match pc {
            0x832A8888 => {
    //   block [0x832A8888..0x832A8894)
	// 832A8888: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A888C: 386B7B94  addi r3, r11, 0x7b94
	ctx.r[3].s64 = ctx.r[11].s64 + 31636;
	// 832A8890: 4AF6C548  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8898 size=12
    let mut pc: u32 = 0x832A8898;
    'dispatch: loop {
        match pc {
            0x832A8898 => {
    //   block [0x832A8898..0x832A88A4)
	// 832A8898: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A889C: 386B7B98  addi r3, r11, 0x7b98
	ctx.r[3].s64 = ctx.r[11].s64 + 31640;
	// 832A88A0: 4AF6C538  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A88A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A88A8 size=12
    let mut pc: u32 = 0x832A88A8;
    'dispatch: loop {
        match pc {
            0x832A88A8 => {
    //   block [0x832A88A8..0x832A88B4)
	// 832A88A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A88AC: 386B7B9C  addi r3, r11, 0x7b9c
	ctx.r[3].s64 = ctx.r[11].s64 + 31644;
	// 832A88B0: 4AF6C528  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A88B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A88B8 size=12
    let mut pc: u32 = 0x832A88B8;
    'dispatch: loop {
        match pc {
            0x832A88B8 => {
    //   block [0x832A88B8..0x832A88C4)
	// 832A88B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A88BC: 386B7BA0  addi r3, r11, 0x7ba0
	ctx.r[3].s64 = ctx.r[11].s64 + 31648;
	// 832A88C0: 4AF6C518  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A88C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A88C8 size=12
    let mut pc: u32 = 0x832A88C8;
    'dispatch: loop {
        match pc {
            0x832A88C8 => {
    //   block [0x832A88C8..0x832A88D4)
	// 832A88C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A88CC: 386B7BA4  addi r3, r11, 0x7ba4
	ctx.r[3].s64 = ctx.r[11].s64 + 31652;
	// 832A88D0: 4AF6C508  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A88D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A88D8 size=12
    let mut pc: u32 = 0x832A88D8;
    'dispatch: loop {
        match pc {
            0x832A88D8 => {
    //   block [0x832A88D8..0x832A88E4)
	// 832A88D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A88DC: 386B7BA8  addi r3, r11, 0x7ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 31656;
	// 832A88E0: 4AF6C4F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A88E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A88E8 size=12
    let mut pc: u32 = 0x832A88E8;
    'dispatch: loop {
        match pc {
            0x832A88E8 => {
    //   block [0x832A88E8..0x832A88F4)
	// 832A88E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A88EC: 386B7BB0  addi r3, r11, 0x7bb0
	ctx.r[3].s64 = ctx.r[11].s64 + 31664;
	// 832A88F0: 4AF6C4E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A88F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A88F8 size=12
    let mut pc: u32 = 0x832A88F8;
    'dispatch: loop {
        match pc {
            0x832A88F8 => {
    //   block [0x832A88F8..0x832A8904)
	// 832A88F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A88FC: 386B7BB4  addi r3, r11, 0x7bb4
	ctx.r[3].s64 = ctx.r[11].s64 + 31668;
	// 832A8900: 4AF6C4D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8908 size=12
    let mut pc: u32 = 0x832A8908;
    'dispatch: loop {
        match pc {
            0x832A8908 => {
    //   block [0x832A8908..0x832A8914)
	// 832A8908: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A890C: 386B7BB8  addi r3, r11, 0x7bb8
	ctx.r[3].s64 = ctx.r[11].s64 + 31672;
	// 832A8910: 4AF6C4C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8918 size=12
    let mut pc: u32 = 0x832A8918;
    'dispatch: loop {
        match pc {
            0x832A8918 => {
    //   block [0x832A8918..0x832A8924)
	// 832A8918: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A891C: 386B7BBC  addi r3, r11, 0x7bbc
	ctx.r[3].s64 = ctx.r[11].s64 + 31676;
	// 832A8920: 4AF6C4B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8928 size=12
    let mut pc: u32 = 0x832A8928;
    'dispatch: loop {
        match pc {
            0x832A8928 => {
    //   block [0x832A8928..0x832A8934)
	// 832A8928: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A892C: 386B7BC0  addi r3, r11, 0x7bc0
	ctx.r[3].s64 = ctx.r[11].s64 + 31680;
	// 832A8930: 4AF6C4A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8938 size=12
    let mut pc: u32 = 0x832A8938;
    'dispatch: loop {
        match pc {
            0x832A8938 => {
    //   block [0x832A8938..0x832A8944)
	// 832A8938: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A893C: 386B7BC8  addi r3, r11, 0x7bc8
	ctx.r[3].s64 = ctx.r[11].s64 + 31688;
	// 832A8940: 4AF6C498  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8948 size=12
    let mut pc: u32 = 0x832A8948;
    'dispatch: loop {
        match pc {
            0x832A8948 => {
    //   block [0x832A8948..0x832A8954)
	// 832A8948: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A894C: 386B7BCC  addi r3, r11, 0x7bcc
	ctx.r[3].s64 = ctx.r[11].s64 + 31692;
	// 832A8950: 4AF6C488  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8958 size=12
    let mut pc: u32 = 0x832A8958;
    'dispatch: loop {
        match pc {
            0x832A8958 => {
    //   block [0x832A8958..0x832A8964)
	// 832A8958: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A895C: 386B7BD0  addi r3, r11, 0x7bd0
	ctx.r[3].s64 = ctx.r[11].s64 + 31696;
	// 832A8960: 4AF6C478  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8968 size=12
    let mut pc: u32 = 0x832A8968;
    'dispatch: loop {
        match pc {
            0x832A8968 => {
    //   block [0x832A8968..0x832A8974)
	// 832A8968: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A896C: 386B7BD4  addi r3, r11, 0x7bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 31700;
	// 832A8970: 4AF6C468  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8978 size=12
    let mut pc: u32 = 0x832A8978;
    'dispatch: loop {
        match pc {
            0x832A8978 => {
    //   block [0x832A8978..0x832A8984)
	// 832A8978: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A897C: 386B7BE0  addi r3, r11, 0x7be0
	ctx.r[3].s64 = ctx.r[11].s64 + 31712;
	// 832A8980: 4AF6C458  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8988 size=12
    let mut pc: u32 = 0x832A8988;
    'dispatch: loop {
        match pc {
            0x832A8988 => {
    //   block [0x832A8988..0x832A8994)
	// 832A8988: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A898C: 386B7BE4  addi r3, r11, 0x7be4
	ctx.r[3].s64 = ctx.r[11].s64 + 31716;
	// 832A8990: 4AF6C448  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8998 size=12
    let mut pc: u32 = 0x832A8998;
    'dispatch: loop {
        match pc {
            0x832A8998 => {
    //   block [0x832A8998..0x832A89A4)
	// 832A8998: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A899C: 386B7BE8  addi r3, r11, 0x7be8
	ctx.r[3].s64 = ctx.r[11].s64 + 31720;
	// 832A89A0: 4AF6C438  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A89A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A89A8 size=12
    let mut pc: u32 = 0x832A89A8;
    'dispatch: loop {
        match pc {
            0x832A89A8 => {
    //   block [0x832A89A8..0x832A89B4)
	// 832A89A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A89AC: 386B7BEC  addi r3, r11, 0x7bec
	ctx.r[3].s64 = ctx.r[11].s64 + 31724;
	// 832A89B0: 4AF6C428  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A89B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A89B8 size=12
    let mut pc: u32 = 0x832A89B8;
    'dispatch: loop {
        match pc {
            0x832A89B8 => {
    //   block [0x832A89B8..0x832A89C4)
	// 832A89B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A89BC: 386B7BF0  addi r3, r11, 0x7bf0
	ctx.r[3].s64 = ctx.r[11].s64 + 31728;
	// 832A89C0: 4AF6C418  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A89C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A89C8 size=12
    let mut pc: u32 = 0x832A89C8;
    'dispatch: loop {
        match pc {
            0x832A89C8 => {
    //   block [0x832A89C8..0x832A89D4)
	// 832A89C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A89CC: 386B7BF8  addi r3, r11, 0x7bf8
	ctx.r[3].s64 = ctx.r[11].s64 + 31736;
	// 832A89D0: 4AF6C408  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A89D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A89D8 size=12
    let mut pc: u32 = 0x832A89D8;
    'dispatch: loop {
        match pc {
            0x832A89D8 => {
    //   block [0x832A89D8..0x832A89E4)
	// 832A89D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A89DC: 386B7BFC  addi r3, r11, 0x7bfc
	ctx.r[3].s64 = ctx.r[11].s64 + 31740;
	// 832A89E0: 4AF6C3F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A89E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A89E8 size=12
    let mut pc: u32 = 0x832A89E8;
    'dispatch: loop {
        match pc {
            0x832A89E8 => {
    //   block [0x832A89E8..0x832A89F4)
	// 832A89E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A89EC: 386B7C00  addi r3, r11, 0x7c00
	ctx.r[3].s64 = ctx.r[11].s64 + 31744;
	// 832A89F0: 4AF6C3E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A89F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A89F8 size=12
    let mut pc: u32 = 0x832A89F8;
    'dispatch: loop {
        match pc {
            0x832A89F8 => {
    //   block [0x832A89F8..0x832A8A04)
	// 832A89F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A89FC: 386B7C04  addi r3, r11, 0x7c04
	ctx.r[3].s64 = ctx.r[11].s64 + 31748;
	// 832A8A00: 4AF6C3D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A08 size=12
    let mut pc: u32 = 0x832A8A08;
    'dispatch: loop {
        match pc {
            0x832A8A08 => {
    //   block [0x832A8A08..0x832A8A14)
	// 832A8A08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A0C: 386B7C08  addi r3, r11, 0x7c08
	ctx.r[3].s64 = ctx.r[11].s64 + 31752;
	// 832A8A10: 4AF6C3C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A18 size=12
    let mut pc: u32 = 0x832A8A18;
    'dispatch: loop {
        match pc {
            0x832A8A18 => {
    //   block [0x832A8A18..0x832A8A24)
	// 832A8A18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A1C: 386B7C0C  addi r3, r11, 0x7c0c
	ctx.r[3].s64 = ctx.r[11].s64 + 31756;
	// 832A8A20: 4AF6C3B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A28 size=12
    let mut pc: u32 = 0x832A8A28;
    'dispatch: loop {
        match pc {
            0x832A8A28 => {
    //   block [0x832A8A28..0x832A8A34)
	// 832A8A28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A2C: 386B7C10  addi r3, r11, 0x7c10
	ctx.r[3].s64 = ctx.r[11].s64 + 31760;
	// 832A8A30: 4AF6C3A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A38 size=12
    let mut pc: u32 = 0x832A8A38;
    'dispatch: loop {
        match pc {
            0x832A8A38 => {
    //   block [0x832A8A38..0x832A8A44)
	// 832A8A38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A3C: 386B7C14  addi r3, r11, 0x7c14
	ctx.r[3].s64 = ctx.r[11].s64 + 31764;
	// 832A8A40: 4AF6C398  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A48 size=12
    let mut pc: u32 = 0x832A8A48;
    'dispatch: loop {
        match pc {
            0x832A8A48 => {
    //   block [0x832A8A48..0x832A8A54)
	// 832A8A48: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A4C: 386B7C18  addi r3, r11, 0x7c18
	ctx.r[3].s64 = ctx.r[11].s64 + 31768;
	// 832A8A50: 4AF6C388  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A58 size=12
    let mut pc: u32 = 0x832A8A58;
    'dispatch: loop {
        match pc {
            0x832A8A58 => {
    //   block [0x832A8A58..0x832A8A64)
	// 832A8A58: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A5C: 386B7C1C  addi r3, r11, 0x7c1c
	ctx.r[3].s64 = ctx.r[11].s64 + 31772;
	// 832A8A60: 4AF6C378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A68 size=12
    let mut pc: u32 = 0x832A8A68;
    'dispatch: loop {
        match pc {
            0x832A8A68 => {
    //   block [0x832A8A68..0x832A8A74)
	// 832A8A68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A6C: 386B7C40  addi r3, r11, 0x7c40
	ctx.r[3].s64 = ctx.r[11].s64 + 31808;
	// 832A8A70: 4AF6C368  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A78 size=12
    let mut pc: u32 = 0x832A8A78;
    'dispatch: loop {
        match pc {
            0x832A8A78 => {
    //   block [0x832A8A78..0x832A8A84)
	// 832A8A78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A7C: 386B7C44  addi r3, r11, 0x7c44
	ctx.r[3].s64 = ctx.r[11].s64 + 31812;
	// 832A8A80: 4AF6C358  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A88 size=12
    let mut pc: u32 = 0x832A8A88;
    'dispatch: loop {
        match pc {
            0x832A8A88 => {
    //   block [0x832A8A88..0x832A8A94)
	// 832A8A88: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A8C: 386B7C48  addi r3, r11, 0x7c48
	ctx.r[3].s64 = ctx.r[11].s64 + 31816;
	// 832A8A90: 4AF6C348  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8A98 size=12
    let mut pc: u32 = 0x832A8A98;
    'dispatch: loop {
        match pc {
            0x832A8A98 => {
    //   block [0x832A8A98..0x832A8AA4)
	// 832A8A98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8A9C: 386B7C4C  addi r3, r11, 0x7c4c
	ctx.r[3].s64 = ctx.r[11].s64 + 31820;
	// 832A8AA0: 4AF6C338  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8AA8 size=12
    let mut pc: u32 = 0x832A8AA8;
    'dispatch: loop {
        match pc {
            0x832A8AA8 => {
    //   block [0x832A8AA8..0x832A8AB4)
	// 832A8AA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8AAC: 386B7C50  addi r3, r11, 0x7c50
	ctx.r[3].s64 = ctx.r[11].s64 + 31824;
	// 832A8AB0: 4AF6C328  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8AB8 size=12
    let mut pc: u32 = 0x832A8AB8;
    'dispatch: loop {
        match pc {
            0x832A8AB8 => {
    //   block [0x832A8AB8..0x832A8AC4)
	// 832A8AB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8ABC: 386B7C54  addi r3, r11, 0x7c54
	ctx.r[3].s64 = ctx.r[11].s64 + 31828;
	// 832A8AC0: 4AF6C318  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8AC8 size=12
    let mut pc: u32 = 0x832A8AC8;
    'dispatch: loop {
        match pc {
            0x832A8AC8 => {
    //   block [0x832A8AC8..0x832A8AD4)
	// 832A8AC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8ACC: 386B7C78  addi r3, r11, 0x7c78
	ctx.r[3].s64 = ctx.r[11].s64 + 31864;
	// 832A8AD0: 4AF6C308  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8AD8 size=12
    let mut pc: u32 = 0x832A8AD8;
    'dispatch: loop {
        match pc {
            0x832A8AD8 => {
    //   block [0x832A8AD8..0x832A8AE4)
	// 832A8AD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8ADC: 386B7C7C  addi r3, r11, 0x7c7c
	ctx.r[3].s64 = ctx.r[11].s64 + 31868;
	// 832A8AE0: 4AF6C2F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8AE8 size=12
    let mut pc: u32 = 0x832A8AE8;
    'dispatch: loop {
        match pc {
            0x832A8AE8 => {
    //   block [0x832A8AE8..0x832A8AF4)
	// 832A8AE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8AEC: 386B7C80  addi r3, r11, 0x7c80
	ctx.r[3].s64 = ctx.r[11].s64 + 31872;
	// 832A8AF0: 4AF6C2E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8AF8 size=12
    let mut pc: u32 = 0x832A8AF8;
    'dispatch: loop {
        match pc {
            0x832A8AF8 => {
    //   block [0x832A8AF8..0x832A8B04)
	// 832A8AF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8AFC: 386B7C84  addi r3, r11, 0x7c84
	ctx.r[3].s64 = ctx.r[11].s64 + 31876;
	// 832A8B00: 4AF6C2D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B08 size=12
    let mut pc: u32 = 0x832A8B08;
    'dispatch: loop {
        match pc {
            0x832A8B08 => {
    //   block [0x832A8B08..0x832A8B14)
	// 832A8B08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B0C: 386B7C88  addi r3, r11, 0x7c88
	ctx.r[3].s64 = ctx.r[11].s64 + 31880;
	// 832A8B10: 4AF6C2C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B18 size=12
    let mut pc: u32 = 0x832A8B18;
    'dispatch: loop {
        match pc {
            0x832A8B18 => {
    //   block [0x832A8B18..0x832A8B24)
	// 832A8B18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B1C: 386B7C8C  addi r3, r11, 0x7c8c
	ctx.r[3].s64 = ctx.r[11].s64 + 31884;
	// 832A8B20: 4AF6C2B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B28 size=12
    let mut pc: u32 = 0x832A8B28;
    'dispatch: loop {
        match pc {
            0x832A8B28 => {
    //   block [0x832A8B28..0x832A8B34)
	// 832A8B28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B2C: 386B7C90  addi r3, r11, 0x7c90
	ctx.r[3].s64 = ctx.r[11].s64 + 31888;
	// 832A8B30: 4AF6C2A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B38 size=12
    let mut pc: u32 = 0x832A8B38;
    'dispatch: loop {
        match pc {
            0x832A8B38 => {
    //   block [0x832A8B38..0x832A8B44)
	// 832A8B38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B3C: 386B7C94  addi r3, r11, 0x7c94
	ctx.r[3].s64 = ctx.r[11].s64 + 31892;
	// 832A8B40: 4AF6C298  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B48 size=12
    let mut pc: u32 = 0x832A8B48;
    'dispatch: loop {
        match pc {
            0x832A8B48 => {
    //   block [0x832A8B48..0x832A8B54)
	// 832A8B48: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B4C: 386B7C98  addi r3, r11, 0x7c98
	ctx.r[3].s64 = ctx.r[11].s64 + 31896;
	// 832A8B50: 4AF6C288  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B58 size=12
    let mut pc: u32 = 0x832A8B58;
    'dispatch: loop {
        match pc {
            0x832A8B58 => {
    //   block [0x832A8B58..0x832A8B64)
	// 832A8B58: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B5C: 386B7C9C  addi r3, r11, 0x7c9c
	ctx.r[3].s64 = ctx.r[11].s64 + 31900;
	// 832A8B60: 4AF6C278  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B68 size=12
    let mut pc: u32 = 0x832A8B68;
    'dispatch: loop {
        match pc {
            0x832A8B68 => {
    //   block [0x832A8B68..0x832A8B74)
	// 832A8B68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B6C: 386B7CA0  addi r3, r11, 0x7ca0
	ctx.r[3].s64 = ctx.r[11].s64 + 31904;
	// 832A8B70: 4AF6C268  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B78 size=12
    let mut pc: u32 = 0x832A8B78;
    'dispatch: loop {
        match pc {
            0x832A8B78 => {
    //   block [0x832A8B78..0x832A8B84)
	// 832A8B78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B7C: 386B7CA4  addi r3, r11, 0x7ca4
	ctx.r[3].s64 = ctx.r[11].s64 + 31908;
	// 832A8B80: 4AF6C258  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B88 size=12
    let mut pc: u32 = 0x832A8B88;
    'dispatch: loop {
        match pc {
            0x832A8B88 => {
    //   block [0x832A8B88..0x832A8B94)
	// 832A8B88: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B8C: 386B7CCC  addi r3, r11, 0x7ccc
	ctx.r[3].s64 = ctx.r[11].s64 + 31948;
	// 832A8B90: 4AF6C248  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8B98 size=12
    let mut pc: u32 = 0x832A8B98;
    'dispatch: loop {
        match pc {
            0x832A8B98 => {
    //   block [0x832A8B98..0x832A8BA4)
	// 832A8B98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8B9C: 386B7CD0  addi r3, r11, 0x7cd0
	ctx.r[3].s64 = ctx.r[11].s64 + 31952;
	// 832A8BA0: 4AF6C238  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8BA8 size=12
    let mut pc: u32 = 0x832A8BA8;
    'dispatch: loop {
        match pc {
            0x832A8BA8 => {
    //   block [0x832A8BA8..0x832A8BB4)
	// 832A8BA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8BAC: 386B7CD4  addi r3, r11, 0x7cd4
	ctx.r[3].s64 = ctx.r[11].s64 + 31956;
	// 832A8BB0: 4AF6C228  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8BB8 size=12
    let mut pc: u32 = 0x832A8BB8;
    'dispatch: loop {
        match pc {
            0x832A8BB8 => {
    //   block [0x832A8BB8..0x832A8BC4)
	// 832A8BB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8BBC: 386B7CD8  addi r3, r11, 0x7cd8
	ctx.r[3].s64 = ctx.r[11].s64 + 31960;
	// 832A8BC0: 4AF6C218  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8BC8 size=12
    let mut pc: u32 = 0x832A8BC8;
    'dispatch: loop {
        match pc {
            0x832A8BC8 => {
    //   block [0x832A8BC8..0x832A8BD4)
	// 832A8BC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8BCC: 386B7CDC  addi r3, r11, 0x7cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 31964;
	// 832A8BD0: 4AF6C208  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8BD8 size=12
    let mut pc: u32 = 0x832A8BD8;
    'dispatch: loop {
        match pc {
            0x832A8BD8 => {
    //   block [0x832A8BD8..0x832A8BE4)
	// 832A8BD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8BDC: 386B7CE0  addi r3, r11, 0x7ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 31968;
	// 832A8BE0: 4AF6C1F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8BE8 size=12
    let mut pc: u32 = 0x832A8BE8;
    'dispatch: loop {
        match pc {
            0x832A8BE8 => {
    //   block [0x832A8BE8..0x832A8BF4)
	// 832A8BE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8BEC: 386B7CE4  addi r3, r11, 0x7ce4
	ctx.r[3].s64 = ctx.r[11].s64 + 31972;
	// 832A8BF0: 4AF6C1E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8BF8 size=12
    let mut pc: u32 = 0x832A8BF8;
    'dispatch: loop {
        match pc {
            0x832A8BF8 => {
    //   block [0x832A8BF8..0x832A8C04)
	// 832A8BF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8BFC: 386B7CE8  addi r3, r11, 0x7ce8
	ctx.r[3].s64 = ctx.r[11].s64 + 31976;
	// 832A8C00: 4AF6C1D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C08 size=12
    let mut pc: u32 = 0x832A8C08;
    'dispatch: loop {
        match pc {
            0x832A8C08 => {
    //   block [0x832A8C08..0x832A8C14)
	// 832A8C08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C0C: 386B7CEC  addi r3, r11, 0x7cec
	ctx.r[3].s64 = ctx.r[11].s64 + 31980;
	// 832A8C10: 4AF6C1C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C18 size=12
    let mut pc: u32 = 0x832A8C18;
    'dispatch: loop {
        match pc {
            0x832A8C18 => {
    //   block [0x832A8C18..0x832A8C24)
	// 832A8C18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C1C: 386B7CF0  addi r3, r11, 0x7cf0
	ctx.r[3].s64 = ctx.r[11].s64 + 31984;
	// 832A8C20: 4AF6C1B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C28 size=12
    let mut pc: u32 = 0x832A8C28;
    'dispatch: loop {
        match pc {
            0x832A8C28 => {
    //   block [0x832A8C28..0x832A8C34)
	// 832A8C28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C2C: 386B7CF4  addi r3, r11, 0x7cf4
	ctx.r[3].s64 = ctx.r[11].s64 + 31988;
	// 832A8C30: 4AF6C1A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C38 size=12
    let mut pc: u32 = 0x832A8C38;
    'dispatch: loop {
        match pc {
            0x832A8C38 => {
    //   block [0x832A8C38..0x832A8C44)
	// 832A8C38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C3C: 386B7CF8  addi r3, r11, 0x7cf8
	ctx.r[3].s64 = ctx.r[11].s64 + 31992;
	// 832A8C40: 4AF6C198  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C48 size=12
    let mut pc: u32 = 0x832A8C48;
    'dispatch: loop {
        match pc {
            0x832A8C48 => {
    //   block [0x832A8C48..0x832A8C54)
	// 832A8C48: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C4C: 386B7CFC  addi r3, r11, 0x7cfc
	ctx.r[3].s64 = ctx.r[11].s64 + 31996;
	// 832A8C50: 4AF6C188  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C58 size=12
    let mut pc: u32 = 0x832A8C58;
    'dispatch: loop {
        match pc {
            0x832A8C58 => {
    //   block [0x832A8C58..0x832A8C64)
	// 832A8C58: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C5C: 386B7D00  addi r3, r11, 0x7d00
	ctx.r[3].s64 = ctx.r[11].s64 + 32000;
	// 832A8C60: 4AF6C178  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C68 size=12
    let mut pc: u32 = 0x832A8C68;
    'dispatch: loop {
        match pc {
            0x832A8C68 => {
    //   block [0x832A8C68..0x832A8C74)
	// 832A8C68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C6C: 386B7D04  addi r3, r11, 0x7d04
	ctx.r[3].s64 = ctx.r[11].s64 + 32004;
	// 832A8C70: 4AF6C168  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C78 size=12
    let mut pc: u32 = 0x832A8C78;
    'dispatch: loop {
        match pc {
            0x832A8C78 => {
    //   block [0x832A8C78..0x832A8C84)
	// 832A8C78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C7C: 386B7D08  addi r3, r11, 0x7d08
	ctx.r[3].s64 = ctx.r[11].s64 + 32008;
	// 832A8C80: 4AF6C158  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C88 size=12
    let mut pc: u32 = 0x832A8C88;
    'dispatch: loop {
        match pc {
            0x832A8C88 => {
    //   block [0x832A8C88..0x832A8C94)
	// 832A8C88: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C8C: 386B7D0C  addi r3, r11, 0x7d0c
	ctx.r[3].s64 = ctx.r[11].s64 + 32012;
	// 832A8C90: 4AF6C148  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8C98 size=12
    let mut pc: u32 = 0x832A8C98;
    'dispatch: loop {
        match pc {
            0x832A8C98 => {
    //   block [0x832A8C98..0x832A8CA4)
	// 832A8C98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8C9C: 386B7D10  addi r3, r11, 0x7d10
	ctx.r[3].s64 = ctx.r[11].s64 + 32016;
	// 832A8CA0: 4AF6C138  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8CA8 size=12
    let mut pc: u32 = 0x832A8CA8;
    'dispatch: loop {
        match pc {
            0x832A8CA8 => {
    //   block [0x832A8CA8..0x832A8CB4)
	// 832A8CA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8CAC: 386B7D14  addi r3, r11, 0x7d14
	ctx.r[3].s64 = ctx.r[11].s64 + 32020;
	// 832A8CB0: 4AF6C128  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8CB8 size=12
    let mut pc: u32 = 0x832A8CB8;
    'dispatch: loop {
        match pc {
            0x832A8CB8 => {
    //   block [0x832A8CB8..0x832A8CC4)
	// 832A8CB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8CBC: 386B7D18  addi r3, r11, 0x7d18
	ctx.r[3].s64 = ctx.r[11].s64 + 32024;
	// 832A8CC0: 4AF6C118  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8CC8 size=100
    let mut pc: u32 = 0x832A8CC8;
    'dispatch: loop {
        match pc {
            0x832A8CC8 => {
    //   block [0x832A8CC8..0x832A8D2C)
	// 832A8CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A8CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8CD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8CDC: 3BEB7D2C  addi r31, r11, 0x7d2c
	ctx.r[31].s64 = ctx.r[11].s64 + 32044;
	// 832A8CE0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8CE4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A8CE8: 419A0018  beq cr6, 0x832a8d00
	if ctx.cr[6].eq {
	pc = 0x832A8D00; continue 'dispatch;
	}
	// 832A8CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A8CF0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A8CF4: 4B1371D5  bl 0x823dfec8
	ctx.lr = 0x832A8CF8;
	sub_823DFEC8(ctx, base);
	// 832A8CF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8CFC: 4AF7303D  bl 0x8221bd38
	ctx.lr = 0x832A8D00;
	sub_8221BD38(ctx, base);
	// 832A8D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8D04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8D08: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A8D0C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8D10: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A8D14: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A8D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8D30 size=100
    let mut pc: u32 = 0x832A8D30;
    'dispatch: loop {
        match pc {
            0x832A8D30 => {
    //   block [0x832A8D30..0x832A8D94)
	// 832A8D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A8D3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8D40: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8D44: 3BEB7D3C  addi r31, r11, 0x7d3c
	ctx.r[31].s64 = ctx.r[11].s64 + 32060;
	// 832A8D48: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8D4C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A8D50: 419A0018  beq cr6, 0x832a8d68
	if ctx.cr[6].eq {
	pc = 0x832A8D68; continue 'dispatch;
	}
	// 832A8D54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A8D58: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A8D5C: 4B13716D  bl 0x823dfec8
	ctx.lr = 0x832A8D60;
	sub_823DFEC8(ctx, base);
	// 832A8D60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8D64: 4AF72FD5  bl 0x8221bd38
	ctx.lr = 0x832A8D68;
	sub_8221BD38(ctx, base);
	// 832A8D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8D70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A8D74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8D78: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A8D7C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A8D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8D8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8D98 size=12
    let mut pc: u32 = 0x832A8D98;
    'dispatch: loop {
        match pc {
            0x832A8D98 => {
    //   block [0x832A8D98..0x832A8DA4)
	// 832A8D98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8D9C: 386B7D4C  addi r3, r11, 0x7d4c
	ctx.r[3].s64 = ctx.r[11].s64 + 32076;
	// 832A8DA0: 4AF6C038  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8DA8 size=12
    let mut pc: u32 = 0x832A8DA8;
    'dispatch: loop {
        match pc {
            0x832A8DA8 => {
    //   block [0x832A8DA8..0x832A8DB4)
	// 832A8DA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8DAC: 386B7D50  addi r3, r11, 0x7d50
	ctx.r[3].s64 = ctx.r[11].s64 + 32080;
	// 832A8DB0: 4AF6C028  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8DB8 size=12
    let mut pc: u32 = 0x832A8DB8;
    'dispatch: loop {
        match pc {
            0x832A8DB8 => {
    //   block [0x832A8DB8..0x832A8DC4)
	// 832A8DB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8DBC: 386B7D54  addi r3, r11, 0x7d54
	ctx.r[3].s64 = ctx.r[11].s64 + 32084;
	// 832A8DC0: 4AF6C018  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8DC8 size=12
    let mut pc: u32 = 0x832A8DC8;
    'dispatch: loop {
        match pc {
            0x832A8DC8 => {
    //   block [0x832A8DC8..0x832A8DD4)
	// 832A8DC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8DCC: 386B7D58  addi r3, r11, 0x7d58
	ctx.r[3].s64 = ctx.r[11].s64 + 32088;
	// 832A8DD0: 4AF6C008  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8DD8 size=12
    let mut pc: u32 = 0x832A8DD8;
    'dispatch: loop {
        match pc {
            0x832A8DD8 => {
    //   block [0x832A8DD8..0x832A8DE4)
	// 832A8DD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8DDC: 386B7D5C  addi r3, r11, 0x7d5c
	ctx.r[3].s64 = ctx.r[11].s64 + 32092;
	// 832A8DE0: 4AF6BFF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8DE8 size=12
    let mut pc: u32 = 0x832A8DE8;
    'dispatch: loop {
        match pc {
            0x832A8DE8 => {
    //   block [0x832A8DE8..0x832A8DF4)
	// 832A8DE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8DEC: 386B7D60  addi r3, r11, 0x7d60
	ctx.r[3].s64 = ctx.r[11].s64 + 32096;
	// 832A8DF0: 4AF6BFE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8DF8 size=12
    let mut pc: u32 = 0x832A8DF8;
    'dispatch: loop {
        match pc {
            0x832A8DF8 => {
    //   block [0x832A8DF8..0x832A8E04)
	// 832A8DF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8DFC: 386B7D64  addi r3, r11, 0x7d64
	ctx.r[3].s64 = ctx.r[11].s64 + 32100;
	// 832A8E00: 4AF6BFD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E08 size=12
    let mut pc: u32 = 0x832A8E08;
    'dispatch: loop {
        match pc {
            0x832A8E08 => {
    //   block [0x832A8E08..0x832A8E14)
	// 832A8E08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E0C: 386B7D68  addi r3, r11, 0x7d68
	ctx.r[3].s64 = ctx.r[11].s64 + 32104;
	// 832A8E10: 4B14B2E0  b 0x823f40f0
	sub_823F40F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E18 size=12
    let mut pc: u32 = 0x832A8E18;
    'dispatch: loop {
        match pc {
            0x832A8E18 => {
    //   block [0x832A8E18..0x832A8E24)
	// 832A8E18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E1C: 386B7D74  addi r3, r11, 0x7d74
	ctx.r[3].s64 = ctx.r[11].s64 + 32116;
	// 832A8E20: 4AF6BFB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E28 size=12
    let mut pc: u32 = 0x832A8E28;
    'dispatch: loop {
        match pc {
            0x832A8E28 => {
    //   block [0x832A8E28..0x832A8E34)
	// 832A8E28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E2C: 386B7D78  addi r3, r11, 0x7d78
	ctx.r[3].s64 = ctx.r[11].s64 + 32120;
	// 832A8E30: 4AF6BFA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E38 size=12
    let mut pc: u32 = 0x832A8E38;
    'dispatch: loop {
        match pc {
            0x832A8E38 => {
    //   block [0x832A8E38..0x832A8E44)
	// 832A8E38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E3C: 386B7D7C  addi r3, r11, 0x7d7c
	ctx.r[3].s64 = ctx.r[11].s64 + 32124;
	// 832A8E40: 4AF6BF98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E48 size=12
    let mut pc: u32 = 0x832A8E48;
    'dispatch: loop {
        match pc {
            0x832A8E48 => {
    //   block [0x832A8E48..0x832A8E54)
	// 832A8E48: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E4C: 386B7D80  addi r3, r11, 0x7d80
	ctx.r[3].s64 = ctx.r[11].s64 + 32128;
	// 832A8E50: 4AF6BF88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E58 size=12
    let mut pc: u32 = 0x832A8E58;
    'dispatch: loop {
        match pc {
            0x832A8E58 => {
    //   block [0x832A8E58..0x832A8E64)
	// 832A8E58: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E5C: 386B7D84  addi r3, r11, 0x7d84
	ctx.r[3].s64 = ctx.r[11].s64 + 32132;
	// 832A8E60: 4AF6BF78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E68 size=12
    let mut pc: u32 = 0x832A8E68;
    'dispatch: loop {
        match pc {
            0x832A8E68 => {
    //   block [0x832A8E68..0x832A8E74)
	// 832A8E68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E6C: 386B7D88  addi r3, r11, 0x7d88
	ctx.r[3].s64 = ctx.r[11].s64 + 32136;
	// 832A8E70: 4AF6BF68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E78 size=12
    let mut pc: u32 = 0x832A8E78;
    'dispatch: loop {
        match pc {
            0x832A8E78 => {
    //   block [0x832A8E78..0x832A8E84)
	// 832A8E78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E7C: 386B7D8C  addi r3, r11, 0x7d8c
	ctx.r[3].s64 = ctx.r[11].s64 + 32140;
	// 832A8E80: 4AF6BF58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E88 size=12
    let mut pc: u32 = 0x832A8E88;
    'dispatch: loop {
        match pc {
            0x832A8E88 => {
    //   block [0x832A8E88..0x832A8E94)
	// 832A8E88: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E8C: 386B7D90  addi r3, r11, 0x7d90
	ctx.r[3].s64 = ctx.r[11].s64 + 32144;
	// 832A8E90: 4AF6BF48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8E98 size=12
    let mut pc: u32 = 0x832A8E98;
    'dispatch: loop {
        match pc {
            0x832A8E98 => {
    //   block [0x832A8E98..0x832A8EA4)
	// 832A8E98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8E9C: 386B7D94  addi r3, r11, 0x7d94
	ctx.r[3].s64 = ctx.r[11].s64 + 32148;
	// 832A8EA0: 4AF6BF38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8EA8 size=12
    let mut pc: u32 = 0x832A8EA8;
    'dispatch: loop {
        match pc {
            0x832A8EA8 => {
    //   block [0x832A8EA8..0x832A8EB4)
	// 832A8EA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8EAC: 386B7DB4  addi r3, r11, 0x7db4
	ctx.r[3].s64 = ctx.r[11].s64 + 32180;
	// 832A8EB0: 4AF6BF28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8EB8 size=12
    let mut pc: u32 = 0x832A8EB8;
    'dispatch: loop {
        match pc {
            0x832A8EB8 => {
    //   block [0x832A8EB8..0x832A8EC4)
	// 832A8EB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8EBC: 386B7DB8  addi r3, r11, 0x7db8
	ctx.r[3].s64 = ctx.r[11].s64 + 32184;
	// 832A8EC0: 4AF6BF18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8EC8 size=12
    let mut pc: u32 = 0x832A8EC8;
    'dispatch: loop {
        match pc {
            0x832A8EC8 => {
    //   block [0x832A8EC8..0x832A8ED4)
	// 832A8EC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8ECC: 386B7DBC  addi r3, r11, 0x7dbc
	ctx.r[3].s64 = ctx.r[11].s64 + 32188;
	// 832A8ED0: 4AF6BF08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8ED8 size=12
    let mut pc: u32 = 0x832A8ED8;
    'dispatch: loop {
        match pc {
            0x832A8ED8 => {
    //   block [0x832A8ED8..0x832A8EE4)
	// 832A8ED8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8EDC: 386B7DC0  addi r3, r11, 0x7dc0
	ctx.r[3].s64 = ctx.r[11].s64 + 32192;
	// 832A8EE0: 4AF6BEF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8EE8 size=12
    let mut pc: u32 = 0x832A8EE8;
    'dispatch: loop {
        match pc {
            0x832A8EE8 => {
    //   block [0x832A8EE8..0x832A8EF4)
	// 832A8EE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8EEC: 386B7DC4  addi r3, r11, 0x7dc4
	ctx.r[3].s64 = ctx.r[11].s64 + 32196;
	// 832A8EF0: 4AF6BEE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8EF8 size=12
    let mut pc: u32 = 0x832A8EF8;
    'dispatch: loop {
        match pc {
            0x832A8EF8 => {
    //   block [0x832A8EF8..0x832A8F04)
	// 832A8EF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8EFC: 386B7DD0  addi r3, r11, 0x7dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 32208;
	// 832A8F00: 4AF6BED8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F08 size=12
    let mut pc: u32 = 0x832A8F08;
    'dispatch: loop {
        match pc {
            0x832A8F08 => {
    //   block [0x832A8F08..0x832A8F14)
	// 832A8F08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F0C: 386B7DD4  addi r3, r11, 0x7dd4
	ctx.r[3].s64 = ctx.r[11].s64 + 32212;
	// 832A8F10: 4AF6BEC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F18 size=12
    let mut pc: u32 = 0x832A8F18;
    'dispatch: loop {
        match pc {
            0x832A8F18 => {
    //   block [0x832A8F18..0x832A8F24)
	// 832A8F18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F1C: 386B7DD8  addi r3, r11, 0x7dd8
	ctx.r[3].s64 = ctx.r[11].s64 + 32216;
	// 832A8F20: 4AF6BEB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F28 size=12
    let mut pc: u32 = 0x832A8F28;
    'dispatch: loop {
        match pc {
            0x832A8F28 => {
    //   block [0x832A8F28..0x832A8F34)
	// 832A8F28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F2C: 386B7DDC  addi r3, r11, 0x7ddc
	ctx.r[3].s64 = ctx.r[11].s64 + 32220;
	// 832A8F30: 4AF6BEA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F38 size=12
    let mut pc: u32 = 0x832A8F38;
    'dispatch: loop {
        match pc {
            0x832A8F38 => {
    //   block [0x832A8F38..0x832A8F44)
	// 832A8F38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F3C: 386B7DE0  addi r3, r11, 0x7de0
	ctx.r[3].s64 = ctx.r[11].s64 + 32224;
	// 832A8F40: 4AF6BE98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F48 size=12
    let mut pc: u32 = 0x832A8F48;
    'dispatch: loop {
        match pc {
            0x832A8F48 => {
    //   block [0x832A8F48..0x832A8F54)
	// 832A8F48: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F4C: 386B7DE4  addi r3, r11, 0x7de4
	ctx.r[3].s64 = ctx.r[11].s64 + 32228;
	// 832A8F50: 4AF6BE88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F58 size=12
    let mut pc: u32 = 0x832A8F58;
    'dispatch: loop {
        match pc {
            0x832A8F58 => {
    //   block [0x832A8F58..0x832A8F64)
	// 832A8F58: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F5C: 386B7DE8  addi r3, r11, 0x7de8
	ctx.r[3].s64 = ctx.r[11].s64 + 32232;
	// 832A8F60: 4AF6BE78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F68 size=12
    let mut pc: u32 = 0x832A8F68;
    'dispatch: loop {
        match pc {
            0x832A8F68 => {
    //   block [0x832A8F68..0x832A8F74)
	// 832A8F68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F6C: 386B7DEC  addi r3, r11, 0x7dec
	ctx.r[3].s64 = ctx.r[11].s64 + 32236;
	// 832A8F70: 4AF6BE68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F78 size=12
    let mut pc: u32 = 0x832A8F78;
    'dispatch: loop {
        match pc {
            0x832A8F78 => {
    //   block [0x832A8F78..0x832A8F84)
	// 832A8F78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F7C: 386B7DF0  addi r3, r11, 0x7df0
	ctx.r[3].s64 = ctx.r[11].s64 + 32240;
	// 832A8F80: 4AF6BE58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F88 size=12
    let mut pc: u32 = 0x832A8F88;
    'dispatch: loop {
        match pc {
            0x832A8F88 => {
    //   block [0x832A8F88..0x832A8F94)
	// 832A8F88: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F8C: 386B7DF4  addi r3, r11, 0x7df4
	ctx.r[3].s64 = ctx.r[11].s64 + 32244;
	// 832A8F90: 4AF6BE48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8F98 size=12
    let mut pc: u32 = 0x832A8F98;
    'dispatch: loop {
        match pc {
            0x832A8F98 => {
    //   block [0x832A8F98..0x832A8FA4)
	// 832A8F98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8F9C: 386B7DF8  addi r3, r11, 0x7df8
	ctx.r[3].s64 = ctx.r[11].s64 + 32248;
	// 832A8FA0: 4AF6BE38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8FA8 size=12
    let mut pc: u32 = 0x832A8FA8;
    'dispatch: loop {
        match pc {
            0x832A8FA8 => {
    //   block [0x832A8FA8..0x832A8FB4)
	// 832A8FA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8FAC: 386B7DFC  addi r3, r11, 0x7dfc
	ctx.r[3].s64 = ctx.r[11].s64 + 32252;
	// 832A8FB0: 4AF6BE28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8FB8 size=12
    let mut pc: u32 = 0x832A8FB8;
    'dispatch: loop {
        match pc {
            0x832A8FB8 => {
    //   block [0x832A8FB8..0x832A8FC4)
	// 832A8FB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8FBC: 386B7E00  addi r3, r11, 0x7e00
	ctx.r[3].s64 = ctx.r[11].s64 + 32256;
	// 832A8FC0: 4AF6BE18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8FC8 size=12
    let mut pc: u32 = 0x832A8FC8;
    'dispatch: loop {
        match pc {
            0x832A8FC8 => {
    //   block [0x832A8FC8..0x832A8FD4)
	// 832A8FC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8FCC: 386B7E04  addi r3, r11, 0x7e04
	ctx.r[3].s64 = ctx.r[11].s64 + 32260;
	// 832A8FD0: 4AF6BE08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8FD8 size=12
    let mut pc: u32 = 0x832A8FD8;
    'dispatch: loop {
        match pc {
            0x832A8FD8 => {
    //   block [0x832A8FD8..0x832A8FE4)
	// 832A8FD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8FDC: 386B7E08  addi r3, r11, 0x7e08
	ctx.r[3].s64 = ctx.r[11].s64 + 32264;
	// 832A8FE0: 4AF6BDF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8FE8 size=12
    let mut pc: u32 = 0x832A8FE8;
    'dispatch: loop {
        match pc {
            0x832A8FE8 => {
    //   block [0x832A8FE8..0x832A8FF4)
	// 832A8FE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8FEC: 386B7E0C  addi r3, r11, 0x7e0c
	ctx.r[3].s64 = ctx.r[11].s64 + 32268;
	// 832A8FF0: 4AF6BDE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8FF8 size=12
    let mut pc: u32 = 0x832A8FF8;
    'dispatch: loop {
        match pc {
            0x832A8FF8 => {
    //   block [0x832A8FF8..0x832A9004)
	// 832A8FF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8FFC: 386B7E10  addi r3, r11, 0x7e10
	ctx.r[3].s64 = ctx.r[11].s64 + 32272;
	// 832A9000: 4AF6BDD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9008 size=12
    let mut pc: u32 = 0x832A9008;
    'dispatch: loop {
        match pc {
            0x832A9008 => {
    //   block [0x832A9008..0x832A9014)
	// 832A9008: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A900C: 386B7E14  addi r3, r11, 0x7e14
	ctx.r[3].s64 = ctx.r[11].s64 + 32276;
	// 832A9010: 4AF6BDC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9018 size=12
    let mut pc: u32 = 0x832A9018;
    'dispatch: loop {
        match pc {
            0x832A9018 => {
    //   block [0x832A9018..0x832A9024)
	// 832A9018: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A901C: 386B7E18  addi r3, r11, 0x7e18
	ctx.r[3].s64 = ctx.r[11].s64 + 32280;
	// 832A9020: 4AF6BDB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9028 size=12
    let mut pc: u32 = 0x832A9028;
    'dispatch: loop {
        match pc {
            0x832A9028 => {
    //   block [0x832A9028..0x832A9034)
	// 832A9028: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A902C: 386B7E1C  addi r3, r11, 0x7e1c
	ctx.r[3].s64 = ctx.r[11].s64 + 32284;
	// 832A9030: 4AF6BDA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9038 size=12
    let mut pc: u32 = 0x832A9038;
    'dispatch: loop {
        match pc {
            0x832A9038 => {
    //   block [0x832A9038..0x832A9044)
	// 832A9038: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A903C: 386B7E20  addi r3, r11, 0x7e20
	ctx.r[3].s64 = ctx.r[11].s64 + 32288;
	// 832A9040: 4AF6BD98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9048 size=12
    let mut pc: u32 = 0x832A9048;
    'dispatch: loop {
        match pc {
            0x832A9048 => {
    //   block [0x832A9048..0x832A9054)
	// 832A9048: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A904C: 386B7E24  addi r3, r11, 0x7e24
	ctx.r[3].s64 = ctx.r[11].s64 + 32292;
	// 832A9050: 4AF6BD88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9058 size=12
    let mut pc: u32 = 0x832A9058;
    'dispatch: loop {
        match pc {
            0x832A9058 => {
    //   block [0x832A9058..0x832A9064)
	// 832A9058: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A905C: 386B7E28  addi r3, r11, 0x7e28
	ctx.r[3].s64 = ctx.r[11].s64 + 32296;
	// 832A9060: 4AF6BD78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9068 size=12
    let mut pc: u32 = 0x832A9068;
    'dispatch: loop {
        match pc {
            0x832A9068 => {
    //   block [0x832A9068..0x832A9074)
	// 832A9068: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A906C: 386B7E2C  addi r3, r11, 0x7e2c
	ctx.r[3].s64 = ctx.r[11].s64 + 32300;
	// 832A9070: 4AF6BD68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9078 size=12
    let mut pc: u32 = 0x832A9078;
    'dispatch: loop {
        match pc {
            0x832A9078 => {
    //   block [0x832A9078..0x832A9084)
	// 832A9078: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A907C: 386B7E30  addi r3, r11, 0x7e30
	ctx.r[3].s64 = ctx.r[11].s64 + 32304;
	// 832A9080: 4AF6BD58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9088 size=12
    let mut pc: u32 = 0x832A9088;
    'dispatch: loop {
        match pc {
            0x832A9088 => {
    //   block [0x832A9088..0x832A9094)
	// 832A9088: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A908C: 386B7E34  addi r3, r11, 0x7e34
	ctx.r[3].s64 = ctx.r[11].s64 + 32308;
	// 832A9090: 4AF6BD48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9098 size=12
    let mut pc: u32 = 0x832A9098;
    'dispatch: loop {
        match pc {
            0x832A9098 => {
    //   block [0x832A9098..0x832A90A4)
	// 832A9098: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A909C: 386B7E38  addi r3, r11, 0x7e38
	ctx.r[3].s64 = ctx.r[11].s64 + 32312;
	// 832A90A0: 4AF6BD38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A90A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A90A8 size=12
    let mut pc: u32 = 0x832A90A8;
    'dispatch: loop {
        match pc {
            0x832A90A8 => {
    //   block [0x832A90A8..0x832A90B4)
	// 832A90A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A90AC: 386B7E3C  addi r3, r11, 0x7e3c
	ctx.r[3].s64 = ctx.r[11].s64 + 32316;
	// 832A90B0: 4AF6BD28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A90B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A90B8 size=12
    let mut pc: u32 = 0x832A90B8;
    'dispatch: loop {
        match pc {
            0x832A90B8 => {
    //   block [0x832A90B8..0x832A90C4)
	// 832A90B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A90BC: 386B7E40  addi r3, r11, 0x7e40
	ctx.r[3].s64 = ctx.r[11].s64 + 32320;
	// 832A90C0: 4AF6BD18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A90C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A90C8 size=12
    let mut pc: u32 = 0x832A90C8;
    'dispatch: loop {
        match pc {
            0x832A90C8 => {
    //   block [0x832A90C8..0x832A90D4)
	// 832A90C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A90CC: 386B7E44  addi r3, r11, 0x7e44
	ctx.r[3].s64 = ctx.r[11].s64 + 32324;
	// 832A90D0: 4AF6BD08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A90D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A90D8 size=12
    let mut pc: u32 = 0x832A90D8;
    'dispatch: loop {
        match pc {
            0x832A90D8 => {
    //   block [0x832A90D8..0x832A90E4)
	// 832A90D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A90DC: 386B7E48  addi r3, r11, 0x7e48
	ctx.r[3].s64 = ctx.r[11].s64 + 32328;
	// 832A90E0: 4AF6BCF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A90E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A90E8 size=12
    let mut pc: u32 = 0x832A90E8;
    'dispatch: loop {
        match pc {
            0x832A90E8 => {
    //   block [0x832A90E8..0x832A90F4)
	// 832A90E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A90EC: 386B7E4C  addi r3, r11, 0x7e4c
	ctx.r[3].s64 = ctx.r[11].s64 + 32332;
	// 832A90F0: 4AF6BCE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A90F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A90F8 size=12
    let mut pc: u32 = 0x832A90F8;
    'dispatch: loop {
        match pc {
            0x832A90F8 => {
    //   block [0x832A90F8..0x832A9104)
	// 832A90F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A90FC: 386B7E50  addi r3, r11, 0x7e50
	ctx.r[3].s64 = ctx.r[11].s64 + 32336;
	// 832A9100: 4AF6BCD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9108 size=12
    let mut pc: u32 = 0x832A9108;
    'dispatch: loop {
        match pc {
            0x832A9108 => {
    //   block [0x832A9108..0x832A9114)
	// 832A9108: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A910C: 386B7E54  addi r3, r11, 0x7e54
	ctx.r[3].s64 = ctx.r[11].s64 + 32340;
	// 832A9110: 4AF6BCC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A9118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A9118 size=12
    let mut pc: u32 = 0x832A9118;
    'dispatch: loop {
        match pc {
            0x832A9118 => {
    //   block [0x832A9118..0x832A9124)
	// 832A9118: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A911C: 386B7E58  addi r3, r11, 0x7e58
	ctx.r[3].s64 = ctx.r[11].s64 + 32344;
	// 832A9120: 4AF6BCB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


