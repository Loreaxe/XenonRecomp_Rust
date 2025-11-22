pub fn sub_8290C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290C878 size=76
    let mut pc: u32 = 0x8290C878;
    'dispatch: loop {
        match pc {
            0x8290C878 => {
    //   block [0x8290C878..0x8290C8C4)
	// 8290C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C87C: 4889B8ED  bl 0x831a8168
	ctx.lr = 0x8290C880;
	sub_831A8130(ctx, base);
	// 8290C880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290C884: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290C888: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290C88C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290C890: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8290C894: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290C898: 48106DE9  bl 0x82a13680
	ctx.lr = 0x8290C89C;
	sub_82A13680(ctx, base);
	// 8290C89C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290C8A0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8290C8A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C8A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8290C8AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290C8B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290C8B4: 4E800421  bctrl
	ctx.lr = 0x8290C8B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290C8B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290C8BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290C8C0: 4889B8F8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290C8C8 size=340
    let mut pc: u32 = 0x8290C8C8;
    'dispatch: loop {
        match pc {
            0x8290C8C8 => {
    //   block [0x8290C8C8..0x8290CA1C)
	// 8290C8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290C8CC: 4889B899  bl 0x831a8164
	ctx.lr = 0x8290C8D0;
	sub_831A8130(ctx, base);
	// 8290C8D0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8290C8D4: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290CA20 size=112
    let mut pc: u32 = 0x8290CA20;
    'dispatch: loop {
        match pc {
            0x8290CA20 => {
    //   block [0x8290CA20..0x8290CA90)
	// 8290CA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290CA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290CA28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290CA2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290CA30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290CA34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290CA38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290CA3C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8290CA40: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290CA44: 4BF8A5FD  bl 0x82897040
	ctx.lr = 0x8290CA48;
	sub_82897040(ctx, base);
	// 8290CA48: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290CA4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290CA50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290CA54: 4B9B35AD  bl 0x822c0000
	ctx.lr = 0x8290CA58;
	sub_822C0000(ctx, base);
	// 8290CA58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290CA5C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290CA60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290CA64: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290CA68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290CA6C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290CA70: 419A0008  beq cr6, 0x8290ca78
	if ctx.cr[6].eq {
	pc = 0x8290CA78; continue 'dispatch;
	}
	// 8290CA74: 4B9B3E1D  bl 0x822c0890
	ctx.lr = 0x8290CA78;
	sub_822C0890(ctx, base);
	// 8290CA78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290CA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290CA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290CA84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290CA88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290CA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290CA90 size=188
    let mut pc: u32 = 0x8290CA90;
    'dispatch: loop {
        match pc {
            0x8290CA90 => {
    //   block [0x8290CA90..0x8290CB4C)
	// 8290CA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290CA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290CA98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290CA9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290CAA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290CAA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290CAA8: 4802BBB9  bl 0x82938660
	ctx.lr = 0x8290CAAC;
	sub_82938660(ctx, base);
	// 8290CAAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290CAB0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290CAB4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290CAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290CABC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290CAC0: 419A0024  beq cr6, 0x8290cae4
	if ctx.cr[6].eq {
	pc = 0x8290CAE4; continue 'dispatch;
	}
	// 8290CAC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290CAC8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290CACC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290CAD0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290CAD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290CAD8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290CADC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290CAE0: 4082FFE8  bne 0x8290cac8
	if !ctx.cr[0].eq {
	pc = 0x8290CAC8; continue 'dispatch;
	}
	// 8290CAE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290CAE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8290CAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290CAF0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290CAF4: 389F00FC  addi r4, r31, 0xfc
	ctx.r[4].s64 = ctx.r[31].s64 + 252;
	// 8290CAF8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290CAFC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290CB00: 48551BA9  bl 0x82e5e6a8
	ctx.lr = 0x8290CB04;
	sub_82E5E6A8(ctx, base);
	// 8290CB04: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290CB08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290CB0C: 419A0008  beq cr6, 0x8290cb14
	if ctx.cr[6].eq {
	pc = 0x8290CB14; continue 'dispatch;
	}
	// 8290CB10: 4B9B3D81  bl 0x822c0890
	ctx.lr = 0x8290CB14;
	sub_822C0890(ctx, base);
	// 8290CB14: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290CB18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290CB1C: 419A0008  beq cr6, 0x8290cb24
	if ctx.cr[6].eq {
	pc = 0x8290CB24; continue 'dispatch;
	}
	// 8290CB20: 4B9B3D71  bl 0x822c0890
	ctx.lr = 0x8290CB24;
	sub_822C0890(ctx, base);
	// 8290CB24: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290CB28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290CB2C: 419A0008  beq cr6, 0x8290cb34
	if ctx.cr[6].eq {
	pc = 0x8290CB34; continue 'dispatch;
	}
	// 8290CB30: 4B9B3D61  bl 0x822c0890
	ctx.lr = 0x8290CB34;
	sub_822C0890(ctx, base);
	// 8290CB34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8290CB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290CB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290CB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290CB44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290CB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290CB50 size=248
    let mut pc: u32 = 0x8290CB50;
    'dispatch: loop {
        match pc {
            0x8290CB50 => {
    //   block [0x8290CB50..0x8290CC48)
	// 8290CB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290CB54: 4889B611  bl 0x831a8164
	ctx.lr = 0x8290CB58;
	sub_831A8130(ctx, base);
	// 8290CB58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290CB5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290CB60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8290CB64: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8290CB68: 807E0244  lwz r3, 0x244(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290CB6C: 48015E2D  bl 0x82922998
	ctx.lr = 0x8290CB70;
	sub_82922998(ctx, base);
	// 8290CB70: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8290CB74: 418200CC  beq 0x8290cc40
	if ctx.cr[0].eq {
	pc = 0x8290CC40; continue 'dispatch;
	}
	// 8290CB78: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290CB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290CB80: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290CB84: 38A00314  li r5, 0x314
	ctx.r[5].s64 = 788;
	// 8290CB88: 38600114  li r3, 0x114
	ctx.r[3].s64 = 276;
	// 8290CB8C: 484E585D  bl 0x82df23e8
	ctx.lr = 0x8290CB90;
	sub_82DF23E8(ctx, base);
	// 8290CB90: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8290CB94: 41820020  beq 0x8290cbb4
	if ctx.cr[0].eq {
	pc = 0x8290CBB4; continue 'dispatch;
	}
	// 8290CB98: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 8290CB9C: 486FC41D  bl 0x83008fb8
	ctx.lr = 0x8290CBA0;
	sub_83008FB8(ctx, base);
	// 8290CBA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290CBA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290CBA8: 480108C1  bl 0x8291d468
	ctx.lr = 0x8290CBAC;
	sub_8291D468(ctx, base);
	// 8290CBAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290CBB0: 48000008  b 0x8290cbb8
	pc = 0x8290CBB8; continue 'dispatch;
	// 8290CBB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290CBB8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8290CBBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290CBC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290CBC4: 4BFFDC4D  bl 0x8290a810
	ctx.lr = 0x8290CBC8;
	sub_8290A810(ctx, base);
	// 8290CBC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290CBCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290CBD0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290CBD4: 4B9B342D  bl 0x822c0000
	ctx.lr = 0x8290CBD8;
	sub_822C0000(ctx, base);
	// 8290CBD8: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290CBDC: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290CBE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290CBE4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8290CBE8: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8290CBEC: 419A0024  beq cr6, 0x8290cc10
	if ctx.cr[6].eq {
	pc = 0x8290CC10; continue 'dispatch;
	}
	// 8290CBF0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8290CBF4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290CBF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290CBFC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290CC00: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290CC04: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290CC08: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290CC0C: 4082FFE8  bne 0x8290cbf4
	if !ctx.cr[0].eq {
	pc = 0x8290CBF4; continue 'dispatch;
	}
	// 8290CC10: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8290CC14: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290CC18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8290CC1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8290CC20: 4BC008E9  bl 0x8250d508
	ctx.lr = 0x8290CC24;
	sub_8250D508(ctx, base);
	// 8290CC24: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 8290CC28: 486FC391  bl 0x83008fb8
	ctx.lr = 0x8290CC2C;
	sub_83008FB8(ctx, base);
	// 8290CC2C: 907E02EC  stw r3, 0x2ec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(748 as u32), ctx.r[3].u32 ) };
	// 8290CC30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8290CC34: 419A000C  beq cr6, 0x8290cc40
	if ctx.cr[6].eq {
	pc = 0x8290CC40; continue 'dispatch;
	}
	// 8290CC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290CC3C: 4B9B3C55  bl 0x822c0890
	ctx.lr = 0x8290CC40;
	sub_822C0890(ctx, base);
	// 8290CC40: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8290CC44: 4889B570  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290CC48 size=32
    let mut pc: u32 = 0x8290CC48;
    'dispatch: loop {
        match pc {
            0x8290CC48 => {
    //   block [0x8290CC48..0x8290CC68)
	// 8290CC48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290CC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8290CC50: 394B01DC  addi r10, r11, 0x1dc
	ctx.r[10].s64 = ctx.r[11].s64 + 476;
	// 8290CC54: 912B01DC  stw r9, 0x1dc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(476 as u32), ctx.r[9].u32 ) };
	// 8290CC58: 806B01E0  lwz r3, 0x1e0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(480 as u32) ) } as u64;
	// 8290CC5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290CC60: 912B01E0  stw r9, 0x1e0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(480 as u32), ctx.r[9].u32 ) };
	// 8290CC64: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290CC68 size=8
    let mut pc: u32 = 0x8290CC68;
    'dispatch: loop {
        match pc {
            0x8290CC68 => {
    //   block [0x8290CC68..0x8290CC70)
	// 8290CC68: 4B9B3C28  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8290CC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290CC70 size=32
    let mut pc: u32 = 0x8290CC70;
    'dispatch: loop {
        match pc {
            0x8290CC70 => {
    //   block [0x8290CC70..0x8290CC90)
	// 8290CC70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290CC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8290CC78: 394B01E8  addi r10, r11, 0x1e8
	ctx.r[10].s64 = ctx.r[11].s64 + 488;
	// 8290CC7C: 912B01E8  stw r9, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[9].u32 ) };
	// 8290CC80: 806B01EC  lwz r3, 0x1ec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(492 as u32) ) } as u64;
	// 8290CC84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290CC88: 912B01EC  stw r9, 0x1ec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(492 as u32), ctx.r[9].u32 ) };
	// 8290CC8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8290CC90 size=8
    let mut pc: u32 = 0x8290CC90;
    'dispatch: loop {
        match pc {
            0x8290CC90 => {
    //   block [0x8290CC90..0x8290CC98)
	// 8290CC90: 4B9B3C00  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8290CC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290CC98 size=176
    let mut pc: u32 = 0x8290CC98;
    'dispatch: loop {
        match pc {
            0x8290CC98 => {
    //   block [0x8290CC98..0x8290CD48)
	// 8290CC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290CC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290CCA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290CCA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290CCA8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8290CCAC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290CCB0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8290CCB4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8290CCB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290CCBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290CCC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290CCC4: 388BA68C  addi r4, r11, -0x5974
	ctx.r[4].s64 = ctx.r[11].s64 + -22900;
	// 8290CCC8: 484E6D41  bl 0x82df3a08
	ctx.lr = 0x8290CCCC;
	sub_82DF3A08(ctx, base);
	// 8290CCCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290CCD0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290CCD4: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290CCD8: 4856F301  bl 0x82e7bfd8
	ctx.lr = 0x8290CCDC;
	sub_82E7BFD8(ctx, base);
	// 8290CCDC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290CCE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290CCE4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290CCE8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290CCEC: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290CCF0: 4BEDF511  bl 0x827ec200
	ctx.lr = 0x8290CCF4;
	sub_827EC200(ctx, base);
	// 8290CCF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290CCF8: 484E6731  bl 0x82df3428
	ctx.lr = 0x8290CCFC;
	sub_82DF3428(ctx, base);
	// 8290CCFC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8290CD00: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290CD04: 38AB6194  addi r5, r11, 0x6194
	ctx.r[5].s64 = ctx.r[11].s64 + 24980;
	// 8290CD08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290CD0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290CD10: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8290CD14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290CD18: 4E800421  bctrl
	ctx.lr = 0x8290CD1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290CD1C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290CD20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290CD24: 419A0008  beq cr6, 0x8290cd2c
	if ctx.cr[6].eq {
	pc = 0x8290CD2C; continue 'dispatch;
	}
	// 8290CD28: 4B9B3B69  bl 0x822c0890
	ctx.lr = 0x8290CD2C;
	sub_822C0890(ctx, base);
	// 8290CD2C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8290CD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290CD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290CD38: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8290CD3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290CD40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290CD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290CD48 size=348
    let mut pc: u32 = 0x8290CD48;
    'dispatch: loop {
        match pc {
            0x8290CD48 => {
    //   block [0x8290CD48..0x8290CEA4)
	// 8290CD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290CD4C: 4889B415  bl 0x831a8160
	ctx.lr = 0x8290CD50;
	sub_831A8130(ctx, base);
	// 8290CD50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290CD54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290CD58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290CD5C: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 8290CD60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290CD64: 4E800421  bctrl
	ctx.lr = 0x8290CD68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290CD68: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290CD6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290CD70: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290CD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290CD78: 38A00874  li r5, 0x874
	ctx.r[5].s64 = 2164;
	// 8290CD7C: 38600144  li r3, 0x144
	ctx.r[3].s64 = 324;
	// 8290CD80: 484E5669  bl 0x82df23e8
	ctx.lr = 0x8290CD84;
	sub_82DF23E8(ctx, base);
	// 8290CD84: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8290CD88: 41820054  beq 0x8290cddc
	if ctx.cr[0].eq {
	pc = 0x8290CDDC; continue 'dispatch;
	}
	// 8290CD8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8290CD90: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8290CD94: 3B9D0058  addi r28, r29, 0x58
	ctx.r[28].s64 = ctx.r[29].s64 + 88;
	// 8290CD98: 83AB6814  lwz r29, 0x6814(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26644 as u32) ) } as u64;
	// 8290CD9C: 486FC21D  bl 0x83008fb8
	ctx.lr = 0x8290CDA0;
	sub_83008FB8(ctx, base);
	// 8290CDA0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8290CDA4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8290CDA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290CDAC: 3B4BBA80  addi r26, r11, -0x4580
	ctx.r[26].s64 = ctx.r[11].s64 + -17792;
	// 8290CDB0: 481068D1  bl 0x82a13680
	ctx.lr = 0x8290CDB4;
	sub_82A13680(ctx, base);
	// 8290CDB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290CDB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290CDBC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8290CDC0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8290CDC4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8290CDC8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8290CDCC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8290CDD0: 4BEFE561  bl 0x8280b330
	ctx.lr = 0x8290CDD4;
	sub_8280B330(ctx, base);
	// 8290CDD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290CDD8: 48000008  b 0x8290cde0
	pc = 0x8290CDE0; continue 'dispatch;
	// 8290CDDC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8290CDE0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8290CDE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290CDE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290CDEC: 4BF67265  bl 0x82874050
	ctx.lr = 0x8290CDF0;
	sub_82874050(ctx, base);
	// 8290CDF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290CDF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290CDF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290CDFC: 4B9B3205  bl 0x822c0000
	ctx.lr = 0x8290CE00;
	sub_822C0000(ctx, base);
	// 8290CE00: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290CE04: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290CE08: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290CE0C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8290CE10: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290CE14: 419A0024  beq cr6, 0x8290ce38
	if ctx.cr[6].eq {
	pc = 0x8290CE38; continue 'dispatch;
	}
	// 8290CE18: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8290CE1C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290CE20: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290CE24: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290CE28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290CE2C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290CE30: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290CE34: 4082FFE8  bne 0x8290ce1c
	if !ctx.cr[0].eq {
	pc = 0x8290CE1C; continue 'dispatch;
	}
	// 8290CE38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290CE3C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290CE40: 4BC02689  bl 0x8250f4c8
	ctx.lr = 0x8290CE44;
	sub_8250F4C8(ctx, base);
	// 8290CE44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290CE48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290CE4C: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 8290CE50: 409A0008  bne cr6, 0x8290ce58
	if !ctx.cr[6].eq {
	pc = 0x8290CE58; continue 'dispatch;
	}
	// 8290CE54: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8290CE58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290CE5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290CE60: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8290CE64: 4BC026B5  bl 0x8250f518
	ctx.lr = 0x8290CE68;
	sub_8250F518(ctx, base);
	// 8290CE68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290CE6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290CE70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290CE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290CE78: 4BC00691  bl 0x8250d508
	ctx.lr = 0x8290CE7C;
	sub_8250D508(ctx, base);
	// 8290CE7C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290CE80: 484E4E11  bl 0x82df1c90
	ctx.lr = 0x8290CE84;
	sub_82DF1C90(ctx, base);
	// 8290CE84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290CE88: 484E4E09  bl 0x82df1c90
	ctx.lr = 0x8290CE8C;
	sub_82DF1C90(ctx, base);
	// 8290CE8C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290CE90: 419A000C  beq cr6, 0x8290ce9c
	if ctx.cr[6].eq {
	pc = 0x8290CE9C; continue 'dispatch;
	}
	// 8290CE94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290CE98: 4B9B39F9  bl 0x822c0890
	ctx.lr = 0x8290CE9C;
	sub_822C0890(ctx, base);
	// 8290CE9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8290CEA0: 4889B310  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290CEA8 size=292
    let mut pc: u32 = 0x8290CEA8;
    'dispatch: loop {
        match pc {
            0x8290CEA8 => {
    //   block [0x8290CEA8..0x8290CFCC)
	// 8290CEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290CEAC: 4889B2C1  bl 0x831a816c
	ctx.lr = 0x8290CEB0;
	sub_831A8130(ctx, base);
	// 8290CEB0: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8290CEB4: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8290CEB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290CEBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290CEC0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290CEC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290CEC8: C01F032C  lfs f0, 0x32c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290CECC: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290CED0: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8290CED4: 409900E8  ble cr6, 0x8290cfbc
	if !ctx.cr[6].gt {
	pc = 0x8290CFBC; continue 'dispatch;
	}
	// 8290CED8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290CEDC: 816B015C  lwz r11, 0x15c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) } as u64;
	// 8290CEE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290CEE4: 4E800421  bctrl
	ctx.lr = 0x8290CEE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290CEE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290CEEC: 418200D0  beq 0x8290cfbc
	if ctx.cr[0].eq {
	pc = 0x8290CFBC; continue 'dispatch;
	}
	// 8290CEF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290CEF4: 4804DD7D  bl 0x8295ac70
	ctx.lr = 0x8290CEF8;
	sub_8295AC70(ctx, base);
	// 8290CEF8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8290CEFC: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 8290CF00: 409900BC  ble cr6, 0x8290cfbc
	if !ctx.cr[6].gt {
	pc = 0x8290CFBC; continue 'dispatch;
	}
	// 8290CF04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290CF08: 48106779  bl 0x82a13680
	ctx.lr = 0x8290CF0C;
	sub_82A13680(ctx, base);
	// 8290CF0C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8290CF10: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290CF14: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 8290CF18: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8290CF1C: 13DD58C7  vcmpequd (lvx128) v30, v29, v11
	tmp.u32 = ctx.r[29].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290CFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290CFD0 size=964
    let mut pc: u32 = 0x8290CFD0;
    'dispatch: loop {
        match pc {
            0x8290CFD0 => {
    //   block [0x8290CFD0..0x8290D394)
	// 8290CFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290CFD4: 4889B199  bl 0x831a816c
	ctx.lr = 0x8290CFD8;
	sub_831A8130(ctx, base);
	// 8290CFD8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8290CFDC: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290CFE0: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290CFE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290CFE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290CFEC: 409A02C0  bne cr6, 0x8290d2ac
	if !ctx.cr[6].eq {
	pc = 0x8290D2AC; continue 'dispatch;
	}
	// 8290CFF0: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290CFF4: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290CFF8: 4800A039  bl 0x82917030
	ctx.lr = 0x8290CFFC;
	sub_82917030(ctx, base);
	// 8290CFFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D000: 40820388  bne 0x8290d388
	if !ctx.cr[0].eq {
	pc = 0x8290D388; continue 'dispatch;
	}
	// 8290D004: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290D008: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290D00C: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290D010: 38A00968  li r5, 0x968
	ctx.r[5].s64 = 2408;
	// 8290D014: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 8290D018: 484E53D1  bl 0x82df23e8
	ctx.lr = 0x8290D01C;
	sub_82DF23E8(ctx, base);
	// 8290D01C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290D020: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8290D024: C3EB08A8  lfs f31, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290D028: 41820040  beq 0x8290d068
	if ctx.cr[0].eq {
	pc = 0x8290D068; continue 'dispatch;
	}
	// 8290D02C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290D030: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8290D034: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8290D038: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8290D03C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290D040: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8290D044: D0010088  stfs f0, 0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8290D048: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8290D04C: 4856EF8D  bl 0x82e7bfd8
	ctx.lr = 0x8290D050;
	sub_82E7BFD8(ctx, base);
	// 8290D050: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290D054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D058: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290D05C: 4BC7DA05  bl 0x8258aa60
	ctx.lr = 0x8290D060;
	sub_8258AA60(ctx, base);
	// 8290D060: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D064: 48000008  b 0x8290d06c
	pc = 0x8290D06C; continue 'dispatch;
	// 8290D068: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290D06C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8290D070: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290D074: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8290D078: 4BF89FC9  bl 0x82897040
	ctx.lr = 0x8290D07C;
	sub_82897040(ctx, base);
	// 8290D07C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290D080: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290D084: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8290D088: 4B9B2F79  bl 0x822c0000
	ctx.lr = 0x8290D08C;
	sub_822C0000(ctx, base);
	// 8290D08C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D094: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290D098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D09C: 4E800421  bctrl
	ctx.lr = 0x8290D0A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D0A0: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D0A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D0A8: 418200EC  beq 0x8290d194
	if ctx.cr[0].eq {
	pc = 0x8290D194; continue 'dispatch;
	}
	// 8290D0AC: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8290D0B0: 484E6959  bl 0x82df3a08
	ctx.lr = 0x8290D0B4;
	sub_82DF3A08(ctx, base);
	// 8290D0B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D0B8: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8290D0BC: 4BEDF50D  bl 0x827ec5c8
	ctx.lr = 0x8290D0C0;
	sub_827EC5C8(ctx, base);
	// 8290D0C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290D0C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290D0C8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8290D0CC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D0D0: 48507F61  bl 0x82e15030
	ctx.lr = 0x8290D0D4;
	sub_82E15030(ctx, base);
	// 8290D0D4: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 8290D0D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D0DC: 419A0008  beq cr6, 0x8290d0e4
	if ctx.cr[6].eq {
	pc = 0x8290D0E4; continue 'dispatch;
	}
	// 8290D0E0: 4B9B37B1  bl 0x822c0890
	ctx.lr = 0x8290D0E4;
	sub_822C0890(ctx, base);
	// 8290D0E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D0E8: 484E6341  bl 0x82df3428
	ctx.lr = 0x8290D0EC;
	sub_82DF3428(ctx, base);
	// 8290D0EC: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8290D0F0: 83A1005C  lwz r29, 0x5c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290D0F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D0F8: 419A0094  beq cr6, 0x8290d18c
	if ctx.cr[6].eq {
	pc = 0x8290D18C; continue 'dispatch;
	}
	// 8290D0FC: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290D100: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290D104: 48505D2D  bl 0x82e12e30
	ctx.lr = 0x8290D108;
	sub_82E12E30(ctx, base);
	// 8290D108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D10C: 48505605  bl 0x82e12710
	ctx.lr = 0x8290D110;
	sub_82E12710(ctx, base);
	// 8290D110: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290D114: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D118: 388B58B0  addi r4, r11, 0x58b0
	ctx.r[4].s64 = ctx.r[11].s64 + 22704;
	// 8290D11C: 484E68ED  bl 0x82df3a08
	ctx.lr = 0x8290D120;
	sub_82DF3A08(ctx, base);
	// 8290D120: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 8290D124: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 8290D128: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290D12C: 419A0024  beq cr6, 0x8290d150
	if ctx.cr[6].eq {
	pc = 0x8290D150; continue 'dispatch;
	}
	// 8290D130: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8290D134: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290D138: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290D13C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290D140: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290D144: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290D148: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290D14C: 4082FFE8  bne 0x8290d134
	if !ctx.cr[0].eq {
	pc = 0x8290D134; continue 'dispatch;
	}
	// 8290D150: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8290D154: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290D158: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290D15C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290D160: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8290D164: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D168: 4BEDF0A1  bl 0x827ec208
	ctx.lr = 0x8290D16C;
	sub_827EC208(ctx, base);
	// 8290D16C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290D170: 907E0318  stw r3, 0x318(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(792 as u32), ctx.r[3].u32 ) };
	// 8290D174: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290D178: 419A000C  beq cr6, 0x8290d184
	if ctx.cr[6].eq {
	pc = 0x8290D184; continue 'dispatch;
	}
	// 8290D17C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8290D180: 4B9B3711  bl 0x822c0890
	ctx.lr = 0x8290D184;
	sub_822C0890(ctx, base);
	// 8290D184: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D188: 484E62A1  bl 0x82df3428
	ctx.lr = 0x8290D18C;
	sub_82DF3428(ctx, base);
	// 8290D18C: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8290D190: 480000EC  b 0x8290d27c
	pc = 0x8290D27C; continue 'dispatch;
	// 8290D194: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290D198: 388B7868  addi r4, r11, 0x7868
	ctx.r[4].s64 = ctx.r[11].s64 + 30824;
	// 8290D19C: 484E686D  bl 0x82df3a08
	ctx.lr = 0x8290D1A0;
	sub_82DF3A08(ctx, base);
	// 8290D1A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D1A4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8290D1A8: 4BEDF421  bl 0x827ec5c8
	ctx.lr = 0x8290D1AC;
	sub_827EC5C8(ctx, base);
	// 8290D1AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290D1B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290D1B4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290D1B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D1BC: 48507E75  bl 0x82e15030
	ctx.lr = 0x8290D1C0;
	sub_82E15030(ctx, base);
	// 8290D1C0: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8290D1C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D1C8: 419A0008  beq cr6, 0x8290d1d0
	if ctx.cr[6].eq {
	pc = 0x8290D1D0; continue 'dispatch;
	}
	// 8290D1CC: 4B9B36C5  bl 0x822c0890
	ctx.lr = 0x8290D1D0;
	sub_822C0890(ctx, base);
	// 8290D1D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D1D4: 484E6255  bl 0x82df3428
	ctx.lr = 0x8290D1D8;
	sub_82DF3428(ctx, base);
	// 8290D1D8: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8290D1DC: 83A1005C  lwz r29, 0x5c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290D1E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D1E4: 419A0094  beq cr6, 0x8290d278
	if ctx.cr[6].eq {
	pc = 0x8290D278; continue 'dispatch;
	}
	// 8290D1E8: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290D1EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290D1F0: 48505C41  bl 0x82e12e30
	ctx.lr = 0x8290D1F4;
	sub_82E12E30(ctx, base);
	// 8290D1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D1F8: 48505519  bl 0x82e12710
	ctx.lr = 0x8290D1FC;
	sub_82E12710(ctx, base);
	// 8290D1FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290D200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D204: 388B58B0  addi r4, r11, 0x58b0
	ctx.r[4].s64 = ctx.r[11].s64 + 22704;
	// 8290D208: 484E6801  bl 0x82df3a08
	ctx.lr = 0x8290D20C;
	sub_82DF3A08(ctx, base);
	// 8290D20C: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 8290D210: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8290D214: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290D218: 419A0024  beq cr6, 0x8290d23c
	if ctx.cr[6].eq {
	pc = 0x8290D23C; continue 'dispatch;
	}
	// 8290D21C: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8290D220: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290D224: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290D228: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290D22C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290D230: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290D234: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290D238: 4082FFE8  bne 0x8290d220
	if !ctx.cr[0].eq {
	pc = 0x8290D220; continue 'dispatch;
	}
	// 8290D23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8290D240: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290D244: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8290D248: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290D24C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8290D250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D254: 4BEDEFB5  bl 0x827ec208
	ctx.lr = 0x8290D258;
	sub_827EC208(ctx, base);
	// 8290D258: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290D25C: 907E0318  stw r3, 0x318(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(792 as u32), ctx.r[3].u32 ) };
	// 8290D260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290D264: 419A000C  beq cr6, 0x8290d270
	if ctx.cr[6].eq {
	pc = 0x8290D270; continue 'dispatch;
	}
	// 8290D268: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8290D26C: 4B9B3625  bl 0x822c0890
	ctx.lr = 0x8290D270;
	sub_822C0890(ctx, base);
	// 8290D270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D274: 484E61B5  bl 0x82df3428
	ctx.lr = 0x8290D278;
	sub_82DF3428(ctx, base);
	// 8290D278: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290D27C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D280: 419A0008  beq cr6, 0x8290d288
	if ctx.cr[6].eq {
	pc = 0x8290D288; continue 'dispatch;
	}
	// 8290D284: 4B9B360D  bl 0x822c0890
	ctx.lr = 0x8290D288;
	sub_822C0890(ctx, base);
	// 8290D288: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290D28C: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290D290: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290D294: 48009D8D  bl 0x82917020
	ctx.lr = 0x8290D298;
	sub_82917020(ctx, base);
	// 8290D298: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290D29C: 419A00EC  beq cr6, 0x8290d388
	if ctx.cr[6].eq {
	pc = 0x8290D388; continue 'dispatch;
	}
	// 8290D2A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290D2A4: 4B9B35ED  bl 0x822c0890
	ctx.lr = 0x8290D2A8;
	sub_822C0890(ctx, base);
	// 8290D2A8: 480000E0  b 0x8290d388
	pc = 0x8290D388; continue 'dispatch;
	// 8290D2AC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8290D2B0: 409A0058  bne cr6, 0x8290d308
	if !ctx.cr[6].eq {
	pc = 0x8290D308; continue 'dispatch;
	}
	// 8290D2B4: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290D2B8: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290D2BC: 48009D75  bl 0x82917030
	ctx.lr = 0x8290D2C0;
	sub_82917030(ctx, base);
	// 8290D2C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D2C4: 418200C4  beq 0x8290d388
	if ctx.cr[0].eq {
	pc = 0x8290D388; continue 'dispatch;
	}
	// 8290D2C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D2CC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8290D2D0: 4BC021F9  bl 0x8250f4c8
	ctx.lr = 0x8290D2D4;
	sub_8250F4C8(ctx, base);
	// 8290D2D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D2D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290D2DC: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8290D2E0: 409A0008  bne cr6, 0x8290d2e8
	if !ctx.cr[6].eq {
	pc = 0x8290D2E8; continue 'dispatch;
	}
	// 8290D2E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290D2E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D2EC: 80BE0318  lwz r5, 0x318(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(792 as u32) ) } as u64;
	// 8290D2F0: 4BEDEFA1  bl 0x827ec290
	ctx.lr = 0x8290D2F4;
	sub_827EC290(ctx, base);
	// 8290D2F4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8290D2F8: 484E4999  bl 0x82df1c90
	ctx.lr = 0x8290D2FC;
	sub_82DF1C90(ctx, base);
	// 8290D2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290D300: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290D304: 4800007C  b 0x8290d380
	pc = 0x8290D380; continue 'dispatch;
	// 8290D308: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8290D30C: 409A000C  bne cr6, 0x8290d318
	if !ctx.cr[6].eq {
	pc = 0x8290D318; continue 'dispatch;
	}
	// 8290D310: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290D314: 48000068  b 0x8290d37c
	pc = 0x8290D37C; continue 'dispatch;
	// 8290D318: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8290D31C: 409A006C  bne cr6, 0x8290d388
	if !ctx.cr[6].eq {
	pc = 0x8290D388; continue 'dispatch;
	}
	// 8290D320: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290D324: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290D328: 48009D09  bl 0x82917030
	ctx.lr = 0x8290D32C;
	sub_82917030(ctx, base);
	// 8290D32C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D330: 41820048  beq 0x8290d378
	if ctx.cr[0].eq {
	pc = 0x8290D378; continue 'dispatch;
	}
	// 8290D334: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D338: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8290D33C: 4BC0218D  bl 0x8250f4c8
	ctx.lr = 0x8290D340;
	sub_8250F4C8(ctx, base);
	// 8290D340: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D344: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290D348: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8290D34C: 409A0008  bne cr6, 0x8290d354
	if !ctx.cr[6].eq {
	pc = 0x8290D354; continue 'dispatch;
	}
	// 8290D350: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290D354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D358: 80BE0318  lwz r5, 0x318(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(792 as u32) ) } as u64;
	// 8290D35C: 4BEDEF35  bl 0x827ec290
	ctx.lr = 0x8290D360;
	sub_827EC290(ctx, base);
	// 8290D360: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8290D364: 484E492D  bl 0x82df1c90
	ctx.lr = 0x8290D368;
	sub_82DF1C90(ctx, base);
	// 8290D368: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290D36C: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290D370: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290D374: 48009CAD  bl 0x82917020
	ctx.lr = 0x8290D378;
	sub_82917020(ctx, base);
	// 8290D378: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290D37C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8290D380: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290D384: 48009C9D  bl 0x82917020
	ctx.lr = 0x8290D388;
	sub_82917020(ctx, base);
	// 8290D388: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8290D38C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8290D390: 4889AE2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290D398 size=404
    let mut pc: u32 = 0x8290D398;
    'dispatch: loop {
        match pc {
            0x8290D398 => {
    //   block [0x8290D398..0x8290D52C)
	// 8290D398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D39C: 4889ADCD  bl 0x831a8168
	ctx.lr = 0x8290D3A0;
	sub_831A8130(ctx, base);
	// 8290D3A0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D3A4: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290D3A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290D3AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290D3B0: 41820174  beq 0x8290d524
	if ctx.cr[0].eq {
	pc = 0x8290D524; continue 'dispatch;
	}
	// 8290D3B4: 817E031C  lwz r11, 0x31c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(796 as u32) ) } as u64;
	// 8290D3B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290D3BC: 409A0168  bne cr6, 0x8290d524
	if !ctx.cr[6].eq {
	pc = 0x8290D524; continue 'dispatch;
	}
	// 8290D3C0: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290D3C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D3C8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8290D3CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D3D0: 4E800421  bctrl
	ctx.lr = 0x8290D3D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D3D4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8290D3D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290D3DC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8290D3E0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290D3E4: C1AA0000  lfs f13, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290D3E8: C06B08A4  lfs f3, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8290D3EC: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290D3F0: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 8290D3F4: EC4D002A  fadds f2, f13, f0
	ctx.f[2].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8290D3F8: 4856EC21  bl 0x82e7c018
	ctx.lr = 0x8290D3FC;
	sub_82E7C018(ctx, base);
	// 8290D3FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290D400: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290D404: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290D408: 38A009F2  li r5, 0x9f2
	ctx.r[5].s64 = 2546;
	// 8290D40C: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 8290D410: 484E4FD9  bl 0x82df23e8
	ctx.lr = 0x8290D414;
	sub_82DF23E8(ctx, base);
	// 8290D414: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290D418: 41820018  beq 0x8290d430
	if ctx.cr[0].eq {
	pc = 0x8290D430; continue 'dispatch;
	}
	// 8290D41C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290D420: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8290D424: 4BC7D63D  bl 0x8258aa60
	ctx.lr = 0x8290D428;
	sub_8258AA60(ctx, base);
	// 8290D428: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290D42C: 48000008  b 0x8290d434
	pc = 0x8290D434; continue 'dispatch;
	// 8290D430: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290D434: 3BFE0320  addi r31, r30, 0x320
	ctx.r[31].s64 = ctx.r[30].s64 + 800;
	// 8290D438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D43C: 4BFFF5E5  bl 0x8290ca20
	ctx.lr = 0x8290D440;
	sub_8290CA20(ctx, base);
	// 8290D440: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D444: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290D448: 83BE0320  lwz r29, 0x320(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(800 as u32) ) } as u64;
	// 8290D44C: 4819E89D  bl 0x82aabce8
	ctx.lr = 0x8290D450;
	sub_82AABCE8(ctx, base);
	// 8290D450: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290D454: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D458: 485059D9  bl 0x82e12e30
	ctx.lr = 0x8290D45C;
	sub_82E12E30(ctx, base);
	// 8290D45C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290D460: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D464: 419A0008  beq cr6, 0x8290d46c
	if ctx.cr[6].eq {
	pc = 0x8290D46C; continue 'dispatch;
	}
	// 8290D468: 4B9B3429  bl 0x822c0890
	ctx.lr = 0x8290D46C;
	sub_822C0890(ctx, base);
	// 8290D46C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D470: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8290D474: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8290D478: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8290D47C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8290D480: 485932B1  bl 0x82ea0730
	ctx.lr = 0x8290D484;
	sub_82EA0730(ctx, base);
	// 8290D484: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8290D488: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8290D48C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8290D490: C02A964C  lfs f1, -0x69b4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290D494: 4860CD5D  bl 0x82f1a1f0
	ctx.lr = 0x8290D498;
	sub_82F1A1F0(ctx, base);
	// 8290D498: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290D49C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D4A0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8290D4A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290D4A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290D4AC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8290D4B0: 419A0024  beq cr6, 0x8290d4d4
	if ctx.cr[6].eq {
	pc = 0x8290D4D4; continue 'dispatch;
	}
	// 8290D4B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290D4B8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290D4BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290D4C0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290D4C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290D4C8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290D4CC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290D4D0: 4082FFE8  bne 0x8290d4b8
	if !ctx.cr[0].eq {
	pc = 0x8290D4B8; continue 'dispatch;
	}
	// 8290D4D4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8290D4D8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290D4DC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290D4E0: 388A5748  addi r4, r10, 0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + 22344;
	// 8290D4E4: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 8290D4E8: 83EB6710  lwz r31, 0x6710(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26384 as u32) ) } as u64;
	// 8290D4EC: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 8290D4F0: 484EC041  bl 0x82df9530
	ctx.lr = 0x8290D4F4;
	sub_82DF9530(ctx, base);
	// 8290D4F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290D4F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D4FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8290D500: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8290D504: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8290D508: 4BC05BF1  bl 0x825130f8
	ctx.lr = 0x8290D50C;
	sub_825130F8(ctx, base);
	// 8290D50C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290D510: 907E031C  stw r3, 0x31c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(796 as u32), ctx.r[3].u32 ) };
	// 8290D514: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290D518: 419A000C  beq cr6, 0x8290d524
	if ctx.cr[6].eq {
	pc = 0x8290D524; continue 'dispatch;
	}
	// 8290D51C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8290D520: 4B9DAD49  bl 0x822e8268
	ctx.lr = 0x8290D524;
	sub_822E8268(ctx, base);
	// 8290D524: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8290D528: 4889AC90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D530 size=120
    let mut pc: u32 = 0x8290D530;
    'dispatch: loop {
        match pc {
            0x8290D530 => {
    //   block [0x8290D530..0x8290D5A8)
	// 8290D530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D53C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D544: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D548: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290D54C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D550: 4E800421  bctrl
	ctx.lr = 0x8290D554;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D554: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D558: 4182003C  beq 0x8290d594
	if ctx.cr[0].eq {
	pc = 0x8290D594; continue 'dispatch;
	}
	// 8290D55C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D560: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D564: 484E64A5  bl 0x82df3a08
	ctx.lr = 0x8290D568;
	sub_82DF3A08(ctx, base);
	// 8290D568: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D570: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8290D574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D578: 4E800421  bctrl
	ctx.lr = 0x8290D57C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D57C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290D580: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290D584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D588: 4BFFF0F9  bl 0x8290c680
	ctx.lr = 0x8290D58C;
	sub_8290C680(ctx, base);
	// 8290D58C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D590: 484E5E99  bl 0x82df3428
	ctx.lr = 0x8290D594;
	sub_82DF3428(ctx, base);
	// 8290D594: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D5A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D5A8 size=120
    let mut pc: u32 = 0x8290D5A8;
    'dispatch: loop {
        match pc {
            0x8290D5A8 => {
    //   block [0x8290D5A8..0x8290D620)
	// 8290D5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D5B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D5B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D5B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D5BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D5C0: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290D5C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D5C8: 4E800421  bctrl
	ctx.lr = 0x8290D5CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D5CC: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D5D0: 4182003C  beq 0x8290d60c
	if ctx.cr[0].eq {
	pc = 0x8290D60C; continue 'dispatch;
	}
	// 8290D5D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D5D8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D5DC: 484E642D  bl 0x82df3a08
	ctx.lr = 0x8290D5E0;
	sub_82DF3A08(ctx, base);
	// 8290D5E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D5E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D5E8: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8290D5EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D5F0: 4E800421  bctrl
	ctx.lr = 0x8290D5F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D5F4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8290D5F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290D5FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D600: 4BFFF081  bl 0x8290c680
	ctx.lr = 0x8290D604;
	sub_8290C680(ctx, base);
	// 8290D604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D608: 484E5E21  bl 0x82df3428
	ctx.lr = 0x8290D60C;
	sub_82DF3428(ctx, base);
	// 8290D60C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D620 size=120
    let mut pc: u32 = 0x8290D620;
    'dispatch: loop {
        match pc {
            0x8290D620 => {
    //   block [0x8290D620..0x8290D698)
	// 8290D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D62C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D634: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D638: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290D63C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D640: 4E800421  bctrl
	ctx.lr = 0x8290D644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D644: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D648: 4182003C  beq 0x8290d684
	if ctx.cr[0].eq {
	pc = 0x8290D684; continue 'dispatch;
	}
	// 8290D64C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D650: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D654: 484E63B5  bl 0x82df3a08
	ctx.lr = 0x8290D658;
	sub_82DF3A08(ctx, base);
	// 8290D658: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D65C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D660: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8290D664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D668: 4E800421  bctrl
	ctx.lr = 0x8290D66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D66C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8290D670: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290D674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D678: 4BFFF009  bl 0x8290c680
	ctx.lr = 0x8290D67C;
	sub_8290C680(ctx, base);
	// 8290D67C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D680: 484E5DA9  bl 0x82df3428
	ctx.lr = 0x8290D684;
	sub_82DF3428(ctx, base);
	// 8290D684: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D698 size=120
    let mut pc: u32 = 0x8290D698;
    'dispatch: loop {
        match pc {
            0x8290D698 => {
    //   block [0x8290D698..0x8290D710)
	// 8290D698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D6A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D6A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D6A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D6AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D6B0: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290D6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D6B8: 4E800421  bctrl
	ctx.lr = 0x8290D6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D6BC: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D6C0: 4182003C  beq 0x8290d6fc
	if ctx.cr[0].eq {
	pc = 0x8290D6FC; continue 'dispatch;
	}
	// 8290D6C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D6C8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D6CC: 484E633D  bl 0x82df3a08
	ctx.lr = 0x8290D6D0;
	sub_82DF3A08(ctx, base);
	// 8290D6D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D6D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D6D8: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8290D6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D6E0: 4E800421  bctrl
	ctx.lr = 0x8290D6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D6E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290D6E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290D6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D6F0: 4BFFF031  bl 0x8290c720
	ctx.lr = 0x8290D6F4;
	sub_8290C720(ctx, base);
	// 8290D6F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D6F8: 484E5D31  bl 0x82df3428
	ctx.lr = 0x8290D6FC;
	sub_82DF3428(ctx, base);
	// 8290D6FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D710 size=120
    let mut pc: u32 = 0x8290D710;
    'dispatch: loop {
        match pc {
            0x8290D710 => {
    //   block [0x8290D710..0x8290D788)
	// 8290D710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D718: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D71C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D720: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D724: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D728: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290D72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D730: 4E800421  bctrl
	ctx.lr = 0x8290D734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D734: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D738: 4182003C  beq 0x8290d774
	if ctx.cr[0].eq {
	pc = 0x8290D774; continue 'dispatch;
	}
	// 8290D73C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D740: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D744: 484E62C5  bl 0x82df3a08
	ctx.lr = 0x8290D748;
	sub_82DF3A08(ctx, base);
	// 8290D748: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D74C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D750: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8290D754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D758: 4E800421  bctrl
	ctx.lr = 0x8290D75C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D75C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8290D760: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290D764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D768: 4BFFEFB9  bl 0x8290c720
	ctx.lr = 0x8290D76C;
	sub_8290C720(ctx, base);
	// 8290D76C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D770: 484E5CB9  bl 0x82df3428
	ctx.lr = 0x8290D774;
	sub_82DF3428(ctx, base);
	// 8290D774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D788 size=120
    let mut pc: u32 = 0x8290D788;
    'dispatch: loop {
        match pc {
            0x8290D788 => {
    //   block [0x8290D788..0x8290D800)
	// 8290D788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D794: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D79C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D7A0: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290D7A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D7A8: 4E800421  bctrl
	ctx.lr = 0x8290D7AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D7AC: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290D7B0: 4182003C  beq 0x8290d7ec
	if ctx.cr[0].eq {
	pc = 0x8290D7EC; continue 'dispatch;
	}
	// 8290D7B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D7B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D7BC: 484E624D  bl 0x82df3a08
	ctx.lr = 0x8290D7C0;
	sub_82DF3A08(ctx, base);
	// 8290D7C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290D7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D7C8: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8290D7CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290D7D0: 4E800421  bctrl
	ctx.lr = 0x8290D7D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290D7D4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8290D7D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290D7DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290D7E0: 4BFFEF41  bl 0x8290c720
	ctx.lr = 0x8290D7E4;
	sub_8290C720(ctx, base);
	// 8290D7E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290D7E8: 484E5C41  bl 0x82df3428
	ctx.lr = 0x8290D7EC;
	sub_82DF3428(ctx, base);
	// 8290D7EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D7F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D800 size=112
    let mut pc: u32 = 0x8290D800;
    'dispatch: loop {
        match pc {
            0x8290D800 => {
    //   block [0x8290D800..0x8290D870)
	// 8290D800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290D80C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D814: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290D818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D81C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8290D820: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290D824: 4BFFD0B5  bl 0x8290a8d8
	ctx.lr = 0x8290D828;
	sub_8290A8D8(ctx, base);
	// 8290D828: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290D82C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D830: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290D834: 4B9B27CD  bl 0x822c0000
	ctx.lr = 0x8290D838;
	sub_822C0000(ctx, base);
	// 8290D838: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290D83C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290D840: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290D844: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290D848: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D84C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290D850: 419A0008  beq cr6, 0x8290d858
	if ctx.cr[6].eq {
	pc = 0x8290D858; continue 'dispatch;
	}
	// 8290D854: 4B9B303D  bl 0x822c0890
	ctx.lr = 0x8290D858;
	sub_822C0890(ctx, base);
	// 8290D858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D864: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290D868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D870 size=112
    let mut pc: u32 = 0x8290D870;
    'dispatch: loop {
        match pc {
            0x8290D870 => {
    //   block [0x8290D870..0x8290D8E0)
	// 8290D870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D878: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290D87C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D884: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290D888: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D88C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8290D890: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290D894: 4BFFD10D  bl 0x8290a9a0
	ctx.lr = 0x8290D898;
	sub_8290A9A0(ctx, base);
	// 8290D898: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290D89C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D8A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290D8A4: 4B9B275D  bl 0x822c0000
	ctx.lr = 0x8290D8A8;
	sub_822C0000(ctx, base);
	// 8290D8A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290D8AC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290D8B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290D8B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290D8B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D8BC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290D8C0: 419A0008  beq cr6, 0x8290d8c8
	if ctx.cr[6].eq {
	pc = 0x8290D8C8; continue 'dispatch;
	}
	// 8290D8C4: 4B9B2FCD  bl 0x822c0890
	ctx.lr = 0x8290D8C8;
	sub_822C0890(ctx, base);
	// 8290D8C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D8D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290D8D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D8E0 size=112
    let mut pc: u32 = 0x8290D8E0;
    'dispatch: loop {
        match pc {
            0x8290D8E0 => {
    //   block [0x8290D8E0..0x8290D950)
	// 8290D8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290D8E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290D8EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290D8F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D8F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290D8F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D8FC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8290D900: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290D904: 4BFFD165  bl 0x8290aa68
	ctx.lr = 0x8290D908;
	sub_8290AA68(ctx, base);
	// 8290D908: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290D90C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290D910: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290D914: 4B9B26ED  bl 0x822c0000
	ctx.lr = 0x8290D918;
	sub_822C0000(ctx, base);
	// 8290D918: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290D91C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290D920: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290D924: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290D928: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290D92C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290D930: 419A0008  beq cr6, 0x8290d938
	if ctx.cr[6].eq {
	pc = 0x8290D938; continue 'dispatch;
	}
	// 8290D934: 4B9B2F5D  bl 0x822c0890
	ctx.lr = 0x8290D938;
	sub_822C0890(ctx, base);
	// 8290D938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290D940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290D944: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290D948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290D94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D950 size=120
    let mut pc: u32 = 0x8290D950;
    'dispatch: loop {
        match pc {
            0x8290D950 => {
    //   block [0x8290D950..0x8290D9C8)
	// 8290D950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D954: 4889A819  bl 0x831a816c
	ctx.lr = 0x8290D958;
	sub_831A8130(ctx, base);
	// 8290D958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D95C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290D960: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290D964: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290D968: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290D96C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290D970: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8290D974: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8290D978: 484E4A71  bl 0x82df23e8
	ctx.lr = 0x8290D97C;
	sub_82DF23E8(ctx, base);
	// 8290D97C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290D980: 41820014  beq 0x8290d994
	if ctx.cr[0].eq {
	pc = 0x8290D994; continue 'dispatch;
	}
	// 8290D984: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290D988: 48007EB9  bl 0x82915840
	ctx.lr = 0x8290D98C;
	sub_82915840(ctx, base);
	// 8290D98C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290D990: 48000008  b 0x8290d998
	pc = 0x8290D998; continue 'dispatch;
	// 8290D994: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290D998: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290D99C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290D9A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290D9A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D9A8: 4BFFD189  bl 0x8290ab30
	ctx.lr = 0x8290D9AC;
	sub_8290AB30(ctx, base);
	// 8290D9AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290D9B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290D9B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290D9B8: 4B9B2649  bl 0x822c0000
	ctx.lr = 0x8290D9BC;
	sub_822C0000(ctx, base);
	// 8290D9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290D9C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290D9C4: 4889A7F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290D9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290D9C8 size=120
    let mut pc: u32 = 0x8290D9C8;
    'dispatch: loop {
        match pc {
            0x8290D9C8 => {
    //   block [0x8290D9C8..0x8290DA40)
	// 8290D9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290D9CC: 4889A7A1  bl 0x831a816c
	ctx.lr = 0x8290D9D0;
	sub_831A8130(ctx, base);
	// 8290D9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290D9D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290D9D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290D9DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290D9E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290D9E4: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290D9E8: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8290D9EC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8290D9F0: 484E49F9  bl 0x82df23e8
	ctx.lr = 0x8290D9F4;
	sub_82DF23E8(ctx, base);
	// 8290D9F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290D9F8: 41820014  beq 0x8290da0c
	if ctx.cr[0].eq {
	pc = 0x8290DA0C; continue 'dispatch;
	}
	// 8290D9FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DA00: 48047409  bl 0x82954e08
	ctx.lr = 0x8290DA04;
	sub_82954E08(ctx, base);
	// 8290DA04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DA08: 48000008  b 0x8290da10
	pc = 0x8290DA10; continue 'dispatch;
	// 8290DA0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DA10: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DA14: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DA18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DA1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DA20: 4BFFD1D9  bl 0x8290abf8
	ctx.lr = 0x8290DA24;
	sub_8290ABF8(ctx, base);
	// 8290DA24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DA28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DA2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DA30: 4B9B25D1  bl 0x822c0000
	ctx.lr = 0x8290DA34;
	sub_822C0000(ctx, base);
	// 8290DA34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DA38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DA3C: 4889A780  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DA40 size=112
    let mut pc: u32 = 0x8290DA40;
    'dispatch: loop {
        match pc {
            0x8290DA40 => {
    //   block [0x8290DA40..0x8290DAB0)
	// 8290DA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DA44: 4889A729  bl 0x831a816c
	ctx.lr = 0x8290DA48;
	sub_831A8130(ctx, base);
	// 8290DA48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DA4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DA50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DA58: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DA5C: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 8290DA60: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8290DA64: 484E4985  bl 0x82df23e8
	ctx.lr = 0x8290DA68;
	sub_82DF23E8(ctx, base);
	// 8290DA68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DA6C: 41820010  beq 0x8290da7c
	if ctx.cr[0].eq {
	pc = 0x8290DA7C; continue 'dispatch;
	}
	// 8290DA70: 48007BE9  bl 0x82915658
	ctx.lr = 0x8290DA74;
	sub_82915658(ctx, base);
	// 8290DA74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DA78: 48000008  b 0x8290da80
	pc = 0x8290DA80; continue 'dispatch;
	// 8290DA7C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DA80: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DA84: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DA88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DA8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DA90: 4BFFD231  bl 0x8290acc0
	ctx.lr = 0x8290DA94;
	sub_8290ACC0(ctx, base);
	// 8290DA94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DA98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DA9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DAA0: 4B9B2561  bl 0x822c0000
	ctx.lr = 0x8290DAA4;
	sub_822C0000(ctx, base);
	// 8290DAA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DAA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DAAC: 4889A710  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DAB0 size=112
    let mut pc: u32 = 0x8290DAB0;
    'dispatch: loop {
        match pc {
            0x8290DAB0 => {
    //   block [0x8290DAB0..0x8290DB20)
	// 8290DAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DAB4: 4889A6B9  bl 0x831a816c
	ctx.lr = 0x8290DAB8;
	sub_831A8130(ctx, base);
	// 8290DAB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DABC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DAC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DAC8: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DACC: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 8290DAD0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8290DAD4: 484E4915  bl 0x82df23e8
	ctx.lr = 0x8290DAD8;
	sub_82DF23E8(ctx, base);
	// 8290DAD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DADC: 41820010  beq 0x8290daec
	if ctx.cr[0].eq {
	pc = 0x8290DAEC; continue 'dispatch;
	}
	// 8290DAE0: 48039331  bl 0x82946e10
	ctx.lr = 0x8290DAE4;
	sub_82946E10(ctx, base);
	// 8290DAE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DAE8: 48000008  b 0x8290daf0
	pc = 0x8290DAF0; continue 'dispatch;
	// 8290DAEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DAF0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DAF4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DAF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DAFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DB00: 4BFFD289  bl 0x8290ad88
	ctx.lr = 0x8290DB04;
	sub_8290AD88(ctx, base);
	// 8290DB04: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DB08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DB0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DB10: 4B9B24F1  bl 0x822c0000
	ctx.lr = 0x8290DB14;
	sub_822C0000(ctx, base);
	// 8290DB14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DB18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DB1C: 4889A6A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DB20 size=112
    let mut pc: u32 = 0x8290DB20;
    'dispatch: loop {
        match pc {
            0x8290DB20 => {
    //   block [0x8290DB20..0x8290DB90)
	// 8290DB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DB24: 4889A649  bl 0x831a816c
	ctx.lr = 0x8290DB28;
	sub_831A8130(ctx, base);
	// 8290DB28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DB2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DB30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DB38: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DB3C: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 8290DB40: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8290DB44: 484E48A5  bl 0x82df23e8
	ctx.lr = 0x8290DB48;
	sub_82DF23E8(ctx, base);
	// 8290DB48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DB4C: 41820010  beq 0x8290db5c
	if ctx.cr[0].eq {
	pc = 0x8290DB5C; continue 'dispatch;
	}
	// 8290DB50: 48007B49  bl 0x82915698
	ctx.lr = 0x8290DB54;
	sub_82915698(ctx, base);
	// 8290DB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DB58: 48000008  b 0x8290db60
	pc = 0x8290DB60; continue 'dispatch;
	// 8290DB5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DB60: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DB64: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DB68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DB6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DB70: 4BFFD2E1  bl 0x8290ae50
	ctx.lr = 0x8290DB74;
	sub_8290AE50(ctx, base);
	// 8290DB74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DB78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DB7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DB80: 4B9B2481  bl 0x822c0000
	ctx.lr = 0x8290DB84;
	sub_822C0000(ctx, base);
	// 8290DB84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DB88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DB8C: 4889A630  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DB90 size=112
    let mut pc: u32 = 0x8290DB90;
    'dispatch: loop {
        match pc {
            0x8290DB90 => {
    //   block [0x8290DB90..0x8290DC00)
	// 8290DB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DB94: 4889A5D9  bl 0x831a816c
	ctx.lr = 0x8290DB98;
	sub_831A8130(ctx, base);
	// 8290DB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DB9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DBA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DBA8: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DBAC: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 8290DBB0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8290DBB4: 484E4835  bl 0x82df23e8
	ctx.lr = 0x8290DBB8;
	sub_82DF23E8(ctx, base);
	// 8290DBB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DBBC: 41820010  beq 0x8290dbcc
	if ctx.cr[0].eq {
	pc = 0x8290DBCC; continue 'dispatch;
	}
	// 8290DBC0: 48039211  bl 0x82946dd0
	ctx.lr = 0x8290DBC4;
	sub_82946DD0(ctx, base);
	// 8290DBC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DBC8: 48000008  b 0x8290dbd0
	pc = 0x8290DBD0; continue 'dispatch;
	// 8290DBCC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DBD0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DBD4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DBD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DBDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DBE0: 4BFFD339  bl 0x8290af18
	ctx.lr = 0x8290DBE4;
	sub_8290AF18(ctx, base);
	// 8290DBE4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DBE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DBEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DBF0: 4B9B2411  bl 0x822c0000
	ctx.lr = 0x8290DBF4;
	sub_822C0000(ctx, base);
	// 8290DBF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DBF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DBFC: 4889A5C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DC00 size=128
    let mut pc: u32 = 0x8290DC00;
    'dispatch: loop {
        match pc {
            0x8290DC00 => {
    //   block [0x8290DC00..0x8290DC80)
	// 8290DC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DC04: 4889A569  bl 0x831a816c
	ctx.lr = 0x8290DC08;
	sub_831A8130(ctx, base);
	// 8290DC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DC0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DC10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DC14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290DC18: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8290DC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DC20: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DC24: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8290DC28: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8290DC2C: 484E47BD  bl 0x82df23e8
	ctx.lr = 0x8290DC30;
	sub_82DF23E8(ctx, base);
	// 8290DC30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DC34: 41820018  beq 0x8290dc4c
	if ctx.cr[0].eq {
	pc = 0x8290DC4C; continue 'dispatch;
	}
	// 8290DC38: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290DC3C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290DC40: 48245C71  bl 0x82b538b0
	ctx.lr = 0x8290DC44;
	sub_82B538B0(ctx, base);
	// 8290DC44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DC48: 48000008  b 0x8290dc50
	pc = 0x8290DC50; continue 'dispatch;
	// 8290DC4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DC50: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DC54: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DC58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DC5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DC60: 4BFFD381  bl 0x8290afe0
	ctx.lr = 0x8290DC64;
	sub_8290AFE0(ctx, base);
	// 8290DC64: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DC68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DC6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DC70: 4B9B2391  bl 0x822c0000
	ctx.lr = 0x8290DC74;
	sub_822C0000(ctx, base);
	// 8290DC74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DC78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DC7C: 4889A540  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290DC80 size=128
    let mut pc: u32 = 0x8290DC80;
    'dispatch: loop {
        match pc {
            0x8290DC80 => {
    //   block [0x8290DC80..0x8290DD00)
	// 8290DC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DC84: 4889A4E9  bl 0x831a816c
	ctx.lr = 0x8290DC88;
	sub_831A8130(ctx, base);
	// 8290DC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DC8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DC90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DC94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290DC98: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8290DC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DCA0: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DCA4: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8290DCA8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8290DCAC: 484E473D  bl 0x82df23e8
	ctx.lr = 0x8290DCB0;
	sub_82DF23E8(ctx, base);
	// 8290DCB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DCB4: 41820018  beq 0x8290dccc
	if ctx.cr[0].eq {
	pc = 0x8290DCCC; continue 'dispatch;
	}
	// 8290DCB8: C05E0000  lfs f2, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8290DCBC: C03F0000  lfs f1, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290DCC0: 48008159  bl 0x82915e18
	ctx.lr = 0x8290DCC4;
	sub_82915E18(ctx, base);
	// 8290DCC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DCC8: 48000008  b 0x8290dcd0
	pc = 0x8290DCD0; continue 'dispatch;
	// 8290DCCC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DCD0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DCD4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DCD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DCDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DCE0: 4BFFD3C9  bl 0x8290b0a8
	ctx.lr = 0x8290DCE4;
	sub_8290B0A8(ctx, base);
	// 8290DCE4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DCE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DCEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DCF0: 4B9B2311  bl 0x822c0000
	ctx.lr = 0x8290DCF4;
	sub_822C0000(ctx, base);
	// 8290DCF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DCF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DCFC: 4889A4C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DD00 size=112
    let mut pc: u32 = 0x8290DD00;
    'dispatch: loop {
        match pc {
            0x8290DD00 => {
    //   block [0x8290DD00..0x8290DD70)
	// 8290DD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290DD08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290DD0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290DD10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DD14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290DD18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DD1C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8290DD20: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290DD24: 4BFFD44D  bl 0x8290b170
	ctx.lr = 0x8290DD28;
	sub_8290B170(ctx, base);
	// 8290DD28: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8290DD2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290DD30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290DD34: 4B9B22CD  bl 0x822c0000
	ctx.lr = 0x8290DD38;
	sub_822C0000(ctx, base);
	// 8290DD38: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8290DD3C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290DD40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8290DD44: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290DD48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290DD4C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8290DD50: 419A0008  beq cr6, 0x8290dd58
	if ctx.cr[6].eq {
	pc = 0x8290DD58; continue 'dispatch;
	}
	// 8290DD54: 4B9B2B3D  bl 0x822c0890
	ctx.lr = 0x8290DD58;
	sub_822C0890(ctx, base);
	// 8290DD58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290DD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290DD64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290DD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290DD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DD70 size=112
    let mut pc: u32 = 0x8290DD70;
    'dispatch: loop {
        match pc {
            0x8290DD70 => {
    //   block [0x8290DD70..0x8290DDE0)
	// 8290DD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DD74: 4889A3F9  bl 0x831a816c
	ctx.lr = 0x8290DD78;
	sub_831A8130(ctx, base);
	// 8290DD78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DD7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DD80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DD84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DD88: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DD8C: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 8290DD90: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8290DD94: 484E4655  bl 0x82df23e8
	ctx.lr = 0x8290DD98;
	sub_82DF23E8(ctx, base);
	// 8290DD98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DD9C: 41820010  beq 0x8290ddac
	if ctx.cr[0].eq {
	pc = 0x8290DDAC; continue 'dispatch;
	}
	// 8290DDA0: 48007D89  bl 0x82915b28
	ctx.lr = 0x8290DDA4;
	sub_82915B28(ctx, base);
	// 8290DDA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DDA8: 48000008  b 0x8290ddb0
	pc = 0x8290DDB0; continue 'dispatch;
	// 8290DDAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DDB0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DDB4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DDB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DDBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DDC0: 4BFFD541  bl 0x8290b300
	ctx.lr = 0x8290DDC4;
	sub_8290B300(ctx, base);
	// 8290DDC4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DDC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DDCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DDD0: 4B9B2231  bl 0x822c0000
	ctx.lr = 0x8290DDD4;
	sub_822C0000(ctx, base);
	// 8290DDD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DDD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DDDC: 4889A3E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290DDE0 size=112
    let mut pc: u32 = 0x8290DDE0;
    'dispatch: loop {
        match pc {
            0x8290DDE0 => {
    //   block [0x8290DDE0..0x8290DE50)
	// 8290DDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DDE4: 4889A389  bl 0x831a816c
	ctx.lr = 0x8290DDE8;
	sub_831A8130(ctx, base);
	// 8290DDE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DDEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8290DDF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290DDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290DDF8: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8290DDFC: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 8290DE00: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8290DE04: 484E45E5  bl 0x82df23e8
	ctx.lr = 0x8290DE08;
	sub_82DF23E8(ctx, base);
	// 8290DE08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290DE0C: 41820010  beq 0x8290de1c
	if ctx.cr[0].eq {
	pc = 0x8290DE1C; continue 'dispatch;
	}
	// 8290DE10: 48037E59  bl 0x82945c68
	ctx.lr = 0x8290DE14;
	sub_82945C68(ctx, base);
	// 8290DE14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DE18: 48000008  b 0x8290de20
	pc = 0x8290DE20; continue 'dispatch;
	// 8290DE1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290DE20: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8290DE24: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8290DE28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DE2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DE30: 4BFFD599  bl 0x8290b3c8
	ctx.lr = 0x8290DE34;
	sub_8290B3C8(ctx, base);
	// 8290DE34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8290DE38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290DE3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290DE40: 4B9B21C1  bl 0x822c0000
	ctx.lr = 0x8290DE44;
	sub_822C0000(ctx, base);
	// 8290DE44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290DE48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290DE4C: 4889A370  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290DE50 size=212
    let mut pc: u32 = 0x8290DE50;
    'dispatch: loop {
        match pc {
            0x8290DE50 => {
    //   block [0x8290DE50..0x8290DF24)
	// 8290DE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DE54: 4889A319  bl 0x831a816c
	ctx.lr = 0x8290DE58;
	sub_831A8130(ctx, base);
	// 8290DE58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DE5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290DE60: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290DE64: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8290DE68: 4BC01661  bl 0x8250f4c8
	ctx.lr = 0x8290DE6C;
	sub_8250F4C8(ctx, base);
	// 8290DE6C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290DE70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290DE74: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8290DE78: 409A0008  bne cr6, 0x8290de80
	if !ctx.cr[6].eq {
	pc = 0x8290DE80; continue 'dispatch;
	}
	// 8290DE7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290DE80: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8290DE84: 4BBFA7FD  bl 0x82508680
	ctx.lr = 0x8290DE88;
	sub_82508680(ctx, base);
	// 8290DE88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DE8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290DE90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290DE94: 4BFFFABD  bl 0x8290d950
	ctx.lr = 0x8290DE98;
	sub_8290D950(ctx, base);
	// 8290DE98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290DE9C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290DEA0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290DEA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290DEA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290DEAC: 419A0024  beq cr6, 0x8290ded0
	if ctx.cr[6].eq {
	pc = 0x8290DED0; continue 'dispatch;
	}
	// 8290DEB0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290DEB4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290DEB8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290DEBC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290DEC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290DEC4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290DEC8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290DECC: 4082FFE8  bne 0x8290deb4
	if !ctx.cr[0].eq {
	pc = 0x8290DEB4; continue 'dispatch;
	}
	// 8290DED0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290DED4: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290DED8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290DEDC: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290DEE0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8290DEE4: 38A001BF  li r5, 0x1bf
	ctx.r[5].s64 = 447;
	// 8290DEE8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290DEEC: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 8290DEF0: 4854AB51  bl 0x82e58a40
	ctx.lr = 0x8290DEF4;
	sub_82E58A40(ctx, base);
	// 8290DEF4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290DEF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290DEFC: 419A0008  beq cr6, 0x8290df04
	if ctx.cr[6].eq {
	pc = 0x8290DF04; continue 'dispatch;
	}
	// 8290DF00: 4B9B2991  bl 0x822c0890
	ctx.lr = 0x8290DF04;
	sub_822C0890(ctx, base);
	// 8290DF04: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290DF08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290DF0C: 419A0008  beq cr6, 0x8290df14
	if ctx.cr[6].eq {
	pc = 0x8290DF14; continue 'dispatch;
	}
	// 8290DF10: 4B9B2981  bl 0x822c0890
	ctx.lr = 0x8290DF14;
	sub_822C0890(ctx, base);
	// 8290DF14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290DF18: 484E3D79  bl 0x82df1c90
	ctx.lr = 0x8290DF1C;
	sub_82DF1C90(ctx, base);
	// 8290DF1C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8290DF20: 4889A29C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290DF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290DF28 size=212
    let mut pc: u32 = 0x8290DF28;
    'dispatch: loop {
        match pc {
            0x8290DF28 => {
    //   block [0x8290DF28..0x8290DFFC)
	// 8290DF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290DF2C: 4889A241  bl 0x831a816c
	ctx.lr = 0x8290DF30;
	sub_831A8130(ctx, base);
	// 8290DF30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290DF34: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290DF38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8290DF3C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8290DF40: 4BC01589  bl 0x8250f4c8
	ctx.lr = 0x8290DF44;
	sub_8250F4C8(ctx, base);
	// 8290DF44: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290DF48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290DF4C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8290DF50: 409A0008  bne cr6, 0x8290df58
	if !ctx.cr[6].eq {
	pc = 0x8290DF58; continue 'dispatch;
	}
	// 8290DF54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290DF58: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8290DF5C: 4BBFA725  bl 0x82508680
	ctx.lr = 0x8290DF60;
	sub_82508680(ctx, base);
	// 8290DF60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290DF64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290DF68: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290DF6C: 4BFFFA5D  bl 0x8290d9c8
	ctx.lr = 0x8290DF70;
	sub_8290D9C8(ctx, base);
	// 8290DF70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290DF74: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290DF78: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290DF7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290DF80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290DF84: 419A0024  beq cr6, 0x8290dfa8
	if ctx.cr[6].eq {
	pc = 0x8290DFA8; continue 'dispatch;
	}
	// 8290DF88: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290DF8C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290DF90: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290DF94: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290DF98: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290DF9C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290DFA0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290DFA4: 4082FFE8  bne 0x8290df8c
	if !ctx.cr[0].eq {
	pc = 0x8290DF8C; continue 'dispatch;
	}
	// 8290DFA8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290DFAC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290DFB0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290DFB4: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290DFB8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8290DFBC: 38A001C7  li r5, 0x1c7
	ctx.r[5].s64 = 455;
	// 8290DFC0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290DFC4: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 8290DFC8: 4854AA79  bl 0x82e58a40
	ctx.lr = 0x8290DFCC;
	sub_82E58A40(ctx, base);
	// 8290DFCC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290DFD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290DFD4: 419A0008  beq cr6, 0x8290dfdc
	if ctx.cr[6].eq {
	pc = 0x8290DFDC; continue 'dispatch;
	}
	// 8290DFD8: 4B9B28B9  bl 0x822c0890
	ctx.lr = 0x8290DFDC;
	sub_822C0890(ctx, base);
	// 8290DFDC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290DFE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290DFE4: 419A0008  beq cr6, 0x8290dfec
	if ctx.cr[6].eq {
	pc = 0x8290DFEC; continue 'dispatch;
	}
	// 8290DFE8: 4B9B28A9  bl 0x822c0890
	ctx.lr = 0x8290DFEC;
	sub_822C0890(ctx, base);
	// 8290DFEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290DFF0: 484E3CA1  bl 0x82df1c90
	ctx.lr = 0x8290DFF4;
	sub_82DF1C90(ctx, base);
	// 8290DFF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8290DFF8: 4889A1C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290E000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290E000 size=300
    let mut pc: u32 = 0x8290E000;
    'dispatch: loop {
        match pc {
            0x8290E000 => {
    //   block [0x8290E000..0x8290E12C)
	// 8290E000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290E004: 4889A169  bl 0x831a816c
	ctx.lr = 0x8290E008;
	sub_831A8130(ctx, base);
	// 8290E008: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8290E00C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290E010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290E014: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290E018: 4BFFFA29  bl 0x8290da40
	ctx.lr = 0x8290E01C;
	sub_8290DA40(ctx, base);
	// 8290E01C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E020: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290E024: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E028: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E02C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290E030: 419A0024  beq cr6, 0x8290e054
	if ctx.cr[6].eq {
	pc = 0x8290E054; continue 'dispatch;
	}
	// 8290E034: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290E038: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290E03C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E040: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290E044: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290E048: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290E04C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E050: 4082FFE8  bne 0x8290e038
	if !ctx.cr[0].eq {
	pc = 0x8290E038; continue 'dispatch;
	}
	// 8290E054: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290E058: 80DF02D0  lwz r6, 0x2d0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as u64;
	// 8290E05C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290E060: 3BAB5758  addi r29, r11, 0x5758
	ctx.r[29].s64 = ctx.r[11].s64 + 22360;
	// 8290E064: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 8290E068: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290E06C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290E070: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290E074: 38A00238  li r5, 0x238
	ctx.r[5].s64 = 568;
	// 8290E078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290E07C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290E080: 4854A9C1  bl 0x82e58a40
	ctx.lr = 0x8290E084;
	sub_82E58A40(ctx, base);
	// 8290E084: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290E088: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E08C: 419A0008  beq cr6, 0x8290e094
	if ctx.cr[6].eq {
	pc = 0x8290E094; continue 'dispatch;
	}
	// 8290E090: 4B9B2801  bl 0x822c0890
	ctx.lr = 0x8290E094;
	sub_822C0890(ctx, base);
	// 8290E094: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290E098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E09C: 419A0008  beq cr6, 0x8290e0a4
	if ctx.cr[6].eq {
	pc = 0x8290E0A4; continue 'dispatch;
	}
	// 8290E0A0: 4B9B27F1  bl 0x822c0890
	ctx.lr = 0x8290E0A4;
	sub_822C0890(ctx, base);
	// 8290E0A4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290E0A8: 4BFFFA09  bl 0x8290dab0
	ctx.lr = 0x8290E0AC;
	sub_8290DAB0(ctx, base);
	// 8290E0AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E0B0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290E0B4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E0B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E0BC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290E0C0: 419A0024  beq cr6, 0x8290e0e4
	if ctx.cr[6].eq {
	pc = 0x8290E0E4; continue 'dispatch;
	}
	// 8290E0C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290E0C8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290E0CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E0D0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290E0D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290E0D8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290E0DC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E0E0: 4082FFE8  bne 0x8290e0c8
	if !ctx.cr[0].eq {
	pc = 0x8290E0C8; continue 'dispatch;
	}
	// 8290E0E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290E0E8: 80DF02D8  lwz r6, 0x2d8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(728 as u32) ) } as u64;
	// 8290E0EC: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8290E0F0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290E0F4: 38A00239  li r5, 0x239
	ctx.r[5].s64 = 569;
	// 8290E0F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290E0FC: 4854A945  bl 0x82e58a40
	ctx.lr = 0x8290E100;
	sub_82E58A40(ctx, base);
	// 8290E100: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290E104: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E108: 419A0008  beq cr6, 0x8290e110
	if ctx.cr[6].eq {
	pc = 0x8290E110; continue 'dispatch;
	}
	// 8290E10C: 4B9B2785  bl 0x822c0890
	ctx.lr = 0x8290E110;
	sub_822C0890(ctx, base);
	// 8290E110: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290E114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E118: 419A0008  beq cr6, 0x8290e120
	if ctx.cr[6].eq {
	pc = 0x8290E120; continue 'dispatch;
	}
	// 8290E11C: 4B9B2775  bl 0x822c0890
	ctx.lr = 0x8290E120;
	sub_822C0890(ctx, base);
	// 8290E120: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8290E124: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8290E128: 4889A094  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290E130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290E130 size=300
    let mut pc: u32 = 0x8290E130;
    'dispatch: loop {
        match pc {
            0x8290E130 => {
    //   block [0x8290E130..0x8290E25C)
	// 8290E130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290E134: 4889A039  bl 0x831a816c
	ctx.lr = 0x8290E138;
	sub_831A8130(ctx, base);
	// 8290E138: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8290E13C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290E140: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290E144: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290E148: 4BFFF9D9  bl 0x8290db20
	ctx.lr = 0x8290E14C;
	sub_8290DB20(ctx, base);
	// 8290E14C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E150: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290E154: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E15C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290E160: 419A0024  beq cr6, 0x8290e184
	if ctx.cr[6].eq {
	pc = 0x8290E184; continue 'dispatch;
	}
	// 8290E164: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290E168: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290E16C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E170: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290E174: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290E178: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290E17C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E180: 4082FFE8  bne 0x8290e168
	if !ctx.cr[0].eq {
	pc = 0x8290E168; continue 'dispatch;
	}
	// 8290E184: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290E188: 80DF02D0  lwz r6, 0x2d0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as u64;
	// 8290E18C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8290E190: 3BAB5758  addi r29, r11, 0x5758
	ctx.r[29].s64 = ctx.r[11].s64 + 22360;
	// 8290E194: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 8290E198: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290E19C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290E1A0: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290E1A4: 38A00244  li r5, 0x244
	ctx.r[5].s64 = 580;
	// 8290E1A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290E1AC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290E1B0: 4854A891  bl 0x82e58a40
	ctx.lr = 0x8290E1B4;
	sub_82E58A40(ctx, base);
	// 8290E1B4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290E1B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E1BC: 419A0008  beq cr6, 0x8290e1c4
	if ctx.cr[6].eq {
	pc = 0x8290E1C4; continue 'dispatch;
	}
	// 8290E1C0: 4B9B26D1  bl 0x822c0890
	ctx.lr = 0x8290E1C4;
	sub_822C0890(ctx, base);
	// 8290E1C4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290E1C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E1CC: 419A0008  beq cr6, 0x8290e1d4
	if ctx.cr[6].eq {
	pc = 0x8290E1D4; continue 'dispatch;
	}
	// 8290E1D0: 4B9B26C1  bl 0x822c0890
	ctx.lr = 0x8290E1D4;
	sub_822C0890(ctx, base);
	// 8290E1D4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290E1D8: 4BFFF9B9  bl 0x8290db90
	ctx.lr = 0x8290E1DC;
	sub_8290DB90(ctx, base);
	// 8290E1DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E1E0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290E1E4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E1E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E1EC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290E1F0: 419A0024  beq cr6, 0x8290e214
	if ctx.cr[6].eq {
	pc = 0x8290E214; continue 'dispatch;
	}
	// 8290E1F4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290E1F8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290E1FC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E200: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290E204: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290E208: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290E20C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E210: 4082FFE8  bne 0x8290e1f8
	if !ctx.cr[0].eq {
	pc = 0x8290E1F8; continue 'dispatch;
	}
	// 8290E214: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290E218: 80DF02D8  lwz r6, 0x2d8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(728 as u32) ) } as u64;
	// 8290E21C: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8290E220: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290E224: 38A00245  li r5, 0x245
	ctx.r[5].s64 = 581;
	// 8290E228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290E22C: 4854A815  bl 0x82e58a40
	ctx.lr = 0x8290E230;
	sub_82E58A40(ctx, base);
	// 8290E230: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290E234: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E238: 419A0008  beq cr6, 0x8290e240
	if ctx.cr[6].eq {
	pc = 0x8290E240; continue 'dispatch;
	}
	// 8290E23C: 4B9B2655  bl 0x822c0890
	ctx.lr = 0x8290E240;
	sub_822C0890(ctx, base);
	// 8290E240: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290E244: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E248: 419A0008  beq cr6, 0x8290e250
	if ctx.cr[6].eq {
	pc = 0x8290E250; continue 'dispatch;
	}
	// 8290E24C: 4B9B2645  bl 0x822c0890
	ctx.lr = 0x8290E250;
	sub_822C0890(ctx, base);
	// 8290E250: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8290E254: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8290E258: 48899F64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290E260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290E260 size=492
    let mut pc: u32 = 0x8290E260;
    'dispatch: loop {
        match pc {
            0x8290E260 => {
    //   block [0x8290E260..0x8290E44C)
	// 8290E260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290E264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290E268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290E26C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290E270: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290E274: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290E278: 897E0354  lbz r11, 0x354(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(852 as u32) ) } as u64;
	// 8290E27C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290E280: 408201B4  bne 0x8290e434
	if !ctx.cr[0].eq {
	pc = 0x8290E434; continue 'dispatch;
	}
	// 8290E284: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E288: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 8290E28C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290E290: 4E800421  bctrl
	ctx.lr = 0x8290E294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290E294: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8290E298: 4182019C  beq 0x8290e434
	if ctx.cr[0].eq {
	pc = 0x8290E434; continue 'dispatch;
	}
	// 8290E29C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8290E2A0: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290E2A4: 48008D8D  bl 0x82917030
	ctx.lr = 0x8290E2A8;
	sub_82917030(ctx, base);
	// 8290E2A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290E2AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290E2B0: 418200C0  beq 0x8290e370
	if ctx.cr[0].eq {
	pc = 0x8290E370; continue 'dispatch;
	}
	// 8290E2B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290E2B8: 389F0024  addi r4, r31, 0x24
	ctx.r[4].s64 = ctx.r[31].s64 + 36;
	// 8290E2BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290E2C0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290E2C4: 4BFFF93D  bl 0x8290dc00
	ctx.lr = 0x8290E2C8;
	sub_8290DC00(ctx, base);
	// 8290E2C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E2CC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290E2D0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E2D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E2D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290E2DC: 419A0024  beq cr6, 0x8290e300
	if ctx.cr[6].eq {
	pc = 0x8290E300; continue 'dispatch;
	}
	// 8290E2E0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290E2E4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290E2E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E2EC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290E2F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290E2F4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290E2F8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E2FC: 4082FFE8  bne 0x8290e2e4
	if !ctx.cr[0].eq {
	pc = 0x8290E2E4; continue 'dispatch;
	}
	// 8290E300: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290E304: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290E308: 4BC011C1  bl 0x8250f4c8
	ctx.lr = 0x8290E30C;
	sub_8250F4C8(ctx, base);
	// 8290E30C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E310: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E314: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8290E318: 409A0008  bne cr6, 0x8290e320
	if !ctx.cr[6].eq {
	pc = 0x8290E320; continue 'dispatch;
	}
	// 8290E31C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290E320: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290E324: 3BE10058  addi r31, r1, 0x58
	ctx.r[31].s64 = ctx.r[1].s64 + 88;
	// 8290E328: 4BBFA6F1  bl 0x82508a18
	ctx.lr = 0x8290E32C;
	sub_82508A18(ctx, base);
	// 8290E32C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290E330: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290E334: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290E338: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290E33C: 38A0027B  li r5, 0x27b
	ctx.r[5].s64 = 635;
	// 8290E340: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 8290E344: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290E348: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8290E34C: 4854A6F5  bl 0x82e58a40
	ctx.lr = 0x8290E350;
	sub_82E58A40(ctx, base);
	// 8290E350: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290E354: 484E393D  bl 0x82df1c90
	ctx.lr = 0x8290E358;
	sub_82DF1C90(ctx, base);
	// 8290E358: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290E35C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E360: 419A0008  beq cr6, 0x8290e368
	if ctx.cr[6].eq {
	pc = 0x8290E368; continue 'dispatch;
	}
	// 8290E364: 4B9B252D  bl 0x822c0890
	ctx.lr = 0x8290E368;
	sub_822C0890(ctx, base);
	// 8290E368: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290E36C: 480000BC  b 0x8290e428
	pc = 0x8290E428; continue 'dispatch;
	// 8290E370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290E374: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8290E378: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290E37C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290E380: 4BFFF881  bl 0x8290dc00
	ctx.lr = 0x8290E384;
	sub_8290DC00(ctx, base);
	// 8290E384: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E388: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290E38C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E394: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8290E398: 419A0024  beq cr6, 0x8290e3bc
	if ctx.cr[6].eq {
	pc = 0x8290E3BC; continue 'dispatch;
	}
	// 8290E39C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290E3A0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290E3A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E3A8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290E3AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290E3B0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290E3B4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E3B8: 4082FFE8  bne 0x8290e3a0
	if !ctx.cr[0].eq {
	pc = 0x8290E3A0; continue 'dispatch;
	}
	// 8290E3BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290E3C0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8290E3C4: 4BC01105  bl 0x8250f4c8
	ctx.lr = 0x8290E3C8;
	sub_8250F4C8(ctx, base);
	// 8290E3C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E3CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E3D0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8290E3D4: 409A0008  bne cr6, 0x8290e3dc
	if !ctx.cr[6].eq {
	pc = 0x8290E3DC; continue 'dispatch;
	}
	// 8290E3D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290E3DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290E3E0: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 8290E3E4: 4BBFA635  bl 0x82508a18
	ctx.lr = 0x8290E3E8;
	sub_82508A18(ctx, base);
	// 8290E3E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290E3EC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290E3F0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290E3F4: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290E3F8: 38A00280  li r5, 0x280
	ctx.r[5].s64 = 640;
	// 8290E3FC: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 8290E400: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290E404: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8290E408: 4854A639  bl 0x82e58a40
	ctx.lr = 0x8290E40C;
	sub_82E58A40(ctx, base);
	// 8290E40C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8290E410: 484E3881  bl 0x82df1c90
	ctx.lr = 0x8290E414;
	sub_82DF1C90(ctx, base);
	// 8290E414: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290E418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E41C: 419A0008  beq cr6, 0x8290e424
	if ctx.cr[6].eq {
	pc = 0x8290E424; continue 'dispatch;
	}
	// 8290E420: 4B9B2471  bl 0x822c0890
	ctx.lr = 0x8290E424;
	sub_822C0890(ctx, base);
	// 8290E424: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8290E428: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E42C: 419A0008  beq cr6, 0x8290e434
	if ctx.cr[6].eq {
	pc = 0x8290E434; continue 'dispatch;
	}
	// 8290E430: 4B9B2461  bl 0x822c0890
	ctx.lr = 0x8290E434;
	sub_822C0890(ctx, base);
	// 8290E434: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8290E438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290E43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290E440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290E444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290E448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290E450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290E450 size=252
    let mut pc: u32 = 0x8290E450;
    'dispatch: loop {
        match pc {
            0x8290E450 => {
    //   block [0x8290E450..0x8290E54C)
	// 8290E450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290E454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290E458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290E45C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8290E460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290E464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290E468: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8290E46C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8290E470: 394AB218  addi r10, r10, -0x4de8
	ctx.r[10].s64 = ctx.r[10].s64 + -19944;
	// 8290E474: 38BF0330  addi r5, r31, 0x330
	ctx.r[5].s64 = ctx.r[31].s64 + 816;
	// 8290E478: 817F0244  lwz r11, 0x244(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290E47C: C01F0350  lfs f0, 0x350(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(848 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290E480: 389F032C  addi r4, r31, 0x32c
	ctx.r[4].s64 = ctx.r[31].s64 + 812;
	// 8290E484: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8290E488: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290E48C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8290E490: 7DAB542E  lfsx f13, r11, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290E494: EDAD0072  fmuls f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 8290E498: D1BF0330  stfs f13, 0x330(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(816 as u32), tmp.u32 ) };
	// 8290E49C: D1BF032C  stfs f13, 0x32c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 8290E4A0: 40990008  ble cr6, 0x8290e4a8
	if !ctx.cr[6].gt {
	pc = 0x8290E4A8; continue 'dispatch;
	}
	// 8290E4A4: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8290E4A8: 817F02EC  lwz r11, 0x2ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8290E4AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E4B0: 419A0084  beq cr6, 0x8290e534
	if ctx.cr[6].eq {
	pc = 0x8290E534; continue 'dispatch;
	}
	// 8290E4B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290E4B8: 4BFFF7C9  bl 0x8290dc80
	ctx.lr = 0x8290E4BC;
	sub_8290DC80(ctx, base);
	// 8290E4BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E4C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290E4C4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E4C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E4CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290E4D0: 419A0024  beq cr6, 0x8290e4f4
	if ctx.cr[6].eq {
	pc = 0x8290E4F4; continue 'dispatch;
	}
	// 8290E4D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290E4D8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290E4DC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E4E0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290E4E4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290E4E8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290E4EC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290E4F0: 4082FFE8  bne 0x8290e4d8
	if !ctx.cr[0].eq {
	pc = 0x8290E4D8; continue 'dispatch;
	}
	// 8290E4F4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290E4F8: 80DF02EC  lwz r6, 0x2ec(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8290E4FC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290E500: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290E504: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290E508: 38A00333  li r5, 0x333
	ctx.r[5].s64 = 819;
	// 8290E50C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8290E510: 4854A531  bl 0x82e58a40
	ctx.lr = 0x8290E514;
	sub_82E58A40(ctx, base);
	// 8290E514: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290E518: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E51C: 419A0008  beq cr6, 0x8290e524
	if ctx.cr[6].eq {
	pc = 0x8290E524; continue 'dispatch;
	}
	// 8290E520: 4B9B2371  bl 0x822c0890
	ctx.lr = 0x8290E524;
	sub_822C0890(ctx, base);
	// 8290E524: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290E528: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290E52C: 419A0008  beq cr6, 0x8290e534
	if ctx.cr[6].eq {
	pc = 0x8290E534; continue 'dispatch;
	}
	// 8290E530: 4B9B2361  bl 0x822c0890
	ctx.lr = 0x8290E534;
	sub_822C0890(ctx, base);
	// 8290E534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290E538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290E53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290E540: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290E544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290E548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290E550 size=888
    let mut pc: u32 = 0x8290E550;
    'dispatch: loop {
        match pc {
            0x8290E550 => {
    //   block [0x8290E550..0x8290E8C8)
	// 8290E550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290E554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290E558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290E55C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290E560: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8290E564: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290E568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290E56C: 897F0354  lbz r11, 0x354(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(852 as u32) ) } as u64;
	// 8290E570: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290E574: 40820338  bne 0x8290e8ac
	if !ctx.cr[0].eq {
	pc = 0x8290E8AC; continue 'dispatch;
	}
	// 8290E578: 788B0020  clrldi r11, r4, 0x20
	ctx.r[11].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 8290E57C: 815F0244  lwz r10, 0x244(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290E580: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8290E584: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8290E588: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8290E58C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8290E590: 3969B318  addi r11, r9, -0x4ce8
	ctx.r[11].s64 = ctx.r[9].s64 + -19688;
	// 8290E594: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8290E598: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8290E59C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8290E5A0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8290E5A4: 7DAA5C2E  lfsx f13, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290E5A8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8290E5AC: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8290E5B0: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8290E5B4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290E5B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290E5BC: 409A000C  bne cr6, 0x8290e5c8
	if !ctx.cr[6].eq {
	pc = 0x8290E5C8; continue 'dispatch;
	}
	// 8290E5C0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8290E5C4: 48000014  b 0x8290e5d8
	pc = 0x8290E5D8; continue 'dispatch;
	// 8290E5C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E5CC: 409A000C  bne cr6, 0x8290e5d8
	if !ctx.cr[6].eq {
	pc = 0x8290E5D8; continue 'dispatch;
	}
	// 8290E5D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290E5D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290E5D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290E5E0: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290E5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290E5E8: 4E800421  bctrl
	ctx.lr = 0x8290E5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290E5EC: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290E5F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290E5F4: 41820158  beq 0x8290e74c
	if ctx.cr[0].eq {
	pc = 0x8290E74C; continue 'dispatch;
	}
	// 8290E5F8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290E5FC: 484E540D  bl 0x82df3a08
	ctx.lr = 0x8290E600;
	sub_82DF3A08(ctx, base);
	// 8290E600: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290E604: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8290E608: 4BEDDFC1  bl 0x827ec5c8
	ctx.lr = 0x8290E60C;
	sub_827EC5C8(ctx, base);
	// 8290E60C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290E610: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290E614: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8290E618: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E61C: 48506A15  bl 0x82e15030
	ctx.lr = 0x8290E620;
	sub_82E15030(ctx, base);
	// 8290E620: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E624: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E628: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8290E62C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290E630: 4E800421  bctrl
	ctx.lr = 0x8290E634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290E634: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290E638: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8290E63C: 806100B4  lwz r3, 0xb4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 8290E640: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8290E644: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 8290E648: 38E100D0  addi r7, r1, 0xd0
	ctx.r[7].s64 = ctx.r[1].s64 + 208;
	// 8290E64C: 38C100E0  addi r6, r1, 0xe0
	ctx.r[6].s64 = ctx.r[1].s64 + 224;
	// 8290E650: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290E654: 13CA5C07  vcmpneb. (lvlx128) v30, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290E658: 38A100F0  addi r5, r1, 0xf0
	ctx.r[5].s64 = ctx.r[1].s64 + 240;
	// 8290E65C: 39410100  addi r10, r1, 0x100
	ctx.r[10].s64 = ctx.r[1].s64 + 256;
	// 8290E660: 13A95C07  vcmpneb. (lvlx128) v29, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290E664: 13885C07  vcmpneb. (lvlx128) v28, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290E668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290E8C8 size=156
    let mut pc: u32 = 0x8290E8C8;
    'dispatch: loop {
        match pc {
            0x8290E8C8 => {
    //   block [0x8290E8C8..0x8290E964)
	// 8290E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290E8CC: 4889989D  bl 0x831a8168
	ctx.lr = 0x8290E8D0;
	sub_831A8130(ctx, base);
	// 8290E8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290E8D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290E8D8: 3BBF0340  addi r29, r31, 0x340
	ctx.r[29].s64 = ctx.r[31].s64 + 832;
	// 8290E8DC: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 8290E8E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E8E4: 409A0074  bne cr6, 0x8290e958
	if !ctx.cr[6].eq {
	pc = 0x8290E958; continue 'dispatch;
	}
	// 8290E8E8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290E8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8290E8F0: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290E8F4: 38A003E8  li r5, 0x3e8
	ctx.r[5].s64 = 1000;
	// 8290E8F8: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8290E8FC: 4B9B1ADD  bl 0x822c03d8
	ctx.lr = 0x8290E900;
	sub_822C03D8(ctx, base);
	// 8290E900: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8290E904: 41820048  beq 0x8290e94c
	if ctx.cr[0].eq {
	pc = 0x8290E94C; continue 'dispatch;
	}
	// 8290E908: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290E90C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E910: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8290E914: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290E918: 4E800421  bctrl
	ctx.lr = 0x8290E91C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290E91C: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8290E920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290E924: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8290E928: 557CDFFE  rlwinm r28, r11, 0x1b, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8290E92C: 4BEDD8C5  bl 0x827ec1f0
	ctx.lr = 0x8290E930;
	sub_827EC1F0(ctx, base);
	// 8290E930: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8290E934: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290E938: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290E93C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8290E940: 4BEE5671  bl 0x827f3fb0
	ctx.lr = 0x8290E944;
	sub_827F3FB0(ctx, base);
	// 8290E944: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290E948: 48000008  b 0x8290e950
	pc = 0x8290E950; continue 'dispatch;
	// 8290E94C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8290E950: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290E954: 4BFFF3AD  bl 0x8290dd00
	ctx.lr = 0x8290E958;
	sub_8290DD00(ctx, base);
	// 8290E958: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8290E95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290E960: 48899858  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290E968 size=848
    let mut pc: u32 = 0x8290E968;
    'dispatch: loop {
        match pc {
            0x8290E968 => {
    //   block [0x8290E968..0x8290ECB8)
	// 8290E968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290E96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290E970: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290E974: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290E978: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290E97C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290E980: 897F0354  lbz r11, 0x354(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(852 as u32) ) } as u64;
	// 8290E984: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290E988: 40820318  bne 0x8290eca0
	if !ctx.cr[0].eq {
	pc = 0x8290ECA0; continue 'dispatch;
	}
	// 8290E98C: 788B0020  clrldi r11, r4, 0x20
	ctx.r[11].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 8290E990: 815F0244  lwz r10, 0x244(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 8290E994: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 8290E998: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8290E99C: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8290E9A0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8290E9A4: 3969B398  addi r11, r9, -0x4c68
	ctx.r[11].s64 = ctx.r[9].s64 + -19560;
	// 8290E9A8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8290E9AC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8290E9B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8290E9B4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8290E9B8: 7DAA5C2E  lfsx f13, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8290E9BC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8290E9C0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8290E9C4: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8290E9C8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290E9CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290E9D0: 409A000C  bne cr6, 0x8290e9dc
	if !ctx.cr[6].eq {
	pc = 0x8290E9DC; continue 'dispatch;
	}
	// 8290E9D4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8290E9D8: 48000014  b 0x8290e9ec
	pc = 0x8290E9EC; continue 'dispatch;
	// 8290E9DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290E9E0: 409A000C  bne cr6, 0x8290e9ec
	if !ctx.cr[6].eq {
	pc = 0x8290E9EC; continue 'dispatch;
	}
	// 8290E9E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290E9E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290E9EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290E9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290E9F4: 816B0164  lwz r11, 0x164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 8290E9F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290E9FC: 4E800421  bctrl
	ctx.lr = 0x8290EA00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290EA00: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290EA04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290EA08: 41820148  beq 0x8290eb50
	if ctx.cr[0].eq {
	pc = 0x8290EB50; continue 'dispatch;
	}
	// 8290EA0C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290EA10: 484E4FF9  bl 0x82df3a08
	ctx.lr = 0x8290EA14;
	sub_82DF3A08(ctx, base);
	// 8290EA14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290EA18: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8290EA1C: 4BEDDBAD  bl 0x827ec5c8
	ctx.lr = 0x8290EA20;
	sub_827EC5C8(ctx, base);
	// 8290EA20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290EA24: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8290EA28: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290EA2C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290EA30: 48506601  bl 0x82e15030
	ctx.lr = 0x8290EA34;
	sub_82E15030(ctx, base);
	// 8290EA34: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290EA38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290EA3C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8290EA40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290EA44: 4E800421  bctrl
	ctx.lr = 0x8290EA48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290EA48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8290EA4C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8290EA50: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8290EA54: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8290EA58: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 8290EA5C: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 8290EA60: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 8290EA64: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290EA68: 13CA5C07  vcmpneb. (lvlx128) v30, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290EA6C: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 8290EA70: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 8290EA74: 13A95C07  vcmpneb. (lvlx128) v29, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290EA78: 13885C07  vcmpneb. (lvlx128) v28, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8290EA7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290ECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290ECB8 size=192
    let mut pc: u32 = 0x8290ECB8;
    'dispatch: loop {
        match pc {
            0x8290ECB8 => {
    //   block [0x8290ECB8..0x8290ED78)
	// 8290ECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290ECBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290ECC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290ECC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290ECC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290ECCC: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8290ECD0: 897F0354  lbz r11, 0x354(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(852 as u32) ) } as u64;
	// 8290ECD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290ECD8: 4082008C  bne 0x8290ed64
	if !ctx.cr[0].eq {
	pc = 0x8290ED64; continue 'dispatch;
	}
	// 8290ECDC: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 8290ECE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290ECE4: 480BB06D  bl 0x829c9d50
	ctx.lr = 0x8290ECE8;
	sub_829C9D50(ctx, base);
	// 8290ECE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290ECEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290ECF0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290ECF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290ECF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290ECFC: 419A0024  beq cr6, 0x8290ed20
	if ctx.cr[6].eq {
	pc = 0x8290ED20; continue 'dispatch;
	}
	// 8290ED00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290ED04: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290ED08: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290ED0C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290ED10: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290ED14: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290ED18: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290ED1C: 4082FFE8  bne 0x8290ed04
	if !ctx.cr[0].eq {
	pc = 0x8290ED04; continue 'dispatch;
	}
	// 8290ED20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290ED24: 80DF02DC  lwz r6, 0x2dc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(732 as u32) ) } as u64;
	// 8290ED28: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290ED2C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290ED30: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290ED34: 38A00425  li r5, 0x425
	ctx.r[5].s64 = 1061;
	// 8290ED38: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8290ED3C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290ED40: 48549D01  bl 0x82e58a40
	ctx.lr = 0x8290ED44;
	sub_82E58A40(ctx, base);
	// 8290ED44: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290ED48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290ED4C: 419A0008  beq cr6, 0x8290ed54
	if ctx.cr[6].eq {
	pc = 0x8290ED54; continue 'dispatch;
	}
	// 8290ED50: 4B9B1B41  bl 0x822c0890
	ctx.lr = 0x8290ED54;
	sub_822C0890(ctx, base);
	// 8290ED54: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290ED58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290ED5C: 419A0008  beq cr6, 0x8290ed64
	if ctx.cr[6].eq {
	pc = 0x8290ED64; continue 'dispatch;
	}
	// 8290ED60: 4B9B1B31  bl 0x822c0890
	ctx.lr = 0x8290ED64;
	sub_822C0890(ctx, base);
	// 8290ED64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290ED68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290ED6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290ED70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290ED74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290ED78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290ED78 size=228
    let mut pc: u32 = 0x8290ED78;
    'dispatch: loop {
        match pc {
            0x8290ED78 => {
    //   block [0x8290ED78..0x8290EE5C)
	// 8290ED78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290ED7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290ED80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290ED84: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8290ED88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290ED8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290ED90: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 8290ED94: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8290ED98: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 8290ED9C: 419A000C  beq cr6, 0x8290eda8
	if ctx.cr[6].eq {
	pc = 0x8290EDA8; continue 'dispatch;
	}
	// 8290EDA0: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 8290EDA4: 409A001C  bne cr6, 0x8290edc0
	if !ctx.cr[6].eq {
	pc = 0x8290EDC0; continue 'dispatch;
	}
	// 8290EDA8: 897F0338  lbz r11, 0x338(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(824 as u32) ) } as u64;
	// 8290EDAC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8290EDB0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8290EDB4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8290EDB8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290EDBC: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8290EDC0: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 8290EDC4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290EDC8: 4BF596F1  bl 0x828684b8
	ctx.lr = 0x8290EDCC;
	sub_828684B8(ctx, base);
	// 8290EDCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290EDD0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290EDD4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290EDD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290EDDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290EDE0: 419A0024  beq cr6, 0x8290ee04
	if ctx.cr[6].eq {
	pc = 0x8290EE04; continue 'dispatch;
	}
	// 8290EDE4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290EDE8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290EDEC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290EDF0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290EDF4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290EDF8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290EDFC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290EE00: 4082FFE8  bne 0x8290ede8
	if !ctx.cr[0].eq {
	pc = 0x8290EDE8; continue 'dispatch;
	}
	// 8290EE04: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290EE08: 80DF02D4  lwz r6, 0x2d4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(724 as u32) ) } as u64;
	// 8290EE0C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290EE10: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290EE14: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290EE18: 38A00463  li r5, 0x463
	ctx.r[5].s64 = 1123;
	// 8290EE1C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8290EE20: 48549C21  bl 0x82e58a40
	ctx.lr = 0x8290EE24;
	sub_82E58A40(ctx, base);
	// 8290EE24: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290EE28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290EE2C: 419A0008  beq cr6, 0x8290ee34
	if ctx.cr[6].eq {
	pc = 0x8290EE34; continue 'dispatch;
	}
	// 8290EE30: 4B9B1A61  bl 0x822c0890
	ctx.lr = 0x8290EE34;
	sub_822C0890(ctx, base);
	// 8290EE34: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290EE38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290EE3C: 419A0008  beq cr6, 0x8290ee44
	if ctx.cr[6].eq {
	pc = 0x8290EE44; continue 'dispatch;
	}
	// 8290EE40: 4B9B1A51  bl 0x822c0890
	ctx.lr = 0x8290EE44;
	sub_822C0890(ctx, base);
	// 8290EE44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290EE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290EE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290EE50: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290EE54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290EE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290EE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290EE60 size=196
    let mut pc: u32 = 0x8290EE60;
    'dispatch: loop {
        match pc {
            0x8290EE60 => {
    //   block [0x8290EE60..0x8290EF24)
	// 8290EE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290EE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290EE68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290EE6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290EE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290EE74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290EE78: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290EE7C: 4BAC4A8D  bl 0x823d3908
	ctx.lr = 0x8290EE80;
	sub_823D3908(ctx, base);
	// 8290EE80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290EE84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290EE88: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290EE8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290EE90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290EE94: 419A0024  beq cr6, 0x8290eeb8
	if ctx.cr[6].eq {
	pc = 0x8290EEB8; continue 'dispatch;
	}
	// 8290EE98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290EE9C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290EEA0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290EEA4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290EEA8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290EEAC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290EEB0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290EEB4: 4082FFE8  bne 0x8290ee9c
	if !ctx.cr[0].eq {
	pc = 0x8290EE9C; continue 'dispatch;
	}
	// 8290EEB8: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290EEBC: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290EEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290EEC4: 486FA0F5  bl 0x83008fb8
	ctx.lr = 0x8290EEC8;
	sub_83008FB8(ctx, base);
	// 8290EEC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290EECC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290EED0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290EED4: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290EED8: 38A007C9  li r5, 0x7c9
	ctx.r[5].s64 = 1993;
	// 8290EEDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290EEE0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290EEE4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290EEE8: 48549B59  bl 0x82e58a40
	ctx.lr = 0x8290EEEC;
	sub_82E58A40(ctx, base);
	// 8290EEEC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290EEF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290EEF4: 419A0008  beq cr6, 0x8290eefc
	if ctx.cr[6].eq {
	pc = 0x8290EEFC; continue 'dispatch;
	}
	// 8290EEF8: 4B9B1999  bl 0x822c0890
	ctx.lr = 0x8290EEFC;
	sub_822C0890(ctx, base);
	// 8290EEFC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290EF00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290EF04: 419A0008  beq cr6, 0x8290ef0c
	if ctx.cr[6].eq {
	pc = 0x8290EF0C; continue 'dispatch;
	}
	// 8290EF08: 4B9B1989  bl 0x822c0890
	ctx.lr = 0x8290EF0C;
	sub_822C0890(ctx, base);
	// 8290EF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290EF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290EF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290EF18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290EF1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290EF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290EF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290EF28 size=560
    let mut pc: u32 = 0x8290EF28;
    'dispatch: loop {
        match pc {
            0x8290EF28 => {
    //   block [0x8290EF28..0x8290F158)
	// 8290EF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290EF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290EF30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290EF34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290EF38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290EF3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290EF40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290EF44: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290EF48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290EF4C: 419A0014  beq cr6, 0x8290ef60
	if ctx.cr[6].eq {
	pc = 0x8290EF60; continue 'dispatch;
	}
	// 8290EF50: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8290EF54: 480080DD  bl 0x82917030
	ctx.lr = 0x8290EF58;
	sub_82917030(ctx, base);
	// 8290EF58: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290EF5C: 408201E4  bne 0x8290f140
	if !ctx.cr[0].eq {
	pc = 0x8290F140; continue 'dispatch;
	}
	// 8290EF60: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290EF64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290EF68: 409A0098  bne cr6, 0x8290f000
	if !ctx.cr[6].eq {
	pc = 0x8290F000; continue 'dispatch;
	}
	// 8290EF6C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290EF70: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290EF74: 388B0B58  addi r4, r11, 0xb58
	ctx.r[4].s64 = ctx.r[11].s64 + 2904;
	// 8290EF78: 4BAFB059  bl 0x82409fd0
	ctx.lr = 0x8290EF7C;
	sub_82409FD0(ctx, base);
	// 8290EF7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290EF80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290EF84: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290EF88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290EF8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290EF90: 419A0024  beq cr6, 0x8290efb4
	if ctx.cr[6].eq {
	pc = 0x8290EFB4; continue 'dispatch;
	}
	// 8290EF94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290EF98: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290EF9C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290EFA0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290EFA4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290EFA8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290EFAC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290EFB0: 4082FFE8  bne 0x8290ef98
	if !ctx.cr[0].eq {
	pc = 0x8290EF98; continue 'dispatch;
	}
	// 8290EFB4: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290EFB8: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290EFBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290EFC0: 486F9FF9  bl 0x83008fb8
	ctx.lr = 0x8290EFC4;
	sub_83008FB8(ctx, base);
	// 8290EFC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290EFC8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290EFCC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290EFD0: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290EFD4: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 8290EFD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290EFDC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290EFE0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290EFE4: 48549A5D  bl 0x82e58a40
	ctx.lr = 0x8290EFE8;
	sub_82E58A40(ctx, base);
	// 8290EFE8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290EFEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290EFF0: 419A0008  beq cr6, 0x8290eff8
	if ctx.cr[6].eq {
	pc = 0x8290EFF8; continue 'dispatch;
	}
	// 8290EFF4: 4B9B189D  bl 0x822c0890
	ctx.lr = 0x8290EFF8;
	sub_822C0890(ctx, base);
	// 8290EFF8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290EFFC: 48000138  b 0x8290f134
	pc = 0x8290F134; continue 'dispatch;
	// 8290F000: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8290F004: 409A0098  bne cr6, 0x8290f09c
	if !ctx.cr[6].eq {
	pc = 0x8290F09C; continue 'dispatch;
	}
	// 8290F008: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290F00C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290F010: 388B0B64  addi r4, r11, 0xb64
	ctx.r[4].s64 = ctx.r[11].s64 + 2916;
	// 8290F014: 4BAFAFBD  bl 0x82409fd0
	ctx.lr = 0x8290F018;
	sub_82409FD0(ctx, base);
	// 8290F018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F01C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290F020: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F024: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290F02C: 419A0024  beq cr6, 0x8290f050
	if ctx.cr[6].eq {
	pc = 0x8290F050; continue 'dispatch;
	}
	// 8290F030: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F034: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F038: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F03C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F040: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F044: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F048: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F04C: 4082FFE8  bne 0x8290f034
	if !ctx.cr[0].eq {
	pc = 0x8290F034; continue 'dispatch;
	}
	// 8290F050: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290F054: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8290F058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F05C: 486F9F5D  bl 0x83008fb8
	ctx.lr = 0x8290F060;
	sub_83008FB8(ctx, base);
	// 8290F060: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F064: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290F068: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F06C: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290F070: 38A00805  li r5, 0x805
	ctx.r[5].s64 = 2053;
	// 8290F074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F078: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290F07C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290F080: 485499C1  bl 0x82e58a40
	ctx.lr = 0x8290F084;
	sub_82E58A40(ctx, base);
	// 8290F084: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290F088: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F08C: 419A0008  beq cr6, 0x8290f094
	if ctx.cr[6].eq {
	pc = 0x8290F094; continue 'dispatch;
	}
	// 8290F090: 4B9B1801  bl 0x822c0890
	ctx.lr = 0x8290F094;
	sub_822C0890(ctx, base);
	// 8290F094: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290F098: 4800009C  b 0x8290f134
	pc = 0x8290F134; continue 'dispatch;
	// 8290F09C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8290F0A0: 409A00A0  bne cr6, 0x8290f140
	if !ctx.cr[6].eq {
	pc = 0x8290F140; continue 'dispatch;
	}
	// 8290F0A4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290F0A8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8290F0AC: 388B0B68  addi r4, r11, 0xb68
	ctx.r[4].s64 = ctx.r[11].s64 + 2920;
	// 8290F0B0: 4BAFAF21  bl 0x82409fd0
	ctx.lr = 0x8290F0B4;
	sub_82409FD0(ctx, base);
	// 8290F0B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F0B8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290F0BC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F0C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F0C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8290F0C8: 419A0024  beq cr6, 0x8290f0ec
	if ctx.cr[6].eq {
	pc = 0x8290F0EC; continue 'dispatch;
	}
	// 8290F0CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F0D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F0D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F0D8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F0DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F0E0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F0E4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F0E8: 4082FFE8  bne 0x8290f0d0
	if !ctx.cr[0].eq {
	pc = 0x8290F0D0; continue 'dispatch;
	}
	// 8290F0EC: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290F0F0: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 8290F0F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F0F8: 486F9EC1  bl 0x83008fb8
	ctx.lr = 0x8290F0FC;
	sub_83008FB8(ctx, base);
	// 8290F0FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F100: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290F104: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F108: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290F10C: 38A0080A  li r5, 0x80a
	ctx.r[5].s64 = 2058;
	// 8290F110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F114: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290F118: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290F11C: 48549925  bl 0x82e58a40
	ctx.lr = 0x8290F120;
	sub_82E58A40(ctx, base);
	// 8290F120: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290F124: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F128: 419A0008  beq cr6, 0x8290f130
	if ctx.cr[6].eq {
	pc = 0x8290F130; continue 'dispatch;
	}
	// 8290F12C: 4B9B1765  bl 0x822c0890
	ctx.lr = 0x8290F130;
	sub_822C0890(ctx, base);
	// 8290F130: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8290F134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F138: 419A0008  beq cr6, 0x8290f140
	if ctx.cr[6].eq {
	pc = 0x8290F140; continue 'dispatch;
	}
	// 8290F13C: 4B9B1755  bl 0x822c0890
	ctx.lr = 0x8290F140;
	sub_822C0890(ctx, base);
	// 8290F140: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8290F144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290F148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290F14C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290F150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290F154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290F158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290F158 size=444
    let mut pc: u32 = 0x8290F158;
    'dispatch: loop {
        match pc {
            0x8290F158 => {
    //   block [0x8290F158..0x8290F314)
	// 8290F158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290F15C: 4889900D  bl 0x831a8168
	ctx.lr = 0x8290F160;
	sub_831A8130(ctx, base);
	// 8290F160: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290F164: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8290F168: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8290F16C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290F170: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290F174: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8290F178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F17C: 409A0034  bne cr6, 0x8290f1b0
	if !ctx.cr[6].eq {
	pc = 0x8290F1B0; continue 'dispatch;
	}
	// 8290F180: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290F184: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290F188: 808B0B58  lwz r4, 0xb58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2904 as u32) ) } as u64;
	// 8290F18C: 484E487D  bl 0x82df3a08
	ctx.lr = 0x8290F190;
	sub_82DF3A08(ctx, base);
	// 8290F190: 807D00E4  lwz r3, 0xe4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F194: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8290F198: 3B810058  addi r28, r1, 0x58
	ctx.r[28].s64 = ctx.r[1].s64 + 88;
	// 8290F19C: 4BEE05AD  bl 0x827ef748
	ctx.lr = 0x8290F1A0;
	sub_827EF748(ctx, base);
	// 8290F1A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8290F1A4: 484E4165  bl 0x82df3308
	ctx.lr = 0x8290F1A8;
	sub_82DF3308(ctx, base);
	// 8290F1A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F1AC: 4082007C  bne 0x8290f228
	if !ctx.cr[0].eq {
	pc = 0x8290F228; continue 'dispatch;
	}
	// 8290F1B0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290F1B4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8290F1B8: 409A0034  bne cr6, 0x8290f1ec
	if !ctx.cr[6].eq {
	pc = 0x8290F1EC; continue 'dispatch;
	}
	// 8290F1BC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290F1C0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290F1C4: 808B0B64  lwz r4, 0xb64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2916 as u32) ) } as u64;
	// 8290F1C8: 484E4841  bl 0x82df3a08
	ctx.lr = 0x8290F1CC;
	sub_82DF3A08(ctx, base);
	// 8290F1CC: 807D00E4  lwz r3, 0xe4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F1D0: 63FF0002  ori r31, r31, 2
	ctx.r[31].u64 = ctx.r[31].u64 | 2;
	// 8290F1D4: 3B810054  addi r28, r1, 0x54
	ctx.r[28].s64 = ctx.r[1].s64 + 84;
	// 8290F1D8: 4BEE0571  bl 0x827ef748
	ctx.lr = 0x8290F1DC;
	sub_827EF748(ctx, base);
	// 8290F1DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8290F1E0: 484E4129  bl 0x82df3308
	ctx.lr = 0x8290F1E4;
	sub_82DF3308(ctx, base);
	// 8290F1E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F1E8: 40820040  bne 0x8290f228
	if !ctx.cr[0].eq {
	pc = 0x8290F228; continue 'dispatch;
	}
	// 8290F1EC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290F1F0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8290F1F4: 409A003C  bne cr6, 0x8290f230
	if !ctx.cr[6].eq {
	pc = 0x8290F230; continue 'dispatch;
	}
	// 8290F1F8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290F1FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290F200: 808B0B68  lwz r4, 0xb68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2920 as u32) ) } as u64;
	// 8290F204: 484E4805  bl 0x82df3a08
	ctx.lr = 0x8290F208;
	sub_82DF3A08(ctx, base);
	// 8290F208: 807D00E4  lwz r3, 0xe4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F20C: 63FF0004  ori r31, r31, 4
	ctx.r[31].u64 = ctx.r[31].u64 | 4;
	// 8290F210: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290F214: 4BEE0535  bl 0x827ef748
	ctx.lr = 0x8290F218;
	sub_827EF748(ctx, base);
	// 8290F218: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8290F21C: 484E40ED  bl 0x82df3308
	ctx.lr = 0x8290F220;
	sub_82DF3308(ctx, base);
	// 8290F220: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F224: 4182000C  beq 0x8290f230
	if ctx.cr[0].eq {
	pc = 0x8290F230; continue 'dispatch;
	}
	// 8290F228: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290F22C: 48000008  b 0x8290f234
	pc = 0x8290F234; continue 'dispatch;
	// 8290F230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290F234: 57EA077B  rlwinm. r10, r31, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8290F238: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8290F23C: 41820010  beq 0x8290f24c
	if ctx.cr[0].eq {
	pc = 0x8290F24C; continue 'dispatch;
	}
	// 8290F240: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8290F244: 57FF07B8  rlwinm r31, r31, 0, 0x1e, 0x1c
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8290F248: 484E41E1  bl 0x82df3428
	ctx.lr = 0x8290F24C;
	sub_82DF3428(ctx, base);
	// 8290F24C: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F250: 41820010  beq 0x8290f260
	if ctx.cr[0].eq {
	pc = 0x8290F260; continue 'dispatch;
	}
	// 8290F254: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8290F258: 57FF07FA  rlwinm r31, r31, 0, 0x1f, 0x1d
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8290F25C: 484E41CD  bl 0x82df3428
	ctx.lr = 0x8290F260;
	sub_82DF3428(ctx, base);
	// 8290F260: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F264: 4182000C  beq 0x8290f270
	if ctx.cr[0].eq {
	pc = 0x8290F270; continue 'dispatch;
	}
	// 8290F268: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290F26C: 484E41BD  bl 0x82df3428
	ctx.lr = 0x8290F270;
	sub_82DF3428(ctx, base);
	// 8290F270: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F274: 41820098  beq 0x8290f30c
	if ctx.cr[0].eq {
	pc = 0x8290F30C; continue 'dispatch;
	}
	// 8290F278: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290F27C: 4BFFEAF5  bl 0x8290dd70
	ctx.lr = 0x8290F280;
	sub_8290DD70(ctx, base);
	// 8290F280: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F284: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290F288: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F28C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8290F294: 419A0024  beq cr6, 0x8290f2b8
	if ctx.cr[6].eq {
	pc = 0x8290F2B8; continue 'dispatch;
	}
	// 8290F298: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F29C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F2A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F2A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F2A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F2AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F2B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F2B4: 4082FFE8  bne 0x8290f29c
	if !ctx.cr[0].eq {
	pc = 0x8290F29C; continue 'dispatch;
	}
	// 8290F2B8: 3BFD0028  addi r31, r29, 0x28
	ctx.r[31].s64 = ctx.r[29].s64 + 40;
	// 8290F2BC: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 8290F2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F2C4: 486F9CF5  bl 0x83008fb8
	ctx.lr = 0x8290F2C8;
	sub_83008FB8(ctx, base);
	// 8290F2C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F2CC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290F2D0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F2D4: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290F2D8: 38A00819  li r5, 0x819
	ctx.r[5].s64 = 2073;
	// 8290F2DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F2E0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290F2E4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290F2E8: 48549759  bl 0x82e58a40
	ctx.lr = 0x8290F2EC;
	sub_82E58A40(ctx, base);
	// 8290F2EC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290F2F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F2F4: 419A0008  beq cr6, 0x8290f2fc
	if ctx.cr[6].eq {
	pc = 0x8290F2FC; continue 'dispatch;
	}
	// 8290F2F8: 4B9B1599  bl 0x822c0890
	ctx.lr = 0x8290F2FC;
	sub_822C0890(ctx, base);
	// 8290F2FC: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290F300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F304: 419A0008  beq cr6, 0x8290f30c
	if ctx.cr[6].eq {
	pc = 0x8290F30C; continue 'dispatch;
	}
	// 8290F308: 4B9B1589  bl 0x822c0890
	ctx.lr = 0x8290F30C;
	sub_822C0890(ctx, base);
	// 8290F30C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8290F310: 48898EA8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290F318 size=436
    let mut pc: u32 = 0x8290F318;
    'dispatch: loop {
        match pc {
            0x8290F318 => {
    //   block [0x8290F318..0x8290F4CC)
	// 8290F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290F320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290F324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290F328: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290F32C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290F330: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290F334: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8290F338: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290F33C: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F340: 418200B8  beq 0x8290f3f8
	if ctx.cr[0].eq {
	pc = 0x8290F3F8; continue 'dispatch;
	}
	// 8290F344: 48007CED  bl 0x82917030
	ctx.lr = 0x8290F348;
	sub_82917030(ctx, base);
	// 8290F348: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F34C: 41820168  beq 0x8290f4b4
	if ctx.cr[0].eq {
	pc = 0x8290F4B4; continue 'dispatch;
	}
	// 8290F350: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8290F354: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F358: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8290F35C: 48007CC5  bl 0x82917020
	ctx.lr = 0x8290F360;
	sub_82917020(ctx, base);
	// 8290F360: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8290F364: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290F368: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290F36C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8290F370: 4BF71501  bl 0x82880870
	ctx.lr = 0x8290F374;
	sub_82880870(ctx, base);
	// 8290F374: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F378: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290F37C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F384: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290F388: 419A0024  beq cr6, 0x8290f3ac
	if ctx.cr[6].eq {
	pc = 0x8290F3AC; continue 'dispatch;
	}
	// 8290F38C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F390: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F394: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F398: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F39C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F3A0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F3A4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F3A8: 4082FFE8  bne 0x8290f390
	if !ctx.cr[0].eq {
	pc = 0x8290F390; continue 'dispatch;
	}
	// 8290F3AC: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290F3B0: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8290F3B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F3B8: 486F9C01  bl 0x83008fb8
	ctx.lr = 0x8290F3BC;
	sub_83008FB8(ctx, base);
	// 8290F3BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F3C0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290F3C4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F3C8: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290F3CC: 38A008BB  li r5, 0x8bb
	ctx.r[5].s64 = 2235;
	// 8290F3D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F3D4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290F3D8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290F3DC: 48549665  bl 0x82e58a40
	ctx.lr = 0x8290F3E0;
	sub_82E58A40(ctx, base);
	// 8290F3E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290F3E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F3E8: 419A0008  beq cr6, 0x8290f3f0
	if ctx.cr[6].eq {
	pc = 0x8290F3F0; continue 'dispatch;
	}
	// 8290F3EC: 4B9B14A5  bl 0x822c0890
	ctx.lr = 0x8290F3F0;
	sub_822C0890(ctx, base);
	// 8290F3F0: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290F3F4: 480000B4  b 0x8290f4a8
	pc = 0x8290F4A8; continue 'dispatch;
	// 8290F3F8: 48007C39  bl 0x82917030
	ctx.lr = 0x8290F3FC;
	sub_82917030(ctx, base);
	// 8290F3FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F400: 408200B4  bne 0x8290f4b4
	if !ctx.cr[0].eq {
	pc = 0x8290F4B4; continue 'dispatch;
	}
	// 8290F404: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8290F408: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F40C: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8290F410: 48007C11  bl 0x82917020
	ctx.lr = 0x8290F414;
	sub_82917020(ctx, base);
	// 8290F414: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8290F418: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290F41C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290F420: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290F424: 4BF7144D  bl 0x82880870
	ctx.lr = 0x8290F428;
	sub_82880870(ctx, base);
	// 8290F428: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F42C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290F430: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F434: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8290F43C: 419A0024  beq cr6, 0x8290f460
	if ctx.cr[6].eq {
	pc = 0x8290F460; continue 'dispatch;
	}
	// 8290F440: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F444: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F448: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F44C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F450: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F454: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F458: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F45C: 4082FFE8  bne 0x8290f444
	if !ctx.cr[0].eq {
	pc = 0x8290F444; continue 'dispatch;
	}
	// 8290F460: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290F464: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 8290F468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F46C: 486F9B4D  bl 0x83008fb8
	ctx.lr = 0x8290F470;
	sub_83008FB8(ctx, base);
	// 8290F470: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F474: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290F478: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F47C: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290F480: 38A008C4  li r5, 0x8c4
	ctx.r[5].s64 = 2244;
	// 8290F484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F488: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290F48C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290F490: 485495B1  bl 0x82e58a40
	ctx.lr = 0x8290F494;
	sub_82E58A40(ctx, base);
	// 8290F494: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290F498: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F49C: 419A0008  beq cr6, 0x8290f4a4
	if ctx.cr[6].eq {
	pc = 0x8290F4A4; continue 'dispatch;
	}
	// 8290F4A0: 4B9B13F1  bl 0x822c0890
	ctx.lr = 0x8290F4A4;
	sub_822C0890(ctx, base);
	// 8290F4A4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290F4A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F4AC: 419A0008  beq cr6, 0x8290f4b4
	if ctx.cr[6].eq {
	pc = 0x8290F4B4; continue 'dispatch;
	}
	// 8290F4B0: 4B9B13E1  bl 0x822c0890
	ctx.lr = 0x8290F4B4;
	sub_822C0890(ctx, base);
	// 8290F4B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8290F4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290F4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290F4C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290F4C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290F4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290F4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290F4D0 size=184
    let mut pc: u32 = 0x8290F4D0;
    'dispatch: loop {
        match pc {
            0x8290F4D0 => {
    //   block [0x8290F4D0..0x8290F588)
	// 8290F4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290F4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290F4D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290F4DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290F4E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290F4E4: 817F02E0  lwz r11, 0x2e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(736 as u32) ) } as u64;
	// 8290F4E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F4EC: 419A0088  beq cr6, 0x8290f574
	if ctx.cr[6].eq {
	pc = 0x8290F574; continue 'dispatch;
	}
	// 8290F4F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290F4F4: 4BEF1365  bl 0x82800858
	ctx.lr = 0x8290F4F8;
	sub_82800858(ctx, base);
	// 8290F4F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F4FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290F500: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F504: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F508: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290F50C: 419A0024  beq cr6, 0x8290f530
	if ctx.cr[6].eq {
	pc = 0x8290F530; continue 'dispatch;
	}
	// 8290F510: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F514: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F518: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F51C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F520: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F524: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F528: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F52C: 4082FFE8  bne 0x8290f514
	if !ctx.cr[0].eq {
	pc = 0x8290F514; continue 'dispatch;
	}
	// 8290F530: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F534: 80DF02E0  lwz r6, 0x2e0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(736 as u32) ) } as u64;
	// 8290F538: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290F53C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290F540: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290F544: 38A00A2E  li r5, 0xa2e
	ctx.r[5].s64 = 2606;
	// 8290F548: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8290F54C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290F550: 485494F1  bl 0x82e58a40
	ctx.lr = 0x8290F554;
	sub_82E58A40(ctx, base);
	// 8290F554: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290F558: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F55C: 419A0008  beq cr6, 0x8290f564
	if ctx.cr[6].eq {
	pc = 0x8290F564; continue 'dispatch;
	}
	// 8290F560: 4B9B1331  bl 0x822c0890
	ctx.lr = 0x8290F564;
	sub_822C0890(ctx, base);
	// 8290F564: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290F568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F56C: 419A0008  beq cr6, 0x8290f574
	if ctx.cr[6].eq {
	pc = 0x8290F574; continue 'dispatch;
	}
	// 8290F570: 4B9B1321  bl 0x822c0890
	ctx.lr = 0x8290F574;
	sub_822C0890(ctx, base);
	// 8290F574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8290F578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290F57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290F580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290F584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290F588 size=236
    let mut pc: u32 = 0x8290F588;
    'dispatch: loop {
        match pc {
            0x8290F588 => {
    //   block [0x8290F588..0x8290F674)
	// 8290F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290F58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8290F590: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8290F594: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8290F598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290F59C: D021009C  stfs f1, 0x9c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8290F5A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290F5A4: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 8290F5A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8290F5AC: 4BA0DC9D  bl 0x8231d248
	ctx.lr = 0x8290F5B0;
	sub_8231D248(ctx, base);
	// 8290F5B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F5B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290F5B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F5BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F5C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8290F5C4: 419A0024  beq cr6, 0x8290f5e8
	if ctx.cr[6].eq {
	pc = 0x8290F5E8; continue 'dispatch;
	}
	// 8290F5C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F5CC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F5D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F5D4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F5D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F5DC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F5E0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F5E4: 4082FFE8  bne 0x8290f5cc
	if !ctx.cr[0].eq {
	pc = 0x8290F5CC; continue 'dispatch;
	}
	// 8290F5E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290F5EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290F5F0: 4BBFFED9  bl 0x8250f4c8
	ctx.lr = 0x8290F5F4;
	sub_8250F4C8(ctx, base);
	// 8290F5F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F5F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F5FC: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8290F600: 409A0008  bne cr6, 0x8290f608
	if !ctx.cr[6].eq {
	pc = 0x8290F608; continue 'dispatch;
	}
	// 8290F604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290F608: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8290F60C: 4BBF9475  bl 0x82508a80
	ctx.lr = 0x8290F610;
	sub_82508A80(ctx, base);
	// 8290F610: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F614: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8290F618: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F61C: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 8290F620: 38A00A35  li r5, 0xa35
	ctx.r[5].s64 = 2613;
	// 8290F624: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8290F628: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8290F62C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290F630: 48549411  bl 0x82e58a40
	ctx.lr = 0x8290F634;
	sub_82E58A40(ctx, base);
	// 8290F634: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290F638: 484E2659  bl 0x82df1c90
	ctx.lr = 0x8290F63C;
	sub_82DF1C90(ctx, base);
	// 8290F63C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290F640: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F644: 419A0008  beq cr6, 0x8290f64c
	if ctx.cr[6].eq {
	pc = 0x8290F64C; continue 'dispatch;
	}
	// 8290F648: 4B9B1249  bl 0x822c0890
	ctx.lr = 0x8290F64C;
	sub_822C0890(ctx, base);
	// 8290F64C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290F650: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F654: 419A0008  beq cr6, 0x8290f65c
	if ctx.cr[6].eq {
	pc = 0x8290F65C; continue 'dispatch;
	}
	// 8290F658: 4B9B1239  bl 0x822c0890
	ctx.lr = 0x8290F65C;
	sub_822C0890(ctx, base);
	// 8290F65C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8290F660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8290F664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8290F668: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8290F66C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8290F670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290F678 size=340
    let mut pc: u32 = 0x8290F678;
    'dispatch: loop {
        match pc {
            0x8290F678 => {
    //   block [0x8290F678..0x8290F7CC)
	// 8290F678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290F67C: 48898AED  bl 0x831a8168
	ctx.lr = 0x8290F680;
	sub_831A8130(ctx, base);
	// 8290F680: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8290F684: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290F688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290F68C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8290F690: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F694: 816B0158  lwz r11, 0x158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 8290F698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290F69C: 4E800421  bctrl
	ctx.lr = 0x8290F6A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290F6A0: 3BC300B8  addi r30, r3, 0xb8
	ctx.r[30].s64 = ctx.r[3].s64 + 184;
	// 8290F6A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290F6A8: 4804BDB1  bl 0x8295b458
	ctx.lr = 0x8290F6AC;
	sub_8295B458(ctx, base);
	// 8290F6AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8290F6B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290F6B4: 4804BDAD  bl 0x8295b460
	ctx.lr = 0x8290F6B8;
	sub_8295B460(ctx, base);
	// 8290F6B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290F6BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290F6C0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8290F6C4: 4804BDA5  bl 0x8295b468
	ctx.lr = 0x8290F6C8;
	sub_8295B468(ctx, base);
	// 8290F6C8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290F6CC: 419A00F0  beq cr6, 0x8290f7bc
	if ctx.cr[6].eq {
	pc = 0x8290F7BC; continue 'dispatch;
	}
	// 8290F6D0: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8290F6D4: 409A00C4  bne cr6, 0x8290f798
	if !ctx.cr[6].eq {
	pc = 0x8290F798; continue 'dispatch;
	}
	// 8290F6D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8290F6DC: 4BFFE705  bl 0x8290dde0
	ctx.lr = 0x8290F6E0;
	sub_8290DDE0(ctx, base);
	// 8290F6E0: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290F6E4: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8290F6E8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8290F6EC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8290F6F0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8290F6F4: 419A0024  beq cr6, 0x8290f718
	if ctx.cr[6].eq {
	pc = 0x8290F718; continue 'dispatch;
	}
	// 8290F6F8: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8290F6FC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F700: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F704: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F708: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F70C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F710: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F714: 4082FFE8  bne 0x8290f6fc
	if !ctx.cr[0].eq {
	pc = 0x8290F6FC; continue 'dispatch;
	}
	// 8290F718: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290F71C: 80DF02D4  lwz r6, 0x2d4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(724 as u32) ) } as u64;
	// 8290F720: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8290F724: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290F728: 38A00BCF  li r5, 0xbcf
	ctx.r[5].s64 = 3023;
	// 8290F72C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8290F730: 485478B9  bl 0x82e56fe8
	ctx.lr = 0x8290F734;
	sub_82E56FE8(ctx, base);
	// 8290F734: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8290F738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290F73C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F740: 419A000C  beq cr6, 0x8290f74c
	if ctx.cr[6].eq {
	pc = 0x8290F74C; continue 'dispatch;
	}
	// 8290F744: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8290F748: 4B9B1149  bl 0x822c0890
	ctx.lr = 0x8290F74C;
	sub_822C0890(ctx, base);
	// 8290F74C: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F750: 41820034  beq 0x8290f784
	if ctx.cr[0].eq {
	pc = 0x8290F784; continue 'dispatch;
	}
	// 8290F754: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8290F758: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8290F75C: 40980028  bge cr6, 0x8290f784
	if !ctx.cr[6].lt {
	pc = 0x8290F784; continue 'dispatch;
	}
	// 8290F760: C01C0018  lfs f0, 0x18(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290F764: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8290F768: 4098001C  bge cr6, 0x8290f784
	if !ctx.cr[6].lt {
	pc = 0x8290F784; continue 'dispatch;
	}
	// 8290F76C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8290F770: 419A000C  beq cr6, 0x8290f77c
	if ctx.cr[6].eq {
	pc = 0x8290F77C; continue 'dispatch;
	}
	// 8290F774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290F778: 4B9B1119  bl 0x822c0890
	ctx.lr = 0x8290F77C;
	sub_822C0890(ctx, base);
	// 8290F77C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8290F780: 48000040  b 0x8290f7c0
	pc = 0x8290F7C0; continue 'dispatch;
	// 8290F784: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8290F788: 419A0034  beq cr6, 0x8290f7bc
	if ctx.cr[6].eq {
	pc = 0x8290F7BC; continue 'dispatch;
	}
	// 8290F78C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290F790: 4B9B1101  bl 0x822c0890
	ctx.lr = 0x8290F794;
	sub_822C0890(ctx, base);
	// 8290F794: 48000028  b 0x8290f7bc
	pc = 0x8290F7BC; continue 'dispatch;
	// 8290F798: 2B1D0002  cmplwi cr6, r29, 2
	ctx.cr[6].compare_u32(ctx.r[29].u32, 2 as u32, &mut ctx.xer);
	// 8290F79C: 409A0020  bne cr6, 0x8290f7bc
	if !ctx.cr[6].eq {
	pc = 0x8290F7BC; continue 'dispatch;
	}
	// 8290F7A0: C01F032C  lfs f0, 0x32c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290F7A4: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8290F7A8: 41990014  bgt cr6, 0x8290f7bc
	if ctx.cr[6].gt {
	pc = 0x8290F7BC; continue 'dispatch;
	}
	// 8290F7AC: C01C0018  lfs f0, 0x18(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290F7B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8290F7B4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8290F7B8: 41980008  blt cr6, 0x8290f7c0
	if ctx.cr[6].lt {
	pc = 0x8290F7C0; continue 'dispatch;
	}
	// 8290F7BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8290F7C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8290F7C4: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8290F7C8: 488989F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290F7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8290F7D0 size=700
    let mut pc: u32 = 0x8290F7D0;
    'dispatch: loop {
        match pc {
            0x8290F7D0 => {
    //   block [0x8290F7D0..0x8290FA8C)
	// 8290F7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290F7D4: 48898995  bl 0x831a8168
	ctx.lr = 0x8290F7D8;
	sub_831A8130(ctx, base);
	// 8290F7D8: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8290F7DC: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8290F7E0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290F7E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8290F7E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F7EC: 816B015C  lwz r11, 0x15c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) } as u64;
	// 8290F7F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8290F7F4: 4E800421  bctrl
	ctx.lr = 0x8290F7F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8290F7F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8290F7FC: 41820280  beq 0x8290fa7c
	if ctx.cr[0].eq {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F800: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290F804: 4804B46D  bl 0x8295ac70
	ctx.lr = 0x8290F808;
	sub_8295AC70(ctx, base);
	// 8290F808: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8290F80C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8290F810: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8290F814: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 8290F818: 40990264  ble cr6, 0x8290fa7c
	if !ctx.cr[6].gt {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F81C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8290F820: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F824: 4800780D  bl 0x82917030
	ctx.lr = 0x8290F828;
	sub_82917030(ctx, base);
	// 8290F828: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F82C: 40820250  bne 0x8290fa7c
	if !ctx.cr[0].eq {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F830: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8290F834: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F838: 480077F9  bl 0x82917030
	ctx.lr = 0x8290F83C;
	sub_82917030(ctx, base);
	// 8290F83C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F840: 4082023C  bne 0x8290fa7c
	if !ctx.cr[0].eq {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F844: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8290F848: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F84C: 480077E5  bl 0x82917030
	ctx.lr = 0x8290F850;
	sub_82917030(ctx, base);
	// 8290F850: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F854: 40820228  bne 0x8290fa7c
	if !ctx.cr[0].eq {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F858: C01F032C  lfs f0, 0x32c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290F85C: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F860: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8290F864: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290F868: 419900A4  bgt cr6, 0x8290f90c
	if ctx.cr[6].gt {
	pc = 0x8290F90C; continue 'dispatch;
	}
	// 8290F86C: 480077C5  bl 0x82917030
	ctx.lr = 0x8290F870;
	sub_82917030(ctx, base);
	// 8290F870: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F874: 41820208  beq 0x8290fa7c
	if ctx.cr[0].eq {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F878: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290F87C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290F880: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290F884: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8290F888: 4BF70FE9  bl 0x82880870
	ctx.lr = 0x8290F88C;
	sub_82880870(ctx, base);
	// 8290F88C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F890: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8290F894: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F898: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F89C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8290F8A0: 419A0024  beq cr6, 0x8290f8c4
	if ctx.cr[6].eq {
	pc = 0x8290F8C4; continue 'dispatch;
	}
	// 8290F8A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F8A8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F8AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F8B0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F8B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F8B8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F8BC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F8C0: 4082FFE8  bne 0x8290f8a8
	if !ctx.cr[0].eq {
	pc = 0x8290F8A8; continue 'dispatch;
	}
	// 8290F8C4: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290F8C8: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8290F8CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F8D0: 486F96E9  bl 0x83008fb8
	ctx.lr = 0x8290F8D4;
	sub_83008FB8(ctx, base);
	// 8290F8D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290F8D8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F8DC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290F8E0: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 8290F8E4: 38A00C07  li r5, 0xc07
	ctx.r[5].s64 = 3079;
	// 8290F8E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290F8EC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290F8F0: 48549151  bl 0x82e58a40
	ctx.lr = 0x8290F8F4;
	sub_82E58A40(ctx, base);
	// 8290F8F4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8290F8F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F8FC: 419A0008  beq cr6, 0x8290f904
	if ctx.cr[6].eq {
	pc = 0x8290F904; continue 'dispatch;
	}
	// 8290F900: 4B9B0F91  bl 0x822c0890
	ctx.lr = 0x8290F904;
	sub_822C0890(ctx, base);
	// 8290F904: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8290F908: 48000168  b 0x8290fa70
	pc = 0x8290FA70; continue 'dispatch;
	// 8290F90C: 48007725  bl 0x82917030
	ctx.lr = 0x8290F910;
	sub_82917030(ctx, base);
	// 8290F910: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8290F914: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8290F918: 3BAB5758  addi r29, r11, 0x5758
	ctx.r[29].s64 = ctx.r[11].s64 + 22360;
	// 8290F91C: 408200A8  bne 0x8290f9c4
	if !ctx.cr[0].eq {
	pc = 0x8290F9C4; continue 'dispatch;
	}
	// 8290F920: C01F032C  lfs f0, 0x32c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290F924: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8290F928: 4099009C  ble cr6, 0x8290f9c4
	if !ctx.cr[6].gt {
	pc = 0x8290F9C4; continue 'dispatch;
	}
	// 8290F92C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8290F930: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290F934: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290F938: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8290F93C: 4BF70F35  bl 0x82880870
	ctx.lr = 0x8290F940;
	sub_82880870(ctx, base);
	// 8290F940: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F944: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8290F948: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290F94C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290F950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8290F954: 419A0024  beq cr6, 0x8290f978
	if ctx.cr[6].eq {
	pc = 0x8290F978; continue 'dispatch;
	}
	// 8290F958: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290F95C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290F960: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F964: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290F968: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290F96C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290F970: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290F974: 4082FFE8  bne 0x8290f95c
	if !ctx.cr[0].eq {
	pc = 0x8290F95C; continue 'dispatch;
	}
	// 8290F978: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 8290F97C: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 8290F980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290F984: 486F9635  bl 0x83008fb8
	ctx.lr = 0x8290F988;
	sub_83008FB8(ctx, base);
	// 8290F988: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290F98C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290F990: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290F994: 38A00C0F  li r5, 0xc0f
	ctx.r[5].s64 = 3087;
	// 8290F998: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8290F99C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8290F9A0: 485490A1  bl 0x82e58a40
	ctx.lr = 0x8290F9A4;
	sub_82E58A40(ctx, base);
	// 8290F9A4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8290F9A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F9AC: 419A0008  beq cr6, 0x8290f9b4
	if ctx.cr[6].eq {
	pc = 0x8290F9B4; continue 'dispatch;
	}
	// 8290F9B0: 4B9B0EE1  bl 0x822c0890
	ctx.lr = 0x8290F9B4;
	sub_822C0890(ctx, base);
	// 8290F9B4: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8290F9B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290F9BC: 419A0008  beq cr6, 0x8290f9c4
	if ctx.cr[6].eq {
	pc = 0x8290F9C4; continue 'dispatch;
	}
	// 8290F9C0: 4B9B0ED1  bl 0x822c0890
	ctx.lr = 0x8290F9C4;
	sub_822C0890(ctx, base);
	// 8290F9C4: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8290F9C8: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8290F9CC: 48007665  bl 0x82917030
	ctx.lr = 0x8290F9D0;
	sub_82917030(ctx, base);
	// 8290F9D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290F9D4: 418200A8  beq 0x8290fa7c
	if ctx.cr[0].eq {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F9D8: C01F032C  lfs f0, 0x32c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(812 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8290F9DC: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8290F9E0: 4098009C  bge cr6, 0x8290fa7c
	if !ctx.cr[6].lt {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290F9E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8290F9E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8290F9EC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8290F9F0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8290F9F4: 4BF70E7D  bl 0x82880870
	ctx.lr = 0x8290F9F8;
	sub_82880870(ctx, base);
	// 8290F9F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8290F9FC: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8290FA00: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8290FA04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8290FA08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8290FA0C: 419A0024  beq cr6, 0x8290fa30
	if ctx.cr[6].eq {
	pc = 0x8290FA30; continue 'dispatch;
	}
	// 8290FA10: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8290FA14: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8290FA18: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290FA1C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8290FA20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8290FA24: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8290FA28: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8290FA2C: 4082FFE8  bne 0x8290fa14
	if !ctx.cr[0].eq {
	pc = 0x8290FA14; continue 'dispatch;
	}
	// 8290FA30: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8290FA34: 3BC10068  addi r30, r1, 0x68
	ctx.r[30].s64 = ctx.r[1].s64 + 104;
	// 8290FA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FA3C: 486F957D  bl 0x83008fb8
	ctx.lr = 0x8290FA40;
	sub_83008FB8(ctx, base);
	// 8290FA40: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8290FA44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8290FA48: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8290FA4C: 38A00C14  li r5, 0xc14
	ctx.r[5].s64 = 3092;
	// 8290FA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FA54: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8290FA58: 48548FE9  bl 0x82e58a40
	ctx.lr = 0x8290FA5C;
	sub_82E58A40(ctx, base);
	// 8290FA5C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8290FA60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290FA64: 419A0008  beq cr6, 0x8290fa6c
	if ctx.cr[6].eq {
	pc = 0x8290FA6C; continue 'dispatch;
	}
	// 8290FA68: 4B9B0E29  bl 0x822c0890
	ctx.lr = 0x8290FA6C;
	sub_822C0890(ctx, base);
	// 8290FA6C: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8290FA70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8290FA74: 419A0008  beq cr6, 0x8290fa7c
	if ctx.cr[6].eq {
	pc = 0x8290FA7C; continue 'dispatch;
	}
	// 8290FA78: 4B9B0E19  bl 0x822c0890
	ctx.lr = 0x8290FA7C;
	sub_822C0890(ctx, base);
	// 8290FA7C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8290FA80: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8290FA84: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8290FA88: 48898730  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8290FA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8290FA90 size=2068
    let mut pc: u32 = 0x8290FA90;
    'dispatch: loop {
        match pc {
            0x8290FA90 => {
    //   block [0x8290FA90..0x829102A4)
	// 8290FA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8290FA94: 488986D5  bl 0x831a8168
	ctx.lr = 0x8290FA98;
	sub_831A8130(ctx, base);
	// 8290FA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8290FA9C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8290FAA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8290FAA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8290FAA8: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8290FAAC: 41820038  beq 0x8290fae4
	if ctx.cr[0].eq {
	pc = 0x8290FAE4; continue 'dispatch;
	}
	// 8290FAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FAB4: 48899ED5  bl 0x831a9988
	ctx.lr = 0x8290FAB8;
	sub_831A9988(ctx, base);
	// 8290FAB8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FABC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FAC0: 386BE300  addi r3, r11, -0x1d00
	ctx.r[3].s64 = ctx.r[11].s64 + -7424;
	// 8290FAC4: 48898635  bl 0x831a80f8
	ctx.lr = 0x8290FAC8;
	sub_831A80F8(ctx, base);
	// 8290FAC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FACC: 41820018  beq 0x8290fae4
	if ctx.cr[0].eq {
	pc = 0x8290FAE4; continue 'dispatch;
	}
	// 8290FAD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FAD4: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FAD8: 4BFFC2D1  bl 0x8290bda8
	ctx.lr = 0x8290FADC;
	sub_8290BDA8(ctx, base);
	// 8290FADC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8290FAE0: 480007BC  b 0x8291029c
	pc = 0x8291029C; continue 'dispatch;
	// 8290FAE4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FAE8: 419A07A4  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FAEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FAF0: 48899E99  bl 0x831a9988
	ctx.lr = 0x8290FAF4;
	sub_831A9988(ctx, base);
	// 8290FAF4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8290FAF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FAFC: 386B5A88  addi r3, r11, 0x5a88
	ctx.r[3].s64 = ctx.r[11].s64 + 23176;
	// 8290FB00: 488985F9  bl 0x831a80f8
	ctx.lr = 0x8290FB04;
	sub_831A80F8(ctx, base);
	// 8290FB04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FB08: 41820014  beq 0x8290fb1c
	if ctx.cr[0].eq {
	pc = 0x8290FB1C; continue 'dispatch;
	}
	// 8290FB0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FB10: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FB14: 4BFFF34D  bl 0x8290ee60
	ctx.lr = 0x8290FB18;
	sub_8290EE60(ctx, base);
	// 8290FB18: 4BFFFFC4  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FB1C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FB20: 419A076C  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FB24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FB28: 48899E61  bl 0x831a9988
	ctx.lr = 0x8290FB2C;
	sub_831A9988(ctx, base);
	// 8290FB2C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8290FB30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FB34: 386BDFF4  addi r3, r11, -0x200c
	ctx.r[3].s64 = ctx.r[11].s64 + -8204;
	// 8290FB38: 488985C1  bl 0x831a80f8
	ctx.lr = 0x8290FB3C;
	sub_831A80F8(ctx, base);
	// 8290FB3C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FB40: 41820014  beq 0x8290fb54
	if ctx.cr[0].eq {
	pc = 0x8290FB54; continue 'dispatch;
	}
	// 8290FB44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FB48: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FB4C: 4BFF9515  bl 0x82909060
	ctx.lr = 0x8290FB50;
	sub_82909060(ctx, base);
	// 8290FB50: 4BFFFF8C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FB54: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FB58: 419A0734  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FB5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FB60: 48899E29  bl 0x831a9988
	ctx.lr = 0x8290FB64;
	sub_831A9988(ctx, base);
	// 8290FB64: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8290FB68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FB6C: 386BDD38  addi r3, r11, -0x22c8
	ctx.r[3].s64 = ctx.r[11].s64 + -8904;
	// 8290FB70: 48898589  bl 0x831a80f8
	ctx.lr = 0x8290FB74;
	sub_831A80F8(ctx, base);
	// 8290FB74: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FB78: 41820014  beq 0x8290fb8c
	if ctx.cr[0].eq {
	pc = 0x8290FB8C; continue 'dispatch;
	}
	// 8290FB7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FB80: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FB84: 4BFFA32D  bl 0x82909eb0
	ctx.lr = 0x8290FB88;
	sub_82909EB0(ctx, base);
	// 8290FB88: 4BFFFF54  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FB8C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FB90: 419A06FC  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FB98: 48899DF1  bl 0x831a9988
	ctx.lr = 0x8290FB9C;
	sub_831A9988(ctx, base);
	// 8290FB9C: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8290FBA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FBA4: 386BDCF0  addi r3, r11, -0x2310
	ctx.r[3].s64 = ctx.r[11].s64 + -8976;
	// 8290FBA8: 48898551  bl 0x831a80f8
	ctx.lr = 0x8290FBAC;
	sub_831A80F8(ctx, base);
	// 8290FBAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FBB0: 41820014  beq 0x8290fbc4
	if ctx.cr[0].eq {
	pc = 0x8290FBC4; continue 'dispatch;
	}
	// 8290FBB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FBB8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FBBC: 4BFFA2FD  bl 0x82909eb8
	ctx.lr = 0x8290FBC0;
	sub_82909EB8(ctx, base);
	// 8290FBC0: 4BFFFF1C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FBC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FBC8: 419A06C4  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FBD0: 48899DB9  bl 0x831a9988
	ctx.lr = 0x8290FBD4;
	sub_831A9988(ctx, base);
	// 8290FBD4: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8290FBD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FBDC: 386BFBC0  addi r3, r11, -0x440
	ctx.r[3].s64 = ctx.r[11].s64 + -1088;
	// 8290FBE0: 48898519  bl 0x831a80f8
	ctx.lr = 0x8290FBE4;
	sub_831A80F8(ctx, base);
	// 8290FBE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FBE8: 41820014  beq 0x8290fbfc
	if ctx.cr[0].eq {
	pc = 0x8290FBFC; continue 'dispatch;
	}
	// 8290FBEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FBF0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FBF4: 4BFFA2CD  bl 0x82909ec0
	ctx.lr = 0x8290FBF8;
	sub_82909EC0(ctx, base);
	// 8290FBF8: 4BFFFEE4  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FBFC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FC00: 419A068C  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FC04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FC08: 48899D81  bl 0x831a9988
	ctx.lr = 0x8290FC0C;
	sub_831A9988(ctx, base);
	// 8290FC0C: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8290FC10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FC14: 386BDCB0  addi r3, r11, -0x2350
	ctx.r[3].s64 = ctx.r[11].s64 + -9040;
	// 8290FC18: 488984E1  bl 0x831a80f8
	ctx.lr = 0x8290FC1C;
	sub_831A80F8(ctx, base);
	// 8290FC1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FC20: 41820014  beq 0x8290fc34
	if ctx.cr[0].eq {
	pc = 0x8290FC34; continue 'dispatch;
	}
	// 8290FC24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FC28: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FC2C: 4BFFF2FD  bl 0x8290ef28
	ctx.lr = 0x8290FC30;
	sub_8290EF28(ctx, base);
	// 8290FC30: 4BFFFEAC  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FC34: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FC38: 419A0654  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FC3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FC40: 48899D49  bl 0x831a9988
	ctx.lr = 0x8290FC44;
	sub_831A9988(ctx, base);
	// 8290FC44: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8290FC48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FC4C: 386BDC74  addi r3, r11, -0x238c
	ctx.r[3].s64 = ctx.r[11].s64 + -9100;
	// 8290FC50: 488984A9  bl 0x831a80f8
	ctx.lr = 0x8290FC54;
	sub_831A80F8(ctx, base);
	// 8290FC54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FC58: 41820014  beq 0x8290fc6c
	if ctx.cr[0].eq {
	pc = 0x8290FC6C; continue 'dispatch;
	}
	// 8290FC5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FC60: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FC64: 4BFFF4F5  bl 0x8290f158
	ctx.lr = 0x8290FC68;
	sub_8290F158(ctx, base);
	// 8290FC68: 4BFFFE74  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FC6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FC70: 419A061C  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FC78: 48899D11  bl 0x831a9988
	ctx.lr = 0x8290FC7C;
	sub_831A9988(ctx, base);
	// 8290FC7C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8290FC80: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FC84: 386BF928  addi r3, r11, -0x6d8
	ctx.r[3].s64 = ctx.r[11].s64 + -1752;
	// 8290FC88: 48898471  bl 0x831a80f8
	ctx.lr = 0x8290FC8C;
	sub_831A80F8(ctx, base);
	// 8290FC8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FC90: 41820014  beq 0x8290fca4
	if ctx.cr[0].eq {
	pc = 0x8290FCA4; continue 'dispatch;
	}
	// 8290FC94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FC98: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FC9C: 4BFFA17D  bl 0x82909e18
	ctx.lr = 0x8290FCA0;
	sub_82909E18(ctx, base);
	// 8290FCA0: 4BFFFE3C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FCA4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FCA8: 419A05E4  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FCB0: 48899CD9  bl 0x831a9988
	ctx.lr = 0x8290FCB4;
	sub_831A9988(ctx, base);
	// 8290FCB4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FCB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FCBC: 386BE2D4  addi r3, r11, -0x1d2c
	ctx.r[3].s64 = ctx.r[11].s64 + -7468;
	// 8290FCC0: 48898439  bl 0x831a80f8
	ctx.lr = 0x8290FCC4;
	sub_831A80F8(ctx, base);
	// 8290FCC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FCC8: 41820014  beq 0x8290fcdc
	if ctx.cr[0].eq {
	pc = 0x8290FCDC; continue 'dispatch;
	}
	// 8290FCCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FCD0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FCD4: 4BFFC27D  bl 0x8290bf50
	ctx.lr = 0x8290FCD8;
	sub_8290BF50(ctx, base);
	// 8290FCD8: 4BFFFE04  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FCDC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FCE0: 419A05AC  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FCE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FCE8: 48899CA1  bl 0x831a9988
	ctx.lr = 0x8290FCEC;
	sub_831A9988(ctx, base);
	// 8290FCEC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FCF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FCF4: 386BE2A8  addi r3, r11, -0x1d58
	ctx.r[3].s64 = ctx.r[11].s64 + -7512;
	// 8290FCF8: 48898401  bl 0x831a80f8
	ctx.lr = 0x8290FCFC;
	sub_831A80F8(ctx, base);
	// 8290FCFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FD00: 41820014  beq 0x8290fd14
	if ctx.cr[0].eq {
	pc = 0x8290FD14; continue 'dispatch;
	}
	// 8290FD04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FD08: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FD0C: 4BFFC555  bl 0x8290c260
	ctx.lr = 0x8290FD10;
	sub_8290C260(ctx, base);
	// 8290FD10: 4BFFFDCC  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FD14: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FD18: 419A0574  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FD1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FD20: 48899C69  bl 0x831a9988
	ctx.lr = 0x8290FD24;
	sub_831A9988(ctx, base);
	// 8290FD24: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FD28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FD2C: 386BE274  addi r3, r11, -0x1d8c
	ctx.r[3].s64 = ctx.r[11].s64 + -7564;
	// 8290FD30: 488983C9  bl 0x831a80f8
	ctx.lr = 0x8290FD34;
	sub_831A80F8(ctx, base);
	// 8290FD34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FD38: 41820014  beq 0x8290fd4c
	if ctx.cr[0].eq {
	pc = 0x8290FD4C; continue 'dispatch;
	}
	// 8290FD3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FD40: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FD44: 4BFF933D  bl 0x82909080
	ctx.lr = 0x8290FD48;
	sub_82909080(ctx, base);
	// 8290FD48: 4BFFFD94  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FD4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FD50: 419A053C  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FD54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FD58: 48899C31  bl 0x831a9988
	ctx.lr = 0x8290FD5C;
	sub_831A9988(ctx, base);
	// 8290FD5C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FD60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FD64: 386BE244  addi r3, r11, -0x1dbc
	ctx.r[3].s64 = ctx.r[11].s64 + -7612;
	// 8290FD68: 48898391  bl 0x831a80f8
	ctx.lr = 0x8290FD6C;
	sub_831A80F8(ctx, base);
	// 8290FD6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FD70: 41820014  beq 0x8290fd84
	if ctx.cr[0].eq {
	pc = 0x8290FD84; continue 'dispatch;
	}
	// 8290FD74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FD78: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FD7C: 4BFFCFCD  bl 0x8290cd48
	ctx.lr = 0x8290FD80;
	sub_8290CD48(ctx, base);
	// 8290FD80: 4BFFFD5C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FD84: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FD88: 419A0504  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FD8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FD90: 48899BF9  bl 0x831a9988
	ctx.lr = 0x8290FD94;
	sub_831A9988(ctx, base);
	// 8290FD94: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FD98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FD9C: 386BE214  addi r3, r11, -0x1dec
	ctx.r[3].s64 = ctx.r[11].s64 + -7660;
	// 8290FDA0: 48898359  bl 0x831a80f8
	ctx.lr = 0x8290FDA4;
	sub_831A80F8(ctx, base);
	// 8290FDA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FDA8: 41820014  beq 0x8290fdbc
	if ctx.cr[0].eq {
	pc = 0x8290FDBC; continue 'dispatch;
	}
	// 8290FDAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FDB0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FDB4: 4BFFA115  bl 0x82909ec8
	ctx.lr = 0x8290FDB8;
	sub_82909EC8(ctx, base);
	// 8290FDB8: 4BFFFD24  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FDBC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FDC0: 419A04CC  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FDC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FDC8: 48899BC1  bl 0x831a9988
	ctx.lr = 0x8290FDCC;
	sub_831A9988(ctx, base);
	// 8290FDCC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8290FDD0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FDD4: 386B75A4  addi r3, r11, 0x75a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30116;
	// 8290FDD8: 48898321  bl 0x831a80f8
	ctx.lr = 0x8290FDDC;
	sub_831A80F8(ctx, base);
	// 8290FDDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FDE0: 41820014  beq 0x8290fdf4
	if ctx.cr[0].eq {
	pc = 0x8290FDF4; continue 'dispatch;
	}
	// 8290FDE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FDE8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FDEC: 4BFF94D5  bl 0x829092c0
	ctx.lr = 0x8290FDF0;
	sub_829092C0(ctx, base);
	// 8290FDF0: 4BFFFCEC  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FDF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FDF8: 419A0494  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FDFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FE00: 48899B89  bl 0x831a9988
	ctx.lr = 0x8290FE04;
	sub_831A9988(ctx, base);
	// 8290FE04: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8290FE08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FE0C: 386B595C  addi r3, r11, 0x595c
	ctx.r[3].s64 = ctx.r[11].s64 + 22876;
	// 8290FE10: 488982E9  bl 0x831a80f8
	ctx.lr = 0x8290FE14;
	sub_831A80F8(ctx, base);
	// 8290FE14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FE18: 41820014  beq 0x8290fe2c
	if ctx.cr[0].eq {
	pc = 0x8290FE2C; continue 'dispatch;
	}
	// 8290FE1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FE20: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FE24: 4BFF94DD  bl 0x82909300
	ctx.lr = 0x8290FE28;
	sub_82909300(ctx, base);
	// 8290FE28: 4BFFFCB4  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FE2C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FE30: 419A045C  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FE34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FE38: 48899B51  bl 0x831a9988
	ctx.lr = 0x8290FE3C;
	sub_831A9988(ctx, base);
	// 8290FE3C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8290FE40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FE44: 386B59B0  addi r3, r11, 0x59b0
	ctx.r[3].s64 = ctx.r[11].s64 + 22960;
	// 8290FE48: 488982B1  bl 0x831a80f8
	ctx.lr = 0x8290FE4C;
	sub_831A80F8(ctx, base);
	// 8290FE4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FE50: 41820014  beq 0x8290fe64
	if ctx.cr[0].eq {
	pc = 0x8290FE64; continue 'dispatch;
	}
	// 8290FE54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FE58: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FE5C: 4BFF94E5  bl 0x82909340
	ctx.lr = 0x8290FE60;
	sub_82909340(ctx, base);
	// 8290FE60: 4BFFFC7C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FE64: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FE68: 419A0424  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8290FE6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FE70: 48899B19  bl 0x831a9988
	ctx.lr = 0x8290FE74;
	sub_831A9988(ctx, base);
	// 8290FE74: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8290FE78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FE7C: 386B5984  addi r3, r11, 0x5984
	ctx.r[3].s64 = ctx.r[11].s64 + 22916;
	// 8290FE80: 48898279  bl 0x831a80f8
	ctx.lr = 0x8290FE84;
	sub_831A80F8(ctx, base);
	// 8290FE84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FE88: 41820014  beq 0x8290fe9c
	if ctx.cr[0].eq {
	pc = 0x8290FE9C; continue 'dispatch;
	}
	// 8290FE8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FE90: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FE94: 4BFF94ED  bl 0x82909380
	ctx.lr = 0x8290FE98;
	sub_82909380(ctx, base);
	// 8290FE98: 4BFFFC44  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FE9C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FEA0: 419A0034  beq cr6, 0x8290fed4
	if ctx.cr[6].eq {
	pc = 0x8290FED4; continue 'dispatch;
	}
	// 8290FEA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FEA8: 48899AE1  bl 0x831a9988
	ctx.lr = 0x8290FEAC;
	sub_831A9988(ctx, base);
	// 8290FEAC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FEB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FEB4: 386BE1DC  addi r3, r11, -0x1e24
	ctx.r[3].s64 = ctx.r[11].s64 + -7716;
	// 8290FEB8: 48898241  bl 0x831a80f8
	ctx.lr = 0x8290FEBC;
	sub_831A80F8(ctx, base);
	// 8290FEBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FEC0: 41820014  beq 0x8290fed4
	if ctx.cr[0].eq {
	pc = 0x8290FED4; continue 'dispatch;
	}
	// 8290FEC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FEC8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FECC: 4BFF9405  bl 0x829092d0
	ctx.lr = 0x8290FED0;
	sub_829092D0(ctx, base);
	// 8290FED0: 4BFFFC0C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FED4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FED8: 419A0034  beq cr6, 0x8290ff0c
	if ctx.cr[6].eq {
	pc = 0x8290FF0C; continue 'dispatch;
	}
	// 8290FEDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FEE0: 48899AA9  bl 0x831a9988
	ctx.lr = 0x8290FEE4;
	sub_831A9988(ctx, base);
	// 8290FEE4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FEE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FEEC: 386BE1AC  addi r3, r11, -0x1e54
	ctx.r[3].s64 = ctx.r[11].s64 + -7764;
	// 8290FEF0: 48898209  bl 0x831a80f8
	ctx.lr = 0x8290FEF4;
	sub_831A80F8(ctx, base);
	// 8290FEF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FEF8: 41820014  beq 0x8290ff0c
	if ctx.cr[0].eq {
	pc = 0x8290FF0C; continue 'dispatch;
	}
	// 8290FEFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FF00: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FF04: 4BFF940D  bl 0x82909310
	ctx.lr = 0x8290FF08;
	sub_82909310(ctx, base);
	// 8290FF08: 4BFFFBD4  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FF0C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FF10: 419A0034  beq cr6, 0x8290ff44
	if ctx.cr[6].eq {
	pc = 0x8290FF44; continue 'dispatch;
	}
	// 8290FF14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FF18: 48899A71  bl 0x831a9988
	ctx.lr = 0x8290FF1C;
	sub_831A9988(ctx, base);
	// 8290FF1C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8290FF20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FF24: 386BEEF8  addi r3, r11, -0x1108
	ctx.r[3].s64 = ctx.r[11].s64 + -4360;
	// 8290FF28: 488981D1  bl 0x831a80f8
	ctx.lr = 0x8290FF2C;
	sub_831A80F8(ctx, base);
	// 8290FF2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FF30: 41820014  beq 0x8290ff44
	if ctx.cr[0].eq {
	pc = 0x8290FF44; continue 'dispatch;
	}
	// 8290FF34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FF38: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FF3C: 4BFF9415  bl 0x82909350
	ctx.lr = 0x8290FF40;
	sub_82909350(ctx, base);
	// 8290FF40: 4BFFFB9C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FF44: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FF48: 419A0034  beq cr6, 0x8290ff7c
	if ctx.cr[6].eq {
	pc = 0x8290FF7C; continue 'dispatch;
	}
	// 8290FF4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FF50: 48899A39  bl 0x831a9988
	ctx.lr = 0x8290FF54;
	sub_831A9988(ctx, base);
	// 8290FF54: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FF58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FF5C: 386BE178  addi r3, r11, -0x1e88
	ctx.r[3].s64 = ctx.r[11].s64 + -7816;
	// 8290FF60: 48898199  bl 0x831a80f8
	ctx.lr = 0x8290FF64;
	sub_831A80F8(ctx, base);
	// 8290FF64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FF68: 41820014  beq 0x8290ff7c
	if ctx.cr[0].eq {
	pc = 0x8290FF7C; continue 'dispatch;
	}
	// 8290FF6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FF70: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FF74: 4BFF941D  bl 0x82909390
	ctx.lr = 0x8290FF78;
	sub_82909390(ctx, base);
	// 8290FF78: 4BFFFB64  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FF7C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FF80: 419A0034  beq cr6, 0x8290ffb4
	if ctx.cr[6].eq {
	pc = 0x8290FFB4; continue 'dispatch;
	}
	// 8290FF84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FF88: 48899A01  bl 0x831a9988
	ctx.lr = 0x8290FF8C;
	sub_831A9988(ctx, base);
	// 8290FF8C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FF90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FF94: 386BE14C  addi r3, r11, -0x1eb4
	ctx.r[3].s64 = ctx.r[11].s64 + -7860;
	// 8290FF98: 48898161  bl 0x831a80f8
	ctx.lr = 0x8290FF9C;
	sub_831A80F8(ctx, base);
	// 8290FF9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FFA0: 41820014  beq 0x8290ffb4
	if ctx.cr[0].eq {
	pc = 0x8290FFB4; continue 'dispatch;
	}
	// 8290FFA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FFA8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FFAC: 4BFF9335  bl 0x829092e0
	ctx.lr = 0x8290FFB0;
	sub_829092E0(ctx, base);
	// 8290FFB0: 4BFFFB2C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FFB4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FFB8: 419A0034  beq cr6, 0x8290ffec
	if ctx.cr[6].eq {
	pc = 0x8290FFEC; continue 'dispatch;
	}
	// 8290FFBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FFC0: 488999C9  bl 0x831a9988
	ctx.lr = 0x8290FFC4;
	sub_831A9988(ctx, base);
	// 8290FFC4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8290FFC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8290FFCC: 386BE11C  addi r3, r11, -0x1ee4
	ctx.r[3].s64 = ctx.r[11].s64 + -7908;
	// 8290FFD0: 48898129  bl 0x831a80f8
	ctx.lr = 0x8290FFD4;
	sub_831A80F8(ctx, base);
	// 8290FFD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8290FFD8: 41820014  beq 0x8290ffec
	if ctx.cr[0].eq {
	pc = 0x8290FFEC; continue 'dispatch;
	}
	// 8290FFDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8290FFE0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8290FFE4: 4BFF933D  bl 0x82909320
	ctx.lr = 0x8290FFE8;
	sub_82909320(ctx, base);
	// 8290FFE8: 4BFFFAF4  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8290FFEC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8290FFF0: 419A0034  beq cr6, 0x82910024
	if ctx.cr[6].eq {
	pc = 0x82910024; continue 'dispatch;
	}
	// 8290FFF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8290FFF8: 48899991  bl 0x831a9988
	ctx.lr = 0x8290FFFC;
	sub_831A9988(ctx, base);
	// 8290FFFC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 82910000: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910004: 386BDF88  addi r3, r11, -0x2078
	ctx.r[3].s64 = ctx.r[11].s64 + -8312;
	// 82910008: 488980F1  bl 0x831a80f8
	ctx.lr = 0x8291000C;
	sub_831A80F8(ctx, base);
	// 8291000C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910010: 41820014  beq 0x82910024
	if ctx.cr[0].eq {
	pc = 0x82910024; continue 'dispatch;
	}
	// 82910014: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910018: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8291001C: 4BFF9345  bl 0x82909360
	ctx.lr = 0x82910020;
	sub_82909360(ctx, base);
	// 82910020: 4BFFFABC  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 82910024: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910028: 419A0034  beq cr6, 0x8291005c
	if ctx.cr[6].eq {
	pc = 0x8291005C; continue 'dispatch;
	}
	// 8291002C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910030: 48899959  bl 0x831a9988
	ctx.lr = 0x82910034;
	sub_831A9988(ctx, base);
	// 82910034: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82910038: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8291003C: 386BE0E4  addi r3, r11, -0x1f1c
	ctx.r[3].s64 = ctx.r[11].s64 + -7964;
	// 82910040: 488980B9  bl 0x831a80f8
	ctx.lr = 0x82910044;
	sub_831A80F8(ctx, base);
	// 82910044: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910048: 41820014  beq 0x8291005c
	if ctx.cr[0].eq {
	pc = 0x8291005C; continue 'dispatch;
	}
	// 8291004C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910050: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82910054: 4BFF934D  bl 0x829093a0
	ctx.lr = 0x82910058;
	sub_829093A0(ctx, base);
	// 82910058: 4BFFFA84  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8291005C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910060: 419A0034  beq cr6, 0x82910094
	if ctx.cr[6].eq {
	pc = 0x82910094; continue 'dispatch;
	}
	// 82910064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910068: 48899921  bl 0x831a9988
	ctx.lr = 0x8291006C;
	sub_831A9988(ctx, base);
	// 8291006C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82910070: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910074: 386B66B4  addi r3, r11, 0x66b4
	ctx.r[3].s64 = ctx.r[11].s64 + 26292;
	// 82910078: 48898081  bl 0x831a80f8
	ctx.lr = 0x8291007C;
	sub_831A80F8(ctx, base);
	// 8291007C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910080: 41820014  beq 0x82910094
	if ctx.cr[0].eq {
	pc = 0x82910094; continue 'dispatch;
	}
	// 82910084: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910088: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8291008C: 4BFF9005  bl 0x82909090
	ctx.lr = 0x82910090;
	sub_82909090(ctx, base);
	// 82910090: 4BFFFA4C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 82910094: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910098: 419A0034  beq cr6, 0x829100cc
	if ctx.cr[6].eq {
	pc = 0x829100CC; continue 'dispatch;
	}
	// 8291009C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829100A0: 488998E9  bl 0x831a9988
	ctx.lr = 0x829100A4;
	sub_831A9988(ctx, base);
	// 829100A4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829100A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829100AC: 386BE0B4  addi r3, r11, -0x1f4c
	ctx.r[3].s64 = ctx.r[11].s64 + -8012;
	// 829100B0: 48898049  bl 0x831a80f8
	ctx.lr = 0x829100B4;
	sub_831A80F8(ctx, base);
	// 829100B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829100B8: 41820014  beq 0x829100cc
	if ctx.cr[0].eq {
	pc = 0x829100CC; continue 'dispatch;
	}
	// 829100BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829100C0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 829100C4: 4BFF922D  bl 0x829092f0
	ctx.lr = 0x829100C8;
	sub_829092F0(ctx, base);
	// 829100C8: 4BFFFA14  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 829100CC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 829100D0: 419A0034  beq cr6, 0x82910104
	if ctx.cr[6].eq {
	pc = 0x82910104; continue 'dispatch;
	}
	// 829100D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829100D8: 488998B1  bl 0x831a9988
	ctx.lr = 0x829100DC;
	sub_831A9988(ctx, base);
	// 829100DC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 829100E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829100E4: 386BD788  addi r3, r11, -0x2878
	ctx.r[3].s64 = ctx.r[11].s64 + -10360;
	// 829100E8: 48898011  bl 0x831a80f8
	ctx.lr = 0x829100EC;
	sub_831A80F8(ctx, base);
	// 829100EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829100F0: 41820014  beq 0x82910104
	if ctx.cr[0].eq {
	pc = 0x82910104; continue 'dispatch;
	}
	// 829100F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829100F8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 829100FC: 4BFF9235  bl 0x82909330
	ctx.lr = 0x82910100;
	sub_82909330(ctx, base);
	// 82910100: 4BFFF9DC  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 82910104: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910108: 419A0034  beq cr6, 0x8291013c
	if ctx.cr[6].eq {
	pc = 0x8291013C; continue 'dispatch;
	}
	// 8291010C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910110: 48899879  bl 0x831a9988
	ctx.lr = 0x82910114;
	sub_831A9988(ctx, base);
	// 82910114: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82910118: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8291011C: 386B1B00  addi r3, r11, 0x1b00
	ctx.r[3].s64 = ctx.r[11].s64 + 6912;
	// 82910120: 48897FD9  bl 0x831a80f8
	ctx.lr = 0x82910124;
	sub_831A80F8(ctx, base);
	// 82910124: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910128: 41820014  beq 0x8291013c
	if ctx.cr[0].eq {
	pc = 0x8291013C; continue 'dispatch;
	}
	// 8291012C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910130: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82910134: 4BFF923D  bl 0x82909370
	ctx.lr = 0x82910138;
	sub_82909370(ctx, base);
	// 82910138: 4BFFF9A4  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8291013C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910140: 419A0034  beq cr6, 0x82910174
	if ctx.cr[6].eq {
	pc = 0x82910174; continue 'dispatch;
	}
	// 82910144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910148: 48899841  bl 0x831a9988
	ctx.lr = 0x8291014C;
	sub_831A9988(ctx, base);
	// 8291014C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82910150: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910154: 386B3650  addi r3, r11, 0x3650
	ctx.r[3].s64 = ctx.r[11].s64 + 13904;
	// 82910158: 48897FA1  bl 0x831a80f8
	ctx.lr = 0x8291015C;
	sub_831A80F8(ctx, base);
	// 8291015C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910160: 41820014  beq 0x82910174
	if ctx.cr[0].eq {
	pc = 0x82910174; continue 'dispatch;
	}
	// 82910164: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910168: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8291016C: 4BFFD22D  bl 0x8290d398
	ctx.lr = 0x82910170;
	sub_8290D398(ctx, base);
	// 82910170: 4BFFF96C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 82910174: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910178: 419A0034  beq cr6, 0x829101ac
	if ctx.cr[6].eq {
	pc = 0x829101AC; continue 'dispatch;
	}
	// 8291017C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910180: 48899809  bl 0x831a9988
	ctx.lr = 0x82910184;
	sub_831A9988(ctx, base);
	// 82910184: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82910188: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8291018C: 386B3644  addi r3, r11, 0x3644
	ctx.r[3].s64 = ctx.r[11].s64 + 13892;
	// 82910190: 48897F69  bl 0x831a80f8
	ctx.lr = 0x82910194;
	sub_831A80F8(ctx, base);
	// 82910194: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910198: 41820014  beq 0x829101ac
	if ctx.cr[0].eq {
	pc = 0x829101AC; continue 'dispatch;
	}
	// 8291019C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829101A0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 829101A4: 4BFC9D3D  bl 0x828d9ee0
	ctx.lr = 0x829101A8;
	sub_828D9EE0(ctx, base);
	// 829101A8: 4BFFF934  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 829101AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 829101B0: 419A0034  beq cr6, 0x829101e4
	if ctx.cr[6].eq {
	pc = 0x829101E4; continue 'dispatch;
	}
	// 829101B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829101B8: 488997D1  bl 0x831a9988
	ctx.lr = 0x829101BC;
	sub_831A9988(ctx, base);
	// 829101BC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829101C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829101C4: 386BE07C  addi r3, r11, -0x1f84
	ctx.r[3].s64 = ctx.r[11].s64 + -8068;
	// 829101C8: 48897F31  bl 0x831a80f8
	ctx.lr = 0x829101CC;
	sub_831A80F8(ctx, base);
	// 829101CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829101D0: 41820014  beq 0x829101e4
	if ctx.cr[0].eq {
	pc = 0x829101E4; continue 'dispatch;
	}
	// 829101D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829101D8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 829101DC: 4BFFA0E5  bl 0x8290a2c0
	ctx.lr = 0x829101E0;
	sub_8290A2C0(ctx, base);
	// 829101E0: 4BFFF8FC  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 829101E4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 829101E8: 419A0034  beq cr6, 0x8291021c
	if ctx.cr[6].eq {
	pc = 0x8291021C; continue 'dispatch;
	}
	// 829101EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829101F0: 48899799  bl 0x831a9988
	ctx.lr = 0x829101F4;
	sub_831A9988(ctx, base);
	// 829101F4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829101F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829101FC: 386BE044  addi r3, r11, -0x1fbc
	ctx.r[3].s64 = ctx.r[11].s64 + -8124;
	// 82910200: 48897EF9  bl 0x831a80f8
	ctx.lr = 0x82910204;
	sub_831A80F8(ctx, base);
	// 82910204: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910208: 41820014  beq 0x8291021c
	if ctx.cr[0].eq {
	pc = 0x8291021C; continue 'dispatch;
	}
	// 8291020C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910210: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82910214: 4BFFA0B5  bl 0x8290a2c8
	ctx.lr = 0x82910218;
	sub_8290A2C8(ctx, base);
	// 82910218: 4BFFF8C4  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8291021C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910220: 419A0034  beq cr6, 0x82910254
	if ctx.cr[6].eq {
	pc = 0x82910254; continue 'dispatch;
	}
	// 82910224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910228: 48899761  bl 0x831a9988
	ctx.lr = 0x8291022C;
	sub_831A9988(ctx, base);
	// 8291022C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82910230: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910234: 386B31EC  addi r3, r11, 0x31ec
	ctx.r[3].s64 = ctx.r[11].s64 + 12780;
	// 82910238: 48897EC1  bl 0x831a80f8
	ctx.lr = 0x8291023C;
	sub_831A80F8(ctx, base);
	// 8291023C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910240: 41820014  beq 0x82910254
	if ctx.cr[0].eq {
	pc = 0x82910254; continue 'dispatch;
	}
	// 82910244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910248: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8291024C: 4BFFA085  bl 0x8290a2d0
	ctx.lr = 0x82910250;
	sub_8290A2D0(ctx, base);
	// 82910250: 4BFFF88C  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 82910254: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82910258: 419A0034  beq cr6, 0x8291028c
	if ctx.cr[6].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8291025C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910260: 48899729  bl 0x831a9988
	ctx.lr = 0x82910264;
	sub_831A9988(ctx, base);
	// 82910264: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82910268: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8291026C: 386BE018  addi r3, r11, -0x1fe8
	ctx.r[3].s64 = ctx.r[11].s64 + -8168;
	// 82910270: 48897E89  bl 0x831a80f8
	ctx.lr = 0x82910274;
	sub_831A80F8(ctx, base);
	// 82910274: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910278: 41820014  beq 0x8291028c
	if ctx.cr[0].eq {
	pc = 0x8291028C; continue 'dispatch;
	}
	// 8291027C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910280: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82910284: 4BE35E4D  bl 0x827460d0
	ctx.lr = 0x82910288;
	sub_827460D0(ctx, base);
	// 82910288: 4BFFF854  b 0x8290fadc
	pc = 0x8290FADC; continue 'dispatch;
	// 8291028C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82910290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910294: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82910298: 4BEDD089  bl 0x827ed320
	ctx.lr = 0x8291029C;
	sub_827ED320(ctx, base);
	// 8291029C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 829102A0: 48897F18  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829102A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829102A8 size=156
    let mut pc: u32 = 0x829102A8;
    'dispatch: loop {
        match pc {
            0x829102A8 => {
    //   block [0x829102A8..0x82910344)
	// 829102A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829102AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829102B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829102B4: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 829102B8: 488987BD  bl 0x831a8a74
	ctx.lr = 0x829102BC;
	sub_831A8A40(ctx, base);
	// 829102BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829102C0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829102C4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 829102C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829102CC: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 829102D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 829102D4: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 829102D8: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 829102DC: FF802090  fmr f28, f4
	ctx.f[28].f64 = ctx.f[4].f64;
	// 829102E0: 38A00389  li r5, 0x389
	ctx.r[5].s64 = 905;
	// 829102E4: FF602890  fmr f27, f5
	ctx.f[27].f64 = ctx.f[5].f64;
	// 829102E8: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 829102EC: 4B9B00ED  bl 0x822c03d8
	ctx.lr = 0x829102F0;
	sub_822C03D8(ctx, base);
	// 829102F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829102F4: 41820028  beq 0x8291031c
	if ctx.cr[0].eq {
	pc = 0x8291031C; continue 'dispatch;
	}
	// 829102F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829102FC: FCA0D890  fmr f5, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[5].f64 = ctx.f[27].f64;
	// 82910300: FC80E090  fmr f4, f28
	ctx.f[4].f64 = ctx.f[28].f64;
	// 82910304: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82910308: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8291030C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82910310: 4800B821  bl 0x8291bb30
	ctx.lr = 0x82910314;
	sub_8291BB30(ctx, base);
	// 82910314: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910318: 48000008  b 0x82910320
	pc = 0x82910320; continue 'dispatch;
	// 8291031C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82910320: 387F0250  addi r3, r31, 0x250
	ctx.r[3].s64 = ctx.r[31].s64 + 592;
	// 82910324: 4BF97515  bl 0x828a7838
	ctx.lr = 0x82910328;
	sub_828A7838(ctx, base);
	// 82910328: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8291032C: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82910330: 48898791  bl 0x831a8ac0
	ctx.lr = 0x82910334;
	sub_831A8A8C(ctx, base);
	// 82910334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82910338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291033C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82910340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82910348 size=72
    let mut pc: u32 = 0x82910348;
    'dispatch: loop {
        match pc {
            0x82910348 => {
    //   block [0x82910348..0x82910390)
	// 82910348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82910350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82910354: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82910358: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291035C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910360: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 82910364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82910368: 4E800421  bctrl
	ctx.lr = 0x8291036C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8291036C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82910370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910374: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82910378: 4BFFE1D9  bl 0x8290e550
	ctx.lr = 0x8291037C;
	sub_8290E550(ctx, base);
	// 8291037C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82910380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82910384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82910388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8291038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82910390 size=112
    let mut pc: u32 = 0x82910390;
    'dispatch: loop {
        match pc {
            0x82910390 => {
    //   block [0x82910390..0x82910400)
	// 82910390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82910394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82910398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291039C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829103A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829103A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829103A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829103AC: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 829103B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829103B4: 4E800421  bctrl
	ctx.lr = 0x829103B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829103B8: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 829103BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829103C0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 829103C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 829103C8: 48006C69  bl 0x82917030
	ctx.lr = 0x829103CC;
	sub_82917030(ctx, base);
	// 829103CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829103D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829103D4: 4182000C  beq 0x829103e0
	if ctx.cr[0].eq {
	pc = 0x829103E0; continue 'dispatch;
	}
	// 829103D8: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 829103DC: 48000008  b 0x829103e4
	pc = 0x829103E4; continue 'dispatch;
	// 829103E0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 829103E4: 4BFFE585  bl 0x8290e968
	ctx.lr = 0x829103E8;
	sub_8290E968(ctx, base);
	// 829103E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829103EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829103F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829103F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829103F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829103FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82910400 size=100
    let mut pc: u32 = 0x82910400;
    'dispatch: loop {
        match pc {
            0x82910400 => {
    //   block [0x82910400..0x82910464)
	// 82910400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82910404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82910408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8291040C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82910410: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82910414: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910418: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8291041C: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 82910420: 38A00B24  li r5, 0xb24
	ctx.r[5].s64 = 2852;
	// 82910424: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82910428: 4B9AFFB1  bl 0x822c03d8
	ctx.lr = 0x8291042C;
	sub_822C03D8(ctx, base);
	// 8291042C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82910430: 41820014  beq 0x82910444
	if ctx.cr[0].eq {
	pc = 0x82910444; continue 'dispatch;
	}
	// 82910434: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82910438: 48235FF9  bl 0x82b46430
	ctx.lr = 0x8291043C;
	sub_82B46430(ctx, base);
	// 8291043C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910440: 48000008  b 0x82910448
	pc = 0x82910448; continue 'dispatch;
	// 82910444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82910448: 387F0310  addi r3, r31, 0x310
	ctx.r[3].s64 = ctx.r[31].s64 + 784;
	// 8291044C: 4BF44F85  bl 0x828553d0
	ctx.lr = 0x82910450;
	sub_828553D0(ctx, base);
	// 82910450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82910454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82910458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291045C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82910460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82910468 size=184
    let mut pc: u32 = 0x82910468;
    'dispatch: loop {
        match pc {
            0x82910468 => {
    //   block [0x82910468..0x82910520)
	// 82910468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291046C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82910470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82910474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82910478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291047C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82910480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82910484: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82910488: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8291048C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82910490: 4B9B04A9  bl 0x822c0938
	ctx.lr = 0x82910494;
	sub_822C0938(ctx, base);
	// 82910494: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82910498: 41820028  beq 0x829104c0
	if ctx.cr[0].eq {
	pc = 0x829104C0; continue 'dispatch;
	}
	// 8291049C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829104A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 829104A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 829104A8: 392B55DC  addi r9, r11, 0x55dc
	ctx.r[9].s64 = ctx.r[11].s64 + 21980;
	// 829104AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 829104B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 829104B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 829104B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 829104BC: 48000008  b 0x829104c4
	pc = 0x829104C4; continue 'dispatch;
	// 829104C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829104C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829104C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829104CC: 409A0038  bne cr6, 0x82910504
	if !ctx.cr[6].eq {
	pc = 0x82910504; continue 'dispatch;
	}
	// 829104D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 829104D4: 419A0010  beq cr6, 0x829104e4
	if ctx.cr[6].eq {
	pc = 0x829104E4; continue 'dispatch;
	}
	// 829104D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 829104DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829104E0: 4817B0E1  bl 0x82a8b5c0
	ctx.lr = 0x829104E4;
	sub_82A8B5C0(ctx, base);
	// 829104E4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829104E8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 829104EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829104F0: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 829104F4: 816BD9BC  lwz r11, -0x2644(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9796 as u32) ) } as u64;
	// 829104F8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 829104FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82910500: 4B9AFB01  bl 0x822c0000
	ctx.lr = 0x82910504;
	sub_822C0000(ctx, base);
	// 82910504: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82910508: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8291050C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82910510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82910514: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82910518: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8291051C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82910520 size=12
    let mut pc: u32 = 0x82910520;
    'dispatch: loop {
        match pc {
            0x82910520 => {
    //   block [0x82910520..0x8291052C)
	// 82910520: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82910524: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910528: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8291052C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8291052C size=8
    let mut pc: u32 = 0x8291052C;
    'dispatch: loop {
        match pc {
            0x8291052C => {
    //   block [0x8291052C..0x82910534)
	// 8291052C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82910530: 4817B090  b 0x82a8b5c0
	sub_82A8B5C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82910534 size=4
    let mut pc: u32 = 0x82910534;
    'dispatch: loop {
        match pc {
            0x82910534 => {
    //   block [0x82910534..0x82910538)
	// 82910534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82910538 size=112
    let mut pc: u32 = 0x82910538;
    'dispatch: loop {
        match pc {
            0x82910538 => {
    //   block [0x82910538..0x829105A8)
	// 82910538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291053C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82910540: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82910544: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82910548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291054C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82910550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910554: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82910558: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8291055C: 4BFFFF0D  bl 0x82910468
	ctx.lr = 0x82910560;
	sub_82910468(ctx, base);
	// 82910560: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82910564: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82910568: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8291056C: 4B9AFA95  bl 0x822c0000
	ctx.lr = 0x82910570;
	sub_822C0000(ctx, base);
	// 82910570: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82910574: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82910578: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8291057C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82910580: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910584: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82910588: 419A0008  beq cr6, 0x82910590
	if ctx.cr[6].eq {
	pc = 0x82910590; continue 'dispatch;
	}
	// 8291058C: 4B9B0305  bl 0x822c0890
	ctx.lr = 0x82910590;
	sub_822C0890(ctx, base);
	// 82910590: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82910594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82910598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291059C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829105A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829105A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829105A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829105A8 size=88
    let mut pc: u32 = 0x829105A8;
    'dispatch: loop {
        match pc {
            0x829105A8 => {
    //   block [0x829105A8..0x82910600)
	// 829105A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829105AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829105B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829105B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829105B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829105BC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 829105C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829105C4: 419A0018  beq cr6, 0x829105dc
	if ctx.cr[6].eq {
	pc = 0x829105DC; continue 'dispatch;
	}
	// 829105C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 829105CC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 829105D0: 4BEC1D79  bl 0x827d2348
	ctx.lr = 0x829105D4;
	sub_827D2348(ctx, base);
	// 829105D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 829105D8: 4B9AFC91  bl 0x822c0268
	ctx.lr = 0x829105DC;
	sub_822C0268(ctx, base);
	// 829105DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829105E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 829105E4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 829105E8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 829105EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829105F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829105F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829105F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829105FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82910600 size=324
    let mut pc: u32 = 0x82910600;
    'dispatch: loop {
        match pc {
            0x82910600 => {
    //   block [0x82910600..0x82910744)
	// 82910600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82910604: 48897B69  bl 0x831a816c
	ctx.lr = 0x82910608;
	sub_831A8130(ctx, base);
	// 82910608: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291060C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910610: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910614: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82910618: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8291061C: 4851A4CD  bl 0x82e2aae8
	ctx.lr = 0x82910620;
	sub_82E2AAE8(ctx, base);
	// 82910620: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82910624: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82910628: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8291062C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82910630: 4851E841  bl 0x82e2ee70
	ctx.lr = 0x82910634;
	sub_82E2EE70(ctx, base);
	// 82910634: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82910638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8291063C: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 82910640: 419A00DC  beq cr6, 0x8291071c
	if ctx.cr[6].eq {
	pc = 0x8291071C; continue 'dispatch;
	}
	// 82910644: 556B0734  rlwinm r11, r11, 0, 0x1c, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82910648: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8291064C: 917F034C  stw r11, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[11].u32 ) };
	// 82910650: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82910654: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 82910658: 38A001DC  li r5, 0x1dc
	ctx.r[5].s64 = 476;
	// 8291065C: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82910660: 484E1D89  bl 0x82df23e8
	ctx.lr = 0x82910664;
	sub_82DF23E8(ctx, base);
	// 82910664: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82910668: 41820014  beq 0x8291067c
	if ctx.cr[0].eq {
	pc = 0x8291067C; continue 'dispatch;
	}
	// 8291066C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82910670: 48506221  bl 0x82e16890
	ctx.lr = 0x82910674;
	sub_82E16890(ctx, base);
	// 82910674: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910678: 48000008  b 0x82910680
	pc = 0x82910680; continue 'dispatch;
	// 8291067C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82910680: 3BDF00EC  addi r30, r31, 0xec
	ctx.r[30].s64 = ctx.r[31].s64 + 236;
	// 82910684: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82910688: 4BA4CCE9  bl 0x8235d370
	ctx.lr = 0x8291068C;
	sub_8235D370(ctx, base);
	// 8291068C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82910690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910694: 4BEDBAC5  bl 0x827ec158
	ctx.lr = 0x82910698;
	sub_827EC158(ctx, base);
	// 82910698: 809F00EC  lwz r4, 0xec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8291069C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 829106A0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 829106A4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 829106A8: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 829106AC: 80640084  lwz r3, 0x84(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 829106B0: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 829106B4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 829106B8: 13E85C07  vcmpneb. (lvlx128) v31, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829106BC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 829106C0: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 829106C4: 13C93C07  vcmpneb. (lvlx128) v30, v9, v7
	tmp.u32 = ctx.r[9].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829106C8: 13AA3407  vcmpneb. (lvlx128) v29, v10, v6
	tmp.u32 = ctx.r[10].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829106CC: 13802C07  vcmpneb. (lvlx128) v28, v0, v5
	tmp.u32 = ctx.r[5].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82910748 size=20
    let mut pc: u32 = 0x82910748;
    'dispatch: loop {
        match pc {
            0x82910748 => {
    //   block [0x82910748..0x8291075C)
	// 82910748: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291074C: 8163034C  lwz r11, 0x34c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(844 as u32) ) } as u64;
	// 82910750: 4182000C  beq 0x8291075c
	if ctx.cr[0].eq {
		sub_8291075C(ctx, base);
		return;
	}
	// 82910754: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82910758: 48000008  b 0x82910760
	sub_8291075C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8291075C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8291075C size=12
    let mut pc: u32 = 0x8291075C;
    'dispatch: loop {
        match pc {
            0x8291075C => {
    //   block [0x8291075C..0x82910768)
	// 8291075C: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82910760: 9163034C  stw r11, 0x34c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(844 as u32), ctx.r[11].u32 ) };
	// 82910764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82910768 size=12
    let mut pc: u32 = 0x82910768;
    'dispatch: loop {
        match pc {
            0x82910768 => {
    //   block [0x82910768..0x82910774)
	// 82910768: 8163034C  lwz r11, 0x34c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(844 as u32) ) } as u64;
	// 8291076C: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82910770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82910778 size=308
    let mut pc: u32 = 0x82910778;
    'dispatch: loop {
        match pc {
            0x82910778 => {
    //   block [0x82910778..0x829108AC)
	// 82910778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291077C: 488979F1  bl 0x831a816c
	ctx.lr = 0x82910780;
	sub_831A8130(ctx, base);
	// 82910780: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82910784: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82910788: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829108B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829108B0 size=16
    let mut pc: u32 = 0x829108B0;
    'dispatch: loop {
        match pc {
            0x829108B0 => {
    //   block [0x829108B0..0x829108C0)
	// 829108B0: 8163034C  lwz r11, 0x34c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(844 as u32) ) } as u64;
	// 829108B4: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 829108B8: 9163034C  stw r11, 0x34c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(844 as u32), ctx.r[11].u32 ) };
	// 829108BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829108C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829108C0 size=12
    let mut pc: u32 = 0x829108C0;
    'dispatch: loop {
        match pc {
            0x829108C0 => {
    //   block [0x829108C0..0x829108CC)
	// 829108C0: 8163034C  lwz r11, 0x34c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(844 as u32) ) } as u64;
	// 829108C4: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 829108C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829108D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829108D0 size=72
    let mut pc: u32 = 0x829108D0;
    'dispatch: loop {
        match pc {
            0x829108D0 => {
    //   block [0x829108D0..0x82910918)
	// 829108D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829108D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829108D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829108DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829108E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829108E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 829108E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829108EC: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 829108F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829108F4: 4E800421  bctrl
	ctx.lr = 0x829108F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829108F8: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 829108FC: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82910900: 917F034C  stw r11, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[11].u32 ) };
	// 82910904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82910908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291090C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82910910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82910914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82910918 size=396
    let mut pc: u32 = 0x82910918;
    'dispatch: loop {
        match pc {
            0x82910918 => {
    //   block [0x82910918..0x82910AA4)
	// 82910918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291091C: 4889784D  bl 0x831a8168
	ctx.lr = 0x82910920;
	sub_831A8130(ctx, base);
	// 82910920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82910924: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82910928: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8291092C: 3BBE034C  addi r29, r30, 0x34c
	ctx.r[29].s64 = ctx.r[30].s64 + 844;
	// 82910930: 817E034C  lwz r11, 0x34c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(844 as u32) ) } as u64;
	// 82910934: 556BEFFF  rlwinm. r11, r11, 0x1d, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910938: 41820164  beq 0x82910a9c
	if ctx.cr[0].eq {
	pc = 0x82910A9C; continue 'dispatch;
	}
	// 8291093C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82910940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82910944: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 82910948: 484E30C1  bl 0x82df3a08
	ctx.lr = 0x8291094C;
	sub_82DF3A08(ctx, base);
	// 8291094C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82910950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82910954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910958: 484E29B1  bl 0x82df3308
	ctx.lr = 0x8291095C;
	sub_82DF3308(ctx, base);
	// 8291095C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82910960: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82910964: 484E2AC5  bl 0x82df3428
	ctx.lr = 0x82910968;
	sub_82DF3428(ctx, base);
	// 82910968: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291096C: 418200DC  beq 0x82910a48
	if ctx.cr[0].eq {
	pc = 0x82910A48; continue 'dispatch;
	}
	// 82910970: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82910974: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910978: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8291097C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82910980: 4E800421  bctrl
	ctx.lr = 0x82910984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82910984: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82910988: 41820114  beq 0x82910a9c
	if ctx.cr[0].eq {
	pc = 0x82910A9C; continue 'dispatch;
	}
	// 8291098C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910990: 4BED9919  bl 0x827ea2a8
	ctx.lr = 0x82910994;
	sub_827EA2A8(ctx, base);
	// 82910994: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910998: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8291099C: 484E3265  bl 0x82df3c00
	ctx.lr = 0x829109A0;
	sub_82DF3C00(ctx, base);
	// 829109A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 829109A4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829109A8: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 829109AC: 484E305D  bl 0x82df3a08
	ctx.lr = 0x829109B0;
	sub_82DF3A08(ctx, base);
	// 829109B0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 829109B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829109B8: 484E28E9  bl 0x82df32a0
	ctx.lr = 0x829109BC;
	sub_82DF32A0(ctx, base);
	// 829109BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 829109C0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829109C4: 484E2A65  bl 0x82df3428
	ctx.lr = 0x829109C8;
	sub_82DF3428(ctx, base);
	// 829109C8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829109CC: 41820070  beq 0x82910a3c
	if ctx.cr[0].eq {
	pc = 0x82910A3C; continue 'dispatch;
	}
	// 829109D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829109D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829109D8: 4BED9BC1  bl 0x827ea598
	ctx.lr = 0x829109DC;
	sub_827EA598(ctx, base);
	// 829109DC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 829109E0: 4182005C  beq 0x82910a3c
	if ctx.cr[0].eq {
	pc = 0x82910A3C; continue 'dispatch;
	}
	// 829109E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829109E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829109EC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 829109F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829109F4: 4E800421  bctrl
	ctx.lr = 0x829109F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829109F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829109FC: 41820040  beq 0x82910a3c
	if ctx.cr[0].eq {
	pc = 0x82910A3C; continue 'dispatch;
	}
	// 82910A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910A04: 482A6035  bl 0x82bb6a38
	ctx.lr = 0x82910A08;
	sub_82BB6A38(ctx, base);
	// 82910A08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82910A0C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82910A10: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82910A14: 40990028  ble cr6, 0x82910a3c
	if !ctx.cr[6].gt {
	pc = 0x82910A3C; continue 'dispatch;
	}
	// 82910A18: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910A1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82910A20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82910A24: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82910A28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82910A2C: 4E800421  bctrl
	ctx.lr = 0x82910A30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82910A30: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910A34: 556B0776  rlwinm r11, r11, 0, 0x1d, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82910A38: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82910A3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82910A40: 484E29E9  bl 0x82df3428
	ctx.lr = 0x82910A44;
	sub_82DF3428(ctx, base);
	// 82910A44: 48000058  b 0x82910a9c
	pc = 0x82910A9C; continue 'dispatch;
	// 82910A48: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82910A4C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82910A50: 808BE264  lwz r4, -0x1d9c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7580 as u32) ) } as u64;
	// 82910A54: 484E2FB5  bl 0x82df3a08
	ctx.lr = 0x82910A58;
	sub_82DF3A08(ctx, base);
	// 82910A58: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82910A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910A60: 484E28A9  bl 0x82df3308
	ctx.lr = 0x82910A64;
	sub_82DF3308(ctx, base);
	// 82910A64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910A68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82910A6C: 484E29BD  bl 0x82df3428
	ctx.lr = 0x82910A70;
	sub_82DF3428(ctx, base);
	// 82910A70: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910A74: 41820028  beq 0x82910a9c
	if ctx.cr[0].eq {
	pc = 0x82910A9C; continue 'dispatch;
	}
	// 82910A78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910A7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82910A80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82910A84: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82910A88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82910A8C: 4E800421  bctrl
	ctx.lr = 0x82910A90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82910A90: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910A94: 556B0776  rlwinm r11, r11, 0, 0x1d, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82910A98: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82910A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82910AA0: 48897718  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82910AA8 size=108
    let mut pc: u32 = 0x82910AA8;
    'dispatch: loop {
        match pc {
            0x82910AA8 => {
    //   block [0x82910AA8..0x82910B14)
	// 82910AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82910AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82910AB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82910AB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82910AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910ABC: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 82910AC0: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910AC4: 4082003C  bne 0x82910b00
	if !ctx.cr[0].eq {
	pc = 0x82910B00; continue 'dispatch;
	}
	// 82910AC8: 817F02E0  lwz r11, 0x2e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(736 as u32) ) } as u64;
	// 82910ACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82910AD0: 419A0018  beq cr6, 0x82910ae8
	if ctx.cr[6].eq {
	pc = 0x82910AE8; continue 'dispatch;
	}
	// 82910AD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910AD8: 816B0144  lwz r11, 0x144(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(324 as u32) ) } as u64;
	// 82910ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82910AE0: 4E800421  bctrl
	ctx.lr = 0x82910AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82910AE4: 48000010  b 0x82910af4
	pc = 0x82910AF4; continue 'dispatch;
	// 82910AE8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82910AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910AF0: 4BBFEF61  bl 0x8250fa50
	ctx.lr = 0x82910AF4;
	sub_8250FA50(ctx, base);
	// 82910AF4: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 82910AF8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82910AFC: 917F034C  stw r11, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[11].u32 ) };
	// 82910B00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82910B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82910B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82910B0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82910B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82910B18 size=360
    let mut pc: u32 = 0x82910B18;
    'dispatch: loop {
        match pc {
            0x82910B18 => {
    //   block [0x82910B18..0x82910C80)
	// 82910B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82910B1C: 4889764D  bl 0x831a8168
	ctx.lr = 0x82910B20;
	sub_831A8130(ctx, base);
	// 82910B20: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82910B24: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82910B28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910B2C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82910B30: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82910B34: 4BEDC85D  bl 0x827ed390
	ctx.lr = 0x82910B38;
	sub_827ED390(ctx, base);
	// 82910B38: 387F0208  addi r3, r31, 0x208
	ctx.r[3].s64 = ctx.r[31].s64 + 520;
	// 82910B3C: 48036315  bl 0x82946e50
	ctx.lr = 0x82910B40;
	sub_82946E50(ctx, base);
	// 82910B40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82910B44: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82910B48: 93DF0244  stw r30, 0x244(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(580 as u32), ctx.r[30].u32 ) };
	// 82910B4C: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 82910B50: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82910B54: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82910B58: C3EB08A8  lfs f31, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82910B5C: 39695A94  addi r11, r9, 0x5a94
	ctx.r[11].s64 = ctx.r[9].s64 + 23188;
	// 82910B60: 394A58D4  addi r10, r10, 0x58d4
	ctx.r[10].s64 = ctx.r[10].s64 + 22740;
	// 82910B64: D3FF0240  stfs f31, 0x240(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(576 as u32), tmp.u32 ) };
	// 82910B68: 39285A78  addi r9, r8, 0x5a78
	ctx.r[9].s64 = ctx.r[8].s64 + 23160;
	// 82910B6C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82910B70: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82910B74: 387F0270  addi r3, r31, 0x270
	ctx.r[3].s64 = ctx.r[31].s64 + 624;
	// 82910B78: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82910B7C: 3BBF0260  addi r29, r31, 0x260
	ctx.r[29].s64 = ctx.r[31].s64 + 608;
	// 82910B80: 93DF0248  stw r30, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[30].u32 ) };
	// 82910B84: 93DF024C  stw r30, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[30].u32 ) };
	// 82910B88: 93DF0250  stw r30, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[30].u32 ) };
	// 82910B8C: 93DF0254  stw r30, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[30].u32 ) };
	// 82910B90: 93DF0258  stw r30, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[30].u32 ) };
	// 82910B94: 93DF025C  stw r30, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[30].u32 ) };
	// 82910B98: 93DF0260  stw r30, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[30].u32 ) };
	// 82910B9C: 93DF0264  stw r30, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[30].u32 ) };
	// 82910BA0: 93DF0268  stw r30, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u32 ) };
	// 82910BA4: 93DF026C  stw r30, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[30].u32 ) };
	// 82910BA8: 4BAA3449  bl 0x823b3ff0
	ctx.lr = 0x82910BAC;
	sub_823B3FF0(ctx, base);
	// 82910BAC: 93DF02D0  stw r30, 0x2d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(720 as u32), ctx.r[30].u32 ) };
	// 82910BB0: 93DF02D4  stw r30, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[30].u32 ) };
	// 82910BB4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82910BB8: 93DF02E0  stw r30, 0x2e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), ctx.r[30].u32 ) };
	// 82910BBC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82910BC0: 93DF02EC  stw r30, 0x2ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(748 as u32), ctx.r[30].u32 ) };
	// 82910BC4: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82910BC8: 93DF02F4  stw r30, 0x2f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(756 as u32), ctx.r[30].u32 ) };
	// 82910BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82910BD0: 93DF02F8  stw r30, 0x2f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[30].u32 ) };
	// 82910BD4: 38895758  addi r4, r9, 0x5758
	ctx.r[4].s64 = ctx.r[9].s64 + 22360;
	// 82910BD8: 93DF02FC  stw r30, 0x2fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(764 as u32), ctx.r[30].u32 ) };
	// 82910BDC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82910BE0: 93DF0300  stw r30, 0x300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(768 as u32), ctx.r[30].u32 ) };
	// 82910BE4: 38A000AF  li r5, 0xaf
	ctx.r[5].s64 = 175;
	// 82910BE8: 93DF0304  stw r30, 0x304(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(772 as u32), ctx.r[30].u32 ) };
	// 82910BEC: 38600190  li r3, 0x190
	ctx.r[3].s64 = 400;
	// 82910BF0: 93DF0308  stw r30, 0x308(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[30].u32 ) };
	// 82910BF4: 93DF030C  stw r30, 0x30c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(780 as u32), ctx.r[30].u32 ) };
	// 82910BF8: 93DF0310  stw r30, 0x310(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(784 as u32), ctx.r[30].u32 ) };
	// 82910BFC: 93DF0314  stw r30, 0x314(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(788 as u32), ctx.r[30].u32 ) };
	// 82910C00: 93DF0318  stw r30, 0x318(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(792 as u32), ctx.r[30].u32 ) };
	// 82910C04: 93DF031C  stw r30, 0x31c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(796 as u32), ctx.r[30].u32 ) };
	// 82910C08: 93DF0320  stw r30, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[30].u32 ) };
	// 82910C0C: 93DF0324  stw r30, 0x324(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(804 as u32), ctx.r[30].u32 ) };
	// 82910C10: D3FF032C  stfs f31, 0x32c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(812 as u32), tmp.u32 ) };
	// 82910C14: D3FF0330  stfs f31, 0x330(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(816 as u32), tmp.u32 ) };
	// 82910C18: 93DF0328  stw r30, 0x328(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(808 as u32), ctx.r[30].u32 ) };
	// 82910C1C: C1BC0004  lfs f13, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82910C20: 9BDF0338  stb r30, 0x338(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(824 as u32), ctx.r[30].u8 ) };
	// 82910C24: D01F033C  stfs f0, 0x33c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), tmp.u32 ) };
	// 82910C28: D1BF0334  stfs f13, 0x334(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(820 as u32), tmp.u32 ) };
	// 82910C2C: 93DF0340  stw r30, 0x340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(832 as u32), ctx.r[30].u32 ) };
	// 82910C30: 93DF0344  stw r30, 0x344(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(836 as u32), ctx.r[30].u32 ) };
	// 82910C34: C00A9534  lfs f0, -0x6acc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82910C38: 9BDF0348  stb r30, 0x348(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(840 as u32), ctx.r[30].u8 ) };
	// 82910C3C: 93DF034C  stw r30, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[30].u32 ) };
	// 82910C40: D01F0350  stfs f0, 0x350(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(848 as u32), tmp.u32 ) };
	// 82910C44: 9BDF0354  stb r30, 0x354(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(852 as u32), ctx.r[30].u8 ) };
	// 82910C48: 4B9AF791  bl 0x822c03d8
	ctx.lr = 0x82910C4C;
	sub_822C03D8(ctx, base);
	// 82910C4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82910C50: 41820014  beq 0x82910c64
	if ctx.cr[0].eq {
	pc = 0x82910C64; continue 'dispatch;
	}
	// 82910C54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910C58: 48002D69  bl 0x829139c0
	ctx.lr = 0x82910C5C;
	sub_829139C0(ctx, base);
	// 82910C5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910C60: 48000008  b 0x82910c68
	pc = 0x82910C68; continue 'dispatch;
	// 82910C64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82910C68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82910C6C: 4BFFCB95  bl 0x8290d800
	ctx.lr = 0x82910C70;
	sub_8290D800(ctx, base);
	// 82910C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82910C78: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82910C7C: 4889753C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82910C80 size=8
    let mut pc: u32 = 0x82910C80;
    'dispatch: loop {
        match pc {
            0x82910C80 => {
    //   block [0x82910C80..0x82910C88)
	// 82910C80: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82910C84: 48000C4C  b 0x829118d0
	sub_829118D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82910C88 size=8
    let mut pc: u32 = 0x82910C88;
    'dispatch: loop {
        match pc {
            0x82910C88 => {
    //   block [0x82910C88..0x82910C90)
	// 82910C88: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82910C8C: 48000C44  b 0x829118d0
	sub_829118D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82910C90 size=256
    let mut pc: u32 = 0x82910C90;
    'dispatch: loop {
        match pc {
            0x82910C90 => {
    //   block [0x82910C90..0x82910D90)
	// 82910C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82910C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82910C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82910C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82910CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82910CA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910CA8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82910CAC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82910CB0: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82910CB4: 396B58D4  addi r11, r11, 0x58d4
	ctx.r[11].s64 = ctx.r[11].s64 + 22740;
	// 82910CB8: 394A5A94  addi r10, r10, 0x5a94
	ctx.r[10].s64 = ctx.r[10].s64 + 23188;
	// 82910CBC: 39295A78  addi r9, r9, 0x5a78
	ctx.r[9].s64 = ctx.r[9].s64 + 23160;
	// 82910CC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82910CC4: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82910CC8: 3BDF0208  addi r30, r31, 0x208
	ctx.r[30].s64 = ctx.r[31].s64 + 520;
	// 82910CCC: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 82910CD0: 807F0344  lwz r3, 0x344(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(836 as u32) ) } as u64;
	// 82910CD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910CD8: 419A0008  beq cr6, 0x82910ce0
	if ctx.cr[6].eq {
	pc = 0x82910CE0; continue 'dispatch;
	}
	// 82910CDC: 4B9AFBB5  bl 0x822c0890
	ctx.lr = 0x82910CE0;
	sub_822C0890(ctx, base);
	// 82910CE0: 807F0324  lwz r3, 0x324(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(804 as u32) ) } as u64;
	// 82910CE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910CE8: 419A0008  beq cr6, 0x82910cf0
	if ctx.cr[6].eq {
	pc = 0x82910CF0; continue 'dispatch;
	}
	// 82910CEC: 4B9AFBA5  bl 0x822c0890
	ctx.lr = 0x82910CF0;
	sub_822C0890(ctx, base);
	// 82910CF0: 807F0314  lwz r3, 0x314(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(788 as u32) ) } as u64;
	// 82910CF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910CF8: 419A0008  beq cr6, 0x82910d00
	if ctx.cr[6].eq {
	pc = 0x82910D00; continue 'dispatch;
	}
	// 82910CFC: 4B9AFB95  bl 0x822c0890
	ctx.lr = 0x82910D00;
	sub_822C0890(ctx, base);
	// 82910D00: 807F030C  lwz r3, 0x30c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(780 as u32) ) } as u64;
	// 82910D04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910D08: 419A0008  beq cr6, 0x82910d10
	if ctx.cr[6].eq {
	pc = 0x82910D10; continue 'dispatch;
	}
	// 82910D0C: 4B9AFB85  bl 0x822c0890
	ctx.lr = 0x82910D10;
	sub_822C0890(ctx, base);
	// 82910D10: 807F0304  lwz r3, 0x304(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(772 as u32) ) } as u64;
	// 82910D14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910D18: 419A0008  beq cr6, 0x82910d20
	if ctx.cr[6].eq {
	pc = 0x82910D20; continue 'dispatch;
	}
	// 82910D1C: 4B9AFB75  bl 0x822c0890
	ctx.lr = 0x82910D20;
	sub_822C0890(ctx, base);
	// 82910D20: 387F02F0  addi r3, r31, 0x2f0
	ctx.r[3].s64 = ctx.r[31].s64 + 752;
	// 82910D24: 4BFFF885  bl 0x829105a8
	ctx.lr = 0x82910D28;
	sub_829105A8(ctx, base);
	// 82910D28: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 82910D2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910D30: 419A0008  beq cr6, 0x82910d38
	if ctx.cr[6].eq {
	pc = 0x82910D38; continue 'dispatch;
	}
	// 82910D34: 4B9AFB5D  bl 0x822c0890
	ctx.lr = 0x82910D38;
	sub_822C0890(ctx, base);
	// 82910D38: 807F0264  lwz r3, 0x264(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(612 as u32) ) } as u64;
	// 82910D3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910D40: 419A0008  beq cr6, 0x82910d48
	if ctx.cr[6].eq {
	pc = 0x82910D48; continue 'dispatch;
	}
	// 82910D44: 4B9AFB4D  bl 0x822c0890
	ctx.lr = 0x82910D48;
	sub_822C0890(ctx, base);
	// 82910D48: 807F025C  lwz r3, 0x25c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(604 as u32) ) } as u64;
	// 82910D4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910D50: 419A0008  beq cr6, 0x82910d58
	if ctx.cr[6].eq {
	pc = 0x82910D58; continue 'dispatch;
	}
	// 82910D54: 4B9AFB3D  bl 0x822c0890
	ctx.lr = 0x82910D58;
	sub_822C0890(ctx, base);
	// 82910D58: 807F0254  lwz r3, 0x254(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(596 as u32) ) } as u64;
	// 82910D5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910D60: 419A0008  beq cr6, 0x82910d68
	if ctx.cr[6].eq {
	pc = 0x82910D68; continue 'dispatch;
	}
	// 82910D64: 4B9AFB2D  bl 0x822c0890
	ctx.lr = 0x82910D68;
	sub_822C0890(ctx, base);
	// 82910D68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82910D6C: 480360E5  bl 0x82946e50
	ctx.lr = 0x82910D70;
	sub_82946E50(ctx, base);
	// 82910D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910D74: 4BEDBD05  bl 0x827eca78
	ctx.lr = 0x82910D78;
	sub_827ECA78(ctx, base);
	// 82910D78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82910D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82910D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82910D84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82910D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82910D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82910D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82910D90 size=1004
    let mut pc: u32 = 0x82910D90;
    'dispatch: loop {
        match pc {
            0x82910D90 => {
    //   block [0x82910D90..0x8291117C)
	// 82910D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82910D94: 488973CD  bl 0x831a8160
	ctx.lr = 0x82910D98;
	sub_831A8130(ctx, base);
	// 82910D98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82910D9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82910DA0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82910DA4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82910DA8: 4BEDBE11  bl 0x827ecbb8
	ctx.lr = 0x82910DAC;
	sub_827ECBB8(ctx, base);
	// 82910DAC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82910DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910DB4: 4BBFE5BD  bl 0x8250f370
	ctx.lr = 0x82910DB8;
	sub_8250F370(ctx, base);
	// 82910DB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82910DBC: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82910DC0: 409A0008  bne cr6, 0x82910dc8
	if !ctx.cr[6].eq {
	pc = 0x82910DC8; continue 'dispatch;
	}
	// 82910DC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82910DC8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82910DCC: 4BBF79D5  bl 0x825087a0
	ctx.lr = 0x82910DD0;
	sub_825087A0(ctx, base);
	// 82910DD0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82910DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82910DD8: 3B4B5758  addi r26, r11, 0x5758
	ctx.r[26].s64 = ctx.r[11].s64 + 22360;
	// 82910DDC: 38A000C1  li r5, 0xc1
	ctx.r[5].s64 = 193;
	// 82910DE0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82910DE4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82910DE8: 4B9AF5F1  bl 0x822c03d8
	ctx.lr = 0x82910DEC;
	sub_822C03D8(ctx, base);
	// 82910DEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82910DF0: 41820014  beq 0x82910e04
	if ctx.cr[0].eq {
	pc = 0x82910E04; continue 'dispatch;
	}
	// 82910DF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910DF8: 4802F8D9  bl 0x829406d0
	ctx.lr = 0x82910DFC;
	sub_829406D0(ctx, base);
	// 82910DFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82910E00: 48000008  b 0x82910e08
	pc = 0x82910E08; continue 'dispatch;
	// 82910E04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82910E08: 387F0258  addi r3, r31, 0x258
	ctx.r[3].s64 = ctx.r[31].s64 + 600;
	// 82910E0C: 4BFFCA65  bl 0x8290d870
	ctx.lr = 0x82910E10;
	sub_8290D870(ctx, base);
	// 82910E10: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82910E14: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82910E18: 4BBF7869  bl 0x82508680
	ctx.lr = 0x82910E1C;
	sub_82508680(ctx, base);
	// 82910E1C: 907F02D0  stw r3, 0x2d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(720 as u32), ctx.r[3].u32 ) };
	// 82910E20: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82910E24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82910E28: 4BBF7859  bl 0x82508680
	ctx.lr = 0x82910E2C;
	sub_82508680(ctx, base);
	// 82910E2C: 907F02D4  stw r3, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[3].u32 ) };
	// 82910E30: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82910E34: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82910E38: 4BBF7849  bl 0x82508680
	ctx.lr = 0x82910E3C;
	sub_82508680(ctx, base);
	// 82910E3C: 907F02D8  stw r3, 0x2d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(728 as u32), ctx.r[3].u32 ) };
	// 82910E40: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82910E44: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82910E48: 4BBF7839  bl 0x82508680
	ctx.lr = 0x82910E4C;
	sub_82508680(ctx, base);
	// 82910E4C: 907F02DC  stw r3, 0x2dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(732 as u32), ctx.r[3].u32 ) };
	// 82910E50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910E54: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82910E58: 4BBFE671  bl 0x8250f4c8
	ctx.lr = 0x82910E5C;
	sub_8250F4C8(ctx, base);
	// 82910E5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910E60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82910E64: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82910E68: 409A0008  bne cr6, 0x82910e70
	if !ctx.cr[6].eq {
	pc = 0x82910E70; continue 'dispatch;
	}
	// 82910E6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82910E70: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82910E74: 4BBF780D  bl 0x82508680
	ctx.lr = 0x82910E78;
	sub_82508680(ctx, base);
	// 82910E78: 907F02E4  stw r3, 0x2e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(740 as u32), ctx.r[3].u32 ) };
	// 82910E7C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82910E80: 484E0E11  bl 0x82df1c90
	ctx.lr = 0x82910E84;
	sub_82DF1C90(ctx, base);
	// 82910E84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82910E88: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82910E8C: 4BBFE63D  bl 0x8250f4c8
	ctx.lr = 0x82910E90;
	sub_8250F4C8(ctx, base);
	// 82910E90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910E94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82910E98: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82910E9C: 409A0008  bne cr6, 0x82910ea4
	if !ctx.cr[6].eq {
	pc = 0x82910EA4; continue 'dispatch;
	}
	// 82910EA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82910EA4: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82910EA8: 4BBF77D9  bl 0x82508680
	ctx.lr = 0x82910EAC;
	sub_82508680(ctx, base);
	// 82910EAC: 907F02E8  stw r3, 0x2e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(744 as u32), ctx.r[3].u32 ) };
	// 82910EB0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82910EB4: 484E0DDD  bl 0x82df1c90
	ctx.lr = 0x82910EB8;
	sub_82DF1C90(ctx, base);
	// 82910EB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910EBC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82910EC0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82910EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910EC8: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82910ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82910ED0: 4E800421  bctrl
	ctx.lr = 0x82910ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82910ED4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82910ED8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82910EDC: 38A000D9  li r5, 0xd9
	ctx.r[5].s64 = 217;
	// 82910EE0: 386000EC  li r3, 0xec
	ctx.r[3].s64 = 236;
	// 82910EE4: 484E1505  bl 0x82df23e8
	ctx.lr = 0x82910EE8;
	sub_82DF23E8(ctx, base);
	// 82910EE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82910EEC: 41820010  beq 0x82910efc
	if ctx.cr[0].eq {
	pc = 0x82910EFC; continue 'dispatch;
	}
	// 82910EF0: 4BEE13D1  bl 0x827f22c0
	ctx.lr = 0x82910EF4;
	sub_827F22C0(ctx, base);
	// 82910EF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82910EF8: 48000008  b 0x82910f00
	pc = 0x82910F00; continue 'dispatch;
	// 82910EFC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82910F00: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82910F04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82910F08: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82910F0C: 4BFF983D  bl 0x8290a748
	ctx.lr = 0x82910F10;
	sub_8290A748(ctx, base);
	// 82910F10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82910F14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82910F18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82910F1C: 4B9AF0E5  bl 0x822c0000
	ctx.lr = 0x82910F20;
	sub_822C0000(ctx, base);
	// 82910F20: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82910F24: 3BDF01F4  addi r30, r31, 0x1f4
	ctx.r[30].s64 = ctx.r[31].s64 + 500;
	// 82910F28: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82910F2C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82910F30: 917F01F4  stw r11, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[11].u32 ) };
	// 82910F34: 4B9B352D  bl 0x822c4460
	ctx.lr = 0x82910F38;
	sub_822C4460(ctx, base);
	// 82910F38: 817F01F4  lwz r11, 0x1f4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82910F3C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82910F40: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82910F44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82910F48: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82910F4C: 697D0001  xori r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u64 ^ 1;
	// 82910F50: 419A0008  beq cr6, 0x82910f58
	if ctx.cr[6].eq {
	pc = 0x82910F58; continue 'dispatch;
	}
	// 82910F54: 4B9AF93D  bl 0x822c0890
	ctx.lr = 0x82910F58;
	sub_822C0890(ctx, base);
	// 82910F58: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82910F5C: 41820050  beq 0x82910fac
	if ctx.cr[0].eq {
	pc = 0x82910FAC; continue 'dispatch;
	}
	// 82910F60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82910F64: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910F68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82910F6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82910F70: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82910F74: 419A0024  beq cr6, 0x82910f98
	if ctx.cr[6].eq {
	pc = 0x82910F98; continue 'dispatch;
	}
	// 82910F78: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82910F7C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82910F80: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82910F84: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82910F88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82910F8C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82910F90: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82910F94: 4082FFE8  bne 0x82910f7c
	if !ctx.cr[0].eq {
	pc = 0x82910F7C; continue 'dispatch;
	}
	// 82910F98: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82910F9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82910FA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82910FA4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82910FA8: 4BBFC561  bl 0x8250d508
	ctx.lr = 0x82910FAC;
	sub_8250D508(ctx, base);
	// 82910FAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82910FB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82910FB4: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 82910FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82910FBC: 4E800421  bctrl
	ctx.lr = 0x82910FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82910FC0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82910FC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82910FC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82910FCC: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82910FD0: 4182001C  beq 0x82910fec
	if ctx.cr[0].eq {
	pc = 0x82910FEC; continue 'dispatch;
	}
	// 82910FD4: 38A000E4  li r5, 0xe4
	ctx.r[5].s64 = 228;
	// 82910FD8: 4B9AF401  bl 0x822c03d8
	ctx.lr = 0x82910FDC;
	sub_822C03D8(ctx, base);
	// 82910FDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82910FE0: 4182003C  beq 0x8291101c
	if ctx.cr[0].eq {
	pc = 0x8291101C; continue 'dispatch;
	}
	// 82910FE4: 38BE0038  addi r5, r30, 0x38
	ctx.r[5].s64 = ctx.r[30].s64 + 56;
	// 82910FE8: 48000024  b 0x8291100c
	pc = 0x8291100C; continue 'dispatch;
	// 82910FEC: 38A000E8  li r5, 0xe8
	ctx.r[5].s64 = 232;
	// 82910FF0: 4B9AF3E9  bl 0x822c03d8
	ctx.lr = 0x82910FF4;
	sub_822C03D8(ctx, base);
	// 82910FF4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82910FF8: 41820024  beq 0x8291101c
	if ctx.cr[0].eq {
	pc = 0x8291101C; continue 'dispatch;
	}
	// 82910FFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911000: 48006491  bl 0x82917490
	ctx.lr = 0x82911004;
	sub_82917490(ctx, base);
	// 82911004: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82911008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8291100C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82911010: 48006291  bl 0x829172a0
	ctx.lr = 0x82911014;
	sub_829172A0(ctx, base);
	// 82911014: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82911018: 48000008  b 0x82911020
	pc = 0x82911020; continue 'dispatch;
	// 8291101C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82911020: 387F0268  addi r3, r31, 0x268
	ctx.r[3].s64 = ctx.r[31].s64 + 616;
	// 82911024: 4BFFC8BD  bl 0x8290d8e0
	ctx.lr = 0x82911028;
	sub_8290D8E0(ctx, base);
	// 82911028: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8291102C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911030: 38A000EE  li r5, 0xee
	ctx.r[5].s64 = 238;
	// 82911034: 38600124  li r3, 0x124
	ctx.r[3].s64 = 292;
	// 82911038: 484E13B1  bl 0x82df23e8
	ctx.lr = 0x8291103C;
	sub_82DF23E8(ctx, base);
	// 8291103C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82911040: 41820024  beq 0x82911064
	if ctx.cr[0].eq {
	pc = 0x82911064; continue 'dispatch;
	}
	// 82911044: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82911048: 486F7F71  bl 0x83008fb8
	ctx.lr = 0x8291104C;
	sub_83008FB8(ctx, base);
	// 8291104C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82911050: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82911054: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82911058: 4BEB8D01  bl 0x827c9d58
	ctx.lr = 0x8291105C;
	sub_827C9D58(ctx, base);
	// 8291105C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82911060: 48000008  b 0x82911068
	pc = 0x82911068; continue 'dispatch;
	// 82911064: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82911068: 3BBF0300  addi r29, r31, 0x300
	ctx.r[29].s64 = ctx.r[31].s64 + 768;
	// 8291106C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82911070: 4BDD0A09  bl 0x826e1a78
	ctx.lr = 0x82911074;
	sub_826E1A78(ctx, base);
	// 82911074: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291107C: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 82911080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82911084: 4E800421  bctrl
	ctx.lr = 0x82911088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82911088: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8291108C: 41820080  beq 0x8291110c
	if ctx.cr[0].eq {
	pc = 0x8291110C; continue 'dispatch;
	}
	// 82911090: 897E0048  lbz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82911094: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82911098: 41820020  beq 0x829110b8
	if ctx.cr[0].eq {
	pc = 0x829110B8; continue 'dispatch;
	}
	// 8291109C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 829110A0: C07E0054  lfs f3, 0x54(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 829110A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 829110A8: C05E0050  lfs f2, 0x50(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 829110AC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 829110B0: C03E004C  lfs f1, 0x4c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829110B4: 4BEB8615  bl 0x827c96c8
	ctx.lr = 0x829110B8;
	sub_827C96C8(ctx, base);
	// 829110B8: 897E0058  lbz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 829110BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829110C0: 41820020  beq 0x829110e0
	if ctx.cr[0].eq {
	pc = 0x829110E0; continue 'dispatch;
	}
	// 829110C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 829110C8: C07E0064  lfs f3, 0x64(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 829110CC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 829110D0: C05E0060  lfs f2, 0x60(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 829110D4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 829110D8: C03E005C  lfs f1, 0x5c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829110DC: 4BEB85ED  bl 0x827c96c8
	ctx.lr = 0x829110E0;
	sub_827C96C8(ctx, base);
	// 829110E0: 897E0068  lbz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 829110E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829110E8: 41820024  beq 0x8291110c
	if ctx.cr[0].eq {
	pc = 0x8291110C; continue 'dispatch;
	}
	// 829110EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 829110F0: C03E006C  lfs f1, 0x6c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829110F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 829110F8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 829110FC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82911100: FC400890  fmr f2, f1
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82911104: C06BDD6C  lfs f3, -0x2294(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82911108: 4BEB85C1  bl 0x827c96c8
	ctx.lr = 0x8291110C;
	sub_827C96C8(ctx, base);
	// 8291110C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82911110: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911114: 38A00106  li r5, 0x106
	ctx.r[5].s64 = 262;
	// 82911118: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8291111C: 4B9AF2BD  bl 0x822c03d8
	ctx.lr = 0x82911120;
	sub_822C03D8(ctx, base);
	// 82911120: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82911124: 41820014  beq 0x82911138
	if ctx.cr[0].eq {
	pc = 0x82911138; continue 'dispatch;
	}
	// 82911128: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8291112C: 4801052D  bl 0x82921658
	ctx.lr = 0x82911130;
	sub_82921658(ctx, base);
	// 82911130: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82911134: 48000008  b 0x8291113c
	pc = 0x8291113C; continue 'dispatch;
	// 82911138: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8291113C: 387F0308  addi r3, r31, 0x308
	ctx.r[3].s64 = ctx.r[31].s64 + 776;
	// 82911140: 4BFFF3F9  bl 0x82910538
	ctx.lr = 0x82911144;
	sub_82910538(ctx, base);
	// 82911144: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291114C: 816B0184  lwz r11, 0x184(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(388 as u32) ) } as u64;
	// 82911150: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82911154: 4E800421  bctrl
	ctx.lr = 0x82911158;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82911158: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 8291115C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911160: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82911164: 917F034C  stw r11, 0x34c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(844 as u32), ctx.r[11].u32 ) };
	// 82911168: 48102519  bl 0x82a13680
	ctx.lr = 0x8291116C;
	sub_82A13680(ctx, base);
	// 8291116C: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82911170: D01F0334  stfs f0, 0x334(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(820 as u32), tmp.u32 ) };
	// 82911174: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82911178: 48897038  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82911180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82911180 size=156
    let mut pc: u32 = 0x82911180;
    'dispatch: loop {
        match pc {
            0x82911180 => {
    //   block [0x82911180..0x8291121C)
	// 82911180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82911184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82911188: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291118C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82911190: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82911194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82911198: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291119C: 4BFF949D  bl 0x8290a638
	ctx.lr = 0x829111A0;
	sub_8290A638(ctx, base);
	// 829111A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829111A4: 4182000C  beq 0x829111b0
	if ctx.cr[0].eq {
	pc = 0x829111B0; continue 'dispatch;
	}
	// 829111A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829111AC: 4BFFF8FD  bl 0x82910aa8
	ctx.lr = 0x829111B0;
	sub_82910AA8(ctx, base);
	// 829111B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 829111B4: 397F0268  addi r11, r31, 0x268
	ctx.r[11].s64 = ctx.r[31].s64 + 616;
	// 829111B8: 915F0268  stw r10, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[10].u32 ) };
	// 829111BC: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 829111C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829111C4: 915F026C  stw r10, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[10].u32 ) };
	// 829111C8: 419A0008  beq cr6, 0x829111d0
	if ctx.cr[6].eq {
	pc = 0x829111D0; continue 'dispatch;
	}
	// 829111CC: 4B9AF6C5  bl 0x822c0890
	ctx.lr = 0x829111D0;
	sub_822C0890(ctx, base);
	// 829111D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829111D4: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 829111D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829111DC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829111E0: 4BFFDB99  bl 0x8290ed78
	ctx.lr = 0x829111E4;
	sub_8290ED78(ctx, base);
	// 829111E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829111E8: 807F0308  lwz r3, 0x308(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 829111EC: 48010275  bl 0x82921460
	ctx.lr = 0x829111F0;
	sub_82921460(ctx, base);
	// 829111F0: 807F0310  lwz r3, 0x310(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(784 as u32) ) } as u64;
	// 829111F4: 48234E85  bl 0x82b46078
	ctx.lr = 0x829111F8;
	sub_82B46078(ctx, base);
	// 829111F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829111FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911200: 4BEDBA79  bl 0x827ecc78
	ctx.lr = 0x82911204;
	sub_827ECC78(ctx, base);
	// 82911204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82911208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291120C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82911210: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82911214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82911218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82911220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82911220 size=816
    let mut pc: u32 = 0x82911220;
    'dispatch: loop {
        match pc {
            0x82911220 => {
    //   block [0x82911220..0x82911550)
	// 82911220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82911224: 48896F41  bl 0x831a8164
	ctx.lr = 0x82911228;
	sub_831A8130(ctx, base);
	// 82911228: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8291122C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82911230: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82911234: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82911238: 4BEDB189  bl 0x827ec3c0
	ctx.lr = 0x8291123C;
	sub_827EC3C0(ctx, base);
	// 8291123C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911240: 4BEDB219  bl 0x827ec458
	ctx.lr = 0x82911244;
	sub_827EC458(ctx, base);
	// 82911244: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82911248: 4082003C  bne 0x82911284
	if !ctx.cr[0].eq {
	pc = 0x82911284; continue 'dispatch;
	}
	// 8291124C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911254: 816B0198  lwz r11, 0x198(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(408 as u32) ) } as u64;
	// 82911258: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8291125C: 4E800421  bctrl
	ctx.lr = 0x82911260;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82911260: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82911264: 41820020  beq 0x82911284
	if ctx.cr[0].eq {
	pc = 0x82911284; continue 'dispatch;
	}
	// 82911268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8291126C: 484E279D  bl 0x82df3a08
	ctx.lr = 0x82911270;
	sub_82DF3A08(ctx, base);
	// 82911270: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82911274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911278: 4BEDBA71  bl 0x827ecce8
	ctx.lr = 0x8291127C;
	sub_827ECCE8(ctx, base);
	// 8291127C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911280: 484E21A9  bl 0x82df3428
	ctx.lr = 0x82911284;
	sub_82DF3428(ctx, base);
	// 82911284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911288: 4BEDB2F1  bl 0x827ec578
	ctx.lr = 0x8291128C;
	sub_827EC578(ctx, base);
	// 8291128C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82911290: 4082003C  bne 0x829112cc
	if !ctx.cr[0].eq {
	pc = 0x829112CC; continue 'dispatch;
	}
	// 82911294: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291129C: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 829112A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829112A4: 4E800421  bctrl
	ctx.lr = 0x829112A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829112A8: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 829112AC: 41820020  beq 0x829112cc
	if ctx.cr[0].eq {
	pc = 0x829112CC; continue 'dispatch;
	}
	// 829112B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829112B4: 484E2755  bl 0x82df3a08
	ctx.lr = 0x829112B8;
	sub_82DF3A08(ctx, base);
	// 829112B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829112BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829112C0: 4BEDB249  bl 0x827ec508
	ctx.lr = 0x829112C4;
	sub_827EC508(ctx, base);
	// 829112C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829112C8: 484E2161  bl 0x82df3428
	ctx.lr = 0x829112CC;
	sub_82DF3428(ctx, base);
	// 829112CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 829112D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829112D4: 4BFFF645  bl 0x82910918
	ctx.lr = 0x829112D8;
	sub_82910918(ctx, base);
	// 829112D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829112DC: C01F033C  lfs f0, 0x33c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829112E0: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 829112E4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 829112E8: 4099002C  ble cr6, 0x82911314
	if !ctx.cr[6].gt {
	pc = 0x82911314; continue 'dispatch;
	}
	// 829112EC: C1BC0000  lfs f13, 0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 829112F0: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 829112F4: D01F033C  stfs f0, 0x33c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), tmp.u32 ) };
	// 829112F8: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 829112FC: 41990018  bgt cr6, 0x82911314
	if ctx.cr[6].gt {
	pc = 0x82911314; continue 'dispatch;
	}
	// 82911300: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82911304: D3FF033C  stfs f31, 0x33c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), tmp.u32 ) };
	// 82911308: 816AB1F8  lwz r11, -0x4e08(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19976 as u32) ) } as u64;
	// 8291130C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82911310: 916AB1F8  stw r11, -0x4e08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19976 as u32), ctx.r[11].u32 ) };
	// 82911314: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911318: 4BB6C1C9  bl 0x8247d4e0
	ctx.lr = 0x8291131C;
	sub_8247D4E0(ctx, base);
	// 8291131C: 83610054  lwz r27, 0x54(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82911320: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82911324: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82911328: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 8291132C: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82911330: 419A0024  beq cr6, 0x82911354
	if ctx.cr[6].eq {
	pc = 0x82911354; continue 'dispatch;
	}
	// 82911334: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 82911338: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8291133C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82911340: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82911344: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82911348: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8291134C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82911350: 4082FFE8  bne 0x82911338
	if !ctx.cr[0].eq {
	pc = 0x82911338; continue 'dispatch;
	}
	// 82911354: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82911358: 80DF02D0  lwz r6, 0x2d0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(720 as u32) ) } as u64;
	// 8291135C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82911360: 388B5758  addi r4, r11, 0x5758
	ctx.r[4].s64 = ctx.r[11].s64 + 22360;
	// 82911364: 38A0016B  li r5, 0x16b
	ctx.r[5].s64 = 363;
	// 82911368: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8291136C: 48545C7D  bl 0x82e56fe8
	ctx.lr = 0x82911370;
	sub_82E56FE8(ctx, base);
	// 82911370: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82911374: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82911378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8291137C: 419A000C  beq cr6, 0x82911388
	if ctx.cr[6].eq {
	pc = 0x82911388; continue 'dispatch;
	}
	// 82911380: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82911384: 4B9AF50D  bl 0x822c0890
	ctx.lr = 0x82911388;
	sub_822C0890(ctx, base);
	// 82911388: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291138C: 41820054  beq 0x829113e0
	if ctx.cr[0].eq {
	pc = 0x829113E0; continue 'dispatch;
	}
	// 82911390: C01E0020  lfs f0, 0x20(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82911394: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82911398: D01F0270  stfs f0, 0x270(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), tmp.u32 ) };
	// 8291139C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 829113A0: C01E0024  lfs f0, 0x24(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829113A4: 395F0270  addi r10, r31, 0x270
	ctx.r[10].s64 = ctx.r[31].s64 + 624;
	// 829113A8: D01F0274  stfs f0, 0x274(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), tmp.u32 ) };
	// 829113AC: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 829113B0: 13EB30C7  vcmpequd (lvx128) v31, v11, v6
	tmp.u32 = ctx.r[11].u32 + ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829113B4: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82911550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82911550 size=332
    let mut pc: u32 = 0x82911550;
    'dispatch: loop {
        match pc {
            0x82911550 => {
    //   block [0x82911550..0x8291169C)
	// 82911550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82911554: 48896C19  bl 0x831a816c
	ctx.lr = 0x82911558;
	sub_831A8130(ctx, base);
	// 82911558: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291155C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82911560: 4BFFF549  bl 0x82910aa8
	ctx.lr = 0x82911564;
	sub_82910AA8(ctx, base);
	// 82911564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911568: 4BFF8FE9  bl 0x8290a550
	ctx.lr = 0x8291156C;
	sub_8290A550(ctx, base);
	// 8291156C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82911570: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82911574: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82911578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8291157C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82911580: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82911584: 38AA9BC9  addi r5, r10, -0x6437
	ctx.r[5].s64 = ctx.r[10].s64 + -25655;
	// 82911588: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8291158C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82911590: 4804CBD9  bl 0x8295e168
	ctx.lr = 0x82911594;
	sub_8295E168(ctx, base);
	// 82911594: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911598: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8291159C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 829115A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829115A4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 829115A8: 419A0024  beq cr6, 0x829115cc
	if ctx.cr[6].eq {
	pc = 0x829115CC; continue 'dispatch;
	}
	// 829115AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 829115B0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 829115B4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829115B8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 829115BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 829115C0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 829115C4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829115C8: 4082FFE8  bne 0x829115b0
	if !ctx.cr[0].eq {
	pc = 0x829115B0; continue 'dispatch;
	}
	// 829115CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 829115D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829115D4: 4BBFDEF5  bl 0x8250f4c8
	ctx.lr = 0x829115D8;
	sub_8250F4C8(ctx, base);
	// 829115D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829115DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829115E0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 829115E4: 409A0008  bne cr6, 0x829115ec
	if !ctx.cr[6].eq {
	pc = 0x829115EC; continue 'dispatch;
	}
	// 829115E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 829115EC: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 829115F0: 4BBF7871  bl 0x82508e60
	ctx.lr = 0x829115F4;
	sub_82508E60(ctx, base);
	// 829115F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 829115F8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 829115FC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82911600: 388A5758  addi r4, r10, 0x5758
	ctx.r[4].s64 = ctx.r[10].s64 + 22360;
	// 82911604: 38A0025B  li r5, 0x25b
	ctx.r[5].s64 = 603;
	// 82911608: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8291160C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82911610: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82911614: 4854742D  bl 0x82e58a40
	ctx.lr = 0x82911618;
	sub_82E58A40(ctx, base);
	// 82911618: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8291161C: 484E0675  bl 0x82df1c90
	ctx.lr = 0x82911620;
	sub_82DF1C90(ctx, base);
	// 82911620: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82911624: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82911628: 419A0008  beq cr6, 0x82911630
	if ctx.cr[6].eq {
	pc = 0x82911630; continue 'dispatch;
	}
	// 8291162C: 4B9AF265  bl 0x822c0890
	ctx.lr = 0x82911630;
	sub_822C0890(ctx, base);
	// 82911630: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82911634: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82911638: 419A0008  beq cr6, 0x82911640
	if ctx.cr[6].eq {
	pc = 0x82911640; continue 'dispatch;
	}
	// 8291163C: 4B9AF255  bl 0x822c0890
	ctx.lr = 0x82911640;
	sub_822C0890(ctx, base);
	// 82911640: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 82911644: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82911648: 40820040  bne 0x82911688
	if !ctx.cr[0].eq {
	pc = 0x82911688; continue 'dispatch;
	}
	// 8291164C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82911650: 4BBD9439  bl 0x824eaa88
	ctx.lr = 0x82911654;
	sub_824EAA88(ctx, base);
	// 82911654: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82911658: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8291165C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911660: 4BBD9C61  bl 0x824eb2c0
	ctx.lr = 0x82911664;
	sub_824EB2C0(ctx, base);
	// 82911664: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82911668: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291166C: 4BC06B95  bl 0x82518200
	ctx.lr = 0x82911670;
	sub_82518200(ctx, base);
	// 82911670: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82911674: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82911678: 419A0008  beq cr6, 0x82911680
	if ctx.cr[6].eq {
	pc = 0x82911680; continue 'dispatch;
	}
	// 8291167C: 4B9AF215  bl 0x822c0890
	ctx.lr = 0x82911680;
	sub_822C0890(ctx, base);
	// 82911680: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82911684: 484E060D  bl 0x82df1c90
	ctx.lr = 0x82911688;
	sub_82DF1C90(ctx, base);
	// 82911688: 9BDF0348  stb r30, 0x348(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(840 as u32), ctx.r[30].u8 ) };
	// 8291168C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911690: 4BC00209  bl 0x82511898
	ctx.lr = 0x82911694;
	sub_82511898(ctx, base);
	// 82911694: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82911698: 48896B24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829116A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829116A0 size=320
    let mut pc: u32 = 0x829116A0;
    'dispatch: loop {
        match pc {
            0x829116A0 => {
    //   block [0x829116A0..0x829117E0)
	// 829116A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829116A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829116A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829116AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829116B0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829116B4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 829116B8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829116BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829116C0: 3BEBB1B0  addi r31, r11, -0x4e50
	ctx.r[31].s64 = ctx.r[11].s64 + -20048;
	// 829116C4: 816AB1C0  lwz r11, -0x4e40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20032 as u32) ) } as u64;
	// 829116C8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 829116CC: 4082002C  bne 0x829116f8
	if !ctx.cr[0].eq {
	pc = 0x829116F8; continue 'dispatch;
	}
	// 829116D0: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 829116D4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 829116D8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 829116DC: 916AB1C0  stw r11, -0x4e40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20032 as u32), ctx.r[11].u32 ) };
	// 829116E0: C00989AC  lfs f0, -0x7654(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829116E4: C1A808A4  lfs f13, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 829116E8: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 829116EC: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 829116F0: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 829116F4: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 829116F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829116FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911700: 4BBFDE19  bl 0x8250f518
	ctx.lr = 0x82911704;
	sub_8250F518(ctx, base);
	// 82911704: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8291170C: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82911710: 409A0008  bne cr6, 0x82911718
	if !ctx.cr[6].eq {
	pc = 0x82911718; continue 'dispatch;
	}
	// 82911714: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82911718: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8291171C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82911720: 4BC169D1  bl 0x825280f0
	ctx.lr = 0x82911724;
	sub_825280F0(ctx, base);
	// 82911724: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911728: 484E0569  bl 0x82df1c90
	ctx.lr = 0x8291172C;
	sub_82DF1C90(ctx, base);
	// 8291172C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82911730: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82911734: 4856BDF5  bl 0x82e7d528
	ctx.lr = 0x82911738;
	sub_82E7D528(ctx, base);
	// 82911738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8291173C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82911740: 4856BDE9  bl 0x82e7d528
	ctx.lr = 0x82911744;
	sub_82E7D528(ctx, base);
	// 82911744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82911748: 48101F39  bl 0x82a13680
	ctx.lr = 0x8291174C;
	sub_82A13680(ctx, base);
	// 8291174C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82911750: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82911754: C1A10064  lfs f13, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829117E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829117E0 size=92
    let mut pc: u32 = 0x829117E0;
    'dispatch: loop {
        match pc {
            0x829117E0 => {
    //   block [0x829117E0..0x8291183C)
	// 829117E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829117E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829117E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829117EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829117F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829117F4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 829117F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829117FC: 4BD6BD6D  bl 0x8267d568
	ctx.lr = 0x82911800;
	sub_8267D568(ctx, base);
	// 82911800: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82911804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911808: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8291180C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82911810: 4BBA39C9  bl 0x824b51d8
	ctx.lr = 0x82911814;
	sub_824B51D8(ctx, base);
	// 82911814: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82911818: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8291181C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82911820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911824: 4BBA39B5  bl 0x824b51d8
	ctx.lr = 0x82911828;
	sub_824B51D8(ctx, base);
	// 82911828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8291182C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82911830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82911834: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82911838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82911840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82911840 size=140
    let mut pc: u32 = 0x82911840;
    'dispatch: loop {
        match pc {
            0x82911840 => {
    //   block [0x82911840..0x829118CC)
	// 82911840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82911844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82911848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291184C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82911850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82911854: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82911858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291185C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82911860: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911864: 388B5AA4  addi r4, r11, 0x5aa4
	ctx.r[4].s64 = ctx.r[11].s64 + 23204;
	// 82911868: 484E21A1  bl 0x82df3a08
	ctx.lr = 0x8291186C;
	sub_82DF3A08(ctx, base);
	// 8291186C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82911870: 38BF0348  addi r5, r31, 0x348
	ctx.r[5].s64 = ctx.r[31].s64 + 840;
	// 82911874: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911878: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8291187C: 4BCE680D  bl 0x825f8088
	ctx.lr = 0x82911880;
	sub_825F8088(ctx, base);
	// 82911880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911884: 484E1BA5  bl 0x82df3428
	ctx.lr = 0x82911888;
	sub_82DF3428(ctx, base);
	// 82911888: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8291188C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911890: 388B7AF0  addi r4, r11, 0x7af0
	ctx.r[4].s64 = ctx.r[11].s64 + 31472;
	// 82911894: 484E2175  bl 0x82df3a08
	ctx.lr = 0x82911898;
	sub_82DF3A08(ctx, base);
	// 82911898: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8291189C: 38BF0350  addi r5, r31, 0x350
	ctx.r[5].s64 = ctx.r[31].s64 + 848;
	// 829118A0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 829118A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829118A8: 4BBFB5F9  bl 0x8250cea0
	ctx.lr = 0x829118AC;
	sub_8250CEA0(ctx, base);
	// 829118AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829118B0: 484E1B79  bl 0x82df3428
	ctx.lr = 0x829118B4;
	sub_82DF3428(ctx, base);
	// 829118B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829118B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829118BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829118C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829118C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829118C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829118D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829118D0 size=76
    let mut pc: u32 = 0x829118D0;
    'dispatch: loop {
        match pc {
            0x829118D0 => {
    //   block [0x829118D0..0x8291191C)
	// 829118D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829118D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829118D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829118DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829118E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829118E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829118E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829118EC: 4BFFF3A5  bl 0x82910c90
	ctx.lr = 0x829118F0;
	sub_82910C90(ctx, base);
	// 829118F0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829118F4: 4182000C  beq 0x82911900
	if ctx.cr[0].eq {
	pc = 0x82911900; continue 'dispatch;
	}
	// 829118F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829118FC: 484E0ADD  bl 0x82df23d8
	ctx.lr = 0x82911900;
	sub_82DF23D8(ctx, base);
	// 82911900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82911908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291190C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82911910: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82911914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82911918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82911920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82911920 size=472
    let mut pc: u32 = 0x82911920;
    'dispatch: loop {
        match pc {
            0x82911920 => {
    //   block [0x82911920..0x82911AF8)
	// 82911920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82911924: 48896845  bl 0x831a8168
	ctx.lr = 0x82911928;
	sub_831A8130(ctx, base);
	// 82911928: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8291192C: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82911930: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82911934: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82911938: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8291193C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82911940: 4B9D58C9  bl 0x822e7208
	ctx.lr = 0x82911944;
	sub_822E7208(ctx, base);
	// 82911944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911948: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8291194C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82911950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82911954: 4B9D5A3D  bl 0x822e7390
	ctx.lr = 0x82911958;
	sub_822E7390(ctx, base);
	// 82911958: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8291195C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82911960: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82911964: 93E10084  stw r31, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 82911968: 93E10088  stw r31, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 8291196C: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82911970: 4B9DD4D9  bl 0x822eee48
	ctx.lr = 0x82911974;
	sub_822EEE48(ctx, base);
	// 82911974: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82911978: 38DE0028  addi r6, r30, 0x28
	ctx.r[6].s64 = ctx.r[30].s64 + 40;
	// 8291197C: 409A0008  bne cr6, 0x82911984
	if !ctx.cr[6].eq {
	pc = 0x82911984; continue 'dispatch;
	}
	// 82911980: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82911984: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82911988: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8291198C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82911990: 4B9DCD49  bl 0x822ee6d8
	ctx.lr = 0x82911994;
	sub_822EE6D8(ctx, base);
	// 82911994: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82911998: 3BFE01E8  addi r31, r30, 0x1e8
	ctx.r[31].s64 = ctx.r[30].s64 + 488;
	// 8291199C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 829119A0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 829119A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829119A8: 917E01E8  stw r11, 0x1e8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(488 as u32), ctx.r[11].u32 ) };
	// 829119AC: 4B9B2AB5  bl 0x822c4460
	ctx.lr = 0x829119B0;
	sub_822C4460(ctx, base);
	// 829119B0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 829119B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829119B8: 419A0008  beq cr6, 0x829119c0
	if ctx.cr[6].eq {
	pc = 0x829119C0; continue 'dispatch;
	}
	// 829119BC: 4B9AEED5  bl 0x822c0890
	ctx.lr = 0x829119C0;
	sub_822C0890(ctx, base);
	// 829119C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 829119C4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829119C8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 829119CC: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 829119D0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 829119D4: 80CB6844  lwz r6, 0x6844(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26692 as u32) ) } as u64;
	// 829119D8: 80AA686C  lwz r5, 0x686c(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26732 as u32) ) } as u64;
	// 829119DC: 80896750  lwz r4, 0x6750(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26448 as u32) ) } as u64;
	// 829119E0: 4B9E5E39  bl 0x822f7818
	ctx.lr = 0x829119E4;
	sub_822F7818(ctx, base);
	// 829119E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 829119E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 829119EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 829119F0: 808B6874  lwz r4, 0x6874(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26740 as u32) ) } as u64;
	// 829119F4: 4B9D353D  bl 0x822e4f30
	ctx.lr = 0x829119F8;
	sub_822E4F30(ctx, base);
	// 829119F8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 829119FC: E89C0000  ld r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 82911A00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911A04: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82911A08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82911A0C: 4BB7A87D  bl 0x8248c288
	ctx.lr = 0x82911A10;
	sub_8248C288(ctx, base);
	// 82911A10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82911A14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82911A18: 4BB7D989  bl 0x8248f3a0
	ctx.lr = 0x82911A1C;
	sub_8248F3A0(ctx, base);
	// 82911A1C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82911A20: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A24: 4B9D810D  bl 0x822e9b30
	ctx.lr = 0x82911A28;
	sub_822E9B30(ctx, base);
	// 82911A28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82911A2C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A30: 48101C51  bl 0x82a13680
	ctx.lr = 0x82911A34;
	sub_82A13680(ctx, base);
	// 82911A34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82911A38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82911A3C: 4B9DBCE5  bl 0x822ed720
	ctx.lr = 0x82911A40;
	sub_822ED720(ctx, base);
	// 82911A40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82911A44: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A48: 4BB7D901  bl 0x8248f348
	ctx.lr = 0x82911A4C;
	sub_8248F348(ctx, base);
	// 82911A4C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82911A50: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A54: C3EB7BC8  lfs f31, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82911A58: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82911A5C: 4B9DC0FD  bl 0x822edb58
	ctx.lr = 0x82911A60;
	sub_822EDB58(ctx, base);
	// 82911A60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82911A64: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A68: C02B9C28  lfs f1, -0x63d8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25560 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82911A6C: 4B9DB9A5  bl 0x822ed410
	ctx.lr = 0x82911A70;
	sub_822ED410(ctx, base);
	// 82911A70: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82911A74: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A78: C02B093C  lfs f1, 0x93c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2364 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82911A7C: 4B9DB91D  bl 0x822ed398
	ctx.lr = 0x82911A80;
	sub_822ED398(ctx, base);
	// 82911A80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82911A84: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A88: C3CB964C  lfs f30, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82911A8C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82911A90: 4B9DB711  bl 0x822ed1a0
	ctx.lr = 0x82911A94;
	sub_822ED1A0(ctx, base);
	// 82911A94: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911A98: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82911A9C: 4B9DB715  bl 0x822ed1b0
	ctx.lr = 0x82911AA0;
	sub_822ED1B0(ctx, base);
	// 82911AA0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911AA4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82911AA8: 4B9DB719  bl 0x822ed1c0
	ctx.lr = 0x82911AAC;
	sub_822ED1C0(ctx, base);
	// 82911AAC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82911AB0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911AB4: C02BACFC  lfs f1, -0x5304(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21252 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82911AB8: 4B9DB711  bl 0x822ed1c8
	ctx.lr = 0x82911ABC;
	sub_822ED1C8(ctx, base);
	// 82911ABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82911AC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82911AC4: 4BBFF27D  bl 0x82510d40
	ctx.lr = 0x82911AC8;
	sub_82510D40(ctx, base);
	// 82911AC8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82911ACC: 4BAC091D  bl 0x823d23e8
	ctx.lr = 0x82911AD0;
	sub_823D23E8(ctx, base);
	// 82911AD0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82911AD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82911AD8: 419A0008  beq cr6, 0x82911ae0
	if ctx.cr[6].eq {
	pc = 0x82911AE0; continue 'dispatch;
	}
	// 82911ADC: 4B9AEDB5  bl 0x822c0890
	ctx.lr = 0x82911AE0;
	sub_822C0890(ctx, base);
	// 82911AE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82911AE4: 4B9D573D  bl 0x822e7220
	ctx.lr = 0x82911AE8;
	sub_822E7220(ctx, base);
	// 82911AE8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82911AEC: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82911AF0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82911AF4: 488966C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82911AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82911AF8 size=788
    let mut pc: u32 = 0x82911AF8;
    'dispatch: loop {
        match pc {
            0x82911AF8 => {
    //   block [0x82911AF8..0x82911E0C)
	// 82911AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82911AFC: 4889665D  bl 0x831a8158
	ctx.lr = 0x82911B00;
	sub_831A8130(ctx, base);
	// 82911B00: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82911B04: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82911B08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82911B0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82911B10: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82911B14: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82911B18: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911B1C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82911B20: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82911B24: 48518FC5  bl 0x82e2aae8
	ctx.lr = 0x82911B28;
	sub_82E2AAE8(ctx, base);
	// 82911B28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911B2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82911B30: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82911B34: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82911B38: 4851D339  bl 0x82e2ee70
	ctx.lr = 0x82911B3C;
	sub_82E2EE70(ctx, base);
	// 82911B3C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82911B40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82911B44: 419A029C  beq cr6, 0x82911de0
	if ctx.cr[6].eq {
	pc = 0x82911DE0; continue 'dispatch;
	}
	// 82911B48: 815E034C  lwz r10, 0x34c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(844 as u32) ) } as u64;
	// 82911B4C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82911B50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911B54: 554A0734  rlwinm r10, r10, 0, 0x1c, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82911B58: 3BAB5758  addi r29, r11, 0x5758
	ctx.r[29].s64 = ctx.r[11].s64 + 22360;
	// 82911B5C: 915E034C  stw r10, 0x34c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(844 as u32), ctx.r[10].u32 ) };
	// 82911B60: 38A001F8  li r5, 0x1f8
	ctx.r[5].s64 = 504;
	// 82911B64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82911B68: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82911B6C: 484E087D  bl 0x82df23e8
	ctx.lr = 0x82911B70;
	sub_82DF23E8(ctx, base);
	// 82911B70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82911B74: 41820014  beq 0x82911b88
	if ctx.cr[0].eq {
	pc = 0x82911B88; continue 'dispatch;
	}
	// 82911B78: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82911B7C: 48504D15  bl 0x82e16890
	ctx.lr = 0x82911B80;
	sub_82E16890(ctx, base);
	// 82911B80: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82911B84: 48000008  b 0x82911b8c
	pc = 0x82911B8C; continue 'dispatch;
	// 82911B88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82911B8C: 3BFE00EC  addi r31, r30, 0xec
	ctx.r[31].s64 = ctx.r[30].s64 + 236;
	// 82911B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82911B94: 4BA4B7DD  bl 0x8235d370
	ctx.lr = 0x82911B98;
	sub_8235D370(ctx, base);
	// 82911B98: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82911B9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82911BA0: 4BEDA5B9  bl 0x827ec158
	ctx.lr = 0x82911BA4;
	sub_827EC158(ctx, base);
	// 82911BA4: 809E00EC  lwz r4, 0xec(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 82911BA8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82911BAC: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82911BB0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82911BB4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82911BB8: 80640084  lwz r3, 0x84(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 82911BBC: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82911BC0: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82911BC4: 13E85C07  vcmpneb. (lvlx128) v31, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82911BC8: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82911BCC: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82911BD0: 13C93C07  vcmpneb. (lvlx128) v30, v9, v7
	tmp.u32 = ctx.r[9].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82911BD4: 13AA3407  vcmpneb. (lvlx128) v29, v10, v6
	tmp.u32 = ctx.r[10].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82911BD8: 13802C07  vcmpneb. (lvlx128) v28, v0, v5
	tmp.u32 = ctx.r[5].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82911E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82911E10 size=856
    let mut pc: u32 = 0x82911E10;
    'dispatch: loop {
        match pc {
            0x82911E10 => {
    //   block [0x82911E10..0x82912168)
	// 82911E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82911E14: 48896339  bl 0x831a814c
	ctx.lr = 0x82911E18;
	sub_831A8130(ctx, base);
	// 82911E18: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82911E1C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82911E20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82911E24: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82911E28: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82911E2C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82911E30: 80970000  lwz r4, 0(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82911E34: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82911E38: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82911E3C: 48518CAD  bl 0x82e2aae8
	ctx.lr = 0x82911E40;
	sub_82E2AAE8(ctx, base);
	// 82911E40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911E44: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82911E48: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82911E4C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82911E50: 4851D021  bl 0x82e2ee70
	ctx.lr = 0x82911E54;
	sub_82E2EE70(ctx, base);
	// 82911E54: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82911E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82911E5C: 419A02E0  beq cr6, 0x8291213c
	if ctx.cr[6].eq {
	pc = 0x8291213C; continue 'dispatch;
	}
	// 82911E60: 815E034C  lwz r10, 0x34c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(844 as u32) ) } as u64;
	// 82911E64: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82911E68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82911E6C: 554A0734  rlwinm r10, r10, 0, 0x1c, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82911E70: 3ACB5758  addi r22, r11, 0x5758
	ctx.r[22].s64 = ctx.r[11].s64 + 22360;
	// 82911E74: 915E034C  stw r10, 0x34c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(844 as u32), ctx.r[10].u32 ) };
	// 82911E78: 38A0021C  li r5, 0x21c
	ctx.r[5].s64 = 540;
	// 82911E7C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82911E80: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82911E84: 484E0565  bl 0x82df23e8
	ctx.lr = 0x82911E88;
	sub_82DF23E8(ctx, base);
	// 82911E88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82911E8C: 41820014  beq 0x82911ea0
	if ctx.cr[0].eq {
	pc = 0x82911EA0; continue 'dispatch;
	}
	// 82911E90: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82911E94: 485049FD  bl 0x82e16890
	ctx.lr = 0x82911E98;
	sub_82E16890(ctx, base);
	// 82911E98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82911E9C: 48000008  b 0x82911ea4
	pc = 0x82911EA4; continue 'dispatch;
	// 82911EA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82911EA4: 3BBE00EC  addi r29, r30, 0xec
	ctx.r[29].s64 = ctx.r[30].s64 + 236;
	// 82911EA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82911EAC: 4BA4B4C5  bl 0x8235d370
	ctx.lr = 0x82911EB0;
	sub_8235D370(ctx, base);
	// 82911EB0: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82911EB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82911EB8: 4BEDA2A1  bl 0x827ec158
	ctx.lr = 0x82911EBC;
	sub_827EC158(ctx, base);
	// 82911EBC: 809E00EC  lwz r4, 0xec(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 82911EC0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82911EC4: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82911EC8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82911ECC: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82911ED0: 80640084  lwz r3, 0x84(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 82911ED4: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82911ED8: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82911EDC: 13E85C07  vcmpneb. (lvlx128) v31, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82911EE0: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 82911EE4: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82911EE8: 13C93C07  vcmpneb. (lvlx128) v30, v9, v7
	tmp.u32 = ctx.r[9].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82911EEC: 13AA3407  vcmpneb. (lvlx128) v29, v10, v6
	tmp.u32 = ctx.r[10].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82911EF0: 13802C07  vcmpneb. (lvlx128) v28, v0, v5
	tmp.u32 = ctx.r[5].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912168 size=68
    let mut pc: u32 = 0x82912168;
    'dispatch: loop {
        match pc {
            0x82912168 => {
    //   block [0x82912168..0x829121AC)
	// 82912168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291216C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912170: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82912174: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291217C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82912180: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82912184: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82912188: 388A9BC9  addi r4, r10, -0x6437
	ctx.r[4].s64 = ctx.r[10].s64 + -25655;
	// 8291218C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912190: 484E1879  bl 0x82df3a08
	ctx.lr = 0x82912194;
	sub_82DF3A08(ctx, base);
	// 82912194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82912198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8291219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829121A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829121A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829121A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829121B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829121B0 size=8
    let mut pc: u32 = 0x829121B0;
    'dispatch: loop {
        match pc {
            0x829121B0 => {
    //   block [0x829121B0..0x829121B8)
	// 829121B0: 38630180  addi r3, r3, 0x180
	ctx.r[3].s64 = ctx.r[3].s64 + 384;
	// 829121B4: 48002CBC  b 0x82914e70
	sub_82914E70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829121B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829121B8 size=20
    let mut pc: u32 = 0x829121B8;
    'dispatch: loop {
        match pc {
            0x829121B8 => {
    //   block [0x829121B8..0x829121CC)
	// 829121B8: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 829121BC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 829121C0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 829121C4: 386B0044  addi r3, r11, 0x44
	ctx.r[3].s64 = ctx.r[11].s64 + 68;
	// 829121C8: 484E1A08  b 0x82df3bd0
	sub_82DF3BD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829121D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x829121D0 size=8
    let mut pc: u32 = 0x829121D0;
    'dispatch: loop {
        match pc {
            0x829121D0 => {
    //   block [0x829121D0..0x829121D8)
	// 829121D0: D0230134  stfs f1, 0x134(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 829121D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829121D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829121D8 size=8
    let mut pc: u32 = 0x829121D8;
    'dispatch: loop {
        match pc {
            0x829121D8 => {
    //   block [0x829121D8..0x829121E0)
	// 829121D8: 98830184  stb r4, 0x184(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(388 as u32), ctx.r[4].u8 ) };
	// 829121DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829121E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829121E0 size=340
    let mut pc: u32 = 0x829121E0;
    'dispatch: loop {
        match pc {
            0x829121E0 => {
    //   block [0x829121E0..0x82912334)
	// 829121E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829121E4: 48895F81  bl 0x831a8164
	ctx.lr = 0x829121E8;
	sub_831A8130(ctx, base);
	// 829121E8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829121EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829121F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 829121F4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 829121F8: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 829121FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82912200: 418200F0  beq 0x829122f0
	if ctx.cr[0].eq {
	pc = 0x829122F0; continue 'dispatch;
	}
	// 82912204: 3BE000FF  li r31, 0xff
	ctx.r[31].s64 = 255;
	// 82912208: 809E005C  lwz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8291220C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82912210: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82912214: 9BE10051  stb r31, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[31].u8 ) };
	// 82912218: 9BE10052  stb r31, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[31].u8 ) };
	// 8291221C: 9BE10053  stb r31, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[31].u8 ) };
	// 82912220: 4BBFD2A9  bl 0x8250f4c8
	ctx.lr = 0x82912224;
	sub_8250F4C8(ctx, base);
	// 82912224: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8291222C: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82912230: 409A0008  bne cr6, 0x82912238
	if !ctx.cr[6].eq {
	pc = 0x82912238; continue 'dispatch;
	}
	// 82912234: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82912238: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8291223C: 4BBF6775  bl 0x825089b0
	ctx.lr = 0x82912240;
	sub_825089B0(ctx, base);
	// 82912240: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82912248: 386BFF34  addi r3, r11, -0xcc
	ctx.r[3].s64 = ctx.r[11].s64 + -204;
	// 8291224C: 409A0008  bne cr6, 0x82912254
	if !ctx.cr[6].eq {
	pc = 0x82912254; continue 'dispatch;
	}
	// 82912250: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82912254: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82912258: C03E00E4  lfs f1, 0xe4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8291225C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82912260: 3BABBA80  addi r29, r11, -0x4580
	ctx.r[29].s64 = ctx.r[11].s64 + -17792;
	// 82912264: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82912268: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8291226C: 4B9B57FD  bl 0x822c7a68
	ctx.lr = 0x82912270;
	sub_822C7A68(ctx, base);
	// 82912270: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82912274: 484DFA1D  bl 0x82df1c90
	ctx.lr = 0x82912278;
	sub_82DF1C90(ctx, base);
	// 82912278: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8291227C: 484DFA15  bl 0x82df1c90
	ctx.lr = 0x82912280;
	sub_82DF1C90(ctx, base);
	// 82912280: 9BE10054  stb r31, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u8 ) };
	// 82912284: 9BE10055  stb r31, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[31].u8 ) };
	// 82912288: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8291228C: 9BE10056  stb r31, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[31].u8 ) };
	// 82912290: 9BE10057  stb r31, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[31].u8 ) };
	// 82912294: 809E005C  lwz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82912298: 4BBFD231  bl 0x8250f4c8
	ctx.lr = 0x8291229C;
	sub_8250F4C8(ctx, base);
	// 8291229C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829122A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829122A4: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 829122A8: 409A0008  bne cr6, 0x829122b0
	if !ctx.cr[6].eq {
	pc = 0x829122B0; continue 'dispatch;
	}
	// 829122AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 829122B0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 829122B4: 4BBF66FD  bl 0x825089b0
	ctx.lr = 0x829122B8;
	sub_825089B0(ctx, base);
	// 829122B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829122BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829122C0: 386BFF34  addi r3, r11, -0xcc
	ctx.r[3].s64 = ctx.r[11].s64 + -204;
	// 829122C4: 409A0008  bne cr6, 0x829122cc
	if !ctx.cr[6].eq {
	pc = 0x829122CC; continue 'dispatch;
	}
	// 829122C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 829122CC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 829122D0: C03E00E4  lfs f1, 0xe4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829122D4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 829122D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 829122DC: 4B9B578D  bl 0x822c7a68
	ctx.lr = 0x829122E0;
	sub_822C7A68(ctx, base);
	// 829122E0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 829122E4: 484DF9AD  bl 0x82df1c90
	ctx.lr = 0x829122E8;
	sub_82DF1C90(ctx, base);
	// 829122E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 829122EC: 484DF9A5  bl 0x82df1c90
	ctx.lr = 0x829122F0;
	sub_82DF1C90(ctx, base);
	// 829122F0: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 829122F4: 83FE0108  lwz r31, 0x108(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) } as u64;
	// 829122F8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 829122FC: 486F6CBD  bl 0x83008fb8
	ctx.lr = 0x82912300;
	sub_83008FB8(ctx, base);
	// 82912300: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82912304: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82912308: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8291230C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82912310: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82912314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82912318: 4BB7FC51  bl 0x82491f68
	ctx.lr = 0x8291231C;
	sub_82491F68(ctx, base);
	// 8291231C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82912320: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82912324: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82912328: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8291232C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82912330: 48895E84  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912338 size=196
    let mut pc: u32 = 0x82912338;
    'dispatch: loop {
        match pc {
            0x82912338 => {
    //   block [0x82912338..0x829123FC)
	// 82912338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82912344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82912348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291234C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82912350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82912354: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82912358: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8291235C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912360: 4B9AE5D9  bl 0x822c0938
	ctx.lr = 0x82912364;
	sub_822C0938(ctx, base);
	// 82912364: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82912368: 41820028  beq 0x82912390
	if ctx.cr[0].eq {
	pc = 0x82912390; continue 'dispatch;
	}
	// 8291236C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82912370: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82912374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82912378: 392B5AB4  addi r9, r11, 0x5ab4
	ctx.r[9].s64 = ctx.r[11].s64 + 23220;
	// 8291237C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82912380: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82912384: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82912388: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8291238C: 48000008  b 0x82912394
	pc = 0x82912394; continue 'dispatch;
	// 82912390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82912394: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8291239C: 409A0044  bne cr6, 0x829123e0
	if !ctx.cr[6].eq {
	pc = 0x829123E0; continue 'dispatch;
	}
	// 829123A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 829123A4: 419A001C  beq cr6, 0x829123c0
	if ctx.cr[6].eq {
	pc = 0x829123C0; continue 'dispatch;
	}
	// 829123A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829123AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 829123B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829123B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829123B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829123BC: 4E800421  bctrl
	ctx.lr = 0x829123C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829123C0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 829123C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 829123C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829123CC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 829123D0: 816BE330  lwz r11, -0x1cd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7376 as u32) ) } as u64;
	// 829123D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 829123D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 829123DC: 4B9ADC25  bl 0x822c0000
	ctx.lr = 0x829123E0;
	sub_822C0000(ctx, base);
	// 829123E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829123E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829123E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829123EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829123F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829123F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829123F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912400 size=196
    let mut pc: u32 = 0x82912400;
    'dispatch: loop {
        match pc {
            0x82912400 => {
    //   block [0x82912400..0x829124C4)
	// 82912400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291240C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82912410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82912418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8291241C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82912420: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82912424: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912428: 4B9AE511  bl 0x822c0938
	ctx.lr = 0x8291242C;
	sub_822C0938(ctx, base);
	// 8291242C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82912430: 41820028  beq 0x82912458
	if ctx.cr[0].eq {
	pc = 0x82912458; continue 'dispatch;
	}
	// 82912434: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82912438: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8291243C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82912440: 392B5AC8  addi r9, r11, 0x5ac8
	ctx.r[9].s64 = ctx.r[11].s64 + 23240;
	// 82912444: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82912448: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8291244C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82912450: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82912454: 48000008  b 0x8291245c
	pc = 0x8291245C; continue 'dispatch;
	// 82912458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8291245C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82912464: 409A0044  bne cr6, 0x829124a8
	if !ctx.cr[6].eq {
	pc = 0x829124A8; continue 'dispatch;
	}
	// 82912468: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8291246C: 419A001C  beq cr6, 0x82912488
	if ctx.cr[6].eq {
	pc = 0x82912488; continue 'dispatch;
	}
	// 82912470: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912474: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82912478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291247C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82912484: 4E800421  bctrl
	ctx.lr = 0x82912488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82912488: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8291248C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82912490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82912494: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82912498: 816BE330  lwz r11, -0x1cd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7376 as u32) ) } as u64;
	// 8291249C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 829124A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 829124A4: 4B9ADB5D  bl 0x822c0000
	ctx.lr = 0x829124A8;
	sub_822C0000(ctx, base);
	// 829124A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829124AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829124B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829124B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829124B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829124BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829124C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829124C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829124C8 size=196
    let mut pc: u32 = 0x829124C8;
    'dispatch: loop {
        match pc {
            0x829124C8 => {
    //   block [0x829124C8..0x8291258C)
	// 829124C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829124CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829124D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829124D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829124D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829124DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829124E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829124E4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 829124E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829124EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829124F0: 4B9AE449  bl 0x822c0938
	ctx.lr = 0x829124F4;
	sub_822C0938(ctx, base);
	// 829124F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829124F8: 41820028  beq 0x82912520
	if ctx.cr[0].eq {
	pc = 0x82912520; continue 'dispatch;
	}
	// 829124FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82912500: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82912504: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82912508: 392B5ADC  addi r9, r11, 0x5adc
	ctx.r[9].s64 = ctx.r[11].s64 + 23260;
	// 8291250C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82912510: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82912514: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82912518: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8291251C: 48000008  b 0x82912524
	pc = 0x82912524; continue 'dispatch;
	// 82912520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82912524: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912528: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8291252C: 409A0044  bne cr6, 0x82912570
	if !ctx.cr[6].eq {
	pc = 0x82912570; continue 'dispatch;
	}
	// 82912530: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82912534: 419A001C  beq cr6, 0x82912550
	if ctx.cr[6].eq {
	pc = 0x82912550; continue 'dispatch;
	}
	// 82912538: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291253C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82912540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82912544: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8291254C: 4E800421  bctrl
	ctx.lr = 0x82912550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82912550: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82912554: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82912558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8291255C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82912560: 816BE330  lwz r11, -0x1cd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7376 as u32) ) } as u64;
	// 82912564: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82912568: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8291256C: 4B9ADA95  bl 0x822c0000
	ctx.lr = 0x82912570;
	sub_822C0000(ctx, base);
	// 82912570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82912574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82912578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291257C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82912580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82912584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82912588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912590 size=196
    let mut pc: u32 = 0x82912590;
    'dispatch: loop {
        match pc {
            0x82912590 => {
    //   block [0x82912590..0x82912654)
	// 82912590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291259C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829125A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829125A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829125A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829125AC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 829125B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829125B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829125B8: 4B9AE381  bl 0x822c0938
	ctx.lr = 0x829125BC;
	sub_822C0938(ctx, base);
	// 829125BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829125C0: 41820028  beq 0x829125e8
	if ctx.cr[0].eq {
	pc = 0x829125E8; continue 'dispatch;
	}
	// 829125C4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829125C8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 829125CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 829125D0: 392B5AF0  addi r9, r11, 0x5af0
	ctx.r[9].s64 = ctx.r[11].s64 + 23280;
	// 829125D4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 829125D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 829125DC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 829125E0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 829125E4: 48000008  b 0x829125ec
	pc = 0x829125EC; continue 'dispatch;
	// 829125E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829125EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829125F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829125F4: 409A0044  bne cr6, 0x82912638
	if !ctx.cr[6].eq {
	pc = 0x82912638; continue 'dispatch;
	}
	// 829125F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 829125FC: 419A001C  beq cr6, 0x82912618
	if ctx.cr[6].eq {
	pc = 0x82912618; continue 'dispatch;
	}
	// 82912600: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912604: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82912608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291260C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82912614: 4E800421  bctrl
	ctx.lr = 0x82912618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82912618: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8291261C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82912620: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82912624: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82912628: 816BE330  lwz r11, -0x1cd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7376 as u32) ) } as u64;
	// 8291262C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82912630: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82912634: 4B9AD9CD  bl 0x822c0000
	ctx.lr = 0x82912638;
	sub_822C0000(ctx, base);
	// 82912638: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8291263C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82912640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82912644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82912648: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291264C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82912650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82912658 size=400
    let mut pc: u32 = 0x82912658;
    'dispatch: loop {
        match pc {
            0x82912658 => {
    //   block [0x82912658..0x829127E8)
	// 82912658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291265C: 48895B11  bl 0x831a816c
	ctx.lr = 0x82912660;
	sub_831A8130(ctx, base);
	// 82912660: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82912664: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291266C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82912670: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82912674: 4BED9B5D  bl 0x827ec1d0
	ctx.lr = 0x82912678;
	sub_827EC1D0(ctx, base);
	// 82912678: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291267C: C3FD0000  lfs f31, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82912680: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82912688: 4E800421  bctrl
	ctx.lr = 0x8291268C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8291268C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82912690: 482A2E09  bl 0x82bb5498
	ctx.lr = 0x82912694;
	sub_82BB5498(ctx, base);
	// 82912694: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82912698: 896401E4  lbz r11, 0x1e4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(484 as u32) ) } as u64;
	// 8291269C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829126A0: 4082003C  bne 0x829126dc
	if !ctx.cr[0].eq {
	pc = 0x829126DC; continue 'dispatch;
	}
	// 829126A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829126A8: 4BED9F21  bl 0x827ec5c8
	ctx.lr = 0x829126AC;
	sub_827EC5C8(ctx, base);
	// 829126AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829126B0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 829126B4: 484EC12D  bl 0x82dfe7e0
	ctx.lr = 0x829126B8;
	sub_82DFE7E0(ctx, base);
	// 829126B8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 829126BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829126C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829126C4: 419A000C  beq cr6, 0x829126d0
	if ctx.cr[6].eq {
	pc = 0x829126D0; continue 'dispatch;
	}
	// 829126C8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 829126CC: 4B9AE1C5  bl 0x822c0890
	ctx.lr = 0x829126D0;
	sub_822C0890(ctx, base);
	// 829126D0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829126D4: 41820108  beq 0x829127dc
	if ctx.cr[0].eq {
	pc = 0x829127DC; continue 'dispatch;
	}
	// 829126D8: 480000C4  b 0x8291279c
	pc = 0x8291279C; continue 'dispatch;
	// 829126DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 829126E0: 4BED9A81  bl 0x827ec160
	ctx.lr = 0x829126E4;
	sub_827EC160(ctx, base);
	// 829126E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829126E8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 829126EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 829126F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829126F4: 4E800421  bctrl
	ctx.lr = 0x829126F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829126F8: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 829126FC: 4BED9A65  bl 0x827ec160
	ctx.lr = 0x82912700;
	sub_827EC160(ctx, base);
	// 82912700: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912704: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82912708: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8291270C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82912710: 4E800421  bctrl
	ctx.lr = 0x82912714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82912714: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82912718: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8291271C: 4856977D  bl 0x82e7be98
	ctx.lr = 0x82912720;
	sub_82E7BE98(ctx, base);
	// 82912720: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82912724: C0610068  lfs f3, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82912728: C0410064  lfs f2, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8291272C: C0210060  lfs f1, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82912730: 4856A051  bl 0x82e7c780
	ctx.lr = 0x82912734;
	sub_82E7C780(ctx, base);
	// 82912734: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82912738: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8291273C: 4BED9E8D  bl 0x827ec5c8
	ctx.lr = 0x82912740;
	sub_827EC5C8(ctx, base);
	// 82912740: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912744: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82912748: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8291274C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82912750: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82912754: 806B0084  lwz r3, 0x84(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82912758: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8291275C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82912760: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82912764: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82912768: 13E83C07  vcmpneb. (lvlx128) v31, v8, v7
	tmp.u32 = ctx.r[8].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8291276C: 13C93407  vcmpneb. (lvlx128) v30, v9, v6
	tmp.u32 = ctx.r[9].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82912770: 13AA2C07  vcmpneb. (lvlx128) v29, v10, v5
	tmp.u32 = ctx.r[10].u32 + ctx.r[5].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82912774: 13802407  vcmpneb. (lvlx128) v28, v0, v4
	tmp.u32 = ctx.r[4].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829127E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829127E8 size=300
    let mut pc: u32 = 0x829127E8;
    'dispatch: loop {
        match pc {
            0x829127E8 => {
    //   block [0x829127E8..0x82912914)
	// 829127E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829127EC: 48895981  bl 0x831a816c
	ctx.lr = 0x829127F0;
	sub_831A8130(ctx, base);
	// 829127F0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829127F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829127F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 829127FC: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82912800: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82912804: 40980014  bge cr6, 0x82912818
	if !ctx.cr[6].lt {
	pc = 0x82912818; continue 'dispatch;
	}
	// 82912808: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8291280C: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82912810: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82912814: 480000F0  b 0x82912904
	pc = 0x82912904; continue 'dispatch;
	// 82912818: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 8291281C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82912820: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82912824: 409A0054  bne cr6, 0x82912878
	if !ctx.cr[6].eq {
	pc = 0x82912878; continue 'dispatch;
	}
	// 82912828: 4BED99A9  bl 0x827ec1d0
	ctx.lr = 0x8291282C;
	sub_827EC1D0(ctx, base);
	// 8291282C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912830: 83FF00E0  lwz r31, 0xe0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82912834: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8291283C: 4E800421  bctrl
	ctx.lr = 0x82912840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82912840: 481D1051  bl 0x82ae3890
	ctx.lr = 0x82912844;
	sub_82AE3890(ctx, base);
	// 82912844: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82912848: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8291284C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82912850: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82912854: 40820014  bne 0x82912868
	if !ctx.cr[0].eq {
	pc = 0x82912868; continue 'dispatch;
	}
	// 82912858: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8291285C: 1D5F0030  mulli r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 * 48;
	// 82912860: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82912864: 4800000C  b 0x82912870
	pc = 0x82912870; continue 'dispatch;
	// 82912868: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8291286C: 4869BF0D  bl 0x82fae778
	ctx.lr = 0x82912870;
	sub_82FAE778(ctx, base);
	// 82912870: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82912874: 48000090  b 0x82912904
	pc = 0x82912904; continue 'dispatch;
	// 82912878: 4BED9959  bl 0x827ec1d0
	ctx.lr = 0x8291287C;
	sub_827EC1D0(ctx, base);
	// 8291287C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912880: 83DF00E0  lwz r30, 0xe0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82912884: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8291288C: 4E800421  bctrl
	ctx.lr = 0x82912890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82912890: 481D1001  bl 0x82ae3890
	ctx.lr = 0x82912894;
	sub_82AE3890(ctx, base);
	// 82912894: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82912898: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8291289C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 829128A0: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829128A4: 40820014  bne 0x829128b8
	if !ctx.cr[0].eq {
	pc = 0x829128B8; continue 'dispatch;
	}
	// 829128A8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 829128AC: 1D5E0030  mulli r10, r30, 0x30
	ctx.r[10].s64 = ctx.r[30].s64 * 48;
	// 829128B0: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 829128B4: 48000010  b 0x829128c4
	pc = 0x829128C4; continue 'dispatch;
	// 829128B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829128BC: 4869BEBD  bl 0x82fae778
	ctx.lr = 0x829128C0;
	sub_82FAE778(ctx, base);
	// 829128C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 829128C4: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 829128C8: 48100DC1  bl 0x82a13688
	ctx.lr = 0x829128CC;
	sub_82A13688(ctx, base);
	// 829128CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829128D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 829128D4: 485695C5  bl 0x82e7be98
	ctx.lr = 0x829128D8;
	sub_82E7BE98(ctx, base);
	// 829128D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 829128DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 829128E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829128E4: 485693E5  bl 0x82e7bcc8
	ctx.lr = 0x829128E8;
	sub_82E7BCC8(ctx, base);
	// 829128E8: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 829128EC: 48100D95  bl 0x82a13680
	ctx.lr = 0x829128F0;
	sub_82A13680(ctx, base);
	// 829128F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 829128F4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 829128F8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829128FC: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912918 size=56
    let mut pc: u32 = 0x82912918;
    'dispatch: loop {
        match pc {
            0x82912918 => {
    //   block [0x82912918..0x82912950)
	// 82912918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291291C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82912924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912928: 38630160  addi r3, r3, 0x160
	ctx.r[3].s64 = ctx.r[3].s64 + 352;
	// 8291292C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82912930: 4BB5E799  bl 0x824710c8
	ctx.lr = 0x82912934;
	sub_824710C8(ctx, base);
	// 82912934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82912938: 4B9B6381  bl 0x822c8cb8
	ctx.lr = 0x8291293C;
	sub_822C8CB8(ctx, base);
	// 8291293C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82912940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82912944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82912948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8291294C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912950 size=112
    let mut pc: u32 = 0x82912950;
    'dispatch: loop {
        match pc {
            0x82912950 => {
    //   block [0x82912950..0x829129C0)
	// 82912950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291295C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82912960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912964: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82912968: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291296C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82912970: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82912974: 4BFFF9C5  bl 0x82912338
	ctx.lr = 0x82912978;
	sub_82912338(ctx, base);
	// 82912978: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8291297C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82912980: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82912984: 4B9AD67D  bl 0x822c0000
	ctx.lr = 0x82912988;
	sub_822C0000(ctx, base);
	// 82912988: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8291298C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82912990: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912994: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82912998: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8291299C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 829129A0: 419A0008  beq cr6, 0x829129a8
	if ctx.cr[6].eq {
	pc = 0x829129A8; continue 'dispatch;
	}
	// 829129A4: 4B9ADEED  bl 0x822c0890
	ctx.lr = 0x829129A8;
	sub_822C0890(ctx, base);
	// 829129A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829129AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829129B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829129B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829129B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829129BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829129C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829129C0 size=112
    let mut pc: u32 = 0x829129C0;
    'dispatch: loop {
        match pc {
            0x829129C0 => {
    //   block [0x829129C0..0x82912A30)
	// 829129C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829129C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829129C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829129CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829129D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829129D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829129D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829129DC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 829129E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829129E4: 4BFFFA1D  bl 0x82912400
	ctx.lr = 0x829129E8;
	sub_82912400(ctx, base);
	// 829129E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 829129EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 829129F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829129F4: 4B9AD60D  bl 0x822c0000
	ctx.lr = 0x829129F8;
	sub_822C0000(ctx, base);
	// 829129F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829129FC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82912A00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82912A04: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82912A08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82912A0C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82912A10: 419A0008  beq cr6, 0x82912a18
	if ctx.cr[6].eq {
	pc = 0x82912A18; continue 'dispatch;
	}
	// 82912A14: 4B9ADE7D  bl 0x822c0890
	ctx.lr = 0x82912A18;
	sub_822C0890(ctx, base);
	// 82912A18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82912A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82912A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82912A24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82912A28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82912A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912A30 size=120
    let mut pc: u32 = 0x82912A30;
    'dispatch: loop {
        match pc {
            0x82912A30 => {
    //   block [0x82912A30..0x82912AA8)
	// 82912A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912A34: 48895739  bl 0x831a816c
	ctx.lr = 0x82912A38;
	sub_831A8130(ctx, base);
	// 82912A38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912A3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82912A40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82912A44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82912A48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82912A4C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82912A50: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82912A54: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82912A58: 484DF991  bl 0x82df23e8
	ctx.lr = 0x82912A5C;
	sub_82DF23E8(ctx, base);
	// 82912A5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82912A60: 41820014  beq 0x82912a74
	if ctx.cr[0].eq {
	pc = 0x82912A74; continue 'dispatch;
	}
	// 82912A64: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82912A68: 48240329  bl 0x82b52d90
	ctx.lr = 0x82912A6C;
	sub_82B52D90(ctx, base);
	// 82912A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82912A70: 48000008  b 0x82912a78
	pc = 0x82912A78; continue 'dispatch;
	// 82912A74: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82912A78: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82912A7C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82912A80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82912A84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82912A88: 4BFFFA41  bl 0x829124c8
	ctx.lr = 0x82912A8C;
	sub_829124C8(ctx, base);
	// 82912A8C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82912A90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82912A94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82912A98: 4B9AD569  bl 0x822c0000
	ctx.lr = 0x82912A9C;
	sub_822C0000(ctx, base);
	// 82912A9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82912AA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82912AA4: 48895718  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912AA8 size=112
    let mut pc: u32 = 0x82912AA8;
    'dispatch: loop {
        match pc {
            0x82912AA8 => {
    //   block [0x82912AA8..0x82912B18)
	// 82912AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912AAC: 488956C1  bl 0x831a816c
	ctx.lr = 0x82912AB0;
	sub_831A8130(ctx, base);
	// 82912AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912AB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82912AB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82912ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82912AC0: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82912AC4: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82912AC8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82912ACC: 484DF91D  bl 0x82df23e8
	ctx.lr = 0x82912AD0;
	sub_82DF23E8(ctx, base);
	// 82912AD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82912AD4: 41820010  beq 0x82912ae4
	if ctx.cr[0].eq {
	pc = 0x82912AE4; continue 'dispatch;
	}
	// 82912AD8: 48240D91  bl 0x82b53868
	ctx.lr = 0x82912ADC;
	sub_82B53868(ctx, base);
	// 82912ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82912AE0: 48000008  b 0x82912ae8
	pc = 0x82912AE8; continue 'dispatch;
	// 82912AE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82912AE8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82912AEC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82912AF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82912AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82912AF8: 4BFFFA99  bl 0x82912590
	ctx.lr = 0x82912AFC;
	sub_82912590(ctx, base);
	// 82912AFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82912B00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82912B04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82912B08: 4B9AD4F9  bl 0x822c0000
	ctx.lr = 0x82912B0C;
	sub_822C0000(ctx, base);
	// 82912B0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82912B10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82912B14: 488956A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82912B18 size=1052
    let mut pc: u32 = 0x82912B18;
    'dispatch: loop {
        match pc {
            0x82912B18 => {
    //   block [0x82912B18..0x82912F34)
	// 82912B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912B1C: 4889564D  bl 0x831a8168
	ctx.lr = 0x82912B20;
	sub_831A8130(ctx, base);
	// 82912B20: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82912B24: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82912F38 size=8
    let mut pc: u32 = 0x82912F38;
    'dispatch: loop {
        match pc {
            0x82912F38 => {
    //   block [0x82912F38..0x82912F40)
	// 82912F38: 38630150  addi r3, r3, 0x150
	ctx.r[3].s64 = ctx.r[3].s64 + 336;
	// 82912F3C: 4BA61254  b 0x82374190
	sub_82374190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912F40 size=56
    let mut pc: u32 = 0x82912F40;
    'dispatch: loop {
        match pc {
            0x82912F40 => {
    //   block [0x82912F40..0x82912F78)
	// 82912F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912F48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82912F4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912F50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82912F54: 387F0144  addi r3, r31, 0x144
	ctx.r[3].s64 = ctx.r[31].s64 + 324;
	// 82912F58: 4BA61239  bl 0x82374190
	ctx.lr = 0x82912F5C;
	sub_82374190(ctx, base);
	// 82912F5C: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 82912F60: 4BA61231  bl 0x82374190
	ctx.lr = 0x82912F64;
	sub_82374190(ctx, base);
	// 82912F64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82912F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82912F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82912F70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82912F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82912F78 size=116
    let mut pc: u32 = 0x82912F78;
    'dispatch: loop {
        match pc {
            0x82912F78 => {
    //   block [0x82912F78..0x82912FEC)
	// 82912F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912F7C: 488951F1  bl 0x831a816c
	ctx.lr = 0x82912F80;
	sub_831A8130(ctx, base);
	// 82912F80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82912F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82912F88: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82912F8C: 3BDF0160  addi r30, r31, 0x160
	ctx.r[30].s64 = ctx.r[31].s64 + 352;
	// 82912F90: 817F0160  lwz r11, 0x160(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 82912F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82912F98: 419A004C  beq cr6, 0x82912fe4
	if ctx.cr[6].eq {
	pc = 0x82912FE4; continue 'dispatch;
	}
	// 82912F9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82912FA0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82912FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82912FA8: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82912FAC: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82912FB0: 387F0180  addi r3, r31, 0x180
	ctx.r[3].s64 = ctx.r[31].s64 + 384;
	// 82912FB4: 99210060  stb r9, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u8 ) };
	// 82912FB8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82912FBC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82912FC0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82912FC4: 48001EC5  bl 0x82914e88
	ctx.lr = 0x82912FC8;
	sub_82914E88(ctx, base);
	// 82912FC8: C01F0134  lfs f0, 0x134(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82912FCC: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82912FD0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82912FD4: 9BA10060  stb r29, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u8 ) };
	// 82912FD8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82912FDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82912FE0: 4BC9E8E9  bl 0x825b18c8
	ctx.lr = 0x82912FE4;
	sub_825B18C8(ctx, base);
	// 82912FE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82912FE8: 488951D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82912FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82912FF0 size=104
    let mut pc: u32 = 0x82912FF0;
    'dispatch: loop {
        match pc {
            0x82912FF0 => {
    //   block [0x82912FF0..0x82913058)
	// 82912FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82912FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82912FF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82912FFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82913000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82913004: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82913008: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8291300C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82913010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82913014: 4BFFF1CD  bl 0x829121e0
	ctx.lr = 0x82913018;
	sub_829121E0(ctx, base);
	// 82913018: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291301C: 41820020  beq 0x8291303c
	if ctx.cr[0].eq {
	pc = 0x8291303C; continue 'dispatch;
	}
	// 82913020: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82913024: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82913028: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8291302C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82913030: 4BFFFF49  bl 0x82912f78
	ctx.lr = 0x82913034;
	sub_82912F78(ctx, base);
	// 82913034: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82913038: 48000008  b 0x82913040
	pc = 0x82913040; continue 'dispatch;
	// 8291303C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82913040: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82913044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82913048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291304C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82913050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82913054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82913058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82913058 size=96
    let mut pc: u32 = 0x82913058;
    'dispatch: loop {
        match pc {
            0x82913058 => {
    //   block [0x82913058..0x829130B8)
	// 82913058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291305C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82913060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82913064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82913068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291306C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82913070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82913074: 4BFFFAA5  bl 0x82912b18
	ctx.lr = 0x82913078;
	sub_82912B18(ctx, base);
	// 82913078: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291307C: 41820020  beq 0x8291309c
	if ctx.cr[0].eq {
	pc = 0x8291309C; continue 'dispatch;
	}
	// 82913080: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82913084: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82913088: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8291308C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82913090: 4BFFFEE9  bl 0x82912f78
	ctx.lr = 0x82913094;
	sub_82912F78(ctx, base);
	// 82913094: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82913098: 48000008  b 0x829130a0
	pc = 0x829130A0; continue 'dispatch;
	// 8291309C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 829130A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829130A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829130A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829130AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829130B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829130B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829130B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829130B8 size=336
    let mut pc: u32 = 0x829130B8;
    'dispatch: loop {
        match pc {
            0x829130B8 => {
    //   block [0x829130B8..0x82913208)
	// 829130B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829130BC: 488950B1  bl 0x831a816c
	ctx.lr = 0x829130C0;
	sub_831A8130(ctx, base);
	// 829130C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829130C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829130C8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829130CC: 396B5B70  addi r11, r11, 0x5b70
	ctx.r[11].s64 = ctx.r[11].s64 + 23408;
	// 829130D0: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 829130D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829130D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 829130DC: 419A001C  beq cr6, 0x829130f8
	if ctx.cr[6].eq {
	pc = 0x829130F8; continue 'dispatch;
	}
	// 829130E0: 809F0090  lwz r4, 0x90(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 829130E4: 807F00A0  lwz r3, 0xa0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 829130E8: 484FF459  bl 0x82e12540
	ctx.lr = 0x829130EC;
	sub_82E12540(ctx, base);
	// 829130EC: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 829130F0: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 829130F4: 484FF44D  bl 0x82e12540
	ctx.lr = 0x829130F8;
	sub_82E12540(ctx, base);
	// 829130F8: 387F0180  addi r3, r31, 0x180
	ctx.r[3].s64 = ctx.r[31].s64 + 384;
	// 829130FC: 4B9ACF05  bl 0x822c0000
	ctx.lr = 0x82913100;
	sub_822C0000(ctx, base);
	// 82913100: 387F0160  addi r3, r31, 0x160
	ctx.r[3].s64 = ctx.r[31].s64 + 352;
	// 82913104: 4B9B5BB5  bl 0x822c8cb8
	ctx.lr = 0x82913108;
	sub_822C8CB8(ctx, base);
	// 82913108: 387F0150  addi r3, r31, 0x150
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	// 8291310C: 4BA0DDE5  bl 0x82320ef0
	ctx.lr = 0x82913110;
	sub_82320EF0(ctx, base);
	// 82913110: 387F0144  addi r3, r31, 0x144
	ctx.r[3].s64 = ctx.r[31].s64 + 324;
	// 82913114: 4BA0DDDD  bl 0x82320ef0
	ctx.lr = 0x82913118;
	sub_82320EF0(ctx, base);
	// 82913118: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 8291311C: 4BA0DDD5  bl 0x82320ef0
	ctx.lr = 0x82913120;
	sub_82320EF0(ctx, base);
	// 82913120: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82913124: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913128: 419A0008  beq cr6, 0x82913130
	if ctx.cr[6].eq {
	pc = 0x82913130; continue 'dispatch;
	}
	// 8291312C: 4B9AD765  bl 0x822c0890
	ctx.lr = 0x82913130;
	sub_822C0890(ctx, base);
	// 82913130: 807F0124  lwz r3, 0x124(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 82913134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913138: 419A0008  beq cr6, 0x82913140
	if ctx.cr[6].eq {
	pc = 0x82913140; continue 'dispatch;
	}
	// 8291313C: 4B9AD755  bl 0x822c0890
	ctx.lr = 0x82913140;
	sub_822C0890(ctx, base);
	// 82913140: 807F011C  lwz r3, 0x11c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 82913144: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913148: 419A0008  beq cr6, 0x82913150
	if ctx.cr[6].eq {
	pc = 0x82913150; continue 'dispatch;
	}
	// 8291314C: 4B9AD745  bl 0x822c0890
	ctx.lr = 0x82913150;
	sub_822C0890(ctx, base);
	// 82913150: 807F0114  lwz r3, 0x114(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 82913154: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913158: 419A0008  beq cr6, 0x82913160
	if ctx.cr[6].eq {
	pc = 0x82913160; continue 'dispatch;
	}
	// 8291315C: 4B9AD735  bl 0x822c0890
	ctx.lr = 0x82913160;
	sub_822C0890(ctx, base);
	// 82913160: 807F010C  lwz r3, 0x10c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 82913164: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913168: 419A0008  beq cr6, 0x82913170
	if ctx.cr[6].eq {
	pc = 0x82913170; continue 'dispatch;
	}
	// 8291316C: 4B9AD725  bl 0x822c0890
	ctx.lr = 0x82913170;
	sub_822C0890(ctx, base);
	// 82913170: 807F0104  lwz r3, 0x104(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 82913174: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913178: 419A0008  beq cr6, 0x82913180
	if ctx.cr[6].eq {
	pc = 0x82913180; continue 'dispatch;
	}
	// 8291317C: 4B9AD715  bl 0x822c0890
	ctx.lr = 0x82913180;
	sub_822C0890(ctx, base);
	// 82913180: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82913184: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913188: 419A0008  beq cr6, 0x82913190
	if ctx.cr[6].eq {
	pc = 0x82913190; continue 'dispatch;
	}
	// 8291318C: 4B9AD705  bl 0x822c0890
	ctx.lr = 0x82913190;
	sub_822C0890(ctx, base);
	// 82913190: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82913194: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82913198: 419A0008  beq cr6, 0x829131a0
	if ctx.cr[6].eq {
	pc = 0x829131A0; continue 'dispatch;
	}
	// 8291319C: 4B9AD6F5  bl 0x822c0890
	ctx.lr = 0x829131A0;
	sub_822C0890(ctx, base);
	// 829131A0: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 829131A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829131A8: 419A0008  beq cr6, 0x829131b0
	if ctx.cr[6].eq {
	pc = 0x829131B0; continue 'dispatch;
	}
	// 829131AC: 4B9AD6E5  bl 0x822c0890
	ctx.lr = 0x829131B0;
	sub_822C0890(ctx, base);
	// 829131B0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 829131B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829131B8: 419A0008  beq cr6, 0x829131c0
	if ctx.cr[6].eq {
	pc = 0x829131C0; continue 'dispatch;
	}
	// 829131BC: 4B9AD6D5  bl 0x822c0890
	ctx.lr = 0x829131C0;
	sub_822C0890(ctx, base);
	// 829131C0: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 829131C4: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 829131C8: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 829131CC: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 829131D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829131D4: 484E0255  bl 0x82df3428
	ctx.lr = 0x829131D8;
	sub_82DF3428(ctx, base);
	// 829131D8: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 829131DC: 4080FFF0  bge 0x829131cc
	if !ctx.cr[0].lt {
	pc = 0x829131CC; continue 'dispatch;
	}
	// 829131E0: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 829131E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829131E8: 419A0008  beq cr6, 0x829131f0
	if ctx.cr[6].eq {
	pc = 0x829131F0; continue 'dispatch;
	}
	// 829131EC: 4B9AD6A5  bl 0x822c0890
	ctx.lr = 0x829131F0;
	sub_822C0890(ctx, base);
	// 829131F0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 829131F4: 4BB56E0D  bl 0x8246a000
	ctx.lr = 0x829131F8;
	sub_8246A000(ctx, base);
	// 829131F8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 829131FC: 4BB56E05  bl 0x8246a000
	ctx.lr = 0x82913200;
	sub_8246A000(ctx, base);
	// 82913200: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82913204: 48894FB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82913208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82913208 size=1976
    let mut pc: u32 = 0x82913208;
    'dispatch: loop {
        match pc {
            0x82913208 => {
    //   block [0x82913208..0x829139C0)
	// 82913208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291320C: 48894F3D  bl 0x831a8148
	ctx.lr = 0x82913210;
	sub_831A8130(ctx, base);
	// 82913210: 3981FF98  addi r12, r1, -0x68
	ctx.r[12].s64 = ctx.r[1].s64 + -104;
	// 82913214: 48895861  bl 0x831a8a74
	ctx.lr = 0x82913218;
	sub_831A8A40(ctx, base);
	// 82913218: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829139C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829139C0 size=568
    let mut pc: u32 = 0x829139C0;
    'dispatch: loop {
        match pc {
            0x829139C0 => {
    //   block [0x829139C0..0x82913BF8)
	// 829139C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829139C4: 48894799  bl 0x831a815c
	ctx.lr = 0x829139C8;
	sub_831A8130(ctx, base);
	// 829139C8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 829139CC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829139D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829139D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829139D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 829139DC: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 829139E0: 396B5B70  addi r11, r11, 0x5b70
	ctx.r[11].s64 = ctx.r[11].s64 + 23408;
	// 829139E4: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 829139E8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 829139EC: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 829139F0: 3B7F0040  addi r27, r31, 0x40
	ctx.r[27].s64 = ctx.r[31].s64 + 64;
	// 829139F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829139F8: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 829139FC: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82913A00: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82913A04: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82913A08: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82913A0C: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82913A10: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82913A14: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82913A18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82913A1C: 4BFFE74D  bl 0x82912168
	ctx.lr = 0x82913A20;
	sub_82912168(ctx, base);
	// 82913A20: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82913A24: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82913A28: 4080FFF0  bge 0x82913a18
	if !ctx.cr[0].lt {
	pc = 0x82913A18; continue 'dispatch;
	}
	// 82913A2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82913A30: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82913A34: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82913A38: C3EB08A8  lfs f31, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82913A3C: D3FF0058  stfs f31, 0x58(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82913A40: 4823F991  bl 0x82b533d0
	ctx.lr = 0x82913A44;
	sub_82B533D0(ctx, base);
	// 82913A44: 93DF0090  stw r30, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82913A48: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82913A4C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82913A50: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 82913A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82913A58: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 82913A5C: 3B8B5B18  addi r28, r11, 0x5b18
	ctx.r[28].s64 = ctx.r[11].s64 + 23320;
	// 82913A60: 38A0007B  li r5, 0x7b
	ctx.r[5].s64 = 123;
	// 82913A64: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82913A68: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82913A6C: 484DE97D  bl 0x82df23e8
	ctx.lr = 0x82913A70;
	sub_82DF23E8(ctx, base);
	// 82913A70: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82913A74: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82913A78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82913A7C: 3B6B6910  addi r27, r11, 0x6910
	ctx.r[27].s64 = ctx.r[11].s64 + 26896;
	// 82913A80: 3B2ABA80  addi r25, r10, -0x4580
	ctx.r[25].s64 = ctx.r[10].s64 + -17792;
	// 82913A84: 41820018  beq 0x82913a9c
	if ctx.cr[0].eq {
	pc = 0x82913A9C; continue 'dispatch;
	}
	// 82913A88: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82913A8C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82913A90: 4BC77291  bl 0x8258ad20
	ctx.lr = 0x82913A94;
	sub_8258AD20(ctx, base);
	// 82913A94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82913A98: 48000008  b 0x82913aa0
	pc = 0x82913AA0; continue 'dispatch;
	// 82913A9C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82913AA0: 93BF00A0  stw r29, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[29].u32 ) };
	// 82913AA4: 397F00A0  addi r11, r31, 0xa0
	ctx.r[11].s64 = ctx.r[31].s64 + 160;
	// 82913AA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82913AAC: 3B4B0004  addi r26, r11, 4
	ctx.r[26].s64 = ctx.r[11].s64 + 4;
	// 82913AB0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82913AB4: 4B9F2EDD  bl 0x82306990
	ctx.lr = 0x82913AB8;
	sub_82306990(ctx, base);
	// 82913AB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82913ABC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82913AC0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82913AC4: 4B9AC53D  bl 0x822c0000
	ctx.lr = 0x82913AC8;
	sub_822C0000(ctx, base);
	// 82913AC8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82913ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82913AD0: 38A0007C  li r5, 0x7c
	ctx.r[5].s64 = 124;
	// 82913AD4: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82913AD8: 484DE911  bl 0x82df23e8
	ctx.lr = 0x82913ADC;
	sub_82DF23E8(ctx, base);
	// 82913ADC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82913AE0: 41820018  beq 0x82913af8
	if ctx.cr[0].eq {
	pc = 0x82913AF8; continue 'dispatch;
	}
	// 82913AE4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82913AE8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82913AEC: 4BC77235  bl 0x8258ad20
	ctx.lr = 0x82913AF0;
	sub_8258AD20(ctx, base);
	// 82913AF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82913AF4: 48000008  b 0x82913afc
	pc = 0x82913AFC; continue 'dispatch;
	// 82913AF8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82913AFC: 93BF00A8  stw r29, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 82913B00: 397F00A8  addi r11, r31, 0xa8
	ctx.r[11].s64 = ctx.r[31].s64 + 168;
	// 82913B04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82913B08: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 82913B0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82913B10: 4B9F2E81  bl 0x82306990
	ctx.lr = 0x82913B14;
	sub_82306990(ctx, base);
	// 82913B14: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82913B18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82913B1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82913B20: 4B9AC4E1  bl 0x822c0000
	ctx.lr = 0x82913B24;
	sub_822C0000(ctx, base);
	// 82913B24: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 82913B28: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82913B2C: 392000D0  li r9, 0xd0
	ctx.r[9].s64 = 208;
	// 82913B30: 915F00B0  stw r10, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 82913B34: 13E0D8C7  vcmpequd (lvx128) v31, v0, v27
	tmp.u32 = ctx.r[27].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82913B38: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82913BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82913BF8 size=76
    let mut pc: u32 = 0x82913BF8;
    'dispatch: loop {
        match pc {
            0x82913BF8 => {
    //   block [0x82913BF8..0x82913C44)
	// 82913BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82913BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82913C00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82913C04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82913C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82913C0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82913C10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82913C14: 4BFFF4A5  bl 0x829130b8
	ctx.lr = 0x82913C18;
	sub_829130B8(ctx, base);
	// 82913C18: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82913C1C: 4182000C  beq 0x82913c28
	if ctx.cr[0].eq {
	pc = 0x82913C28; continue 'dispatch;
	}
	// 82913C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82913C24: 4B9AC645  bl 0x822c0268
	ctx.lr = 0x82913C28;
	sub_822C0268(ctx, base);
	// 82913C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82913C2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82913C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82913C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82913C38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82913C3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82913C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82913C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82913C48 size=2548
    let mut pc: u32 = 0x82913C48;
    'dispatch: loop {
        match pc {
            0x82913C48 => {
    //   block [0x82913C48..0x8291463C)
	// 82913C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82913C4C: 488944E5  bl 0x831a8130
	ctx.lr = 0x82913C50;
	sub_831A8130(ctx, base);
	// 82913C50: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82913C54: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82913C58: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82913C5C: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82913C60: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82913C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82913C68: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82913C6C: 39C00000  li r14, 0
	ctx.r[14].s64 = 0;
	// 82913C70: 3A9F0060  addi r20, r31, 0x60
	ctx.r[20].s64 = ctx.r[31].s64 + 96;
	// 82913C74: C3EA08A8  lfs f31, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82913C78: 7DDE7378  mr r30, r14
	ctx.r[30].u64 = ctx.r[14].u64;
	// 82913C7C: D3FF0058  stfs f31, 0x58(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82913C80: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82913C84: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82913C88: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82913C8C: 81470004  lwz r10, 4(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82913C90: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82913C94: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82913C98: 13E758C7  vcmpequd (lvx128) v31, v7, v11
	tmp.u32 = ctx.r[7].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82913C9C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82914640 size=356
    let mut pc: u32 = 0x82914640;
    'dispatch: loop {
        match pc {
            0x82914640 => {
    //   block [0x82914640..0x829147A4)
	// 82914640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82914644: 48893B21  bl 0x831a8164
	ctx.lr = 0x82914648;
	sub_831A8130(ctx, base);
	// 82914648: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291464C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82914650: 9081010C  stw r4, 0x10c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), ctx.r[4].u32 ) };
	// 82914654: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82914658: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8291465C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82914660: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82914664: 816AB1CC  lwz r11, -0x4e34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20020 as u32) ) } as u64;
	// 82914668: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8291466C: 4082004C  bne 0x829146b8
	if !ctx.cr[0].eq {
	pc = 0x829146B8; continue 'dispatch;
	}
	// 82914670: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82914674: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 82914678: 916AB1CC  stw r11, -0x4e34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20020 as u32), ctx.r[11].u32 ) };
	// 8291467C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82914680: 80896878  lwz r4, 0x6878(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26744 as u32) ) } as u64;
	// 82914684: 4B9D08AD  bl 0x822e4f30
	ctx.lr = 0x82914688;
	sub_822E4F30(ctx, base);
	// 82914688: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8291468C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82914690: 4B9D0831  bl 0x822e4ec0
	ctx.lr = 0x82914694;
	sub_822E4EC0(ctx, base);
	// 82914694: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82914698: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8291469C: E89B0000  ld r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 829146A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 829146A4: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 829146A8: 4BB77BE1  bl 0x8248c288
	ctx.lr = 0x829146AC;
	sub_8248C288(ctx, base);
	// 829146AC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 829146B0: 90FFB1C8  stw r7, -0x4e38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20024 as u32), ctx.r[7].u32 ) };
	// 829146B4: 48000008  b 0x829146bc
	pc = 0x829146BC; continue 'dispatch;
	// 829146B8: 80FFB1C8  lwz r7, -0x4e38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-20024 as u32) ) } as u64;
	// 829146BC: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 829146C0: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829146C4: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 829146C8: 13C0E0C7  vcmpequd (lvx128) v30, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829146CC: 813E012C  lwz r9, 0x12c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(300 as u32) ) } as u64;
	// 829146D0: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 829146D4: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 829146D8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829147A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829147A8 size=1716
    let mut pc: u32 = 0x829147A8;
    'dispatch: loop {
        match pc {
            0x829147A8 => {
    //   block [0x829147A8..0x82914E5C)
	// 829147A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829147AC: 48893999  bl 0x831a8144
	ctx.lr = 0x829147B0;
	sub_831A8130(ctx, base);
	// 829147B0: 3981FF90  addi r12, r1, -0x70
	ctx.r[12].s64 = ctx.r[1].s64 + -112;
	// 829147B4: 488942C1  bl 0x831a8a74
	ctx.lr = 0x829147B8;
	sub_831A8A40(ctx, base);
	// 829147B8: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82914E60 size=16
    let mut pc: u32 = 0x82914E60;
    'dispatch: loop {
        match pc {
            0x82914E60 => {
    //   block [0x82914E60..0x82914E70)
	// 82914E60: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82914E64: 396B5BA0  addi r11, r11, 0x5ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 23456;
	// 82914E68: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82914E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82914E70 size=8
    let mut pc: u32 = 0x82914E70;
    'dispatch: loop {
        match pc {
            0x82914E70 => {
    //   block [0x82914E70..0x82914E78)
	// 82914E70: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82914E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82914E78 size=16
    let mut pc: u32 = 0x82914E78;
    'dispatch: loop {
        match pc {
            0x82914E78 => {
    //   block [0x82914E78..0x82914E88)
	// 82914E78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82914E7C: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82914E80: 7C2A5C2E  lfsx f1, r10, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82914E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82914E88 size=20
    let mut pc: u32 = 0x82914E88;
    'dispatch: loop {
        match pc {
            0x82914E88 => {
    //   block [0x82914E88..0x82914E9C)
	// 82914E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82914E8C: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82914E90: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82914E94: C02B0004  lfs f1, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82914E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82914EA0 size=20
    let mut pc: u32 = 0x82914EA0;
    'dispatch: loop {
        match pc {
            0x82914EA0 => {
    //   block [0x82914EA0..0x82914EB4)
	// 82914EA0: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82914EA4: 41980040  blt cr6, 0x82914ee4
	if ctx.cr[6].lt {
		sub_82914EE4(ctx, base);
		return;
	}
	// 82914EA8: 419A000C  beq cr6, 0x82914eb4
	if ctx.cr[6].eq {
		sub_82914EB4(ctx, base);
		return;
	}
	// 82914EAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82914EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914EB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82914EB4 size=48
    let mut pc: u32 = 0x82914EB4;
    'dispatch: loop {
        match pc {
            0x82914EB4 => {
    //   block [0x82914EB4..0x82914EE4)
	// 82914EB4: 3CA08338  lis r5, -0x7cc8
	ctx.r[5].s64 = -2093481984;
	// 82914EB8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82914EBC: 396BB1E0  addi r11, r11, -0x4e20
	ctx.r[11].s64 = ctx.r[11].s64 + -20000;
	// 82914EC0: 8145B1F0  lwz r10, -0x4e10(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-19984 as u32) ) } as u64;
	// 82914EC4: 554907BD  rlwinm. r9, r10, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82914EC8: 4082006C  bne 0x82914f34
	if !ctx.cr[0].eq {
		sub_82914EE4(ctx, base);
		return;
	}
	// 82914ECC: 3D208336  lis r9, -0x7cca
	ctx.r[9].s64 = -2093613056;
	// 82914ED0: 3CC08336  lis r6, -0x7cca
	ctx.r[6].s64 = -2093613056;
	// 82914ED4: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 82914ED8: 812985B8  lwz r9, -0x7a48(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-31304 as u32) ) } as u64;
	// 82914EDC: 80C685B0  lwz r6, -0x7a50(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-31312 as u32) ) } as u64;
	// 82914EE0: 48000030  b 0x82914f10
	sub_82914EE4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82914EE4 size=92
    let mut pc: u32 = 0x82914EE4;
    'dispatch: loop {
        match pc {
            0x82914EE4 => {
    //   block [0x82914EE4..0x82914F40)
	// 82914EE4: 3CA08338  lis r5, -0x7cc8
	ctx.r[5].s64 = -2093481984;
	// 82914EE8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82914EEC: 396BB1D0  addi r11, r11, -0x4e30
	ctx.r[11].s64 = ctx.r[11].s64 + -20016;
	// 82914EF0: 8145B1F0  lwz r10, -0x4e10(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-19984 as u32) ) } as u64;
	// 82914EF4: 554907FF  clrlwi. r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82914EF8: 4082003C  bne 0x82914f34
	if !ctx.cr[0].eq {
	pc = 0x82914F34; continue 'dispatch;
	}
	// 82914EFC: 3D208336  lis r9, -0x7cca
	ctx.r[9].s64 = -2093613056;
	// 82914F00: 3CC08336  lis r6, -0x7cca
	ctx.r[6].s64 = -2093613056;
	// 82914F04: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 82914F08: 812985BC  lwz r9, -0x7a44(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-31300 as u32) ) } as u64;
	// 82914F0C: 80C685B4  lwz r6, -0x7a4c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-31308 as u32) ) } as u64;
	// 82914F10: 3CE08336  lis r7, -0x7cca
	ctx.r[7].s64 = -2093613056;
	// 82914F14: 9145B1F0  stw r10, -0x4e10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(-19984 as u32), ctx.r[10].u32 ) };
	// 82914F18: 3D008336  lis r8, -0x7cca
	ctx.r[8].s64 = -2093613056;
	// 82914F1C: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82914F20: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82914F24: 80E785E8  lwz r7, -0x7a18(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-31256 as u32) ) } as u64;
	// 82914F28: 810885EC  lwz r8, -0x7a14(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-31252 as u32) ) } as u64;
	// 82914F2C: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82914F30: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82914F34: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82914F38: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82914F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82914F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82914F40 size=384
    let mut pc: u32 = 0x82914F40;
    'dispatch: loop {
        match pc {
            0x82914F40 => {
    //   block [0x82914F40..0x829150C0)
	// 82914F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82914F44: 48893215  bl 0x831a8158
	ctx.lr = 0x82914F48;
	sub_831A8130(ctx, base);
	// 82914F48: DBA1FFA0  stfd f29, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[29].u64 ) };
	// 82914F4C: DBC1FFA8  stfd f30, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 82914F50: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82914F54: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82914F58: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82914F5C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82914F60: 419A014C  beq cr6, 0x829150ac
	if ctx.cr[6].eq {
	pc = 0x829150AC; continue 'dispatch;
	}
	// 82914F64: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82914F68: 3CE08204  lis r7, -0x7dfc
	ctx.r[7].s64 = -2113667072;
	// 82914F6C: 3B2BE588  addi r25, r11, -0x1a78
	ctx.r[25].s64 = ctx.r[11].s64 + -6776;
	// 82914F70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82914F74: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82914F78: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82914F7C: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 82914F80: C3C7DD6C  lfs f30, -0x2294(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82914F84: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82914F88: C3AB964C  lfs f29, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82914F8C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82914F90: C3E608A4  lfs f31, 0x8a4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82914F94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82914F98: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82914F9C: 3BA95BEC  addi r29, r9, 0x5bec
	ctx.r[29].s64 = ctx.r[9].s64 + 23532;
	// 82914FA0: 3B885BE4  addi r28, r8, 0x5be4
	ctx.r[28].s64 = ctx.r[8].s64 + 23524;
	// 82914FA4: 3B6A5BD4  addi r27, r10, 0x5bd4
	ctx.r[27].s64 = ctx.r[10].s64 + 23508;
	// 82914FA8: 3B4B1A04  addi r26, r11, 0x1a04
	ctx.r[26].s64 = ctx.r[11].s64 + 6660;
	// 82914FAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82914FB0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82914FB4: 484DEA55  bl 0x82df3a08
	ctx.lr = 0x82914FB8;
	sub_82DF3A08(ctx, base);
	// 82914FB8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82914FBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82914FC0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82914FC4: 484DED35  bl 0x82df3cf8
	ctx.lr = 0x82914FC8;
	sub_82DF3CF8(ctx, base);
	// 82914FC8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82914FCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82914FD0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82914FD4: 484DED25  bl 0x82df3cf8
	ctx.lr = 0x82914FD8;
	sub_82DF3CF8(ctx, base);
	// 82914FD8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82914FDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82914FE0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82914FE4: 484DED15  bl 0x82df3cf8
	ctx.lr = 0x82914FE8;
	sub_82DF3CF8(ctx, base);
	// 82914FE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82914FEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82914FF0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82914FF4: 484DED05  bl 0x82df3cf8
	ctx.lr = 0x82914FF8;
	sub_82DF3CF8(ctx, base);
	// 82914FF8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82914FFC: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82915000: FC60E890  fmr f3, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82915004: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82915008: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8291500C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82915010: 4BC8E299  bl 0x825a32a8
	ctx.lr = 0x82915014;
	sub_825A32A8(ctx, base);
	// 82915014: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82915018: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8291501C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82915020: 4BC8C751  bl 0x825a1770
	ctx.lr = 0x82915024;
	sub_825A1770(ctx, base);
	// 82915024: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82915028: 484DE401  bl 0x82df3428
	ctx.lr = 0x8291502C;
	sub_82DF3428(ctx, base);
	// 8291502C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82915030: 4B9B3C89  bl 0x822c8cb8
	ctx.lr = 0x82915034;
	sub_822C8CB8(ctx, base);
	// 82915034: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 82915038: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8291503C: FC60E890  fmr f3, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82915040: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82915044: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82915048: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8291504C: 4BC8E25D  bl 0x825a32a8
	ctx.lr = 0x82915050;
	sub_825A32A8(ctx, base);
	// 82915050: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82915054: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82915058: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8291505C: 4BC8C715  bl 0x825a1770
	ctx.lr = 0x82915060;
	sub_825A1770(ctx, base);
	// 82915060: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82915064: 484DE3C5  bl 0x82df3428
	ctx.lr = 0x82915068;
	sub_82DF3428(ctx, base);
	// 82915068: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8291506C: 4B9B3C4D  bl 0x822c8cb8
	ctx.lr = 0x82915070;
	sub_822C8CB8(ctx, base);
	// 82915070: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82915074: 484DE3B5  bl 0x82df3428
	ctx.lr = 0x82915078;
	sub_82DF3428(ctx, base);
	// 82915078: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8291507C: 484DE3AD  bl 0x82df3428
	ctx.lr = 0x82915080;
	sub_82DF3428(ctx, base);
	// 82915080: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82915084: 484DE3A5  bl 0x82df3428
	ctx.lr = 0x82915088;
	sub_82DF3428(ctx, base);
	// 82915088: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8291508C: 484DE39D  bl 0x82df3428
	ctx.lr = 0x82915090;
	sub_82DF3428(ctx, base);
	// 82915090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82915094: 484DE395  bl 0x82df3428
	ctx.lr = 0x82915098;
	sub_82DF3428(ctx, base);
	// 82915098: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8291509C: 39790010  addi r11, r25, 0x10
	ctx.r[11].s64 = ctx.r[25].s64 + 16;
	// 829150A0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 829150A4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 829150A8: 4198FF04  blt cr6, 0x82914fac
	if ctx.cr[6].lt {
	pc = 0x82914FAC; continue 'dispatch;
	}
	// 829150AC: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 829150B0: CBA1FFA0  lfd f29, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 829150B4: CBC1FFA8  lfd f30, -0x58(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 829150B8: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 829150BC: 488930EC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829150C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829150C0 size=132
    let mut pc: u32 = 0x829150C0;
    'dispatch: loop {
        match pc {
            0x829150C0 => {
    //   block [0x829150C0..0x82915144)
	// 829150C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829150C4: 488930A9  bl 0x831a816c
	ctx.lr = 0x829150C8;
	sub_831A8130(ctx, base);
	// 829150C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829150CC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829150D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829150D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829150D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 829150DC: 388B5C10  addi r4, r11, 0x5c10
	ctx.r[4].s64 = ctx.r[11].s64 + 23568;
	// 829150E0: 484DE929  bl 0x82df3a08
	ctx.lr = 0x829150E4;
	sub_82DF3A08(ctx, base);
	// 829150E4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829150E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 829150EC: 388B5BF8  addi r4, r11, 0x5bf8
	ctx.r[4].s64 = ctx.r[11].s64 + 23544;
	// 829150F0: 484DE919  bl 0x82df3a08
	ctx.lr = 0x829150F4;
	sub_82DF3A08(ctx, base);
	// 829150F4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 829150F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 829150FC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82915100: 4BCB5401  bl 0x825ca500
	ctx.lr = 0x82915104;
	sub_825CA500(ctx, base);
	// 82915104: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82915108: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8291510C: 484DE31D  bl 0x82df3428
	ctx.lr = 0x82915110;
	sub_82DF3428(ctx, base);
	// 82915110: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82915114: 484DE315  bl 0x82df3428
	ctx.lr = 0x82915118;
	sub_82DF3428(ctx, base);
	// 82915118: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8291511C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915120: 4BFFFE21  bl 0x82914f40
	ctx.lr = 0x82915124;
	sub_82914F40(ctx, base);
	// 82915124: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82915128: 4BCB4BA9  bl 0x825c9cd0
	ctx.lr = 0x8291512C;
	sub_825C9CD0(ctx, base);
	// 8291512C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82915130: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82915134: 419A0008  beq cr6, 0x8291513c
	if ctx.cr[6].eq {
	pc = 0x8291513C; continue 'dispatch;
	}
	// 82915138: 4B9AB759  bl 0x822c0890
	ctx.lr = 0x8291513C;
	sub_822C0890(ctx, base);
	// 8291513C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82915140: 4889307C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915148 size=92
    let mut pc: u32 = 0x82915148;
    'dispatch: loop {
        match pc {
            0x82915148 => {
    //   block [0x82915148..0x829151A4)
	// 82915148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291514C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915154: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82915158: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8291515C: 4BF55E35  bl 0x8286af90
	ctx.lr = 0x82915160;
	sub_8286AF90(ctx, base);
	// 82915160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82915164: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82915168: 41820008  beq 0x82915170
	if ctx.cr[0].eq {
	pc = 0x82915170; continue 'dispatch;
	}
	// 8291516C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82915170: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82915174: 41820008  beq 0x8291517c
	if ctx.cr[0].eq {
	pc = 0x8291517C; continue 'dispatch;
	}
	// 82915178: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8291517C: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82915180: 41820008  beq 0x82915188
	if ctx.cr[0].eq {
	pc = 0x82915188; continue 'dispatch;
	}
	// 82915184: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82915188: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8291518C: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 82915190: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82915194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291519C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829151A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829151A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829151A8 size=88
    let mut pc: u32 = 0x829151A8;
    'dispatch: loop {
        match pc {
            0x829151A8 => {
    //   block [0x829151A8..0x82915200)
	// 829151A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829151AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829151B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829151B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829151B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829151BC: 4BFFFF8D  bl 0x82915148
	ctx.lr = 0x829151C0;
	sub_82915148(ctx, base);
	// 829151C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 829151C4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 829151C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 829151CC: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 829151D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 829151D4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 829151D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 829151DC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829151E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 829151E4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 829151E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 829151EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829151F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829151F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829151F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829151FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915200 size=112
    let mut pc: u32 = 0x82915200;
    'dispatch: loop {
        match pc {
            0x82915200 => {
    //   block [0x82915200..0x82915270)
	// 82915200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8291520C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915210: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915214: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82915218: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291521C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82915220: 4800002C  b 0x8291524c
	pc = 0x8291524C; continue 'dispatch;
	// 82915224: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82915228: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8291522C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82915230: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 82915234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82915238: 4E800421  bctrl
	ctx.lr = 0x8291523C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8291523C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82915240: 4BBD3E51  bl 0x824e9090
	ctx.lr = 0x82915244;
	sub_824E9090(ctx, base);
	// 82915244: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82915248: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8291524C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82915250: 409AFFD4  bne cr6, 0x82915224
	if !ctx.cr[6].eq {
	pc = 0x82915224; continue 'dispatch;
	}
	// 82915254: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82915258: 4800CC59  bl 0x82921eb0
	ctx.lr = 0x8291525C;
	sub_82921EB0(ctx, base);
	// 8291525C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8291526C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915270 size=216
    let mut pc: u32 = 0x82915270;
    'dispatch: loop {
        match pc {
            0x82915270 => {
    //   block [0x82915270..0x82915348)
	// 82915270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291527C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915288: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 8291528C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82915290: 556BF7FF  rlwinm. r11, r11, 0x1e, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82915294: 4082009C  bne 0x82915330
	if !ctx.cr[0].eq {
	pc = 0x82915330; continue 'dispatch;
	}
	// 82915298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8291529C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 829152A0: 4BED7329  bl 0x827ec5c8
	ctx.lr = 0x829152A4;
	sub_827EC5C8(ctx, base);
	// 829152A4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829152A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829152AC: 419A0074  beq cr6, 0x82915320
	if ctx.cr[6].eq {
	pc = 0x82915320; continue 'dispatch;
	}
	// 829152B0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 829152B4: 484E952D  bl 0x82dfe7e0
	ctx.lr = 0x829152B8;
	sub_82DFE7E0(ctx, base);
	// 829152B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829152BC: 41820064  beq 0x82915320
	if ctx.cr[0].eq {
	pc = 0x82915320; continue 'dispatch;
	}
	// 829152C0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 829152C4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 829152C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 829152CC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 829152D0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 829152D4: 419A0024  beq cr6, 0x829152f8
	if ctx.cr[6].eq {
	pc = 0x829152F8; continue 'dispatch;
	}
	// 829152D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 829152DC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 829152E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829152E4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 829152E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 829152EC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 829152F0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 829152F4: 4082FFE8  bne 0x829152dc
	if !ctx.cr[0].eq {
	pc = 0x829152DC; continue 'dispatch;
	}
	// 829152F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 829152FC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82915300: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82915304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915308: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8291530C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82915310: 4E800421  bctrl
	ctx.lr = 0x82915314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82915314: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82915318: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 8291531C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915320: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82915324: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82915328: 419A0008  beq cr6, 0x82915330
	if ctx.cr[6].eq {
	pc = 0x82915330; continue 'dispatch;
	}
	// 8291532C: 4B9AB565  bl 0x822c0890
	ctx.lr = 0x82915330;
	sub_822C0890(ctx, base);
	// 82915330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82915334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291533C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82915340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915348 size=116
    let mut pc: u32 = 0x82915348;
    'dispatch: loop {
        match pc {
            0x82915348 => {
    //   block [0x82915348..0x829153BC)
	// 82915348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82915354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291535C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915360: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82915364: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82915368: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291536C: 41820034  beq 0x829153a0
	if ctx.cr[0].eq {
	pc = 0x829153A0; continue 'dispatch;
	}
	// 82915370: 38A40018  addi r5, r4, 0x18
	ctx.r[5].s64 = ctx.r[4].s64 + 24;
	// 82915374: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82915378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8291537C: 4BF55CE5  bl 0x8286b060
	ctx.lr = 0x82915380;
	sub_8286B060(ctx, base);
	// 82915380: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82915384: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82915388: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8291538C: 419A0014  beq cr6, 0x829153a0
	if ctx.cr[6].eq {
	pc = 0x829153A0; continue 'dispatch;
	}
	// 82915390: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82915394: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82915398: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8291539C: 48000008  b 0x829153a4
	pc = 0x829153A4; continue 'dispatch;
	// 829153A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 829153A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829153A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829153AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829153B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829153B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829153B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829153C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829153C0 size=20
    let mut pc: u32 = 0x829153C0;
    'dispatch: loop {
        match pc {
            0x829153C0 => {
    //   block [0x829153C0..0x829153D4)
	// 829153C0: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829153C4: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 829153C8: 4182000C  beq 0x829153d4
	if ctx.cr[0].eq {
		sub_829153D4(ctx, base);
		return;
	}
	// 829153CC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 829153D0: 48000008  b 0x829153d8
	sub_829153D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829153D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829153D4 size=12
    let mut pc: u32 = 0x829153D4;
    'dispatch: loop {
        match pc {
            0x829153D4 => {
    //   block [0x829153D4..0x829153E0)
	// 829153D4: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 829153D8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 829153DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829153E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829153E0 size=20
    let mut pc: u32 = 0x829153E0;
    'dispatch: loop {
        match pc {
            0x829153E0 => {
    //   block [0x829153E0..0x829153F4)
	// 829153E0: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829153E4: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 829153E8: 4182000C  beq 0x829153f4
	if ctx.cr[0].eq {
		sub_829153F4(ctx, base);
		return;
	}
	// 829153EC: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 829153F0: 48000008  b 0x829153f8
	sub_829153F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829153F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829153F4 size=12
    let mut pc: u32 = 0x829153F4;
    'dispatch: loop {
        match pc {
            0x829153F4 => {
    //   block [0x829153F4..0x82915400)
	// 829153F4: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 829153F8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 829153FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82915400 size=12
    let mut pc: u32 = 0x82915400;
    'dispatch: loop {
        match pc {
            0x82915400 => {
    //   block [0x82915400..0x8291540C)
	// 82915400: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82915404: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82915408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82915410 size=12
    let mut pc: u32 = 0x82915410;
    'dispatch: loop {
        match pc {
            0x82915410 => {
    //   block [0x82915410..0x8291541C)
	// 82915410: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82915414: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82915418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82915420 size=12
    let mut pc: u32 = 0x82915420;
    'dispatch: loop {
        match pc {
            0x82915420 => {
    //   block [0x82915420..0x8291542C)
	// 82915420: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82915424: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82915428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915430 size=196
    let mut pc: u32 = 0x82915430;
    'dispatch: loop {
        match pc {
            0x82915430 => {
    //   block [0x82915430..0x829154F4)
	// 82915430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291543C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915444: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82915448: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8291544C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82915450: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82915454: 484E938D  bl 0x82dfe7e0
	ctx.lr = 0x82915458;
	sub_82DFE7E0(ctx, base);
	// 82915458: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291545C: 41820064  beq 0x829154c0
	if ctx.cr[0].eq {
	pc = 0x829154C0; continue 'dispatch;
	}
	// 82915460: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82915464: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82915468: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8291546C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82915470: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82915474: 419A0024  beq cr6, 0x82915498
	if ctx.cr[6].eq {
	pc = 0x82915498; continue 'dispatch;
	}
	// 82915478: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8291547C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82915480: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82915484: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82915488: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8291548C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82915490: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82915494: 4082FFE8  bne 0x8291547c
	if !ctx.cr[0].eq {
	pc = 0x8291547C; continue 'dispatch;
	}
	// 82915498: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291549C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 829154A0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 829154A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829154A8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 829154AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829154B0: 4E800421  bctrl
	ctx.lr = 0x829154B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829154B4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 829154B8: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 829154BC: 4800000C  b 0x829154c8
	pc = 0x829154C8; continue 'dispatch;
	// 829154C0: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 829154C4: 556B07B8  rlwinm r11, r11, 0, 0x1e, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 829154C8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 829154CC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 829154D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 829154D4: 419A0008  beq cr6, 0x829154dc
	if ctx.cr[6].eq {
	pc = 0x829154DC; continue 'dispatch;
	}
	// 829154D8: 4B9AB3B9  bl 0x822c0890
	ctx.lr = 0x829154DC;
	sub_822C0890(ctx, base);
	// 829154DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829154E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829154E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829154E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829154EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829154F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829154F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829154F8 size=80
    let mut pc: u32 = 0x829154F8;
    'dispatch: loop {
        match pc {
            0x829154F8 => {
    //   block [0x829154F8..0x82915548)
	// 829154F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829154FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915500: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915504: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291550C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82915510: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82915514: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82915518: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291551C: 4BFA3065  bl 0x828b8580
	ctx.lr = 0x82915520;
	sub_828B8580(ctx, base);
	// 82915520: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82915524: 4B9AAD45  bl 0x822c0268
	ctx.lr = 0x82915528;
	sub_822C0268(ctx, base);
	// 82915528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8291552C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82915530: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82915534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291553C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915548 size=64
    let mut pc: u32 = 0x82915548;
    'dispatch: loop {
        match pc {
            0x82915548 => {
    //   block [0x82915548..0x82915588)
	// 82915548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291554C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915554: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291555C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82915560: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82915564: 4BED7065  bl 0x827ec5c8
	ctx.lr = 0x82915568;
	sub_827EC5C8(ctx, base);
	// 82915568: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8291556C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915570: 4BFFFEC1  bl 0x82915430
	ctx.lr = 0x82915574;
	sub_82915430(ctx, base);
	// 82915574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291557C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82915588 size=24
    let mut pc: u32 = 0x82915588;
    'dispatch: loop {
        match pc {
            0x82915588 => {
    //   block [0x82915588..0x829155A0)
	// 82915588: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8291558C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82915590: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82915594: 394A5C28  addi r10, r10, 0x5c28
	ctx.r[10].s64 = ctx.r[10].s64 + 23592;
	// 82915598: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8291559C: 4BFFFF5C  b 0x829154f8
	sub_829154F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829155A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829155A0 size=88
    let mut pc: u32 = 0x829155A0;
    'dispatch: loop {
        match pc {
            0x829155A0 => {
    //   block [0x829155A0..0x829155F8)
	// 829155A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829155A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829155A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829155AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829155B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829155B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829155B8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829155BC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 829155C0: 396B5C28  addi r11, r11, 0x5c28
	ctx.r[11].s64 = ctx.r[11].s64 + 23592;
	// 829155C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829155C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829155CC: 4BFFFBDD  bl 0x829151a8
	ctx.lr = 0x829155D0;
	sub_829151A8(ctx, base);
	// 829155D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829155D4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 829155D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829155DC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 829155E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829155E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829155E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829155EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829155F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829155F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829155F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829155F8 size=92
    let mut pc: u32 = 0x829155F8;
    'dispatch: loop {
        match pc {
            0x829155F8 => {
    //   block [0x829155F8..0x82915654)
	// 829155F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829155FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82915604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291560C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915610: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915614: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82915618: 396B5C28  addi r11, r11, 0x5c28
	ctx.r[11].s64 = ctx.r[11].s64 + 23592;
	// 8291561C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915620: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915624: 4BFFFED5  bl 0x829154f8
	ctx.lr = 0x82915628;
	sub_829154F8(ctx, base);
	// 82915628: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291562C: 4182000C  beq 0x82915638
	if ctx.cr[0].eq {
	pc = 0x82915638; continue 'dispatch;
	}
	// 82915630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915634: 4B9AAC35  bl 0x822c0268
	ctx.lr = 0x82915638;
	sub_822C0268(ctx, base);
	// 82915638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291563C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915648: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291564C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915658 size=60
    let mut pc: u32 = 0x82915658;
    'dispatch: loop {
        match pc {
            0x82915658 => {
    //   block [0x82915658..0x82915694)
	// 82915658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291565C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291566C: 48544015  bl 0x82e59680
	ctx.lr = 0x82915670;
	sub_82E59680(ctx, base);
	// 82915670: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915678: 396B5C5C  addi r11, r11, 0x5c5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23644;
	// 8291567C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291568C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915698 size=60
    let mut pc: u32 = 0x82915698;
    'dispatch: loop {
        match pc {
            0x82915698 => {
    //   block [0x82915698..0x829156D4)
	// 82915698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291569C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829156A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829156A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829156A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829156AC: 48543FD5  bl 0x82e59680
	ctx.lr = 0x829156B0;
	sub_82E59680(ctx, base);
	// 829156B0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829156B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829156B8: 396B5C64  addi r11, r11, 0x5c64
	ctx.r[11].s64 = ctx.r[11].s64 + 23652;
	// 829156BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829156C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829156C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829156C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829156CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829156D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829156D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829156D8 size=60
    let mut pc: u32 = 0x829156D8;
    'dispatch: loop {
        match pc {
            0x829156D8 => {
    //   block [0x829156D8..0x82915714)
	// 829156D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829156DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829156E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829156E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829156E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829156EC: 48543F95  bl 0x82e59680
	ctx.lr = 0x829156F0;
	sub_82E59680(ctx, base);
	// 829156F0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829156F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829156F8: 396B5C6C  addi r11, r11, 0x5c6c
	ctx.r[11].s64 = ctx.r[11].s64 + 23660;
	// 829156FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291570C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915718 size=60
    let mut pc: u32 = 0x82915718;
    'dispatch: loop {
        match pc {
            0x82915718 => {
    //   block [0x82915718..0x82915754)
	// 82915718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291571C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291572C: 48543F55  bl 0x82e59680
	ctx.lr = 0x82915730;
	sub_82E59680(ctx, base);
	// 82915730: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915738: 396B5C74  addi r11, r11, 0x5c74
	ctx.r[11].s64 = ctx.r[11].s64 + 23668;
	// 8291573C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291574C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915758 size=76
    let mut pc: u32 = 0x82915758;
    'dispatch: loop {
        match pc {
            0x82915758 => {
    //   block [0x82915758..0x829157A4)
	// 82915758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291575C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915760: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82915764: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291576C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915770: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915774: 48543F0D  bl 0x82e59680
	ctx.lr = 0x82915778;
	sub_82E59680(ctx, base);
	// 82915778: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8291577C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82915780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915784: 396B5C7C  addi r11, r11, 0x5c7c
	ctx.r[11].s64 = ctx.r[11].s64 + 23676;
	// 82915788: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8291578C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915798: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291579C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829157A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829157A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829157A8 size=68
    let mut pc: u32 = 0x829157A8;
    'dispatch: loop {
        match pc {
            0x829157A8 => {
    //   block [0x829157A8..0x829157EC)
	// 829157A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829157AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829157B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829157B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829157B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829157BC: 48543EE5  bl 0x82e596a0
	ctx.lr = 0x829157C0;
	sub_82E596A0(ctx, base);
	// 829157C0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829157C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 829157C8: 396B5C84  addi r11, r11, 0x5c84
	ctx.r[11].s64 = ctx.r[11].s64 + 23684;
	// 829157CC: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 829157D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829157D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829157D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829157DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829157E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829157E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829157E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829157F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829157F0 size=76
    let mut pc: u32 = 0x829157F0;
    'dispatch: loop {
        match pc {
            0x829157F0 => {
    //   block [0x829157F0..0x8291583C)
	// 829157F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829157F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829157F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829157FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915808: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291580C: 48543E75  bl 0x82e59680
	ctx.lr = 0x82915810;
	sub_82E59680(ctx, base);
	// 82915810: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915814: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82915818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291581C: 396B5C8C  addi r11, r11, 0x5c8c
	ctx.r[11].s64 = ctx.r[11].s64 + 23692;
	// 82915820: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915824: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291582C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915830: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82915834: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915840 size=84
    let mut pc: u32 = 0x82915840;
    'dispatch: loop {
        match pc {
            0x82915840 => {
    //   block [0x82915840..0x82915894)
	// 82915840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291584C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915858: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291585C: 48543E25  bl 0x82e59680
	ctx.lr = 0x82915860;
	sub_82E59680(ctx, base);
	// 82915860: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915864: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82915868: 396B5C94  addi r11, r11, 0x5c94
	ctx.r[11].s64 = ctx.r[11].s64 + 23700;
	// 8291586C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82915870: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915874: 484DE38D  bl 0x82df3c00
	ctx.lr = 0x82915878;
	sub_82DF3C00(ctx, base);
	// 82915878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291587C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291588C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915898 size=76
    let mut pc: u32 = 0x82915898;
    'dispatch: loop {
        match pc {
            0x82915898 => {
    //   block [0x82915898..0x829158E4)
	// 82915898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291589C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829158A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829158A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829158A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829158AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829158B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829158B4: 48543DCD  bl 0x82e59680
	ctx.lr = 0x829158B8;
	sub_82E59680(ctx, base);
	// 829158B8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829158BC: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 829158C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829158C4: 396B5C9C  addi r11, r11, 0x5c9c
	ctx.r[11].s64 = ctx.r[11].s64 + 23708;
	// 829158C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829158CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829158D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829158D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829158D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829158DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829158E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829158E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829158E8 size=92
    let mut pc: u32 = 0x829158E8;
    'dispatch: loop {
        match pc {
            0x829158E8 => {
    //   block [0x829158E8..0x82915944)
	// 829158E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829158EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829158F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829158F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829158F8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 829158FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915904: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82915908: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291590C: 48543D75  bl 0x82e59680
	ctx.lr = 0x82915910;
	sub_82E59680(ctx, base);
	// 82915910: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915914: D3FF001C  stfs f31, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82915918: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 8291591C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915920: 396B5CA4  addi r11, r11, 0x5ca4
	ctx.r[11].s64 = ctx.r[11].s64 + 23716;
	// 82915924: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915928: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8291592C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915934: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82915938: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291593C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82915948 size=76
    let mut pc: u32 = 0x82915948;
    'dispatch: loop {
        match pc {
            0x82915948 => {
    //   block [0x82915948..0x82915994)
	// 82915948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291594C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915954: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82915958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291595C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915960: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82915964: 48543D1D  bl 0x82e59680
	ctx.lr = 0x82915968;
	sub_82E59680(ctx, base);
	// 82915968: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8291596C: D3FF0018  stfs f31, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82915970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915974: 396B5CAC  addi r11, r11, 0x5cac
	ctx.r[11].s64 = ctx.r[11].s64 + 23724;
	// 82915978: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8291597C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915988: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291598C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915998 size=60
    let mut pc: u32 = 0x82915998;
    'dispatch: loop {
        match pc {
            0x82915998 => {
    //   block [0x82915998..0x829159D4)
	// 82915998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291599C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829159A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829159A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829159A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829159AC: 48543CD5  bl 0x82e59680
	ctx.lr = 0x829159B0;
	sub_82E59680(ctx, base);
	// 829159B0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829159B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829159B8: 396B5CB4  addi r11, r11, 0x5cb4
	ctx.r[11].s64 = ctx.r[11].s64 + 23732;
	// 829159BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829159C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829159C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829159C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829159CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829159D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829159D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829159D8 size=60
    let mut pc: u32 = 0x829159D8;
    'dispatch: loop {
        match pc {
            0x829159D8 => {
    //   block [0x829159D8..0x82915A14)
	// 829159D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829159DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829159E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829159E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829159E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829159EC: 48543C95  bl 0x82e59680
	ctx.lr = 0x829159F0;
	sub_82E59680(ctx, base);
	// 829159F0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829159F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829159F8: 396B5CBC  addi r11, r11, 0x5cbc
	ctx.r[11].s64 = ctx.r[11].s64 + 23740;
	// 829159FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915A18 size=60
    let mut pc: u32 = 0x82915A18;
    'dispatch: loop {
        match pc {
            0x82915A18 => {
    //   block [0x82915A18..0x82915A54)
	// 82915A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915A2C: 48543C55  bl 0x82e59680
	ctx.lr = 0x82915A30;
	sub_82E59680(ctx, base);
	// 82915A30: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915A38: 396B5CC4  addi r11, r11, 0x5cc4
	ctx.r[11].s64 = ctx.r[11].s64 + 23748;
	// 82915A3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915A40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915A4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915A58 size=60
    let mut pc: u32 = 0x82915A58;
    'dispatch: loop {
        match pc {
            0x82915A58 => {
    //   block [0x82915A58..0x82915A94)
	// 82915A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915A60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915A64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915A68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915A6C: 48543C15  bl 0x82e59680
	ctx.lr = 0x82915A70;
	sub_82E59680(ctx, base);
	// 82915A70: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915A78: 396B5CCC  addi r11, r11, 0x5ccc
	ctx.r[11].s64 = ctx.r[11].s64 + 23756;
	// 82915A7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915A80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915A8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915A98 size=76
    let mut pc: u32 = 0x82915A98;
    'dispatch: loop {
        match pc {
            0x82915A98 => {
    //   block [0x82915A98..0x82915AE4)
	// 82915A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915AA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82915AA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915AA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915AAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915AB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915AB4: 48543BCD  bl 0x82e59680
	ctx.lr = 0x82915AB8;
	sub_82E59680(ctx, base);
	// 82915AB8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915ABC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82915AC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915AC4: 396B5CD4  addi r11, r11, 0x5cd4
	ctx.r[11].s64 = ctx.r[11].s64 + 23764;
	// 82915AC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915ACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915AD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82915ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915AE8 size=60
    let mut pc: u32 = 0x82915AE8;
    'dispatch: loop {
        match pc {
            0x82915AE8 => {
    //   block [0x82915AE8..0x82915B24)
	// 82915AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915AEC: 48892681  bl 0x831a816c
	ctx.lr = 0x82915AF0;
	sub_831A8130(ctx, base);
	// 82915AF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915AF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915AF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915AFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82915B00: 48543B81  bl 0x82e59680
	ctx.lr = 0x82915B04;
	sub_82E59680(ctx, base);
	// 82915B04: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915B08: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82915B0C: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 82915B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915B14: 396B5CDC  addi r11, r11, 0x5cdc
	ctx.r[11].s64 = ctx.r[11].s64 + 23772;
	// 82915B18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915B1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915B20: 4889269C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915B28 size=60
    let mut pc: u32 = 0x82915B28;
    'dispatch: loop {
        match pc {
            0x82915B28 => {
    //   block [0x82915B28..0x82915B64)
	// 82915B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915B30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915B34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915B3C: 48543B45  bl 0x82e59680
	ctx.lr = 0x82915B40;
	sub_82E59680(ctx, base);
	// 82915B40: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915B48: 396B5CE4  addi r11, r11, 0x5ce4
	ctx.r[11].s64 = ctx.r[11].s64 + 23780;
	// 82915B4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915B5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915B68 size=76
    let mut pc: u32 = 0x82915B68;
    'dispatch: loop {
        match pc {
            0x82915B68 => {
    //   block [0x82915B68..0x82915BB4)
	// 82915B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82915B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915B80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915B84: 48543AFD  bl 0x82e59680
	ctx.lr = 0x82915B88;
	sub_82E59680(ctx, base);
	// 82915B88: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915B8C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82915B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915B94: 396B5CEC  addi r11, r11, 0x5cec
	ctx.r[11].s64 = ctx.r[11].s64 + 23788;
	// 82915B98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915B9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915BA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82915BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915BB8 size=60
    let mut pc: u32 = 0x82915BB8;
    'dispatch: loop {
        match pc {
            0x82915BB8 => {
    //   block [0x82915BB8..0x82915BF4)
	// 82915BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915BC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915BC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915BC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915BCC: 48543AB5  bl 0x82e59680
	ctx.lr = 0x82915BD0;
	sub_82E59680(ctx, base);
	// 82915BD0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915BD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915BD8: 396B5CF4  addi r11, r11, 0x5cf4
	ctx.r[11].s64 = ctx.r[11].s64 + 23796;
	// 82915BDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915BF8 size=60
    let mut pc: u32 = 0x82915BF8;
    'dispatch: loop {
        match pc {
            0x82915BF8 => {
    //   block [0x82915BF8..0x82915C34)
	// 82915BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915C04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915C0C: 48543A75  bl 0x82e59680
	ctx.lr = 0x82915C10;
	sub_82E59680(ctx, base);
	// 82915C10: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915C18: 396B5CFC  addi r11, r11, 0x5cfc
	ctx.r[11].s64 = ctx.r[11].s64 + 23804;
	// 82915C1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915C2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915C38 size=60
    let mut pc: u32 = 0x82915C38;
    'dispatch: loop {
        match pc {
            0x82915C38 => {
    //   block [0x82915C38..0x82915C74)
	// 82915C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915C40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915C44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915C4C: 48543A55  bl 0x82e596a0
	ctx.lr = 0x82915C50;
	sub_82E596A0(ctx, base);
	// 82915C50: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915C58: 396B5D04  addi r11, r11, 0x5d04
	ctx.r[11].s64 = ctx.r[11].s64 + 23812;
	// 82915C5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915C6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82915C78 size=76
    let mut pc: u32 = 0x82915C78;
    'dispatch: loop {
        match pc {
            0x82915C78 => {
    //   block [0x82915C78..0x82915CC4)
	// 82915C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915C84: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82915C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915C90: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82915C94: 485439ED  bl 0x82e59680
	ctx.lr = 0x82915C98;
	sub_82E59680(ctx, base);
	// 82915C98: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915C9C: D3FF0018  stfs f31, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82915CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915CA4: 396B5D0C  addi r11, r11, 0x5d0c
	ctx.r[11].s64 = ctx.r[11].s64 + 23820;
	// 82915CA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915CB8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82915CBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915CC8 size=60
    let mut pc: u32 = 0x82915CC8;
    'dispatch: loop {
        match pc {
            0x82915CC8 => {
    //   block [0x82915CC8..0x82915D04)
	// 82915CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915CDC: 485439A5  bl 0x82e59680
	ctx.lr = 0x82915CE0;
	sub_82E59680(ctx, base);
	// 82915CE0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915CE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915CE8: 396B5D14  addi r11, r11, 0x5d14
	ctx.r[11].s64 = ctx.r[11].s64 + 23828;
	// 82915CEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915D08 size=68
    let mut pc: u32 = 0x82915D08;
    'dispatch: loop {
        match pc {
            0x82915D08 => {
    //   block [0x82915D08..0x82915D4C)
	// 82915D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915D18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915D1C: 48543985  bl 0x82e596a0
	ctx.lr = 0x82915D20;
	sub_82E596A0(ctx, base);
	// 82915D20: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915D24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82915D28: 396B5D1C  addi r11, r11, 0x5d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 23836;
	// 82915D2C: 995F0018  stb r10, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82915D30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915D34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915D50 size=60
    let mut pc: u32 = 0x82915D50;
    'dispatch: loop {
        match pc {
            0x82915D50 => {
    //   block [0x82915D50..0x82915D8C)
	// 82915D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915D5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915D64: 4854391D  bl 0x82e59680
	ctx.lr = 0x82915D68;
	sub_82E59680(ctx, base);
	// 82915D68: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915D70: 396B5D24  addi r11, r11, 0x5d24
	ctx.r[11].s64 = ctx.r[11].s64 + 23844;
	// 82915D74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915D84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915D90 size=60
    let mut pc: u32 = 0x82915D90;
    'dispatch: loop {
        match pc {
            0x82915D90 => {
    //   block [0x82915D90..0x82915DCC)
	// 82915D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915DA4: 485438DD  bl 0x82e59680
	ctx.lr = 0x82915DA8;
	sub_82E59680(ctx, base);
	// 82915DA8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915DB0: 396B5D2C  addi r11, r11, 0x5d2c
	ctx.r[11].s64 = ctx.r[11].s64 + 23852;
	// 82915DB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82915DD0 size=68
    let mut pc: u32 = 0x82915DD0;
    'dispatch: loop {
        match pc {
            0x82915DD0 => {
    //   block [0x82915DD0..0x82915E14)
	// 82915DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915DD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915DDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915DE4: 485438BD  bl 0x82e596a0
	ctx.lr = 0x82915DE8;
	sub_82E596A0(ctx, base);
	// 82915DE8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915DEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82915DF0: 396B5D34  addi r11, r11, 0x5d34
	ctx.r[11].s64 = ctx.r[11].s64 + 23860;
	// 82915DF4: 995F0018  stb r10, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82915DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915DFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82915E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915E0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82915E18 size=92
    let mut pc: u32 = 0x82915E18;
    'dispatch: loop {
        match pc {
            0x82915E18 => {
    //   block [0x82915E18..0x82915E74)
	// 82915E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915E20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915E24: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82915E28: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82915E2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915E34: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82915E38: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82915E3C: 48543845  bl 0x82e59680
	ctx.lr = 0x82915E40;
	sub_82E59680(ctx, base);
	// 82915E40: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915E44: D3FF0018  stfs f31, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82915E48: D3DF001C  stfs f30, 0x1c(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82915E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915E50: 396B5D3C  addi r11, r11, 0x5d3c
	ctx.r[11].s64 = ctx.r[11].s64 + 23868;
	// 82915E54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915E58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82915E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82915E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82915E64: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82915E68: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82915E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82915E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82915E78 size=108
    let mut pc: u32 = 0x82915E78;
    'dispatch: loop {
        match pc {
            0x82915E78 => {
    //   block [0x82915E78..0x82915EE4)
	// 82915E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915E7C: 488922E5  bl 0x831a8160
	ctx.lr = 0x82915E80;
	sub_831A8130(ctx, base);
	// 82915E80: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82915E84: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915E8C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82915E90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915E94: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82915E98: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82915E9C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82915EA0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82915EA4: 485437DD  bl 0x82e59680
	ctx.lr = 0x82915EA8;
	sub_82E59680(ctx, base);
	// 82915EA8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915EAC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82915EB0: 396B5D44  addi r11, r11, 0x5d44
	ctx.r[11].s64 = ctx.r[11].s64 + 23876;
	// 82915EB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915EB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82915EBC: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82915EE8 size=96
    let mut pc: u32 = 0x82915EE8;
    'dispatch: loop {
        match pc {
            0x82915EE8 => {
    //   block [0x82915EE8..0x82915F48)
	// 82915EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915EF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915EF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915EF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915EFC: 485437A5  bl 0x82e596a0
	ctx.lr = 0x82915F00;
	sub_82E596A0(ctx, base);
	// 82915F00: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82915F04: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82915F08: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82915F0C: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82915F10: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82915F14: 39295D4C  addi r9, r9, 0x5d4c
	ctx.r[9].s64 = ctx.r[9].s64 + 23884;
	// 82915F18: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82915F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82915F20: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82915F24: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82915F28: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82915F2C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82915F48 size=136
    let mut pc: u32 = 0x82915F48;
    'dispatch: loop {
        match pc {
            0x82915F48 => {
    //   block [0x82915F48..0x82915FD0)
	// 82915F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82915F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82915F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82915F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915F60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915F64: 4854373D  bl 0x82e596a0
	ctx.lr = 0x82915F68;
	sub_82E596A0(ctx, base);
	// 82915F68: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82915F6C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82915F70: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82915F74: 390B6910  addi r8, r11, 0x6910
	ctx.r[8].s64 = ctx.r[11].s64 + 26896;
	// 82915F78: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82915F7C: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82915F80: 39295D54  addi r9, r9, 0x5d54
	ctx.r[9].s64 = ctx.r[9].s64 + 23892;
	// 82915F84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82915F88: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82915F8C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82915F90: 13E040C7  vcmpequd (lvx128) v31, v0, v8
	tmp.u32 = ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82915F94: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82915F98: C00708A4  lfs f0, 0x8a4(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82915F9C: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82915FA0: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82915FA4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82915FA8: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82915FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82915FD0 size=152
    let mut pc: u32 = 0x82915FD0;
    'dispatch: loop {
        match pc {
            0x82915FD0 => {
    //   block [0x82915FD0..0x82916068)
	// 82915FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82915FD4: 48892199  bl 0x831a816c
	ctx.lr = 0x82915FD8;
	sub_831A8130(ctx, base);
	// 82915FD8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82915FDC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82915FE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82915FE4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82915FE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82915FEC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82915FF0: 485436B1  bl 0x82e596a0
	ctx.lr = 0x82915FF4;
	sub_82E596A0(ctx, base);
	// 82915FF4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82915FF8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82915FFC: 396B5D5C  addi r11, r11, 0x5d5c
	ctx.r[11].s64 = ctx.r[11].s64 + 23900;
	// 82916000: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82916004: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82916008: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8291600C: 38EA6910  addi r7, r10, 0x6910
	ctx.r[7].s64 = ctx.r[10].s64 + 26896;
	// 82916010: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82916014: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82916068 size=120
    let mut pc: u32 = 0x82916068;
    'dispatch: loop {
        match pc {
            0x82916068 => {
    //   block [0x82916068..0x829160E0)
	// 82916068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291606C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82916074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291607C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82916084: 4854361D  bl 0x82e596a0
	ctx.lr = 0x82916088;
	sub_82E596A0(ctx, base);
	// 82916088: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8291608C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82916090: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82916094: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82916098: 396A5D64  addi r11, r10, 0x5d64
	ctx.r[11].s64 = ctx.r[10].s64 + 23908;
	// 8291609C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 829160A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829160A4: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 829160A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829160AC: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 829160B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829160E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829160E0 size=100
    let mut pc: u32 = 0x829160E0;
    'dispatch: loop {
        match pc {
            0x829160E0 => {
    //   block [0x829160E0..0x82916144)
	// 829160E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829160E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829160E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829160EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829160F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829160F4: 4854358D  bl 0x82e59680
	ctx.lr = 0x829160F8;
	sub_82E59680(ctx, base);
	// 829160F8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829160FC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82916100: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82916104: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82916108: 394A5D6C  addi r10, r10, 0x5d6c
	ctx.r[10].s64 = ctx.r[10].s64 + 23916;
	// 8291610C: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82916110: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82916114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82916118: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8291611C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82916148 size=132
    let mut pc: u32 = 0x82916148;
    'dispatch: loop {
        match pc {
            0x82916148 => {
    //   block [0x82916148..0x829161CC)
	// 82916148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291614C: 48892019  bl 0x831a8164
	ctx.lr = 0x82916150;
	sub_831A8130(ctx, base);
	// 82916150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916158: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291615C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82916160: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82916164: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82916168: 48543519  bl 0x82e59680
	ctx.lr = 0x8291616C;
	sub_82E59680(ctx, base);
	// 8291616C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916170: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82916174: 396B5D74  addi r11, r11, 0x5d74
	ctx.r[11].s64 = ctx.r[11].s64 + 23924;
	// 82916178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291617C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916180: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829161D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829161D0 size=80
    let mut pc: u32 = 0x829161D0;
    'dispatch: loop {
        match pc {
            0x829161D0 => {
    //   block [0x829161D0..0x82916220)
	// 829161D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829161D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829161D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829161DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829161E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829161E4: 485434BD  bl 0x82e596a0
	ctx.lr = 0x829161E8;
	sub_82E596A0(ctx, base);
	// 829161E8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 829161EC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 829161F0: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 829161F4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 829161F8: 394A5D7C  addi r10, r10, 0x5d7c
	ctx.r[10].s64 = ctx.r[10].s64 + 23932;
	// 829161FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916200: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82916204: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82916220 size=80
    let mut pc: u32 = 0x82916220;
    'dispatch: loop {
        match pc {
            0x82916220 => {
    //   block [0x82916220..0x82916270)
	// 82916220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916228: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8291622C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916230: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916234: 4854346D  bl 0x82e596a0
	ctx.lr = 0x82916238;
	sub_82E596A0(ctx, base);
	// 82916238: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8291623C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82916240: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82916244: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82916248: 394A5D84  addi r10, r10, 0x5d84
	ctx.r[10].s64 = ctx.r[10].s64 + 23940;
	// 8291624C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916250: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82916254: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82916270 size=112
    let mut pc: u32 = 0x82916270;
    'dispatch: loop {
        match pc {
            0x82916270 => {
    //   block [0x82916270..0x829162E0)
	// 82916270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916274: 48891EF9  bl 0x831a816c
	ctx.lr = 0x82916278;
	sub_831A8130(ctx, base);
	// 82916278: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8291627C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916284: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82916288: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291628C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82916290: 485433F1  bl 0x82e59680
	ctx.lr = 0x82916294;
	sub_82E59680(ctx, base);
	// 82916294: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916298: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8291629C: 396B5D8C  addi r11, r11, 0x5d8c
	ctx.r[11].s64 = ctx.r[11].s64 + 23948;
	// 829162A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829162A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829162A8: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829162E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829162E0 size=112
    let mut pc: u32 = 0x829162E0;
    'dispatch: loop {
        match pc {
            0x829162E0 => {
    //   block [0x829162E0..0x82916350)
	// 829162E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829162E4: 48891E81  bl 0x831a8164
	ctx.lr = 0x829162E8;
	sub_831A8130(ctx, base);
	// 829162E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829162EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829162F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829162F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 829162F8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 829162FC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82916300: 48543381  bl 0x82e59680
	ctx.lr = 0x82916304;
	sub_82E59680(ctx, base);
	// 82916304: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916308: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8291630C: 396B5D94  addi r11, r11, 0x5d94
	ctx.r[11].s64 = ctx.r[11].s64 + 23956;
	// 82916310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916314: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916318: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82916350 size=92
    let mut pc: u32 = 0x82916350;
    'dispatch: loop {
        match pc {
            0x82916350 => {
    //   block [0x82916350..0x829163AC)
	// 82916350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916354: 48891E19  bl 0x831a816c
	ctx.lr = 0x82916358;
	sub_831A8130(ctx, base);
	// 82916358: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8291635C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916364: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82916368: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291636C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82916370: 48543331  bl 0x82e596a0
	ctx.lr = 0x82916374;
	sub_82E596A0(ctx, base);
	// 82916374: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916378: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8291637C: 396B5D9C  addi r11, r11, 0x5d9c
	ctx.r[11].s64 = ctx.r[11].s64 + 23964;
	// 82916380: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82916384: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916388: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8291638C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829163B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829163B0 size=104
    let mut pc: u32 = 0x829163B0;
    'dispatch: loop {
        match pc {
            0x829163B0 => {
    //   block [0x829163B0..0x82916418)
	// 829163B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829163B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829163B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829163BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829163C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829163C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829163C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 829163CC: 485432B5  bl 0x82e59680
	ctx.lr = 0x829163D0;
	sub_82E59680(ctx, base);
	// 829163D0: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 829163D4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 829163D8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 829163DC: 39295D6C  addi r9, r9, 0x5d6c
	ctx.r[9].s64 = ctx.r[9].s64 + 23916;
	// 829163E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829163E4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 829163E8: 13FE50C7  vcmpequd (lvx128) v31, v30, v10
	tmp.u32 = ctx.r[30].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916418 size=36
    let mut pc: u32 = 0x82916418;
    'dispatch: loop {
        match pc {
            0x82916418 => {
    //   block [0x82916418..0x8291643C)
	// 82916418: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8291641C: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82916420: 13E450C7  vcmpequd (lvx128) v31, v4, v10
	tmp.u32 = ctx.r[4].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916440 size=80
    let mut pc: u32 = 0x82916440;
    'dispatch: loop {
        match pc {
            0x82916440 => {
    //   block [0x82916440..0x82916490)
	// 82916440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291644C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82916450: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82916454: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 82916458: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 8291645C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82916460: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82916464: 484DBC65  bl 0x82df20c8
	ctx.lr = 0x82916468;
	sub_82DF20C8(ctx, base);
	// 82916468: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8291646C: 41820008  beq 0x82916474
	if ctx.cr[0].eq {
	pc = 0x82916474; continue 'dispatch;
	}
	// 82916470: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82916474: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82916478: 41820008  beq 0x82916480
	if ctx.cr[0].eq {
	pc = 0x82916480; continue 'dispatch;
	}
	// 8291647C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82916480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82916484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291648C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916490 size=88
    let mut pc: u32 = 0x82916490;
    'dispatch: loop {
        match pc {
            0x82916490 => {
    //   block [0x82916490..0x829164E8)
	// 82916490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916494: 48891CD1  bl 0x831a8164
	ctx.lr = 0x82916498;
	sub_831A8130(ctx, base);
	// 82916498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291649C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 829164A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 829164A4: 3BBC0008  addi r29, r28, 8
	ctx.r[29].s64 = ctx.r[28].s64 + 8;
	// 829164A8: 3BC00013  li r30, 0x13
	ctx.r[30].s64 = 19;
	// 829164AC: 3BFD0054  addi r31, r29, 0x54
	ctx.r[31].s64 = ctx.r[29].s64 + 84;
	// 829164B0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 829164B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829164B8: 484DCF71  bl 0x82df3428
	ctx.lr = 0x829164BC;
	sub_82DF3428(ctx, base);
	// 829164BC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 829164C0: 4080FFF0  bge 0x829164b0
	if !ctx.cr[0].lt {
	pc = 0x829164B0; continue 'dispatch;
	}
	// 829164C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 829164C8: 484DCF61  bl 0x82df3428
	ctx.lr = 0x829164CC;
	sub_82DF3428(ctx, base);
	// 829164CC: 576B07FF  clrlwi. r11, r27, 0x1f
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 829164D0: 4182000C  beq 0x829164dc
	if ctx.cr[0].eq {
	pc = 0x829164DC; continue 'dispatch;
	}
	// 829164D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 829164D8: 4B9A9D91  bl 0x822c0268
	ctx.lr = 0x829164DC;
	sub_822C0268(ctx, base);
	// 829164DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 829164E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 829164E4: 48891CD0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829164E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829164E8 size=112
    let mut pc: u32 = 0x829164E8;
    'dispatch: loop {
        match pc {
            0x829164E8 => {
    //   block [0x829164E8..0x82916558)
	// 829164E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829164EC: 48891C7D  bl 0x831a8168
	ctx.lr = 0x829164F0;
	sub_831A8130(ctx, base);
	// 829164F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829164F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 829164F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 829164FC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82916500: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82916504: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916508: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8291650C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82916510: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82916514: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82916518: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8291651C: 419A0034  beq cr6, 0x82916550
	if ctx.cr[6].eq {
	pc = 0x82916550; continue 'dispatch;
	}
	// 82916520: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82916524: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82916528: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291652C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916530: 4BFFFF61  bl 0x82916490
	ctx.lr = 0x82916534;
	sub_82916490(ctx, base);
	// 82916534: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82916538: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8291653C: 484DBC4D  bl 0x82df2188
	ctx.lr = 0x82916540;
	sub_82DF2188(ctx, base);
	// 82916540: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82916544: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82916548: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8291654C: 409AFFD8  bne cr6, 0x82916524
	if !ctx.cr[6].eq {
	pc = 0x82916524; continue 'dispatch;
	}
	// 82916550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82916554: 48891C64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916558 size=120
    let mut pc: u32 = 0x82916558;
    'dispatch: loop {
        match pc {
            0x82916558 => {
    //   block [0x82916558..0x829165D0)
	// 82916558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82916564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291656C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916570: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82916574: 4854312D  bl 0x82e596a0
	ctx.lr = 0x82916578;
	sub_82E596A0(ctx, base);
	// 82916578: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8291657C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82916580: 396B5DA4  addi r11, r11, 0x5da4
	ctx.r[11].s64 = ctx.r[11].s64 + 23972;
	// 82916584: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82916588: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8291658C: 484DD675  bl 0x82df3c00
	ctx.lr = 0x82916590;
	sub_82DF3C00(ctx, base);
	// 82916590: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82916594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82916598: 4BFFFEA9  bl 0x82916440
	ctx.lr = 0x8291659C;
	sub_82916440(ctx, base);
	// 8291659C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829165A0: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 829165A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 829165A8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 829165AC: 997F0028  stb r11, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 829165B0: 4BFFFF39  bl 0x829164e8
	ctx.lr = 0x829165B4;
	sub_829164E8(ctx, base);
	// 829165B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829165B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829165BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829165C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829165C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 829165C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829165CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829165D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829165D0 size=88
    let mut pc: u32 = 0x829165D0;
    'dispatch: loop {
        match pc {
            0x829165D0 => {
    //   block [0x829165D0..0x82916628)
	// 829165D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829165D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829165D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829165DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829165E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829165E4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 829165E8: 4BFFFF01  bl 0x829164e8
	ctx.lr = 0x829165EC;
	sub_829164E8(ctx, base);
	// 829165EC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 829165F0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 829165F4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 829165F8: 484DBB91  bl 0x82df2188
	ctx.lr = 0x829165FC;
	sub_82DF2188(ctx, base);
	// 829165FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82916600: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82916604: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82916608: 484DCE21  bl 0x82df3428
	ctx.lr = 0x8291660C;
	sub_82DF3428(ctx, base);
	// 8291660C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916610: 48542FC9  bl 0x82e595d8
	ctx.lr = 0x82916614;
	sub_82E595D8(ctx, base);
	// 82916614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82916618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291661C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82916620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916628 size=76
    let mut pc: u32 = 0x82916628;
    'dispatch: loop {
        match pc {
            0x82916628 => {
    //   block [0x82916628..0x82916674)
	// 82916628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291662C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82916634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291663C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916640: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82916644: 4BFFFF8D  bl 0x829165d0
	ctx.lr = 0x82916648;
	sub_829165D0(ctx, base);
	// 82916648: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291664C: 4182000C  beq 0x82916658
	if ctx.cr[0].eq {
	pc = 0x82916658; continue 'dispatch;
	}
	// 82916650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916654: 484DBD85  bl 0x82df23d8
	ctx.lr = 0x82916658;
	sub_82DF23D8(ctx, base);
	// 82916658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8291665C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82916660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82916668: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291666C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916678 size=8
    let mut pc: u32 = 0x82916678;
    'dispatch: loop {
        match pc {
            0x82916678 => {
    //   block [0x82916678..0x82916680)
	// 82916678: 38630090  addi r3, r3, 0x90
	ctx.r[3].s64 = ctx.r[3].s64 + 144;
	// 8291667C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916680 size=40
    let mut pc: u32 = 0x82916680;
    'dispatch: loop {
        match pc {
            0x82916680 => {
    //   block [0x82916680..0x829166A8)
	// 82916680: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82916684: 39430090  addi r10, r3, 0x90
	ctx.r[10].s64 = ctx.r[3].s64 + 144;
	// 82916688: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8291668C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82916690: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82916694: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82916698: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8291669C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 829166A0: 4200FFF0  bdnz 0x82916690
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82916690; continue 'dispatch;
	}
	// 829166A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829166A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829166A8 size=52
    let mut pc: u32 = 0x829166A8;
    'dispatch: loop {
        match pc {
            0x829166A8 => {
    //   block [0x829166A8..0x829166DC)
	// 829166A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829166AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829166B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829166B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829166B8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 829166BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829166C0: 484DD541  bl 0x82df3c00
	ctx.lr = 0x829166C4;
	sub_82DF3C00(ctx, base);
	// 829166C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829166C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829166CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829166D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829166D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 829166D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829166E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x829166E0 size=68
    let mut pc: u32 = 0x829166E0;
    'dispatch: loop {
        match pc {
            0x829166E0 => {
    //   block [0x829166E0..0x82916724)
	// 829166E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829166E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829166E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829166EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829166F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829166F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 829166F8: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 829166FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82916700: 4E800421  bctrl
	ctx.lr = 0x82916704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82916704: 4BED3BA5  bl 0x827ea2a8
	ctx.lr = 0x82916708;
	sub_827EA2A8(ctx, base);
	// 82916708: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8291670C: 484DCBFD  bl 0x82df3308
	ctx.lr = 0x82916710;
	sub_82DF3308(ctx, base);
	// 82916710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82916714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291671C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916728 size=64
    let mut pc: u32 = 0x82916728;
    'dispatch: loop {
        match pc {
            0x82916728 => {
    //   block [0x82916728..0x82916768)
	// 82916728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291672C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916730: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916734: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916738: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291673C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82916740: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82916744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82916748: 4E800421  bctrl
	ctx.lr = 0x8291674C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8291674C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82916750: 4BED3D91  bl 0x827ea4e0
	ctx.lr = 0x82916754;
	sub_827EA4E0(ctx, base);
	// 82916754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82916758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8291675C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82916760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916768 size=4
    let mut pc: u32 = 0x82916768;
    'dispatch: loop {
        match pc {
            0x82916768 => {
    //   block [0x82916768..0x8291676C)
	// 82916768: 4BED70B0  b 0x827ed818
	sub_827ED818(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916770 size=8
    let mut pc: u32 = 0x82916770;
    'dispatch: loop {
        match pc {
            0x82916770 => {
    //   block [0x82916770..0x82916778)
	// 82916770: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82916774: 4BEDEC5C  b 0x827f53d0
	sub_827F53D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916778 size=8
    let mut pc: u32 = 0x82916778;
    'dispatch: loop {
        match pc {
            0x82916778 => {
    //   block [0x82916778..0x82916780)
	// 82916778: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 8291677C: 4BC746EC  b 0x8258ae68
	sub_8258AE68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916780 size=8
    let mut pc: u32 = 0x82916780;
    'dispatch: loop {
        match pc {
            0x82916780 => {
    //   block [0x82916780..0x82916788)
	// 82916780: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82916784: 4BB70334  b 0x82486ab8
	sub_82486AB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916788 size=8
    let mut pc: u32 = 0x82916788;
    'dispatch: loop {
        match pc {
            0x82916788 => {
    //   block [0x82916788..0x82916790)
	// 82916788: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 8291678C: 4BC74724  b 0x8258aeb0
	sub_8258AEB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916790 size=36
    let mut pc: u32 = 0x82916790;
    'dispatch: loop {
        match pc {
            0x82916790 => {
    //   block [0x82916790..0x829167B4)
	// 82916790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291679C: 4BED707D  bl 0x827ed818
	ctx.lr = 0x829167A0;
	sub_827ED818(ctx, base);
	// 829167A0: 4BFF26B9  bl 0x82908e58
	ctx.lr = 0x829167A4;
	sub_82908E58(ctx, base);
	// 829167A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 829167A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829167AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829167B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829167B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829167B8 size=12
    let mut pc: u32 = 0x829167B8;
    'dispatch: loop {
        match pc {
            0x829167B8 => {
    //   block [0x829167B8..0x829167C4)
	// 829167B8: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 829167BC: 386B00AC  addi r3, r11, 0xac
	ctx.r[3].s64 = ctx.r[11].s64 + 172;
	// 829167C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829167C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829167C8 size=128
    let mut pc: u32 = 0x829167C8;
    'dispatch: loop {
        match pc {
            0x829167C8 => {
    //   block [0x829167C8..0x82916848)
	// 829167C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829167CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 829167D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 829167D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829167D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829167DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829167E0: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 829167E4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829167E8: 386B00AC  addi r3, r11, 0xac
	ctx.r[3].s64 = ctx.r[11].s64 + 172;
	// 829167EC: 48891D25  bl 0x831a8510
	ctx.lr = 0x829167F0;
	sub_831A8510(ctx, base);
	// 829167F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829167F4: 4BED8FA5  bl 0x827ef798
	ctx.lr = 0x829167F8;
	sub_827EF798(ctx, base);
	// 829167F8: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829167FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82916800: 388B00AC  addi r4, r11, 0xac
	ctx.r[4].s64 = ctx.r[11].s64 + 172;
	// 82916804: 4BEDBA05  bl 0x827f2208
	ctx.lr = 0x82916808;
	sub_827F2208(ctx, base);
	// 82916808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8291680C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916810: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82916814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82916818: 4E800421  bctrl
	ctx.lr = 0x8291681C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8291681C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82916820: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82916824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82916828: C02B0000  lfs f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8291682C: 4BEDB995  bl 0x827f21c0
	ctx.lr = 0x82916830;
	sub_827F21C0(ctx, base);
	// 82916830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82916834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8291683C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82916840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916848 size=20
    let mut pc: u32 = 0x82916848;
    'dispatch: loop {
        match pc {
            0x82916848 => {
    //   block [0x82916848..0x8291685C)
	// 82916848: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 8291684C: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82916850: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916860 size=36
    let mut pc: u32 = 0x82916860;
    'dispatch: loop {
        match pc {
            0x82916860 => {
    //   block [0x82916860..0x82916884)
	// 82916860: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916864: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82916868: D02B00A4  stfs f1, 0xa4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8291686C: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916870: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916888 size=20
    let mut pc: u32 = 0x82916888;
    'dispatch: loop {
        match pc {
            0x82916888 => {
    //   block [0x82916888..0x8291689C)
	// 82916888: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 8291688C: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82916890: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829168A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x829168A0 size=20
    let mut pc: u32 = 0x829168A0;
    'dispatch: loop {
        match pc {
            0x829168A0 => {
    //   block [0x829168A0..0x829168B4)
	// 829168A0: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 829168A4: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 829168A8: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829168B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829168B8 size=12
    let mut pc: u32 = 0x829168B8;
    'dispatch: loop {
        match pc {
            0x829168B8 => {
    //   block [0x829168B8..0x829168C4)
	// 829168B8: 816300B4  lwz r11, 0xb4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 829168BC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829168C0: 4B9D4CF0  b 0x822eb5b0
	sub_822EB5B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829168C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829168C8 size=12
    let mut pc: u32 = 0x829168C8;
    'dispatch: loop {
        match pc {
            0x829168C8 => {
    //   block [0x829168C8..0x829168D4)
	// 829168C8: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 829168CC: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829168D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829168D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829168D8 size=12
    let mut pc: u32 = 0x829168D8;
    'dispatch: loop {
        match pc {
            0x829168D8 => {
    //   block [0x829168D8..0x829168E4)
	// 829168D8: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 829168DC: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 829168E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829168E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x829168E8 size=12
    let mut pc: u32 = 0x829168E8;
    'dispatch: loop {
        match pc {
            0x829168E8 => {
    //   block [0x829168E8..0x829168F4)
	// 829168E8: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 829168EC: 988B0001  stb r4, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 829168F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829168F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x829168F8 size=12
    let mut pc: u32 = 0x829168F8;
    'dispatch: loop {
        match pc {
            0x829168F8 => {
    //   block [0x829168F8..0x82916904)
	// 829168F8: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 829168FC: C02B00A4  lfs f1, 0xa4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82916900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916908 size=12
    let mut pc: u32 = 0x82916908;
    'dispatch: loop {
        match pc {
            0x82916908 => {
    //   block [0x82916908..0x82916914)
	// 82916908: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8291690C: 886B00A8  lbz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82916910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916918 size=20
    let mut pc: u32 = 0x82916918;
    'dispatch: loop {
        match pc {
            0x82916918 => {
    //   block [0x82916918..0x8291692C)
	// 82916918: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 8291691C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82916920: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916930 size=20
    let mut pc: u32 = 0x82916930;
    'dispatch: loop {
        match pc {
            0x82916930 => {
    //   block [0x82916930..0x82916944)
	// 82916930: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916934: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82916938: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916948 size=12
    let mut pc: u32 = 0x82916948;
    'dispatch: loop {
        match pc {
            0x82916948 => {
    //   block [0x82916948..0x82916954)
	// 82916948: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8291694C: D02B0008  stfs f1, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82916950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916958 size=12
    let mut pc: u32 = 0x82916958;
    'dispatch: loop {
        match pc {
            0x82916958 => {
    //   block [0x82916958..0x82916964)
	// 82916958: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8291695C: C02B0008  lfs f1, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82916960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916968 size=40
    let mut pc: u32 = 0x82916968;
    'dispatch: loop {
        match pc {
            0x82916968 => {
    //   block [0x82916968..0x82916990)
	// 82916968: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8291696C: C00B00B4  lfs f0, 0xb4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82916970: C1AB00B8  lfs f13, 0xb8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82916974: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82916978: 40990008  ble cr6, 0x82916980
	if !ctx.cr[6].gt {
	pc = 0x82916980; continue 'dispatch;
	}
	// 8291697C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82916980: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82916984: C1ABCEE4  lfs f13, -0x311c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82916988: EC200372  fmuls f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8291698C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82916990 size=316
    let mut pc: u32 = 0x82916990;
    'dispatch: loop {
        match pc {
            0x82916990 => {
    //   block [0x82916990..0x82916ACC)
	// 82916990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916998: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291699C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 829169A0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 829169A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 829169A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 829169AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 829169B0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829169B4: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829169B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 829169BC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 829169C0: 40990008  ble cr6, 0x829169c8
	if !ctx.cr[6].gt {
	pc = 0x829169C8; continue 'dispatch;
	}
	// 829169C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 829169C8: 815F00A4  lwz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829169CC: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 829169D0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829169D4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 829169D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 829169DC: 418200A4  beq 0x82916a80
	if ctx.cr[0].eq {
	pc = 0x82916A80; continue 'dispatch;
	}
	// 829169E0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829169E4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 829169E8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 829169EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829169F0: D02B000C  stfs f1, 0xc(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 829169F4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829169F8: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916AD0 size=12
    let mut pc: u32 = 0x82916AD0;
    'dispatch: loop {
        match pc {
            0x82916AD0 => {
    //   block [0x82916AD0..0x82916ADC)
	// 82916AD0: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916AD4: C02B000C  lfs f1, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82916AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916AE0 size=12
    let mut pc: u32 = 0x82916AE0;
    'dispatch: loop {
        match pc {
            0x82916AE0 => {
    //   block [0x82916AE0..0x82916AEC)
	// 82916AE0: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916AE4: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82916AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916AF0 size=12
    let mut pc: u32 = 0x82916AF0;
    'dispatch: loop {
        match pc {
            0x82916AF0 => {
    //   block [0x82916AF0..0x82916AFC)
	// 82916AF0: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916AF4: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82916AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916B00 size=32
    let mut pc: u32 = 0x82916B00;
    'dispatch: loop {
        match pc {
            0x82916B00 => {
    //   block [0x82916B00..0x82916B20)
	// 82916B00: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916B04: 816B0104  lwz r11, 0x104(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82916B08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82916B0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82916B10: 41990008  bgt cr6, 0x82916b18
	if ctx.cr[6].gt {
	pc = 0x82916B18; continue 'dispatch;
	}
	// 82916B14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82916B18: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82916B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916B20 size=20
    let mut pc: u32 = 0x82916B20;
    'dispatch: loop {
        match pc {
            0x82916B20 => {
    //   block [0x82916B20..0x82916B34)
	// 82916B20: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916B24: 814B0104  lwz r10, 0x104(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82916B28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82916B2C: 914B0104  stw r10, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 82916B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916B38 size=32
    let mut pc: u32 = 0x82916B38;
    'dispatch: loop {
        match pc {
            0x82916B38 => {
    //   block [0x82916B38..0x82916B58)
	// 82916B38: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916B3C: 814B0104  lwz r10, 0x104(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82916B40: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82916B44: 914B0104  stw r10, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 82916B48: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916B4C: 816B0104  lwz r11, 0x104(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82916B50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82916B54: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916B58 size=16
    let mut pc: u32 = 0x82916B58;
    'dispatch: loop {
        match pc {
            0x82916B58 => {
    //   block [0x82916B58..0x82916B68)
	// 82916B58: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916B5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82916B60: 914B0104  stw r10, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 82916B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916B68 size=108
    let mut pc: u32 = 0x82916B68;
    'dispatch: loop {
        match pc {
            0x82916B68 => {
    //   block [0x82916B68..0x82916BD4)
	// 82916B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916B70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916B74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916B78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916B7C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916B80: 396B5DC4  addi r11, r11, 0x5dc4
	ctx.r[11].s64 = ctx.r[11].s64 + 24004;
	// 82916B84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916B88: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82916B8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82916B90: 419A0008  beq cr6, 0x82916b98
	if ctx.cr[6].eq {
	pc = 0x82916B98; continue 'dispatch;
	}
	// 82916B94: 4B9A9CFD  bl 0x822c0890
	ctx.lr = 0x82916B98;
	sub_822C0890(ctx, base);
	// 82916B98: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82916B9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82916BA0: 419A0008  beq cr6, 0x82916ba8
	if ctx.cr[6].eq {
	pc = 0x82916BA8; continue 'dispatch;
	}
	// 82916BA4: 4B9A9CED  bl 0x822c0890
	ctx.lr = 0x82916BA8;
	sub_822C0890(ctx, base);
	// 82916BA8: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82916BAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82916BB0: 419A0008  beq cr6, 0x82916bb8
	if ctx.cr[6].eq {
	pc = 0x82916BB8; continue 'dispatch;
	}
	// 82916BB4: 4B9A9CDD  bl 0x822c0890
	ctx.lr = 0x82916BB8;
	sub_822C0890(ctx, base);
	// 82916BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916BBC: 4BED8BE5  bl 0x827ef7a0
	ctx.lr = 0x82916BC0;
	sub_827EF7A0(ctx, base);
	// 82916BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82916BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82916BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82916BD8 size=104
    let mut pc: u32 = 0x82916BD8;
    'dispatch: loop {
        match pc {
            0x82916BD8 => {
    //   block [0x82916BD8..0x82916C40)
	// 82916BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916BE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916BE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916BE8: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82916BEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82916BF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82916BF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82916BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82916BFC: 4E800421  bctrl
	ctx.lr = 0x82916C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82916C00: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82916C04: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82916C08: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82916C0C: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82916C10: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82916C14: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82916C18: 138B1C07  vcmpneb. (lvlx128) v28, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916C40 size=160
    let mut pc: u32 = 0x82916C40;
    'dispatch: loop {
        match pc {
            0x82916C40 => {
    //   block [0x82916C40..0x82916CE0)
	// 82916C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82916C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916C50: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82916C54: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82916CE0 size=36
    let mut pc: u32 = 0x82916CE0;
    'dispatch: loop {
        match pc {
            0x82916CE0 => {
    //   block [0x82916CE0..0x82916D04)
	// 82916CE0: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916CE4: 394000A0  li r10, 0xa0
	ctx.r[10].s64 = 160;
	// 82916CE8: 392000D0  li r9, 0xd0
	ctx.r[9].s64 = 208;
	// 82916CEC: 13EB5407  vcmpneb. (lvlx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916D08 size=76
    let mut pc: u32 = 0x82916D08;
    'dispatch: loop {
        match pc {
            0x82916D08 => {
    //   block [0x82916D08..0x82916D54)
	// 82916D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916D10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82916D14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916D18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916D20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82916D24: 4BFFFE45  bl 0x82916b68
	ctx.lr = 0x82916D28;
	sub_82916B68(ctx, base);
	// 82916D28: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82916D2C: 4182000C  beq 0x82916d38
	if ctx.cr[0].eq {
	pc = 0x82916D38; continue 'dispatch;
	}
	// 82916D30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916D34: 4B9A9535  bl 0x822c0268
	ctx.lr = 0x82916D38;
	sub_822C0268(ctx, base);
	// 82916D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82916D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82916D48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82916D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916D58 size=24
    let mut pc: u32 = 0x82916D58;
    'dispatch: loop {
        match pc {
            0x82916D58 => {
    //   block [0x82916D58..0x82916D70)
	// 82916D58: 816400AC  lwz r11, 0xac(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82916D5C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916D60: 816400B0  lwz r11, 0xb0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(176 as u32) ) } as u64;
	// 82916D64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82916D68: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82916D6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916D70 size=36
    let mut pc: u32 = 0x82916D70;
    'dispatch: loop {
        match pc {
            0x82916D70 => {
    //   block [0x82916D70..0x82916D94)
	// 82916D70: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82916D74: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82916D78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82916D7C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82916D80: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82916D84: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82916D88: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82916D8C: 4082FFE8  bne 0x82916d74
	if !ctx.cr[0].eq {
	pc = 0x82916D74; continue 'dispatch;
	}
	// 82916D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916D98 size=24
    let mut pc: u32 = 0x82916D98;
    'dispatch: loop {
        match pc {
            0x82916D98 => {
    //   block [0x82916D98..0x82916DB0)
	// 82916D98: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 82916D9C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916DA0: 816400A8  lwz r11, 0xa8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) } as u64;
	// 82916DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82916DA8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82916DAC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82916DB0 size=36
    let mut pc: u32 = 0x82916DB0;
    'dispatch: loop {
        match pc {
            0x82916DB0 => {
    //   block [0x82916DB0..0x82916DD4)
	// 82916DB0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82916DB4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82916DB8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82916DBC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82916DC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82916DC4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82916DC8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82916DCC: 4082FFE8  bne 0x82916db4
	if !ctx.cr[0].eq {
	pc = 0x82916DB4; continue 'dispatch;
	}
	// 82916DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916DD8 size=188
    let mut pc: u32 = 0x82916DD8;
    'dispatch: loop {
        match pc {
            0x82916DD8 => {
    //   block [0x82916DD8..0x82916E94)
	// 82916DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82916DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916DEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82916DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82916DF4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82916DF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82916DFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916E00: 4B9A9B39  bl 0x822c0938
	ctx.lr = 0x82916E04;
	sub_822C0938(ctx, base);
	// 82916E04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82916E08: 41820028  beq 0x82916e30
	if ctx.cr[0].eq {
	pc = 0x82916E30; continue 'dispatch;
	}
	// 82916E0C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916E10: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82916E14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82916E18: 392B5DAC  addi r9, r11, 0x5dac
	ctx.r[9].s64 = ctx.r[11].s64 + 23980;
	// 82916E1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82916E20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82916E24: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82916E28: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82916E2C: 48000008  b 0x82916e34
	pc = 0x82916E34; continue 'dispatch;
	// 82916E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82916E34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916E38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82916E3C: 409A003C  bne cr6, 0x82916e78
	if !ctx.cr[6].eq {
	pc = 0x82916E78; continue 'dispatch;
	}
	// 82916E40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82916E44: 419A0014  beq cr6, 0x82916e58
	if ctx.cr[6].eq {
	pc = 0x82916E58; continue 'dispatch;
	}
	// 82916E48: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82916E4C: 484DC5DD  bl 0x82df3428
	ctx.lr = 0x82916E50;
	sub_82DF3428(ctx, base);
	// 82916E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916E54: 4B9A9415  bl 0x822c0268
	ctx.lr = 0x82916E58;
	sub_822C0268(ctx, base);
	// 82916E58: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82916E5C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82916E60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82916E64: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82916E68: 816BE950  lwz r11, -0x16b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5808 as u32) ) } as u64;
	// 82916E6C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82916E70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82916E74: 4B9A918D  bl 0x822c0000
	ctx.lr = 0x82916E78;
	sub_822C0000(ctx, base);
	// 82916E78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82916E7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82916E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82916E88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82916E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916E98 size=64
    let mut pc: u32 = 0x82916E98;
    'dispatch: loop {
        match pc {
            0x82916E98 => {
    //   block [0x82916E98..0x82916ED8)
	// 82916E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82916EA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82916EA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916EA8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82916EAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82916EB0: 419A0014  beq cr6, 0x82916ec4
	if ctx.cr[6].eq {
	pc = 0x82916EC4; continue 'dispatch;
	}
	// 82916EB4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82916EB8: 484DC571  bl 0x82df3428
	ctx.lr = 0x82916EBC;
	sub_82DF3428(ctx, base);
	// 82916EBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82916EC0: 4B9A93A9  bl 0x822c0268
	ctx.lr = 0x82916EC4;
	sub_822C0268(ctx, base);
	// 82916EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82916EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82916ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82916ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82916ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82916ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82916ED8 size=324
    let mut pc: u32 = 0x82916ED8;
    'dispatch: loop {
        match pc {
            0x82916ED8 => {
    //   block [0x82916ED8..0x8291701C)
	// 82916ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82916EDC: 48891285  bl 0x831a8160
	ctx.lr = 0x82916EE0;
	sub_831A8130(ctx, base);
	// 82916EE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82916EE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82916EE8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82916EEC: 4BED8B75  bl 0x827efa60
	ctx.lr = 0x82916EF0;
	sub_827EFA60(ctx, base);
	// 82916EF0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916EF4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82916EF8: 396B5DC4  addi r11, r11, 0x5dc4
	ctx.r[11].s64 = ctx.r[11].s64 + 24004;
	// 82916EFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82916F00: 4800A479  bl 0x82921378
	ctx.lr = 0x82916F04;
	sub_82921378(ctx, base);
	// 82916F04: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82916F08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82916F0C: 3B8B5E60  addi r28, r11, 0x5e60
	ctx.r[28].s64 = ctx.r[11].s64 + 24160;
	// 82916F10: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82916F14: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82916F18: 38600110  li r3, 0x110
	ctx.r[3].s64 = 272;
	// 82916F1C: 4B9A94BD  bl 0x822c03d8
	ctx.lr = 0x82916F20;
	sub_822C03D8(ctx, base);
	// 82916F20: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82916F24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82916F28: 41820010  beq 0x82916f38
	if ctx.cr[0].eq {
	pc = 0x82916F38; continue 'dispatch;
	}
	// 82916F2C: 480001ED  bl 0x82917118
	ctx.lr = 0x82916F30;
	sub_82917118(ctx, base);
	// 82916F30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82916F34: 48000008  b 0x82916f3c
	pc = 0x82916F3C; continue 'dispatch;
	// 82916F38: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82916F3C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82916F40: 397F00A4  addi r11, r31, 0xa4
	ctx.r[11].s64 = ctx.r[31].s64 + 164;
	// 82916F44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82916F48: 3BAB0004  addi r29, r11, 4
	ctx.r[29].s64 = ctx.r[11].s64 + 4;
	// 82916F4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82916F50: 4BFFFE89  bl 0x82916dd8
	ctx.lr = 0x82916F54;
	sub_82916DD8(ctx, base);
	// 82916F54: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82916F58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82916F5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82916F60: 4B9A90A1  bl 0x822c0000
	ctx.lr = 0x82916F64;
	sub_822C0000(ctx, base);
	// 82916F64: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82916F68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82916F6C: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82916F70: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82916F74: 484DB475  bl 0x82df23e8
	ctx.lr = 0x82916F78;
	sub_82DF23E8(ctx, base);
	// 82916F78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82916F7C: 41820010  beq 0x82916f8c
	if ctx.cr[0].eq {
	pc = 0x82916F8C; continue 'dispatch;
	}
	// 82916F80: 4BC73D19  bl 0x8258ac98
	ctx.lr = 0x82916F84;
	sub_8258AC98(ctx, base);
	// 82916F84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82916F88: 48000008  b 0x82916f90
	pc = 0x82916F90; continue 'dispatch;
	// 82916F8C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82916F90: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82916F94: 397F00AC  addi r11, r31, 0xac
	ctx.r[11].s64 = ctx.r[31].s64 + 172;
	// 82916F98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82916F9C: 3BAB0004  addi r29, r11, 4
	ctx.r[29].s64 = ctx.r[11].s64 + 4;
	// 82916FA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82916FA4: 4B9EF9ED  bl 0x82306990
	ctx.lr = 0x82916FA8;
	sub_82306990(ctx, base);
	// 82916FA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82916FAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82916FB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82916FB4: 4B9A904D  bl 0x822c0000
	ctx.lr = 0x82916FB8;
	sub_822C0000(ctx, base);
	// 82916FB8: 937F00B4  stw r27, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 82916FBC: 937F00B8  stw r27, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[27].u32 ) };
	// 82916FC0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82916FC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82916FC8: 3BDF00B4  addi r30, r31, 0xb4
	ctx.r[30].s64 = ctx.r[31].s64 + 180;
	// 82916FCC: 4BBF854D  bl 0x8250f518
	ctx.lr = 0x82916FD0;
	sub_8250F518(ctx, base);
	// 82916FD0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82916FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82916FD8: 38A0002B  li r5, 0x2b
	ctx.r[5].s64 = 43;
	// 82916FDC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82916FE0: 4B9A93F9  bl 0x822c03d8
	ctx.lr = 0x82916FE4;
	sub_822C03D8(ctx, base);
	// 82916FE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82916FE8: 41820014  beq 0x82916ffc
	if ctx.cr[0].eq {
	pc = 0x82916FFC; continue 'dispatch;
	}
	// 82916FEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82916FF0: 4BB7A101  bl 0x824910f0
	ctx.lr = 0x82916FF4;
	sub_824910F0(ctx, base);
	// 82916FF4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82916FF8: 48000008  b 0x82917000
	pc = 0x82917000; continue 'dispatch;
	// 82916FFC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82917000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82917004: 4B9F36DD  bl 0x8230a6e0
	ctx.lr = 0x82917008;
	sub_8230A6E0(ctx, base);
	// 82917008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8291700C: 484DAC85  bl 0x82df1c90
	ctx.lr = 0x82917010;
	sub_82DF1C90(ctx, base);
	// 82917010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82917014: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82917018: 48891198  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82917020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82917020 size=12
    let mut pc: u32 = 0x82917020;
    'dispatch: loop {
        match pc {
            0x82917020 => {
    //   block [0x82917020..0x8291702C)
	// 82917020: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82917024: 386B0100  addi r3, r11, 0x100
	ctx.r[3].s64 = ctx.r[11].s64 + 256;
	// 82917028: 4BABCFC8  b 0x823d3ff0
	sub_823D3FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82917030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82917030 size=12
    let mut pc: u32 = 0x82917030;
    'dispatch: loop {
        match pc {
            0x82917030 => {
    //   block [0x82917030..0x8291703C)
	// 82917030: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82917034: 386B0100  addi r3, r11, 0x100
	ctx.r[3].s64 = ctx.r[11].s64 + 256;
	// 82917038: 4BABD018  b 0x823d4050
	sub_823D4050(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82917040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82917040 size=212
    let mut pc: u32 = 0x82917040;
    'dispatch: loop {
        match pc {
            0x82917040 => {
    //   block [0x82917040..0x82917114)
	// 82917040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82917044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82917048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8291704C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82917050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82917054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82917058: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291705C: 4BED67BD  bl 0x827ed818
	ctx.lr = 0x82917060;
	sub_827ED818(ctx, base);
	// 82917060: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82917064: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82917068: 816B014C  lwz r11, 0x14c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) } as u64;
	// 8291706C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82917070: 4E800421  bctrl
	ctx.lr = 0x82917074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82917074: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82917078: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8291707C: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82917080: 386B0100  addi r3, r11, 0x100
	ctx.r[3].s64 = ctx.r[11].s64 + 256;
	// 82917084: 4BABCFCD  bl 0x823d4050
	ctx.lr = 0x82917088;
	sub_823D4050(ctx, base);
	// 82917088: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8291708C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82917090: 41820014  beq 0x829170a4
	if ctx.cr[0].eq {
	pc = 0x829170A4; continue 'dispatch;
	}
	// 82917094: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82917098: 914B00F4  stw r10, 0xf4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 8291709C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 829170A0: 48000010  b 0x829170b0
	pc = 0x829170B0; continue 'dispatch;
	// 829170A4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 829170A8: 914B00F4  stw r10, 0xf4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 829170AC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 829170B0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829170B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 829170B8: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 829170BC: 4BED675D  bl 0x827ed818
	ctx.lr = 0x829170C0;
	sub_827ED818(ctx, base);
	// 829170C0: 4BFF1D29  bl 0x82908de8
	ctx.lr = 0x829170C4;
	sub_82908DE8(ctx, base);
	// 829170C4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 829170C8: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 829170CC: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 829170D0: 394AB298  addi r10, r10, -0x4d68
	ctx.r[10].s64 = ctx.r[10].s64 + -19816;
	// 829170D4: 810B00F4  lwz r8, 0xf4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 829170D8: 7C09542E  lfsx f0, r9, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829170DC: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 829170E0: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 829170E4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 829170E8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 829170EC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 829170F0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 829170F4: 398000F4  li r12, 0xf4
	ctx.r[12].s64 = 244;
	// 829170F8: 7C0B67AE  stfiwx f0, r11, r12
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[12].u32), tmp.u32) };
	// 829170FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82917100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82917104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82917108: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8291710C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82917110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82917118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82917118 size=320
    let mut pc: u32 = 0x82917118;
    'dispatch: loop {
        match pc {
            0x82917118 => {
    //   block [0x82917118..0x82917258)
	// 82917118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291711C: 4889104D  bl 0x831a8168
	ctx.lr = 0x82917120;
	sub_831A8130(ctx, base);
	// 82917120: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82917124: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82917128: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8291712C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82917130: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82917134: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82917138: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8291713C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82917140: 3B8B6910  addi r28, r11, 0x6910
	ctx.r[28].s64 = ctx.r[11].s64 + 26896;
	// 82917144: C3EA08A8  lfs f31, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82917148: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8291714C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82917150: C009964C  lfs f0, -0x69b4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82917154: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82917158: C3C808A4  lfs f30, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8291715C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82917160: D3FF0004  stfs f31, 4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82917164: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82917168: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 8291716C: D3DF000C  stfs f30, 0xc(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82917170: 9BBF0001  stb r29, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 82917174: D3DF0010  stfs f30, 0x10(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82917178: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 8291717C: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82917180: 39000060  li r8, 0x60
	ctx.r[8].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82917258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82917258 size=68
    let mut pc: u32 = 0x82917258;
    'dispatch: loop {
        match pc {
            0x82917258 => {
    //   block [0x82917258..0x8291729C)
	// 82917258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8291725C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82917260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82917264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82917268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8291726C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82917270: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82917274: 396B5EBC  addi r11, r11, 0x5ebc
	ctx.r[11].s64 = ctx.r[11].s64 + 24252;
	// 82917278: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8291727C: 41820008  beq 0x82917284
	if ctx.cr[0].eq {
	pc = 0x82917284; continue 'dispatch;
	}
	// 82917280: 4B9A8FE9  bl 0x822c0268
	ctx.lr = 0x82917284;
	sub_822C0268(ctx, base);
	// 82917284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82917288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8291728C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82917290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82917294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82917298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829172A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x829172A0 size=88
    let mut pc: u32 = 0x829172A0;
    'dispatch: loop {
        match pc {
            0x829172A0 => {
    //   block [0x829172A0..0x829172F8)
	// 829172A0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 829172A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 829172A8: 396B5EBC  addi r11, r11, 0x5ebc
	ctx.r[11].s64 = ctx.r[11].s64 + 24252;
	// 829172AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 829172B0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 829172B4: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 829172B8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 829172BC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 829172C0: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829172C4: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 829172C8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 829172CC: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 829172D0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 829172D4: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 829172D8: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 829172DC: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 829172E0: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 829172E4: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 829172E8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 829172EC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 829172F0: 91230028  stw r9, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 829172F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_829172F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x829172F8 size=276
    let mut pc: u32 = 0x829172F8;
    'dispatch: loop {
        match pc {
            0x829172F8 => {
    //   block [0x829172F8..0x8291740C)
	// 829172F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 829172FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82917300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82917304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82917308: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8291730C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82917310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82917314: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82917318: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8291731C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82917320: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82917324: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82917328: 41820060  beq 0x82917388
	if ctx.cr[0].eq {
	pc = 0x82917388; continue 'dispatch;
	}
	// 8291732C: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82917330: C1BF001C  lfs f13, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82917334: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82917338: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8291733C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82917340: 41990024  bgt cr6, 0x82917364
	if ctx.cr[6].gt {
	pc = 0x82917364; continue 'dispatch;
	}
	// 82917344: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82917348: C03F0004  lfs f1, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8291734C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82917350: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82917354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82917358: 4E800421  bctrl
	ctx.lr = 0x8291735C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8291735C: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82917360: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82917364: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82917368: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8291736C: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82917370: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82917374: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82917378: 41990010  bgt cr6, 0x82917388
	if ctx.cr[6].gt {
	pc = 0x82917388; continue 'dispatch;
	}
	// 8291737C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82917380: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82917384: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82917388: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8291738C: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82917390: 41820060  beq 0x829173f0
	if ctx.cr[0].eq {
	pc = 0x829173F0; continue 'dispatch;
	}
	// 82917394: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82917398: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8291739C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 829173A0: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 829173A4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 829173A8: 41990024  bgt cr6, 0x829173cc
	if ctx.cr[6].gt {
	pc = 0x829173CC; continue 'dispatch;
	}
	// 829173AC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 829173B0: C03F000C  lfs f1, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 829173B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 829173B8: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 829173BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 829173C0: 4E800421  bctrl
	ctx.lr = 0x829173C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 829173C4: C01F0010  lfs f0, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829173C8: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 829173CC: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 829173D0: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 829173D4: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 829173D8: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 829173DC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 829173E0: 41990010  bgt cr6, 0x829173f0
	if ctx.cr[6].gt {
	pc = 0x829173F0; continue 'dispatch;
	}
	// 829173E4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 829173E8: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 829173EC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 829173F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 829173F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 829173F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 829173FC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82917400: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82917404: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82917408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


