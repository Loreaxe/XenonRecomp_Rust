pub fn sub_832ABDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABDE8 size=4
    let mut pc: u32 = 0x832ABDE8;
    'dispatch: loop {
        match pc {
            0x832ABDE8 => {
    //   block [0x832ABDE8..0x832ABDEC)
	// 832ABDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABDF0 size=104
    let mut pc: u32 = 0x832ABDF0;
    'dispatch: loop {
        match pc {
            0x832ABDF0 => {
    //   block [0x832ABDF0..0x832ABE58)
	// 832ABDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABDF4: 4B9FD615  bl 0x82ca9408
	ctx.lr = 0x832ABDF8;
	sub_82CA93D0(ctx, base);
	// 832ABDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABDFC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE00: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 832ABE04: 396BAA08  addi r11, r11, -0x55f8
	ctx.r[11].s64 = ctx.r[11].s64 + -22008;
	// 832ABE08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ABE0C: 3BEB0088  addi r31, r11, 0x88
	ctx.r[31].s64 = ctx.r[11].s64 + 136;
	// 832ABE10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ABE14: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ABE18: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832ABE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABE20: 4AF1A949  bl 0x821c6768
	ctx.lr = 0x832ABE24;
	sub_821C6768(ctx, base);
	// 832ABE24: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ABE28: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ABE2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE30: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ABE34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ABE38: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ABE3C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE40: 4082FFE8  bne 0x832abe28
	if !ctx.cr[0].eq {
	pc = 0x832ABE28; continue 'dispatch;
	}
	// 832ABE44: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ABE48: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ABE4C: 4080FFCC  bge 0x832abe18
	if !ctx.cr[0].lt {
	pc = 0x832ABE18; continue 'dispatch;
	}
	// 832ABE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ABE54: 4B9FD604  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABE58 size=12
    let mut pc: u32 = 0x832ABE58;
    'dispatch: loop {
        match pc {
            0x832ABE58 => {
    //   block [0x832ABE58..0x832ABE64)
	// 832ABE58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE5C: 386BAA90  addi r3, r11, -0x5570
	ctx.r[3].s64 = ctx.r[11].s64 + -21872;
	// 832ABE60: 4AF68F78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABE68 size=12
    let mut pc: u32 = 0x832ABE68;
    'dispatch: loop {
        match pc {
            0x832ABE68 => {
    //   block [0x832ABE68..0x832ABE74)
	// 832ABE68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE6C: 386BAA94  addi r3, r11, -0x556c
	ctx.r[3].s64 = ctx.r[11].s64 + -21868;
	// 832ABE70: 4AF68F68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABE78 size=12
    let mut pc: u32 = 0x832ABE78;
    'dispatch: loop {
        match pc {
            0x832ABE78 => {
    //   block [0x832ABE78..0x832ABE84)
	// 832ABE78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE7C: 386BAA98  addi r3, r11, -0x5568
	ctx.r[3].s64 = ctx.r[11].s64 + -21864;
	// 832ABE80: 4AF68F58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABE88 size=100
    let mut pc: u32 = 0x832ABE88;
    'dispatch: loop {
        match pc {
            0x832ABE88 => {
    //   block [0x832ABE88..0x832ABEEC)
	// 832ABE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABE94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABE98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE9C: 3BEBAA9C  addi r31, r11, -0x5564
	ctx.r[31].s64 = ctx.r[11].s64 + -21860;
	// 832ABEA0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832ABEA8: 419A0018  beq cr6, 0x832abec0
	if ctx.cr[6].eq {
	pc = 0x832ABEC0; continue 'dispatch;
	}
	// 832ABEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABEB0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832ABEB4: 4B134015  bl 0x823dfec8
	ctx.lr = 0x832ABEB8;
	sub_823DFEC8(ctx, base);
	// 832ABEB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEBC: 4AF6FE7D  bl 0x8221bd38
	ctx.lr = 0x832ABEC0;
	sub_8221BD38(ctx, base);
	// 832ABEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABEC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABEC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABECC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABED0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABED4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABEE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABEF0 size=12
    let mut pc: u32 = 0x832ABEF0;
    'dispatch: loop {
        match pc {
            0x832ABEF0 => {
    //   block [0x832ABEF0..0x832ABEFC)
	// 832ABEF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABEF4: 386BAAB0  addi r3, r11, -0x5550
	ctx.r[3].s64 = ctx.r[11].s64 + -21840;
	// 832ABEF8: 4AF68EE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF00 size=12
    let mut pc: u32 = 0x832ABF00;
    'dispatch: loop {
        match pc {
            0x832ABF00 => {
    //   block [0x832ABF00..0x832ABF0C)
	// 832ABF00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF04: 386BAAB4  addi r3, r11, -0x554c
	ctx.r[3].s64 = ctx.r[11].s64 + -21836;
	// 832ABF08: 4AF68ED0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF10 size=12
    let mut pc: u32 = 0x832ABF10;
    'dispatch: loop {
        match pc {
            0x832ABF10 => {
    //   block [0x832ABF10..0x832ABF1C)
	// 832ABF10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF14: 386BAAB8  addi r3, r11, -0x5548
	ctx.r[3].s64 = ctx.r[11].s64 + -21832;
	// 832ABF18: 4AF68EC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF20 size=12
    let mut pc: u32 = 0x832ABF20;
    'dispatch: loop {
        match pc {
            0x832ABF20 => {
    //   block [0x832ABF20..0x832ABF2C)
	// 832ABF20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF24: 386BAABC  addi r3, r11, -0x5544
	ctx.r[3].s64 = ctx.r[11].s64 + -21828;
	// 832ABF28: 4AF68EB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF30 size=12
    let mut pc: u32 = 0x832ABF30;
    'dispatch: loop {
        match pc {
            0x832ABF30 => {
    //   block [0x832ABF30..0x832ABF3C)
	// 832ABF30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF34: 386BAAC0  addi r3, r11, -0x5540
	ctx.r[3].s64 = ctx.r[11].s64 + -21824;
	// 832ABF38: 4AF68EA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF40 size=12
    let mut pc: u32 = 0x832ABF40;
    'dispatch: loop {
        match pc {
            0x832ABF40 => {
    //   block [0x832ABF40..0x832ABF4C)
	// 832ABF40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF44: 386BAAC4  addi r3, r11, -0x553c
	ctx.r[3].s64 = ctx.r[11].s64 + -21820;
	// 832ABF48: 4AF68E90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF50 size=12
    let mut pc: u32 = 0x832ABF50;
    'dispatch: loop {
        match pc {
            0x832ABF50 => {
    //   block [0x832ABF50..0x832ABF5C)
	// 832ABF50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF54: 386BAAC8  addi r3, r11, -0x5538
	ctx.r[3].s64 = ctx.r[11].s64 + -21816;
	// 832ABF58: 4AF68E80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF60 size=12
    let mut pc: u32 = 0x832ABF60;
    'dispatch: loop {
        match pc {
            0x832ABF60 => {
    //   block [0x832ABF60..0x832ABF6C)
	// 832ABF60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF64: 386BAACC  addi r3, r11, -0x5534
	ctx.r[3].s64 = ctx.r[11].s64 + -21812;
	// 832ABF68: 4AF68E70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF70 size=12
    let mut pc: u32 = 0x832ABF70;
    'dispatch: loop {
        match pc {
            0x832ABF70 => {
    //   block [0x832ABF70..0x832ABF7C)
	// 832ABF70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF74: 386BAAD0  addi r3, r11, -0x5530
	ctx.r[3].s64 = ctx.r[11].s64 + -21808;
	// 832ABF78: 4AF68E60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF80 size=12
    let mut pc: u32 = 0x832ABF80;
    'dispatch: loop {
        match pc {
            0x832ABF80 => {
    //   block [0x832ABF80..0x832ABF8C)
	// 832ABF80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF84: 386BAAD4  addi r3, r11, -0x552c
	ctx.r[3].s64 = ctx.r[11].s64 + -21804;
	// 832ABF88: 4AF68E50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF90 size=12
    let mut pc: u32 = 0x832ABF90;
    'dispatch: loop {
        match pc {
            0x832ABF90 => {
    //   block [0x832ABF90..0x832ABF9C)
	// 832ABF90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF94: 386BAAD8  addi r3, r11, -0x5528
	ctx.r[3].s64 = ctx.r[11].s64 + -21800;
	// 832ABF98: 4AF68E40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFA0 size=12
    let mut pc: u32 = 0x832ABFA0;
    'dispatch: loop {
        match pc {
            0x832ABFA0 => {
    //   block [0x832ABFA0..0x832ABFAC)
	// 832ABFA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFA4: 386BAADC  addi r3, r11, -0x5524
	ctx.r[3].s64 = ctx.r[11].s64 + -21796;
	// 832ABFA8: 4AF68E30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFB0 size=12
    let mut pc: u32 = 0x832ABFB0;
    'dispatch: loop {
        match pc {
            0x832ABFB0 => {
    //   block [0x832ABFB0..0x832ABFBC)
	// 832ABFB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFB4: 386BAAE0  addi r3, r11, -0x5520
	ctx.r[3].s64 = ctx.r[11].s64 + -21792;
	// 832ABFB8: 4AF68E20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFC0 size=12
    let mut pc: u32 = 0x832ABFC0;
    'dispatch: loop {
        match pc {
            0x832ABFC0 => {
    //   block [0x832ABFC0..0x832ABFCC)
	// 832ABFC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFC4: 386BAAE4  addi r3, r11, -0x551c
	ctx.r[3].s64 = ctx.r[11].s64 + -21788;
	// 832ABFC8: 4AF68E10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFD0 size=12
    let mut pc: u32 = 0x832ABFD0;
    'dispatch: loop {
        match pc {
            0x832ABFD0 => {
    //   block [0x832ABFD0..0x832ABFDC)
	// 832ABFD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFD4: 386BAAE8  addi r3, r11, -0x5518
	ctx.r[3].s64 = ctx.r[11].s64 + -21784;
	// 832ABFD8: 4AF68E00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFE0 size=12
    let mut pc: u32 = 0x832ABFE0;
    'dispatch: loop {
        match pc {
            0x832ABFE0 => {
    //   block [0x832ABFE0..0x832ABFEC)
	// 832ABFE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFE4: 386BAAEC  addi r3, r11, -0x5514
	ctx.r[3].s64 = ctx.r[11].s64 + -21780;
	// 832ABFE8: 4AF68DF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFF0 size=12
    let mut pc: u32 = 0x832ABFF0;
    'dispatch: loop {
        match pc {
            0x832ABFF0 => {
    //   block [0x832ABFF0..0x832ABFFC)
	// 832ABFF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFF4: 386BAAF0  addi r3, r11, -0x5510
	ctx.r[3].s64 = ctx.r[11].s64 + -21776;
	// 832ABFF8: 4AF68DE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC000 size=12
    let mut pc: u32 = 0x832AC000;
    'dispatch: loop {
        match pc {
            0x832AC000 => {
    //   block [0x832AC000..0x832AC00C)
	// 832AC000: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC004: 386BAAF4  addi r3, r11, -0x550c
	ctx.r[3].s64 = ctx.r[11].s64 + -21772;
	// 832AC008: 4AF68DD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC010 size=12
    let mut pc: u32 = 0x832AC010;
    'dispatch: loop {
        match pc {
            0x832AC010 => {
    //   block [0x832AC010..0x832AC01C)
	// 832AC010: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC014: 386BAAF8  addi r3, r11, -0x5508
	ctx.r[3].s64 = ctx.r[11].s64 + -21768;
	// 832AC018: 4AF68DC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC020 size=12
    let mut pc: u32 = 0x832AC020;
    'dispatch: loop {
        match pc {
            0x832AC020 => {
    //   block [0x832AC020..0x832AC02C)
	// 832AC020: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC024: 386BAAFC  addi r3, r11, -0x5504
	ctx.r[3].s64 = ctx.r[11].s64 + -21764;
	// 832AC028: 4AF68DB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC030 size=12
    let mut pc: u32 = 0x832AC030;
    'dispatch: loop {
        match pc {
            0x832AC030 => {
    //   block [0x832AC030..0x832AC03C)
	// 832AC030: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC034: 386BAB00  addi r3, r11, -0x5500
	ctx.r[3].s64 = ctx.r[11].s64 + -21760;
	// 832AC038: 4AF68DA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC040 size=12
    let mut pc: u32 = 0x832AC040;
    'dispatch: loop {
        match pc {
            0x832AC040 => {
    //   block [0x832AC040..0x832AC04C)
	// 832AC040: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC044: 386BAB04  addi r3, r11, -0x54fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21756;
	// 832AC048: 4AF68D90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC050 size=12
    let mut pc: u32 = 0x832AC050;
    'dispatch: loop {
        match pc {
            0x832AC050 => {
    //   block [0x832AC050..0x832AC05C)
	// 832AC050: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC054: 386BAB08  addi r3, r11, -0x54f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21752;
	// 832AC058: 4AF68D80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC060 size=12
    let mut pc: u32 = 0x832AC060;
    'dispatch: loop {
        match pc {
            0x832AC060 => {
    //   block [0x832AC060..0x832AC06C)
	// 832AC060: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC064: 386BAB0C  addi r3, r11, -0x54f4
	ctx.r[3].s64 = ctx.r[11].s64 + -21748;
	// 832AC068: 4B075678  b 0x823216e0
	sub_823216E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC070 size=12
    let mut pc: u32 = 0x832AC070;
    'dispatch: loop {
        match pc {
            0x832AC070 => {
    //   block [0x832AC070..0x832AC07C)
	// 832AC070: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC074: 386BAB18  addi r3, r11, -0x54e8
	ctx.r[3].s64 = ctx.r[11].s64 + -21736;
	// 832AC078: 4AF68D60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC080 size=12
    let mut pc: u32 = 0x832AC080;
    'dispatch: loop {
        match pc {
            0x832AC080 => {
    //   block [0x832AC080..0x832AC08C)
	// 832AC080: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC084: 386BAB1C  addi r3, r11, -0x54e4
	ctx.r[3].s64 = ctx.r[11].s64 + -21732;
	// 832AC088: 4AF68D50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC090 size=12
    let mut pc: u32 = 0x832AC090;
    'dispatch: loop {
        match pc {
            0x832AC090 => {
    //   block [0x832AC090..0x832AC09C)
	// 832AC090: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC094: 386BAB20  addi r3, r11, -0x54e0
	ctx.r[3].s64 = ctx.r[11].s64 + -21728;
	// 832AC098: 4AF68D40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC0A0 size=12
    let mut pc: u32 = 0x832AC0A0;
    'dispatch: loop {
        match pc {
            0x832AC0A0 => {
    //   block [0x832AC0A0..0x832AC0AC)
	// 832AC0A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC0A4: 386BAB24  addi r3, r11, -0x54dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21724;
	// 832AC0A8: 4B76BCB8  b 0x82a17d60
	sub_82A17D60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AC0B0 size=104
    let mut pc: u32 = 0x832AC0B0;
    'dispatch: loop {
        match pc {
            0x832AC0B0 => {
    //   block [0x832AC0B0..0x832AC118)
	// 832AC0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AC0B4: 4B9FD355  bl 0x82ca9408
	ctx.lr = 0x832AC0B8;
	sub_82CA93D0(ctx, base);
	// 832AC0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AC0BC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC0C0: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 832AC0C4: 396BAB30  addi r11, r11, -0x54d0
	ctx.r[11].s64 = ctx.r[11].s64 + -21712;
	// 832AC0C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AC0CC: 3BEB0024  addi r31, r11, 0x24
	ctx.r[31].s64 = ctx.r[11].s64 + 36;
	// 832AC0D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AC0D4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AC0D8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AC0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AC0E0: 4AF1A689  bl 0x821c6768
	ctx.lr = 0x832AC0E4;
	sub_821C6768(ctx, base);
	// 832AC0E4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AC0E8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AC0EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC0F0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AC0F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AC0F8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AC0FC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC100: 4082FFE8  bne 0x832ac0e8
	if !ctx.cr[0].eq {
	pc = 0x832AC0E8; continue 'dispatch;
	}
	// 832AC104: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AC108: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AC10C: 4080FFCC  bge 0x832ac0d8
	if !ctx.cr[0].lt {
	pc = 0x832AC0D8; continue 'dispatch;
	}
	// 832AC110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AC114: 4B9FD344  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC118 size=4
    let mut pc: u32 = 0x832AC118;
    'dispatch: loop {
        match pc {
            0x832AC118 => {
    //   block [0x832AC118..0x832AC11C)
	// 832AC118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC120 size=4
    let mut pc: u32 = 0x832AC120;
    'dispatch: loop {
        match pc {
            0x832AC120 => {
    //   block [0x832AC120..0x832AC124)
	// 832AC120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC128 size=12
    let mut pc: u32 = 0x832AC128;
    'dispatch: loop {
        match pc {
            0x832AC128 => {
    //   block [0x832AC128..0x832AC134)
	// 832AC128: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AC12C: 386B3D2C  addi r3, r11, 0x3d2c
	ctx.r[3].s64 = ctx.r[11].s64 + 15660;
	// 832AC130: 4AF68CA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC138 size=12
    let mut pc: u32 = 0x832AC138;
    'dispatch: loop {
        match pc {
            0x832AC138 => {
    //   block [0x832AC138..0x832AC144)
	// 832AC138: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC13C: 386BAB54  addi r3, r11, -0x54ac
	ctx.r[3].s64 = ctx.r[11].s64 + -21676;
	// 832AC140: 4AF68C98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC148 size=12
    let mut pc: u32 = 0x832AC148;
    'dispatch: loop {
        match pc {
            0x832AC148 => {
    //   block [0x832AC148..0x832AC154)
	// 832AC148: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC14C: 386BAB58  addi r3, r11, -0x54a8
	ctx.r[3].s64 = ctx.r[11].s64 + -21672;
	// 832AC150: 4AF68C88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC158 size=12
    let mut pc: u32 = 0x832AC158;
    'dispatch: loop {
        match pc {
            0x832AC158 => {
    //   block [0x832AC158..0x832AC164)
	// 832AC158: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC15C: 386BAB5C  addi r3, r11, -0x54a4
	ctx.r[3].s64 = ctx.r[11].s64 + -21668;
	// 832AC160: 4AF68C78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC168 size=12
    let mut pc: u32 = 0x832AC168;
    'dispatch: loop {
        match pc {
            0x832AC168 => {
    //   block [0x832AC168..0x832AC174)
	// 832AC168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC16C: 386BABB4  addi r3, r11, -0x544c
	ctx.r[3].s64 = ctx.r[11].s64 + -21580;
	// 832AC170: 4AF68C68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC178 size=12
    let mut pc: u32 = 0x832AC178;
    'dispatch: loop {
        match pc {
            0x832AC178 => {
    //   block [0x832AC178..0x832AC184)
	// 832AC178: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC17C: 386BABB8  addi r3, r11, -0x5448
	ctx.r[3].s64 = ctx.r[11].s64 + -21576;
	// 832AC180: 4AF68C58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC188 size=12
    let mut pc: u32 = 0x832AC188;
    'dispatch: loop {
        match pc {
            0x832AC188 => {
    //   block [0x832AC188..0x832AC194)
	// 832AC188: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC18C: 386BABBC  addi r3, r11, -0x5444
	ctx.r[3].s64 = ctx.r[11].s64 + -21572;
	// 832AC190: 4AF68C48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC198 size=12
    let mut pc: u32 = 0x832AC198;
    'dispatch: loop {
        match pc {
            0x832AC198 => {
    //   block [0x832AC198..0x832AC1A4)
	// 832AC198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC19C: 386BABC0  addi r3, r11, -0x5440
	ctx.r[3].s64 = ctx.r[11].s64 + -21568;
	// 832AC1A0: 4AF68C38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1A8 size=12
    let mut pc: u32 = 0x832AC1A8;
    'dispatch: loop {
        match pc {
            0x832AC1A8 => {
    //   block [0x832AC1A8..0x832AC1B4)
	// 832AC1A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1AC: 386BABC4  addi r3, r11, -0x543c
	ctx.r[3].s64 = ctx.r[11].s64 + -21564;
	// 832AC1B0: 4AF68C28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1B8 size=12
    let mut pc: u32 = 0x832AC1B8;
    'dispatch: loop {
        match pc {
            0x832AC1B8 => {
    //   block [0x832AC1B8..0x832AC1C4)
	// 832AC1B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1BC: 386BABC8  addi r3, r11, -0x5438
	ctx.r[3].s64 = ctx.r[11].s64 + -21560;
	// 832AC1C0: 4AF68C18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1C8 size=12
    let mut pc: u32 = 0x832AC1C8;
    'dispatch: loop {
        match pc {
            0x832AC1C8 => {
    //   block [0x832AC1C8..0x832AC1D4)
	// 832AC1C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1CC: 386BABCC  addi r3, r11, -0x5434
	ctx.r[3].s64 = ctx.r[11].s64 + -21556;
	// 832AC1D0: 4AF68C08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1D8 size=12
    let mut pc: u32 = 0x832AC1D8;
    'dispatch: loop {
        match pc {
            0x832AC1D8 => {
    //   block [0x832AC1D8..0x832AC1E4)
	// 832AC1D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1DC: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 832AC1E0: 4AF68BF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1E8 size=12
    let mut pc: u32 = 0x832AC1E8;
    'dispatch: loop {
        match pc {
            0x832AC1E8 => {
    //   block [0x832AC1E8..0x832AC1F4)
	// 832AC1E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1EC: 386BABD4  addi r3, r11, -0x542c
	ctx.r[3].s64 = ctx.r[11].s64 + -21548;
	// 832AC1F0: 4AF68BE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1F8 size=12
    let mut pc: u32 = 0x832AC1F8;
    'dispatch: loop {
        match pc {
            0x832AC1F8 => {
    //   block [0x832AC1F8..0x832AC204)
	// 832AC1F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1FC: 386BABD8  addi r3, r11, -0x5428
	ctx.r[3].s64 = ctx.r[11].s64 + -21544;
	// 832AC200: 4AF68BD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC208 size=12
    let mut pc: u32 = 0x832AC208;
    'dispatch: loop {
        match pc {
            0x832AC208 => {
    //   block [0x832AC208..0x832AC214)
	// 832AC208: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC20C: 386BABDC  addi r3, r11, -0x5424
	ctx.r[3].s64 = ctx.r[11].s64 + -21540;
	// 832AC210: 4AF68BC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC218 size=12
    let mut pc: u32 = 0x832AC218;
    'dispatch: loop {
        match pc {
            0x832AC218 => {
    //   block [0x832AC218..0x832AC224)
	// 832AC218: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC21C: 386BABE0  addi r3, r11, -0x5420
	ctx.r[3].s64 = ctx.r[11].s64 + -21536;
	// 832AC220: 4AF68BB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC228 size=12
    let mut pc: u32 = 0x832AC228;
    'dispatch: loop {
        match pc {
            0x832AC228 => {
    //   block [0x832AC228..0x832AC234)
	// 832AC228: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC22C: 386BABE4  addi r3, r11, -0x541c
	ctx.r[3].s64 = ctx.r[11].s64 + -21532;
	// 832AC230: 4AF68BA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC238 size=12
    let mut pc: u32 = 0x832AC238;
    'dispatch: loop {
        match pc {
            0x832AC238 => {
    //   block [0x832AC238..0x832AC244)
	// 832AC238: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC23C: 386BABE8  addi r3, r11, -0x5418
	ctx.r[3].s64 = ctx.r[11].s64 + -21528;
	// 832AC240: 4AF68B98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC248 size=12
    let mut pc: u32 = 0x832AC248;
    'dispatch: loop {
        match pc {
            0x832AC248 => {
    //   block [0x832AC248..0x832AC254)
	// 832AC248: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC24C: 386BABEC  addi r3, r11, -0x5414
	ctx.r[3].s64 = ctx.r[11].s64 + -21524;
	// 832AC250: 4AF68B88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC258 size=12
    let mut pc: u32 = 0x832AC258;
    'dispatch: loop {
        match pc {
            0x832AC258 => {
    //   block [0x832AC258..0x832AC264)
	// 832AC258: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC25C: 386BABF0  addi r3, r11, -0x5410
	ctx.r[3].s64 = ctx.r[11].s64 + -21520;
	// 832AC260: 4AF68B78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC268 size=12
    let mut pc: u32 = 0x832AC268;
    'dispatch: loop {
        match pc {
            0x832AC268 => {
    //   block [0x832AC268..0x832AC274)
	// 832AC268: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC26C: 386BABF4  addi r3, r11, -0x540c
	ctx.r[3].s64 = ctx.r[11].s64 + -21516;
	// 832AC270: 4AF68B68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC278 size=12
    let mut pc: u32 = 0x832AC278;
    'dispatch: loop {
        match pc {
            0x832AC278 => {
    //   block [0x832AC278..0x832AC284)
	// 832AC278: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC27C: 386BABF8  addi r3, r11, -0x5408
	ctx.r[3].s64 = ctx.r[11].s64 + -21512;
	// 832AC280: 4AF68B58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC288 size=12
    let mut pc: u32 = 0x832AC288;
    'dispatch: loop {
        match pc {
            0x832AC288 => {
    //   block [0x832AC288..0x832AC294)
	// 832AC288: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC28C: 386BABFC  addi r3, r11, -0x5404
	ctx.r[3].s64 = ctx.r[11].s64 + -21508;
	// 832AC290: 4AF68B48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC298 size=12
    let mut pc: u32 = 0x832AC298;
    'dispatch: loop {
        match pc {
            0x832AC298 => {
    //   block [0x832AC298..0x832AC2A4)
	// 832AC298: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC29C: 386BAC00  addi r3, r11, -0x5400
	ctx.r[3].s64 = ctx.r[11].s64 + -21504;
	// 832AC2A0: 4AF68B38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2A8 size=12
    let mut pc: u32 = 0x832AC2A8;
    'dispatch: loop {
        match pc {
            0x832AC2A8 => {
    //   block [0x832AC2A8..0x832AC2B4)
	// 832AC2A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2AC: 386BAC04  addi r3, r11, -0x53fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21500;
	// 832AC2B0: 4AF68B28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2B8 size=12
    let mut pc: u32 = 0x832AC2B8;
    'dispatch: loop {
        match pc {
            0x832AC2B8 => {
    //   block [0x832AC2B8..0x832AC2C4)
	// 832AC2B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2BC: 386BAC08  addi r3, r11, -0x53f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21496;
	// 832AC2C0: 4AF68B18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2C8 size=12
    let mut pc: u32 = 0x832AC2C8;
    'dispatch: loop {
        match pc {
            0x832AC2C8 => {
    //   block [0x832AC2C8..0x832AC2D4)
	// 832AC2C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2CC: 386BAC0C  addi r3, r11, -0x53f4
	ctx.r[3].s64 = ctx.r[11].s64 + -21492;
	// 832AC2D0: 4AF68B08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2D8 size=12
    let mut pc: u32 = 0x832AC2D8;
    'dispatch: loop {
        match pc {
            0x832AC2D8 => {
    //   block [0x832AC2D8..0x832AC2E4)
	// 832AC2D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2DC: 386BAC10  addi r3, r11, -0x53f0
	ctx.r[3].s64 = ctx.r[11].s64 + -21488;
	// 832AC2E0: 4AF68AF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2E8 size=4
    let mut pc: u32 = 0x832AC2E8;
    'dispatch: loop {
        match pc {
            0x832AC2E8 => {
    //   block [0x832AC2E8..0x832AC2EC)
	// 832AC2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2F0 size=4
    let mut pc: u32 = 0x832AC2F0;
    'dispatch: loop {
        match pc {
            0x832AC2F0 => {
    //   block [0x832AC2F0..0x832AC2F4)
	// 832AC2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2F8 size=4
    let mut pc: u32 = 0x832AC2F8;
    'dispatch: loop {
        match pc {
            0x832AC2F8 => {
    //   block [0x832AC2F8..0x832AC2FC)
	// 832AC2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC300 size=4
    let mut pc: u32 = 0x832AC300;
    'dispatch: loop {
        match pc {
            0x832AC300 => {
    //   block [0x832AC300..0x832AC304)
	// 832AC300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC308 size=4
    let mut pc: u32 = 0x832AC308;
    'dispatch: loop {
        match pc {
            0x832AC308 => {
    //   block [0x832AC308..0x832AC30C)
	// 832AC308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC310 size=4
    let mut pc: u32 = 0x832AC310;
    'dispatch: loop {
        match pc {
            0x832AC310 => {
    //   block [0x832AC310..0x832AC314)
	// 832AC310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC318 size=4
    let mut pc: u32 = 0x832AC318;
    'dispatch: loop {
        match pc {
            0x832AC318 => {
    //   block [0x832AC318..0x832AC31C)
	// 832AC318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC320 size=4
    let mut pc: u32 = 0x832AC320;
    'dispatch: loop {
        match pc {
            0x832AC320 => {
    //   block [0x832AC320..0x832AC324)
	// 832AC320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC328 size=4
    let mut pc: u32 = 0x832AC328;
    'dispatch: loop {
        match pc {
            0x832AC328 => {
    //   block [0x832AC328..0x832AC32C)
	// 832AC328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC330 size=4
    let mut pc: u32 = 0x832AC330;
    'dispatch: loop {
        match pc {
            0x832AC330 => {
    //   block [0x832AC330..0x832AC334)
	// 832AC330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC338 size=4
    let mut pc: u32 = 0x832AC338;
    'dispatch: loop {
        match pc {
            0x832AC338 => {
    //   block [0x832AC338..0x832AC33C)
	// 832AC338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC340 size=12
    let mut pc: u32 = 0x832AC340;
    'dispatch: loop {
        match pc {
            0x832AC340 => {
    //   block [0x832AC340..0x832AC34C)
	// 832AC340: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC344: 386BAC40  addi r3, r11, -0x53c0
	ctx.r[3].s64 = ctx.r[11].s64 + -21440;
	// 832AC348: 4AF68A90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC350 size=12
    let mut pc: u32 = 0x832AC350;
    'dispatch: loop {
        match pc {
            0x832AC350 => {
    //   block [0x832AC350..0x832AC35C)
	// 832AC350: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC354: 386BAC44  addi r3, r11, -0x53bc
	ctx.r[3].s64 = ctx.r[11].s64 + -21436;
	// 832AC358: 4AF68A80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC360 size=12
    let mut pc: u32 = 0x832AC360;
    'dispatch: loop {
        match pc {
            0x832AC360 => {
    //   block [0x832AC360..0x832AC36C)
	// 832AC360: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC364: 386BAC48  addi r3, r11, -0x53b8
	ctx.r[3].s64 = ctx.r[11].s64 + -21432;
	// 832AC368: 4AF68A70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC370 size=12
    let mut pc: u32 = 0x832AC370;
    'dispatch: loop {
        match pc {
            0x832AC370 => {
    //   block [0x832AC370..0x832AC37C)
	// 832AC370: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC374: 386BACA0  addi r3, r11, -0x5360
	ctx.r[3].s64 = ctx.r[11].s64 + -21344;
	// 832AC378: 4AF68A60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC380 size=12
    let mut pc: u32 = 0x832AC380;
    'dispatch: loop {
        match pc {
            0x832AC380 => {
    //   block [0x832AC380..0x832AC38C)
	// 832AC380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC384: 386BACA4  addi r3, r11, -0x535c
	ctx.r[3].s64 = ctx.r[11].s64 + -21340;
	// 832AC388: 4AF68A50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC390 size=12
    let mut pc: u32 = 0x832AC390;
    'dispatch: loop {
        match pc {
            0x832AC390 => {
    //   block [0x832AC390..0x832AC39C)
	// 832AC390: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC394: 386BACA8  addi r3, r11, -0x5358
	ctx.r[3].s64 = ctx.r[11].s64 + -21336;
	// 832AC398: 4AF68A40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3A0 size=12
    let mut pc: u32 = 0x832AC3A0;
    'dispatch: loop {
        match pc {
            0x832AC3A0 => {
    //   block [0x832AC3A0..0x832AC3AC)
	// 832AC3A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3A4: 386BAD00  addi r3, r11, -0x5300
	ctx.r[3].s64 = ctx.r[11].s64 + -21248;
	// 832AC3A8: 4AF68A30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3B0 size=12
    let mut pc: u32 = 0x832AC3B0;
    'dispatch: loop {
        match pc {
            0x832AC3B0 => {
    //   block [0x832AC3B0..0x832AC3BC)
	// 832AC3B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3B4: 386BAD04  addi r3, r11, -0x52fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21244;
	// 832AC3B8: 4AF68A20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3C0 size=12
    let mut pc: u32 = 0x832AC3C0;
    'dispatch: loop {
        match pc {
            0x832AC3C0 => {
    //   block [0x832AC3C0..0x832AC3CC)
	// 832AC3C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3C4: 386BAD08  addi r3, r11, -0x52f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21240;
	// 832AC3C8: 4AF68A10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3D0 size=12
    let mut pc: u32 = 0x832AC3D0;
    'dispatch: loop {
        match pc {
            0x832AC3D0 => {
    //   block [0x832AC3D0..0x832AC3DC)
	// 832AC3D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3D4: 386BAD0C  addi r3, r11, -0x52f4
	ctx.r[3].s64 = ctx.r[11].s64 + -21236;
	// 832AC3D8: 4AF68A00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3E0 size=12
    let mut pc: u32 = 0x832AC3E0;
    'dispatch: loop {
        match pc {
            0x832AC3E0 => {
    //   block [0x832AC3E0..0x832AC3EC)
	// 832AC3E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3E4: 386BAD10  addi r3, r11, -0x52f0
	ctx.r[3].s64 = ctx.r[11].s64 + -21232;
	// 832AC3E8: 4AF689F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3F0 size=12
    let mut pc: u32 = 0x832AC3F0;
    'dispatch: loop {
        match pc {
            0x832AC3F0 => {
    //   block [0x832AC3F0..0x832AC3FC)
	// 832AC3F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3F4: 386BAD14  addi r3, r11, -0x52ec
	ctx.r[3].s64 = ctx.r[11].s64 + -21228;
	// 832AC3F8: 4AF689E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC400 size=12
    let mut pc: u32 = 0x832AC400;
    'dispatch: loop {
        match pc {
            0x832AC400 => {
    //   block [0x832AC400..0x832AC40C)
	// 832AC400: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC404: 386BAD18  addi r3, r11, -0x52e8
	ctx.r[3].s64 = ctx.r[11].s64 + -21224;
	// 832AC408: 4AF689D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC410 size=12
    let mut pc: u32 = 0x832AC410;
    'dispatch: loop {
        match pc {
            0x832AC410 => {
    //   block [0x832AC410..0x832AC41C)
	// 832AC410: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC414: 386BAD1C  addi r3, r11, -0x52e4
	ctx.r[3].s64 = ctx.r[11].s64 + -21220;
	// 832AC418: 4AF689C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC420 size=12
    let mut pc: u32 = 0x832AC420;
    'dispatch: loop {
        match pc {
            0x832AC420 => {
    //   block [0x832AC420..0x832AC42C)
	// 832AC420: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC424: 386BAD20  addi r3, r11, -0x52e0
	ctx.r[3].s64 = ctx.r[11].s64 + -21216;
	// 832AC428: 4AF689B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC430 size=12
    let mut pc: u32 = 0x832AC430;
    'dispatch: loop {
        match pc {
            0x832AC430 => {
    //   block [0x832AC430..0x832AC43C)
	// 832AC430: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC434: 386BAD24  addi r3, r11, -0x52dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21212;
	// 832AC438: 4AF689A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC440 size=12
    let mut pc: u32 = 0x832AC440;
    'dispatch: loop {
        match pc {
            0x832AC440 => {
    //   block [0x832AC440..0x832AC44C)
	// 832AC440: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC444: 386BAD28  addi r3, r11, -0x52d8
	ctx.r[3].s64 = ctx.r[11].s64 + -21208;
	// 832AC448: 4AF68990  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC450 size=12
    let mut pc: u32 = 0x832AC450;
    'dispatch: loop {
        match pc {
            0x832AC450 => {
    //   block [0x832AC450..0x832AC45C)
	// 832AC450: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC454: 386BAD2C  addi r3, r11, -0x52d4
	ctx.r[3].s64 = ctx.r[11].s64 + -21204;
	// 832AC458: 4AF68980  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC460 size=12
    let mut pc: u32 = 0x832AC460;
    'dispatch: loop {
        match pc {
            0x832AC460 => {
    //   block [0x832AC460..0x832AC46C)
	// 832AC460: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC464: 386BAD30  addi r3, r11, -0x52d0
	ctx.r[3].s64 = ctx.r[11].s64 + -21200;
	// 832AC468: 4AF68970  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC470 size=12
    let mut pc: u32 = 0x832AC470;
    'dispatch: loop {
        match pc {
            0x832AC470 => {
    //   block [0x832AC470..0x832AC47C)
	// 832AC470: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC474: 386BAD34  addi r3, r11, -0x52cc
	ctx.r[3].s64 = ctx.r[11].s64 + -21196;
	// 832AC478: 4AF68960  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC480 size=12
    let mut pc: u32 = 0x832AC480;
    'dispatch: loop {
        match pc {
            0x832AC480 => {
    //   block [0x832AC480..0x832AC48C)
	// 832AC480: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC484: 386BAD38  addi r3, r11, -0x52c8
	ctx.r[3].s64 = ctx.r[11].s64 + -21192;
	// 832AC488: 4AF68950  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC490 size=12
    let mut pc: u32 = 0x832AC490;
    'dispatch: loop {
        match pc {
            0x832AC490 => {
    //   block [0x832AC490..0x832AC49C)
	// 832AC490: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC494: 386BAD3C  addi r3, r11, -0x52c4
	ctx.r[3].s64 = ctx.r[11].s64 + -21188;
	// 832AC498: 4AF68940  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4A0 size=12
    let mut pc: u32 = 0x832AC4A0;
    'dispatch: loop {
        match pc {
            0x832AC4A0 => {
    //   block [0x832AC4A0..0x832AC4AC)
	// 832AC4A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4A4: 386BAD40  addi r3, r11, -0x52c0
	ctx.r[3].s64 = ctx.r[11].s64 + -21184;
	// 832AC4A8: 4AF68930  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4B0 size=12
    let mut pc: u32 = 0x832AC4B0;
    'dispatch: loop {
        match pc {
            0x832AC4B0 => {
    //   block [0x832AC4B0..0x832AC4BC)
	// 832AC4B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4B4: 386BAD44  addi r3, r11, -0x52bc
	ctx.r[3].s64 = ctx.r[11].s64 + -21180;
	// 832AC4B8: 4AF68920  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4C0 size=12
    let mut pc: u32 = 0x832AC4C0;
    'dispatch: loop {
        match pc {
            0x832AC4C0 => {
    //   block [0x832AC4C0..0x832AC4CC)
	// 832AC4C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4C4: 386BAD48  addi r3, r11, -0x52b8
	ctx.r[3].s64 = ctx.r[11].s64 + -21176;
	// 832AC4C8: 4AF68910  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4D0 size=12
    let mut pc: u32 = 0x832AC4D0;
    'dispatch: loop {
        match pc {
            0x832AC4D0 => {
    //   block [0x832AC4D0..0x832AC4DC)
	// 832AC4D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4D4: 386BAD4C  addi r3, r11, -0x52b4
	ctx.r[3].s64 = ctx.r[11].s64 + -21172;
	// 832AC4D8: 4AF68900  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4E0 size=12
    let mut pc: u32 = 0x832AC4E0;
    'dispatch: loop {
        match pc {
            0x832AC4E0 => {
    //   block [0x832AC4E0..0x832AC4EC)
	// 832AC4E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4E4: 386BAD50  addi r3, r11, -0x52b0
	ctx.r[3].s64 = ctx.r[11].s64 + -21168;
	// 832AC4E8: 4AF688F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4F0 size=12
    let mut pc: u32 = 0x832AC4F0;
    'dispatch: loop {
        match pc {
            0x832AC4F0 => {
    //   block [0x832AC4F0..0x832AC4FC)
	// 832AC4F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4F4: 386BAD54  addi r3, r11, -0x52ac
	ctx.r[3].s64 = ctx.r[11].s64 + -21164;
	// 832AC4F8: 4AF688E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC500 size=12
    let mut pc: u32 = 0x832AC500;
    'dispatch: loop {
        match pc {
            0x832AC500 => {
    //   block [0x832AC500..0x832AC50C)
	// 832AC500: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC504: 386BAD58  addi r3, r11, -0x52a8
	ctx.r[3].s64 = ctx.r[11].s64 + -21160;
	// 832AC508: 4AF688D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC510 size=12
    let mut pc: u32 = 0x832AC510;
    'dispatch: loop {
        match pc {
            0x832AC510 => {
    //   block [0x832AC510..0x832AC51C)
	// 832AC510: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC514: 386BAD5C  addi r3, r11, -0x52a4
	ctx.r[3].s64 = ctx.r[11].s64 + -21156;
	// 832AC518: 4AF688C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC520 size=12
    let mut pc: u32 = 0x832AC520;
    'dispatch: loop {
        match pc {
            0x832AC520 => {
    //   block [0x832AC520..0x832AC52C)
	// 832AC520: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC524: 386BAD60  addi r3, r11, -0x52a0
	ctx.r[3].s64 = ctx.r[11].s64 + -21152;
	// 832AC528: 4AF688B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC530 size=12
    let mut pc: u32 = 0x832AC530;
    'dispatch: loop {
        match pc {
            0x832AC530 => {
    //   block [0x832AC530..0x832AC53C)
	// 832AC530: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC534: 386BAD64  addi r3, r11, -0x529c
	ctx.r[3].s64 = ctx.r[11].s64 + -21148;
	// 832AC538: 4AF688A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC540 size=12
    let mut pc: u32 = 0x832AC540;
    'dispatch: loop {
        match pc {
            0x832AC540 => {
    //   block [0x832AC540..0x832AC54C)
	// 832AC540: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC544: 386BAD68  addi r3, r11, -0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + -21144;
	// 832AC548: 4AF68890  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC550 size=12
    let mut pc: u32 = 0x832AC550;
    'dispatch: loop {
        match pc {
            0x832AC550 => {
    //   block [0x832AC550..0x832AC55C)
	// 832AC550: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC554: 386BAD6C  addi r3, r11, -0x5294
	ctx.r[3].s64 = ctx.r[11].s64 + -21140;
	// 832AC558: 4AF68880  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC560 size=12
    let mut pc: u32 = 0x832AC560;
    'dispatch: loop {
        match pc {
            0x832AC560 => {
    //   block [0x832AC560..0x832AC56C)
	// 832AC560: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC564: 386BAD70  addi r3, r11, -0x5290
	ctx.r[3].s64 = ctx.r[11].s64 + -21136;
	// 832AC568: 4AF68870  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC570 size=12
    let mut pc: u32 = 0x832AC570;
    'dispatch: loop {
        match pc {
            0x832AC570 => {
    //   block [0x832AC570..0x832AC57C)
	// 832AC570: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC574: 386BAD74  addi r3, r11, -0x528c
	ctx.r[3].s64 = ctx.r[11].s64 + -21132;
	// 832AC578: 4AF68860  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC580 size=12
    let mut pc: u32 = 0x832AC580;
    'dispatch: loop {
        match pc {
            0x832AC580 => {
    //   block [0x832AC580..0x832AC58C)
	// 832AC580: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC584: 386BAD78  addi r3, r11, -0x5288
	ctx.r[3].s64 = ctx.r[11].s64 + -21128;
	// 832AC588: 4AF68850  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC590 size=12
    let mut pc: u32 = 0x832AC590;
    'dispatch: loop {
        match pc {
            0x832AC590 => {
    //   block [0x832AC590..0x832AC59C)
	// 832AC590: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC594: 386BAD7C  addi r3, r11, -0x5284
	ctx.r[3].s64 = ctx.r[11].s64 + -21124;
	// 832AC598: 4AF68840  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5A0 size=12
    let mut pc: u32 = 0x832AC5A0;
    'dispatch: loop {
        match pc {
            0x832AC5A0 => {
    //   block [0x832AC5A0..0x832AC5AC)
	// 832AC5A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5A4: 386BAD80  addi r3, r11, -0x5280
	ctx.r[3].s64 = ctx.r[11].s64 + -21120;
	// 832AC5A8: 4AF68830  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5B0 size=12
    let mut pc: u32 = 0x832AC5B0;
    'dispatch: loop {
        match pc {
            0x832AC5B0 => {
    //   block [0x832AC5B0..0x832AC5BC)
	// 832AC5B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5B4: 386BADD8  addi r3, r11, -0x5228
	ctx.r[3].s64 = ctx.r[11].s64 + -21032;
	// 832AC5B8: 4AF68820  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5C0 size=12
    let mut pc: u32 = 0x832AC5C0;
    'dispatch: loop {
        match pc {
            0x832AC5C0 => {
    //   block [0x832AC5C0..0x832AC5CC)
	// 832AC5C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5C4: 386BADDC  addi r3, r11, -0x5224
	ctx.r[3].s64 = ctx.r[11].s64 + -21028;
	// 832AC5C8: 4AF68810  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5D0 size=12
    let mut pc: u32 = 0x832AC5D0;
    'dispatch: loop {
        match pc {
            0x832AC5D0 => {
    //   block [0x832AC5D0..0x832AC5DC)
	// 832AC5D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5D4: 386BADE0  addi r3, r11, -0x5220
	ctx.r[3].s64 = ctx.r[11].s64 + -21024;
	// 832AC5D8: 4AF68800  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5E0 size=12
    let mut pc: u32 = 0x832AC5E0;
    'dispatch: loop {
        match pc {
            0x832AC5E0 => {
    //   block [0x832AC5E0..0x832AC5EC)
	// 832AC5E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5E4: 386BAE38  addi r3, r11, -0x51c8
	ctx.r[3].s64 = ctx.r[11].s64 + -20936;
	// 832AC5E8: 4AF687F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5F0 size=12
    let mut pc: u32 = 0x832AC5F0;
    'dispatch: loop {
        match pc {
            0x832AC5F0 => {
    //   block [0x832AC5F0..0x832AC5FC)
	// 832AC5F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5F4: 386BAE3C  addi r3, r11, -0x51c4
	ctx.r[3].s64 = ctx.r[11].s64 + -20932;
	// 832AC5F8: 4AF687E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC600 size=12
    let mut pc: u32 = 0x832AC600;
    'dispatch: loop {
        match pc {
            0x832AC600 => {
    //   block [0x832AC600..0x832AC60C)
	// 832AC600: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC604: 386BAE9C  addi r3, r11, -0x5164
	ctx.r[3].s64 = ctx.r[11].s64 + -20836;
	// 832AC608: 4AF687D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC610 size=12
    let mut pc: u32 = 0x832AC610;
    'dispatch: loop {
        match pc {
            0x832AC610 => {
    //   block [0x832AC610..0x832AC61C)
	// 832AC610: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC614: 386BAEA0  addi r3, r11, -0x5160
	ctx.r[3].s64 = ctx.r[11].s64 + -20832;
	// 832AC618: 4AF687C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC620 size=12
    let mut pc: u32 = 0x832AC620;
    'dispatch: loop {
        match pc {
            0x832AC620 => {
    //   block [0x832AC620..0x832AC62C)
	// 832AC620: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC624: 386BAEF8  addi r3, r11, -0x5108
	ctx.r[3].s64 = ctx.r[11].s64 + -20744;
	// 832AC628: 4AF687B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC630 size=12
    let mut pc: u32 = 0x832AC630;
    'dispatch: loop {
        match pc {
            0x832AC630 => {
    //   block [0x832AC630..0x832AC63C)
	// 832AC630: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC634: 386BAEFC  addi r3, r11, -0x5104
	ctx.r[3].s64 = ctx.r[11].s64 + -20740;
	// 832AC638: 4AF687A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC640 size=12
    let mut pc: u32 = 0x832AC640;
    'dispatch: loop {
        match pc {
            0x832AC640 => {
    //   block [0x832AC640..0x832AC64C)
	// 832AC640: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC644: 386BAF00  addi r3, r11, -0x5100
	ctx.r[3].s64 = ctx.r[11].s64 + -20736;
	// 832AC648: 4AF68790  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC650 size=12
    let mut pc: u32 = 0x832AC650;
    'dispatch: loop {
        match pc {
            0x832AC650 => {
    //   block [0x832AC650..0x832AC65C)
	// 832AC650: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC654: 386BAF04  addi r3, r11, -0x50fc
	ctx.r[3].s64 = ctx.r[11].s64 + -20732;
	// 832AC658: 4AF68780  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC660 size=12
    let mut pc: u32 = 0x832AC660;
    'dispatch: loop {
        match pc {
            0x832AC660 => {
    //   block [0x832AC660..0x832AC66C)
	// 832AC660: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC664: 386BAF08  addi r3, r11, -0x50f8
	ctx.r[3].s64 = ctx.r[11].s64 + -20728;
	// 832AC668: 4AF68770  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC670 size=12
    let mut pc: u32 = 0x832AC670;
    'dispatch: loop {
        match pc {
            0x832AC670 => {
    //   block [0x832AC670..0x832AC67C)
	// 832AC670: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC674: 386BAF60  addi r3, r11, -0x50a0
	ctx.r[3].s64 = ctx.r[11].s64 + -20640;
	// 832AC678: 4AF68760  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC680 size=12
    let mut pc: u32 = 0x832AC680;
    'dispatch: loop {
        match pc {
            0x832AC680 => {
    //   block [0x832AC680..0x832AC68C)
	// 832AC680: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC684: 386BAF64  addi r3, r11, -0x509c
	ctx.r[3].s64 = ctx.r[11].s64 + -20636;
	// 832AC688: 4AF68750  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC690 size=12
    let mut pc: u32 = 0x832AC690;
    'dispatch: loop {
        match pc {
            0x832AC690 => {
    //   block [0x832AC690..0x832AC69C)
	// 832AC690: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC694: 386BAF68  addi r3, r11, -0x5098
	ctx.r[3].s64 = ctx.r[11].s64 + -20632;
	// 832AC698: 4AF1A160  b 0x821c67f8
	sub_821C67F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6A0 size=12
    let mut pc: u32 = 0x832AC6A0;
    'dispatch: loop {
        match pc {
            0x832AC6A0 => {
    //   block [0x832AC6A0..0x832AC6AC)
	// 832AC6A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6A4: 386BAF6C  addi r3, r11, -0x5094
	ctx.r[3].s64 = ctx.r[11].s64 + -20628;
	// 832AC6A8: 4AF68730  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6B0 size=12
    let mut pc: u32 = 0x832AC6B0;
    'dispatch: loop {
        match pc {
            0x832AC6B0 => {
    //   block [0x832AC6B0..0x832AC6BC)
	// 832AC6B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6B4: 386BAF70  addi r3, r11, -0x5090
	ctx.r[3].s64 = ctx.r[11].s64 + -20624;
	// 832AC6B8: 4AF68720  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6C0 size=12
    let mut pc: u32 = 0x832AC6C0;
    'dispatch: loop {
        match pc {
            0x832AC6C0 => {
    //   block [0x832AC6C0..0x832AC6CC)
	// 832AC6C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6C4: 386BAF74  addi r3, r11, -0x508c
	ctx.r[3].s64 = ctx.r[11].s64 + -20620;
	// 832AC6C8: 4AF68710  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6D0 size=12
    let mut pc: u32 = 0x832AC6D0;
    'dispatch: loop {
        match pc {
            0x832AC6D0 => {
    //   block [0x832AC6D0..0x832AC6DC)
	// 832AC6D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6D4: 386BAF78  addi r3, r11, -0x5088
	ctx.r[3].s64 = ctx.r[11].s64 + -20616;
	// 832AC6D8: 4AF68700  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6E0 size=12
    let mut pc: u32 = 0x832AC6E0;
    'dispatch: loop {
        match pc {
            0x832AC6E0 => {
    //   block [0x832AC6E0..0x832AC6EC)
	// 832AC6E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6E4: 386BAF7C  addi r3, r11, -0x5084
	ctx.r[3].s64 = ctx.r[11].s64 + -20612;
	// 832AC6E8: 4AF686F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6F0 size=12
    let mut pc: u32 = 0x832AC6F0;
    'dispatch: loop {
        match pc {
            0x832AC6F0 => {
    //   block [0x832AC6F0..0x832AC6FC)
	// 832AC6F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6F4: 386BAF80  addi r3, r11, -0x5080
	ctx.r[3].s64 = ctx.r[11].s64 + -20608;
	// 832AC6F8: 4AF686E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC700 size=12
    let mut pc: u32 = 0x832AC700;
    'dispatch: loop {
        match pc {
            0x832AC700 => {
    //   block [0x832AC700..0x832AC70C)
	// 832AC700: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC704: 386BAF84  addi r3, r11, -0x507c
	ctx.r[3].s64 = ctx.r[11].s64 + -20604;
	// 832AC708: 4AF686D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC710 size=12
    let mut pc: u32 = 0x832AC710;
    'dispatch: loop {
        match pc {
            0x832AC710 => {
    //   block [0x832AC710..0x832AC71C)
	// 832AC710: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC714: 386BAF88  addi r3, r11, -0x5078
	ctx.r[3].s64 = ctx.r[11].s64 + -20600;
	// 832AC718: 4AF686C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC720 size=12
    let mut pc: u32 = 0x832AC720;
    'dispatch: loop {
        match pc {
            0x832AC720 => {
    //   block [0x832AC720..0x832AC72C)
	// 832AC720: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC724: 386BAF8C  addi r3, r11, -0x5074
	ctx.r[3].s64 = ctx.r[11].s64 + -20596;
	// 832AC728: 4AF686B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC730 size=12
    let mut pc: u32 = 0x832AC730;
    'dispatch: loop {
        match pc {
            0x832AC730 => {
    //   block [0x832AC730..0x832AC73C)
	// 832AC730: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC734: 386BAFF4  addi r3, r11, -0x500c
	ctx.r[3].s64 = ctx.r[11].s64 + -20492;
	// 832AC738: 4AF686A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC740 size=12
    let mut pc: u32 = 0x832AC740;
    'dispatch: loop {
        match pc {
            0x832AC740 => {
    //   block [0x832AC740..0x832AC74C)
	// 832AC740: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC744: 386BAFF8  addi r3, r11, -0x5008
	ctx.r[3].s64 = ctx.r[11].s64 + -20488;
	// 832AC748: 4AF68690  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC750 size=12
    let mut pc: u32 = 0x832AC750;
    'dispatch: loop {
        match pc {
            0x832AC750 => {
    //   block [0x832AC750..0x832AC75C)
	// 832AC750: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC754: 386BAFFC  addi r3, r11, -0x5004
	ctx.r[3].s64 = ctx.r[11].s64 + -20484;
	// 832AC758: 4AF68680  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC760 size=12
    let mut pc: u32 = 0x832AC760;
    'dispatch: loop {
        match pc {
            0x832AC760 => {
    //   block [0x832AC760..0x832AC76C)
	// 832AC760: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC764: 386BB000  addi r3, r11, -0x5000
	ctx.r[3].s64 = ctx.r[11].s64 + -20480;
	// 832AC768: 4AF68670  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC770 size=12
    let mut pc: u32 = 0x832AC770;
    'dispatch: loop {
        match pc {
            0x832AC770 => {
    //   block [0x832AC770..0x832AC77C)
	// 832AC770: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC774: 386BB004  addi r3, r11, -0x4ffc
	ctx.r[3].s64 = ctx.r[11].s64 + -20476;
	// 832AC778: 4AF68660  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC780 size=12
    let mut pc: u32 = 0x832AC780;
    'dispatch: loop {
        match pc {
            0x832AC780 => {
    //   block [0x832AC780..0x832AC78C)
	// 832AC780: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC784: 386BB008  addi r3, r11, -0x4ff8
	ctx.r[3].s64 = ctx.r[11].s64 + -20472;
	// 832AC788: 4AF68650  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC790 size=12
    let mut pc: u32 = 0x832AC790;
    'dispatch: loop {
        match pc {
            0x832AC790 => {
    //   block [0x832AC790..0x832AC79C)
	// 832AC790: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC794: 386BB00C  addi r3, r11, -0x4ff4
	ctx.r[3].s64 = ctx.r[11].s64 + -20468;
	// 832AC798: 4AF68640  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7A0 size=12
    let mut pc: u32 = 0x832AC7A0;
    'dispatch: loop {
        match pc {
            0x832AC7A0 => {
    //   block [0x832AC7A0..0x832AC7AC)
	// 832AC7A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7A4: 386BB010  addi r3, r11, -0x4ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -20464;
	// 832AC7A8: 4AF68630  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7B0 size=12
    let mut pc: u32 = 0x832AC7B0;
    'dispatch: loop {
        match pc {
            0x832AC7B0 => {
    //   block [0x832AC7B0..0x832AC7BC)
	// 832AC7B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7B4: 386BB014  addi r3, r11, -0x4fec
	ctx.r[3].s64 = ctx.r[11].s64 + -20460;
	// 832AC7B8: 4AF68620  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7C0 size=12
    let mut pc: u32 = 0x832AC7C0;
    'dispatch: loop {
        match pc {
            0x832AC7C0 => {
    //   block [0x832AC7C0..0x832AC7CC)
	// 832AC7C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7C4: 386BB018  addi r3, r11, -0x4fe8
	ctx.r[3].s64 = ctx.r[11].s64 + -20456;
	// 832AC7C8: 4AF68610  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7D0 size=12
    let mut pc: u32 = 0x832AC7D0;
    'dispatch: loop {
        match pc {
            0x832AC7D0 => {
    //   block [0x832AC7D0..0x832AC7DC)
	// 832AC7D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7D4: 386BB01C  addi r3, r11, -0x4fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -20452;
	// 832AC7D8: 4AF68600  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7E0 size=12
    let mut pc: u32 = 0x832AC7E0;
    'dispatch: loop {
        match pc {
            0x832AC7E0 => {
    //   block [0x832AC7E0..0x832AC7EC)
	// 832AC7E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7E4: 386BB020  addi r3, r11, -0x4fe0
	ctx.r[3].s64 = ctx.r[11].s64 + -20448;
	// 832AC7E8: 4AF685F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7F0 size=12
    let mut pc: u32 = 0x832AC7F0;
    'dispatch: loop {
        match pc {
            0x832AC7F0 => {
    //   block [0x832AC7F0..0x832AC7FC)
	// 832AC7F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7F4: 386BB024  addi r3, r11, -0x4fdc
	ctx.r[3].s64 = ctx.r[11].s64 + -20444;
	// 832AC7F8: 4AF685E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC800 size=12
    let mut pc: u32 = 0x832AC800;
    'dispatch: loop {
        match pc {
            0x832AC800 => {
    //   block [0x832AC800..0x832AC80C)
	// 832AC800: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC804: 386BB028  addi r3, r11, -0x4fd8
	ctx.r[3].s64 = ctx.r[11].s64 + -20440;
	// 832AC808: 4AF685D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC810 size=12
    let mut pc: u32 = 0x832AC810;
    'dispatch: loop {
        match pc {
            0x832AC810 => {
    //   block [0x832AC810..0x832AC81C)
	// 832AC810: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC814: 386BB02C  addi r3, r11, -0x4fd4
	ctx.r[3].s64 = ctx.r[11].s64 + -20436;
	// 832AC818: 4AF685C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC820 size=12
    let mut pc: u32 = 0x832AC820;
    'dispatch: loop {
        match pc {
            0x832AC820 => {
    //   block [0x832AC820..0x832AC82C)
	// 832AC820: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC824: 386BB030  addi r3, r11, -0x4fd0
	ctx.r[3].s64 = ctx.r[11].s64 + -20432;
	// 832AC828: 4AF685B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC830 size=12
    let mut pc: u32 = 0x832AC830;
    'dispatch: loop {
        match pc {
            0x832AC830 => {
    //   block [0x832AC830..0x832AC83C)
	// 832AC830: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC834: 386BB088  addi r3, r11, -0x4f78
	ctx.r[3].s64 = ctx.r[11].s64 + -20344;
	// 832AC838: 4AF685A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC840 size=12
    let mut pc: u32 = 0x832AC840;
    'dispatch: loop {
        match pc {
            0x832AC840 => {
    //   block [0x832AC840..0x832AC84C)
	// 832AC840: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC844: 386BB0E0  addi r3, r11, -0x4f20
	ctx.r[3].s64 = ctx.r[11].s64 + -20256;
	// 832AC848: 4AF68590  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC850 size=12
    let mut pc: u32 = 0x832AC850;
    'dispatch: loop {
        match pc {
            0x832AC850 => {
    //   block [0x832AC850..0x832AC85C)
	// 832AC850: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC854: 386BB0E4  addi r3, r11, -0x4f1c
	ctx.r[3].s64 = ctx.r[11].s64 + -20252;
	// 832AC858: 4AF68580  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC860 size=12
    let mut pc: u32 = 0x832AC860;
    'dispatch: loop {
        match pc {
            0x832AC860 => {
    //   block [0x832AC860..0x832AC86C)
	// 832AC860: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC864: 386BB0E8  addi r3, r11, -0x4f18
	ctx.r[3].s64 = ctx.r[11].s64 + -20248;
	// 832AC868: 4AF68570  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC870 size=12
    let mut pc: u32 = 0x832AC870;
    'dispatch: loop {
        match pc {
            0x832AC870 => {
    //   block [0x832AC870..0x832AC87C)
	// 832AC870: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC874: 386BB0EC  addi r3, r11, -0x4f14
	ctx.r[3].s64 = ctx.r[11].s64 + -20244;
	// 832AC878: 4AF68560  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC880 size=12
    let mut pc: u32 = 0x832AC880;
    'dispatch: loop {
        match pc {
            0x832AC880 => {
    //   block [0x832AC880..0x832AC88C)
	// 832AC880: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC884: 386BB0F0  addi r3, r11, -0x4f10
	ctx.r[3].s64 = ctx.r[11].s64 + -20240;
	// 832AC888: 4AF68550  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC890 size=12
    let mut pc: u32 = 0x832AC890;
    'dispatch: loop {
        match pc {
            0x832AC890 => {
    //   block [0x832AC890..0x832AC89C)
	// 832AC890: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC894: 386BB0F4  addi r3, r11, -0x4f0c
	ctx.r[3].s64 = ctx.r[11].s64 + -20236;
	// 832AC898: 4AF68540  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC8A0 size=12
    let mut pc: u32 = 0x832AC8A0;
    'dispatch: loop {
        match pc {
            0x832AC8A0 => {
    //   block [0x832AC8A0..0x832AC8AC)
	// 832AC8A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC8A4: 386BB0F8  addi r3, r11, -0x4f08
	ctx.r[3].s64 = ctx.r[11].s64 + -20232;
	// 832AC8A8: 4AF68530  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC8B0 size=12
    let mut pc: u32 = 0x832AC8B0;
    'dispatch: loop {
        match pc {
            0x832AC8B0 => {
    //   block [0x832AC8B0..0x832AC8BC)
	// 832AC8B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC8B4: 386BB0FC  addi r3, r11, -0x4f04
	ctx.r[3].s64 = ctx.r[11].s64 + -20228;
	// 832AC8B8: 4AF68520  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC8C0 size=12
    let mut pc: u32 = 0x832AC8C0;
    'dispatch: loop {
        match pc {
            0x832AC8C0 => {
    //   block [0x832AC8C0..0x832AC8CC)
	// 832AC8C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC8C4: 386BB100  addi r3, r11, -0x4f00
	ctx.r[3].s64 = ctx.r[11].s64 + -20224;
	// 832AC8C8: 4AF68510  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC8D0 size=12
    let mut pc: u32 = 0x832AC8D0;
    'dispatch: loop {
        match pc {
            0x832AC8D0 => {
    //   block [0x832AC8D0..0x832AC8DC)
	// 832AC8D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC8D4: 386BB104  addi r3, r11, -0x4efc
	ctx.r[3].s64 = ctx.r[11].s64 + -20220;
	// 832AC8D8: 4AF68500  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC8E0 size=12
    let mut pc: u32 = 0x832AC8E0;
    'dispatch: loop {
        match pc {
            0x832AC8E0 => {
    //   block [0x832AC8E0..0x832AC8EC)
	// 832AC8E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC8E4: 386BB150  addi r3, r11, -0x4eb0
	ctx.r[3].s64 = ctx.r[11].s64 + -20144;
	// 832AC8E8: 4AF684F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC8F0 size=12
    let mut pc: u32 = 0x832AC8F0;
    'dispatch: loop {
        match pc {
            0x832AC8F0 => {
    //   block [0x832AC8F0..0x832AC8FC)
	// 832AC8F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC8F4: 386BB154  addi r3, r11, -0x4eac
	ctx.r[3].s64 = ctx.r[11].s64 + -20140;
	// 832AC8F8: 4AF684E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC900 size=12
    let mut pc: u32 = 0x832AC900;
    'dispatch: loop {
        match pc {
            0x832AC900 => {
    //   block [0x832AC900..0x832AC90C)
	// 832AC900: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC904: 386BB158  addi r3, r11, -0x4ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -20136;
	// 832AC908: 4AF684D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC910 size=12
    let mut pc: u32 = 0x832AC910;
    'dispatch: loop {
        match pc {
            0x832AC910 => {
    //   block [0x832AC910..0x832AC91C)
	// 832AC910: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC914: 386BB15C  addi r3, r11, -0x4ea4
	ctx.r[3].s64 = ctx.r[11].s64 + -20132;
	// 832AC918: 4AF684C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC920 size=12
    let mut pc: u32 = 0x832AC920;
    'dispatch: loop {
        match pc {
            0x832AC920 => {
    //   block [0x832AC920..0x832AC92C)
	// 832AC920: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC924: 386BB160  addi r3, r11, -0x4ea0
	ctx.r[3].s64 = ctx.r[11].s64 + -20128;
	// 832AC928: 4AF684B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC930 size=12
    let mut pc: u32 = 0x832AC930;
    'dispatch: loop {
        match pc {
            0x832AC930 => {
    //   block [0x832AC930..0x832AC93C)
	// 832AC930: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC934: 386BB164  addi r3, r11, -0x4e9c
	ctx.r[3].s64 = ctx.r[11].s64 + -20124;
	// 832AC938: 4AF684A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC940 size=12
    let mut pc: u32 = 0x832AC940;
    'dispatch: loop {
        match pc {
            0x832AC940 => {
    //   block [0x832AC940..0x832AC94C)
	// 832AC940: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC944: 386BB168  addi r3, r11, -0x4e98
	ctx.r[3].s64 = ctx.r[11].s64 + -20120;
	// 832AC948: 4AF68490  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC950 size=12
    let mut pc: u32 = 0x832AC950;
    'dispatch: loop {
        match pc {
            0x832AC950 => {
    //   block [0x832AC950..0x832AC95C)
	// 832AC950: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC954: 386BB16C  addi r3, r11, -0x4e94
	ctx.r[3].s64 = ctx.r[11].s64 + -20116;
	// 832AC958: 4AF68480  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC960 size=12
    let mut pc: u32 = 0x832AC960;
    'dispatch: loop {
        match pc {
            0x832AC960 => {
    //   block [0x832AC960..0x832AC96C)
	// 832AC960: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC964: 386BB170  addi r3, r11, -0x4e90
	ctx.r[3].s64 = ctx.r[11].s64 + -20112;
	// 832AC968: 4AF68470  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC970 size=12
    let mut pc: u32 = 0x832AC970;
    'dispatch: loop {
        match pc {
            0x832AC970 => {
    //   block [0x832AC970..0x832AC97C)
	// 832AC970: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC974: 386BB174  addi r3, r11, -0x4e8c
	ctx.r[3].s64 = ctx.r[11].s64 + -20108;
	// 832AC978: 4AF68460  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC980 size=12
    let mut pc: u32 = 0x832AC980;
    'dispatch: loop {
        match pc {
            0x832AC980 => {
    //   block [0x832AC980..0x832AC98C)
	// 832AC980: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC984: 386BB178  addi r3, r11, -0x4e88
	ctx.r[3].s64 = ctx.r[11].s64 + -20104;
	// 832AC988: 4AF68450  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC990 size=12
    let mut pc: u32 = 0x832AC990;
    'dispatch: loop {
        match pc {
            0x832AC990 => {
    //   block [0x832AC990..0x832AC99C)
	// 832AC990: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC994: 386BB17C  addi r3, r11, -0x4e84
	ctx.r[3].s64 = ctx.r[11].s64 + -20100;
	// 832AC998: 4AF68440  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC9A0 size=12
    let mut pc: u32 = 0x832AC9A0;
    'dispatch: loop {
        match pc {
            0x832AC9A0 => {
    //   block [0x832AC9A0..0x832AC9AC)
	// 832AC9A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC9A4: 386BB180  addi r3, r11, -0x4e80
	ctx.r[3].s64 = ctx.r[11].s64 + -20096;
	// 832AC9A8: 4AF68430  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC9B0 size=12
    let mut pc: u32 = 0x832AC9B0;
    'dispatch: loop {
        match pc {
            0x832AC9B0 => {
    //   block [0x832AC9B0..0x832AC9BC)
	// 832AC9B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC9B4: 386BB184  addi r3, r11, -0x4e7c
	ctx.r[3].s64 = ctx.r[11].s64 + -20092;
	// 832AC9B8: 4AF68420  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC9C0 size=12
    let mut pc: u32 = 0x832AC9C0;
    'dispatch: loop {
        match pc {
            0x832AC9C0 => {
    //   block [0x832AC9C0..0x832AC9CC)
	// 832AC9C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC9C4: 386BB188  addi r3, r11, -0x4e78
	ctx.r[3].s64 = ctx.r[11].s64 + -20088;
	// 832AC9C8: 4AF68410  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC9D0 size=12
    let mut pc: u32 = 0x832AC9D0;
    'dispatch: loop {
        match pc {
            0x832AC9D0 => {
    //   block [0x832AC9D0..0x832AC9DC)
	// 832AC9D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC9D4: 386BB18C  addi r3, r11, -0x4e74
	ctx.r[3].s64 = ctx.r[11].s64 + -20084;
	// 832AC9D8: 4AF68400  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC9E0 size=12
    let mut pc: u32 = 0x832AC9E0;
    'dispatch: loop {
        match pc {
            0x832AC9E0 => {
    //   block [0x832AC9E0..0x832AC9EC)
	// 832AC9E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC9E4: 386BB190  addi r3, r11, -0x4e70
	ctx.r[3].s64 = ctx.r[11].s64 + -20080;
	// 832AC9E8: 4AF683F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC9F0 size=12
    let mut pc: u32 = 0x832AC9F0;
    'dispatch: loop {
        match pc {
            0x832AC9F0 => {
    //   block [0x832AC9F0..0x832AC9FC)
	// 832AC9F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC9F4: 386BB194  addi r3, r11, -0x4e6c
	ctx.r[3].s64 = ctx.r[11].s64 + -20076;
	// 832AC9F8: 4AF683E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA00 size=12
    let mut pc: u32 = 0x832ACA00;
    'dispatch: loop {
        match pc {
            0x832ACA00 => {
    //   block [0x832ACA00..0x832ACA0C)
	// 832ACA00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA04: 386BB198  addi r3, r11, -0x4e68
	ctx.r[3].s64 = ctx.r[11].s64 + -20072;
	// 832ACA08: 4AF683D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA10 size=12
    let mut pc: u32 = 0x832ACA10;
    'dispatch: loop {
        match pc {
            0x832ACA10 => {
    //   block [0x832ACA10..0x832ACA1C)
	// 832ACA10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA14: 386BB19C  addi r3, r11, -0x4e64
	ctx.r[3].s64 = ctx.r[11].s64 + -20068;
	// 832ACA18: 4AF683C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA20 size=12
    let mut pc: u32 = 0x832ACA20;
    'dispatch: loop {
        match pc {
            0x832ACA20 => {
    //   block [0x832ACA20..0x832ACA2C)
	// 832ACA20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA24: 386BB1A0  addi r3, r11, -0x4e60
	ctx.r[3].s64 = ctx.r[11].s64 + -20064;
	// 832ACA28: 4AF683B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA30 size=12
    let mut pc: u32 = 0x832ACA30;
    'dispatch: loop {
        match pc {
            0x832ACA30 => {
    //   block [0x832ACA30..0x832ACA3C)
	// 832ACA30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA34: 386BB1A4  addi r3, r11, -0x4e5c
	ctx.r[3].s64 = ctx.r[11].s64 + -20060;
	// 832ACA38: 4AF683A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA40 size=12
    let mut pc: u32 = 0x832ACA40;
    'dispatch: loop {
        match pc {
            0x832ACA40 => {
    //   block [0x832ACA40..0x832ACA4C)
	// 832ACA40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA44: 386BB1A8  addi r3, r11, -0x4e58
	ctx.r[3].s64 = ctx.r[11].s64 + -20056;
	// 832ACA48: 4AF68390  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA50 size=12
    let mut pc: u32 = 0x832ACA50;
    'dispatch: loop {
        match pc {
            0x832ACA50 => {
    //   block [0x832ACA50..0x832ACA5C)
	// 832ACA50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA54: 386BB1AC  addi r3, r11, -0x4e54
	ctx.r[3].s64 = ctx.r[11].s64 + -20052;
	// 832ACA58: 4AF68380  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA60 size=12
    let mut pc: u32 = 0x832ACA60;
    'dispatch: loop {
        match pc {
            0x832ACA60 => {
    //   block [0x832ACA60..0x832ACA6C)
	// 832ACA60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA64: 386BB1B0  addi r3, r11, -0x4e50
	ctx.r[3].s64 = ctx.r[11].s64 + -20048;
	// 832ACA68: 4AF68370  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA70 size=12
    let mut pc: u32 = 0x832ACA70;
    'dispatch: loop {
        match pc {
            0x832ACA70 => {
    //   block [0x832ACA70..0x832ACA7C)
	// 832ACA70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA74: 386BB1B4  addi r3, r11, -0x4e4c
	ctx.r[3].s64 = ctx.r[11].s64 + -20044;
	// 832ACA78: 4AF68360  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA80 size=12
    let mut pc: u32 = 0x832ACA80;
    'dispatch: loop {
        match pc {
            0x832ACA80 => {
    //   block [0x832ACA80..0x832ACA8C)
	// 832ACA80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA84: 386BB1B8  addi r3, r11, -0x4e48
	ctx.r[3].s64 = ctx.r[11].s64 + -20040;
	// 832ACA88: 4AF68350  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACA90 size=12
    let mut pc: u32 = 0x832ACA90;
    'dispatch: loop {
        match pc {
            0x832ACA90 => {
    //   block [0x832ACA90..0x832ACA9C)
	// 832ACA90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACA94: 386BB1BC  addi r3, r11, -0x4e44
	ctx.r[3].s64 = ctx.r[11].s64 + -20036;
	// 832ACA98: 4AF68340  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACAA0 size=12
    let mut pc: u32 = 0x832ACAA0;
    'dispatch: loop {
        match pc {
            0x832ACAA0 => {
    //   block [0x832ACAA0..0x832ACAAC)
	// 832ACAA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACAA4: 386BB1C0  addi r3, r11, -0x4e40
	ctx.r[3].s64 = ctx.r[11].s64 + -20032;
	// 832ACAA8: 4AF68330  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACAB0 size=12
    let mut pc: u32 = 0x832ACAB0;
    'dispatch: loop {
        match pc {
            0x832ACAB0 => {
    //   block [0x832ACAB0..0x832ACABC)
	// 832ACAB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACAB4: 386BB1C4  addi r3, r11, -0x4e3c
	ctx.r[3].s64 = ctx.r[11].s64 + -20028;
	// 832ACAB8: 4AF68320  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACAC0 size=12
    let mut pc: u32 = 0x832ACAC0;
    'dispatch: loop {
        match pc {
            0x832ACAC0 => {
    //   block [0x832ACAC0..0x832ACACC)
	// 832ACAC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACAC4: 386BB21C  addi r3, r11, -0x4de4
	ctx.r[3].s64 = ctx.r[11].s64 + -19940;
	// 832ACAC8: 4AF68310  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACAD0 size=12
    let mut pc: u32 = 0x832ACAD0;
    'dispatch: loop {
        match pc {
            0x832ACAD0 => {
    //   block [0x832ACAD0..0x832ACADC)
	// 832ACAD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACAD4: 386BB220  addi r3, r11, -0x4de0
	ctx.r[3].s64 = ctx.r[11].s64 + -19936;
	// 832ACAD8: 4AF68300  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACAE0 size=12
    let mut pc: u32 = 0x832ACAE0;
    'dispatch: loop {
        match pc {
            0x832ACAE0 => {
    //   block [0x832ACAE0..0x832ACAEC)
	// 832ACAE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACAE4: 386BB224  addi r3, r11, -0x4ddc
	ctx.r[3].s64 = ctx.r[11].s64 + -19932;
	// 832ACAE8: 4AF682F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACAF0 size=12
    let mut pc: u32 = 0x832ACAF0;
    'dispatch: loop {
        match pc {
            0x832ACAF0 => {
    //   block [0x832ACAF0..0x832ACAFC)
	// 832ACAF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACAF4: 386BB228  addi r3, r11, -0x4dd8
	ctx.r[3].s64 = ctx.r[11].s64 + -19928;
	// 832ACAF8: 4AF682E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB00 size=12
    let mut pc: u32 = 0x832ACB00;
    'dispatch: loop {
        match pc {
            0x832ACB00 => {
    //   block [0x832ACB00..0x832ACB0C)
	// 832ACB00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB04: 386BB22C  addi r3, r11, -0x4dd4
	ctx.r[3].s64 = ctx.r[11].s64 + -19924;
	// 832ACB08: 4AF682D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB10 size=12
    let mut pc: u32 = 0x832ACB10;
    'dispatch: loop {
        match pc {
            0x832ACB10 => {
    //   block [0x832ACB10..0x832ACB1C)
	// 832ACB10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB14: 386BB230  addi r3, r11, -0x4dd0
	ctx.r[3].s64 = ctx.r[11].s64 + -19920;
	// 832ACB18: 4AF682C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB20 size=12
    let mut pc: u32 = 0x832ACB20;
    'dispatch: loop {
        match pc {
            0x832ACB20 => {
    //   block [0x832ACB20..0x832ACB2C)
	// 832ACB20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB24: 386BB288  addi r3, r11, -0x4d78
	ctx.r[3].s64 = ctx.r[11].s64 + -19832;
	// 832ACB28: 4AF682B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB30 size=12
    let mut pc: u32 = 0x832ACB30;
    'dispatch: loop {
        match pc {
            0x832ACB30 => {
    //   block [0x832ACB30..0x832ACB3C)
	// 832ACB30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB34: 386BB28C  addi r3, r11, -0x4d74
	ctx.r[3].s64 = ctx.r[11].s64 + -19828;
	// 832ACB38: 4AF682A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB40 size=12
    let mut pc: u32 = 0x832ACB40;
    'dispatch: loop {
        match pc {
            0x832ACB40 => {
    //   block [0x832ACB40..0x832ACB4C)
	// 832ACB40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB44: 386BB290  addi r3, r11, -0x4d70
	ctx.r[3].s64 = ctx.r[11].s64 + -19824;
	// 832ACB48: 4AF68290  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB50 size=12
    let mut pc: u32 = 0x832ACB50;
    'dispatch: loop {
        match pc {
            0x832ACB50 => {
    //   block [0x832ACB50..0x832ACB5C)
	// 832ACB50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB54: 386BB294  addi r3, r11, -0x4d6c
	ctx.r[3].s64 = ctx.r[11].s64 + -19820;
	// 832ACB58: 4AF68280  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB60 size=12
    let mut pc: u32 = 0x832ACB60;
    'dispatch: loop {
        match pc {
            0x832ACB60 => {
    //   block [0x832ACB60..0x832ACB6C)
	// 832ACB60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB64: 386BB298  addi r3, r11, -0x4d68
	ctx.r[3].s64 = ctx.r[11].s64 + -19816;
	// 832ACB68: 4AF68270  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB70 size=12
    let mut pc: u32 = 0x832ACB70;
    'dispatch: loop {
        match pc {
            0x832ACB70 => {
    //   block [0x832ACB70..0x832ACB7C)
	// 832ACB70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB74: 386BB29C  addi r3, r11, -0x4d64
	ctx.r[3].s64 = ctx.r[11].s64 + -19812;
	// 832ACB78: 4AF68260  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB80 size=12
    let mut pc: u32 = 0x832ACB80;
    'dispatch: loop {
        match pc {
            0x832ACB80 => {
    //   block [0x832ACB80..0x832ACB8C)
	// 832ACB80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB84: 386BB2A0  addi r3, r11, -0x4d60
	ctx.r[3].s64 = ctx.r[11].s64 + -19808;
	// 832ACB88: 4AF68250  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACB90 size=12
    let mut pc: u32 = 0x832ACB90;
    'dispatch: loop {
        match pc {
            0x832ACB90 => {
    //   block [0x832ACB90..0x832ACB9C)
	// 832ACB90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACB94: 386BB2A4  addi r3, r11, -0x4d5c
	ctx.r[3].s64 = ctx.r[11].s64 + -19804;
	// 832ACB98: 4AF68240  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACBA0 size=12
    let mut pc: u32 = 0x832ACBA0;
    'dispatch: loop {
        match pc {
            0x832ACBA0 => {
    //   block [0x832ACBA0..0x832ACBAC)
	// 832ACBA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACBA4: 386BB2A8  addi r3, r11, -0x4d58
	ctx.r[3].s64 = ctx.r[11].s64 + -19800;
	// 832ACBA8: 4AF68230  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACBB0 size=12
    let mut pc: u32 = 0x832ACBB0;
    'dispatch: loop {
        match pc {
            0x832ACBB0 => {
    //   block [0x832ACBB0..0x832ACBBC)
	// 832ACBB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACBB4: 386BB2AC  addi r3, r11, -0x4d54
	ctx.r[3].s64 = ctx.r[11].s64 + -19796;
	// 832ACBB8: 4AF68220  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACBC0 size=12
    let mut pc: u32 = 0x832ACBC0;
    'dispatch: loop {
        match pc {
            0x832ACBC0 => {
    //   block [0x832ACBC0..0x832ACBCC)
	// 832ACBC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACBC4: 386BB2F0  addi r3, r11, -0x4d10
	ctx.r[3].s64 = ctx.r[11].s64 + -19728;
	// 832ACBC8: 4AF68210  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACBD0 size=12
    let mut pc: u32 = 0x832ACBD0;
    'dispatch: loop {
        match pc {
            0x832ACBD0 => {
    //   block [0x832ACBD0..0x832ACBDC)
	// 832ACBD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACBD4: 386BB2F4  addi r3, r11, -0x4d0c
	ctx.r[3].s64 = ctx.r[11].s64 + -19724;
	// 832ACBD8: 4AF68200  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACBE0 size=12
    let mut pc: u32 = 0x832ACBE0;
    'dispatch: loop {
        match pc {
            0x832ACBE0 => {
    //   block [0x832ACBE0..0x832ACBEC)
	// 832ACBE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACBE4: 386BB2F8  addi r3, r11, -0x4d08
	ctx.r[3].s64 = ctx.r[11].s64 + -19720;
	// 832ACBE8: 4AF681F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACBF0 size=104
    let mut pc: u32 = 0x832ACBF0;
    'dispatch: loop {
        match pc {
            0x832ACBF0 => {
    //   block [0x832ACBF0..0x832ACC58)
	// 832ACBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACBF4: 4B9FC815  bl 0x82ca9408
	ctx.lr = 0x832ACBF8;
	sub_82CA93D0(ctx, base);
	// 832ACBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACBFC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACC00: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832ACC04: 396BB380  addi r11, r11, -0x4c80
	ctx.r[11].s64 = ctx.r[11].s64 + -19584;
	// 832ACC08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACC0C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832ACC10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACC14: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACC18: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACC1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACC20: 4AF19B49  bl 0x821c6768
	ctx.lr = 0x832ACC24;
	sub_821C6768(ctx, base);
	// 832ACC24: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACC28: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACC2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACC30: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACC34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACC38: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACC3C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACC40: 4082FFE8  bne 0x832acc28
	if !ctx.cr[0].eq {
	pc = 0x832ACC28; continue 'dispatch;
	}
	// 832ACC44: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACC48: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACC4C: 4080FFCC  bge 0x832acc18
	if !ctx.cr[0].lt {
	pc = 0x832ACC18; continue 'dispatch;
	}
	// 832ACC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACC54: 4B9FC804  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACC58 size=104
    let mut pc: u32 = 0x832ACC58;
    'dispatch: loop {
        match pc {
            0x832ACC58 => {
    //   block [0x832ACC58..0x832ACCC0)
	// 832ACC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACC5C: 4B9FC7AD  bl 0x82ca9408
	ctx.lr = 0x832ACC60;
	sub_82CA93D0(ctx, base);
	// 832ACC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACC64: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACC68: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832ACC6C: 396BB3C0  addi r11, r11, -0x4c40
	ctx.r[11].s64 = ctx.r[11].s64 + -19520;
	// 832ACC70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACC74: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832ACC78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACC7C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACC80: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACC88: 4AF19AE1  bl 0x821c6768
	ctx.lr = 0x832ACC8C;
	sub_821C6768(ctx, base);
	// 832ACC8C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACC90: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACC94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACC98: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACC9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACCA0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACCA4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACCA8: 4082FFE8  bne 0x832acc90
	if !ctx.cr[0].eq {
	pc = 0x832ACC90; continue 'dispatch;
	}
	// 832ACCAC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACCB0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACCB4: 4080FFCC  bge 0x832acc80
	if !ctx.cr[0].lt {
	pc = 0x832ACC80; continue 'dispatch;
	}
	// 832ACCB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACCBC: 4B9FC79C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACCC0 size=12
    let mut pc: u32 = 0x832ACCC0;
    'dispatch: loop {
        match pc {
            0x832ACCC0 => {
    //   block [0x832ACCC0..0x832ACCCC)
	// 832ACCC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACCC4: 386BB454  addi r3, r11, -0x4bac
	ctx.r[3].s64 = ctx.r[11].s64 + -19372;
	// 832ACCC8: 4AF68110  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACCD0 size=12
    let mut pc: u32 = 0x832ACCD0;
    'dispatch: loop {
        match pc {
            0x832ACCD0 => {
    //   block [0x832ACCD0..0x832ACCDC)
	// 832ACCD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACCD4: 386BB458  addi r3, r11, -0x4ba8
	ctx.r[3].s64 = ctx.r[11].s64 + -19368;
	// 832ACCD8: 4AF68100  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACCE0 size=12
    let mut pc: u32 = 0x832ACCE0;
    'dispatch: loop {
        match pc {
            0x832ACCE0 => {
    //   block [0x832ACCE0..0x832ACCEC)
	// 832ACCE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACCE4: 386BB45C  addi r3, r11, -0x4ba4
	ctx.r[3].s64 = ctx.r[11].s64 + -19364;
	// 832ACCE8: 4AF680F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACCF0 size=12
    let mut pc: u32 = 0x832ACCF0;
    'dispatch: loop {
        match pc {
            0x832ACCF0 => {
    //   block [0x832ACCF0..0x832ACCFC)
	// 832ACCF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACCF4: 386BB460  addi r3, r11, -0x4ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -19360;
	// 832ACCF8: 4AF680E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD00 size=12
    let mut pc: u32 = 0x832ACD00;
    'dispatch: loop {
        match pc {
            0x832ACD00 => {
    //   block [0x832ACD00..0x832ACD0C)
	// 832ACD00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD04: 386BB464  addi r3, r11, -0x4b9c
	ctx.r[3].s64 = ctx.r[11].s64 + -19356;
	// 832ACD08: 4AF680D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD10 size=12
    let mut pc: u32 = 0x832ACD10;
    'dispatch: loop {
        match pc {
            0x832ACD10 => {
    //   block [0x832ACD10..0x832ACD1C)
	// 832ACD10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD14: 386BB468  addi r3, r11, -0x4b98
	ctx.r[3].s64 = ctx.r[11].s64 + -19352;
	// 832ACD18: 4AF680C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD20 size=12
    let mut pc: u32 = 0x832ACD20;
    'dispatch: loop {
        match pc {
            0x832ACD20 => {
    //   block [0x832ACD20..0x832ACD2C)
	// 832ACD20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD24: 386BB46C  addi r3, r11, -0x4b94
	ctx.r[3].s64 = ctx.r[11].s64 + -19348;
	// 832ACD28: 4AF680B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD30 size=12
    let mut pc: u32 = 0x832ACD30;
    'dispatch: loop {
        match pc {
            0x832ACD30 => {
    //   block [0x832ACD30..0x832ACD3C)
	// 832ACD30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD34: 386BB470  addi r3, r11, -0x4b90
	ctx.r[3].s64 = ctx.r[11].s64 + -19344;
	// 832ACD38: 4AF680A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD40 size=12
    let mut pc: u32 = 0x832ACD40;
    'dispatch: loop {
        match pc {
            0x832ACD40 => {
    //   block [0x832ACD40..0x832ACD4C)
	// 832ACD40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD44: 386BB474  addi r3, r11, -0x4b8c
	ctx.r[3].s64 = ctx.r[11].s64 + -19340;
	// 832ACD48: 4AF68090  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD50 size=12
    let mut pc: u32 = 0x832ACD50;
    'dispatch: loop {
        match pc {
            0x832ACD50 => {
    //   block [0x832ACD50..0x832ACD5C)
	// 832ACD50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD54: 386BB478  addi r3, r11, -0x4b88
	ctx.r[3].s64 = ctx.r[11].s64 + -19336;
	// 832ACD58: 4AF68080  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD60 size=12
    let mut pc: u32 = 0x832ACD60;
    'dispatch: loop {
        match pc {
            0x832ACD60 => {
    //   block [0x832ACD60..0x832ACD6C)
	// 832ACD60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD64: 386BB47C  addi r3, r11, -0x4b84
	ctx.r[3].s64 = ctx.r[11].s64 + -19332;
	// 832ACD68: 4AF68070  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD70 size=12
    let mut pc: u32 = 0x832ACD70;
    'dispatch: loop {
        match pc {
            0x832ACD70 => {
    //   block [0x832ACD70..0x832ACD7C)
	// 832ACD70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD74: 386BB4D4  addi r3, r11, -0x4b2c
	ctx.r[3].s64 = ctx.r[11].s64 + -19244;
	// 832ACD78: 4AF68060  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD80 size=12
    let mut pc: u32 = 0x832ACD80;
    'dispatch: loop {
        match pc {
            0x832ACD80 => {
    //   block [0x832ACD80..0x832ACD8C)
	// 832ACD80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD84: 386BB4D8  addi r3, r11, -0x4b28
	ctx.r[3].s64 = ctx.r[11].s64 + -19240;
	// 832ACD88: 4AF68050  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACD90 size=12
    let mut pc: u32 = 0x832ACD90;
    'dispatch: loop {
        match pc {
            0x832ACD90 => {
    //   block [0x832ACD90..0x832ACD9C)
	// 832ACD90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACD94: 386BB4DC  addi r3, r11, -0x4b24
	ctx.r[3].s64 = ctx.r[11].s64 + -19236;
	// 832ACD98: 4AF68040  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACDA0 size=12
    let mut pc: u32 = 0x832ACDA0;
    'dispatch: loop {
        match pc {
            0x832ACDA0 => {
    //   block [0x832ACDA0..0x832ACDAC)
	// 832ACDA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACDA4: 386BB510  addi r3, r11, -0x4af0
	ctx.r[3].s64 = ctx.r[11].s64 + -19184;
	// 832ACDA8: 4AF68030  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACDB0 size=12
    let mut pc: u32 = 0x832ACDB0;
    'dispatch: loop {
        match pc {
            0x832ACDB0 => {
    //   block [0x832ACDB0..0x832ACDBC)
	// 832ACDB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACDB4: 386BB514  addi r3, r11, -0x4aec
	ctx.r[3].s64 = ctx.r[11].s64 + -19180;
	// 832ACDB8: 4AF68020  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACDC0 size=12
    let mut pc: u32 = 0x832ACDC0;
    'dispatch: loop {
        match pc {
            0x832ACDC0 => {
    //   block [0x832ACDC0..0x832ACDCC)
	// 832ACDC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACDC4: 386BB518  addi r3, r11, -0x4ae8
	ctx.r[3].s64 = ctx.r[11].s64 + -19176;
	// 832ACDC8: 4AF68010  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACDD0 size=12
    let mut pc: u32 = 0x832ACDD0;
    'dispatch: loop {
        match pc {
            0x832ACDD0 => {
    //   block [0x832ACDD0..0x832ACDDC)
	// 832ACDD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACDD4: 386BB51C  addi r3, r11, -0x4ae4
	ctx.r[3].s64 = ctx.r[11].s64 + -19172;
	// 832ACDD8: 4AF68000  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACDE0 size=12
    let mut pc: u32 = 0x832ACDE0;
    'dispatch: loop {
        match pc {
            0x832ACDE0 => {
    //   block [0x832ACDE0..0x832ACDEC)
	// 832ACDE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACDE4: 386BB520  addi r3, r11, -0x4ae0
	ctx.r[3].s64 = ctx.r[11].s64 + -19168;
	// 832ACDE8: 4AF67FF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACDF0 size=12
    let mut pc: u32 = 0x832ACDF0;
    'dispatch: loop {
        match pc {
            0x832ACDF0 => {
    //   block [0x832ACDF0..0x832ACDFC)
	// 832ACDF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACDF4: 386BB524  addi r3, r11, -0x4adc
	ctx.r[3].s64 = ctx.r[11].s64 + -19164;
	// 832ACDF8: 4AF67FE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE00 size=12
    let mut pc: u32 = 0x832ACE00;
    'dispatch: loop {
        match pc {
            0x832ACE00 => {
    //   block [0x832ACE00..0x832ACE0C)
	// 832ACE00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE04: 386BB528  addi r3, r11, -0x4ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -19160;
	// 832ACE08: 4AF67FD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE10 size=12
    let mut pc: u32 = 0x832ACE10;
    'dispatch: loop {
        match pc {
            0x832ACE10 => {
    //   block [0x832ACE10..0x832ACE1C)
	// 832ACE10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE14: 386BB52C  addi r3, r11, -0x4ad4
	ctx.r[3].s64 = ctx.r[11].s64 + -19156;
	// 832ACE18: 4AF67FC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE20 size=12
    let mut pc: u32 = 0x832ACE20;
    'dispatch: loop {
        match pc {
            0x832ACE20 => {
    //   block [0x832ACE20..0x832ACE2C)
	// 832ACE20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE24: 386BB530  addi r3, r11, -0x4ad0
	ctx.r[3].s64 = ctx.r[11].s64 + -19152;
	// 832ACE28: 4AF67FB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE30 size=12
    let mut pc: u32 = 0x832ACE30;
    'dispatch: loop {
        match pc {
            0x832ACE30 => {
    //   block [0x832ACE30..0x832ACE3C)
	// 832ACE30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE34: 386BB534  addi r3, r11, -0x4acc
	ctx.r[3].s64 = ctx.r[11].s64 + -19148;
	// 832ACE38: 4AF67FA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE40 size=12
    let mut pc: u32 = 0x832ACE40;
    'dispatch: loop {
        match pc {
            0x832ACE40 => {
    //   block [0x832ACE40..0x832ACE4C)
	// 832ACE40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE44: 386BB538  addi r3, r11, -0x4ac8
	ctx.r[3].s64 = ctx.r[11].s64 + -19144;
	// 832ACE48: 4AF67F90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE50 size=12
    let mut pc: u32 = 0x832ACE50;
    'dispatch: loop {
        match pc {
            0x832ACE50 => {
    //   block [0x832ACE50..0x832ACE5C)
	// 832ACE50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE54: 386BB53C  addi r3, r11, -0x4ac4
	ctx.r[3].s64 = ctx.r[11].s64 + -19140;
	// 832ACE58: 4AF67F80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE60 size=12
    let mut pc: u32 = 0x832ACE60;
    'dispatch: loop {
        match pc {
            0x832ACE60 => {
    //   block [0x832ACE60..0x832ACE6C)
	// 832ACE60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE64: 386BB5B8  addi r3, r11, -0x4a48
	ctx.r[3].s64 = ctx.r[11].s64 + -19016;
	// 832ACE68: 4AF67F70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE70 size=12
    let mut pc: u32 = 0x832ACE70;
    'dispatch: loop {
        match pc {
            0x832ACE70 => {
    //   block [0x832ACE70..0x832ACE7C)
	// 832ACE70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE74: 386BB5BC  addi r3, r11, -0x4a44
	ctx.r[3].s64 = ctx.r[11].s64 + -19012;
	// 832ACE78: 4AF67F60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE80 size=12
    let mut pc: u32 = 0x832ACE80;
    'dispatch: loop {
        match pc {
            0x832ACE80 => {
    //   block [0x832ACE80..0x832ACE8C)
	// 832ACE80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE84: 386BB5C0  addi r3, r11, -0x4a40
	ctx.r[3].s64 = ctx.r[11].s64 + -19008;
	// 832ACE88: 4AF67F50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACE90 size=12
    let mut pc: u32 = 0x832ACE90;
    'dispatch: loop {
        match pc {
            0x832ACE90 => {
    //   block [0x832ACE90..0x832ACE9C)
	// 832ACE90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACE94: 386BB5C4  addi r3, r11, -0x4a3c
	ctx.r[3].s64 = ctx.r[11].s64 + -19004;
	// 832ACE98: 4AF67F40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACEA0 size=104
    let mut pc: u32 = 0x832ACEA0;
    'dispatch: loop {
        match pc {
            0x832ACEA0 => {
    //   block [0x832ACEA0..0x832ACF08)
	// 832ACEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACEA4: 4B9FC565  bl 0x82ca9408
	ctx.lr = 0x832ACEA8;
	sub_82CA93D0(ctx, base);
	// 832ACEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACEAC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACEB0: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 832ACEB4: 396BB5C8  addi r11, r11, -0x4a38
	ctx.r[11].s64 = ctx.r[11].s64 + -19000;
	// 832ACEB8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACEBC: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 832ACEC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACEC4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACEC8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACED0: 4AF19899  bl 0x821c6768
	ctx.lr = 0x832ACED4;
	sub_821C6768(ctx, base);
	// 832ACED4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACED8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACEDC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACEE0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACEE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACEE8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACEEC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACEF0: 4082FFE8  bne 0x832aced8
	if !ctx.cr[0].eq {
	pc = 0x832ACED8; continue 'dispatch;
	}
	// 832ACEF4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACEF8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACEFC: 4080FFCC  bge 0x832acec8
	if !ctx.cr[0].lt {
	pc = 0x832ACEC8; continue 'dispatch;
	}
	// 832ACF00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACF04: 4B9FC554  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACF08 size=104
    let mut pc: u32 = 0x832ACF08;
    'dispatch: loop {
        match pc {
            0x832ACF08 => {
    //   block [0x832ACF08..0x832ACF70)
	// 832ACF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACF0C: 4B9FC4FD  bl 0x82ca9408
	ctx.lr = 0x832ACF10;
	sub_82CA93D0(ctx, base);
	// 832ACF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACF14: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACF18: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 832ACF1C: 396BB5E4  addi r11, r11, -0x4a1c
	ctx.r[11].s64 = ctx.r[11].s64 + -18972;
	// 832ACF20: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACF24: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 832ACF28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACF2C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACF30: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACF34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACF38: 4AF19831  bl 0x821c6768
	ctx.lr = 0x832ACF3C;
	sub_821C6768(ctx, base);
	// 832ACF3C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACF40: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACF44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACF48: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACF4C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACF50: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACF54: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACF58: 4082FFE8  bne 0x832acf40
	if !ctx.cr[0].eq {
	pc = 0x832ACF40; continue 'dispatch;
	}
	// 832ACF5C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACF60: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACF64: 4080FFCC  bge 0x832acf30
	if !ctx.cr[0].lt {
	pc = 0x832ACF30; continue 'dispatch;
	}
	// 832ACF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACF6C: 4B9FC4EC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ACF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ACF70 size=4
    let mut pc: u32 = 0x832ACF70;
    'dispatch: loop {
        match pc {
            0x832ACF70 => {
    //   block [0x832ACF70..0x832ACF74)
	// 832ACF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


