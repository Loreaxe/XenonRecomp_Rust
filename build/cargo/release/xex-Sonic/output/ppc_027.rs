pub fn sub_82480550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82480550 size=104
    let mut pc: u32 = 0x82480550;
    'dispatch: loop {
        match pc {
            0x82480550 => {
    //   block [0x82480550..0x824805B8)
	// 82480550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82480558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248055C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82480564: 4BFE91F5  bl 0x82469758
	ctx.lr = 0x82480568;
	sub_82469758(ctx, base);
	// 82480568: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8248056C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82480570: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82480574: 394AB154  addi r10, r10, -0x4eac
	ctx.r[10].s64 = ctx.r[10].s64 + -20140;
	// 82480578: 392000D0  li r9, 0xd0
	ctx.r[9].s64 = 208;
	// 8248057C: 390000E0  li r8, 0xe0
	ctx.r[8].s64 = 224;
	// 82480580: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82480584: 394000F0  li r10, 0xf0
	ctx.r[10].s64 = 240;
	// 82480588: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8248058C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824805B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824805B8 size=328
    let mut pc: u32 = 0x824805B8;
    'dispatch: loop {
        match pc {
            0x824805B8 => {
    //   block [0x824805B8..0x82480700)
	// 824805B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824805BC: 48D27BA9  bl 0x831a8164
	ctx.lr = 0x824805C0;
	sub_831A8130(ctx, base);
	// 824805C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824805C4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824805C8: 48CD4039  bl 0x83154600
	ctx.lr = 0x824805CC;
	sub_83154600(ctx, base);
	// 824805CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824805D0: 4BFE4549  bl 0x82464b18
	ctx.lr = 0x824805D4;
	sub_82464B18(ctx, base);
	// 824805D4: 397B00D0  addi r11, r27, 0xd0
	ctx.r[11].s64 = ctx.r[27].s64 + 208;
	// 824805D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824805DC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824805E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824805E4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 824805E8: 4BFD8D29  bl 0x82459310
	ctx.lr = 0x824805EC;
	sub_82459310(ctx, base);
	// 824805EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824805F0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824805F4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824805F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824805FC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82480600: 419A0024  beq cr6, 0x82480624
	if ctx.cr[6].eq {
	pc = 0x82480624; continue 'dispatch;
	}
	// 82480604: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82480608: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248060C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82480610: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82480614: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82480618: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8248061C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82480620: 4082FFE8  bne 0x82480608
	if !ctx.cr[0].eq {
	pc = 0x82480608; continue 'dispatch;
	}
	// 82480624: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82480628: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8248062C: 3BCBB188  addi r30, r11, -0x4e78
	ctx.r[30].s64 = ctx.r[11].s64 + -20088;
	// 82480630: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82480634: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82480638: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8248063C: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 82480640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82480644: 489D69A5  bl 0x82e56fe8
	ctx.lr = 0x82480648;
	sub_82E56FE8(ctx, base);
	// 82480648: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8248064C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82480650: 419A0008  beq cr6, 0x82480658
	if ctx.cr[6].eq {
	pc = 0x82480658; continue 'dispatch;
	}
	// 82480654: 4BE4023D  bl 0x822c0890
	ctx.lr = 0x82480658;
	sub_822C0890(ctx, base);
	// 82480658: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8248065C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82480660: 419A0008  beq cr6, 0x82480668
	if ctx.cr[6].eq {
	pc = 0x82480668; continue 'dispatch;
	}
	// 82480664: 4BE4022D  bl 0x822c0890
	ctx.lr = 0x82480668;
	sub_822C0890(ctx, base);
	// 82480668: 3B9B00E0  addi r28, r27, 0xe0
	ctx.r[28].s64 = ctx.r[27].s64 + 224;
	// 8248066C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82480670: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82480674: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82480678: 4BFECB41  bl 0x8246d1b8
	ctx.lr = 0x8248067C;
	sub_8246D1B8(ctx, base);
	// 8248067C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82480680: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82480684: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82480688: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248068C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82480690: 419A0024  beq cr6, 0x824806b4
	if ctx.cr[6].eq {
	pc = 0x824806B4; continue 'dispatch;
	}
	// 82480694: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82480698: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248069C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824806A0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 824806A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824806A8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 824806AC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824806B0: 4082FFE8  bne 0x82480698
	if !ctx.cr[0].eq {
	pc = 0x82480698; continue 'dispatch;
	}
	// 824806B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824806B8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 824806BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824806C0: 38A00039  li r5, 0x39
	ctx.r[5].s64 = 57;
	// 824806C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824806C8: 489D6921  bl 0x82e56fe8
	ctx.lr = 0x824806CC;
	sub_82E56FE8(ctx, base);
	// 824806CC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 824806D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824806D4: 419A0008  beq cr6, 0x824806dc
	if ctx.cr[6].eq {
	pc = 0x824806DC; continue 'dispatch;
	}
	// 824806D8: 4BE401B9  bl 0x822c0890
	ctx.lr = 0x824806DC;
	sub_822C0890(ctx, base);
	// 824806DC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 824806E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824806E4: 419A0008  beq cr6, 0x824806ec
	if ctx.cr[6].eq {
	pc = 0x824806EC; continue 'dispatch;
	}
	// 824806E8: 4BE401A9  bl 0x822c0890
	ctx.lr = 0x824806EC;
	sub_822C0890(ctx, base);
	// 824806EC: 396000F0  li r11, 0xf0
	ctx.r[11].s64 = 240;
	// 824806F0: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82480700 size=732
    let mut pc: u32 = 0x82480700;
    'dispatch: loop {
        match pc {
            0x82480700 => {
    //   block [0x82480700..0x824809DC)
	// 82480700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480704: 48D27A51  bl 0x831a8154
	ctx.lr = 0x82480708;
	sub_831A8130(ctx, base);
	// 82480708: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824809E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824809E0 size=200
    let mut pc: u32 = 0x824809E0;
    'dispatch: loop {
        match pc {
            0x824809E0 => {
    //   block [0x824809E0..0x82480AA8)
	// 824809E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824809E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824809E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824809EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824809F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824809F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824809F8: 48CD3C09  bl 0x83154600
	ctx.lr = 0x824809FC;
	sub_83154600(ctx, base);
	// 824809FC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82480A00: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82480A04: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82480A08: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82480A0C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82480A10: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82480A14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82480A18: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82480AA8 size=76
    let mut pc: u32 = 0x82480AA8;
    'dispatch: loop {
        match pc {
            0x82480AA8 => {
    //   block [0x82480AA8..0x82480AF4)
	// 82480AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82480AB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82480AB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82480ABC: 807F00F4  lwz r3, 0xf4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82480AC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82480AC4: 419A0008  beq cr6, 0x82480acc
	if ctx.cr[6].eq {
	pc = 0x82480ACC; continue 'dispatch;
	}
	// 82480AC8: 4BE3FDC9  bl 0x822c0890
	ctx.lr = 0x82480ACC;
	sub_822C0890(ctx, base);
	// 82480ACC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82480AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82480AD4: 396B770C  addi r11, r11, 0x770c
	ctx.r[11].s64 = ctx.r[11].s64 + 30476;
	// 82480AD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82480ADC: 489DBAAD  bl 0x82e5c588
	ctx.lr = 0x82480AE0;
	sub_82E5C588(ctx, base);
	// 82480AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82480AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82480AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82480AEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82480AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82480AF8 size=12
    let mut pc: u32 = 0x82480AF8;
    'dispatch: loop {
        match pc {
            0x82480AF8 => {
    //   block [0x82480AF8..0x82480B04)
	// 82480AF8: 98830110  stb r4, 0x110(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), ctx.r[4].u8 ) };
	// 82480AFC: 98A30111  stb r5, 0x111(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(273 as u32), ctx.r[5].u8 ) };
	// 82480B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82480B08 size=172
    let mut pc: u32 = 0x82480B08;
    'dispatch: loop {
        match pc {
            0x82480B08 => {
    //   block [0x82480B08..0x82480BB4)
	// 82480B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480B0C: 48D27661  bl 0x831a816c
	ctx.lr = 0x82480B10;
	sub_831A8130(ctx, base);
	// 82480B10: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82480B14: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480B18: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82480B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82480B20: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82480B24: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82480B28: 41820064  beq 0x82480b8c
	if ctx.cr[0].eq {
	pc = 0x82480B8C; continue 'dispatch;
	}
	// 82480B2C: 7FA4FA14  add r29, r4, r31
	ctx.r[29].u64 = ctx.r[4].u64 + ctx.r[31].u64;
	// 82480B30: 897D0120  lbz r11, 0x120(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(288 as u32) ) } as u64;
	// 82480B34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82480B38: 4082001C  bne 0x82480b54
	if !ctx.cr[0].eq {
	pc = 0x82480B54; continue 'dispatch;
	}
	// 82480B3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82480B40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82480B44: 997D0120  stb r11, 0x120(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(288 as u32), ctx.r[11].u8 ) };
	// 82480B48: 997D0126  stb r11, 0x126(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(294 as u32), ctx.r[11].u8 ) };
	// 82480B4C: C3EA08A8  lfs f31, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82480B50: 48000054  b 0x82480ba4
	pc = 0x82480BA4; continue 'dispatch;
	// 82480B54: 3964004B  addi r11, r4, 0x4b
	ctx.r[11].s64 = ctx.r[4].s64 + 75;
	// 82480B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82480B5C: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82480B60: 489D8C19  bl 0x82e59778
	ctx.lr = 0x82480B64;
	sub_82E59778(ctx, base);
	// 82480B64: 7C1EFC2E  lfsx f0, r30, r31
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82480B68: EDA1002A  fadds f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82480B6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82480B70: 7DBEFD2E  stfsx f13, r30, r31
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), tmp.u32) };
	// 82480B74: C00B0A90  lfs f0, 0xa90(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2704 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82480B78: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82480B7C: 41980028  blt cr6, 0x82480ba4
	if ctx.cr[6].lt {
	pc = 0x82480BA4; continue 'dispatch;
	}
	// 82480B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82480B84: 997D0120  stb r11, 0x120(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(288 as u32), ctx.r[11].u8 ) };
	// 82480B88: 4800001C  b 0x82480ba4
	pc = 0x82480BA4; continue 'dispatch;
	// 82480B8C: 3964004B  addi r11, r4, 0x4b
	ctx.r[11].s64 = ctx.r[4].s64 + 75;
	// 82480B90: 7D44FA14  add r10, r4, r31
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[31].u64;
	// 82480B94: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82480B98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82480B9C: 992A0120  stb r9, 0x120(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(288 as u32), ctx.r[9].u8 ) };
	// 82480BA0: 7FEBFD2E  stfsx f31, r11, r31
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), tmp.u32) };
	// 82480BA4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82480BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82480BAC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82480BB0: 48D2760C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82480BB8 size=184
    let mut pc: u32 = 0x82480BB8;
    'dispatch: loop {
        match pc {
            0x82480BB8 => {
    //   block [0x82480BB8..0x82480C70)
	// 82480BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480BBC: 48D275B1  bl 0x831a816c
	ctx.lr = 0x82480BC0;
	sub_831A8130(ctx, base);
	// 82480BC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480BC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82480BC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82480BCC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82480BD0: 4BFE8B89  bl 0x82469758
	ctx.lr = 0x82480BD4;
	sub_82469758(ctx, base);
	// 82480BD4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82480BD8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82480BDC: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82480BE0: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82480BE4: 396BB1DC  addi r11, r11, -0x4e24
	ctx.r[11].s64 = ctx.r[11].s64 + -20004;
	// 82480BE8: 38A000F0  li r5, 0xf0
	ctx.r[5].s64 = 240;
	// 82480BEC: 3C808208  lis r4, -0x7df8
	ctx.r[4].s64 = -2113404928;
	// 82480BF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82480BF4: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82480BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82480C70 size=8
    let mut pc: u32 = 0x82480C70;
    'dispatch: loop {
        match pc {
            0x82480C70 => {
    //   block [0x82480C70..0x82480C78)
	// 82480C70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82480C74: 489D8ACC  b 0x82e59740
	sub_82E59740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82480C78 size=100
    let mut pc: u32 = 0x82480C78;
    'dispatch: loop {
        match pc {
            0x82480C78 => {
    //   block [0x82480C78..0x82480CDC)
	// 82480C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82480C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82480C84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480C88: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82480C8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82480C90: 3BEB6604  addi r31, r11, 0x6604
	ctx.r[31].s64 = ctx.r[11].s64 + 26116;
	// 82480C94: 816A6608  lwz r11, 0x6608(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26120 as u32) ) } as u64;
	// 82480C98: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82480C9C: 40820028  bne 0x82480cc4
	if !ctx.cr[0].eq {
	pc = 0x82480CC4; continue 'dispatch;
	}
	// 82480CA0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82480CA4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82480CA8: 916A6608  stw r11, 0x6608(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26120 as u32), ctx.r[11].u32 ) };
	// 82480CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82480CB0: 3889B20C  addi r4, r9, -0x4df4
	ctx.r[4].s64 = ctx.r[9].s64 + -19956;
	// 82480CB4: 48972D55  bl 0x82df3a08
	ctx.lr = 0x82480CB8;
	sub_82DF3A08(ctx, base);
	// 82480CB8: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82480CBC: 386BF468  addi r3, r11, -0xb98
	ctx.r[3].s64 = ctx.r[11].s64 + -2968;
	// 82480CC0: 48D27819  bl 0x831a84d8
	ctx.lr = 0x82480CC4;
	sub_831A84D8(ctx, base);
	// 82480CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82480CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82480CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82480CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82480CD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82480CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82480CE0 size=72
    let mut pc: u32 = 0x82480CE0;
    'dispatch: loop {
        match pc {
            0x82480CE0 => {
    //   block [0x82480CE0..0x82480D28)
	// 82480CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82480CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82480CEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480CF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82480CF4: 48CD390D  bl 0x83154600
	ctx.lr = 0x82480CF8;
	sub_83154600(ctx, base);
	// 82480CF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82480CFC: 38BF00F0  addi r5, r31, 0xf0
	ctx.r[5].s64 = ctx.r[31].s64 + 240;
	// 82480D00: 389F00D0  addi r4, r31, 0xd0
	ctx.r[4].s64 = ctx.r[31].s64 + 208;
	// 82480D04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82480D08: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82480D0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82480D10: 4E800421  bctrl
	ctx.lr = 0x82480D14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82480D14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82480D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82480D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82480D20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82480D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82480D28 size=16
    let mut pc: u32 = 0x82480D28;
    'dispatch: loop {
        match pc {
            0x82480D28 => {
    //   block [0x82480D28..0x82480D38)
	// 82480D28: 396000D0  li r11, 0xd0
	ctx.r[11].s64 = 208;
	// 82480D2C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82480D38 size=16
    let mut pc: u32 = 0x82480D38;
    'dispatch: loop {
        match pc {
            0x82480D38 => {
    //   block [0x82480D38..0x82480D48)
	// 82480D38: 396000E0  li r11, 0xe0
	ctx.r[11].s64 = 224;
	// 82480D3C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82480D48 size=16
    let mut pc: u32 = 0x82480D48;
    'dispatch: loop {
        match pc {
            0x82480D48 => {
    //   block [0x82480D48..0x82480D58)
	// 82480D48: 396000F0  li r11, 0xf0
	ctx.r[11].s64 = 240;
	// 82480D4C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82480D58 size=240
    let mut pc: u32 = 0x82480D58;
    'dispatch: loop {
        match pc {
            0x82480D58 => {
    //   block [0x82480D58..0x82480E48)
	// 82480D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480D5C: 48D27411  bl 0x831a816c
	ctx.lr = 0x82480D60;
	sub_831A8130(ctx, base);
	// 82480D60: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82480D64: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82480D68: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480D6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82480D70: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82480D74: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82480D78: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82480D7C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82480D80: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82480D84: 489FB44D  bl 0x82e7c1d0
	ctx.lr = 0x82480D88;
	sub_82E7C1D0(ctx, base);
	// 82480D88: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82480D8C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82480D90: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82480D94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82480D98: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82480D9C: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82480DA0: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82480DA4: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82480DA8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82480DAC: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82480DB0: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82480DB4: 489FC56D  bl 0x82e7d320
	ctx.lr = 0x82480DB8;
	sub_82E7D320(ctx, base);
	// 82480DB8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82480DBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82480DC0: 489FC691  bl 0x82e7d450
	ctx.lr = 0x82480DC4;
	sub_82E7D450(ctx, base);
	// 82480DC4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82480DC8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82480DCC: 13A0F407  vcmpneb. (lvlx128) v29, v0, v30
	tmp.u32 = ctx.r[30].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82480DD0: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82480E48 size=216
    let mut pc: u32 = 0x82480E48;
    'dispatch: loop {
        match pc {
            0x82480E48 => {
    //   block [0x82480E48..0x82480F20)
	// 82480E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480E4C: 48D27321  bl 0x831a816c
	ctx.lr = 0x82480E50;
	sub_831A8130(ctx, base);
	// 82480E50: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82480E54: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82480E58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82480E60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82480E64: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82480E68: 48CD3799  bl 0x83154600
	ctx.lr = 0x82480E6C;
	sub_83154600(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82480F20 size=108
    let mut pc: u32 = 0x82480F20;
    'dispatch: loop {
        match pc {
            0x82480F20 => {
    //   block [0x82480F20..0x82480F8C)
	// 82480F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82480F28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82480F2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82480F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480F34: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82480F38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82480F3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82480F40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82480F44: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82480F48: 48970CB1  bl 0x82df1bf8
	ctx.lr = 0x82480F4C;
	sub_82DF1BF8(ctx, base);
	// 82480F4C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82480F50: 486C62F1  bl 0x82b47240
	ctx.lr = 0x82480F54;
	sub_82B47240(ctx, base);
	// 82480F54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82480F58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82480F5C: 48970D35  bl 0x82df1c90
	ctx.lr = 0x82480F60;
	sub_82DF1C90(ctx, base);
	// 82480F60: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82480F64: 7D6BF838  and r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[31].u64;
	// 82480F68: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82480F6C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82480F70: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82480F74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82480F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82480F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82480F80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82480F84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82480F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82480F90 size=80
    let mut pc: u32 = 0x82480F90;
    'dispatch: loop {
        match pc {
            0x82480F90 => {
    //   block [0x82480F90..0x82480FE0)
	// 82480F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82480F98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82480F9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480FA0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82480FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82480FA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82480FAC: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 82480FB0: 48970C49  bl 0x82df1bf8
	ctx.lr = 0x82480FB4;
	sub_82DF1BF8(ctx, base);
	// 82480FB4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82480FB8: 486C6289  bl 0x82b47240
	ctx.lr = 0x82480FBC;
	sub_82B47240(ctx, base);
	// 82480FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82480FC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82480FC4: 48970CCD  bl 0x82df1c90
	ctx.lr = 0x82480FC8;
	sub_82DF1C90(ctx, base);
	// 82480FC8: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82480FCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82480FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82480FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82480FD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82480FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82480FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82480FE0 size=336
    let mut pc: u32 = 0x82480FE0;
    'dispatch: loop {
        match pc {
            0x82480FE0 => {
    //   block [0x82480FE0..0x82481130)
	// 82480FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82480FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82480FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82480FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82480FF0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82480FF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82480FF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82480FFC: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 82481000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82481004: 4BFFFF1D  bl 0x82480f20
	ctx.lr = 0x82481008;
	sub_82480F20(ctx, base);
	// 82481008: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248100C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481010: 41820040  beq 0x82481050
	if ctx.cr[0].eq {
	pc = 0x82481050; continue 'dispatch;
	}
	// 82481014: 48131E7D  bl 0x825b2e90
	ctx.lr = 0x82481018;
	sub_825B2E90(ctx, base);
	// 82481018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248101C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82481020: 489D8759  bl 0x82e59778
	ctx.lr = 0x82481024;
	sub_82E59778(ctx, base);
	// 82481024: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 82481028: C19F010C  lfs f12, 0x10c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8248102C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82481030: C17F0100  lfs f11, 0x100(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82481034: C00B9528  lfs f0, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82481038: EDAD62FC  fnmsubs f13, f13, f11, f12
	ctx.f[13].f64 = -(((ctx.f[13].f64 * ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8248103C: D1BF010C  stfs f13, 0x10c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 82481040: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82481044: 4098009C  bge cr6, 0x824810e0
	if !ctx.cr[6].lt {
	pc = 0x824810E0; continue 'dispatch;
	}
	// 82481048: D01F010C  stfs f0, 0x10c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 8248104C: 48000094  b 0x824810e0
	pc = 0x824810E0; continue 'dispatch;
	// 82481050: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82481054: 41820038  beq 0x8248108c
	if ctx.cr[0].eq {
	pc = 0x8248108C; continue 'dispatch;
	}
	// 82481058: 48131DE9  bl 0x825b2e40
	ctx.lr = 0x8248105C;
	sub_825B2E40(ctx, base);
	// 8248105C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481060: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82481064: 489D8715  bl 0x82e59778
	ctx.lr = 0x82481068;
	sub_82E59778(ctx, base);
	// 82481068: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 8248106C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82481070: C19F0108  lfs f12, 0x108(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82481074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481078: C00BA1C4  lfs f0, -0x5e3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248107C: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82481080: D01F0108  stfs f0, 0x108(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 82481084: 48131E0D  bl 0x825b2e90
	ctx.lr = 0x82481088;
	sub_825B2E90(ctx, base);
	// 82481088: 48000034  b 0x824810bc
	pc = 0x824810BC; continue 'dispatch;
	// 8248108C: 4811EC15  bl 0x8259fca0
	ctx.lr = 0x82481090;
	sub_8259FCA0(ctx, base);
	// 82481090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481094: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82481098: 489D86E1  bl 0x82e59778
	ctx.lr = 0x8248109C;
	sub_82E59778(ctx, base);
	// 8248109C: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 824810A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824810A4: C19F0108  lfs f12, 0x108(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824810A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824810AC: C00BA1C4  lfs f0, -0x5e3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824810B0: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 824810B4: D01F0108  stfs f0, 0x108(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 824810B8: 4BFFFED9  bl 0x82480f90
	ctx.lr = 0x824810BC;
	sub_82480F90(ctx, base);
	// 824810BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824810C0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824810C4: 489D86B5  bl 0x82e59778
	ctx.lr = 0x824810C8;
	sub_82E59778(ctx, base);
	// 824810C8: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 824810CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824810D0: C19F0104  lfs f12, 0x104(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824810D4: C00B7BC4  lfs f0, 0x7bc4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31684 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824810D8: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 824810DC: D01F0104  stfs f0, 0x104(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 824810E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824810E4: C01F0104  lfs f0, 0x104(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824810E8: C1AB9584  lfs f13, -0x6a7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824810EC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824810F0: 41990014  bgt cr6, 0x82481104
	if ctx.cr[6].gt {
	pc = 0x82481104; continue 'dispatch;
	}
	// 824810F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824810F8: C1AB78A8  lfs f13, 0x78a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30888 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824810FC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82481100: 40980008  bge cr6, 0x82481108
	if !ctx.cr[6].lt {
	pc = 0x82481108; continue 'dispatch;
	}
	// 82481104: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82481108: D01F0104  stfs f0, 0x104(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 8248110C: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 82481110: 48016D09  bl 0x82497e18
	ctx.lr = 0x82481114;
	sub_82497E18(ctx, base);
	// 82481114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248111C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82481120: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82481124: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82481128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248112C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82481130 size=1828
    let mut pc: u32 = 0x82481130;
    'dispatch: loop {
        match pc {
            0x82481130 => {
    //   block [0x82481130..0x82481854)
	// 82481130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82481134: 48D27039  bl 0x831a816c
	ctx.lr = 0x82481138;
	sub_831A8130(ctx, base);
	// 82481138: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8248113C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82481140: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481144: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82481148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248114C: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82481150: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82481154: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82481158: 895F0118  lbz r10, 0x118(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 8248115C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82481160: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82481858 size=220
    let mut pc: u32 = 0x82481858;
    'dispatch: loop {
        match pc {
            0x82481858 => {
    //   block [0x82481858..0x82481934)
	// 82481858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248185C: 48D26911  bl 0x831a816c
	ctx.lr = 0x82481860;
	sub_831A8130(ctx, base);
	// 82481860: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82481868: 48CD2D99  bl 0x83154600
	ctx.lr = 0x8248186C;
	sub_83154600(ctx, base);
	// 8248186C: 897F0110  lbz r11, 0x110(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 82481870: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82481874: 4182000C  beq 0x82481880
	if ctx.cr[0].eq {
	pc = 0x82481880; continue 'dispatch;
	}
	// 82481878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248187C: 4BFFF8B5  bl 0x82481130
	ctx.lr = 0x82481880;
	sub_82481130(ctx, base);
	// 82481880: 897F0111  lbz r11, 0x111(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(273 as u32) ) } as u64;
	// 82481884: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82481888: 41820010  beq 0x82481898
	if ctx.cr[0].eq {
	pc = 0x82481898; continue 'dispatch;
	}
	// 8248188C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82481890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481894: 4BFFF74D  bl 0x82480fe0
	ctx.lr = 0x82481898;
	sub_82480FE0(ctx, base);
	// 82481898: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8248189C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824818A0: 4BFFF681  bl 0x82480f20
	ctx.lr = 0x824818A4;
	sub_82480F20(ctx, base);
	// 824818A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824818A8: 41820010  beq 0x824818b8
	if ctx.cr[0].eq {
	pc = 0x824818B8; continue 'dispatch;
	}
	// 824818AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824818B0: C00B08AC  lfs f0, 0x8ac(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824818B4: 4800000C  b 0x824818c0
	pc = 0x824818C0; continue 'dispatch;
	// 824818B8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 824818BC: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824818C0: D01F0100  stfs f0, 0x100(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 824818C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824818C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824818CC: C07F0104  lfs f3, 0x104(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 824818D0: C05F0108  lfs f2, 0x108(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824818D4: C03F010C  lfs f1, 0x10c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824818D8: 4811DD89  bl 0x8259f660
	ctx.lr = 0x824818DC;
	sub_8259F660(ctx, base);
	// 824818DC: 397F00F0  addi r11, r31, 0xf0
	ctx.r[11].s64 = ctx.r[31].s64 + 240;
	// 824818E0: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824818E4: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 824818E8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824818EC: 3BBF00E0  addi r29, r31, 0xe0
	ctx.r[29].s64 = ctx.r[31].s64 + 224;
	// 824818F0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824818F4: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824818F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82481938 size=96
    let mut pc: u32 = 0x82481938;
    'dispatch: loop {
        match pc {
            0x82481938 => {
    //   block [0x82481938..0x82481998)
	// 82481938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248193C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82481940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82481944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82481948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248194C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82481950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82481954: 4BFE7E05  bl 0x82469758
	ctx.lr = 0x82481958;
	sub_82469758(ctx, base);
	// 82481958: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248195C: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 82481960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82481964: 396BB260  addi r11, r11, -0x4da0
	ctx.r[11].s64 = ctx.r[11].s64 + -19872;
	// 82481968: 915F00D4  stw r10, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 8248196C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481970: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82481974: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82481978: 4BFE7DD9  bl 0x82469750
	ctx.lr = 0x8248197C;
	sub_82469750(ctx, base);
	// 8248197C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481980: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82481988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248198C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82481990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82481994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82481998 size=196
    let mut pc: u32 = 0x82481998;
    'dispatch: loop {
        match pc {
            0x82481998 => {
    //   block [0x82481998..0x82481A5C)
	// 82481998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824819A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824819A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824819A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824819AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824819B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824819B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 824819B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824819BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824819C0: 4BE3EF79  bl 0x822c0938
	ctx.lr = 0x824819C4;
	sub_822C0938(ctx, base);
	// 824819C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824819C8: 41820028  beq 0x824819f0
	if ctx.cr[0].eq {
	pc = 0x824819F0; continue 'dispatch;
	}
	// 824819CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824819D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 824819D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824819D8: 392BB224  addi r9, r11, -0x4ddc
	ctx.r[9].s64 = ctx.r[11].s64 + -19932;
	// 824819DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824819E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824819E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824819E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824819EC: 48000008  b 0x824819f4
	pc = 0x824819F4; continue 'dispatch;
	// 824819F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824819F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824819F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824819FC: 409A0044  bne cr6, 0x82481a40
	if !ctx.cr[6].eq {
	pc = 0x82481A40; continue 'dispatch;
	}
	// 82481A00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82481A04: 419A001C  beq cr6, 0x82481a20
	if ctx.cr[6].eq {
	pc = 0x82481A20; continue 'dispatch;
	}
	// 82481A08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481A0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82481A10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481A14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481A18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82481A1C: 4E800421  bctrl
	ctx.lr = 0x82481A20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82481A20: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82481A24: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82481A28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481A2C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82481A30: 816B6D4C  lwz r11, 0x6d4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27980 as u32) ) } as u64;
	// 82481A34: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82481A38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82481A3C: 4BE3E5C5  bl 0x822c0000
	ctx.lr = 0x82481A40;
	sub_822C0000(ctx, base);
	// 82481A40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481A44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82481A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82481A50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82481A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82481A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82481A60 size=196
    let mut pc: u32 = 0x82481A60;
    'dispatch: loop {
        match pc {
            0x82481A60 => {
    //   block [0x82481A60..0x82481B24)
	// 82481A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82481A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82481A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82481A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82481A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481A74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82481A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82481A7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82481A80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82481A84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82481A88: 4BE3EEB1  bl 0x822c0938
	ctx.lr = 0x82481A8C;
	sub_822C0938(ctx, base);
	// 82481A8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82481A90: 41820028  beq 0x82481ab8
	if ctx.cr[0].eq {
	pc = 0x82481AB8; continue 'dispatch;
	}
	// 82481A94: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481A98: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82481A9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82481AA0: 392BB238  addi r9, r11, -0x4dc8
	ctx.r[9].s64 = ctx.r[11].s64 + -19912;
	// 82481AA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82481AA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82481AAC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82481AB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82481AB4: 48000008  b 0x82481abc
	pc = 0x82481ABC; continue 'dispatch;
	// 82481AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82481ABC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82481AC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82481AC4: 409A0044  bne cr6, 0x82481b08
	if !ctx.cr[6].eq {
	pc = 0x82481B08; continue 'dispatch;
	}
	// 82481AC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82481ACC: 419A001C  beq cr6, 0x82481ae8
	if ctx.cr[6].eq {
	pc = 0x82481AE8; continue 'dispatch;
	}
	// 82481AD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481AD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82481AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481ADC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481AE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82481AE4: 4E800421  bctrl
	ctx.lr = 0x82481AE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82481AE8: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82481AEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82481AF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481AF4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82481AF8: 816B6D4C  lwz r11, 0x6d4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27980 as u32) ) } as u64;
	// 82481AFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82481B00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82481B04: 4BE3E4FD  bl 0x822c0000
	ctx.lr = 0x82481B08;
	sub_822C0000(ctx, base);
	// 82481B08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481B0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82481B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82481B18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82481B1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82481B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82481B28 size=196
    let mut pc: u32 = 0x82481B28;
    'dispatch: loop {
        match pc {
            0x82481B28 => {
    //   block [0x82481B28..0x82481BEC)
	// 82481B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82481B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82481B30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82481B34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82481B38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481B3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82481B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82481B44: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82481B48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82481B4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82481B50: 4BE3EDE9  bl 0x822c0938
	ctx.lr = 0x82481B54;
	sub_822C0938(ctx, base);
	// 82481B54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82481B58: 41820028  beq 0x82481b80
	if ctx.cr[0].eq {
	pc = 0x82481B80; continue 'dispatch;
	}
	// 82481B5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481B60: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82481B64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82481B68: 392BB24C  addi r9, r11, -0x4db4
	ctx.r[9].s64 = ctx.r[11].s64 + -19892;
	// 82481B6C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82481B70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82481B74: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82481B78: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82481B7C: 48000008  b 0x82481b84
	pc = 0x82481B84; continue 'dispatch;
	// 82481B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82481B84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82481B88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82481B8C: 409A0044  bne cr6, 0x82481bd0
	if !ctx.cr[6].eq {
	pc = 0x82481BD0; continue 'dispatch;
	}
	// 82481B90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82481B94: 419A001C  beq cr6, 0x82481bb0
	if ctx.cr[6].eq {
	pc = 0x82481BB0; continue 'dispatch;
	}
	// 82481B98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481B9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82481BA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481BA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481BA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82481BAC: 4E800421  bctrl
	ctx.lr = 0x82481BB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82481BB0: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82481BB4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82481BB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481BBC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82481BC0: 816B6D4C  lwz r11, 0x6d4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27980 as u32) ) } as u64;
	// 82481BC4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82481BC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82481BCC: 4BE3E435  bl 0x822c0000
	ctx.lr = 0x82481BD0;
	sub_822C0000(ctx, base);
	// 82481BD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481BD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82481BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82481BE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82481BE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82481BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82481BF0 size=120
    let mut pc: u32 = 0x82481BF0;
    'dispatch: loop {
        match pc {
            0x82481BF0 => {
    //   block [0x82481BF0..0x82481C68)
	// 82481BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82481BF4: 48D26579  bl 0x831a816c
	ctx.lr = 0x82481BF8;
	sub_831A8130(ctx, base);
	// 82481BF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481BFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82481C00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82481C04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82481C08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82481C0C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82481C10: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82481C14: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82481C18: 489707D1  bl 0x82df23e8
	ctx.lr = 0x82481C1C;
	sub_82DF23E8(ctx, base);
	// 82481C1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82481C20: 41820014  beq 0x82481c34
	if ctx.cr[0].eq {
	pc = 0x82481C34; continue 'dispatch;
	}
	// 82481C24: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481C28: 486D96F9  bl 0x82b5b320
	ctx.lr = 0x82481C2C;
	sub_82B5B320(ctx, base);
	// 82481C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82481C30: 48000008  b 0x82481c38
	pc = 0x82481C38; continue 'dispatch;
	// 82481C34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82481C38: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82481C3C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82481C40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82481C44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481C48: 4BFFFD51  bl 0x82481998
	ctx.lr = 0x82481C4C;
	sub_82481998(ctx, base);
	// 82481C4C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82481C50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82481C54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481C58: 4BE3E3A9  bl 0x822c0000
	ctx.lr = 0x82481C5C;
	sub_822C0000(ctx, base);
	// 82481C5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82481C60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481C64: 48D26558  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82481C68 size=120
    let mut pc: u32 = 0x82481C68;
    'dispatch: loop {
        match pc {
            0x82481C68 => {
    //   block [0x82481C68..0x82481CE0)
	// 82481C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82481C6C: 48D26501  bl 0x831a816c
	ctx.lr = 0x82481C70;
	sub_831A8130(ctx, base);
	// 82481C70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481C74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82481C78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82481C7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82481C80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82481C84: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82481C88: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82481C8C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82481C90: 48970759  bl 0x82df23e8
	ctx.lr = 0x82481C94;
	sub_82DF23E8(ctx, base);
	// 82481C94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82481C98: 41820014  beq 0x82481cac
	if ctx.cr[0].eq {
	pc = 0x82481CAC; continue 'dispatch;
	}
	// 82481C9C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481CA0: 486D2689  bl 0x82b54328
	ctx.lr = 0x82481CA4;
	sub_82B54328(ctx, base);
	// 82481CA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82481CA8: 48000008  b 0x82481cb0
	pc = 0x82481CB0; continue 'dispatch;
	// 82481CAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82481CB0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82481CB4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82481CB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82481CBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481CC0: 4BFFFDA1  bl 0x82481a60
	ctx.lr = 0x82481CC4;
	sub_82481A60(ctx, base);
	// 82481CC4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82481CC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82481CCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481CD0: 4BE3E331  bl 0x822c0000
	ctx.lr = 0x82481CD4;
	sub_822C0000(ctx, base);
	// 82481CD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82481CD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481CDC: 48D264E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82481CE0 size=120
    let mut pc: u32 = 0x82481CE0;
    'dispatch: loop {
        match pc {
            0x82481CE0 => {
    //   block [0x82481CE0..0x82481D58)
	// 82481CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82481CE4: 48D26489  bl 0x831a816c
	ctx.lr = 0x82481CE8;
	sub_831A8130(ctx, base);
	// 82481CE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481CEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82481CF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82481CF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82481CF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82481CFC: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82481D00: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82481D04: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82481D08: 489706E1  bl 0x82df23e8
	ctx.lr = 0x82481D0C;
	sub_82DF23E8(ctx, base);
	// 82481D0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82481D10: 41820014  beq 0x82481d24
	if ctx.cr[0].eq {
	pc = 0x82481D24; continue 'dispatch;
	}
	// 82481D14: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82481D18: 486D9659  bl 0x82b5b370
	ctx.lr = 0x82481D1C;
	sub_82B5B370(ctx, base);
	// 82481D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82481D20: 48000008  b 0x82481d28
	pc = 0x82481D28; continue 'dispatch;
	// 82481D24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82481D28: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82481D2C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82481D30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82481D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481D38: 4BFFFDF1  bl 0x82481b28
	ctx.lr = 0x82481D3C;
	sub_82481B28(ctx, base);
	// 82481D3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82481D40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82481D44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82481D48: 4BE3E2B9  bl 0x822c0000
	ctx.lr = 0x82481D4C;
	sub_822C0000(ctx, base);
	// 82481D4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82481D50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82481D54: 48D26468  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82481D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82481D58 size=752
    let mut pc: u32 = 0x82481D58;
    'dispatch: loop {
        match pc {
            0x82481D58 => {
    //   block [0x82481D58..0x82482048)
	// 82481D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82481D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82481D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82481D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82481D68: DBA1FFD0  stfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[29].u64 ) };
	// 82481D6C: DBC1FFD8  stfd f30, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82481D70: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82481D74: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82481D78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82481D7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82481D80: 419A02A4  beq cr6, 0x82482024
	if ctx.cr[6].eq {
	pc = 0x82482024; continue 'dispatch;
	}
	// 82481D84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481D8C: 388BB2D0  addi r4, r11, -0x4d30
	ctx.r[4].s64 = ctx.r[11].s64 + -19760;
	// 82481D90: 48971C79  bl 0x82df3a08
	ctx.lr = 0x82481D94;
	sub_82DF3A08(ctx, base);
	// 82481D94: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82481D98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82481D9C: 3BCB660C  addi r30, r11, 0x660c
	ctx.r[30].s64 = ctx.r[11].s64 + 26124;
	// 82481DA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82481DA4: 481211CD  bl 0x825a2f70
	ctx.lr = 0x82481DA8;
	sub_825A2F70(ctx, base);
	// 82481DA8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82481DAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82481DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481DB4: 481203D5  bl 0x825a2188
	ctx.lr = 0x82481DB8;
	sub_825A2188(ctx, base);
	// 82481DB8: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82481DBC: 4897166D  bl 0x82df3428
	ctx.lr = 0x82481DC0;
	sub_82DF3428(ctx, base);
	// 82481DC0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82481DC4: 4BE46EF5  bl 0x822c8cb8
	ctx.lr = 0x82481DC8;
	sub_822C8CB8(ctx, base);
	// 82481DC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481DCC: 4897165D  bl 0x82df3428
	ctx.lr = 0x82481DD0;
	sub_82DF3428(ctx, base);
	// 82481DD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481DD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481DD8: 388BB2C4  addi r4, r11, -0x4d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -19772;
	// 82481DDC: 48971C2D  bl 0x82df3a08
	ctx.lr = 0x82481DE0;
	sub_82DF3A08(ctx, base);
	// 82481DE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82481DE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82481DE8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82481DEC: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82481DF0: 38610150  addi r3, r1, 0x150
	ctx.r[3].s64 = ctx.r[1].s64 + 336;
	// 82481DF4: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82481DF8: C3CA6218  lfs f30, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82481DFC: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82481E00: C3A90790  lfs f29, 0x790(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(1936 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82481E04: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82481E08: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82481E0C: 481213A5  bl 0x825a31b0
	ctx.lr = 0x82481E10;
	sub_825A31B0(ctx, base);
	// 82481E10: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82481E14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82481E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481E1C: 4811F955  bl 0x825a1770
	ctx.lr = 0x82481E20;
	sub_825A1770(ctx, base);
	// 82481E20: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 82481E24: 48971605  bl 0x82df3428
	ctx.lr = 0x82481E28;
	sub_82DF3428(ctx, base);
	// 82481E28: 38610168  addi r3, r1, 0x168
	ctx.r[3].s64 = ctx.r[1].s64 + 360;
	// 82481E2C: 4BE46E8D  bl 0x822c8cb8
	ctx.lr = 0x82481E30;
	sub_822C8CB8(ctx, base);
	// 82481E30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481E34: 489715F5  bl 0x82df3428
	ctx.lr = 0x82481E38;
	sub_82DF3428(ctx, base);
	// 82481E38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481E3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481E40: 388BB2B8  addi r4, r11, -0x4d48
	ctx.r[4].s64 = ctx.r[11].s64 + -19784;
	// 82481E44: 48971BC5  bl 0x82df3a08
	ctx.lr = 0x82481E48;
	sub_82DF3A08(ctx, base);
	// 82481E48: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82481E4C: 386101D0  addi r3, r1, 0x1d0
	ctx.r[3].s64 = ctx.r[1].s64 + 464;
	// 82481E50: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82481E54: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82481E58: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82481E5C: 48121355  bl 0x825a31b0
	ctx.lr = 0x82481E60;
	sub_825A31B0(ctx, base);
	// 82481E60: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82481E64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82481E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481E6C: 4811F905  bl 0x825a1770
	ctx.lr = 0x82481E70;
	sub_825A1770(ctx, base);
	// 82481E70: 38610208  addi r3, r1, 0x208
	ctx.r[3].s64 = ctx.r[1].s64 + 520;
	// 82481E74: 489715B5  bl 0x82df3428
	ctx.lr = 0x82481E78;
	sub_82DF3428(ctx, base);
	// 82481E78: 386101E8  addi r3, r1, 0x1e8
	ctx.r[3].s64 = ctx.r[1].s64 + 488;
	// 82481E7C: 4BE46E3D  bl 0x822c8cb8
	ctx.lr = 0x82481E80;
	sub_822C8CB8(ctx, base);
	// 82481E80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481E84: 489715A5  bl 0x82df3428
	ctx.lr = 0x82481E88;
	sub_82DF3428(ctx, base);
	// 82481E88: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481E8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481E90: 388BB2AC  addi r4, r11, -0x4d54
	ctx.r[4].s64 = ctx.r[11].s64 + -19796;
	// 82481E94: 48971B75  bl 0x82df3a08
	ctx.lr = 0x82481E98;
	sub_82DF3A08(ctx, base);
	// 82481E98: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 82481E9C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82481EA0: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82481EA4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82481EA8: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82481EAC: 48121305  bl 0x825a31b0
	ctx.lr = 0x82481EB0;
	sub_825A31B0(ctx, base);
	// 82481EB0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82481EB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82481EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481EBC: 4811F8B5  bl 0x825a1770
	ctx.lr = 0x82481EC0;
	sub_825A1770(ctx, base);
	// 82481EC0: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82481EC4: 48971565  bl 0x82df3428
	ctx.lr = 0x82481EC8;
	sub_82DF3428(ctx, base);
	// 82481EC8: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82481ECC: 4BE46DED  bl 0x822c8cb8
	ctx.lr = 0x82481ED0;
	sub_822C8CB8(ctx, base);
	// 82481ED0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481ED4: 48971555  bl 0x82df3428
	ctx.lr = 0x82481ED8;
	sub_82DF3428(ctx, base);
	// 82481ED8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481EDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481EE0: 388BB2A0  addi r4, r11, -0x4d60
	ctx.r[4].s64 = ctx.r[11].s64 + -19808;
	// 82481EE4: 48971B25  bl 0x82df3a08
	ctx.lr = 0x82481EE8;
	sub_82DF3A08(ctx, base);
	// 82481EE8: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82481EEC: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82481EF0: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82481EF4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82481EF8: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82481EFC: 481212B5  bl 0x825a31b0
	ctx.lr = 0x82481F00;
	sub_825A31B0(ctx, base);
	// 82481F00: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82481F04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82481F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481F0C: 4811F865  bl 0x825a1770
	ctx.lr = 0x82481F10;
	sub_825A1770(ctx, base);
	// 82481F10: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 82481F14: 48971515  bl 0x82df3428
	ctx.lr = 0x82481F18;
	sub_82DF3428(ctx, base);
	// 82481F18: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82481F1C: 4BE46D9D  bl 0x822c8cb8
	ctx.lr = 0x82481F20;
	sub_822C8CB8(ctx, base);
	// 82481F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481F24: 48971505  bl 0x82df3428
	ctx.lr = 0x82481F28;
	sub_82DF3428(ctx, base);
	// 82481F28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481F2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481F30: 388B8FF4  addi r4, r11, -0x700c
	ctx.r[4].s64 = ctx.r[11].s64 + -28684;
	// 82481F34: 48971AD5  bl 0x82df3a08
	ctx.lr = 0x82481F38;
	sub_82DF3A08(ctx, base);
	// 82481F38: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82481F3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82481F40: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82481F44: 3BCB6D40  addi r30, r11, 0x6d40
	ctx.r[30].s64 = ctx.r[11].s64 + 27968;
	// 82481F48: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82481F4C: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82481F50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82481F54: C02A08A4  lfs f1, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82481F58: 48121259  bl 0x825a31b0
	ctx.lr = 0x82481F5C;
	sub_825A31B0(ctx, base);
	// 82481F5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82481F60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82481F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481F68: 4811F809  bl 0x825a1770
	ctx.lr = 0x82481F6C;
	sub_825A1770(ctx, base);
	// 82481F6C: 38610148  addi r3, r1, 0x148
	ctx.r[3].s64 = ctx.r[1].s64 + 328;
	// 82481F70: 489714B9  bl 0x82df3428
	ctx.lr = 0x82481F74;
	sub_82DF3428(ctx, base);
	// 82481F74: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 82481F78: 4BE46D41  bl 0x822c8cb8
	ctx.lr = 0x82481F7C;
	sub_822C8CB8(ctx, base);
	// 82481F7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481F80: 489714A9  bl 0x82df3428
	ctx.lr = 0x82481F84;
	sub_82DF3428(ctx, base);
	// 82481F84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481F88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481F8C: 388BB298  addi r4, r11, -0x4d68
	ctx.r[4].s64 = ctx.r[11].s64 + -19816;
	// 82481F90: 48971A79  bl 0x82df3a08
	ctx.lr = 0x82481F94;
	sub_82DF3A08(ctx, base);
	// 82481F94: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82481F98: 38610190  addi r3, r1, 0x190
	ctx.r[3].s64 = ctx.r[1].s64 + 400;
	// 82481F9C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82481FA0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82481FA4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82481FA8: 48121209  bl 0x825a31b0
	ctx.lr = 0x82481FAC;
	sub_825A31B0(ctx, base);
	// 82481FAC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82481FB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82481FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82481FB8: 4811F7B9  bl 0x825a1770
	ctx.lr = 0x82481FBC;
	sub_825A1770(ctx, base);
	// 82481FBC: 386101C8  addi r3, r1, 0x1c8
	ctx.r[3].s64 = ctx.r[1].s64 + 456;
	// 82481FC0: 48971469  bl 0x82df3428
	ctx.lr = 0x82481FC4;
	sub_82DF3428(ctx, base);
	// 82481FC4: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 82481FC8: 4BE46CF1  bl 0x822c8cb8
	ctx.lr = 0x82481FCC;
	sub_822C8CB8(ctx, base);
	// 82481FCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481FD0: 48971459  bl 0x82df3428
	ctx.lr = 0x82481FD4;
	sub_82DF3428(ctx, base);
	// 82481FD4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82481FD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82481FDC: 388BB290  addi r4, r11, -0x4d70
	ctx.r[4].s64 = ctx.r[11].s64 + -19824;
	// 82481FE0: 48971A29  bl 0x82df3a08
	ctx.lr = 0x82481FE4;
	sub_82DF3A08(ctx, base);
	// 82481FE4: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82481FE8: 38610210  addi r3, r1, 0x210
	ctx.r[3].s64 = ctx.r[1].s64 + 528;
	// 82481FEC: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82481FF0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82481FF4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82481FF8: 481211B9  bl 0x825a31b0
	ctx.lr = 0x82481FFC;
	sub_825A31B0(ctx, base);
	// 82481FFC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82482000: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82482004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482008: 4811F769  bl 0x825a1770
	ctx.lr = 0x8248200C;
	sub_825A1770(ctx, base);
	// 8248200C: 38610248  addi r3, r1, 0x248
	ctx.r[3].s64 = ctx.r[1].s64 + 584;
	// 82482010: 48971419  bl 0x82df3428
	ctx.lr = 0x82482014;
	sub_82DF3428(ctx, base);
	// 82482014: 38610228  addi r3, r1, 0x228
	ctx.r[3].s64 = ctx.r[1].s64 + 552;
	// 82482018: 4BE46CA1  bl 0x822c8cb8
	ctx.lr = 0x8248201C;
	sub_822C8CB8(ctx, base);
	// 8248201C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482020: 48971409  bl 0x82df3428
	ctx.lr = 0x82482024;
	sub_82DF3428(ctx, base);
	// 82482024: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82482028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248202C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82482030: CBA1FFD0  lfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82482034: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82482038: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8248203C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82482040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82482044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82482048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82482048 size=1012
    let mut pc: u32 = 0x82482048;
    'dispatch: loop {
        match pc {
            0x82482048 => {
    //   block [0x82482048..0x8248243C)
	// 82482048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248204C: 48D2610D  bl 0x831a8158
	ctx.lr = 0x82482050;
	sub_831A8130(ctx, base);
	// 82482050: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82482054: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82482058: 817D00D4  lwz r11, 0xd4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(212 as u32) ) } as u64;
	// 8248205C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82482060: 409A03D4  bne cr6, 0x82482434
	if !ctx.cr[6].eq {
	pc = 0x82482434; continue 'dispatch;
	}
	// 82482064: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82482068: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248206C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82482070: 3B8B6620  addi r28, r11, 0x6620
	ctx.r[28].s64 = ctx.r[11].s64 + 26144;
	// 82482074: 913D00D4  stw r9, 0xd4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(212 as u32), ctx.r[9].u32 ) };
	// 82482078: 816A6630  lwz r11, 0x6630(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26160 as u32) ) } as u64;
	// 8248207C: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82482080: 4082001C  bne 0x8248209c
	if !ctx.cr[0].eq {
	pc = 0x8248209C; continue 'dispatch;
	}
	// 82482084: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82482088: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8248208C: 3929A2F0  addi r9, r9, -0x5d10
	ctx.r[9].s64 = ctx.r[9].s64 + -23824;
	// 82482090: 916A6630  stw r11, 0x6630(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26160 as u32), ctx.r[11].u32 ) };
	// 82482094: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82482440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82482440 size=196
    let mut pc: u32 = 0x82482440;
    'dispatch: loop {
        match pc {
            0x82482440 => {
    //   block [0x82482440..0x82482504)
	// 82482440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82482444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82482448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248244C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82482450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82482454: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82482458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248245C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82482460: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82482464: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82482468: 4BE3E4D1  bl 0x822c0938
	ctx.lr = 0x8248246C;
	sub_822C0938(ctx, base);
	// 8248246C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82482470: 41820028  beq 0x82482498
	if ctx.cr[0].eq {
	pc = 0x82482498; continue 'dispatch;
	}
	// 82482474: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482478: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8248247C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82482480: 392BB324  addi r9, r11, -0x4cdc
	ctx.r[9].s64 = ctx.r[11].s64 + -19676;
	// 82482484: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82482488: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8248248C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82482490: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82482494: 48000008  b 0x8248249c
	pc = 0x8248249C; continue 'dispatch;
	// 82482498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248249C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824824A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824824A4: 409A0044  bne cr6, 0x824824e8
	if !ctx.cr[6].eq {
	pc = 0x824824E8; continue 'dispatch;
	}
	// 824824A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824824AC: 419A001C  beq cr6, 0x824824c8
	if ctx.cr[6].eq {
	pc = 0x824824C8; continue 'dispatch;
	}
	// 824824B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824824B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824824B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824824BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824824C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824824C4: 4E800421  bctrl
	ctx.lr = 0x824824C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824824C8: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 824824CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824824D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824824D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 824824D8: 816B6E88  lwz r11, 0x6e88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28296 as u32) ) } as u64;
	// 824824DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824824E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824824E4: 4BE3DB1D  bl 0x822c0000
	ctx.lr = 0x824824E8;
	sub_822C0000(ctx, base);
	// 824824E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824824EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824824F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824824F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824824F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824824FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82482500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82482508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82482508 size=132
    let mut pc: u32 = 0x82482508;
    'dispatch: loop {
        match pc {
            0x82482508 => {
    //   block [0x82482508..0x8248258C)
	// 82482508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248250C: 48D25C5D  bl 0x831a8168
	ctx.lr = 0x82482510;
	sub_831A8130(ctx, base);
	// 82482510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82482514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82482518: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248251C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82482520: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82482524: 4BFE7235  bl 0x82469758
	ctx.lr = 0x82482528;
	sub_82469758(ctx, base);
	// 82482528: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 8248252C: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 82482530: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82482534: 93BF00D4  stw r29, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 82482538: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8248253C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82482540: 397F00E0  addi r11, r31, 0xe0
	ctx.r[11].s64 = ctx.r[31].s64 + 224;
	// 82482544: 38E7B338  addi r7, r7, -0x4cc8
	ctx.r[7].s64 = ctx.r[7].s64 + -19656;
	// 82482548: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248254C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82482550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482554: 90DF0120  stw r6, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[6].u32 ) };
	// 82482558: 13E8E407  vcmpneb. (lvlx128) v31, v8, v28
	tmp.u32 = ctx.r[8].u32 + ctx.r[28].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8248255C: 13C9E407  vcmpneb. (lvlx128) v30, v9, v28
	tmp.u32 = ctx.r[9].u32 + ctx.r[28].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82482560: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82482564: 13AAE407  vcmpneb. (lvlx128) v29, v10, v28
	tmp.u32 = ctx.r[10].u32 + ctx.r[28].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82482568: 1380E407  vcmpneb. (lvlx128) v28, v0, v28
	tmp.u32 = ctx.r[28].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82482590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82482590 size=88
    let mut pc: u32 = 0x82482590;
    'dispatch: loop {
        match pc {
            0x82482590 => {
    //   block [0x82482590..0x824825E8)
	// 82482590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82482594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82482598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248259C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824825A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824825A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824825A8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824825AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824825B0: 396B770C  addi r11, r11, 0x770c
	ctx.r[11].s64 = ctx.r[11].s64 + 30476;
	// 824825B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824825B8: 489D9FD1  bl 0x82e5c588
	ctx.lr = 0x824825BC;
	sub_82E5C588(ctx, base);
	// 824825BC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824825C0: 4182000C  beq 0x824825cc
	if ctx.cr[0].eq {
	pc = 0x824825CC; continue 'dispatch;
	}
	// 824825C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824825C8: 4896FE11  bl 0x82df23d8
	ctx.lr = 0x824825CC;
	sub_82DF23D8(ctx, base);
	// 824825CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824825D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824825D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824825D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824825DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824825E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824825E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824825E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824825E8 size=120
    let mut pc: u32 = 0x824825E8;
    'dispatch: loop {
        match pc {
            0x824825E8 => {
    //   block [0x824825E8..0x82482660)
	// 824825E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824825EC: 48D25B81  bl 0x831a816c
	ctx.lr = 0x824825F0;
	sub_831A8130(ctx, base);
	// 824825F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824825F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824825F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824825FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82482600: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82482604: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82482608: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8248260C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82482610: 4896FDD9  bl 0x82df23e8
	ctx.lr = 0x82482614;
	sub_82DF23E8(ctx, base);
	// 82482614: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82482618: 41820014  beq 0x8248262c
	if ctx.cr[0].eq {
	pc = 0x8248262C; continue 'dispatch;
	}
	// 8248261C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82482620: 486D8C01  bl 0x82b5b220
	ctx.lr = 0x82482624;
	sub_82B5B220(ctx, base);
	// 82482624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82482628: 48000008  b 0x82482630
	pc = 0x82482630; continue 'dispatch;
	// 8248262C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82482630: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82482634: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82482638: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248263C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82482640: 4BFFFE01  bl 0x82482440
	ctx.lr = 0x82482644;
	sub_82482440(ctx, base);
	// 82482644: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82482648: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248264C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82482650: 4BE3D9B1  bl 0x822c0000
	ctx.lr = 0x82482654;
	sub_822C0000(ctx, base);
	// 82482654: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82482658: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248265C: 48D25B60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82482660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82482660 size=964
    let mut pc: u32 = 0x82482660;
    'dispatch: loop {
        match pc {
            0x82482660 => {
    //   block [0x82482660..0x82482A24)
	// 82482660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82482664: 48D25B09  bl 0x831a816c
	ctx.lr = 0x82482668;
	sub_831A8130(ctx, base);
	// 82482668: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8248266C: 48D2640D  bl 0x831a8a78
	ctx.lr = 0x82482670;
	sub_831A8A40(ctx, base);
	// 82482670: 9421FCB0  stwu r1, -0x350(r1)
	ea = ctx.r[1].u32.wrapping_add(-848 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82482674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82482678: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248267C: 419A0398  beq cr6, 0x82482a14
	if ctx.cr[6].eq {
	pc = 0x82482A14; continue 'dispatch;
	}
	// 82482680: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482684: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482688: 388BB2D0  addi r4, r11, -0x4d30
	ctx.r[4].s64 = ctx.r[11].s64 + -19760;
	// 8248268C: 4897137D  bl 0x82df3a08
	ctx.lr = 0x82482690;
	sub_82DF3A08(ctx, base);
	// 82482690: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82482694: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82482698: 3BCB6634  addi r30, r11, 0x6634
	ctx.r[30].s64 = ctx.r[11].s64 + 26164;
	// 8248269C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824826A0: 481208D1  bl 0x825a2f70
	ctx.lr = 0x824826A4;
	sub_825A2F70(ctx, base);
	// 824826A4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824826A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824826AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824826B0: 4811FAD9  bl 0x825a2188
	ctx.lr = 0x824826B4;
	sub_825A2188(ctx, base);
	// 824826B4: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 824826B8: 48970D71  bl 0x82df3428
	ctx.lr = 0x824826BC;
	sub_82DF3428(ctx, base);
	// 824826BC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 824826C0: 4BE465F9  bl 0x822c8cb8
	ctx.lr = 0x824826C4;
	sub_822C8CB8(ctx, base);
	// 824826C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824826C8: 48970D61  bl 0x82df3428
	ctx.lr = 0x824826CC;
	sub_82DF3428(ctx, base);
	// 824826CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824826D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824826D4: 388BB2C4  addi r4, r11, -0x4d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -19772;
	// 824826D8: 48971331  bl 0x82df3a08
	ctx.lr = 0x824826DC;
	sub_82DF3A08(ctx, base);
	// 824826DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824826E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 824826E4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824826E8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 824826EC: 38610190  addi r3, r1, 0x190
	ctx.r[3].s64 = ctx.r[1].s64 + 400;
	// 824826F0: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824826F4: C3CA6218  lfs f30, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824826F8: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 824826FC: C3A90790  lfs f29, 0x790(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(1936 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82482700: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82482704: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82482708: 48120AA9  bl 0x825a31b0
	ctx.lr = 0x8248270C;
	sub_825A31B0(ctx, base);
	// 8248270C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82482710: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82482714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482718: 4811F059  bl 0x825a1770
	ctx.lr = 0x8248271C;
	sub_825A1770(ctx, base);
	// 8248271C: 386101C8  addi r3, r1, 0x1c8
	ctx.r[3].s64 = ctx.r[1].s64 + 456;
	// 82482720: 48970D09  bl 0x82df3428
	ctx.lr = 0x82482724;
	sub_82DF3428(ctx, base);
	// 82482724: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 82482728: 4BE46591  bl 0x822c8cb8
	ctx.lr = 0x8248272C;
	sub_822C8CB8(ctx, base);
	// 8248272C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482730: 48970CF9  bl 0x82df3428
	ctx.lr = 0x82482734;
	sub_82DF3428(ctx, base);
	// 82482734: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248273C: 388BB2B8  addi r4, r11, -0x4d48
	ctx.r[4].s64 = ctx.r[11].s64 + -19784;
	// 82482740: 489712C9  bl 0x82df3a08
	ctx.lr = 0x82482744;
	sub_82DF3A08(ctx, base);
	// 82482744: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82482748: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8248274C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82482750: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82482754: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82482758: 48120A59  bl 0x825a31b0
	ctx.lr = 0x8248275C;
	sub_825A31B0(ctx, base);
	// 8248275C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82482760: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82482764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482768: 4811F009  bl 0x825a1770
	ctx.lr = 0x8248276C;
	sub_825A1770(ctx, base);
	// 8248276C: 38610148  addi r3, r1, 0x148
	ctx.r[3].s64 = ctx.r[1].s64 + 328;
	// 82482770: 48970CB9  bl 0x82df3428
	ctx.lr = 0x82482774;
	sub_82DF3428(ctx, base);
	// 82482774: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 82482778: 4BE46541  bl 0x822c8cb8
	ctx.lr = 0x8248277C;
	sub_822C8CB8(ctx, base);
	// 8248277C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482780: 48970CA9  bl 0x82df3428
	ctx.lr = 0x82482784;
	sub_82DF3428(ctx, base);
	// 82482784: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248278C: 388BB2AC  addi r4, r11, -0x4d54
	ctx.r[4].s64 = ctx.r[11].s64 + -19796;
	// 82482790: 48971279  bl 0x82df3a08
	ctx.lr = 0x82482794;
	sub_82DF3A08(ctx, base);
	// 82482794: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 82482798: 38610210  addi r3, r1, 0x210
	ctx.r[3].s64 = ctx.r[1].s64 + 528;
	// 8248279C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 824827A0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 824827A4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 824827A8: 48120A09  bl 0x825a31b0
	ctx.lr = 0x824827AC;
	sub_825A31B0(ctx, base);
	// 824827AC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824827B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824827B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824827B8: 4811EFB9  bl 0x825a1770
	ctx.lr = 0x824827BC;
	sub_825A1770(ctx, base);
	// 824827BC: 38610248  addi r3, r1, 0x248
	ctx.r[3].s64 = ctx.r[1].s64 + 584;
	// 824827C0: 48970C69  bl 0x82df3428
	ctx.lr = 0x824827C4;
	sub_82DF3428(ctx, base);
	// 824827C4: 38610228  addi r3, r1, 0x228
	ctx.r[3].s64 = ctx.r[1].s64 + 552;
	// 824827C8: 4BE464F1  bl 0x822c8cb8
	ctx.lr = 0x824827CC;
	sub_822C8CB8(ctx, base);
	// 824827CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824827D0: 48970C59  bl 0x82df3428
	ctx.lr = 0x824827D4;
	sub_82DF3428(ctx, base);
	// 824827D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824827D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824827DC: 388BB2A0  addi r4, r11, -0x4d60
	ctx.r[4].s64 = ctx.r[11].s64 + -19808;
	// 824827E0: 48971229  bl 0x82df3a08
	ctx.lr = 0x824827E4;
	sub_82DF3A08(ctx, base);
	// 824827E4: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 824827E8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 824827EC: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 824827F0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 824827F4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 824827F8: 481209B9  bl 0x825a31b0
	ctx.lr = 0x824827FC;
	sub_825A31B0(ctx, base);
	// 824827FC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82482800: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82482804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482808: 4811EF69  bl 0x825a1770
	ctx.lr = 0x8248280C;
	sub_825A1770(ctx, base);
	// 8248280C: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82482810: 48970C19  bl 0x82df3428
	ctx.lr = 0x82482814;
	sub_82DF3428(ctx, base);
	// 82482814: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82482818: 4BE464A1  bl 0x822c8cb8
	ctx.lr = 0x8248281C;
	sub_822C8CB8(ctx, base);
	// 8248281C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482820: 48970C09  bl 0x82df3428
	ctx.lr = 0x82482824;
	sub_82DF3428(ctx, base);
	// 82482824: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482828: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248282C: 388B8FF4  addi r4, r11, -0x700c
	ctx.r[4].s64 = ctx.r[11].s64 + -28684;
	// 82482830: 489711D9  bl 0x82df3a08
	ctx.lr = 0x82482834;
	sub_82DF3A08(ctx, base);
	// 82482834: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82482838: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248283C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82482840: 3BAB6E74  addi r29, r11, 0x6e74
	ctx.r[29].s64 = ctx.r[11].s64 + 28276;
	// 82482844: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82482848: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 8248284C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82482850: C38A08A4  lfs f28, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82482854: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 82482858: 48120959  bl 0x825a31b0
	ctx.lr = 0x8248285C;
	sub_825A31B0(ctx, base);
	// 8248285C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82482860: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82482864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482868: 4811EF09  bl 0x825a1770
	ctx.lr = 0x8248286C;
	sub_825A1770(ctx, base);
	// 8248286C: 386102C8  addi r3, r1, 0x2c8
	ctx.r[3].s64 = ctx.r[1].s64 + 712;
	// 82482870: 48970BB9  bl 0x82df3428
	ctx.lr = 0x82482874;
	sub_82DF3428(ctx, base);
	// 82482874: 386102A8  addi r3, r1, 0x2a8
	ctx.r[3].s64 = ctx.r[1].s64 + 680;
	// 82482878: 4BE46441  bl 0x822c8cb8
	ctx.lr = 0x8248287C;
	sub_822C8CB8(ctx, base);
	// 8248287C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482880: 48970BA9  bl 0x82df3428
	ctx.lr = 0x82482884;
	sub_82DF3428(ctx, base);
	// 82482884: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248288C: 388BB298  addi r4, r11, -0x4d68
	ctx.r[4].s64 = ctx.r[11].s64 + -19816;
	// 82482890: 48971179  bl 0x82df3a08
	ctx.lr = 0x82482894;
	sub_82DF3A08(ctx, base);
	// 82482894: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 82482898: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8248289C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 824828A0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 824828A4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 824828A8: 48120909  bl 0x825a31b0
	ctx.lr = 0x824828AC;
	sub_825A31B0(ctx, base);
	// 824828AC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824828B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824828B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824828B8: 4811EEB9  bl 0x825a1770
	ctx.lr = 0x824828BC;
	sub_825A1770(ctx, base);
	// 824828BC: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 824828C0: 48970B69  bl 0x82df3428
	ctx.lr = 0x824828C4;
	sub_82DF3428(ctx, base);
	// 824828C4: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 824828C8: 4BE463F1  bl 0x822c8cb8
	ctx.lr = 0x824828CC;
	sub_822C8CB8(ctx, base);
	// 824828CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824828D0: 48970B59  bl 0x82df3428
	ctx.lr = 0x824828D4;
	sub_82DF3428(ctx, base);
	// 824828D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824828D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824828DC: 388BB290  addi r4, r11, -0x4d70
	ctx.r[4].s64 = ctx.r[11].s64 + -19824;
	// 824828E0: 48971129  bl 0x82df3a08
	ctx.lr = 0x824828E4;
	sub_82DF3A08(ctx, base);
	// 824828E4: 389D0008  addi r4, r29, 8
	ctx.r[4].s64 = ctx.r[29].s64 + 8;
	// 824828E8: 38610150  addi r3, r1, 0x150
	ctx.r[3].s64 = ctx.r[1].s64 + 336;
	// 824828EC: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 824828F0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 824828F4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 824828F8: 481208B9  bl 0x825a31b0
	ctx.lr = 0x824828FC;
	sub_825A31B0(ctx, base);
	// 824828FC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82482900: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82482904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482908: 4811EE69  bl 0x825a1770
	ctx.lr = 0x8248290C;
	sub_825A1770(ctx, base);
	// 8248290C: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 82482910: 48970B19  bl 0x82df3428
	ctx.lr = 0x82482914;
	sub_82DF3428(ctx, base);
	// 82482914: 38610168  addi r3, r1, 0x168
	ctx.r[3].s64 = ctx.r[1].s64 + 360;
	// 82482918: 4BE463A1  bl 0x822c8cb8
	ctx.lr = 0x8248291C;
	sub_822C8CB8(ctx, base);
	// 8248291C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482920: 48970B09  bl 0x82df3428
	ctx.lr = 0x82482924;
	sub_82DF3428(ctx, base);
	// 82482924: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248292C: 388BB380  addi r4, r11, -0x4c80
	ctx.r[4].s64 = ctx.r[11].s64 + -19584;
	// 82482930: 489710D9  bl 0x82df3a08
	ctx.lr = 0x82482934;
	sub_82DF3A08(ctx, base);
	// 82482934: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82482938: 386101D0  addi r3, r1, 0x1d0
	ctx.r[3].s64 = ctx.r[1].s64 + 464;
	// 8248293C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82482940: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82482944: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 82482948: 48120869  bl 0x825a31b0
	ctx.lr = 0x8248294C;
	sub_825A31B0(ctx, base);
	// 8248294C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82482950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82482954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82482958: 4811EE19  bl 0x825a1770
	ctx.lr = 0x8248295C;
	sub_825A1770(ctx, base);
	// 8248295C: 38610208  addi r3, r1, 0x208
	ctx.r[3].s64 = ctx.r[1].s64 + 520;
	// 82482960: 48970AC9  bl 0x82df3428
	ctx.lr = 0x82482964;
	sub_82DF3428(ctx, base);
	// 82482964: 386101E8  addi r3, r1, 0x1e8
	ctx.r[3].s64 = ctx.r[1].s64 + 488;
	// 82482968: 4BE46351  bl 0x822c8cb8
	ctx.lr = 0x8248296C;
	sub_822C8CB8(ctx, base);
	// 8248296C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482970: 48970AB9  bl 0x82df3428
	ctx.lr = 0x82482974;
	sub_82DF3428(ctx, base);
	// 82482974: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82482978: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248297C: 388BB374  addi r4, r11, -0x4c8c
	ctx.r[4].s64 = ctx.r[11].s64 + -19596;
	// 82482980: 48971089  bl 0x82df3a08
	ctx.lr = 0x82482984;
	sub_82DF3A08(ctx, base);
	// 82482984: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 82482988: 38610250  addi r3, r1, 0x250
	ctx.r[3].s64 = ctx.r[1].s64 + 592;
	// 8248298C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82482990: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82482994: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82482998: 48120819  bl 0x825a31b0
	ctx.lr = 0x8248299C;
	sub_825A31B0(ctx, base);
	// 8248299C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824829A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824829A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824829A8: 4811EDC9  bl 0x825a1770
	ctx.lr = 0x824829AC;
	sub_825A1770(ctx, base);
	// 824829AC: 38610288  addi r3, r1, 0x288
	ctx.r[3].s64 = ctx.r[1].s64 + 648;
	// 824829B0: 48970A79  bl 0x82df3428
	ctx.lr = 0x824829B4;
	sub_82DF3428(ctx, base);
	// 824829B4: 38610268  addi r3, r1, 0x268
	ctx.r[3].s64 = ctx.r[1].s64 + 616;
	// 824829B8: 4BE46301  bl 0x822c8cb8
	ctx.lr = 0x824829BC;
	sub_822C8CB8(ctx, base);
	// 824829BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824829C0: 48970A69  bl 0x82df3428
	ctx.lr = 0x824829C4;
	sub_82DF3428(ctx, base);
	// 824829C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824829C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824829CC: 388BB368  addi r4, r11, -0x4c98
	ctx.r[4].s64 = ctx.r[11].s64 + -19608;
	// 824829D0: 48971039  bl 0x82df3a08
	ctx.lr = 0x824829D4;
	sub_82DF3A08(ctx, base);
	// 824829D4: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 824829D8: 386102D0  addi r3, r1, 0x2d0
	ctx.r[3].s64 = ctx.r[1].s64 + 720;
	// 824829DC: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 824829E0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 824829E4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 824829E8: 481207C9  bl 0x825a31b0
	ctx.lr = 0x824829EC;
	sub_825A31B0(ctx, base);
	// 824829EC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824829F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824829F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824829F8: 4811ED79  bl 0x825a1770
	ctx.lr = 0x824829FC;
	sub_825A1770(ctx, base);
	// 824829FC: 38610308  addi r3, r1, 0x308
	ctx.r[3].s64 = ctx.r[1].s64 + 776;
	// 82482A00: 48970A29  bl 0x82df3428
	ctx.lr = 0x82482A04;
	sub_82DF3428(ctx, base);
	// 82482A04: 386102E8  addi r3, r1, 0x2e8
	ctx.r[3].s64 = ctx.r[1].s64 + 744;
	// 82482A08: 4BE462B1  bl 0x822c8cb8
	ctx.lr = 0x82482A0C;
	sub_822C8CB8(ctx, base);
	// 82482A0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82482A10: 48970A19  bl 0x82df3428
	ctx.lr = 0x82482A14;
	sub_82DF3428(ctx, base);
	// 82482A14: 38210350  addi r1, r1, 0x350
	ctx.r[1].s64 = ctx.r[1].s64 + 848;
	// 82482A18: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82482A1C: 48D260A9  bl 0x831a8ac4
	ctx.lr = 0x82482A20;
	sub_831A8A8C(ctx, base);
	// 82482A20: 48D2579C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82482A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82482A28 size=1904
    let mut pc: u32 = 0x82482A28;
    'dispatch: loop {
        match pc {
            0x82482A28 => {
    //   block [0x82482A28..0x82483198)
	// 82482A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82482A2C: 48D2572D  bl 0x831a8158
	ctx.lr = 0x82482A30;
	sub_831A8130(ctx, base);
	// 82482A30: DBC1FFA8  stfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 82482A34: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82482A38: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82483198 size=12
    let mut pc: u32 = 0x82483198;
    'dispatch: loop {
        match pc {
            0x82483198 => {
    //   block [0x82483198..0x824831A4)
	// 82483198: D02300D0  stfs f1, 0xd0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 8248319C: D04300D4  stfs f2, 0xd4(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 824831A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824831A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824831A8 size=12
    let mut pc: u32 = 0x824831A8;
    'dispatch: loop {
        match pc {
            0x824831A8 => {
    //   block [0x824831A8..0x824831B4)
	// 824831A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824831AC: 996300E8  stb r11, 0xe8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), ctx.r[11].u8 ) };
	// 824831B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824831B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824831B8 size=164
    let mut pc: u32 = 0x824831B8;
    'dispatch: loop {
        match pc {
            0x824831B8 => {
    //   block [0x824831B8..0x8248325C)
	// 824831B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824831BC: 48D24FAD  bl 0x831a8168
	ctx.lr = 0x824831C0;
	sub_831A8130(ctx, base);
	// 824831C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824831C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824831C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824831CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824831D0: 57BC063F  clrlwi. r28, r29, 0x18
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824831D4: 41820038  beq 0x8248320c
	if ctx.cr[0].eq {
	pc = 0x8248320C; continue 'dispatch;
	}
	// 824831D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824831DC: 48D267AD  bl 0x831a9988
	ctx.lr = 0x824831E0;
	sub_831A9988(ctx, base);
	// 824831E0: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 824831E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824831E8: 386B6F40  addi r3, r11, 0x6f40
	ctx.r[3].s64 = ctx.r[11].s64 + 28480;
	// 824831EC: 48D24F0D  bl 0x831a80f8
	ctx.lr = 0x824831F0;
	sub_831A80F8(ctx, base);
	// 824831F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824831F4: 41820018  beq 0x8248320c
	if ctx.cr[0].eq {
	pc = 0x8248320C; continue 'dispatch;
	}
	// 824831F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824831FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82483200: 482D18E9  bl 0x82754ae8
	ctx.lr = 0x82483204;
	sub_82754AE8(ctx, base);
	// 82483204: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82483208: 4800004C  b 0x82483254
	pc = 0x82483254; continue 'dispatch;
	// 8248320C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82483210: 419A0034  beq cr6, 0x82483244
	if ctx.cr[6].eq {
	pc = 0x82483244; continue 'dispatch;
	}
	// 82483214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483218: 48D26771  bl 0x831a9988
	ctx.lr = 0x8248321C;
	sub_831A9988(ctx, base);
	// 8248321C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82483220: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82483224: 386B6F10  addi r3, r11, 0x6f10
	ctx.r[3].s64 = ctx.r[11].s64 + 28432;
	// 82483228: 48D24ED1  bl 0x831a80f8
	ctx.lr = 0x8248322C;
	sub_831A80F8(ctx, base);
	// 8248322C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82483230: 41820014  beq 0x82483244
	if ctx.cr[0].eq {
	pc = 0x82483244; continue 'dispatch;
	}
	// 82483234: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82483238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248323C: 4BFFFF6D  bl 0x824831a8
	ctx.lr = 0x82483240;
	sub_824831A8(ctx, base);
	// 82483240: 4BFFFFC4  b 0x82483204
	pc = 0x82483204; continue 'dispatch;
	// 82483244: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82483248: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248324C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82483250: 48CFB239  bl 0x8317e488
	ctx.lr = 0x82483254;
	sub_8317E488(ctx, base);
	// 82483254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82483258: 48D24F60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483260 size=168
    let mut pc: u32 = 0x82483260;
    'dispatch: loop {
        match pc {
            0x82483260 => {
    //   block [0x82483260..0x82483308)
	// 82483260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248326C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82483270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483278: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248327C: 4BFE64DD  bl 0x82469758
	ctx.lr = 0x82483280;
	sub_82469758(ctx, base);
	// 82483280: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82483284: 9BDF00E8  stb r30, 0xe8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u8 ) };
	// 82483288: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 8248328C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82483290: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82483294: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82483298: 38E7B3EC  addi r7, r7, -0x4c14
	ctx.r[7].s64 = ctx.r[7].s64 + -19476;
	// 8248329C: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824832A0: C1A9B3E4  lfs f13, -0x4c1c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19484 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824832A4: 997F00E9  stb r11, 0xe9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(233 as u32), ctx.r[11].u8 ) };
	// 824832A8: C18808A8  lfs f12, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824832AC: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824832B0: D01F00D0  stfs f0, 0xd0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 824832B4: 997F00EA  stb r11, 0xea(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(234 as u32), ctx.r[11].u8 ) };
	// 824832B8: D01F00D4  stfs f0, 0xd4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 824832BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824832C0: D1BF00D8  stfs f13, 0xd8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 824832C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824832C8: D19F00DC  stfs f12, 0xdc(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 824832CC: D01F00E0  stfs f0, 0xe0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 824832D0: D01F00E4  stfs f0, 0xe4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 824832D4: D01F0110  stfs f0, 0x110(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 824832D8: D01F0114  stfs f0, 0x114(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 824832DC: D01F0118  stfs f0, 0x118(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 824832E0: D01F011C  stfs f0, 0x11c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 824832E4: D01F0120  stfs f0, 0x120(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 824832E8: 4BFE6469  bl 0x82469750
	ctx.lr = 0x824832EC;
	sub_82469750(ctx, base);
	// 824832EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824832F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824832F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824832F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824832FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82483300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82483304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483308 size=100
    let mut pc: u32 = 0x82483308;
    'dispatch: loop {
        match pc {
            0x82483308 => {
    //   block [0x82483308..0x8248336C)
	// 82483308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248330C: 48D24E61  bl 0x831a816c
	ctx.lr = 0x82483310;
	sub_831A8130(ctx, base);
	// 82483310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82483318: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248331C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483320: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82483324: 3BDF0100  addi r30, r31, 0x100
	ctx.r[30].s64 = ctx.r[31].s64 + 256;
	// 82483328: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248332C: 3BBF00F0  addi r29, r31, 0xf0
	ctx.r[29].s64 = ctx.r[31].s64 + 240;
	// 82483330: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82483334: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82483338: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8248333C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82483340: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82483344: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82483348: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8248334C: 4BFE6525  bl 0x82469870
	ctx.lr = 0x82483350;
	sub_82469870(ctx, base);
	// 82483350: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82483354: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82483358: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8248335C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483360: 4BFE6539  bl 0x82469898
	ctx.lr = 0x82483364;
	sub_82469898(ctx, base);
	// 82483364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82483368: 48D24E54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483370 size=100
    let mut pc: u32 = 0x82483370;
    'dispatch: loop {
        match pc {
            0x82483370 => {
    //   block [0x82483370..0x824833D4)
	// 82483370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248337C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483384: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82483388: 48CD1279  bl 0x83154600
	ctx.lr = 0x8248338C;
	sub_83154600(ctx, base);
	// 8248338C: 39030130  addi r8, r3, 0x130
	ctx.r[8].s64 = ctx.r[3].s64 + 304;
	// 82483390: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82483394: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82483398: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8248339C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824833A0: 13E04407  vcmpneb. (lvlx128) v31, v0, v8
	tmp.u32 = ctx.r[8].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824833A4: 13C94407  vcmpneb. (lvlx128) v30, v9, v8
	tmp.u32 = ctx.r[9].u32 + ctx.r[8].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824833A8: 13AA4407  vcmpneb. (lvlx128) v29, v10, v8
	tmp.u32 = ctx.r[10].u32 + ctx.r[8].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824833AC: 138B4407  vcmpneb. (lvlx128) v28, v11, v8
	tmp.u32 = ctx.r[11].u32 + ctx.r[8].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824833D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824833D8 size=928
    let mut pc: u32 = 0x824833D8;
    'dispatch: loop {
        match pc {
            0x824833D8 => {
    //   block [0x824833D8..0x82483778)
	// 824833D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824833DC: 48D24D8D  bl 0x831a8168
	ctx.lr = 0x824833E0;
	sub_831A8130(ctx, base);
	// 824833E0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 824833E4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 824833E8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483778 size=876
    let mut pc: u32 = 0x82483778;
    'dispatch: loop {
        match pc {
            0x82483778 => {
    //   block [0x82483778..0x82483AE4)
	// 82483778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248377C: 48D249E9  bl 0x831a8164
	ctx.lr = 0x82483780;
	sub_831A8130(ctx, base);
	// 82483780: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82483784: 48D252F5  bl 0x831a8a78
	ctx.lr = 0x82483788;
	sub_831A8A40(ctx, base);
	// 82483788: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248378C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82483790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483794: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82483798: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8248379C: 54DC063F  clrlwi. r28, r6, 0x18
	ctx.r[28].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824837A0: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824837A4: FF80F890  fmr f28, f31
	ctx.f[28].f64 = ctx.f[31].f64;
	// 824837A8: FFA0F890  fmr f29, f31
	ctx.f[29].f64 = ctx.f[31].f64;
	// 824837AC: 40820034  bne 0x824837e0
	if !ctx.cr[0].eq {
	pc = 0x824837E0; continue 'dispatch;
	}
	// 824837B0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 824837B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824837B8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824837BC: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 824837C0: 4896E439  bl 0x82df1bf8
	ctx.lr = 0x824837C4;
	sub_82DF1BF8(ctx, base);
	// 824837C4: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824837C8: 486C3A79  bl 0x82b47240
	ctx.lr = 0x824837CC;
	sub_82B47240(ctx, base);
	// 824837CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824837D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824837D4: 4896E4BD  bl 0x82df1c90
	ctx.lr = 0x824837D8;
	sub_82DF1C90(ctx, base);
	// 824837D8: C39B0014  lfs f28, 0x14(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 824837DC: C3BB0010  lfs f29, 0x10(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 824837E0: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 824837E4: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824837E8: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 824837EC: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82483AE8 size=16
    let mut pc: u32 = 0x82483AE8;
    'dispatch: loop {
        match pc {
            0x82483AE8 => {
    //   block [0x82483AE8..0x82483AF8)
	// 82483AE8: 896300EA  lbz r11, 0xea(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(234 as u32) ) } as u64;
	// 82483AEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82483AF0: 41820008  beq 0x82483af8
	if ctx.cr[0].eq {
		sub_82483AF8(ctx, base);
		return;
	}
	// 82483AF4: 4BFFF814  b 0x82483308
	sub_82483308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82483AF8 size=4
    let mut pc: u32 = 0x82483AF8;
    'dispatch: loop {
        match pc {
            0x82483AF8 => {
    //   block [0x82483AF8..0x82483AFC)
	// 82483AF8: 4BFFF8E0  b 0x824833d8
	sub_824833D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483B00 size=100
    let mut pc: u32 = 0x82483B00;
    'dispatch: loop {
        match pc {
            0x82483B00 => {
    //   block [0x82483B00..0x82483B64)
	// 82483B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483B08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82483B0C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82483B10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483B14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483B18: 489D5C51  bl 0x82e59768
	ctx.lr = 0x82483B1C;
	sub_82E59768(ctx, base);
	// 82483B1C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82483B20: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 82483B24: 489D9355  bl 0x82e5ce78
	ctx.lr = 0x82483B28;
	sub_82E5CE78(ctx, base);
	// 82483B28: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82483B2C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82483B30: 4182001C  beq 0x82483b4c
	if ctx.cr[0].eq {
	pc = 0x82483B4C; continue 'dispatch;
	}
	// 82483B34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483B38: C3FF007C  lfs f31, 0x7c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82483B3C: 48CD0AC5  bl 0x83154600
	ctx.lr = 0x82483B40;
	sub_83154600(ctx, base);
	// 82483B40: C03F0074  lfs f1, 0x74(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82483B44: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82483B48: 4BFE0FE1  bl 0x82464b28
	ctx.lr = 0x82483B4C;
	sub_82464B28(ctx, base);
	// 82483B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82483B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82483B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82483B58: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82483B5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82483B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483B68 size=56
    let mut pc: u32 = 0x82483B68;
    'dispatch: loop {
        match pc {
            0x82483B68 => {
    //   block [0x82483B68..0x82483BA0)
	// 82483B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483B74: 48CD0A8D  bl 0x83154600
	ctx.lr = 0x82483B78;
	sub_83154600(ctx, base);
	// 82483B78: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82483B7C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82483B80: C00B6F94  lfs f0, 0x6f94(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28564 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483B84: C1AACEE4  lfs f13, -0x311c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82483B88: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82483B8C: D0030078  stfs f0, 0x78(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82483B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82483B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82483B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82483B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82483BA0 size=40
    let mut pc: u32 = 0x82483BA0;
    'dispatch: loop {
        match pc {
            0x82483BA0 => {
    //   block [0x82483BA0..0x82483BC8)
	// 82483BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483BAC: 48CD0A55  bl 0x83154600
	ctx.lr = 0x82483BB0;
	sub_83154600(ctx, base);
	// 82483BB0: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82483BB4: 489F97BD  bl 0x82e7d370
	ctx.lr = 0x82483BB8;
	sub_82E7D370(ctx, base);
	// 82483BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82483BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82483BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82483BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483BC8 size=92
    let mut pc: u32 = 0x82483BC8;
    'dispatch: loop {
        match pc {
            0x82483BC8 => {
    //   block [0x82483BC8..0x82483C24)
	// 82483BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483BD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82483BD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82483BD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483BDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483BE0: 48CD0A21  bl 0x83154600
	ctx.lr = 0x82483BE4;
	sub_83154600(ctx, base);
	// 82483BE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82483BE8: 48CD0A19  bl 0x83154600
	ctx.lr = 0x82483BEC;
	sub_83154600(ctx, base);
	// 82483BEC: 4BFE0F7D  bl 0x82464b68
	ctx.lr = 0x82483BF0;
	sub_82464B68(ctx, base);
	// 82483BF0: C01E0074  lfs f0, 0x74(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483BF4: EDA10028  fsubs f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82483BF8: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82483BFC: C00B6FB8  lfs f0, 0x6fb8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28600 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483C00: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82483C04: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82483C08: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82483C0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82483C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82483C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82483C18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82483C1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82483C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483C28 size=60
    let mut pc: u32 = 0x82483C28;
    'dispatch: loop {
        match pc {
            0x82483C28 => {
    //   block [0x82483C28..0x82483C64)
	// 82483C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483C34: 48CD09CD  bl 0x83154600
	ctx.lr = 0x82483C38;
	sub_83154600(ctx, base);
	// 82483C38: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82483C3C: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82483C40: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82483C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82483C48: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483C68 size=148
    let mut pc: u32 = 0x82483C68;
    'dispatch: loop {
        match pc {
            0x82483C68 => {
    //   block [0x82483C68..0x82483CFC)
	// 82483C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483C6C: 48D24501  bl 0x831a816c
	ctx.lr = 0x82483C70;
	sub_831A8130(ctx, base);
	// 82483C70: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82483C74: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483C78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82483C7C: 48CD0985  bl 0x83154600
	ctx.lr = 0x82483C80;
	sub_83154600(ctx, base);
	// 82483C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483C84: 48CD097D  bl 0x83154600
	ctx.lr = 0x82483C88;
	sub_83154600(ctx, base);
	// 82483C88: 4BFE0EE1  bl 0x82464b68
	ctx.lr = 0x82483C8C;
	sub_82464B68(ctx, base);
	// 82483C8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82483C90: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82483C94: 3BDF0078  addi r30, r31, 0x78
	ctx.r[30].s64 = ctx.r[31].s64 + 120;
	// 82483C98: 489D5AE1  bl 0x82e59778
	ctx.lr = 0x82483C9C;
	sub_82E59778(ctx, base);
	// 82483C9C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82483CA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82483CA4: FCC00890  fmr f6, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[6].f64 = ctx.f[1].f64;
	// 82483CA8: 392B6FAC  addi r9, r11, 0x6fac
	ctx.r[9].s64 = ctx.r[11].s64 + 28588;
	// 82483CAC: C03F0074  lfs f1, 0x74(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82483CB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82483CB4: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82483CB8: C18B6FAC  lfs f12, 0x6fac(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28588 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82483CBC: C00ACEE4  lfs f0, -0x311c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483CC0: EC6C0032  fmuls f3, f12, f0
	ctx.f[3].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82483CC4: C1A90008  lfs f13, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82483CC8: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82483CCC: ECAD0032  fmuls f5, f13, f0
	ctx.f[5].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82483CD0: EC8C0032  fmuls f4, f12, f0
	ctx.f[4].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82483CD4: 4801418D  bl 0x82497e60
	ctx.lr = 0x82483CD8;
	sub_82497E60(ctx, base);
	// 82483CD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82483CDC: 489D5A9D  bl 0x82e59778
	ctx.lr = 0x82483CE0;
	sub_82E59778(ctx, base);
	// 82483CE0: C01F0074  lfs f0, 0x74(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483CE4: C1BF0078  lfs f13, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82483CE8: EC01037A  fmadds f0, f1, f13, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82483CEC: D01F0074  stfs f0, 0x74(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82483CF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82483CF4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82483CF8: 48D244C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483D00 size=428
    let mut pc: u32 = 0x82483D00;
    'dispatch: loop {
        match pc {
            0x82483D00 => {
    //   block [0x82483D00..0x82483EAC)
	// 82483D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483D08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82483D0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82483D10: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 82483D14: 48D24D5D  bl 0x831a8a70
	ctx.lr = 0x82483D18;
	sub_831A8A40(ctx, base);
	// 82483D18: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483D20: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 82483D24: 489D5A55  bl 0x82e59778
	ctx.lr = 0x82483D28;
	sub_82E59778(ctx, base);
	// 82483D28: C01F0070  lfs f0, 0x70(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483D2C: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82483D30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82483D34: D01F0070  stfs f0, 0x70(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82483D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483D3C: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82483D40: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82483D44: 40980128  bge cr6, 0x82483e6c
	if !ctx.cr[6].lt {
	pc = 0x82483E6C; continue 'dispatch;
	}
	// 82483D48: 48CD08B9  bl 0x83154600
	ctx.lr = 0x82483D4C;
	sub_83154600(ctx, base);
	// 82483D4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82483D50: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82483D54: 4808B5DD  bl 0x8250f330
	ctx.lr = 0x82483D58;
	sub_8250F330(ctx, base);
	// 82483D58: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82483D5C: 48065AF5  bl 0x824e9850
	ctx.lr = 0x82483D60;
	sub_824E9850(ctx, base);
	// 82483D60: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82483D64: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82483D68: 4896DF29  bl 0x82df1c90
	ctx.lr = 0x82483D6C;
	sub_82DF1C90(ctx, base);
	// 82483D6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82483D70: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82483D74: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 82483D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483D7C: C3CB9450  lfs f30, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82483D80: EDBDF028  fsubs f13, f29, f30
	ctx.f[13].f64 = (((ctx.f[29].f64 - ctx.f[30].f64) as f32) as f64);
	// 82483D84: C3AA08A8  lfs f29, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82483D88: C0096664  lfs f0, 0x6664(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483D8C: EDADE82A  fadds f13, f13, f29
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[29].f64) as f32) as f64;
	// 82483D90: EF6D0032  fmuls f27, f13, f0
	ctx.f[27].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82483D94: 48CD086D  bl 0x83154600
	ctx.lr = 0x82483D98;
	sub_83154600(ctx, base);
	// 82483D98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82483D9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82483DA0: 4808B591  bl 0x8250f330
	ctx.lr = 0x82483DA4;
	sub_8250F330(ctx, base);
	// 82483DA4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82483DA8: 48065AA9  bl 0x824e9850
	ctx.lr = 0x82483DAC;
	sub_824E9850(ctx, base);
	// 82483DAC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82483DB0: FF400890  fmr f26, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[26].f64 = ctx.f[1].f64;
	// 82483DB4: 4896DEDD  bl 0x82df1c90
	ctx.lr = 0x82483DB8;
	sub_82DF1C90(ctx, base);
	// 82483DB8: EC1B0732  fmuls f0, f27, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[27].f64 * ctx.f[28].f64) as f32) as f64);
	// 82483DBC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82483DC0: EDBAF028  fsubs f13, f26, f30
	ctx.f[13].f64 = (((ctx.f[26].f64 - ctx.f[30].f64) as f32) as f64);
	// 82483DC4: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82483DC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82483DCC: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82483DD0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82483DD4: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82483DD8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82483DDC: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82483DE0: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82483DE4: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82483DE8: D3A10068  stfs f29, 0x68(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82483DEC: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82483DF0: C00B2984  lfs f0, 0x2984(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10628 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483DF4: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82483DF8: 489F8E49  bl 0x82e7cc40
	ctx.lr = 0x82483DFC;
	sub_82E7CC40(ctx, base);
	// 82483DFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82483E00: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82483E04: 489F8095  bl 0x82e7be98
	ctx.lr = 0x82483E08;
	sub_82E7BE98(ctx, base);
	// 82483E08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82483E0C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82483E10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82483E14: 489F7F35  bl 0x82e7bd48
	ctx.lr = 0x82483E18;
	sub_82E7BD48(ctx, base);
	// 82483E18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82483E1C: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82483E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483E24: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483EB0 size=104
    let mut pc: u32 = 0x82483EB0;
    'dispatch: loop {
        match pc {
            0x82483EB0 => {
    //   block [0x82483EB0..0x82483F18)
	// 82483EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82483EBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483EC4: 489D8A65  bl 0x82e5c928
	ctx.lr = 0x82483EC8;
	sub_82E5C928(ctx, base);
	// 82483EC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82483ECC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82483ED0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82483ED4: 394AB524  addi r10, r10, -0x4adc
	ctx.r[10].s64 = ctx.r[10].s64 + -19164;
	// 82483ED8: 993F0080  stb r9, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[9].u8 ) };
	// 82483EDC: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 82483EE0: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483EE4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82483EE8: D01F007C  stfs f0, 0x7c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82483EEC: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82483EF0: 489D99E1  bl 0x82e5d8d0
	ctx.lr = 0x82483EF4;
	sub_82E5D8D0(ctx, base);
	// 82483EF4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82483EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483EFC: 396BB424  addi r11, r11, -0x4bdc
	ctx.r[11].s64 = ctx.r[11].s64 + -19420;
	// 82483F00: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82483F04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82483F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82483F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82483F10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82483F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82483F18 size=88
    let mut pc: u32 = 0x82483F18;
    'dispatch: loop {
        match pc {
            0x82483F18 => {
    //   block [0x82483F18..0x82483F70)
	// 82483F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483F20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82483F24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82483F28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483F2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483F30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82483F34: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 82483F38: 489D9911  bl 0x82e5d848
	ctx.lr = 0x82483F3C;
	sub_82E5D848(ctx, base);
	// 82483F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483F40: 489D8649  bl 0x82e5c588
	ctx.lr = 0x82483F44;
	sub_82E5C588(ctx, base);
	// 82483F44: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82483F48: 4182000C  beq 0x82483f54
	if ctx.cr[0].eq {
	pc = 0x82483F54; continue 'dispatch;
	}
	// 82483F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483F50: 4896E489  bl 0x82df23d8
	ctx.lr = 0x82483F54;
	sub_82DF23D8(ctx, base);
	// 82483F54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483F58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82483F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82483F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82483F64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82483F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82483F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483F70 size=128
    let mut pc: u32 = 0x82483F70;
    'dispatch: loop {
        match pc {
            0x82483F70 => {
    //   block [0x82483F70..0x82483FF0)
	// 82483F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82483F78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82483F7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82483F80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82483F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82483F88: 48CD0679  bl 0x83154600
	ctx.lr = 0x82483F8C;
	sub_83154600(ctx, base);
	// 82483F8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82483F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483F94: 4BFFFB6D  bl 0x82483b00
	ctx.lr = 0x82483F98;
	sub_82483B00(ctx, base);
	// 82483F98: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82483F9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82483FA0: 41820018  beq 0x82483fb8
	if ctx.cr[0].eq {
	pc = 0x82483FB8; continue 'dispatch;
	}
	// 82483FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82483FA8: 489D57D1  bl 0x82e59778
	ctx.lr = 0x82483FAC;
	sub_82E59778(ctx, base);
	// 82483FAC: C01F0084  lfs f0, 0x84(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82483FB0: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82483FB4: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82483FB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82483FBC: 38DF0060  addi r6, r31, 0x60
	ctx.r[6].s64 = ctx.r[31].s64 + 96;
	// 82483FC0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82483FC4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82483FC8: 38AA6910  addi r5, r10, 0x6910
	ctx.r[5].s64 = ctx.r[10].s64 + 26896;
	// 82483FCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82483FD0: C02B08A8  lfs f1, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82483FD4: 4BFE2185  bl 0x82466158
	ctx.lr = 0x82483FD8;
	sub_82466158(ctx, base);
	// 82483FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82483FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82483FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82483FE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82483FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82483FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82483FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82483FF0 size=168
    let mut pc: u32 = 0x82483FF0;
    'dispatch: loop {
        match pc {
            0x82483FF0 => {
    //   block [0x82483FF0..0x82484098)
	// 82483FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82483FF4: 48D24171  bl 0x831a8164
	ctx.lr = 0x82483FF8;
	sub_831A8130(ctx, base);
	// 82483FF8: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82483FFC: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82484000: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484004: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82484008: 48CD05F9  bl 0x83154600
	ctx.lr = 0x8248400C;
	sub_83154600(ctx, base);
	// 8248400C: 3F808327  lis r28, -0x7cd9
	ctx.r[28].s64 = -2094596096;
	// 82484010: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82484014: 3B7C6FAC  addi r27, r28, 0x6fac
	ctx.r[27].s64 = ctx.r[28].s64 + 28588;
	// 82484018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248401C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82484020: 3BDF0078  addi r30, r31, 0x78
	ctx.r[30].s64 = ctx.r[31].s64 + 120;
	// 82484024: C3EBCEE4  lfs f31, -0x311c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82484028: C01BFFFC  lfs f0, -4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248402C: EFC007F2  fmuls f30, f0, f31
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82484030: 489D5749  bl 0x82e59778
	ctx.lr = 0x82484034;
	sub_82E59778(ctx, base);
	// 82484034: C01B0008  lfs f0, 8(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248403C: C1BB0004  lfs f13, 4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82484040: FCC00890  fmr f6, f1
	ctx.f[6].f64 = ctx.f[1].f64;
	// 82484044: C19C6FAC  lfs f12, 0x6fac(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28588 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82484048: ECA007F2  fmuls f5, f0, f31
	ctx.f[5].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8248404C: C03F0074  lfs f1, 0x74(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82484050: EC8D07F2  fmuls f4, f13, f31
	ctx.f[4].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 82484054: EC6C07F2  fmuls f3, f12, f31
	ctx.f[3].f64 = (((ctx.f[12].f64 * ctx.f[31].f64) as f32) as f64);
	// 82484058: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8248405C: 48013E05  bl 0x82497e60
	ctx.lr = 0x82484060;
	sub_82497E60(ctx, base);
	// 82484060: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82484064: 489D5715  bl 0x82e59778
	ctx.lr = 0x82484068;
	sub_82E59778(ctx, base);
	// 82484068: C01F0074  lfs f0, 0x74(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248406C: C1BF0078  lfs f13, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82484070: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484074: EC01037A  fmadds f0, f1, f13, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82484078: D01F0074  stfs f0, 0x74(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8248407C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484080: C02B08A8  lfs f1, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82484084: 4BFFFC7D  bl 0x82483d00
	ctx.lr = 0x82484088;
	sub_82483D00(ctx, base);
	// 82484088: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8248408C: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82484090: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82484094: 48D24120  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82484098 size=196
    let mut pc: u32 = 0x82484098;
    'dispatch: loop {
        match pc {
            0x82484098 => {
    //   block [0x82484098..0x8248415C)
	// 82484098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248409C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824840A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824840A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824840A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824840AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824840B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824840B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 824840B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824840BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824840C0: 4BE3C879  bl 0x822c0938
	ctx.lr = 0x824840C4;
	sub_822C0938(ctx, base);
	// 824840C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824840C8: 41820028  beq 0x824840f0
	if ctx.cr[0].eq {
	pc = 0x824840F0; continue 'dispatch;
	}
	// 824840CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824840D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 824840D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824840D8: 392BB4D4  addi r9, r11, -0x4b2c
	ctx.r[9].s64 = ctx.r[11].s64 + -19244;
	// 824840DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824840E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824840E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824840E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824840EC: 48000008  b 0x824840f4
	pc = 0x824840F4; continue 'dispatch;
	// 824840F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824840F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824840F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824840FC: 409A0044  bne cr6, 0x82484140
	if !ctx.cr[6].eq {
	pc = 0x82484140; continue 'dispatch;
	}
	// 82484100: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484104: 419A001C  beq cr6, 0x82484120
	if ctx.cr[6].eq {
	pc = 0x82484120; continue 'dispatch;
	}
	// 82484108: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248410C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82484110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484114: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82484118: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248411C: 4E800421  bctrl
	ctx.lr = 0x82484120;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82484120: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82484124: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82484128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248412C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82484130: 816B6FC0  lwz r11, 0x6fc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28608 as u32) ) } as u64;
	// 82484134: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82484138: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8248413C: 4BE3BEC5  bl 0x822c0000
	ctx.lr = 0x82484140;
	sub_822C0000(ctx, base);
	// 82484140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82484144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82484148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248414C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82484150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82484154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82484158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82484160 size=196
    let mut pc: u32 = 0x82484160;
    'dispatch: loop {
        match pc {
            0x82484160 => {
    //   block [0x82484160..0x82484224)
	// 82484160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82484164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82484168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248416C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82484170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484174: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82484178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248417C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82484180: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82484184: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484188: 4BE3C7B1  bl 0x822c0938
	ctx.lr = 0x8248418C;
	sub_822C0938(ctx, base);
	// 8248418C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82484190: 41820028  beq 0x824841b8
	if ctx.cr[0].eq {
	pc = 0x824841B8; continue 'dispatch;
	}
	// 82484194: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484198: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8248419C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824841A0: 392BB4E8  addi r9, r11, -0x4b18
	ctx.r[9].s64 = ctx.r[11].s64 + -19224;
	// 824841A4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824841A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824841AC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824841B0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824841B4: 48000008  b 0x824841bc
	pc = 0x824841BC; continue 'dispatch;
	// 824841B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824841BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824841C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824841C4: 409A0044  bne cr6, 0x82484208
	if !ctx.cr[6].eq {
	pc = 0x82484208; continue 'dispatch;
	}
	// 824841C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824841CC: 419A001C  beq cr6, 0x824841e8
	if ctx.cr[6].eq {
	pc = 0x824841E8; continue 'dispatch;
	}
	// 824841D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824841D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824841D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824841DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824841E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824841E4: 4E800421  bctrl
	ctx.lr = 0x824841E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824841E8: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 824841EC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824841F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824841F4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 824841F8: 816B6FC0  lwz r11, 0x6fc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28608 as u32) ) } as u64;
	// 824841FC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82484200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82484204: 4BE3BDFD  bl 0x822c0000
	ctx.lr = 0x82484208;
	sub_822C0000(ctx, base);
	// 82484208: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248420C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82484210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82484214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82484218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248421C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82484220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82484228 size=196
    let mut pc: u32 = 0x82484228;
    'dispatch: loop {
        match pc {
            0x82484228 => {
    //   block [0x82484228..0x824842EC)
	// 82484228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248422C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82484230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82484234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82484238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248423C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82484240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82484244: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82484248: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248424C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484250: 4BE3C6E9  bl 0x822c0938
	ctx.lr = 0x82484254;
	sub_822C0938(ctx, base);
	// 82484254: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82484258: 41820028  beq 0x82484280
	if ctx.cr[0].eq {
	pc = 0x82484280; continue 'dispatch;
	}
	// 8248425C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484260: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82484264: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82484268: 392BB4FC  addi r9, r11, -0x4b04
	ctx.r[9].s64 = ctx.r[11].s64 + -19204;
	// 8248426C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82484270: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82484274: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82484278: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248427C: 48000008  b 0x82484284
	pc = 0x82484284; continue 'dispatch;
	// 82484280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82484284: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484288: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248428C: 409A0044  bne cr6, 0x824842d0
	if !ctx.cr[6].eq {
	pc = 0x824842D0; continue 'dispatch;
	}
	// 82484290: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484294: 419A001C  beq cr6, 0x824842b0
	if ctx.cr[6].eq {
	pc = 0x824842B0; continue 'dispatch;
	}
	// 82484298: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248429C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824842A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824842A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824842A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824842AC: 4E800421  bctrl
	ctx.lr = 0x824842B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824842B0: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 824842B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824842B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824842BC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 824842C0: 816B6FC0  lwz r11, 0x6fc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28608 as u32) ) } as u64;
	// 824842C4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824842C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824842CC: 4BE3BD35  bl 0x822c0000
	ctx.lr = 0x824842D0;
	sub_822C0000(ctx, base);
	// 824842D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824842D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824842D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824842DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824842E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824842E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824842E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824842F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824842F0 size=196
    let mut pc: u32 = 0x824842F0;
    'dispatch: loop {
        match pc {
            0x824842F0 => {
    //   block [0x824842F0..0x824843B4)
	// 824842F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824842F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824842F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824842FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82484300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484304: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82484308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248430C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82484310: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82484314: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484318: 4BE3C621  bl 0x822c0938
	ctx.lr = 0x8248431C;
	sub_822C0938(ctx, base);
	// 8248431C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82484320: 41820028  beq 0x82484348
	if ctx.cr[0].eq {
	pc = 0x82484348; continue 'dispatch;
	}
	// 82484324: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484328: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8248432C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82484330: 392BB510  addi r9, r11, -0x4af0
	ctx.r[9].s64 = ctx.r[11].s64 + -19184;
	// 82484334: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82484338: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8248433C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82484340: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82484344: 48000008  b 0x8248434c
	pc = 0x8248434C; continue 'dispatch;
	// 82484348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248434C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484350: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82484354: 409A0044  bne cr6, 0x82484398
	if !ctx.cr[6].eq {
	pc = 0x82484398; continue 'dispatch;
	}
	// 82484358: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248435C: 419A001C  beq cr6, 0x82484378
	if ctx.cr[6].eq {
	pc = 0x82484378; continue 'dispatch;
	}
	// 82484360: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82484364: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82484368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248436C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82484370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82484374: 4E800421  bctrl
	ctx.lr = 0x82484378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82484378: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8248437C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82484380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82484384: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82484388: 816B6FC0  lwz r11, 0x6fc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28608 as u32) ) } as u64;
	// 8248438C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82484390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82484394: 4BE3BC6D  bl 0x822c0000
	ctx.lr = 0x82484398;
	sub_822C0000(ctx, base);
	// 82484398: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248439C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824843A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824843A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824843A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824843AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824843B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824843B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824843B8 size=608
    let mut pc: u32 = 0x824843B8;
    'dispatch: loop {
        match pc {
            0x824843B8 => {
    //   block [0x824843B8..0x82484618)
	// 824843B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824843BC: 48D23DB1  bl 0x831a816c
	ctx.lr = 0x824843C0;
	sub_831A8130(ctx, base);
	// 824843C0: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 824843C4: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 824843C8: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 824843CC: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824843D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824843D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824843D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824843DC: 388BB618  addi r4, r11, -0x49e8
	ctx.r[4].s64 = ctx.r[11].s64 + -18920;
	// 824843E0: 4896F629  bl 0x82df3a08
	ctx.lr = 0x824843E4;
	sub_82DF3A08(ctx, base);
	// 824843E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824843E8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 824843EC: 388BD964  addi r4, r11, -0x269c
	ctx.r[4].s64 = ctx.r[11].s64 + -9884;
	// 824843F0: 4896F619  bl 0x82df3a08
	ctx.lr = 0x824843F4;
	sub_82DF3A08(ctx, base);
	// 824843F4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 824843F8: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 824843FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82484400: 48146101  bl 0x825ca500
	ctx.lr = 0x82484404;
	sub_825CA500(ctx, base);
	// 82484404: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82484408: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8248440C: 4896F01D  bl 0x82df3428
	ctx.lr = 0x82484410;
	sub_82DF3428(ctx, base);
	// 82484410: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82484414: 4896F015  bl 0x82df3428
	ctx.lr = 0x82484418;
	sub_82DF3428(ctx, base);
	// 82484418: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248441C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82484420: 388BB600  addi r4, r11, -0x4a00
	ctx.r[4].s64 = ctx.r[11].s64 + -18944;
	// 82484424: 4896F5E5  bl 0x82df3a08
	ctx.lr = 0x82484428;
	sub_82DF3A08(ctx, base);
	// 82484428: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248442C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484430: 388BB5E8  addi r4, r11, -0x4a18
	ctx.r[4].s64 = ctx.r[11].s64 + -18968;
	// 82484434: 4896F5D5  bl 0x82df3a08
	ctx.lr = 0x82484438;
	sub_82DF3A08(ctx, base);
	// 82484438: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8248443C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82484440: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82484444: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82484448: 3BEB6FBC  addi r31, r11, 0x6fbc
	ctx.r[31].s64 = ctx.r[11].s64 + 28604;
	// 8248444C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82484450: C3CA08A8  lfs f30, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82484454: 38BFFFDC  addi r5, r31, -0x24
	ctx.r[5].s64 = ctx.r[31].s64 + -36;
	// 82484458: C3A9DD6C  lfs f29, -0x2294(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8248445C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82484460: C3E808A4  lfs f31, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82484464: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 82484468: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8248446C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82484470: 48129649  bl 0x825adab8
	ctx.lr = 0x82484474;
	sub_825ADAB8(ctx, base);
	// 82484474: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82484478: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248447C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82484480: 4811D1D1  bl 0x825a1650
	ctx.lr = 0x82484484;
	sub_825A1650(ctx, base);
	// 82484484: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82484488: 4896EFA1  bl 0x82df3428
	ctx.lr = 0x8248448C;
	sub_82DF3428(ctx, base);
	// 8248448C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82484490: 4BE44829  bl 0x822c8cb8
	ctx.lr = 0x82484494;
	sub_822C8CB8(ctx, base);
	// 82484494: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484498: 4896EF91  bl 0x82df3428
	ctx.lr = 0x8248449C;
	sub_82DF3428(ctx, base);
	// 8248449C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824844A0: 4896EF89  bl 0x82df3428
	ctx.lr = 0x824844A4;
	sub_82DF3428(ctx, base);
	// 824844A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824844A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 824844AC: 388BB5D4  addi r4, r11, -0x4a2c
	ctx.r[4].s64 = ctx.r[11].s64 + -18988;
	// 824844B0: 4896F559  bl 0x82df3a08
	ctx.lr = 0x824844B4;
	sub_82DF3A08(ctx, base);
	// 824844B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824844B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824844BC: 388BB5BC  addi r4, r11, -0x4a44
	ctx.r[4].s64 = ctx.r[11].s64 + -19012;
	// 824844C0: 4896F549  bl 0x82df3a08
	ctx.lr = 0x824844C4;
	sub_82DF3A08(ctx, base);
	// 824844C4: 38BFFFEC  addi r5, r31, -0x14
	ctx.r[5].s64 = ctx.r[31].s64 + -20;
	// 824844C8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824844CC: FC60F090  fmr f3, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[30].f64;
	// 824844D0: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 824844D4: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 824844D8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824844DC: 481295DD  bl 0x825adab8
	ctx.lr = 0x824844E0;
	sub_825ADAB8(ctx, base);
	// 824844E0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824844E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824844E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824844EC: 4811D165  bl 0x825a1650
	ctx.lr = 0x824844F0;
	sub_825A1650(ctx, base);
	// 824844F0: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 824844F4: 4896EF35  bl 0x82df3428
	ctx.lr = 0x824844F8;
	sub_82DF3428(ctx, base);
	// 824844F8: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 824844FC: 4BE447BD  bl 0x822c8cb8
	ctx.lr = 0x82484500;
	sub_822C8CB8(ctx, base);
	// 82484500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82484504: 4896EF25  bl 0x82df3428
	ctx.lr = 0x82484508;
	sub_82DF3428(ctx, base);
	// 82484508: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8248450C: 4896EF1D  bl 0x82df3428
	ctx.lr = 0x82484510;
	sub_82DF3428(ctx, base);
	// 82484510: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484514: 388BB5A0  addi r4, r11, -0x4a60
	ctx.r[4].s64 = ctx.r[11].s64 + -19040;
	// 82484518: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8248451C: 4896F4ED  bl 0x82df3a08
	ctx.lr = 0x82484520;
	sub_82DF3A08(ctx, base);
	// 82484520: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484524: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82484528: 388BB588  addi r4, r11, -0x4a78
	ctx.r[4].s64 = ctx.r[11].s64 + -19064;
	// 8248452C: 4896F4DD  bl 0x82df3a08
	ctx.lr = 0x82484530;
	sub_82DF3A08(ctx, base);
	// 82484530: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82484534: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82484538: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8248453C: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 82484540: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82484544: 38A96664  addi r5, r9, 0x6664
	ctx.r[5].s64 = ctx.r[9].s64 + 26212;
	// 82484548: C3CB9F7C  lfs f30, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8248454C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82484550: C3AA89AC  lfs f29, -0x7654(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82484554: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 82484558: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8248455C: 4812955D  bl 0x825adab8
	ctx.lr = 0x82484560;
	sub_825ADAB8(ctx, base);
	// 82484560: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82484564: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82484568: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8248456C: 4811D0E5  bl 0x825a1650
	ctx.lr = 0x82484570;
	sub_825A1650(ctx, base);
	// 82484570: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82484574: 4896EEB5  bl 0x82df3428
	ctx.lr = 0x82484578;
	sub_82DF3428(ctx, base);
	// 82484578: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 8248457C: 4BE4473D  bl 0x822c8cb8
	ctx.lr = 0x82484580;
	sub_822C8CB8(ctx, base);
	// 82484580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82484584: 4896EEA5  bl 0x82df3428
	ctx.lr = 0x82484588;
	sub_82DF3428(ctx, base);
	// 82484588: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8248458C: 4896EE9D  bl 0x82df3428
	ctx.lr = 0x82484590;
	sub_82DF3428(ctx, base);
	// 82484590: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484594: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484598: 388BB568  addi r4, r11, -0x4a98
	ctx.r[4].s64 = ctx.r[11].s64 + -19096;
	// 8248459C: 4896F46D  bl 0x82df3a08
	ctx.lr = 0x824845A0;
	sub_82DF3A08(ctx, base);
	// 824845A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824845A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824845A8: 388BB548  addi r4, r11, -0x4ab8
	ctx.r[4].s64 = ctx.r[11].s64 + -19128;
	// 824845AC: 4896F45D  bl 0x82df3a08
	ctx.lr = 0x824845B0;
	sub_82DF3A08(ctx, base);
	// 824845B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824845B4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824845B8: FC60F090  fmr f3, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[30].f64;
	// 824845BC: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 824845C0: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 824845C4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824845C8: 481294F1  bl 0x825adab8
	ctx.lr = 0x824845CC;
	sub_825ADAB8(ctx, base);
	// 824845CC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824845D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824845D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824845D8: 4811D079  bl 0x825a1650
	ctx.lr = 0x824845DC;
	sub_825A1650(ctx, base);
	// 824845DC: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 824845E0: 4896EE49  bl 0x82df3428
	ctx.lr = 0x824845E4;
	sub_82DF3428(ctx, base);
	// 824845E4: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 824845E8: 4BE446D1  bl 0x822c8cb8
	ctx.lr = 0x824845EC;
	sub_822C8CB8(ctx, base);
	// 824845EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824845F0: 4896EE39  bl 0x82df3428
	ctx.lr = 0x824845F4;
	sub_82DF3428(ctx, base);
	// 824845F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 824845F8: 4896EE31  bl 0x82df3428
	ctx.lr = 0x824845FC;
	sub_82DF3428(ctx, base);
	// 824845FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82484600: 481456D1  bl 0x825c9cd0
	ctx.lr = 0x82484604;
	sub_825C9CD0(ctx, base);
	// 82484604: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82484608: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8248460C: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82484610: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82484614: 48D23BA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484618 size=384
    let mut pc: u32 = 0x82484618;
    'dispatch: loop {
        match pc {
            0x82484618 => {
    //   block [0x82484618..0x82484798)
	// 82484618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248461C: 48D23B4D  bl 0x831a8168
	ctx.lr = 0x82484620;
	sub_831A8130(ctx, base);
	// 82484620: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82484624: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484628: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248462C: 48CCFFD5  bl 0x83154600
	ctx.lr = 0x82484630;
	sub_83154600(ctx, base);
	// 82484630: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82484634: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82484638: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 8248463C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82484640: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82484644: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82484648: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484798 size=448
    let mut pc: u32 = 0x82484798;
    'dispatch: loop {
        match pc {
            0x82484798 => {
    //   block [0x82484798..0x82484958)
	// 82484798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248479C: 48D239CD  bl 0x831a8168
	ctx.lr = 0x824847A0;
	sub_831A8130(ctx, base);
	// 824847A0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 824847A4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 824847A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824847AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824847B0: 48CCFE51  bl 0x83154600
	ctx.lr = 0x824847B4;
	sub_83154600(ctx, base);
	// 824847B4: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 824847B8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824847BC: 3BCB6F94  addi r30, r11, 0x6f94
	ctx.r[30].s64 = ctx.r[11].s64 + 28564;
	// 824847C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824847C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824847C8: 3B9F0078  addi r28, r31, 0x78
	ctx.r[28].s64 = ctx.r[31].s64 + 120;
	// 824847CC: C3EACEE4  lfs f31, -0x311c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824847D0: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824847D4: EFC007F2  fmuls f30, f0, f31
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 824847D8: 489D4FA1  bl 0x82e59778
	ctx.lr = 0x824847DC;
	sub_82E59778(ctx, base);
	// 824847DC: C01E000C  lfs f0, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824847E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824847E4: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824847E8: FCC00890  fmr f6, f1
	ctx.f[6].f64 = ctx.f[1].f64;
	// 824847EC: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824847F0: ECA007F2  fmuls f5, f0, f31
	ctx.f[5].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 824847F4: C03F0074  lfs f1, 0x74(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824847F8: EC8D07F2  fmuls f4, f13, f31
	ctx.f[4].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 824847FC: EC6C07F2  fmuls f3, f12, f31
	ctx.f[3].f64 = (((ctx.f[12].f64 * ctx.f[31].f64) as f32) as f64);
	// 82484800: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82484804: 4801365D  bl 0x82497e60
	ctx.lr = 0x82484808;
	sub_82497E60(ctx, base);
	// 82484808: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248480C: 489D4F6D  bl 0x82e59778
	ctx.lr = 0x82484810;
	sub_82E59778(ctx, base);
	// 82484810: C01F0074  lfs f0, 0x74(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484814: C1BF0078  lfs f13, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82484818: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248481C: EC01037A  fmadds f0, f1, f13, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82484820: D01F0074  stfs f0, 0x74(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82484824: 489D4F3D  bl 0x82e59760
	ctx.lr = 0x82484828;
	sub_82E59760(ctx, base);
	// 82484828: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248482C: EC010024  fdivs f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 82484830: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484834: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82484838: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8248483C: 40990008  ble cr6, 0x82484844
	if !ctx.cr[6].gt {
	pc = 0x82484844; continue 'dispatch;
	}
	// 82484840: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82484844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484848: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8248484C: 4BFFF4B5  bl 0x82483d00
	ctx.lr = 0x82484850;
	sub_82483D00(ctx, base);
	// 82484850: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82484854: 489D4F0D  bl 0x82e59760
	ctx.lr = 0x82484858;
	sub_82E59760(ctx, base);
	// 82484858: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248485C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82484860: 409900E8  ble cr6, 0x82484948
	if !ctx.cr[6].gt {
	pc = 0x82484948; continue 'dispatch;
	}
	// 82484864: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484868: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248486C: 388BB628  addi r4, r11, -0x49d8
	ctx.r[4].s64 = ctx.r[11].s64 + -18904;
	// 82484870: 38A000B2  li r5, 0xb2
	ctx.r[5].s64 = 178;
	// 82484874: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82484878: 4896DB71  bl 0x82df23e8
	ctx.lr = 0x8248487C;
	sub_82DF23E8(ctx, base);
	// 8248487C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82484880: 4182001C  beq 0x8248489c
	if ctx.cr[0].eq {
	pc = 0x8248489C; continue 'dispatch;
	}
	// 82484884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484888: 489D80A1  bl 0x82e5c928
	ctx.lr = 0x8248488C;
	sub_82E5C928(ctx, base);
	// 8248488C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484890: 396BB45C  addi r11, r11, -0x4ba4
	ctx.r[11].s64 = ctx.r[11].s64 + -19364;
	// 82484894: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484898: 48000008  b 0x824848a0
	pc = 0x824848A0; continue 'dispatch;
	// 8248489C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824848A0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 824848A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824848A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 824848AC: 4BFFF8B5  bl 0x82484160
	ctx.lr = 0x824848B0;
	sub_82484160(ctx, base);
	// 824848B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824848B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824848B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 824848BC: 4BE3B745  bl 0x822c0000
	ctx.lr = 0x824848C0;
	sub_822C0000(ctx, base);
	// 824848C0: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824848C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824848C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824848CC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 824848D0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824848D4: 419A0024  beq cr6, 0x824848f8
	if ctx.cr[6].eq {
	pc = 0x824848F8; continue 'dispatch;
	}
	// 824848D8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 824848DC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 824848E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824848E4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 824848E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824848EC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 824848F0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 824848F4: 4082FFE8  bne 0x824848dc
	if !ctx.cr[0].eq {
	pc = 0x824848DC; continue 'dispatch;
	}
	// 824848F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824848FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82484900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82484904: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82484908: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8248490C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82484910: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82484914: 489D9FAD  bl 0x82e5e8c0
	ctx.lr = 0x82484918;
	sub_82E5E8C0(ctx, base);
	// 82484918: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8248491C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484920: 419A0008  beq cr6, 0x82484928
	if ctx.cr[6].eq {
	pc = 0x82484928; continue 'dispatch;
	}
	// 82484924: 4BE3BF6D  bl 0x822c0890
	ctx.lr = 0x82484928;
	sub_822C0890(ctx, base);
	// 82484928: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8248492C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484930: 419A0008  beq cr6, 0x82484938
	if ctx.cr[6].eq {
	pc = 0x82484938; continue 'dispatch;
	}
	// 82484934: 4BE3BF5D  bl 0x822c0890
	ctx.lr = 0x82484938;
	sub_822C0890(ctx, base);
	// 82484938: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248493C: 419A000C  beq cr6, 0x82484948
	if ctx.cr[6].eq {
	pc = 0x82484948; continue 'dispatch;
	}
	// 82484940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484944: 4BE3BF4D  bl 0x822c0890
	ctx.lr = 0x82484948;
	sub_822C0890(ctx, base);
	// 82484948: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8248494C: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82484950: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82484954: 48D23864  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484958 size=376
    let mut pc: u32 = 0x82484958;
    'dispatch: loop {
        match pc {
            0x82484958 => {
    //   block [0x82484958..0x82484AD0)
	// 82484958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248495C: 48D23811  bl 0x831a816c
	ctx.lr = 0x82484960;
	sub_831A8130(ctx, base);
	// 82484960: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82484964: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484968: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248496C: 48CCFC95  bl 0x83154600
	ctx.lr = 0x82484970;
	sub_83154600(ctx, base);
	// 82484970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82484974: 48CCFC8D  bl 0x83154600
	ctx.lr = 0x82484978;
	sub_83154600(ctx, base);
	// 82484978: 4BFE01F1  bl 0x82464b68
	ctx.lr = 0x8248497C;
	sub_82464B68(ctx, base);
	// 8248497C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82484980: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82484984: 489D4DF5  bl 0x82e59778
	ctx.lr = 0x82484988;
	sub_82E59778(ctx, base);
	// 82484988: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 8248498C: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 82484990: C05E0060  lfs f2, 0x60(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82484994: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82484998: 480133F1  bl 0x82497d88
	ctx.lr = 0x8248499C;
	sub_82497D88(ctx, base);
	// 8248499C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824849A0: 489D4DC1  bl 0x82e59760
	ctx.lr = 0x824849A4;
	sub_82E59760(ctx, base);
	// 824849A4: 3FA08327  lis r29, -0x7cd9
	ctx.r[29].s64 = -2094596096;
	// 824849A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824849AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824849B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824849B4: C01D6FB8  lfs f0, 0x6fb8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28600 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824849B8: EDA10024  fdivs f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 824849BC: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824849C0: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824849C4: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824849C8: FC20F82E  fsel f1, f0, f0, f31
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 824849CC: 4BFFF335  bl 0x82483d00
	ctx.lr = 0x824849D0;
	sub_82483D00(ctx, base);
	// 824849D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824849D4: 489D4D8D  bl 0x82e59760
	ctx.lr = 0x824849D8;
	sub_82E59760(ctx, base);
	// 824849D8: C01D6FB8  lfs f0, 0x6fb8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28600 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824849DC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824849E0: 409900E4  ble cr6, 0x82484ac4
	if !ctx.cr[6].gt {
	pc = 0x82484AC4; continue 'dispatch;
	}
	// 824849E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824849E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824849EC: 388BB628  addi r4, r11, -0x49d8
	ctx.r[4].s64 = ctx.r[11].s64 + -18904;
	// 824849F0: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 824849F4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 824849F8: 4896D9F1  bl 0x82df23e8
	ctx.lr = 0x824849FC;
	sub_82DF23E8(ctx, base);
	// 824849FC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82484A00: 4182001C  beq 0x82484a1c
	if ctx.cr[0].eq {
	pc = 0x82484A1C; continue 'dispatch;
	}
	// 82484A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484A08: 489D7F21  bl 0x82e5c928
	ctx.lr = 0x82484A0C;
	sub_82E5C928(ctx, base);
	// 82484A0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484A10: 396BB434  addi r11, r11, -0x4bcc
	ctx.r[11].s64 = ctx.r[11].s64 + -19404;
	// 82484A14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484A18: 48000008  b 0x82484a20
	pc = 0x82484A20; continue 'dispatch;
	// 82484A1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82484A20: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82484A24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82484A28: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484A2C: 4BFFF66D  bl 0x82484098
	ctx.lr = 0x82484A30;
	sub_82484098(ctx, base);
	// 82484A30: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82484A34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82484A38: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484A3C: 4BE3B5C5  bl 0x822c0000
	ctx.lr = 0x82484A40;
	sub_822C0000(ctx, base);
	// 82484A40: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82484A44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82484A48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484A4C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82484A50: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82484A54: 419A0024  beq cr6, 0x82484a78
	if ctx.cr[6].eq {
	pc = 0x82484A78; continue 'dispatch;
	}
	// 82484A58: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82484A5C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82484A60: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82484A64: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82484A68: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82484A6C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82484A70: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82484A74: 4082FFE8  bne 0x82484a5c
	if !ctx.cr[0].eq {
	pc = 0x82484A5C; continue 'dispatch;
	}
	// 82484A78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82484A7C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82484A80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82484A84: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82484A88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82484A8C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82484A90: 489D9E31  bl 0x82e5e8c0
	ctx.lr = 0x82484A94;
	sub_82E5E8C0(ctx, base);
	// 82484A94: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82484A98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484A9C: 419A0008  beq cr6, 0x82484aa4
	if ctx.cr[6].eq {
	pc = 0x82484AA4; continue 'dispatch;
	}
	// 82484AA0: 4BE3BDF1  bl 0x822c0890
	ctx.lr = 0x82484AA4;
	sub_822C0890(ctx, base);
	// 82484AA4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82484AA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484AAC: 419A0008  beq cr6, 0x82484ab4
	if ctx.cr[6].eq {
	pc = 0x82484AB4; continue 'dispatch;
	}
	// 82484AB0: 4BE3BDE1  bl 0x822c0890
	ctx.lr = 0x82484AB4;
	sub_822C0890(ctx, base);
	// 82484AB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484AB8: 419A000C  beq cr6, 0x82484ac4
	if ctx.cr[6].eq {
	pc = 0x82484AC4; continue 'dispatch;
	}
	// 82484ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484AC0: 4BE3BDD1  bl 0x822c0890
	ctx.lr = 0x82484AC4;
	sub_822C0890(ctx, base);
	// 82484AC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82484AC8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82484ACC: 48D236F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484AD0 size=312
    let mut pc: u32 = 0x82484AD0;
    'dispatch: loop {
        match pc {
            0x82484AD0 => {
    //   block [0x82484AD0..0x82484C08)
	// 82484AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82484AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82484AD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82484ADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82484AE0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82484AE4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484AE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82484AEC: 48CCFB15  bl 0x83154600
	ctx.lr = 0x82484AF0;
	sub_83154600(ctx, base);
	// 82484AF0: 4BFE0079  bl 0x82464b68
	ctx.lr = 0x82484AF4;
	sub_82464B68(ctx, base);
	// 82484AF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484AF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82484AFC: D03E0074  stfs f1, 0x74(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82484B00: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82484B04: 995E0080  stb r10, 0x80(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[10].u8 ) };
	// 82484B08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82484B0C: 3889B628  addi r4, r9, -0x49d8
	ctx.r[4].s64 = ctx.r[9].s64 + -18904;
	// 82484B10: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82484B14: 38A00151  li r5, 0x151
	ctx.r[5].s64 = 337;
	// 82484B18: D3FE0084  stfs f31, 0x84(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82484B1C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82484B20: 4896D8C9  bl 0x82df23e8
	ctx.lr = 0x82484B24;
	sub_82DF23E8(ctx, base);
	// 82484B24: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82484B28: 4182001C  beq 0x82484b44
	if ctx.cr[0].eq {
	pc = 0x82484B44; continue 'dispatch;
	}
	// 82484B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484B30: 489D7DF9  bl 0x82e5c928
	ctx.lr = 0x82484B34;
	sub_82E5C928(ctx, base);
	// 82484B34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484B38: 396BB484  addi r11, r11, -0x4b7c
	ctx.r[11].s64 = ctx.r[11].s64 + -19324;
	// 82484B3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484B40: 48000008  b 0x82484b48
	pc = 0x82484B48; continue 'dispatch;
	// 82484B44: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82484B48: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82484B4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82484B50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484B54: 4BFFF6D5  bl 0x82484228
	ctx.lr = 0x82484B58;
	sub_82484228(ctx, base);
	// 82484B58: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82484B5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82484B60: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484B64: 4BE3B49D  bl 0x822c0000
	ctx.lr = 0x82484B68;
	sub_822C0000(ctx, base);
	// 82484B68: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82484B6C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82484B70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484B74: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82484B78: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82484B7C: 419A0024  beq cr6, 0x82484ba0
	if ctx.cr[6].eq {
	pc = 0x82484BA0; continue 'dispatch;
	}
	// 82484B80: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82484B84: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82484B88: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82484B8C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82484B90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82484B94: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82484B98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82484B9C: 4082FFE8  bne 0x82484b84
	if !ctx.cr[0].eq {
	pc = 0x82484B84; continue 'dispatch;
	}
	// 82484BA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82484BA4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82484BA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82484BAC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82484BB0: 389E0088  addi r4, r30, 0x88
	ctx.r[4].s64 = ctx.r[30].s64 + 136;
	// 82484BB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82484BB8: 489D9AF1  bl 0x82e5e6a8
	ctx.lr = 0x82484BBC;
	sub_82E5E6A8(ctx, base);
	// 82484BBC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82484BC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484BC4: 419A0008  beq cr6, 0x82484bcc
	if ctx.cr[6].eq {
	pc = 0x82484BCC; continue 'dispatch;
	}
	// 82484BC8: 4BE3BCC9  bl 0x822c0890
	ctx.lr = 0x82484BCC;
	sub_822C0890(ctx, base);
	// 82484BCC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82484BD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484BD4: 419A0008  beq cr6, 0x82484bdc
	if ctx.cr[6].eq {
	pc = 0x82484BDC; continue 'dispatch;
	}
	// 82484BD8: 4BE3BCB9  bl 0x822c0890
	ctx.lr = 0x82484BDC;
	sub_822C0890(ctx, base);
	// 82484BDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484BE0: 419A000C  beq cr6, 0x82484bec
	if ctx.cr[6].eq {
	pc = 0x82484BEC; continue 'dispatch;
	}
	// 82484BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484BE8: 4BE3BCA9  bl 0x822c0890
	ctx.lr = 0x82484BEC;
	sub_822C0890(ctx, base);
	// 82484BEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82484BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82484BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82484BF8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82484BFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82484C00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82484C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484C08 size=292
    let mut pc: u32 = 0x82484C08;
    'dispatch: loop {
        match pc {
            0x82484C08 => {
    //   block [0x82484C08..0x82484D2C)
	// 82484C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82484C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82484C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82484C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82484C18: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82484C1C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484C20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484C24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82484C28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82484C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82484C30: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 82484C34: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82484C38: 38A0015E  li r5, 0x15e
	ctx.r[5].s64 = 350;
	// 82484C3C: D3FE0084  stfs f31, 0x84(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82484C40: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82484C44: 4896D7A5  bl 0x82df23e8
	ctx.lr = 0x82484C48;
	sub_82DF23E8(ctx, base);
	// 82484C48: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82484C4C: 4182001C  beq 0x82484c68
	if ctx.cr[0].eq {
	pc = 0x82484C68; continue 'dispatch;
	}
	// 82484C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484C54: 489D7CD5  bl 0x82e5c928
	ctx.lr = 0x82484C58;
	sub_82E5C928(ctx, base);
	// 82484C58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82484C5C: 396BB4AC  addi r11, r11, -0x4b54
	ctx.r[11].s64 = ctx.r[11].s64 + -19284;
	// 82484C60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82484C64: 48000008  b 0x82484c6c
	pc = 0x82484C6C; continue 'dispatch;
	// 82484C68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82484C6C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82484C70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82484C74: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484C78: 4BFFF679  bl 0x824842f0
	ctx.lr = 0x82484C7C;
	sub_824842F0(ctx, base);
	// 82484C7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82484C80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82484C84: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82484C88: 4BE3B379  bl 0x822c0000
	ctx.lr = 0x82484C8C;
	sub_822C0000(ctx, base);
	// 82484C8C: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82484C90: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82484C94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484C98: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82484C9C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82484CA0: 419A0024  beq cr6, 0x82484cc4
	if ctx.cr[6].eq {
	pc = 0x82484CC4; continue 'dispatch;
	}
	// 82484CA4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82484CA8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82484CAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82484CB0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82484CB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82484CB8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82484CBC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82484CC0: 4082FFE8  bne 0x82484ca8
	if !ctx.cr[0].eq {
	pc = 0x82484CA8; continue 'dispatch;
	}
	// 82484CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82484CC8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82484CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82484CD0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82484CD4: 389E0088  addi r4, r30, 0x88
	ctx.r[4].s64 = ctx.r[30].s64 + 136;
	// 82484CD8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82484CDC: 489D99CD  bl 0x82e5e6a8
	ctx.lr = 0x82484CE0;
	sub_82E5E6A8(ctx, base);
	// 82484CE0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82484CE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484CE8: 419A0008  beq cr6, 0x82484cf0
	if ctx.cr[6].eq {
	pc = 0x82484CF0; continue 'dispatch;
	}
	// 82484CEC: 4BE3BBA5  bl 0x822c0890
	ctx.lr = 0x82484CF0;
	sub_822C0890(ctx, base);
	// 82484CF0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82484CF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82484CF8: 419A0008  beq cr6, 0x82484d00
	if ctx.cr[6].eq {
	pc = 0x82484D00; continue 'dispatch;
	}
	// 82484CFC: 4BE3BB95  bl 0x822c0890
	ctx.lr = 0x82484D00;
	sub_822C0890(ctx, base);
	// 82484D00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82484D04: 419A000C  beq cr6, 0x82484d10
	if ctx.cr[6].eq {
	pc = 0x82484D10; continue 'dispatch;
	}
	// 82484D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82484D0C: 4BE3BB85  bl 0x822c0890
	ctx.lr = 0x82484D10;
	sub_822C0890(ctx, base);
	// 82484D10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82484D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82484D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82484D1C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82484D20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82484D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82484D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82484D30 size=92
    let mut pc: u32 = 0x82484D30;
    'dispatch: loop {
        match pc {
            0x82484D30 => {
    //   block [0x82484D30..0x82484D8C)
	// 82484D30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484D34: 81430078  lwz r10, 0x78(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82484D38: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82484D3C: C1AB08A4  lfs f13, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82484D40: 409A0014  bne cr6, 0x82484d54
	if !ctx.cr[6].eq {
	pc = 0x82484D54; continue 'dispatch;
	}
	// 82484D44: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484D48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82484D4C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82484D50: 409A0008  bne cr6, 0x82484d58
	if !ctx.cr[6].eq {
	pc = 0x82484D58; continue 'dispatch;
	}
	// 82484D54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82484D58: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82484D5C: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82484D60: C18A08A8  lfs f12, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82484D64: 4182000C  beq 0x82484d70
	if ctx.cr[0].eq {
	pc = 0x82484D70; continue 'dispatch;
	}
	// 82484D68: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484D6C: EDAC0028  fsubs f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82484D70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82484D74: D1A30074  stfs f13, 0x74(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82484D78: C00BC3C8  lfs f0, -0x3c38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484D7C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82484D80: 4099000C  ble cr6, 0x82484d8c
	if !ctx.cr[6].gt {
		sub_82484D8C(ctx, base);
		return;
	}
	// 82484D84: EC0C0824  fdivs f0, f12, f1
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[1].f64) as f32) as f64;
	// 82484D88: 4800000C  b 0x82484d94
	sub_82484D8C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484D8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82484D8C size=32
    let mut pc: u32 = 0x82484D8C;
    'dispatch: loop {
        match pc {
            0x82484D8C => {
    //   block [0x82484D8C..0x82484DAC)
	// 82484D8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82484D90: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484D94: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 82484D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82484D9C: D0030070  stfs f0, 0x70(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82484DA0: 91630078  stw r11, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82484DA4: EC2D0072  fmuls f1, f13, f1
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 82484DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82484DB0 size=92
    let mut pc: u32 = 0x82484DB0;
    'dispatch: loop {
        match pc {
            0x82484DB0 => {
    //   block [0x82484DB0..0x82484E0C)
	// 82484DB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484DB4: 81430078  lwz r10, 0x78(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82484DB8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82484DBC: C1AB08A4  lfs f13, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82484DC0: 409A0014  bne cr6, 0x82484dd4
	if !ctx.cr[6].eq {
	pc = 0x82484DD4; continue 'dispatch;
	}
	// 82484DC4: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484DC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82484DCC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82484DD0: 409A0008  bne cr6, 0x82484dd8
	if !ctx.cr[6].eq {
	pc = 0x82484DD8; continue 'dispatch;
	}
	// 82484DD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82484DD8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82484DDC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82484DE0: C18A08A8  lfs f12, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82484DE4: 4182000C  beq 0x82484df0
	if ctx.cr[0].eq {
	pc = 0x82484DF0; continue 'dispatch;
	}
	// 82484DE8: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484DEC: EDAC0028  fsubs f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82484DF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82484DF4: D1A30074  stfs f13, 0x74(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82484DF8: C00BC3C8  lfs f0, -0x3c38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484DFC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82484E00: 4099000C  ble cr6, 0x82484e0c
	if !ctx.cr[6].gt {
		sub_82484E0C(ctx, base);
		return;
	}
	// 82484E04: EC0C0824  fdivs f0, f12, f1
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[1].f64) as f32) as f64;
	// 82484E08: 4800000C  b 0x82484e14
	sub_82484E0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484E0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82484E0C size=32
    let mut pc: u32 = 0x82484E0C;
    'dispatch: loop {
        match pc {
            0x82484E0C => {
    //   block [0x82484E0C..0x82484E2C)
	// 82484E0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82484E10: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484E14: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 82484E18: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82484E1C: D0030070  stfs f0, 0x70(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82484E20: 91630078  stw r11, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82484E24: EC2D0072  fmuls f1, f13, f1
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 82484E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484E30 size=112
    let mut pc: u32 = 0x82484E30;
    'dispatch: loop {
        match pc {
            0x82484E30 => {
    //   block [0x82484E30..0x82484EA0)
	// 82484E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82484E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82484E38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82484E3C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82484E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484E44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82484E48: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82484E4C: 897F0061  lbz r11, 0x61(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(97 as u32) ) } as u64;
	// 82484E50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82484E54: 40820034  bne 0x82484e88
	if !ctx.cr[0].eq {
	pc = 0x82484E88; continue 'dispatch;
	}
	// 82484E58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484E5C: C01F0078  lfs f0, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484E60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82484E64: D01F007C  stfs f0, 0x7c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82484E68: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82484E6C: 995F0061  stb r10, 0x61(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(97 as u32), ctx.r[10].u8 ) };
	// 82484E70: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82484E74: D01F0074  stfs f0, 0x74(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82484E78: 4BFFFF39  bl 0x82484db0
	ctx.lr = 0x82484E7C;
	sub_82484DB0(ctx, base);
	// 82484E7C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82484E80: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82484E84: 4BFFFF2D  bl 0x82484db0
	ctx.lr = 0x82484E88;
	sub_82484DB0(ctx, base);
	// 82484E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82484E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82484E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82484E94: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82484E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82484E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484EA0 size=288
    let mut pc: u32 = 0x82484EA0;
    'dispatch: loop {
        match pc {
            0x82484EA0 => {
    //   block [0x82484EA0..0x82484FC0)
	// 82484EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82484EA4: 48D232C9  bl 0x831a816c
	ctx.lr = 0x82484EA8;
	sub_831A8130(ctx, base);
	// 82484EA8: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82484EAC: 48D23BCD  bl 0x831a8a78
	ctx.lr = 0x82484EB0;
	sub_831A8A40(ctx, base);
	// 82484EB0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82484EB8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82484EBC: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 82484EC0: FF801890  fmr f28, f3
	ctx.f[28].f64 = ctx.f[3].f64;
	// 82484EC4: FFA02090  fmr f29, f4
	ctx.f[29].f64 = ctx.f[4].f64;
	// 82484EC8: 48CCF739  bl 0x83154600
	ctx.lr = 0x82484ECC;
	sub_83154600(ctx, base);
	// 82484ECC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82484ED0: D3DF006C  stfs f30, 0x6c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82484ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82484ED8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82484EDC: 995F0061  stb r10, 0x61(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(97 as u32), ctx.r[10].u8 ) };
	// 82484EE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82484EE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82484EE8: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82484EEC: D3DF0070  stfs f30, 0x70(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82484EF0: 4BFDFE99  bl 0x82464d88
	ctx.lr = 0x82484EF4;
	sub_82464D88(ctx, base);
	// 82484EF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82484EF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82484EFC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82484F00: 4BFDFE99  bl 0x82464d98
	ctx.lr = 0x82484F04;
	sub_82464D98(ctx, base);
	// 82484F04: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82484F08: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82484FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82484FC0 size=388
    let mut pc: u32 = 0x82484FC0;
    'dispatch: loop {
        match pc {
            0x82484FC0 => {
    //   block [0x82484FC0..0x82485144)
	// 82484FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82484FC4: 48D231A5  bl 0x831a8168
	ctx.lr = 0x82484FC8;
	sub_831A8130(ctx, base);
	// 82484FC8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82484FCC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82484FD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82484FD4: 48CCF62D  bl 0x83154600
	ctx.lr = 0x82484FD8;
	sub_83154600(ctx, base);
	// 82484FD8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82484FDC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82484FE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82484FE4: 4BFDFDA5  bl 0x82464d88
	ctx.lr = 0x82484FE8;
	sub_82464D88(ctx, base);
	// 82484FE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82484FEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82484FF0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82484FF4: 4BFDFDA5  bl 0x82464d98
	ctx.lr = 0x82484FF8;
	sub_82464D98(ctx, base);
	// 82484FF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82484FFC: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82485000: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82485004: 839F0080  lwz r28, 0x80(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82485008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248500C: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485148 size=244
    let mut pc: u32 = 0x82485148;
    'dispatch: loop {
        match pc {
            0x82485148 => {
    //   block [0x82485148..0x8248523C)
	// 82485148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248514C: 48D2300D  bl 0x831a8158
	ctx.lr = 0x82485150;
	sub_831A8130(ctx, base);
	// 82485150: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485158: 489D77D1  bl 0x82e5c928
	ctx.lr = 0x8248515C;
	sub_82E5C928(ctx, base);
	// 8248515C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82485160: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485164: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82485168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248516C: 3929B674  addi r9, r9, -0x498c
	ctx.r[9].s64 = ctx.r[9].s64 + -18828;
	// 82485170: 997F0060  stb r11, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82485174: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 82485178: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248517C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82485180: D01F0064  stfs f0, 0x64(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82485184: 997F0061  stb r11, 0x61(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82485188: D01F0068  stfs f0, 0x68(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8248518C: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 82485190: D01F006C  stfs f0, 0x6c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82485194: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82485198: D01F0070  stfs f0, 0x70(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8248519C: 3BDF0080  addi r30, r31, 0x80
	ctx.r[30].s64 = ctx.r[31].s64 + 128;
	// 824851A0: D01F0074  stfs f0, 0x74(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 824851A4: 3BBF0084  addi r29, r31, 0x84
	ctx.r[29].s64 = ctx.r[31].s64 + 132;
	// 824851A8: D01F0078  stfs f0, 0x78(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 824851AC: D01F007C  stfs f0, 0x7c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 824851B0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 824851B4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 824851B8: 7C7BE02E  lwzx r3, r27, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824851BC: 48A1B575  bl 0x82ea0730
	ctx.lr = 0x824851C0;
	sub_82EA0730(ctx, base);
	// 824851C0: 3B200090  li r25, 0x90
	ctx.r[25].s64 = 144;
	// 824851C4: B3230004  sth r25, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[25].u16 ) };
	// 824851C8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824851CC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 824851D0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 824851D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824851D8: 9B410050  stb r26, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u8 ) };
	// 824851DC: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824851E0: 48B2FDB9  bl 0x82fb4f98
	ctx.lr = 0x824851E4;
	sub_82FB4F98(ctx, base);
	// 824851E4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824851E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824851EC: 48728B0D  bl 0x82badcf8
	ctx.lr = 0x824851F0;
	sub_82BADCF8(ctx, base);
	// 824851F0: 931F0080  stw r24, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 824851F4: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 824851F8: 7C7BE02E  lwzx r3, r27, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824851FC: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82485200: 48A1B531  bl 0x82ea0730
	ctx.lr = 0x82485204;
	sub_82EA0730(ctx, base);
	// 82485204: B3230004  sth r25, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[25].u16 ) };
	// 82485208: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8248520C: 9B410050  stb r26, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u8 ) };
	// 82485210: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82485214: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82485218: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248521C: 48B2FD7D  bl 0x82fb4f98
	ctx.lr = 0x82485220;
	sub_82FB4F98(ctx, base);
	// 82485220: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82485224: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82485228: 48728AD1  bl 0x82badcf8
	ctx.lr = 0x8248522C;
	sub_82BADCF8(ctx, base);
	// 8248522C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82485230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485234: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82485238: 48D22F70  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82485240 size=96
    let mut pc: u32 = 0x82485240;
    'dispatch: loop {
        match pc {
            0x82485240 => {
    //   block [0x82485240..0x824852A0)
	// 82485240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248524C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82485250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485258: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248525C: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 82485260: 48728A99  bl 0x82badcf8
	ctx.lr = 0x82485264;
	sub_82BADCF8(ctx, base);
	// 82485264: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82485268: 48728A91  bl 0x82badcf8
	ctx.lr = 0x8248526C;
	sub_82BADCF8(ctx, base);
	// 8248526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485270: 489D7319  bl 0x82e5c588
	ctx.lr = 0x82485274;
	sub_82E5C588(ctx, base);
	// 82485274: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82485278: 4182000C  beq 0x82485284
	if ctx.cr[0].eq {
	pc = 0x82485284; continue 'dispatch;
	}
	// 8248527C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485280: 4896D159  bl 0x82df23d8
	ctx.lr = 0x82485284;
	sub_82DF23D8(ctx, base);
	// 82485284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248528C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485294: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82485298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248529C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824852A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824852A0 size=8
    let mut pc: u32 = 0x824852A0;
    'dispatch: loop {
        match pc {
            0x824852A0 => {
    //   block [0x824852A0..0x824852A8)
	// 824852A0: 38630090  addi r3, r3, 0x90
	ctx.r[3].s64 = ctx.r[3].s64 + 144;
	// 824852A4: 489F80CC  b 0x82e7d370
	sub_82E7D370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824852A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824852A8 size=220
    let mut pc: u32 = 0x824852A8;
    'dispatch: loop {
        match pc {
            0x824852A8 => {
    //   block [0x824852A8..0x82485384)
	// 824852A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824852AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824852B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824852B4: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 824852B8: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 824852BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824852C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824852C4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824852C8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 824852CC: C1BF0060  lfs f13, 0x60(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824852D0: C01F0064  lfs f0, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824852D4: EFCD0032  fmuls f30, f13, f0
	ctx.f[30].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824852D8: C1ABDFB0  lfs f13, -0x2050(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824852DC: FD80F210  fabs f12, f30
	ctx.f[12].u64 = ctx.f[30].u64 & !0x8000_0000_0000_0000u64;
	// 824852E0: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 824852E4: 41990010  bgt cr6, 0x824852f4
	if ctx.cr[6].gt {
	pc = 0x824852F4; continue 'dispatch;
	}
	// 824852E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824852EC: C02B08A4  lfs f1, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824852F0: 48000078  b 0x82485368
	pc = 0x82485368; continue 'dispatch;
	// 824852F4: E97F006A  lwa r11, 0x68(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as i32) as i64;
	// 824852F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 824852FC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82485300: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82485304: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82485308: C1AADFAC  lfs f13, -0x2054(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8276 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8248530C: FD606018  frsp f11, f12
	ctx.f[11].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82485310: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82485314: C1899524  lfs f12, -0x6adc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82485318: EC0B0024  fdivs f0, f11, f0
	ctx.f[0].f64 = ((ctx.f[11].f64 / ctx.f[0].f64) as f32) as f64;
	// 8248531C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82485320: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82485324: EC200332  fmuls f1, f0, f12
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82485328: 48D23AA1  bl 0x831a8dc8
	ctx.lr = 0x8248532C;
	sub_831A8DC8(ctx, base);
	// 8248532C: E97F006A  lwa r11, 0x68(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as i32) as i64;
	// 82485330: C01F0080  lfs f0, 0x80(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82485334: EDA0F024  fdivs f13, f0, f30
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[30].f64) as f32) as f64;
	// 82485338: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8248533C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82485340: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82485344: FD400018  frsp f10, f0
	ctx.f[10].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82485348: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248534C: C19F0060  lfs f12, 0x60(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82485350: FD600818  frsp f11, f1
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82485354: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82485358: EDAD02B2  fmuls f13, f13, f10
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 8248535C: EC0D07FC  fnmsubs f0, f13, f31, f0
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82485360: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82485364: EC2002F2  fmuls f1, f0, f11
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 82485368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248536C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485374: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82485378: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248537C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485388 size=128
    let mut pc: u32 = 0x82485388;
    'dispatch: loop {
        match pc {
            0x82485388 => {
    //   block [0x82485388..0x82485408)
	// 82485388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485390: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82485394: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485398: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248539C: 489D758D  bl 0x82e5c928
	ctx.lr = 0x824853A0;
	sub_82E5C928(ctx, base);
	// 824853A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824853A4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824853A8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 824853AC: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 824853B0: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 824853B4: C00BD5B8  lfs f0, -0x2a48(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10824 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824853B8: 3929B69C  addi r9, r9, -0x4964
	ctx.r[9].s64 = ctx.r[9].s64 + -18788;
	// 824853BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824853C0: C1AA9450  lfs f13, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824853C4: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824853C8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824853CC: D1BF0064  stfs f13, 0x64(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 824853D0: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 824853D4: C00808A4  lfs f0, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824853D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824853DC: C1A708A8  lfs f13, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824853E0: D01F0070  stfs f0, 0x70(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 824853E4: D1BF0074  stfs f13, 0x74(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 824853E8: D01F0078  stfs f0, 0x78(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 824853EC: D01F007C  stfs f0, 0x7c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 824853F0: D01F0080  stfs f0, 0x80(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 824853F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824853F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824853FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485408 size=164
    let mut pc: u32 = 0x82485408;
    'dispatch: loop {
        match pc {
            0x82485408 => {
    //   block [0x82485408..0x824854AC)
	// 82485408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248540C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82485414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82485418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248541C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485420: 48CCF1E1  bl 0x83154600
	ctx.lr = 0x82485424;
	sub_83154600(ctx, base);
	// 82485424: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82485428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248542C: 489D4335  bl 0x82e59760
	ctx.lr = 0x82485430;
	sub_82E59760(ctx, base);
	// 82485430: C01F0064  lfs f0, 0x64(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82485434: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82485438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248543C: 4098000C  bge cr6, 0x82485448
	if !ctx.cr[6].lt {
	pc = 0x82485448; continue 'dispatch;
	}
	// 82485440: 489D42F1  bl 0x82e59730
	ctx.lr = 0x82485444;
	sub_82E59730(ctx, base);
	// 82485444: 48000050  b 0x82485494
	pc = 0x82485494; continue 'dispatch;
	// 82485448: 489D4319  bl 0x82e59760
	ctx.lr = 0x8248544C;
	sub_82E59760(ctx, base);
	// 8248544C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485450: 4BFFFE59  bl 0x824852a8
	ctx.lr = 0x82485454;
	sub_824852A8(ctx, base);
	// 82485454: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82485458: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 8248545C: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82485460: 38DF0090  addi r6, r31, 0x90
	ctx.r[6].s64 = ctx.r[31].s64 + 144;
	// 82485464: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82485468: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248546C: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824854B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824854B0 size=156
    let mut pc: u32 = 0x824854B0;
    'dispatch: loop {
        match pc {
            0x824854B0 => {
    //   block [0x824854B0..0x8248554C)
	// 824854B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824854B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824854B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824854BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824854C0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 824854C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824854C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824854CC: 48CCF135  bl 0x83154600
	ctx.lr = 0x824854D0;
	sub_83154600(ctx, base);
	// 824854D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824854D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824854D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824854DC: 48089FED  bl 0x8250f4c8
	ctx.lr = 0x824854E0;
	sub_8250F4C8(ctx, base);
	// 824854E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824854E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824854E8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 824854EC: 409A0008  bne cr6, 0x824854f4
	if !ctx.cr[6].eq {
	pc = 0x824854F4; continue 'dispatch;
	}
	// 824854F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824854F4: 48083025  bl 0x82508518
	ctx.lr = 0x824854F8;
	sub_82508518(ctx, base);
	// 824854F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824854FC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82485500: 4896C791  bl 0x82df1c90
	ctx.lr = 0x82485504;
	sub_82DF1C90(ctx, base);
	// 82485504: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82485508: C01F00F0  lfs f0, 0xf0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248550C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82485510: D3FF00F0  stfs f31, 0xf0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 82485514: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82485518: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8248551C: 807F00D0  lwz r3, 0xd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82485520: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485524: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82485528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248552C: 4E800421  bctrl
	ctx.lr = 0x82485530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82485530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82485534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248553C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82485540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82485544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82485550 size=20
    let mut pc: u32 = 0x82485550;
    'dispatch: loop {
        match pc {
            0x82485550 => {
    //   block [0x82485550..0x82485564)
	// 82485550: 806300D0  lwz r3, 0xd0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 82485554: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485558: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248555C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82485560: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485568 size=456
    let mut pc: u32 = 0x82485568;
    'dispatch: loop {
        match pc {
            0x82485568 => {
    //   block [0x82485568..0x82485730)
	// 82485568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248556C: 48D22BF9  bl 0x831a8164
	ctx.lr = 0x82485570;
	sub_831A8130(ctx, base);
	// 82485570: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485574: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82485578: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248557C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82485580: 48CCF081  bl 0x83154600
	ctx.lr = 0x82485584;
	sub_83154600(ctx, base);
	// 82485584: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82485588: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8248558C: 13C0D8C7  vcmpequd (lvx128) v30, v0, v27
	tmp.u32 = ctx.r[27].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82485590: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485730 size=160
    let mut pc: u32 = 0x82485730;
    'dispatch: loop {
        match pc {
            0x82485730 => {
    //   block [0x82485730..0x824857D0)
	// 82485730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485734: 48D22A39  bl 0x831a816c
	ctx.lr = 0x82485738;
	sub_831A8130(ctx, base);
	// 82485738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248573C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485740: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82485744: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82485748: 4BFE4011  bl 0x82469758
	ctx.lr = 0x8248574C;
	sub_82469758(ctx, base);
	// 8248574C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82485750: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 82485754: 396AB6D4  addi r11, r10, -0x492c
	ctx.r[11].s64 = ctx.r[10].s64 + -18732;
	// 82485758: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248575C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485760: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82485764: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82485768: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248576C: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 82485770: 419A0024  beq cr6, 0x82485794
	if ctx.cr[6].eq {
	pc = 0x82485794; continue 'dispatch;
	}
	// 82485774: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82485778: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248577C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82485780: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82485784: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82485788: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8248578C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82485790: 4082FFE8  bne 0x82485778
	if !ctx.cr[0].eq {
	pc = 0x82485778; continue 'dispatch;
	}
	// 82485794: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82485798: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8248579C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824857A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824857A4: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 824857A8: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 824857AC: C1AA9F60  lfs f13, -0x60a0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24736 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824857B0: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824857B4: 9BDF00E4  stb r30, 0xe4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u8 ) };
	// 824857B8: D1BF00E0  stfs f13, 0xe0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 824857BC: D01F00E8  stfs f0, 0xe8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 824857C0: D01F00EC  stfs f0, 0xec(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 824857C4: D01F00F0  stfs f0, 0xf0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 824857C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824857CC: 48D229F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824857D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824857D0 size=104
    let mut pc: u32 = 0x824857D0;
    'dispatch: loop {
        match pc {
            0x824857D0 => {
    //   block [0x824857D0..0x82485838)
	// 824857D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824857D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824857D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824857DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824857E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824857E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824857E8: 396BB6D4  addi r11, r11, -0x492c
	ctx.r[11].s64 = ctx.r[11].s64 + -18732;
	// 824857EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824857F0: 807F00DC  lwz r3, 0xdc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 824857F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824857F8: 419A0008  beq cr6, 0x82485800
	if ctx.cr[6].eq {
	pc = 0x82485800; continue 'dispatch;
	}
	// 824857FC: 4BE3B095  bl 0x822c0890
	ctx.lr = 0x82485800;
	sub_822C0890(ctx, base);
	// 82485800: 807F00D4  lwz r3, 0xd4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82485804: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82485808: 419A0008  beq cr6, 0x82485810
	if ctx.cr[6].eq {
	pc = 0x82485810; continue 'dispatch;
	}
	// 8248580C: 4BE3B085  bl 0x822c0890
	ctx.lr = 0x82485810;
	sub_822C0890(ctx, base);
	// 82485810: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82485814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485818: 396B770C  addi r11, r11, 0x770c
	ctx.r[11].s64 = ctx.r[11].s64 + 30476;
	// 8248581C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82485820: 489D6D69  bl 0x82e5c588
	ctx.lr = 0x82485824;
	sub_82E5C588(ctx, base);
	// 82485824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82485828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248582C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82485838 size=76
    let mut pc: u32 = 0x82485838;
    'dispatch: loop {
        match pc {
            0x82485838 => {
    //   block [0x82485838..0x82485884)
	// 82485838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248583C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82485844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82485848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248584C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485850: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82485854: 4BFFFF7D  bl 0x824857d0
	ctx.lr = 0x82485858;
	sub_824857D0(ctx, base);
	// 82485858: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248585C: 4182000C  beq 0x82485868
	if ctx.cr[0].eq {
	pc = 0x82485868; continue 'dispatch;
	}
	// 82485860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485864: 4896CB75  bl 0x82df23d8
	ctx.lr = 0x82485868;
	sub_82DF23D8(ctx, base);
	// 82485868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248586C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82485870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248587C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485888 size=396
    let mut pc: u32 = 0x82485888;
    'dispatch: loop {
        match pc {
            0x82485888 => {
    //   block [0x82485888..0x82485A14)
	// 82485888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248588C: 48D228C9  bl 0x831a8154
	ctx.lr = 0x82485890;
	sub_831A8130(ctx, base);
	// 82485890: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485894: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82485898: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8248589C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 824858A0: 48CCED61  bl 0x83154600
	ctx.lr = 0x824858A4;
	sub_83154600(ctx, base);
	// 824858A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824858A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824858AC: 388BB708  addi r4, r11, -0x48f8
	ctx.r[4].s64 = ctx.r[11].s64 + -18680;
	// 824858B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824858B4: 38A00032  li r5, 0x32
	ctx.r[5].s64 = 50;
	// 824858B8: 38600200  li r3, 0x200
	ctx.r[3].s64 = 512;
	// 824858BC: 4896CB2D  bl 0x82df23e8
	ctx.lr = 0x824858C0;
	sub_82DF23E8(ctx, base);
	// 824858C0: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824858C4: 4182009C  beq 0x82485960
	if ctx.cr[0].eq {
	pc = 0x82485960; continue 'dispatch;
	}
	// 824858C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824858CC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 824858D0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 824858D4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 824858D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824858DC: 48A1AE55  bl 0x82ea0730
	ctx.lr = 0x824858E0;
	sub_82EA0730(ctx, base);
	// 824858E0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 824858E4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824858E8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824858EC: C02A0A90  lfs f1, 0xa90(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2704 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824858F0: 48A94901  bl 0x82f1a1f0
	ctx.lr = 0x824858F4;
	sub_82F1A1F0(ctx, base);
	// 824858F4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824858F8: 3B9D0028  addi r28, r29, 0x28
	ctx.r[28].s64 = ctx.r[29].s64 + 40;
	// 824858FC: 409A0008  bne cr6, 0x82485904
	if !ctx.cr[6].eq {
	pc = 0x82485904; continue 'dispatch;
	}
	// 82485900: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82485904: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82485908: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248590C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82485910: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82485914: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82485918: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8248591C: 83CB66EC  lwz r30, 0x66ec(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26348 as u32) ) } as u64;
	// 82485920: 3B2A6910  addi r25, r10, 0x6910
	ctx.r[25].s64 = ctx.r[10].s64 + 26896;
	// 82485924: 3B09BA80  addi r24, r9, -0x4580
	ctx.r[24].s64 = ctx.r[9].s64 + -17792;
	// 82485928: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 8248592C: 3AE10050  addi r23, r1, 0x50
	ctx.r[23].s64 = ctx.r[1].s64 + 80;
	// 82485930: 48089BE9  bl 0x8250f518
	ctx.lr = 0x82485934;
	sub_8250F518(ctx, base);
	// 82485934: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82485938: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8248593C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82485940: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82485944: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82485948: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 8248594C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82485950: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82485954: 4866198D  bl 0x82ae72e0
	ctx.lr = 0x82485958;
	sub_82AE72E0(ctx, base);
	// 82485958: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248595C: 48000008  b 0x82485964
	pc = 0x82485964; continue 'dispatch;
	// 82485960: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82485964: 3BDA00D8  addi r30, r26, 0xd8
	ctx.r[30].s64 = ctx.r[26].s64 + 216;
	// 82485968: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248596C: 4BEEE8ED  bl 0x82374258
	ctx.lr = 0x82485970;
	sub_82374258(ctx, base);
	// 82485970: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82485974: 41820010  beq 0x82485984
	if ctx.cr[0].eq {
	pc = 0x82485984; continue 'dispatch;
	}
	// 82485978: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8248597C: 57FF07FA  rlwinm r31, r31, 0, 0x1f, 0x1d
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 82485980: 4896C311  bl 0x82df1c90
	ctx.lr = 0x82485984;
	sub_82DF1C90(ctx, base);
	// 82485984: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82485988: 41820014  beq 0x8248599c
	if ctx.cr[0].eq {
	pc = 0x8248599C; continue 'dispatch;
	}
	// 8248598C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82485990: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82485994: 419A0008  beq cr6, 0x8248599c
	if ctx.cr[6].eq {
	pc = 0x8248599C; continue 'dispatch;
	}
	// 82485998: 4BE628D1  bl 0x822e8268
	ctx.lr = 0x8248599C;
	sub_822E8268(ctx, base);
	// 8248599C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824859A0: C82B2278  lfd f1, 0x2278(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8824 as u32) ) };
	// 824859A4: 48D23505  bl 0x831a8ea8
	ctx.lr = 0x824859A8;
	sub_831A8EA8(ctx, base);
	// 824859A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824859AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824859B0: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 824859B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824859B8: D00B0084  stfs f0, 0x84(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 824859BC: 817A00D0  lwz r11, 0xd0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(208 as u32) ) } as u64;
	// 824859C0: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824859C4: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824859C8: 807A00D0  lwz r3, 0xd0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(208 as u32) ) } as u64;
	// 824859CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824859D0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824859D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824859D8: 4E800421  bctrl
	ctx.lr = 0x824859DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824859DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824859E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824859E4: 48089AE5  bl 0x8250f4c8
	ctx.lr = 0x824859E8;
	sub_8250F4C8(ctx, base);
	// 824859E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824859EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824859F0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 824859F4: 409A0008  bne cr6, 0x824859fc
	if !ctx.cr[6].eq {
	pc = 0x824859FC; continue 'dispatch;
	}
	// 824859F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824859FC: 48082B1D  bl 0x82508518
	ctx.lr = 0x82485A00;
	sub_82508518(ctx, base);
	// 82485A00: D03A00F0  stfs f1, 0xf0(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 82485A04: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82485A08: 4896C289  bl 0x82df1c90
	ctx.lr = 0x82485A0C;
	sub_82DF1C90(ctx, base);
	// 82485A0C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82485A10: 48D22794  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485A18 size=372
    let mut pc: u32 = 0x82485A18;
    'dispatch: loop {
        match pc {
            0x82485A18 => {
    //   block [0x82485A18..0x82485B8C)
	// 82485A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485A1C: 48D2274D  bl 0x831a8168
	ctx.lr = 0x82485A20;
	sub_831A8130(ctx, base);
	// 82485A20: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82485A24: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485A2C: 48CCEBD5  bl 0x83154600
	ctx.lr = 0x82485A30;
	sub_83154600(ctx, base);
	// 82485A30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82485A34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82485A38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82485A3C: 48089A8D  bl 0x8250f4c8
	ctx.lr = 0x82485A40;
	sub_8250F4C8(ctx, base);
	// 82485A40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485A44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82485A48: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82485A4C: 409A0008  bne cr6, 0x82485a54
	if !ctx.cr[6].eq {
	pc = 0x82485A54; continue 'dispatch;
	}
	// 82485A50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82485A54: 48082AC5  bl 0x82508518
	ctx.lr = 0x82485A58;
	sub_82508518(ctx, base);
	// 82485A58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82485A5C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82485A60: 4896C231  bl 0x82df1c90
	ctx.lr = 0x82485A64;
	sub_82DF1C90(ctx, base);
	// 82485A64: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82485A68: C01F00F0  lfs f0, 0xf0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82485A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485A70: D3FF00F0  stfs f31, 0xf0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 82485A74: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82485A78: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82485A7C: 839F00D0  lwz r28, 0xd0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82485A80: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485A84: 485DE26D  bl 0x82a63cf0
	ctx.lr = 0x82485A88;
	sub_82A63CF0(ctx, base);
	// 82485A88: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82485A8C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82485A90: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82485A94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82485A98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82485A9C: 4E800421  bctrl
	ctx.lr = 0x82485AA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82485AA0: 807F00D0  lwz r3, 0xd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82485AA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485AA8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82485AAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82485AB0: 4E800421  bctrl
	ctx.lr = 0x82485AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82485AB4: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82485AB8: 815F00D0  lwz r10, 0xd0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82485ABC: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82485AC0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82485B90 size=92
    let mut pc: u32 = 0x82485B90;
    'dispatch: loop {
        match pc {
            0x82485B90 => {
    //   block [0x82485B90..0x82485BEC)
	// 82485B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485B98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82485B9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82485BA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485BA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485BA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82485BAC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82485BB0: 396BB758  addi r11, r11, -0x48a8
	ctx.r[11].s64 = ctx.r[11].s64 + -18600;
	// 82485BB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82485BB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82485BBC: 4896D86D  bl 0x82df3428
	ctx.lr = 0x82485BC0;
	sub_82DF3428(ctx, base);
	// 82485BC0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82485BC4: 4182000C  beq 0x82485bd0
	if ctx.cr[0].eq {
	pc = 0x82485BD0; continue 'dispatch;
	}
	// 82485BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485BCC: 4BE3A69D  bl 0x822c0268
	ctx.lr = 0x82485BD0;
	sub_822C0268(ctx, base);
	// 82485BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485BD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82485BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485BE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82485BE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82485BF0 size=8
    let mut pc: u32 = 0x82485BF0;
    'dispatch: loop {
        match pc {
            0x82485BF0 => {
    //   block [0x82485BF0..0x82485BF8)
	// 82485BF0: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82485BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82485BF8 size=8
    let mut pc: u32 = 0x82485BF8;
    'dispatch: loop {
        match pc {
            0x82485BF8 => {
    //   block [0x82485BF8..0x82485C00)
	// 82485BF8: 386300E0  addi r3, r3, 0xe0
	ctx.r[3].s64 = ctx.r[3].s64 + 224;
	// 82485BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82485C00 size=176
    let mut pc: u32 = 0x82485C00;
    'dispatch: loop {
        match pc {
            0x82485C00 => {
    //   block [0x82485C00..0x82485CB0)
	// 82485C00: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82485C04: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82485C08: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82485C0C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82485C10: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82485C14: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82485C18: 3CE08204  lis r7, -0x7dfc
	ctx.r[7].s64 = -2113667072;
	// 82485C1C: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82485C20: 3CA08202  lis r5, -0x7dfe
	ctx.r[5].s64 = -2113798144;
	// 82485C24: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82485C28: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82485CB0 size=148
    let mut pc: u32 = 0x82485CB0;
    'dispatch: loop {
        match pc {
            0x82485CB0 => {
    //   block [0x82485CB0..0x82485D44)
	// 82485CB0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82485CB4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82485D48 size=8
    let mut pc: u32 = 0x82485D48;
    'dispatch: loop {
        match pc {
            0x82485D48 => {
    //   block [0x82485D48..0x82485D50)
	// 82485D48: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82485D4C: 4BFFFF64  b 0x82485cb0
	sub_82485CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485D50 size=80
    let mut pc: u32 = 0x82485D50;
    'dispatch: loop {
        match pc {
            0x82485D50 => {
    //   block [0x82485D50..0x82485DA0)
	// 82485D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485D5C: C0040008  lfs f0, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82485D60: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82485D64: C1850008  lfs f12, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82485D68: ED6D0032  fmuls f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82485D6C: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82485D70: C1440000  lfs f10, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82485D74: EC2A5B38  fmsubs f1, f10, f12, f11
	ctx.f[1].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82485D78: EC4D02BA  fmadds f2, f13, f10, f0
	ctx.f[2].f64 = (((ctx.f[13].f64 * ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64);
	// 82485D7C: 48D2523D  bl 0x831aafb8
	ctx.lr = 0x82485D80;
	sub_831AAFB8(ctx, base);
	// 82485D80: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82485D84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82485D88: C00BBBEC  lfs f0, -0x4414(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17428 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82485D8C: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82485D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82485D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82485DA0 size=136
    let mut pc: u32 = 0x82485DA0;
    'dispatch: loop {
        match pc {
            0x82485DA0 => {
    //   block [0x82485DA0..0x82485E28)
	// 82485DA0: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82485E28 size=304
    let mut pc: u32 = 0x82485E28;
    'dispatch: loop {
        match pc {
            0x82485E28 => {
    //   block [0x82485E28..0x82485F58)
	// 82485E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485E2C: 48D2233D  bl 0x831a8168
	ctx.lr = 0x82485E30;
	sub_831A8130(ctx, base);
	// 82485E30: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82485E34: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485E38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82485E3C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82485E40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485E44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82485E48: 388BB7A8  addi r4, r11, -0x4858
	ctx.r[4].s64 = ctx.r[11].s64 + -18520;
	// 82485E4C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82485E50: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82485E54: 4896DBB5  bl 0x82df3a08
	ctx.lr = 0x82485E58;
	sub_82DF3A08(ctx, base);
	// 82485E58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82485E5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82485E60: 396BB758  addi r11, r11, -0x48a8
	ctx.r[11].s64 = ctx.r[11].s64 + -18600;
	// 82485E64: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82485E68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82485E6C: 4896DD95  bl 0x82df3c00
	ctx.lr = 0x82485E70;
	sub_82DF3C00(ctx, base);
	// 82485E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82485E74: 4896D5B5  bl 0x82df3428
	ctx.lr = 0x82485E78;
	sub_82DF3428(ctx, base);
	// 82485E78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82485E7C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82485E80: D3FF000C  stfs f31, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82485E84: 396AB784  addi r11, r10, -0x487c
	ctx.r[11].s64 = ctx.r[10].s64 + -18556;
	// 82485E88: 3BBC0004  addi r29, r28, 4
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	// 82485E8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82485E90: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82485E94: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82485E98: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82485E9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82485EA0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82485EA4: 419A0024  beq cr6, 0x82485ec8
	if ctx.cr[6].eq {
	pc = 0x82485EC8; continue 'dispatch;
	}
	// 82485EA8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82485EAC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82485EB0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82485EB4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82485EB8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82485EBC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82485EC0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82485EC4: 4082FFE8  bne 0x82485eac
	if !ctx.cr[0].eq {
	pc = 0x82485EAC; continue 'dispatch;
	}
	// 82485EC8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82485ECC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82485ED0: 3B8B6910  addi r28, r11, 0x6910
	ctx.r[28].s64 = ctx.r[11].s64 + 26896;
	// 82485ED4: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82485ED8: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82485EDC: 39000050  li r8, 0x50
	ctx.r[8].s64 = 80;
	// 82485EE0: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82485EE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82485EE8: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82485EEC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82485EF0: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82485F58 size=76
    let mut pc: u32 = 0x82485F58;
    'dispatch: loop {
        match pc {
            0x82485F58 => {
    //   block [0x82485F58..0x82485FA4)
	// 82485F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485F60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82485F64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485F68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485F6C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82485F70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82485F74: 419A0008  beq cr6, 0x82485f7c
	if ctx.cr[6].eq {
	pc = 0x82485F7C; continue 'dispatch;
	}
	// 82485F78: 4BE3A919  bl 0x822c0890
	ctx.lr = 0x82485F7C;
	sub_822C0890(ctx, base);
	// 82485F7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82485F80: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82485F84: 396BB758  addi r11, r11, -0x48a8
	ctx.r[11].s64 = ctx.r[11].s64 + -18600;
	// 82485F88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82485F8C: 4896D49D  bl 0x82df3428
	ctx.lr = 0x82485F90;
	sub_82DF3428(ctx, base);
	// 82485F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82485F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485F9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82485FA8 size=76
    let mut pc: u32 = 0x82485FA8;
    'dispatch: loop {
        match pc {
            0x82485FA8 => {
    //   block [0x82485FA8..0x82485FF4)
	// 82485FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82485FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82485FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82485FB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82485FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82485FC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82485FC4: 4BFFFF95  bl 0x82485f58
	ctx.lr = 0x82485FC8;
	sub_82485F58(ctx, base);
	// 82485FC8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82485FCC: 4182000C  beq 0x82485fd8
	if ctx.cr[0].eq {
	pc = 0x82485FD8; continue 'dispatch;
	}
	// 82485FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485FD4: 4BE3A295  bl 0x822c0268
	ctx.lr = 0x82485FD8;
	sub_822C0268(ctx, base);
	// 82485FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82485FDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82485FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82485FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82485FE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82485FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82485FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82485FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82485FF8 size=2412
    let mut pc: u32 = 0x82485FF8;
    'dispatch: loop {
        match pc {
            0x82485FF8 => {
    //   block [0x82485FF8..0x82486964)
	// 82485FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82485FFC: 48D22165  bl 0x831a8160
	ctx.lr = 0x82486000;
	sub_831A8130(ctx, base);
	// 82486000: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 82486004: 48D22A75  bl 0x831a8a78
	ctx.lr = 0x82486008;
	sub_831A8A40(ctx, base);
	// 82486008: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82486968 size=188
    let mut pc: u32 = 0x82486968;
    'dispatch: loop {
        match pc {
            0x82486968 => {
    //   block [0x82486968..0x82486A24)
	// 82486968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248696C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82486970: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82486974: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82486978: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248697C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82486980: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82486984: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82486988: 48088B41  bl 0x8250f4c8
	ctx.lr = 0x8248698C;
	sub_8250F4C8(ctx, base);
	// 8248698C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82486994: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82486998: 409A0008  bne cr6, 0x824869a0
	if !ctx.cr[6].eq {
	pc = 0x824869A0; continue 'dispatch;
	}
	// 8248699C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824869A0: 48081B79  bl 0x82508518
	ctx.lr = 0x824869A4;
	sub_82508518(ctx, base);
	// 824869A4: D03F0018  stfs f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824869A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824869AC: 4896B2E5  bl 0x82df1c90
	ctx.lr = 0x824869B0;
	sub_82DF1C90(ctx, base);
	// 824869B0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824869B4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 824869B8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824869BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824869C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824869C4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824869C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824869CC: 4E800421  bctrl
	ctx.lr = 0x824869D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824869D0: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824869D4: 38BF0070  addi r5, r31, 0x70
	ctx.r[5].s64 = ctx.r[31].s64 + 112;
	// 824869D8: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 824869DC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 824869E0: 48245429  bl 0x826cbe08
	ctx.lr = 0x824869E4;
	sub_826CBE08(ctx, base);
	// 824869E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824869E8: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 824869EC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 824869F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824869F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824869F8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82486A28 size=132
    let mut pc: u32 = 0x82486A28;
    'dispatch: loop {
        match pc {
            0x82486A28 => {
    //   block [0x82486A28..0x82486AAC)
	// 82486A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82486A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82486A30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82486A34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82486A38: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82486A3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82486A40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82486A44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82486A48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82486A4C: 48088A7D  bl 0x8250f4c8
	ctx.lr = 0x82486A50;
	sub_8250F4C8(ctx, base);
	// 82486A50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486A54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82486A58: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82486A5C: 409A0008  bne cr6, 0x82486a64
	if !ctx.cr[6].eq {
	pc = 0x82486A64; continue 'dispatch;
	}
	// 82486A60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82486A64: 48081AB5  bl 0x82508518
	ctx.lr = 0x82486A68;
	sub_82508518(ctx, base);
	// 82486A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82486A6C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82486A70: 4896B221  bl 0x82df1c90
	ctx.lr = 0x82486A74;
	sub_82DF1C90(ctx, base);
	// 82486A74: C01F0018  lfs f0, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82486A78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82486A7C: C05F00B0  lfs f2, 0xb0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82486A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82486A84: EC3F0028  fsubs f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82486A88: 4BFFF571  bl 0x82485ff8
	ctx.lr = 0x82486A8C;
	sub_82485FF8(ctx, base);
	// 82486A8C: D3FF0018  stfs f31, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82486A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82486A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82486A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82486A9C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82486AA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82486AA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82486AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82486AB0 size=8
    let mut pc: u32 = 0x82486AB0;
    'dispatch: loop {
        match pc {
            0x82486AB0 => {
    //   block [0x82486AB0..0x82486AB8)
	// 82486AB0: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 82486AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82486AB8 size=8
    let mut pc: u32 = 0x82486AB8;
    'dispatch: loop {
        match pc {
            0x82486AB8 => {
    //   block [0x82486AB8..0x82486AC0)
	// 82486AB8: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82486ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82486AC0 size=8
    let mut pc: u32 = 0x82486AC0;
    'dispatch: loop {
        match pc {
            0x82486AC0 => {
    //   block [0x82486AC0..0x82486AC8)
	// 82486AC0: C0230038  lfs f1, 0x38(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82486AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82486AC8 size=24
    let mut pc: u32 = 0x82486AC8;
    'dispatch: loop {
        match pc {
            0x82486AC8 => {
    //   block [0x82486AC8..0x82486AE0)
	// 82486AC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82486ACC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82486AD0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82486AD4: 419A0028  beq cr6, 0x82486afc
	if ctx.cr[6].eq {
		sub_82486AFC(ctx, base);
		return;
	}
	// 82486AD8: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82486ADC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82486AE0 size=28
    let mut pc: u32 = 0x82486AE0;
    'dispatch: loop {
        match pc {
            0x82486AE0 => {
    //   block [0x82486AE0..0x82486AFC)
	// 82486AE0: 38CB0050  addi r6, r11, 0x50
	ctx.r[6].s64 = ctx.r[11].s64 + 80;
	// 82486AE4: C04B001C  lfs f2, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82486AE8: 38AB0040  addi r5, r11, 0x40
	ctx.r[5].s64 = ctx.r[11].s64 + 64;
	// 82486AEC: C02B0018  lfs f1, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82486AF0: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82486AF4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82486AF8: 481CCE88  b 0x82653980
	sub_82653980(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486AFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82486AFC size=24
    let mut pc: u32 = 0x82486AFC;
    'dispatch: loop {
        match pc {
            0x82486AFC => {
    //   block [0x82486AFC..0x82486B14)
	// 82486AFC: 38AB0050  addi r5, r11, 0x50
	ctx.r[5].s64 = ctx.r[11].s64 + 80;
	// 82486B00: C04B001C  lfs f2, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82486B04: C02B0018  lfs f1, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82486B08: 388B0040  addi r4, r11, 0x40
	ctx.r[4].s64 = ctx.r[11].s64 + 64;
	// 82486B0C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82486B10: 481CCD88  b 0x82653898
	sub_82653898(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82486B14 size=4
    let mut pc: u32 = 0x82486B14;
    'dispatch: loop {
        match pc {
            0x82486B14 => {
    //   block [0x82486B14..0x82486B18)
	// 82486B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82486B18 size=284
    let mut pc: u32 = 0x82486B18;
    'dispatch: loop {
        match pc {
            0x82486B18 => {
    //   block [0x82486B18..0x82486C34)
	// 82486B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82486B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82486B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82486B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82486B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82486B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82486B30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82486B34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82486B38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82486B3C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82486B40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82486B44: 409A0020  bne cr6, 0x82486b64
	if !ctx.cr[6].eq {
	pc = 0x82486B64; continue 'dispatch;
	}
	// 82486B48: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82486B4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486B50: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82486B54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82486B58: 4E800421  bctrl
	ctx.lr = 0x82486B5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82486B5C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82486B60: 4800001C  b 0x82486b7c
	pc = 0x82486B7C; continue 'dispatch;
	// 82486B64: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82486B68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486B6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82486B70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82486B74: 4E800421  bctrl
	ctx.lr = 0x82486B78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82486B78: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82486B7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486B80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82486B84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82486B88: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82486B8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82486B90: 4E800421  bctrl
	ctx.lr = 0x82486B94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82486B94: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82486B98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82486B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82486BA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486BA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82486BA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82486BAC: 4E800421  bctrl
	ctx.lr = 0x82486BB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82486BB0: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82486BB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486BB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82486BBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82486BC0: 4E800421  bctrl
	ctx.lr = 0x82486BC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82486BC4: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82486BC8: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82486C38 size=24
    let mut pc: u32 = 0x82486C38;
    'dispatch: loop {
        match pc {
            0x82486C38 => {
    //   block [0x82486C38..0x82486C50)
	// 82486C38: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82486C3C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82486C40: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82486C44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82486C48: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82486C4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82486C50 size=36
    let mut pc: u32 = 0x82486C50;
    'dispatch: loop {
        match pc {
            0x82486C50 => {
    //   block [0x82486C50..0x82486C74)
	// 82486C50: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82486C54: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82486C58: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486C5C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82486C60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82486C64: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82486C68: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486C6C: 4082FFE8  bne 0x82486c54
	if !ctx.cr[0].eq {
	pc = 0x82486C54; continue 'dispatch;
	}
	// 82486C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82486C78 size=384
    let mut pc: u32 = 0x82486C78;
    'dispatch: loop {
        match pc {
            0x82486C78 => {
    //   block [0x82486C78..0x82486DF8)
	// 82486C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82486C7C: 48D214E9  bl 0x831a8164
	ctx.lr = 0x82486C80;
	sub_831A8130(ctx, base);
	// 82486C80: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82486C84: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82486DF8 size=436
    let mut pc: u32 = 0x82486DF8;
    'dispatch: loop {
        match pc {
            0x82486DF8 => {
    //   block [0x82486DF8..0x82486FAC)
	// 82486DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82486DFC: 48D21365  bl 0x831a8160
	ctx.lr = 0x82486E00;
	sub_831A8130(ctx, base);
	// 82486E00: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 82486E04: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82486E08: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82486E0C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82486E10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82486E14: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82486E18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82486E1C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82486E20: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82486E24: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82486E28: 396BB758  addi r11, r11, -0x48a8
	ctx.r[11].s64 = ctx.r[11].s64 + -18600;
	// 82486E2C: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82486E30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82486E34: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82486E38: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82486E3C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82486E40: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82486E44: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 82486E48: 4896CDB9  bl 0x82df3c00
	ctx.lr = 0x82486E4C;
	sub_82DF3C00(ctx, base);
	// 82486E4C: 812100F4  lwz r9, 0xf4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 82486E50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82486E54: D3DF0018  stfs f30, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82486E58: D3BF001C  stfs f29, 0x1c(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82486E5C: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82486E60: 394BB7BC  addi r10, r11, -0x4844
	ctx.r[10].s64 = ctx.r[11].s64 + -18500;
	// 82486E64: 935F0010  stw r26, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 82486E68: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82486E6C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82486E70: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 82486E74: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82486E78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486E7C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82486E80: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82486E84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82486E88: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82486E8C: 419A0024  beq cr6, 0x82486eb0
	if ctx.cr[6].eq {
	pc = 0x82486EB0; continue 'dispatch;
	}
	// 82486E90: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82486E94: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82486E98: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486E9C: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82486EA0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82486EA4: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82486EA8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486EAC: 4082FFE8  bne 0x82486e94
	if !ctx.cr[0].eq {
	pc = 0x82486E94; continue 'dispatch;
	}
	// 82486EB0: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486EB4: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 82486EB8: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82486EBC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82486EC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82486EC4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82486EC8: 419A0024  beq cr6, 0x82486eec
	if ctx.cr[6].eq {
	pc = 0x82486EEC; continue 'dispatch;
	}
	// 82486ECC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82486ED0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82486ED4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486ED8: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82486EDC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82486EE0: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82486EE4: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486EE8: 4082FFE8  bne 0x82486ed0
	if !ctx.cr[0].eq {
	pc = 0x82486ED0; continue 'dispatch;
	}
	// 82486EEC: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82486EF0: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82486EF4: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82486EF8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82486EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82486F00: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82486F04: 419A0024  beq cr6, 0x82486f28
	if ctx.cr[6].eq {
	pc = 0x82486F28; continue 'dispatch;
	}
	// 82486F08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82486F0C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82486F10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486F14: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82486F18: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82486F1C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82486F20: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82486F24: 4082FFE8  bne 0x82486f0c
	if !ctx.cr[0].eq {
	pc = 0x82486F0C; continue 'dispatch;
	}
	// 82486F28: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82486F2C: D3FF0038  stfs f31, 0x38(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82486F30: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82486F34: 390B6910  addi r8, r11, 0x6910
	ctx.r[8].s64 = ctx.r[11].s64 + 26896;
	// 82486F38: 38E00050  li r7, 0x50
	ctx.r[7].s64 = 80;
	// 82486F3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82486F40: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82486F44: 13E040C7  vcmpequd (lvx128) v31, v0, v8
	tmp.u32 = ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82486FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82486FB0 size=124
    let mut pc: u32 = 0x82486FB0;
    'dispatch: loop {
        match pc {
            0x82486FB0 => {
    //   block [0x82486FB0..0x8248702C)
	// 82486FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82486FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82486FB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82486FBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82486FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82486FC4: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82486FC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82486FCC: 419A0008  beq cr6, 0x82486fd4
	if ctx.cr[6].eq {
	pc = 0x82486FD4; continue 'dispatch;
	}
	// 82486FD0: 4BE398C1  bl 0x822c0890
	ctx.lr = 0x82486FD4;
	sub_822C0890(ctx, base);
	// 82486FD4: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82486FD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82486FDC: 419A0008  beq cr6, 0x82486fe4
	if ctx.cr[6].eq {
	pc = 0x82486FE4; continue 'dispatch;
	}
	// 82486FE0: 4BE398B1  bl 0x822c0890
	ctx.lr = 0x82486FE4;
	sub_822C0890(ctx, base);
	// 82486FE4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82486FE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82486FEC: 419A0008  beq cr6, 0x82486ff4
	if ctx.cr[6].eq {
	pc = 0x82486FF4; continue 'dispatch;
	}
	// 82486FF0: 4BE398A1  bl 0x822c0890
	ctx.lr = 0x82486FF4;
	sub_822C0890(ctx, base);
	// 82486FF4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82486FF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82486FFC: 419A0008  beq cr6, 0x82487004
	if ctx.cr[6].eq {
	pc = 0x82487004; continue 'dispatch;
	}
	// 82487000: 4BE39891  bl 0x822c0890
	ctx.lr = 0x82487004;
	sub_822C0890(ctx, base);
	// 82487004: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82487008: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8248700C: 396BB758  addi r11, r11, -0x48a8
	ctx.r[11].s64 = ctx.r[11].s64 + -18600;
	// 82487010: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487014: 4896C415  bl 0x82df3428
	ctx.lr = 0x82487018;
	sub_82DF3428(ctx, base);
	// 82487018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248701C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82487020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82487024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82487028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82487030 size=76
    let mut pc: u32 = 0x82487030;
    'dispatch: loop {
        match pc {
            0x82487030 => {
    //   block [0x82487030..0x8248707C)
	// 82487030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487038: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248703C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487048: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248704C: 4BFFFF65  bl 0x82486fb0
	ctx.lr = 0x82487050;
	sub_82486FB0(ctx, base);
	// 82487050: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82487054: 4182000C  beq 0x82487060
	if ctx.cr[0].eq {
	pc = 0x82487060; continue 'dispatch;
	}
	// 82487058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248705C: 4BE3920D  bl 0x822c0268
	ctx.lr = 0x82487060;
	sub_822C0268(ctx, base);
	// 82487060: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82487064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82487068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248706C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82487070: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82487074: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82487078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487080 size=276
    let mut pc: u32 = 0x82487080;
    'dispatch: loop {
        match pc {
            0x82487080 => {
    //   block [0x82487080..0x82487194)
	// 82487080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487084: 48D210E9  bl 0x831a816c
	ctx.lr = 0x82487088;
	sub_831A8130(ctx, base);
	// 82487088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248708C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487090: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82487094: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82487098: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8248709C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824870A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824870A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824870A8: 409A0020  bne cr6, 0x824870c8
	if !ctx.cr[6].eq {
	pc = 0x824870C8; continue 'dispatch;
	}
	// 824870AC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824870B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824870B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824870B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824870BC: 4E800421  bctrl
	ctx.lr = 0x824870C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824870C0: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824870C4: 4800001C  b 0x824870e0
	pc = 0x824870E0; continue 'dispatch;
	// 824870C8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824870CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824870D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824870D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824870D8: 4E800421  bctrl
	ctx.lr = 0x824870DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824870DC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824870E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824870E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824870E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824870EC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824870F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824870F4: 4E800421  bctrl
	ctx.lr = 0x824870F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824870F8: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824870FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82487100: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82487104: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487108: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248710C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487110: 4E800421  bctrl
	ctx.lr = 0x82487114;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487114: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82487118: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248711C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82487120: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487124: 4E800421  bctrl
	ctx.lr = 0x82487128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487128: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 8248712C: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487198 size=364
    let mut pc: u32 = 0x82487198;
    'dispatch: loop {
        match pc {
            0x82487198 => {
    //   block [0x82487198..0x82487304)
	// 82487198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248719C: 48D20FCD  bl 0x831a8168
	ctx.lr = 0x824871A0;
	sub_831A8130(ctx, base);
	// 824871A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824871A4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824871A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824871AC: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 824871B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824871B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824871B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824871BC: 388BB7E0  addi r4, r11, -0x4820
	ctx.r[4].s64 = ctx.r[11].s64 + -18464;
	// 824871C0: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 824871C4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 824871C8: 4BE39211  bl 0x822c03d8
	ctx.lr = 0x824871CC;
	sub_822C03D8(ctx, base);
	// 824871CC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824871D0: 41820028  beq 0x824871f8
	if ctx.cr[0].eq {
	pc = 0x824871F8; continue 'dispatch;
	}
	// 824871D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824871D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824871DC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 824871E0: 48088339  bl 0x8250f518
	ctx.lr = 0x824871E4;
	sub_8250F518(ctx, base);
	// 824871E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824871E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824871EC: 48009F05  bl 0x824910f0
	ctx.lr = 0x824871F0;
	sub_824910F0(ctx, base);
	// 824871F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824871F4: 48000008  b 0x824871fc
	pc = 0x824871FC; continue 'dispatch;
	// 824871F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824871FC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82487200: 4BE834E1  bl 0x8230a6e0
	ctx.lr = 0x82487204;
	sub_8230A6E0(ctx, base);
	// 82487204: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82487208: 4182000C  beq 0x82487214
	if ctx.cr[0].eq {
	pc = 0x82487214; continue 'dispatch;
	}
	// 8248720C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82487210: 4896AA81  bl 0x82df1c90
	ctx.lr = 0x82487214;
	sub_82DF1C90(ctx, base);
	// 82487214: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82487218: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8248721C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82487220: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82487224: 409A0020  bne cr6, 0x82487244
	if !ctx.cr[6].eq {
	pc = 0x82487244; continue 'dispatch;
	}
	// 82487228: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8248722C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487230: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82487234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487238: 4E800421  bctrl
	ctx.lr = 0x8248723C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248723C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82487240: 4800001C  b 0x8248725c
	pc = 0x8248725C; continue 'dispatch;
	// 82487244: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82487248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248724C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82487250: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487254: 4E800421  bctrl
	ctx.lr = 0x82487258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487258: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8248725C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487260: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82487264: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82487268: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248726C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487270: 4E800421  bctrl
	ctx.lr = 0x82487274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487274: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82487278: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8248727C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82487280: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487284: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82487288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248728C: 4E800421  bctrl
	ctx.lr = 0x82487290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487290: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82487294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487298: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248729C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824872A0: 4E800421  bctrl
	ctx.lr = 0x824872A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824872A4: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 824872A8: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82487308 size=64
    let mut pc: u32 = 0x82487308;
    'dispatch: loop {
        match pc {
            0x82487308 => {
    //   block [0x82487308..0x82487348)
	// 82487308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248730C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82487310: 396BB854  addi r11, r11, -0x47ac
	ctx.r[11].s64 = ctx.r[11].s64 + -18348;
	// 82487314: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82487318: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 8248731C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487320: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82487324: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487328: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487348 size=164
    let mut pc: u32 = 0x82487348;
    'dispatch: loop {
        match pc {
            0x82487348 => {
    //   block [0x82487348..0x824873EC)
	// 82487348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82487354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487358: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8248735C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487360: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487368: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8248736C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82487370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487374: 4E800421  bctrl
	ctx.lr = 0x82487378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487378: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 8248737C: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487380: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82487384: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82487388: C3FF0040  lfs f31, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8248738C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82487390: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82487394: 13C0F0C7  vcmpequd (lvx128) v30, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824873F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824873F0 size=16
    let mut pc: u32 = 0x824873F0;
    'dispatch: loop {
        match pc {
            0x824873F0 => {
    //   block [0x824873F0..0x82487400)
	// 824873F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824873F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824873F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824873FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82487400 size=64
    let mut pc: u32 = 0x82487400;
    'dispatch: loop {
        match pc {
            0x82487400 => {
    //   block [0x82487400..0x82487440)
	// 82487400: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82487404: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82487408: 396BB86C  addi r11, r11, -0x4794
	ctx.r[11].s64 = ctx.r[11].s64 + -18324;
	// 8248740C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82487410: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 82487414: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487418: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 8248741C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487420: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487440 size=204
    let mut pc: u32 = 0x82487440;
    'dispatch: loop {
        match pc {
            0x82487440 => {
    //   block [0x82487440..0x8248750C)
	// 82487440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248744C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487450: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487458: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8248745C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82487460: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487464: 4E800421  bctrl
	ctx.lr = 0x82487468;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487468: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8248746C: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487470: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82487474: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82487510 size=80
    let mut pc: u32 = 0x82487510;
    'dispatch: loop {
        match pc {
            0x82487510 => {
    //   block [0x82487510..0x82487560)
	// 82487510: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82487514: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82487518: 396BB884  addi r11, r11, -0x477c
	ctx.r[11].s64 = ctx.r[11].s64 + -18300;
	// 8248751C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82487520: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487524: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 82487528: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8248752C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487530: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82487534: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82487560 size=408
    let mut pc: u32 = 0x82487560;
    'dispatch: loop {
        match pc {
            0x82487560 => {
    //   block [0x82487560..0x824876F8)
	// 82487560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248756C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82487570: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824876F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824876F8 size=64
    let mut pc: u32 = 0x824876F8;
    'dispatch: loop {
        match pc {
            0x824876F8 => {
    //   block [0x824876F8..0x82487738)
	// 824876F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824876FC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82487700: 396BB89C  addi r11, r11, -0x4764
	ctx.r[11].s64 = ctx.r[11].s64 + -18276;
	// 82487704: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82487708: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 8248770C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487710: 39296910  addi r9, r9, 0x6910
	ctx.r[9].s64 = ctx.r[9].s64 + 26896;
	// 82487714: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487718: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82487738 size=160
    let mut pc: u32 = 0x82487738;
    'dispatch: loop {
        match pc {
            0x82487738 => {
    //   block [0x82487738..0x824877D8)
	// 82487738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248773C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487744: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82487748: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824877D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824877D8 size=48
    let mut pc: u32 = 0x824877D8;
    'dispatch: loop {
        match pc {
            0x824877D8 => {
    //   block [0x824877D8..0x82487808)
	// 824877D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824877DC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824877E0: 396BB8B4  addi r11, r11, -0x474c
	ctx.r[11].s64 = ctx.r[11].s64 + -18252;
	// 824877E4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 824877E8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824877EC: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82487808 size=328
    let mut pc: u32 = 0x82487808;
    'dispatch: loop {
        match pc {
            0x82487808 => {
    //   block [0x82487808..0x82487950)
	// 82487808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248780C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82487814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487818: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8248781C: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487950 size=168
    let mut pc: u32 = 0x82487950;
    'dispatch: loop {
        match pc {
            0x82487950 => {
    //   block [0x82487950..0x824879F8)
	// 82487950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248795C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487960: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82487964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487968: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8248796C: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82487970: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82487974: 396AB8CC  addi r11, r10, -0x4734
	ctx.r[11].s64 = ctx.r[10].s64 + -18228;
	// 82487978: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8248797C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487980: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82487984: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824879F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824879F8 size=204
    let mut pc: u32 = 0x824879F8;
    'dispatch: loop {
        match pc {
            0x824879F8 => {
    //   block [0x824879F8..0x82487AC4)
	// 824879F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824879FC: 48D20771  bl 0x831a816c
	ctx.lr = 0x82487A00;
	sub_831A8130(ctx, base);
	// 82487A00: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487AC8 size=188
    let mut pc: u32 = 0x82487AC8;
    'dispatch: loop {
        match pc {
            0x82487AC8 => {
    //   block [0x82487AC8..0x82487B84)
	// 82487AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82487AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487AD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487AE0: C0240008  lfs f1, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82487AE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82487AE8: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82487AEC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82487AF0: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82487AF4: 481CBB25  bl 0x82653618
	ctx.lr = 0x82487AF8;
	sub_82653618(ctx, base);
	// 82487AF8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82487AFC: 13C0F0C7  vcmpequd (lvx128) v30, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487B00: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82487B04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82487B08: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82487B0C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82487B10: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487B14: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487B88 size=180
    let mut pc: u32 = 0x82487B88;
    'dispatch: loop {
        match pc {
            0x82487B88 => {
    //   block [0x82487B88..0x82487C3C)
	// 82487B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487B9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82487BA0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82487BA4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82487BA8: 396BB8E4  addi r11, r11, -0x471c
	ctx.r[11].s64 = ctx.r[11].s64 + -18204;
	// 82487BAC: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 82487BB0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82487BB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487BB8: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82487BBC: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82487C40 size=100
    let mut pc: u32 = 0x82487C40;
    'dispatch: loop {
        match pc {
            0x82487C40 => {
    //   block [0x82487C40..0x82487CA4)
	// 82487C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82487C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487C58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82487C5C: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82487C60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82487C64: 419A0008  beq cr6, 0x82487c6c
	if ctx.cr[6].eq {
	pc = 0x82487C6C; continue 'dispatch;
	}
	// 82487C68: 4BE38C29  bl 0x822c0890
	ctx.lr = 0x82487C6C;
	sub_822C0890(ctx, base);
	// 82487C6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82487C70: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82487C74: 396BB83C  addi r11, r11, -0x47c4
	ctx.r[11].s64 = ctx.r[11].s64 + -18372;
	// 82487C78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487C7C: 4182000C  beq 0x82487c88
	if ctx.cr[0].eq {
	pc = 0x82487C88; continue 'dispatch;
	}
	// 82487C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82487C84: 4BE385E5  bl 0x822c0268
	ctx.lr = 0x82487C88;
	sub_822C0268(ctx, base);
	// 82487C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82487C8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82487C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82487C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82487C98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82487C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82487CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82487CA8 size=16
    let mut pc: u32 = 0x82487CA8;
    'dispatch: loop {
        match pc {
            0x82487CA8 => {
    //   block [0x82487CA8..0x82487CB8)
	// 82487CA8: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 82487CAC: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487CB8 size=188
    let mut pc: u32 = 0x82487CB8;
    'dispatch: loop {
        match pc {
            0x82487CB8 => {
    //   block [0x82487CB8..0x82487D74)
	// 82487CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487CC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487CC8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82487CCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487CD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82487CD4: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82487CD8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82487CDC: 396AB8FC  addi r11, r10, -0x4704
	ctx.r[11].s64 = ctx.r[10].s64 + -18180;
	// 82487CE0: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82487CE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82487CE8: 395F0040  addi r10, r31, 0x40
	ctx.r[10].s64 = ctx.r[31].s64 + 64;
	// 82487CEC: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487CF0: 39650004  addi r11, r5, 4
	ctx.r[11].s64 = ctx.r[5].s64 + 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82487D78 size=352
    let mut pc: u32 = 0x82487D78;
    'dispatch: loop {
        match pc {
            0x82487D78 => {
    //   block [0x82487D78..0x82487ED8)
	// 82487D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487D7C: 48D203ED  bl 0x831a8168
	ctx.lr = 0x82487D80;
	sub_831A8130(ctx, base);
	// 82487D80: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487D84: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82487D88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82487D8C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82487D90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82487D94: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 82487D98: 409A0008  bne cr6, 0x82487da0
	if !ctx.cr[6].eq {
	pc = 0x82487DA0; continue 'dispatch;
	}
	// 82487D9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82487DA0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82487DA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82487DA8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82487DAC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82487DB0: 4BFD1561  bl 0x82459310
	ctx.lr = 0x82487DB4;
	sub_82459310(ctx, base);
	// 82487DB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487DB8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82487DBC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82487DC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82487DC4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82487DC8: 419A0024  beq cr6, 0x82487dec
	if ctx.cr[6].eq {
	pc = 0x82487DEC; continue 'dispatch;
	}
	// 82487DCC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82487DD0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82487DD4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82487DD8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82487DDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82487DE0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82487DE4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82487DE8: 4082FFE8  bne 0x82487dd0
	if !ctx.cr[0].eq {
	pc = 0x82487DD0; continue 'dispatch;
	}
	// 82487DEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82487DF0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82487DF4: 48087775  bl 0x8250f568
	ctx.lr = 0x82487DF8;
	sub_8250F568(ctx, base);
	// 82487DF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487DFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82487E00: 386BFF40  addi r3, r11, -0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + -192;
	// 82487E04: 409A0008  bne cr6, 0x82487e0c
	if !ctx.cr[6].eq {
	pc = 0x82487E0C; continue 'dispatch;
	}
	// 82487E08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82487E0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82487E10: 80DE0020  lwz r6, 0x20(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82487E14: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82487E18: 388BB910  addi r4, r11, -0x46f0
	ctx.r[4].s64 = ctx.r[11].s64 + -18160;
	// 82487E1C: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82487E20: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 82487E24: 48328EBD  bl 0x827b0ce0
	ctx.lr = 0x82487E28;
	sub_827B0CE0(ctx, base);
	// 82487E28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82487E2C: 48969E65  bl 0x82df1c90
	ctx.lr = 0x82487E30;
	sub_82DF1C90(ctx, base);
	// 82487E30: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82487E34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82487E38: 419A0008  beq cr6, 0x82487e40
	if ctx.cr[6].eq {
	pc = 0x82487E40; continue 'dispatch;
	}
	// 82487E3C: 4BE38A55  bl 0x822c0890
	ctx.lr = 0x82487E40;
	sub_822C0890(ctx, base);
	// 82487E40: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82487E44: C03C0008  lfs f1, 8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82487E48: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82487E4C: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82487E50: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82487E54: 3BFE0030  addi r31, r30, 0x30
	ctx.r[31].s64 = ctx.r[30].s64 + 48;
	// 82487E58: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82487E5C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82487E60: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82487E64: 13DE50C7  vcmpequd (lvx128) v30, v30, v10
	tmp.u32 = ctx.r[30].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82487ED8 size=8
    let mut pc: u32 = 0x82487ED8;
    'dispatch: loop {
        match pc {
            0x82487ED8 => {
    //   block [0x82487ED8..0x82487EE0)
	// 82487ED8: D0230048  stfs f1, 0x48(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82487EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82487EE0 size=96
    let mut pc: u32 = 0x82487EE0;
    'dispatch: loop {
        match pc {
            0x82487EE0 => {
    //   block [0x82487EE0..0x82487F40)
	// 82487EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487EE4: 48D20281  bl 0x831a8164
	ctx.lr = 0x82487EE8;
	sub_831A8130(ctx, base);
	// 82487EE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82487EEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82487EF0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82487EF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82487EF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82487EFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82487F00: 839F0040  lwz r28, 0x40(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82487F04: 3B7F0030  addi r27, r31, 0x30
	ctx.r[27].s64 = ctx.r[31].s64 + 48;
	// 82487F08: 4BFDCE91  bl 0x82464d98
	ctx.lr = 0x82487F0C;
	sub_82464D98(ctx, base);
	// 82487F0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82487F10: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82487F14: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82487F18: 481CB799  bl 0x826536b0
	ctx.lr = 0x82487F1C;
	sub_826536B0(ctx, base);
	// 82487F1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82487F20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82487F24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82487F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82487F2C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82487F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82487F34: 4E800421  bctrl
	ctx.lr = 0x82487F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82487F38: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82487F3C: 48D20278  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82487F40 size=16
    let mut pc: u32 = 0x82487F40;
    'dispatch: loop {
        match pc {
            0x82487F40 => {
    //   block [0x82487F40..0x82487F50)
	// 82487F40: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82487F44: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82487F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82487F50 size=180
    let mut pc: u32 = 0x82487F50;
    'dispatch: loop {
        match pc {
            0x82487F50 => {
    //   block [0x82487F50..0x82488004)
	// 82487F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82487F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82487F58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82487F5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82487F60: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488008 size=180
    let mut pc: u32 = 0x82488008;
    'dispatch: loop {
        match pc {
            0x82488008 => {
    //   block [0x82488008..0x824880BC)
	// 82488008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248800C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248801C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82488020: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82488024: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82488028: 396BB978  addi r11, r11, -0x4688
	ctx.r[11].s64 = ctx.r[11].s64 + -18056;
	// 8248802C: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 82488030: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82488034: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82488038: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 8248803C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824880C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824880C0 size=56
    let mut pc: u32 = 0x824880C0;
    'dispatch: loop {
        match pc {
            0x824880C0 => {
    //   block [0x824880C0..0x824880F8)
	// 824880C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824880C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824880C8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824880CC: 396BB990  addi r11, r11, -0x4670
	ctx.r[11].s64 = ctx.r[11].s64 + -18032;
	// 824880D0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 824880D4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824880D8: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824880DC: C00ACEE4  lfs f0, -0x311c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824880F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824880F8 size=276
    let mut pc: u32 = 0x824880F8;
    'dispatch: loop {
        match pc {
            0x824880F8 => {
    //   block [0x824880F8..0x8248820C)
	// 824880F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824880FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488104: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82488210 size=16
    let mut pc: u32 = 0x82488210;
    'dispatch: loop {
        match pc {
            0x82488210 => {
    //   block [0x82488210..0x82488220)
	// 82488210: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82488214: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488220 size=108
    let mut pc: u32 = 0x82488220;
    'dispatch: loop {
        match pc {
            0x82488220 => {
    //   block [0x82488220..0x8248828C)
	// 82488220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248822C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488238: C0240008  lfs f1, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8248823C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82488240: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82488244: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82488248: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8248824C: 481CB3CD  bl 0x82653618
	ctx.lr = 0x82488250;
	sub_82653618(ctx, base);
	// 82488250: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82488254: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82488258: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8248825C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82488260: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488290 size=180
    let mut pc: u32 = 0x82488290;
    'dispatch: loop {
        match pc {
            0x82488290 => {
    //   block [0x82488290..0x82488344)
	// 82488290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248829C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824882A0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 824882A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824882A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 824882AC: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 824882B0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824882B4: 396AB9A8  addi r11, r10, -0x4658
	ctx.r[11].s64 = ctx.r[10].s64 + -18008;
	// 824882B8: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 824882BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824882C0: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 824882C4: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824882C8: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82488348 size=100
    let mut pc: u32 = 0x82488348;
    'dispatch: loop {
        match pc {
            0x82488348 => {
    //   block [0x82488348..0x824883AC)
	// 82488348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248834C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82488354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248835C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488360: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82488364: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82488368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248836C: 419A0008  beq cr6, 0x82488374
	if ctx.cr[6].eq {
	pc = 0x82488374; continue 'dispatch;
	}
	// 82488370: 4BE38521  bl 0x822c0890
	ctx.lr = 0x82488374;
	sub_822C0890(ctx, base);
	// 82488374: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82488378: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248837C: 396BB83C  addi r11, r11, -0x47c4
	ctx.r[11].s64 = ctx.r[11].s64 + -18372;
	// 82488380: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82488384: 4182000C  beq 0x82488390
	if ctx.cr[0].eq {
	pc = 0x82488390; continue 'dispatch;
	}
	// 82488388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248838C: 4BE37EDD  bl 0x822c0268
	ctx.lr = 0x82488390;
	sub_822C0268(ctx, base);
	// 82488390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82488394: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82488398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248839C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824883A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824883A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824883A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824883B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824883B0 size=200
    let mut pc: u32 = 0x824883B0;
    'dispatch: loop {
        match pc {
            0x824883B0 => {
    //   block [0x824883B0..0x82488478)
	// 824883B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824883B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824883B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824883BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824883C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824883C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824883C8: C0240008  lfs f1, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824883CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824883D0: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 824883D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824883D8: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 824883DC: 481CB23D  bl 0x82653618
	ctx.lr = 0x824883E0;
	sub_82653618(ctx, base);
	// 824883E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824883E4: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824883E8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824883EC: 4BE47B45  bl 0x822cff30
	ctx.lr = 0x824883F0;
	sub_822CFF30(ctx, base);
	// 824883F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824883F4: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824883F8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824883FC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82488400: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82488404: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82488408: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82488478 size=688
    let mut pc: u32 = 0x82488478;
    'dispatch: loop {
        match pc {
            0x82488478 => {
    //   block [0x82488478..0x82488728)
	// 82488478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248847C: 48D1FCED  bl 0x831a8168
	ctx.lr = 0x82488480;
	sub_831A8130(ctx, base);
	// 82488480: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82488484: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82488488: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488728 size=292
    let mut pc: u32 = 0x82488728;
    'dispatch: loop {
        match pc {
            0x82488728 => {
    //   block [0x82488728..0x8248884C)
	// 82488728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82488734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248873C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82488740: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82488744: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82488748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248874C: 394B6910  addi r10, r11, 0x6910
	ctx.r[10].s64 = ctx.r[11].s64 + 26896;
	// 82488750: 3968B9C0  addi r11, r8, -0x4640
	ctx.r[11].s64 = ctx.r[8].s64 + -17984;
	// 82488754: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82488758: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 8248875C: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82488760: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82488764: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82488768: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8248876C: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82488770: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82488774: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82488778: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8248877C: 39240004  addi r9, r4, 4
	ctx.r[9].s64 = ctx.r[4].s64 + 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488850 size=72
    let mut pc: u32 = 0x82488850;
    'dispatch: loop {
        match pc {
            0x82488850 => {
    //   block [0x82488850..0x82488898)
	// 82488850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488854: 48D1F919  bl 0x831a816c
	ctx.lr = 0x82488858;
	sub_831A8130(ctx, base);
	// 82488858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248885C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488860: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82488864: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82488868: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8248886C: 4BE476A5  bl 0x822cff10
	ctx.lr = 0x82488870;
	sub_822CFF10(ctx, base);
	// 82488870: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82488874: D03F0044  stfs f1, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82488878: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8248887C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82488880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82488884: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82488888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248888C: 4E800421  bctrl
	ctx.lr = 0x82488890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82488890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82488894: 48D1F928  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488898 size=276
    let mut pc: u32 = 0x82488898;
    'dispatch: loop {
        match pc {
            0x82488898 => {
    //   block [0x82488898..0x824889AC)
	// 82488898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248889C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824888A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824888A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824888A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824888AC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 824888B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824888B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 824888B8: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 824888BC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824888C0: 396AB9D8  addi r11, r10, -0x4628
	ctx.r[11].s64 = ctx.r[10].s64 + -17960;
	// 824888C4: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 824888C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824888CC: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 824888D0: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824888D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824889B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824889B0 size=348
    let mut pc: u32 = 0x824889B0;
    'dispatch: loop {
        match pc {
            0x824889B0 => {
    //   block [0x824889B0..0x82488B0C)
	// 824889B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824889B4: 48D1F7B1  bl 0x831a8164
	ctx.lr = 0x824889B8;
	sub_831A8130(ctx, base);
	// 824889B8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824889BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824889C0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824889C4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824889C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824889CC: 419A0118  beq cr6, 0x82488ae4
	if ctx.cr[6].eq {
	pc = 0x82488AE4; continue 'dispatch;
	}
	// 824889D0: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 824889D4: C0240008  lfs f1, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824889D8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 824889DC: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 824889E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824889E4: 481CAC35  bl 0x82653618
	ctx.lr = 0x824889E8;
	sub_82653618(ctx, base);
	// 824889E8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 824889EC: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824889F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824889F4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824889F8: 3BABB9F0  addi r29, r11, -0x4610
	ctx.r[29].s64 = ctx.r[11].s64 + -17936;
	// 824889FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82488A00: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82488A04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488B10 size=172
    let mut pc: u32 = 0x82488B10;
    'dispatch: loop {
        match pc {
            0x82488B10 => {
    //   block [0x82488B10..0x82488BBC)
	// 82488B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488B18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488B1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488B20: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82488B24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488B28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82488B2C: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82488B30: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82488B34: 394ABA4C  addi r10, r10, -0x45b4
	ctx.r[10].s64 = ctx.r[10].s64 + -17844;
	// 82488B38: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82488B3C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82488B40: 39440004  addi r10, r4, 4
	ctx.r[10].s64 = ctx.r[4].s64 + 4;
	// 82488B44: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82488BC0 size=64
    let mut pc: u32 = 0x82488BC0;
    'dispatch: loop {
        match pc {
            0x82488BC0 => {
    //   block [0x82488BC0..0x82488C00)
	// 82488BC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82488BC4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82488BC8: 396BBA64  addi r11, r11, -0x459c
	ctx.r[11].s64 = ctx.r[11].s64 + -17820;
	// 82488BCC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82488BD0: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 82488BD4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82488BD8: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82488BDC: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82488BE0: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82488C00 size=224
    let mut pc: u32 = 0x82488C00;
    'dispatch: loop {
        match pc {
            0x82488C00 => {
    //   block [0x82488C00..0x82488CE0)
	// 82488C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488C0C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82488C10: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488CE0 size=176
    let mut pc: u32 = 0x82488CE0;
    'dispatch: loop {
        match pc {
            0x82488CE0 => {
    //   block [0x82488CE0..0x82488D90)
	// 82488CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488CE4: 48D1F485  bl 0x831a8168
	ctx.lr = 0x82488CE8;
	sub_831A8130(ctx, base);
	// 82488CE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488CEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488CF0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82488CF4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82488CF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82488CFC: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82488D00: 839F0038  lwz r28, 0x38(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82488D04: 4BFDC095  bl 0x82464d98
	ctx.lr = 0x82488D08;
	sub_82464D98(ctx, base);
	// 82488D08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82488D0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82488D10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82488D14: 481CA99D  bl 0x826536b0
	ctx.lr = 0x82488D18;
	sub_826536B0(ctx, base);
	// 82488D18: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82488D1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82488D20: C03D0008  lfs f1, 8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82488D24: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82488D28: 481CA8F1  bl 0x82653618
	ctx.lr = 0x82488D2C;
	sub_82653618(ctx, base);
	// 82488D2C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82488D30: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82488D34: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82488D38: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82488D3C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82488D40: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82488D44: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488D90 size=160
    let mut pc: u32 = 0x82488D90;
    'dispatch: loop {
        match pc {
            0x82488D90 => {
    //   block [0x82488D90..0x82488E30)
	// 82488D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488D98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82488D9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488DA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488DA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488DA8: C0240008  lfs f1, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82488DAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82488DB0: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82488DB4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82488DB8: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82488DBC: 481CA85D  bl 0x82653618
	ctx.lr = 0x82488DC0;
	sub_82653618(ctx, base);
	// 82488DC0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82488DC4: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82488DC8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82488DCC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82488DD0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82488DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82488DD8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82488DDC: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488E30 size=256
    let mut pc: u32 = 0x82488E30;
    'dispatch: loop {
        match pc {
            0x82488E30 => {
    //   block [0x82488E30..0x82488F30)
	// 82488E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488E38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82488E3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488E44: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82488E48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488E4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82488E50: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82488E54: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82488E58: 396ABA7C  addi r11, r10, -0x4584
	ctx.r[11].s64 = ctx.r[10].s64 + -17796;
	// 82488E5C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82488E60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82488E64: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82488E68: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82488E6C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488F30 size=108
    let mut pc: u32 = 0x82488F30;
    'dispatch: loop {
        match pc {
            0x82488F30 => {
    //   block [0x82488F30..0x82488F9C)
	// 82488F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488F34: 48D1F231  bl 0x831a8164
	ctx.lr = 0x82488F38;
	sub_831A8130(ctx, base);
	// 82488F38: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488F3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488F40: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82488F44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82488F48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82488F4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82488F50: 839F0038  lwz r28, 0x38(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82488F54: 3B7F0020  addi r27, r31, 0x20
	ctx.r[27].s64 = ctx.r[31].s64 + 32;
	// 82488F58: 4BFDBE41  bl 0x82464d98
	ctx.lr = 0x82488F5C;
	sub_82464D98(ctx, base);
	// 82488F5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82488F60: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82488F64: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82488F68: 481CA749  bl 0x826536b0
	ctx.lr = 0x82488F6C;
	sub_826536B0(ctx, base);
	// 82488F6C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82488F70: 4BE46FA1  bl 0x822cff10
	ctx.lr = 0x82488F74;
	sub_822CFF10(ctx, base);
	// 82488F74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82488F78: D03F0044  stfs f1, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82488F7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82488F80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82488F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82488F88: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82488F8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82488F90: 4E800421  bctrl
	ctx.lr = 0x82488F94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82488F94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82488F98: 48D1F21C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82488FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82488FA0 size=276
    let mut pc: u32 = 0x82488FA0;
    'dispatch: loop {
        match pc {
            0x82488FA0 => {
    //   block [0x82488FA0..0x824890B4)
	// 82488FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82488FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82488FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82488FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82488FB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82488FB4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82488FB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82488FBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82488FC0: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82488FC4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82488FC8: 396ABA94  addi r11, r10, -0x456c
	ctx.r[11].s64 = ctx.r[10].s64 + -17772;
	// 82488FCC: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82488FD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82488FD4: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82488FD8: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82488FDC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824890B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824890B8 size=84
    let mut pc: u32 = 0x824890B8;
    'dispatch: loop {
        match pc {
            0x824890B8 => {
    //   block [0x824890B8..0x8248910C)
	// 824890B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824890BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824890C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824890C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824890C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824890CC: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 824890D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824890D4: 419A0008  beq cr6, 0x824890dc
	if ctx.cr[6].eq {
	pc = 0x824890DC; continue 'dispatch;
	}
	// 824890D8: 4BE377B9  bl 0x822c0890
	ctx.lr = 0x824890DC;
	sub_822C0890(ctx, base);
	// 824890DC: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 824890E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824890E4: 419A0008  beq cr6, 0x824890ec
	if ctx.cr[6].eq {
	pc = 0x824890EC; continue 'dispatch;
	}
	// 824890E8: 4BE377A9  bl 0x822c0890
	ctx.lr = 0x824890EC;
	sub_822C0890(ctx, base);
	// 824890EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824890F0: 396BB83C  addi r11, r11, -0x47c4
	ctx.r[11].s64 = ctx.r[11].s64 + -18372;
	// 824890F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824890F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824890FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489110 size=76
    let mut pc: u32 = 0x82489110;
    'dispatch: loop {
        match pc {
            0x82489110 => {
    //   block [0x82489110..0x8248915C)
	// 82489110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248911C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489124: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489128: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248912C: 4BFFFF8D  bl 0x824890b8
	ctx.lr = 0x82489130;
	sub_824890B8(ctx, base);
	// 82489130: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82489134: 4182000C  beq 0x82489140
	if ctx.cr[0].eq {
	pc = 0x82489140; continue 'dispatch;
	}
	// 82489138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248913C: 4BE3712D  bl 0x822c0268
	ctx.lr = 0x82489140;
	sub_822C0268(ctx, base);
	// 82489140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82489144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82489148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248914C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82489154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82489160 size=348
    let mut pc: u32 = 0x82489160;
    'dispatch: loop {
        match pc {
            0x82489160 => {
    //   block [0x82489160..0x824892BC)
	// 82489160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489164: 48D1F001  bl 0x831a8164
	ctx.lr = 0x82489168;
	sub_831A8130(ctx, base);
	// 82489168: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248916C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489170: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82489174: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82489178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248917C: 419A0118  beq cr6, 0x82489294
	if ctx.cr[6].eq {
	pc = 0x82489294; continue 'dispatch;
	}
	// 82489180: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82489184: C0240008  lfs f1, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82489188: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8248918C: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82489190: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82489194: 481CA485  bl 0x82653618
	ctx.lr = 0x82489198;
	sub_82653618(ctx, base);
	// 82489198: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8248919C: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824891A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824891A4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 824891A8: 3BABBAA8  addi r29, r11, -0x4558
	ctx.r[29].s64 = ctx.r[11].s64 + -17752;
	// 824891AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824891B0: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824891B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824892C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824892C0 size=128
    let mut pc: u32 = 0x824892C0;
    'dispatch: loop {
        match pc {
            0x824892C0 => {
    //   block [0x824892C0..0x82489340)
	// 824892C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824892C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824892C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824892CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824892D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824892D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824892D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824892DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824892E0: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824892E4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824892E8: 4BE46C49  bl 0x822cff30
	ctx.lr = 0x824892EC;
	sub_822CFF30(ctx, base);
	// 824892EC: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 824892F0: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824892F4: 4BE46DF5  bl 0x822d00e8
	ctx.lr = 0x824892F8;
	sub_822D00E8(ctx, base);
	// 824892F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824892FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82489300: 480861C9  bl 0x8250f4c8
	ctx.lr = 0x82489304;
	sub_8250F4C8(ctx, base);
	// 82489304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248930C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82489310: 409A0008  bne cr6, 0x82489318
	if !ctx.cr[6].eq {
	pc = 0x82489318; continue 'dispatch;
	}
	// 82489314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82489318: 4807F201  bl 0x82508518
	ctx.lr = 0x8248931C;
	sub_82508518(ctx, base);
	// 8248931C: D03F002C  stfs f1, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82489320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82489324: 4896896D  bl 0x82df1c90
	ctx.lr = 0x82489328;
	sub_82DF1C90(ctx, base);
	// 82489328: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248932C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489334: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82489338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248933C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82489340 size=136
    let mut pc: u32 = 0x82489340;
    'dispatch: loop {
        match pc {
            0x82489340 => {
    //   block [0x82489340..0x824893C8)
	// 82489340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248934C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82489350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489358: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8248935C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82489360: 48086169  bl 0x8250f4c8
	ctx.lr = 0x82489364;
	sub_8250F4C8(ctx, base);
	// 82489364: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489368: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248936C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82489370: 409A0008  bne cr6, 0x82489378
	if !ctx.cr[6].eq {
	pc = 0x82489378; continue 'dispatch;
	}
	// 82489374: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82489378: 4807F1A1  bl 0x82508518
	ctx.lr = 0x8248937C;
	sub_82508518(ctx, base);
	// 8248937C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82489380: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82489384: 4896890D  bl 0x82df1c90
	ctx.lr = 0x82489388;
	sub_82DF1C90(ctx, base);
	// 82489388: C01F002C  lfs f0, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248938C: D3FF002C  stfs f31, 0x2c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82489390: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82489394: C1BF0028  lfs f13, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82489398: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8248939C: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824893A0: 4BE46B61  bl 0x822cff00
	ctx.lr = 0x824893A4;
	sub_822CFF00(ctx, base);
	// 824893A4: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 824893A8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824893AC: 4BE46D3D  bl 0x822d00e8
	ctx.lr = 0x824893B0;
	sub_822D00E8(ctx, base);
	// 824893B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824893B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824893B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824893BC: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824893C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824893C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824893C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824893C8 size=172
    let mut pc: u32 = 0x824893C8;
    'dispatch: loop {
        match pc {
            0x824893C8 => {
    //   block [0x824893C8..0x82489474)
	// 824893C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824893CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824893D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824893D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824893D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 824893DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824893E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 824893E4: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 824893E8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824893EC: 394ABB00  addi r10, r10, -0x4500
	ctx.r[10].s64 = ctx.r[10].s64 + -17664;
	// 824893F0: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 824893F4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824893F8: 39440004  addi r10, r4, 4
	ctx.r[10].s64 = ctx.r[4].s64 + 4;
	// 824893FC: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82489478 size=96
    let mut pc: u32 = 0x82489478;
    'dispatch: loop {
        match pc {
            0x82489478 => {
    //   block [0x82489478..0x824894D8)
	// 82489478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248947C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82489484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248948C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82489490: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82489494: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82489498: 4BE46C19  bl 0x822d00b0
	ctx.lr = 0x8248949C;
	sub_822D00B0(ctx, base);
	// 8248949C: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 824894A0: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 824894A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824894A8: 4BE46C41  bl 0x822d00e8
	ctx.lr = 0x824894AC;
	sub_822D00E8(ctx, base);
	// 824894AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824894B0: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 824894B4: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824894D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824894D8 size=152
    let mut pc: u32 = 0x824894D8;
    'dispatch: loop {
        match pc {
            0x824894D8 => {
    //   block [0x824894D8..0x82489570)
	// 824894D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824894DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824894E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824894E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824894E8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 824894EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824894F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 824894F4: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 824894F8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824894FC: 394ABB18  addi r10, r10, -0x44e8
	ctx.r[10].s64 = ctx.r[10].s64 + -17640;
	// 82489500: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82489504: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82489508: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489570 size=100
    let mut pc: u32 = 0x82489570;
    'dispatch: loop {
        match pc {
            0x82489570 => {
    //   block [0x82489570..0x824895D4)
	// 82489570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248957C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248958C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82489590: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82489594: 419A0008  beq cr6, 0x8248959c
	if ctx.cr[6].eq {
	pc = 0x8248959C; continue 'dispatch;
	}
	// 82489598: 4BE372F9  bl 0x822c0890
	ctx.lr = 0x8248959C;
	sub_822C0890(ctx, base);
	// 8248959C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 824895A0: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824895A4: 396BB83C  addi r11, r11, -0x47c4
	ctx.r[11].s64 = ctx.r[11].s64 + -18372;
	// 824895A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824895AC: 4182000C  beq 0x824895b8
	if ctx.cr[0].eq {
	pc = 0x824895B8; continue 'dispatch;
	}
	// 824895B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824895B4: 4BE36CB5  bl 0x822c0268
	ctx.lr = 0x824895B8;
	sub_822C0268(ctx, base);
	// 824895B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824895BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824895C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824895C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824895C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824895CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824895D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824895D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824895D8 size=96
    let mut pc: u32 = 0x824895D8;
    'dispatch: loop {
        match pc {
            0x824895D8 => {
    //   block [0x824895D8..0x82489638)
	// 824895D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824895DC: 48D1EB89  bl 0x831a8164
	ctx.lr = 0x824895E0;
	sub_831A8130(ctx, base);
	// 824895E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824895E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824895E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824895EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824895F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824895F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824895F8: 839F0030  lwz r28, 0x30(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824895FC: 3B7F0020  addi r27, r31, 0x20
	ctx.r[27].s64 = ctx.r[31].s64 + 32;
	// 82489600: 4BFDB799  bl 0x82464d98
	ctx.lr = 0x82489604;
	sub_82464D98(ctx, base);
	// 82489604: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82489608: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8248960C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82489610: 481CA0A1  bl 0x826536b0
	ctx.lr = 0x82489614;
	sub_826536B0(ctx, base);
	// 82489614: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489618: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8248961C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82489620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82489624: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82489628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248962C: 4E800421  bctrl
	ctx.lr = 0x82489630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489630: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82489634: 48D1EB80  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82489638 size=136
    let mut pc: u32 = 0x82489638;
    'dispatch: loop {
        match pc {
            0x82489638 => {
    //   block [0x82489638..0x824896C0)
	// 82489638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248963C: 48D1EB2D  bl 0x831a8168
	ctx.lr = 0x82489640;
	sub_831A8130(ctx, base);
	// 82489640: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489648: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8248964C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82489650: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82489654: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82489658: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8248965C: C03D0008  lfs f1, 8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82489660: 481C9FB9  bl 0x82653618
	ctx.lr = 0x82489664;
	sub_82653618(ctx, base);
	// 82489664: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82489668: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8248966C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82489670: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489674: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 82489678: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248967C: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824896C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824896C0 size=164
    let mut pc: u32 = 0x824896C0;
    'dispatch: loop {
        match pc {
            0x824896C0 => {
    //   block [0x824896C0..0x82489764)
	// 824896C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824896C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824896C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824896CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824896D0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 824896D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824896D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 824896DC: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 824896E0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 824896E4: 396ABB30  addi r11, r10, -0x44d0
	ctx.r[11].s64 = ctx.r[10].s64 + -17616;
	// 824896E8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824896EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824896F0: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 824896F4: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489768 size=80
    let mut pc: u32 = 0x82489768;
    'dispatch: loop {
        match pc {
            0x82489768 => {
    //   block [0x82489768..0x824897B8)
	// 82489768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248976C: 48D1EA01  bl 0x831a816c
	ctx.lr = 0x82489770;
	sub_831A8130(ctx, base);
	// 82489770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248977C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82489780: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82489784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489788: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248978C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489790: 4E800421  bctrl
	ctx.lr = 0x82489794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489794: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82489798: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8248979C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824897A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824897A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824897A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824897AC: 4E800421  bctrl
	ctx.lr = 0x824897B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824897B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824897B4: 48D1EA08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824897B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824897B8 size=352
    let mut pc: u32 = 0x824897B8;
    'dispatch: loop {
        match pc {
            0x824897B8 => {
    //   block [0x824897B8..0x82489918)
	// 824897B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824897BC: 48D1E9AD  bl 0x831a8168
	ctx.lr = 0x824897C0;
	sub_831A8130(ctx, base);
	// 824897C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824897C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824897C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824897CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824897D0: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824897D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824897D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824897DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824897E0: 4E800421  bctrl
	ctx.lr = 0x824897E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824897E4: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824897E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824897EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824897F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824897F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824897F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824897FC: 4E800421  bctrl
	ctx.lr = 0x82489800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489800: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82489808: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248980C: 83BF0020  lwz r29, 0x20(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82489810: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489814: 4E800421  bctrl
	ctx.lr = 0x82489818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489818: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248981C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82489820: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82489824: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82489828: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248982C: 4E800421  bctrl
	ctx.lr = 0x82489830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489830: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82489834: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82489838: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248983C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82489840: 83BF0028  lwz r29, 0x28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82489844: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82489848: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82489918 size=432
    let mut pc: u32 = 0x82489918;
    'dispatch: loop {
        match pc {
            0x82489918 => {
    //   block [0x82489918..0x82489AC8)
	// 82489918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248991C: 48D1E84D  bl 0x831a8168
	ctx.lr = 0x82489920;
	sub_831A8130(ctx, base);
	// 82489920: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248992C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82489930: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82489934: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489938: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248993C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489940: 4E800421  bctrl
	ctx.lr = 0x82489944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489944: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82489948: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248994C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82489950: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489954: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82489958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248995C: 4E800421  bctrl
	ctx.lr = 0x82489960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489960: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489964: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82489968: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248996C: 83BF0020  lwz r29, 0x20(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82489970: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489974: 4E800421  bctrl
	ctx.lr = 0x82489978;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489978: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248997C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82489980: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82489984: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82489988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248998C: 4E800421  bctrl
	ctx.lr = 0x82489990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489990: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82489994: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82489998: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248999C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 824899A0: 83BF0028  lwz r29, 0x28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824899A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824899A8: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489AC8 size=196
    let mut pc: u32 = 0x82489AC8;
    'dispatch: loop {
        match pc {
            0x82489AC8 => {
    //   block [0x82489AC8..0x82489B8C)
	// 82489AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82489AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489ADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82489AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82489AE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82489AE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82489AEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489AF0: 4BE36E49  bl 0x822c0938
	ctx.lr = 0x82489AF4;
	sub_822C0938(ctx, base);
	// 82489AF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82489AF8: 41820028  beq 0x82489b20
	if ctx.cr[0].eq {
	pc = 0x82489B20; continue 'dispatch;
	}
	// 82489AFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82489B00: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82489B04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82489B08: 392BBB48  addi r9, r11, -0x44b8
	ctx.r[9].s64 = ctx.r[11].s64 + -17592;
	// 82489B0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82489B10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82489B14: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82489B18: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82489B1C: 48000008  b 0x82489b24
	pc = 0x82489B24; continue 'dispatch;
	// 82489B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82489B24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82489B2C: 409A0044  bne cr6, 0x82489b70
	if !ctx.cr[6].eq {
	pc = 0x82489B70; continue 'dispatch;
	}
	// 82489B30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82489B34: 419A001C  beq cr6, 0x82489b50
	if ctx.cr[6].eq {
	pc = 0x82489B50; continue 'dispatch;
	}
	// 82489B38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489B3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82489B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82489B44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489B4C: 4E800421  bctrl
	ctx.lr = 0x82489B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489B50: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82489B54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82489B58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82489B5C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82489B60: 816B7810  lwz r11, 0x7810(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30736 as u32) ) } as u64;
	// 82489B64: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82489B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82489B6C: 4BE36495  bl 0x822c0000
	ctx.lr = 0x82489B70;
	sub_822C0000(ctx, base);
	// 82489B70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82489B74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82489B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489B80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82489B84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489B90 size=196
    let mut pc: u32 = 0x82489B90;
    'dispatch: loop {
        match pc {
            0x82489B90 => {
    //   block [0x82489B90..0x82489C54)
	// 82489B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489B98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82489B9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489BA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489BA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82489BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82489BAC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82489BB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82489BB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489BB8: 4BE36D81  bl 0x822c0938
	ctx.lr = 0x82489BBC;
	sub_822C0938(ctx, base);
	// 82489BBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82489BC0: 41820028  beq 0x82489be8
	if ctx.cr[0].eq {
	pc = 0x82489BE8; continue 'dispatch;
	}
	// 82489BC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82489BC8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82489BCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82489BD0: 392BBB5C  addi r9, r11, -0x44a4
	ctx.r[9].s64 = ctx.r[11].s64 + -17572;
	// 82489BD4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82489BD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82489BDC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82489BE0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82489BE4: 48000008  b 0x82489bec
	pc = 0x82489BEC; continue 'dispatch;
	// 82489BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82489BEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489BF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82489BF4: 409A0044  bne cr6, 0x82489c38
	if !ctx.cr[6].eq {
	pc = 0x82489C38; continue 'dispatch;
	}
	// 82489BF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82489BFC: 419A001C  beq cr6, 0x82489c18
	if ctx.cr[6].eq {
	pc = 0x82489C18; continue 'dispatch;
	}
	// 82489C00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489C04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82489C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82489C0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489C10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489C14: 4E800421  bctrl
	ctx.lr = 0x82489C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489C18: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82489C1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82489C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82489C24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82489C28: 816B7810  lwz r11, 0x7810(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30736 as u32) ) } as u64;
	// 82489C2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82489C30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82489C34: 4BE363CD  bl 0x822c0000
	ctx.lr = 0x82489C38;
	sub_822C0000(ctx, base);
	// 82489C38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82489C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82489C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489C48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82489C4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489C58 size=112
    let mut pc: u32 = 0x82489C58;
    'dispatch: loop {
        match pc {
            0x82489C58 => {
    //   block [0x82489C58..0x82489CC8)
	// 82489C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489C60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82489C64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489C68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489C6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82489C70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489C74: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82489C78: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82489C7C: 4BFFFE4D  bl 0x82489ac8
	ctx.lr = 0x82489C80;
	sub_82489AC8(ctx, base);
	// 82489C80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82489C84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82489C88: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82489C8C: 4BE36375  bl 0x822c0000
	ctx.lr = 0x82489C90;
	sub_822C0000(ctx, base);
	// 82489C90: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82489C94: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82489C98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489C9C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82489CA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82489CA4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82489CA8: 419A0008  beq cr6, 0x82489cb0
	if ctx.cr[6].eq {
	pc = 0x82489CB0; continue 'dispatch;
	}
	// 82489CAC: 4BE36BE5  bl 0x822c0890
	ctx.lr = 0x82489CB0;
	sub_822C0890(ctx, base);
	// 82489CB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82489CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489CBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82489CC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489CC8 size=112
    let mut pc: u32 = 0x82489CC8;
    'dispatch: loop {
        match pc {
            0x82489CC8 => {
    //   block [0x82489CC8..0x82489D38)
	// 82489CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489CD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82489CD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489CDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82489CE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489CE4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82489CE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82489CEC: 4BFFFEA5  bl 0x82489b90
	ctx.lr = 0x82489CF0;
	sub_82489B90(ctx, base);
	// 82489CF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82489CF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82489CF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82489CFC: 4BE36305  bl 0x822c0000
	ctx.lr = 0x82489D00;
	sub_822C0000(ctx, base);
	// 82489D00: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82489D04: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82489D08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489D0C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82489D10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82489D14: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82489D18: 419A0008  beq cr6, 0x82489d20
	if ctx.cr[6].eq {
	pc = 0x82489D20; continue 'dispatch;
	}
	// 82489D1C: 4BE36B75  bl 0x822c0890
	ctx.lr = 0x82489D20;
	sub_822C0890(ctx, base);
	// 82489D20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82489D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489D2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82489D30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82489D38 size=364
    let mut pc: u32 = 0x82489D38;
    'dispatch: loop {
        match pc {
            0x82489D38 => {
    //   block [0x82489D38..0x82489EA4)
	// 82489D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489D3C: 48D1E421  bl 0x831a815c
	ctx.lr = 0x82489D40;
	sub_831A8130(ctx, base);
	// 82489D40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489D44: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82489D48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82489D4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82489D50: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82489D54: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82489D58: 396ABBD0  addi r11, r10, -0x4430
	ctx.r[11].s64 = ctx.r[10].s64 + -17456;
	// 82489D5C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82489D60: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489D64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82489D68: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82489D6C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489EA8 size=84
    let mut pc: u32 = 0x82489EA8;
    'dispatch: loop {
        match pc {
            0x82489EA8 => {
    //   block [0x82489EA8..0x82489EFC)
	// 82489EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489EB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489EB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489EB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489EBC: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82489EC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82489EC4: 419A0008  beq cr6, 0x82489ecc
	if ctx.cr[6].eq {
	pc = 0x82489ECC; continue 'dispatch;
	}
	// 82489EC8: 4BE369C9  bl 0x822c0890
	ctx.lr = 0x82489ECC;
	sub_822C0890(ctx, base);
	// 82489ECC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82489ED0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82489ED4: 419A0008  beq cr6, 0x82489edc
	if ctx.cr[6].eq {
	pc = 0x82489EDC; continue 'dispatch;
	}
	// 82489ED8: 4BE369B9  bl 0x822c0890
	ctx.lr = 0x82489EDC;
	sub_822C0890(ctx, base);
	// 82489EDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82489EE0: 396BB83C  addi r11, r11, -0x47c4
	ctx.r[11].s64 = ctx.r[11].s64 + -18372;
	// 82489EE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82489EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82489EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489EF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82489F00 size=76
    let mut pc: u32 = 0x82489F00;
    'dispatch: loop {
        match pc {
            0x82489F00 => {
    //   block [0x82489F00..0x82489F4C)
	// 82489F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82489F08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82489F0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82489F10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82489F18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82489F1C: 4BFFFF8D  bl 0x82489ea8
	ctx.lr = 0x82489F20;
	sub_82489EA8(ctx, base);
	// 82489F20: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82489F24: 4182000C  beq 0x82489f30
	if ctx.cr[0].eq {
	pc = 0x82489F30; continue 'dispatch;
	}
	// 82489F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82489F2C: 4BE3633D  bl 0x822c0268
	ctx.lr = 0x82489F30;
	sub_822C0268(ctx, base);
	// 82489F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82489F34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82489F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82489F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82489F40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82489F44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82489F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82489F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82489F50 size=328
    let mut pc: u32 = 0x82489F50;
    'dispatch: loop {
        match pc {
            0x82489F50 => {
    //   block [0x82489F50..0x8248A098)
	// 82489F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82489F54: 48D1E219  bl 0x831a816c
	ctx.lr = 0x82489F58;
	sub_831A8130(ctx, base);
	// 82489F58: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82489F5C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82489F60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82489F64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82489F68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82489F6C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82489F70: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82489F74: 83FE0028  lwz r31, 0x28(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82489F78: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82489F7C: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82489F80: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82489F84: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82489F88: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82489F8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489F90: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82489F94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489F98: 4E800421  bctrl
	ctx.lr = 0x82489F9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489F9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82489FA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82489FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82489FA8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82489FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82489FB0: 4E800421  bctrl
	ctx.lr = 0x82489FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82489FB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82489FB8: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82489FBC: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82489FC0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82489FC4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82489FC8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82489FCC: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8248A098 size=256
    let mut pc: u32 = 0x8248A098;
    'dispatch: loop {
        match pc {
            0x8248A098 => {
    //   block [0x8248A098..0x8248A198)
	// 8248A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A0A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248A0A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A0AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A0B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248A0B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248A0B8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8248A0BC: 390BBC00  addi r8, r11, -0x4400
	ctx.r[8].s64 = ctx.r[11].s64 + -17408;
	// 8248A0C0: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 8248A0C4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8248A0C8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8248A0CC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A0D0: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248A0D4: C1A908A8  lfs f13, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8248A0D8: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8248A0DC: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A0E0: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8248A0E4: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8248A0E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248A0EC: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8248A0F0: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8248A0F4: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8248A0F8: 419A0024  beq cr6, 0x8248a11c
	if ctx.cr[6].eq {
	pc = 0x8248A11C; continue 'dispatch;
	}
	// 8248A0FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248A100: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248A104: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248A108: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8248A10C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8248A110: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8248A114: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248A118: 4082FFE8  bne 0x8248a100
	if !ctx.cr[0].eq {
	pc = 0x8248A100; continue 'dispatch;
	}
	// 8248A11C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A120: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 8248A124: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8248A128: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A12C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248A130: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8248A134: 419A0024  beq cr6, 0x8248a158
	if ctx.cr[6].eq {
	pc = 0x8248A158; continue 'dispatch;
	}
	// 8248A138: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248A13C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248A140: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248A144: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8248A148: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8248A14C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8248A150: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248A154: 4082FFE8  bne 0x8248a13c
	if !ctx.cr[0].eq {
	pc = 0x8248A13C; continue 'dispatch;
	}
	// 8248A158: D03F0030  stfs f1, 0x30(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8248A15C: 80640004  lwz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A160: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248A164: 419A0008  beq cr6, 0x8248a16c
	if ctx.cr[6].eq {
	pc = 0x8248A16C; continue 'dispatch;
	}
	// 8248A168: 4BE36729  bl 0x822c0890
	ctx.lr = 0x8248A16C;
	sub_822C0890(ctx, base);
	// 8248A16C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A170: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248A174: 419A0008  beq cr6, 0x8248a17c
	if ctx.cr[6].eq {
	pc = 0x8248A17C; continue 'dispatch;
	}
	// 8248A178: 4BE36719  bl 0x822c0890
	ctx.lr = 0x8248A17C;
	sub_822C0890(ctx, base);
	// 8248A17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248A180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248A184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A18C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248A190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A198 size=84
    let mut pc: u32 = 0x8248A198;
    'dispatch: loop {
        match pc {
            0x8248A198 => {
    //   block [0x8248A198..0x8248A1EC)
	// 8248A198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A1A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A1A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A1A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A1AC: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8248A1B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248A1B4: 419A0008  beq cr6, 0x8248a1bc
	if ctx.cr[6].eq {
	pc = 0x8248A1BC; continue 'dispatch;
	}
	// 8248A1B8: 4BE366D9  bl 0x822c0890
	ctx.lr = 0x8248A1BC;
	sub_822C0890(ctx, base);
	// 8248A1BC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248A1C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248A1C4: 419A0008  beq cr6, 0x8248a1cc
	if ctx.cr[6].eq {
	pc = 0x8248A1CC; continue 'dispatch;
	}
	// 8248A1C8: 4BE366C9  bl 0x822c0890
	ctx.lr = 0x8248A1CC;
	sub_822C0890(ctx, base);
	// 8248A1CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248A1D0: 396BBBE8  addi r11, r11, -0x4418
	ctx.r[11].s64 = ctx.r[11].s64 + -17432;
	// 8248A1D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248A1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A1E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A1F0 size=76
    let mut pc: u32 = 0x8248A1F0;
    'dispatch: loop {
        match pc {
            0x8248A1F0 => {
    //   block [0x8248A1F0..0x8248A23C)
	// 8248A1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A1F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248A1FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A200: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A208: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248A20C: 4BFFFF8D  bl 0x8248a198
	ctx.lr = 0x8248A210;
	sub_8248A198(ctx, base);
	// 8248A210: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A214: 4182000C  beq 0x8248a220
	if ctx.cr[0].eq {
	pc = 0x8248A220; continue 'dispatch;
	}
	// 8248A218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248A21C: 4BE3604D  bl 0x822c0268
	ctx.lr = 0x8248A220;
	sub_822C0268(ctx, base);
	// 8248A220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248A224: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248A228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A22C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A230: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248A234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8248A240 size=64
    let mut pc: u32 = 0x8248A240;
    'dispatch: loop {
        match pc {
            0x8248A240 => {
    //   block [0x8248A240..0x8248A280)
	// 8248A240: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8248A244: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248A248: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 8248A24C: 3929BC18  addi r9, r9, -0x43e8
	ctx.r[9].s64 = ctx.r[9].s64 + -17384;
	// 8248A250: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248A254: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8248A258: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248A25C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8248A260: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8248A264: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8248A268: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8248A26C: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8248A270: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8248A274: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8248A278: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8248A27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A280 size=68
    let mut pc: u32 = 0x8248A280;
    'dispatch: loop {
        match pc {
            0x8248A280 => {
    //   block [0x8248A280..0x8248A2C4)
	// 8248A280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A28C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A294: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248A298: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248A29C: 396BBBE8  addi r11, r11, -0x4418
	ctx.r[11].s64 = ctx.r[11].s64 + -17432;
	// 8248A2A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248A2A4: 41820008  beq 0x8248a2ac
	if ctx.cr[0].eq {
	pc = 0x8248A2AC; continue 'dispatch;
	}
	// 8248A2A8: 4BE35FC1  bl 0x822c0268
	ctx.lr = 0x8248A2AC;
	sub_822C0268(ctx, base);
	// 8248A2AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248A2B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A2BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8248A2C8 size=196
    let mut pc: u32 = 0x8248A2C8;
    'dispatch: loop {
        match pc {
            0x8248A2C8 => {
    //   block [0x8248A2C8..0x8248A38C)
	// 8248A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A2CC: 48D1DEA1  bl 0x831a816c
	ctx.lr = 0x8248A2D0;
	sub_831A8130(ctx, base);
	// 8248A2D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A2D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248A2D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A2DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248A2E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A2E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248A2EC: 4E800421  bctrl
	ctx.lr = 0x8248A2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248A2F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A2F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248A2F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248A2FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248A300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248A304: 4E800421  bctrl
	ctx.lr = 0x8248A308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248A308: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8248A30C: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8248A310: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8248A390 size=60
    let mut pc: u32 = 0x8248A390;
    'dispatch: loop {
        match pc {
            0x8248A390 => {
    //   block [0x8248A390..0x8248A3CC)
	// 8248A390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A39C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8248A3A0: C00BDFAC  lfs f0, -0x2054(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248A3A4: EC200072  fmuls f1, f0, f1
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8248A3A8: 48D1EB01  bl 0x831a8ea8
	ctx.lr = 0x8248A3AC;
	sub_831A8EA8(ctx, base);
	// 8248A3AC: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8248A3B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248A3B4: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8248A3B8: EC2D003C  fnmsubs f1, f13, f0, f0
	ctx.f[1].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[0].f64) as f32) as f64);
	// 8248A3BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A3D0 size=16
    let mut pc: u32 = 0x8248A3D0;
    'dispatch: loop {
        match pc {
            0x8248A3D0 => {
    //   block [0x8248A3D0..0x8248A3E0)
	// 8248A3D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A3D4: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248A3D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248A3DC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A3E0 size=24
    let mut pc: u32 = 0x8248A3E0;
    'dispatch: loop {
        match pc {
            0x8248A3E0 => {
    //   block [0x8248A3E0..0x8248A3F8)
	// 8248A3E0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A3E4: 892A0025  lbz r9, 0x25(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248A3E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248A3EC: 409A0040  bne cr6, 0x8248a42c
	if !ctx.cr[6].eq {
		sub_8248A414(ctx, base);
		return;
	}
	// 8248A3F0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A3F4: 4800000C  b 0x8248a400
	sub_8248A3F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A3F8 size=28
    let mut pc: u32 = 0x8248A3F8;
    'dispatch: loop {
        match pc {
            0x8248A3F8 => {
    //   block [0x8248A3F8..0x8248A414)
	// 8248A3F8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8248A3FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A400: 892B0025  lbz r9, 0x25(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248A404: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248A408: 419AFFF0  beq cr6, 0x8248a3f8
	if ctx.cr[6].eq {
	pc = 0x8248A3F8; continue 'dispatch;
	}
	// 8248A40C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248A410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A414 size=48
    let mut pc: u32 = 0x8248A414;
    'dispatch: loop {
        match pc {
            0x8248A414 => {
    //   block [0x8248A414..0x8248A444)
	// 8248A414: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A418: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A41C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248A420: 409A001C  bne cr6, 0x8248a43c
	if !ctx.cr[6].eq {
	pc = 0x8248A43C; continue 'dispatch;
	}
	// 8248A424: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248A428: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8248A42C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A430: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248A434: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248A438: 419AFFDC  beq cr6, 0x8248a414
	if ctx.cr[6].eq {
	pc = 0x8248A414; continue 'dispatch;
	}
	// 8248A43C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248A440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A448 size=64
    let mut pc: u32 = 0x8248A448;
    'dispatch: loop {
        match pc {
            0x8248A448 => {
    //   block [0x8248A448..0x8248A488)
	// 8248A448: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A44C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A450: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248A454: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A458: 892A0025  lbz r9, 0x25(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248A45C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248A460: 409A0008  bne cr6, 0x8248a468
	if !ctx.cr[6].eq {
	pc = 0x8248A468; continue 'dispatch;
	}
	// 8248A464: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8248A468: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A46C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8248A470: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A474: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A478: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248A47C: 409A000C  bne cr6, 0x8248a488
	if !ctx.cr[6].eq {
		sub_8248A488(ctx, base);
		return;
	}
	// 8248A480: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8248A484: 48000020  b 0x8248a4a4
	sub_8248A4A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A488 size=24
    let mut pc: u32 = 0x8248A488;
    'dispatch: loop {
        match pc {
            0x8248A488 => {
    //   block [0x8248A488..0x8248A4A0)
	// 8248A488: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A48C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A490: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248A494: 409A000C  bne cr6, 0x8248a4a0
	if !ctx.cr[6].eq {
		sub_8248A4A0(ctx, base);
		return;
	}
	// 8248A498: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248A49C: 48000008  b 0x8248a4a4
	sub_8248A4A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A4A0 size=16
    let mut pc: u32 = 0x8248A4A0;
    'dispatch: loop {
        match pc {
            0x8248A4A0 => {
    //   block [0x8248A4A0..0x8248A4B0)
	// 8248A4A0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A4A4: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8248A4A8: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8248A4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A4B0 size=120
    let mut pc: u32 = 0x8248A4B0;
    'dispatch: loop {
        match pc {
            0x8248A4B0 => {
    //   block [0x8248A4B0..0x8248A528)
	// 8248A4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A4B4: 48D1DCAD  bl 0x831a8160
	ctx.lr = 0x8248A4B8;
	sub_831A8130(ctx, base);
	// 8248A4B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A4BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248A4C0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248A4C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248A4C8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8248A4CC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8248A4D0: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8248A4D4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248A4D8: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 8248A4DC: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8248A4E0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8248A4E4: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8248A4E8: 48967BE1  bl 0x82df20c8
	ctx.lr = 0x8248A4EC;
	sub_82DF20C8(ctx, base);
	// 8248A4EC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8248A4F0: 4182002C  beq 0x8248a51c
	if ctx.cr[0].eq {
	pc = 0x8248A51C; continue 'dispatch;
	}
	// 8248A4F4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8248A4F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8248A4FC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8248A500: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8248A504: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8248A508: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8248A50C: 48D1E005  bl 0x831a8510
	ctx.lr = 0x8248A510;
	sub_831A8510(ctx, base);
	// 8248A510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248A514: 9B5F0024  stb r26, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u8 ) };
	// 8248A518: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 8248A51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248A520: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8248A524: 48D1DC8C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A528 size=84
    let mut pc: u32 = 0x8248A528;
    'dispatch: loop {
        match pc {
            0x8248A528 => {
    //   block [0x8248A528..0x8248A57C)
	// 8248A528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A52C: 48D1DC41  bl 0x831a816c
	ctx.lr = 0x8248A530;
	sub_831A8130(ctx, base);
	// 8248A530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A534: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248A538: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248A53C: 897F0025  lbz r11, 0x25(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248A540: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248A544: 409A0030  bne cr6, 0x8248a574
	if !ctx.cr[6].eq {
	pc = 0x8248A574; continue 'dispatch;
	}
	// 8248A548: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 8248A54C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248A550: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A554: 4BFFFFD5  bl 0x8248a528
	ctx.lr = 0x8248A558;
	sub_8248A528(ctx, base);
	// 8248A558: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248A55C: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248A560: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A564: 48967C25  bl 0x82df2188
	ctx.lr = 0x8248A568;
	sub_82DF2188(ctx, base);
	// 8248A568: 897F0025  lbz r11, 0x25(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248A56C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248A570: 419AFFDC  beq cr6, 0x8248a54c
	if ctx.cr[6].eq {
	pc = 0x8248A54C; continue 'dispatch;
	}
	// 8248A574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248A578: 48D1DC44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A580 size=108
    let mut pc: u32 = 0x8248A580;
    'dispatch: loop {
        match pc {
            0x8248A580 => {
    //   block [0x8248A580..0x8248A5EC)
	// 8248A580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A58C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248A590: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248A594: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8248A598: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 8248A59C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8248A5A0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248A5A4: 48967B25  bl 0x82df20c8
	ctx.lr = 0x8248A5A8;
	sub_82DF20C8(ctx, base);
	// 8248A5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8248A5AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248A5B0: 41820008  beq 0x8248a5b8
	if ctx.cr[0].eq {
	pc = 0x8248A5B8; continue 'dispatch;
	}
	// 8248A5B4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248A5B8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A5BC: 41820008  beq 0x8248a5c4
	if ctx.cr[0].eq {
	pc = 0x8248A5C4; continue 'dispatch;
	}
	// 8248A5C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248A5C4: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A5C8: 41820008  beq 0x8248a5d0
	if ctx.cr[0].eq {
	pc = 0x8248A5D0; continue 'dispatch;
	}
	// 8248A5CC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248A5D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248A5D4: 99430025  stb r10, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 8248A5D8: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 8248A5DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248A5F0 size=120
    let mut pc: u32 = 0x8248A5F0;
    'dispatch: loop {
        match pc {
            0x8248A5F0 => {
    //   block [0x8248A5F0..0x8248A668)
	// 8248A5F0: 39030004  addi r8, r3, 4
	ctx.r[8].s64 = ctx.r[3].s64 + 4;
	// 8248A5F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248A5F8: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8248A5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8248A600: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8248A604: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A608: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8248A60C: 4080FFF4  bge 0x8248a600
	if !ctx.cr[0].lt {
	pc = 0x8248A600; continue 'dispatch;
	}
	// 8248A610: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 8248A614: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248A618: 39490004  addi r10, r9, 4
	ctx.r[10].s64 = ctx.r[9].s64 + 4;
	// 8248A61C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8248A620: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A624: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8248A628: 4080FFF4  bge 0x8248a61c
	if !ctx.cr[0].lt {
	pc = 0x8248A61C; continue 'dispatch;
	}
	// 8248A62C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8248A630: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248A634: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8248A638: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8248A63C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A640: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8248A644: 4080FFF4  bge 0x8248a638
	if !ctx.cr[0].lt {
	pc = 0x8248A638; continue 'dispatch;
	}
	// 8248A648: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248A64C: 39490004  addi r10, r9, 4
	ctx.r[10].s64 = ctx.r[9].s64 + 4;
	// 8248A650: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8248A654: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A658: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8248A65C: 4080FFF4  bge 0x8248a650
	if !ctx.cr[0].lt {
	pc = 0x8248A650; continue 'dispatch;
	}
	// 8248A660: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 8248A664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A668 size=84
    let mut pc: u32 = 0x8248A668;
    'dispatch: loop {
        match pc {
            0x8248A668 => {
    //   block [0x8248A668..0x8248A6BC)
	// 8248A668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A67C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A680: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A684: 4BFFFEA5  bl 0x8248a528
	ctx.lr = 0x8248A688;
	sub_8248A528(ctx, base);
	// 8248A688: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A68C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248A690: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8248A694: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A698: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A69C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248A6A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A6A4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A6A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A6B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A6C0 size=88
    let mut pc: u32 = 0x8248A6C0;
    'dispatch: loop {
        match pc {
            0x8248A6C0 => {
    //   block [0x8248A6C0..0x8248A718)
	// 8248A6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A6C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A6CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A6D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A6D4: 4BFFFEAD  bl 0x8248a580
	ctx.lr = 0x8248A6D8;
	sub_8248A580(ctx, base);
	// 8248A6D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248A6DC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8248A6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8248A6E4: 99630025  stb r11, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 8248A6E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A6EC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8248A6F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A6F4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248A6F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A6FC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A700: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248A704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A718 size=88
    let mut pc: u32 = 0x8248A718;
    'dispatch: loop {
        match pc {
            0x8248A718 => {
    //   block [0x8248A718..0x8248A770)
	// 8248A718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A71C: 48D1DA51  bl 0x831a816c
	ctx.lr = 0x8248A720;
	sub_831A8130(ctx, base);
	// 8248A720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A724: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248A728: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248A72C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8248A730: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248A734: 4800002C  b 0x8248a760
	pc = 0x8248A760; continue 'dispatch;
	// 8248A738: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248A73C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A740: 4BFFFFD9  bl 0x8248a718
	ctx.lr = 0x8248A744;
	sub_8248A718(ctx, base);
	// 8248A744: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8248A748: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A74C: 48968CDD  bl 0x82df3428
	ctx.lr = 0x8248A750;
	sub_82DF3428(ctx, base);
	// 8248A750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248A754: 4BE35B15  bl 0x822c0268
	ctx.lr = 0x8248A758;
	sub_822C0268(ctx, base);
	// 8248A758: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248A75C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8248A760: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248A764: 419AFFD4  beq cr6, 0x8248a738
	if ctx.cr[6].eq {
	pc = 0x8248A738; continue 'dispatch;
	}
	// 8248A768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248A76C: 48D1DA50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A770 size=88
    let mut pc: u32 = 0x8248A770;
    'dispatch: loop {
        match pc {
            0x8248A770 => {
    //   block [0x8248A770..0x8248A7C8)
	// 8248A770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A774: 48D1D9F9  bl 0x831a816c
	ctx.lr = 0x8248A778;
	sub_831A8130(ctx, base);
	// 8248A778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A77C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248A780: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248A784: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8248A788: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248A78C: 4800002C  b 0x8248a7b8
	pc = 0x8248A7B8; continue 'dispatch;
	// 8248A790: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248A794: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A798: 4BFFFFD9  bl 0x8248a770
	ctx.lr = 0x8248A79C;
	sub_8248A770(ctx, base);
	// 8248A79C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 8248A7A0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A7A4: 48968C85  bl 0x82df3428
	ctx.lr = 0x8248A7A8;
	sub_82DF3428(ctx, base);
	// 8248A7A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248A7AC: 4BE35ABD  bl 0x822c0268
	ctx.lr = 0x8248A7B0;
	sub_822C0268(ctx, base);
	// 8248A7B0: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248A7B4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8248A7B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248A7BC: 419AFFD4  beq cr6, 0x8248a790
	if ctx.cr[6].eq {
	pc = 0x8248A790; continue 'dispatch;
	}
	// 8248A7C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248A7C4: 48D1D9F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A7C8 size=84
    let mut pc: u32 = 0x8248A7C8;
    'dispatch: loop {
        match pc {
            0x8248A7C8 => {
    //   block [0x8248A7C8..0x8248A81C)
	// 8248A7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A7D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A7D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A7D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A7DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A7E0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A7E4: 4BFFFF35  bl 0x8248a718
	ctx.lr = 0x8248A7E8;
	sub_8248A718(ctx, base);
	// 8248A7E8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A7EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248A7F0: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8248A7F4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A7F8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A7FC: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248A800: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A804: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A820 size=84
    let mut pc: u32 = 0x8248A820;
    'dispatch: loop {
        match pc {
            0x8248A820 => {
    //   block [0x8248A820..0x8248A874)
	// 8248A820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248A82C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248A834: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A838: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A83C: 4BFFFF35  bl 0x8248a770
	ctx.lr = 0x8248A840;
	sub_8248A770(ctx, base);
	// 8248A840: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A844: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248A848: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8248A84C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A850: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A854: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248A858: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A85C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248A860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248A864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A86C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248A870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A878 size=96
    let mut pc: u32 = 0x8248A878;
    'dispatch: loop {
        match pc {
            0x8248A878 => {
    //   block [0x8248A878..0x8248A8D8)
	// 8248A878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248A880: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A884: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248A888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248A88C: 388BA3C4  addi r4, r11, -0x5c3c
	ctx.r[4].s64 = ctx.r[11].s64 + -23612;
	// 8248A890: 4BE3B039  bl 0x822c58c8
	ctx.lr = 0x8248A894;
	sub_822C58C8(ctx, base);
	// 8248A894: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8248A898: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248A89C: 4BE3F615  bl 0x822c9eb0
	ctx.lr = 0x8248A8A0;
	sub_822C9EB0(ctx, base);
	// 8248A8A0: 4BE39A11  bl 0x822c42b0
	ctx.lr = 0x8248A8A4;
	sub_822C42B0(ctx, base);
	// 8248A8A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248A8A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248A8AC: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8248A8B0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8248A8B4: 4BE3ABBD  bl 0x822c5470
	ctx.lr = 0x8248A8B8;
	sub_822C5470(ctx, base);
	// 8248A8B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248A8BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248A8C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248A8C4: 4BE3A41D  bl 0x822c4ce0
	ctx.lr = 0x8248A8C8;
	sub_822C4CE0(ctx, base);
	// 8248A8C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8248A8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248A8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248A8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248A8D8 size=548
    let mut pc: u32 = 0x8248A8D8;
    'dispatch: loop {
        match pc {
            0x8248A8D8 => {
    //   block [0x8248A8D8..0x8248AAFC)
	// 8248A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248A8DC: 48D1D885  bl 0x831a8160
	ctx.lr = 0x8248A8E0;
	sub_831A8130(ctx, base);
	// 8248A8E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248A8E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248A8E8: 3D600AAA  lis r11, 0xaaa
	ctx.r[11].s64 = 178913280;
	// 8248A8EC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8248A8F0: 616BAAA9  ori r11, r11, 0xaaa9
	ctx.r[11].u64 = ctx.r[11].u64 | 43689;
	// 8248A8F4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8248A8F8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A8FC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8248A900: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8248A904: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248A908: 41980048  blt cr6, 0x8248a950
	if ctx.cr[6].lt {
	pc = 0x8248A950; continue 'dispatch;
	}
	// 8248A90C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248A910: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248A914: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 8248A918: 4BE3AFB1  bl 0x822c58c8
	ctx.lr = 0x8248A91C;
	sub_822C58C8(ctx, base);
	// 8248A91C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8248A920: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248A924: 4BE3AEF5  bl 0x822c5818
	ctx.lr = 0x8248A928;
	sub_822C5818(ctx, base);
	// 8248A928: 4BE39989  bl 0x822c42b0
	ctx.lr = 0x8248A92C;
	sub_822C42B0(ctx, base);
	// 8248A92C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248A930: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248A934: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 8248A938: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8248A93C: 4BE3AB35  bl 0x822c5470
	ctx.lr = 0x8248A940;
	sub_822C5470(ctx, base);
	// 8248A940: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248A944: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248A948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248A94C: 4BE3A395  bl 0x822c4ce0
	ctx.lr = 0x8248A950;
	sub_822C4CE0(ctx, base);
	// 8248A950: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8248A958: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8248A95C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8248A960: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8248A964: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248A968: 4BFFFB49  bl 0x8248a4b0
	ctx.lr = 0x8248A96C;
	sub_8248A4B0(ctx, base);
	// 8248A96C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A970: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A974: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8248A978: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8248A97C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248A980: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248A984: 409A0018  bne cr6, 0x8248a99c
	if !ctx.cr[6].eq {
	pc = 0x8248A99C; continue 'dispatch;
	}
	// 8248A988: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8248A98C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A990: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248A994: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A998: 4800003C  b 0x8248a9d4
	pc = 0x8248A9D4; continue 'dispatch;
	// 8248A99C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248A9A0: 41820020  beq 0x8248a9c0
	if ctx.cr[0].eq {
	pc = 0x8248A9C0; continue 'dispatch;
	}
	// 8248A9A4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248A9A8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A9AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A9B0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248A9B4: 409A0024  bne cr6, 0x8248a9d8
	if !ctx.cr[6].eq {
	pc = 0x8248A9D8; continue 'dispatch;
	}
	// 8248A9B8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248A9BC: 4800001C  b 0x8248a9d8
	pc = 0x8248A9D8; continue 'dispatch;
	// 8248A9C0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8248A9C4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A9C8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248A9CC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248A9D0: 409A0008  bne cr6, 0x8248a9d8
	if !ctx.cr[6].eq {
	pc = 0x8248A9D8; continue 'dispatch;
	}
	// 8248A9D4: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8248A9D8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248A9DC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8248A9E0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8248A9E4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 8248A9E8: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248A9EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248A9F0: 409A00F0  bne cr6, 0x8248aae0
	if !ctx.cr[6].eq {
	pc = 0x8248AAE0; continue 'dispatch;
	}
	// 8248A9F4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8248A9F8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248A9FC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA00: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AA04: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248AA08: 409A0054  bne cr6, 0x8248aa5c
	if !ctx.cr[6].eq {
	pc = 0x8248AA5C; continue 'dispatch;
	}
	// 8248AA0C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AA10: 892A0024  lbz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AA14: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248AA18: 419A0054  beq cr6, 0x8248aa6c
	if ctx.cr[6].eq {
	pc = 0x8248AA6C; continue 'dispatch;
	}
	// 8248AA1C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AA20: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248AA24: 409A0010  bne cr6, 0x8248aa34
	if !ctx.cr[6].eq {
	pc = 0x8248AA34; continue 'dispatch;
	}
	// 8248AA28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248AA2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248AA30: 4BFFFA19  bl 0x8248a448
	ctx.lr = 0x8248AA34;
	sub_8248A448(ctx, base);
	// 8248AA34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248AA3C: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AA40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA48: 9B6B0024  stb r27, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 8248AA4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA50: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA54: 4BE6E4FD  bl 0x822f8f50
	ctx.lr = 0x8248AA58;
	sub_822F8F50(ctx, base);
	// 8248AA58: 48000074  b 0x8248aacc
	pc = 0x8248AACC; continue 'dispatch;
	// 8248AA5C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AA60: 892A0024  lbz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AA64: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248AA68: 409A0028  bne cr6, 0x8248aa90
	if !ctx.cr[6].eq {
	pc = 0x8248AA90; continue 'dispatch;
	}
	// 8248AA6C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AA70: 9BA90024  stb r29, 0x24(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AA74: 9BAA0024  stb r29, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AA78: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AA7C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA80: 9B6A0024  stb r27, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 8248AA84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AA88: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AA8C: 48000040  b 0x8248aacc
	pc = 0x8248AACC; continue 'dispatch;
	// 8248AA90: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AA94: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248AA98: 409A0010  bne cr6, 0x8248aaa8
	if !ctx.cr[6].eq {
	pc = 0x8248AAA8; continue 'dispatch;
	}
	// 8248AA9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248AAA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248AAA4: 4BE6E4AD  bl 0x822f8f50
	ctx.lr = 0x8248AAA8;
	sub_822F8F50(ctx, base);
	// 8248AAA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AAAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248AAB0: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AAB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AAB8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AABC: 9B6B0024  stb r27, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 8248AAC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AAC4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AAC8: 4BFFF981  bl 0x8248a448
	ctx.lr = 0x8248AACC;
	sub_8248A448(ctx, base);
	// 8248AACC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AAD0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8248AAD4: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AAD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AADC: 419AFF1C  beq cr6, 0x8248a9f8
	if ctx.cr[6].eq {
	pc = 0x8248A9F8; continue 'dispatch;
	}
	// 8248AAE0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AAE4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8248AAE8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248AAEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AAF0: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AAF4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8248AAF8: 48D1D6B8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248AB00 size=1016
    let mut pc: u32 = 0x8248AB00;
    'dispatch: loop {
        match pc {
            0x8248AB00 => {
    //   block [0x8248AB00..0x8248AEF8)
	// 8248AB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248AB04: 48D1D655  bl 0x831a8158
	ctx.lr = 0x8248AB08;
	sub_831A8130(ctx, base);
	// 8248AB08: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248AB0C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8248AB10: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8248AB14: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8248AB18: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8248AB1C: 897F0025  lbz r11, 0x25(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AB20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248AB24: 419A0048  beq cr6, 0x8248ab6c
	if ctx.cr[6].eq {
	pc = 0x8248AB6C; continue 'dispatch;
	}
	// 8248AB28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248AB2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248AB30: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8248AB34: 4BE3AD95  bl 0x822c58c8
	ctx.lr = 0x8248AB38;
	sub_822C58C8(ctx, base);
	// 8248AB38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8248AB3C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248AB40: 4BE3F371  bl 0x822c9eb0
	ctx.lr = 0x8248AB44;
	sub_822C9EB0(ctx, base);
	// 8248AB44: 4BE3976D  bl 0x822c42b0
	ctx.lr = 0x8248AB48;
	sub_822C42B0(ctx, base);
	// 8248AB48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248AB4C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248AB50: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8248AB54: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8248AB58: 4BE3A919  bl 0x822c5470
	ctx.lr = 0x8248AB5C;
	sub_822C5470(ctx, base);
	// 8248AB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248AB60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248AB64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248AB68: 4BE3A179  bl 0x822c4ce0
	ctx.lr = 0x8248AB6C;
	sub_822C4CE0(ctx, base);
	// 8248AB6C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8248AB70: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 8248AB74: 4BFFF85D  bl 0x8248a3d0
	ctx.lr = 0x8248AB78;
	sub_8248A3D0(ctx, base);
	// 8248AB78: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AB7C: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AB80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AB84: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8248AB88: 419A000C  beq cr6, 0x8248ab94
	if ctx.cr[6].eq {
	pc = 0x8248AB94; continue 'dispatch;
	}
	// 8248AB8C: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AB90: 48000028  b 0x8248abb8
	pc = 0x8248ABB8; continue 'dispatch;
	// 8248AB94: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AB98: 894A0025  lbz r10, 0x25(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AB9C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248ABA0: 419A000C  beq cr6, 0x8248abac
	if ctx.cr[6].eq {
	pc = 0x8248ABAC; continue 'dispatch;
	}
	// 8248ABA4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8248ABA8: 48000010  b 0x8248abb8
	pc = 0x8248ABB8; continue 'dispatch;
	// 8248ABAC: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ABB0: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248ABB4: 409A00DC  bne cr6, 0x8248ac90
	if !ctx.cr[6].eq {
	pc = 0x8248AC90; continue 'dispatch;
	}
	// 8248ABB8: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248ABBC: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ABC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248ABC4: 409A0008  bne cr6, 0x8248abcc
	if !ctx.cr[6].eq {
	pc = 0x8248ABCC; continue 'dispatch;
	}
	// 8248ABC8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8248ABCC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ABD0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ABD4: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248ABD8: 409A000C  bne cr6, 0x8248abe4
	if !ctx.cr[6].eq {
	pc = 0x8248ABE4; continue 'dispatch;
	}
	// 8248ABDC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8248ABE0: 4800001C  b 0x8248abfc
	pc = 0x8248ABFC; continue 'dispatch;
	// 8248ABE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248ABE8: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248ABEC: 409A000C  bne cr6, 0x8248abf8
	if !ctx.cr[6].eq {
	pc = 0x8248ABF8; continue 'dispatch;
	}
	// 8248ABF0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248ABF4: 48000008  b 0x8248abfc
	pc = 0x8248ABFC; continue 'dispatch;
	// 8248ABF8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8248ABFC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AC00: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AC04: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248AC08: 409A003C  bne cr6, 0x8248ac44
	if !ctx.cr[6].eq {
	pc = 0x8248AC44; continue 'dispatch;
	}
	// 8248AC0C: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AC10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248AC14: 419A000C  beq cr6, 0x8248ac20
	if ctx.cr[6].eq {
	pc = 0x8248AC20; continue 'dispatch;
	}
	// 8248AC18: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8248AC1C: 48000024  b 0x8248ac40
	pc = 0x8248AC40; continue 'dispatch;
	// 8248AC20: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AC24: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8248AC28: 4800000C  b 0x8248ac34
	pc = 0x8248AC34; continue 'dispatch;
	// 8248AC2C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8248AC30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AC34: 890B0025  lbz r8, 0x25(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AC38: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8248AC3C: 419AFFF0  beq cr6, 0x8248ac2c
	if ctx.cr[6].eq {
	pc = 0x8248AC2C; continue 'dispatch;
	}
	// 8248AC40: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248AC44: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AC48: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AC4C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248AC50: 409A00D4  bne cr6, 0x8248ad24
	if !ctx.cr[6].eq {
	pc = 0x8248AD24; continue 'dispatch;
	}
	// 8248AC54: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AC58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248AC5C: 419A000C  beq cr6, 0x8248ac68
	if ctx.cr[6].eq {
	pc = 0x8248AC68; continue 'dispatch;
	}
	// 8248AC60: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8248AC64: 48000024  b 0x8248ac88
	pc = 0x8248AC88; continue 'dispatch;
	// 8248AC68: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AC6C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8248AC70: 4800000C  b 0x8248ac7c
	pc = 0x8248AC7C; continue 'dispatch;
	// 8248AC74: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8248AC78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AC7C: 890B0025  lbz r8, 0x25(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AC80: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8248AC84: 419AFFF0  beq cr6, 0x8248ac74
	if ctx.cr[6].eq {
	pc = 0x8248AC74; continue 'dispatch;
	}
	// 8248AC88: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248AC8C: 48000098  b 0x8248ad24
	pc = 0x8248AD24; continue 'dispatch;
	// 8248AC90: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8248AC94: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AC98: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248AC9C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ACA0: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248ACA4: 409A000C  bne cr6, 0x8248acb0
	if !ctx.cr[6].eq {
	pc = 0x8248ACB0; continue 'dispatch;
	}
	// 8248ACA8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8248ACAC: 4800002C  b 0x8248acd8
	pc = 0x8248ACD8; continue 'dispatch;
	// 8248ACB0: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248ACB4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ACB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248ACBC: 409A0008  bne cr6, 0x8248acc4
	if !ctx.cr[6].eq {
	pc = 0x8248ACC4; continue 'dispatch;
	}
	// 8248ACC0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8248ACC4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248ACC8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ACCC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248ACD0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ACD4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8248ACD8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ACDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ACE0: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248ACE4: 409A000C  bne cr6, 0x8248acf0
	if !ctx.cr[6].eq {
	pc = 0x8248ACF0; continue 'dispatch;
	}
	// 8248ACE8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8248ACEC: 48000020  b 0x8248ad0c
	pc = 0x8248AD0C; continue 'dispatch;
	// 8248ACF0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248ACF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248ACF8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248ACFC: 409A000C  bne cr6, 0x8248ad08
	if !ctx.cr[6].eq {
	pc = 0x8248AD08; continue 'dispatch;
	}
	// 8248AD00: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8248AD04: 48000008  b 0x8248ad0c
	pc = 0x8248AD0C; continue 'dispatch;
	// 8248AD08: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8248AD0C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AD10: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8248AD14: 897A0024  lbz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AD18: 89590024  lbz r10, 0x24(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AD1C: 99790024  stb r11, 0x24(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 8248AD20: 995A0024  stb r10, 0x24(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(36 as u32), ctx.r[10].u8 ) };
	// 8248AD24: 897A0024  lbz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AD28: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8248AD2C: 409A0198  bne cr6, 0x8248aec4
	if !ctx.cr[6].eq {
	pc = 0x8248AEC4; continue 'dispatch;
	}
	// 8248AD30: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AD34: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8248AD38: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AD3C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248AD40: 419A0180  beq cr6, 0x8248aec0
	if ctx.cr[6].eq {
	pc = 0x8248AEC0; continue 'dispatch;
	}
	// 8248AD44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8248AD48: 897C0024  lbz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AD4C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8248AD50: 409A0170  bne cr6, 0x8248aec0
	if !ctx.cr[6].eq {
	pc = 0x8248AEC0; continue 'dispatch;
	}
	// 8248AD54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AD58: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248AD5C: 409A00A8  bne cr6, 0x8248ae04
	if !ctx.cr[6].eq {
	pc = 0x8248AE04; continue 'dispatch;
	}
	// 8248AD60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AD64: 894B0024  lbz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AD68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AD6C: 409A001C  bne cr6, 0x8248ad88
	if !ctx.cr[6].eq {
	pc = 0x8248AD88; continue 'dispatch;
	}
	// 8248AD70: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248AD74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248AD78: 9BBF0024  stb r29, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AD7C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248AD80: 4BFFF6C9  bl 0x8248a448
	ctx.lr = 0x8248AD84;
	sub_8248A448(ctx, base);
	// 8248AD84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AD88: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AD8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AD90: 409A00C8  bne cr6, 0x8248ae58
	if !ctx.cr[6].eq {
	pc = 0x8248AE58; continue 'dispatch;
	}
	// 8248AD94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AD98: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AD9C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248ADA0: 409A0014  bne cr6, 0x8248adb4
	if !ctx.cr[6].eq {
	pc = 0x8248ADB4; continue 'dispatch;
	}
	// 8248ADA4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ADA8: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248ADAC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248ADB0: 419A00A4  beq cr6, 0x8248ae54
	if ctx.cr[6].eq {
	pc = 0x8248AE54; continue 'dispatch;
	}
	// 8248ADB4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ADB8: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248ADBC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248ADC0: 409A0020  bne cr6, 0x8248ade0
	if !ctx.cr[6].eq {
	pc = 0x8248ADE0; continue 'dispatch;
	}
	// 8248ADC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248ADC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8248ADCC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248ADD0: 9BCA0024  stb r30, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248ADD4: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248ADD8: 4BE6E179  bl 0x822f8f50
	ctx.lr = 0x8248ADDC;
	sub_822F8F50(ctx, base);
	// 8248ADDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ADE0: 895F0024  lbz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248ADE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248ADE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248ADEC: 994B0024  stb r10, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u8 ) };
	// 8248ADF0: 9BDF0024  stb r30, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248ADF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248ADF8: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248ADFC: 4BFFF64D  bl 0x8248a448
	ctx.lr = 0x8248AE00;
	sub_8248A448(ctx, base);
	// 8248AE00: 480000C0  b 0x8248aec0
	pc = 0x8248AEC0; continue 'dispatch;
	// 8248AE04: 894B0024  lbz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AE08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AE0C: 409A001C  bne cr6, 0x8248ae28
	if !ctx.cr[6].eq {
	pc = 0x8248AE28; continue 'dispatch;
	}
	// 8248AE10: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248AE14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248AE18: 9BBF0024  stb r29, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AE1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248AE20: 4BE6E131  bl 0x822f8f50
	ctx.lr = 0x8248AE24;
	sub_822F8F50(ctx, base);
	// 8248AE24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AE28: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248AE2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AE30: 409A0028  bne cr6, 0x8248ae58
	if !ctx.cr[6].eq {
	pc = 0x8248AE58; continue 'dispatch;
	}
	// 8248AE34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AE38: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AE3C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248AE40: 409A0034  bne cr6, 0x8248ae74
	if !ctx.cr[6].eq {
	pc = 0x8248AE74; continue 'dispatch;
	}
	// 8248AE44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AE48: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AE4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248AE50: 409A0024  bne cr6, 0x8248ae74
	if !ctx.cr[6].eq {
	pc = 0x8248AE74; continue 'dispatch;
	}
	// 8248AE54: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AE58: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AE5C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 8248AE60: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AE64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AE68: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248AE6C: 409AFEDC  bne cr6, 0x8248ad48
	if !ctx.cr[6].eq {
	pc = 0x8248AD48; continue 'dispatch;
	}
	// 8248AE70: 48000050  b 0x8248aec0
	pc = 0x8248AEC0; continue 'dispatch;
	// 8248AE74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AE78: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AE7C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248AE80: 409A0020  bne cr6, 0x8248aea0
	if !ctx.cr[6].eq {
	pc = 0x8248AEA0; continue 'dispatch;
	}
	// 8248AE84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AE88: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8248AE8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248AE90: 9BCA0024  stb r30, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248AE94: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 8248AE98: 4BFFF5B1  bl 0x8248a448
	ctx.lr = 0x8248AE9C;
	sub_8248A448(ctx, base);
	// 8248AE9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AEA0: 895F0024  lbz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8248AEA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248AEA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248AEAC: 994B0024  stb r10, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u8 ) };
	// 8248AEB0: 9BDF0024  stb r30, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248AEB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AEB8: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248AEBC: 4BE6E095  bl 0x822f8f50
	ctx.lr = 0x8248AEC0;
	sub_822F8F50(ctx, base);
	// 8248AEC0: 9BDC0024  stb r30, 0x24(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 8248AEC4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248AEC8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8248AECC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248AED0: 489672B9  bl 0x82df2188
	ctx.lr = 0x8248AED4;
	sub_82DF2188(ctx, base);
	// 8248AED4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248AEDC: 419A000C  beq cr6, 0x8248aee8
	if ctx.cr[6].eq {
	pc = 0x8248AEE8; continue 'dispatch;
	}
	// 8248AEE0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8248AEE4: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248AEE8: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8248AEEC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8248AEF0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8248AEF4: 48D1D2B4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248AEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248AEF8 size=1016
    let mut pc: u32 = 0x8248AEF8;
    'dispatch: loop {
        match pc {
            0x8248AEF8 => {
    //   block [0x8248AEF8..0x8248B2F0)
	// 8248AEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248AEFC: 48D1D25D  bl 0x831a8158
	ctx.lr = 0x8248AF00;
	sub_831A8130(ctx, base);
	// 8248AF00: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248AF04: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8248AF08: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8248AF0C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8248AF10: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8248AF14: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248AF18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248AF1C: 419A0048  beq cr6, 0x8248af64
	if ctx.cr[6].eq {
	pc = 0x8248AF64; continue 'dispatch;
	}
	// 8248AF20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248AF24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248AF28: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8248AF2C: 4BE3A99D  bl 0x822c58c8
	ctx.lr = 0x8248AF30;
	sub_822C58C8(ctx, base);
	// 8248AF30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8248AF34: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248AF38: 4BE3EF79  bl 0x822c9eb0
	ctx.lr = 0x8248AF3C;
	sub_822C9EB0(ctx, base);
	// 8248AF3C: 4BE39375  bl 0x822c42b0
	ctx.lr = 0x8248AF40;
	sub_822C42B0(ctx, base);
	// 8248AF40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8248AF44: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8248AF48: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8248AF4C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8248AF50: 4BE3A521  bl 0x822c5470
	ctx.lr = 0x8248AF54;
	sub_822C5470(ctx, base);
	// 8248AF54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248AF58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248AF5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248AF60: 4BE39D81  bl 0x822c4ce0
	ctx.lr = 0x8248AF64;
	sub_822C4CE0(ctx, base);
	// 8248AF64: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8248AF68: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 8248AF6C: 4805E125  bl 0x824e9090
	ctx.lr = 0x8248AF70;
	sub_824E9090(ctx, base);
	// 8248AF70: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AF74: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248AF78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AF7C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8248AF80: 419A000C  beq cr6, 0x8248af8c
	if ctx.cr[6].eq {
	pc = 0x8248AF8C; continue 'dispatch;
	}
	// 8248AF84: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AF88: 48000028  b 0x8248afb0
	pc = 0x8248AFB0; continue 'dispatch;
	// 8248AF8C: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AF90: 894A0015  lbz r10, 0x15(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248AF94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248AF98: 419A000C  beq cr6, 0x8248afa4
	if ctx.cr[6].eq {
	pc = 0x8248AFA4; continue 'dispatch;
	}
	// 8248AF9C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8248AFA0: 48000010  b 0x8248afb0
	pc = 0x8248AFB0; continue 'dispatch;
	// 8248AFA4: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248AFA8: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248AFAC: 409A00DC  bne cr6, 0x8248b088
	if !ctx.cr[6].eq {
	pc = 0x8248B088; continue 'dispatch;
	}
	// 8248AFB0: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248AFB4: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AFB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248AFBC: 409A0008  bne cr6, 0x8248afc4
	if !ctx.cr[6].eq {
	pc = 0x8248AFC4; continue 'dispatch;
	}
	// 8248AFC0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8248AFC4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AFC8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AFCC: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248AFD0: 409A000C  bne cr6, 0x8248afdc
	if !ctx.cr[6].eq {
	pc = 0x8248AFDC; continue 'dispatch;
	}
	// 8248AFD4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8248AFD8: 4800001C  b 0x8248aff4
	pc = 0x8248AFF4; continue 'dispatch;
	// 8248AFDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AFE0: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248AFE4: 409A000C  bne cr6, 0x8248aff0
	if !ctx.cr[6].eq {
	pc = 0x8248AFF0; continue 'dispatch;
	}
	// 8248AFE8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248AFEC: 48000008  b 0x8248aff4
	pc = 0x8248AFF4; continue 'dispatch;
	// 8248AFF0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8248AFF4: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248AFF8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248AFFC: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248B000: 409A003C  bne cr6, 0x8248b03c
	if !ctx.cr[6].eq {
	pc = 0x8248B03C; continue 'dispatch;
	}
	// 8248B004: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248B008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B00C: 419A000C  beq cr6, 0x8248b018
	if ctx.cr[6].eq {
	pc = 0x8248B018; continue 'dispatch;
	}
	// 8248B010: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8248B014: 48000024  b 0x8248b038
	pc = 0x8248B038; continue 'dispatch;
	// 8248B018: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B01C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8248B020: 4800000C  b 0x8248b02c
	pc = 0x8248B02C; continue 'dispatch;
	// 8248B024: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8248B028: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B02C: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248B030: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8248B034: 419AFFF0  beq cr6, 0x8248b024
	if ctx.cr[6].eq {
	pc = 0x8248B024; continue 'dispatch;
	}
	// 8248B038: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248B03C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B040: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B044: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248B048: 409A00D4  bne cr6, 0x8248b11c
	if !ctx.cr[6].eq {
	pc = 0x8248B11C; continue 'dispatch;
	}
	// 8248B04C: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248B050: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B054: 419A000C  beq cr6, 0x8248b060
	if ctx.cr[6].eq {
	pc = 0x8248B060; continue 'dispatch;
	}
	// 8248B058: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8248B05C: 48000024  b 0x8248b080
	pc = 0x8248B080; continue 'dispatch;
	// 8248B060: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B064: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8248B068: 4800000C  b 0x8248b074
	pc = 0x8248B074; continue 'dispatch;
	// 8248B06C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8248B070: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B074: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248B078: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8248B07C: 419AFFF0  beq cr6, 0x8248b06c
	if ctx.cr[6].eq {
	pc = 0x8248B06C; continue 'dispatch;
	}
	// 8248B080: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248B084: 48000098  b 0x8248b11c
	pc = 0x8248B11C; continue 'dispatch;
	// 8248B088: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8248B08C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B090: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248B094: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B098: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B09C: 409A000C  bne cr6, 0x8248b0a8
	if !ctx.cr[6].eq {
	pc = 0x8248B0A8; continue 'dispatch;
	}
	// 8248B0A0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8248B0A4: 4800002C  b 0x8248b0d0
	pc = 0x8248B0D0; continue 'dispatch;
	// 8248B0A8: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248B0AC: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B0B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B0B4: 409A0008  bne cr6, 0x8248b0bc
	if !ctx.cr[6].eq {
	pc = 0x8248B0BC; continue 'dispatch;
	}
	// 8248B0B8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8248B0BC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8248B0C0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B0C4: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248B0C8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B0CC: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8248B0D0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B0D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B0D8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248B0DC: 409A000C  bne cr6, 0x8248b0e8
	if !ctx.cr[6].eq {
	pc = 0x8248B0E8; continue 'dispatch;
	}
	// 8248B0E0: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8248B0E4: 48000020  b 0x8248b104
	pc = 0x8248B104; continue 'dispatch;
	// 8248B0E8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B0EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B0F0: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248B0F4: 409A000C  bne cr6, 0x8248b100
	if !ctx.cr[6].eq {
	pc = 0x8248B100; continue 'dispatch;
	}
	// 8248B0F8: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8248B0FC: 48000008  b 0x8248b104
	pc = 0x8248B104; continue 'dispatch;
	// 8248B100: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8248B104: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B108: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8248B10C: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B110: 89590014  lbz r10, 0x14(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B114: 99790014  stb r11, 0x14(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8248B118: 995A0014  stb r10, 0x14(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8248B11C: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B120: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8248B124: 409A0198  bne cr6, 0x8248b2bc
	if !ctx.cr[6].eq {
	pc = 0x8248B2BC; continue 'dispatch;
	}
	// 8248B128: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B12C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8248B130: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B134: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B138: 419A0180  beq cr6, 0x8248b2b8
	if ctx.cr[6].eq {
	pc = 0x8248B2B8; continue 'dispatch;
	}
	// 8248B13C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8248B140: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B144: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8248B148: 409A0170  bne cr6, 0x8248b2b8
	if !ctx.cr[6].eq {
	pc = 0x8248B2B8; continue 'dispatch;
	}
	// 8248B14C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B150: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B154: 409A00A8  bne cr6, 0x8248b1fc
	if !ctx.cr[6].eq {
	pc = 0x8248B1FC; continue 'dispatch;
	}
	// 8248B158: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B15C: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B160: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248B164: 409A001C  bne cr6, 0x8248b180
	if !ctx.cr[6].eq {
	pc = 0x8248B180; continue 'dispatch;
	}
	// 8248B168: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B16C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248B170: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8248B174: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248B178: 4872BA59  bl 0x82bb6bd0
	ctx.lr = 0x8248B17C;
	sub_82BB6BD0(ctx, base);
	// 8248B17C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B180: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248B184: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248B188: 409A00C8  bne cr6, 0x8248b250
	if !ctx.cr[6].eq {
	pc = 0x8248B250; continue 'dispatch;
	}
	// 8248B18C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B190: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B194: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248B198: 409A0014  bne cr6, 0x8248b1ac
	if !ctx.cr[6].eq {
	pc = 0x8248B1AC; continue 'dispatch;
	}
	// 8248B19C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B1A0: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B1A4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248B1A8: 419A00A4  beq cr6, 0x8248b24c
	if ctx.cr[6].eq {
	pc = 0x8248B24C; continue 'dispatch;
	}
	// 8248B1AC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B1B0: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B1B4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248B1B8: 409A0020  bne cr6, 0x8248b1d8
	if !ctx.cr[6].eq {
	pc = 0x8248B1D8; continue 'dispatch;
	}
	// 8248B1BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B1C0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8248B1C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248B1C8: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B1CC: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8248B1D0: 4BE63341  bl 0x822ee510
	ctx.lr = 0x8248B1D4;
	sub_822EE510(ctx, base);
	// 8248B1D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B1D8: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B1DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248B1E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248B1E4: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8248B1E8: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B1EC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B1F0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B1F4: 4872B9DD  bl 0x82bb6bd0
	ctx.lr = 0x8248B1F8;
	sub_82BB6BD0(ctx, base);
	// 8248B1F8: 480000C0  b 0x8248b2b8
	pc = 0x8248B2B8; continue 'dispatch;
	// 8248B1FC: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B200: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248B204: 409A001C  bne cr6, 0x8248b220
	if !ctx.cr[6].eq {
	pc = 0x8248B220; continue 'dispatch;
	}
	// 8248B208: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B20C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248B210: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8248B214: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248B218: 4BE632F9  bl 0x822ee510
	ctx.lr = 0x8248B21C;
	sub_822EE510(ctx, base);
	// 8248B21C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B220: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8248B224: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248B228: 409A0028  bne cr6, 0x8248b250
	if !ctx.cr[6].eq {
	pc = 0x8248B250; continue 'dispatch;
	}
	// 8248B22C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B230: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B234: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248B238: 409A0034  bne cr6, 0x8248b26c
	if !ctx.cr[6].eq {
	pc = 0x8248B26C; continue 'dispatch;
	}
	// 8248B23C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B240: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B244: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248B248: 409A0024  bne cr6, 0x8248b26c
	if !ctx.cr[6].eq {
	pc = 0x8248B26C; continue 'dispatch;
	}
	// 8248B24C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8248B250: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B254: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 8248B258: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B25C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B260: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B264: 409AFEDC  bne cr6, 0x8248b140
	if !ctx.cr[6].eq {
	pc = 0x8248B140; continue 'dispatch;
	}
	// 8248B268: 48000050  b 0x8248b2b8
	pc = 0x8248B2B8; continue 'dispatch;
	// 8248B26C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B270: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B274: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8248B278: 409A0020  bne cr6, 0x8248b298
	if !ctx.cr[6].eq {
	pc = 0x8248B298; continue 'dispatch;
	}
	// 8248B27C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B280: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8248B284: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248B288: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B28C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8248B290: 4872B941  bl 0x82bb6bd0
	ctx.lr = 0x8248B294;
	sub_82BB6BD0(ctx, base);
	// 8248B294: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B298: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8248B29C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248B2A0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248B2A4: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8248B2A8: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B2AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B2B0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B2B4: 4BE6325D  bl 0x822ee510
	ctx.lr = 0x8248B2B8;
	sub_822EE510(ctx, base);
	// 8248B2B8: 9BDC0014  stb r30, 0x14(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8248B2BC: 387A000C  addi r3, r26, 0xc
	ctx.r[3].s64 = ctx.r[26].s64 + 12;
	// 8248B2C0: 48968169  bl 0x82df3428
	ctx.lr = 0x8248B2C4;
	sub_82DF3428(ctx, base);
	// 8248B2C4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8248B2C8: 4BE34FA1  bl 0x822c0268
	ctx.lr = 0x8248B2CC;
	sub_822C0268(ctx, base);
	// 8248B2CC: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B2D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B2D4: 419A000C  beq cr6, 0x8248b2e0
	if ctx.cr[6].eq {
	pc = 0x8248B2E0; continue 'dispatch;
	}
	// 8248B2D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8248B2DC: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248B2E0: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8248B2E4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8248B2E8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8248B2EC: 48D1CEBC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248B2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248B2F0 size=100
    let mut pc: u32 = 0x8248B2F0;
    'dispatch: loop {
        match pc {
            0x8248B2F0 => {
    //   block [0x8248B2F0..0x8248B354)
	// 8248B2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248B2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248B2F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248B2FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248B300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248B304: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248B308: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248B30C: 2B1F0040  cmplwi cr6, r31, 0x40
	ctx.cr[6].compare_u32(ctx.r[31].u32, 64 as u32, &mut ctx.xer);
	// 8248B310: 41980008  blt cr6, 0x8248b318
	if ctx.cr[6].lt {
	pc = 0x8248B318; continue 'dispatch;
	}
	// 8248B314: 4BE954F5  bl 0x82320808
	ctx.lr = 0x8248B318;
	sub_82320808(ctx, base);
	// 8248B318: 57EBE8FA  rlwinm r11, r31, 0x1d, 3, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000007u64;
	// 8248B31C: 57EA06FE  clrlwi r10, r31, 0x1b
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 8248B320: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8248B324: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8248B328: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8248B32C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8248B330: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8248B334: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8248B338: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8248B33C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248B340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248B344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248B348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248B34C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248B350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248B358 size=104
    let mut pc: u32 = 0x8248B358;
    'dispatch: loop {
        match pc {
            0x8248B358 => {
    //   block [0x8248B358..0x8248B3C0)
	// 8248B358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248B35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248B360: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248B364: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248B368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248B36C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248B370: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248B374: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B37C: 419A001C  beq cr6, 0x8248b398
	if ctx.cr[6].eq {
	pc = 0x8248B398; continue 'dispatch;
	}
	// 8248B380: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B384: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 8248B388: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8248B38C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8248B390: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8248B394: 41990008  bgt cr6, 0x8248b39c
	if ctx.cr[6].gt {
	pc = 0x8248B39C; continue 'dispatch;
	}
	// 8248B398: 4BFFF4E1  bl 0x8248a878
	ctx.lr = 0x8248B39C;
	sub_8248A878(ctx, base);
	// 8248B39C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B3A0: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 8248B3A4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248B3A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248B3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248B3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248B3B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248B3B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248B3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248B3C0 size=132
    let mut pc: u32 = 0x8248B3C0;
    'dispatch: loop {
        match pc {
            0x8248B3C0 => {
    //   block [0x8248B3C0..0x8248B444)
	// 8248B3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248B3C4: 48D1CDA5  bl 0x831a8168
	ctx.lr = 0x8248B3C8;
	sub_831A8130(ctx, base);
	// 8248B3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248B3CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248B3D0: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8248B3D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248B3D8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8248B3DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B3E0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B3E4: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248B3E8: 409A0044  bne cr6, 0x8248b42c
	if !ctx.cr[6].eq {
	pc = 0x8248B42C; continue 'dispatch;
	}
	// 8248B3EC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B3F0: 409A003C  bne cr6, 0x8248b42c
	if !ctx.cr[6].eq {
	pc = 0x8248B42C; continue 'dispatch;
	}
	// 8248B3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248B3F8: 4BFFF271  bl 0x8248a668
	ctx.lr = 0x8248B3FC;
	sub_8248A668(ctx, base);
	// 8248B3FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B400: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B404: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248B408: 48000030  b 0x8248b438
	pc = 0x8248B438; continue 'dispatch;
	// 8248B40C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8248B410: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8248B414: 4BFFEFBD  bl 0x8248a3d0
	ctx.lr = 0x8248B418;
	sub_8248A3D0(ctx, base);
	// 8248B418: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8248B41C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248B420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248B424: 4BFFF6DD  bl 0x8248ab00
	ctx.lr = 0x8248B428;
	sub_8248AB00(ctx, base);
	// 8248B428: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8248B42C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8248B430: 409AFFDC  bne cr6, 0x8248b40c
	if !ctx.cr[6].eq {
	pc = 0x8248B40C; continue 'dispatch;
	}
	// 8248B434: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8248B438: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248B43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248B440: 48D1CD78  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248B448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248B448 size=1128
    let mut pc: u32 = 0x8248B448;
    'dispatch: loop {
        match pc {
            0x8248B448 => {
    //   block [0x8248B448..0x8248B8B0)
	// 8248B448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248B44C: 48D1CD11  bl 0x831a815c
	ctx.lr = 0x8248B450;
	sub_831A8130(ctx, base);
	// 8248B450: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248B454: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248B458: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248B45C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8248B460: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8248B464: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8248B468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248B46C: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B470: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 8248B474: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248B478: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248B47C: 4200FFF0  bdnz 0x8248b46c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B46C; continue 'dispatch;
	}
	// 8248B480: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B484: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8248B488: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248B48C: 409A000C  bne cr6, 0x8248b498
	if !ctx.cr[6].eq {
	pc = 0x8248B498; continue 'dispatch;
	}
	// 8248B490: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8248B494: 48000010  b 0x8248b4a4
	pc = 0x8248B4A4; continue 'dispatch;
	// 8248B498: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248B49C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8248B4A0: 7D0BCBD6  divw r8, r11, r25
	ctx.r[8].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 8248B4A4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8248B4A8: 419A0400  beq cr6, 0x8248b8a8
	if ctx.cr[6].eq {
	pc = 0x8248B8A8; continue 'dispatch;
	}
	// 8248B4AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248B4B0: 409A000C  bne cr6, 0x8248b4bc
	if !ctx.cr[6].eq {
	pc = 0x8248B4BC; continue 'dispatch;
	}
	// 8248B4B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248B4B8: 48000010  b 0x8248b4c8
	pc = 0x8248B4C8; continue 'dispatch;
	// 8248B4BC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B4C0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8248B4C4: 7D6BCBD6  divw r11, r11, r25
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 8248B4C8: 3D400CCC  lis r10, 0xccc
	ctx.r[10].s64 = 214695936;
	// 8248B4CC: 614ACCCC  ori r10, r10, 0xcccc
	ctx.r[10].u64 = ctx.r[10].u64 | 52428;
	// 8248B4D0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8248B4D4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248B4D8: 4098000C  bge cr6, 0x8248b4e4
	if !ctx.cr[6].lt {
	pc = 0x8248B4E4; continue 'dispatch;
	}
	// 8248B4DC: 4872C845  bl 0x82bb7d20
	ctx.lr = 0x8248B4E0;
	sub_82BB7D20(ctx, base);
	// 8248B4E0: 480003C8  b 0x8248b8a8
	pc = 0x8248B8A8; continue 'dispatch;
	// 8248B4E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248B4E8: 409A000C  bne cr6, 0x8248b4f4
	if !ctx.cr[6].eq {
	pc = 0x8248B4F4; continue 'dispatch;
	}
	// 8248B4EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248B4F0: 48000010  b 0x8248b500
	pc = 0x8248B500; continue 'dispatch;
	// 8248B4F4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B4F8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8248B4FC: 7D6BCBD6  divw r11, r11, r25
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 8248B500: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8248B504: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B508: 409801C8  bge cr6, 0x8248b6d0
	if !ctx.cr[6].lt {
	pc = 0x8248B6D0; continue 'dispatch;
	}
	// 8248B50C: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8248B510: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8248B514: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248B518: 4098000C  bge cr6, 0x8248b524
	if !ctx.cr[6].lt {
	pc = 0x8248B524; continue 'dispatch;
	}
	// 8248B51C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248B520: 48000008  b 0x8248b528
	pc = 0x8248B528; continue 'dispatch;
	// 8248B524: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8248B528: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248B52C: 409A000C  bne cr6, 0x8248b538
	if !ctx.cr[6].eq {
	pc = 0x8248B538; continue 'dispatch;
	}
	// 8248B530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8248B534: 48000010  b 0x8248b544
	pc = 0x8248B544; continue 'dispatch;
	// 8248B538: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B53C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8248B540: 7D4ACBD6  divw r10, r10, r25
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[25].s32;
	// 8248B544: 7D4AD214  add r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 8248B548: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248B54C: 40980024  bge cr6, 0x8248b570
	if !ctx.cr[6].lt {
	pc = 0x8248B570; continue 'dispatch;
	}
	// 8248B550: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248B554: 409A000C  bne cr6, 0x8248b560
	if !ctx.cr[6].eq {
	pc = 0x8248B560; continue 'dispatch;
	}
	// 8248B558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248B55C: 48000010  b 0x8248b56c
	pc = 0x8248B56C; continue 'dispatch;
	// 8248B560: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B564: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8248B568: 7D6BCBD6  divw r11, r11, r25
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 8248B56C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8248B570: 1F6B0014  mulli r27, r11, 0x14
	ctx.r[27].s64 = ctx.r[11].s64 * 20;
	// 8248B574: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 8248B578: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8248B57C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8248B580: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 8248B584: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8248B588: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248B58C: 48966B3D  bl 0x82df20c8
	ctx.lr = 0x8248B590;
	sub_82DF20C8(ctx, base);
	// 8248B590: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248B594: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B598: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8248B59C: 48000038  b 0x8248b5d4
	pc = 0x8248B5D4; continue 'dispatch;
	// 8248B5A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248B5A4: 419A0028  beq cr6, 0x8248b5cc
	if ctx.cr[6].eq {
	pc = 0x8248B5CC; continue 'dispatch;
	}
	// 8248B5A8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8248B5AC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8248B5B0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8248B5B4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8248B5B8: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B5BC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B5C0: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8248B5C4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8248B5C8: 4200FFF0  bdnz 0x8248b5b8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B5B8; continue 'dispatch;
	}
	// 8248B5CC: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8248B5D0: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8248B5D4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8248B5D8: 409AFFC8  bne cr6, 0x8248b5a0
	if !ctx.cr[6].eq {
	pc = 0x8248B5A0; continue 'dispatch;
	}
	// 8248B5DC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8248B5E0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8248B5E4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8248B5E8: 419A003C  beq cr6, 0x8248b624
	if ctx.cr[6].eq {
	pc = 0x8248B624; continue 'dispatch;
	}
	// 8248B5EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B5F0: 419A0028  beq cr6, 0x8248b618
	if ctx.cr[6].eq {
	pc = 0x8248B618; continue 'dispatch;
	}
	// 8248B5F4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8248B5F8: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8248B5FC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8248B600: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8248B604: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B608: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B60C: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8248B610: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8248B614: 4200FFF0  bdnz 0x8248b604
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B604; continue 'dispatch;
	}
	// 8248B618: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8248B61C: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8248B620: 4082FFCC  bne 0x8248b5ec
	if !ctx.cr[0].eq {
	pc = 0x8248B5EC; continue 'dispatch;
	}
	// 8248B624: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B628: 1D3A0014  mulli r9, r26, 0x14
	ctx.r[9].s64 = ctx.r[26].s64 * 20;
	// 8248B62C: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8248B630: 7F1F3040  cmplw cr6, r31, r6
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8248B634: 419A004C  beq cr6, 0x8248b680
	if ctx.cr[6].eq {
	pc = 0x8248B680; continue 'dispatch;
	}
	// 8248B638: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8248B63C: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 8248B640: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 8248B644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B648: 419A0028  beq cr6, 0x8248b670
	if ctx.cr[6].eq {
	pc = 0x8248B670; continue 'dispatch;
	}
	// 8248B64C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8248B650: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8248B654: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8248B658: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8248B65C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B660: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B664: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8248B668: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8248B66C: 4200FFF0  bdnz 0x8248b65c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B65C; continue 'dispatch;
	}
	// 8248B670: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8248B674: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8248B678: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8248B67C: 409AFFC8  bne cr6, 0x8248b644
	if !ctx.cr[6].eq {
	pc = 0x8248B644; continue 'dispatch;
	}
	// 8248B680: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B684: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8248B688: 409A000C  bne cr6, 0x8248b694
	if !ctx.cr[6].eq {
	pc = 0x8248B694; continue 'dispatch;
	}
	// 8248B68C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248B690: 48000010  b 0x8248b6a0
	pc = 0x8248B6A0; continue 'dispatch;
	// 8248B694: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B698: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 8248B69C: 7D6BCBD6  divw r11, r11, r25
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 8248B6A0: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8248B6A4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8248B6A8: 419A000C  beq cr6, 0x8248b6b4
	if ctx.cr[6].eq {
	pc = 0x8248B6B4; continue 'dispatch;
	}
	// 8248B6AC: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248B6B0: 48966AD9  bl 0x82df2188
	ctx.lr = 0x8248B6B4;
	sub_82DF2188(ctx, base);
	// 8248B6B4: 1D7F0014  mulli r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 * 20;
	// 8248B6B8: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8248B6BC: 7D5BF214  add r10, r27, r30
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 8248B6C0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8248B6C4: 915D000C  stw r10, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8248B6C8: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248B6CC: 480001DC  b 0x8248b8a8
	pc = 0x8248B8A8; continue 'dispatch;
	// 8248B6D0: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B6D4: 7D7F4050  subf r11, r31, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[31].s64;
	// 8248B6D8: 7D6BCBD6  divw r11, r11, r25
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 8248B6DC: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8248B6E0: 409800F0  bge cr6, 0x8248b7d0
	if !ctx.cr[6].lt {
	pc = 0x8248B7D0; continue 'dispatch;
	}
	// 8248B6E4: 1CBA0014  mulli r5, r26, 0x14
	ctx.r[5].s64 = ctx.r[26].s64 * 20;
	// 8248B6E8: 7D65FA14  add r11, r5, r31
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[31].u64;
	// 8248B6EC: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248B6F0: 419A0044  beq cr6, 0x8248b734
	if ctx.cr[6].eq {
	pc = 0x8248B734; continue 'dispatch;
	}
	// 8248B6F4: 7D455850  subf r10, r5, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 8248B6F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B6FC: 419A0028  beq cr6, 0x8248b724
	if ctx.cr[6].eq {
	pc = 0x8248B724; continue 'dispatch;
	}
	// 8248B700: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8248B704: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 8248B708: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8248B70C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8248B710: 80C90000  lwz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B714: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B718: 90C70000  stw r6, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8248B71C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8248B720: 4200FFF0  bdnz 0x8248b710
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B710; continue 'dispatch;
	}
	// 8248B724: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8248B728: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8248B72C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248B730: 409AFFC8  bne cr6, 0x8248b6f8
	if !ctx.cr[6].eq {
	pc = 0x8248B6F8; continue 'dispatch;
	}
	// 8248B734: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B738: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 8248B73C: 7D4ACBD6  divw r10, r10, r25
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[25].s32;
	// 8248B740: 7D4AD051  subf. r10, r10, r26
	ctx.r[10].s64 = ctx.r[26].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248B744: 4182003C  beq 0x8248b780
	if ctx.cr[0].eq {
	pc = 0x8248B780; continue 'dispatch;
	}
	// 8248B748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248B74C: 419A0028  beq cr6, 0x8248b774
	if ctx.cr[6].eq {
	pc = 0x8248B774; continue 'dispatch;
	}
	// 8248B750: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8248B754: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8248B758: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8248B75C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8248B760: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B764: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B768: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8248B76C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8248B770: 4200FFF0  bdnz 0x8248b760
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B760; continue 'dispatch;
	}
	// 8248B774: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248B778: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8248B77C: 4082FFCC  bne 0x8248b748
	if !ctx.cr[0].eq {
	pc = 0x8248B748; continue 'dispatch;
	}
	// 8248B780: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248B784: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8248B788: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 8248B78C: 7CE55850  subf r7, r5, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 8248B790: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248B794: 7F1F3840  cmplw cr6, r31, r7
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8248B798: 419A0110  beq cr6, 0x8248b8a8
	if ctx.cr[6].eq {
	pc = 0x8248B8A8; continue 'dispatch;
	}
	// 8248B79C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8248B7A0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8248B7A4: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8248B7A8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8248B7AC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B7B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248B7B4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8248B7B8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B7BC: 4200FFF0  bdnz 0x8248b7ac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B7AC; continue 'dispatch;
	}
	// 8248B7C0: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8248B7C4: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8248B7C8: 409AFFD4  bne cr6, 0x8248b79c
	if !ctx.cr[6].eq {
	pc = 0x8248B79C; continue 'dispatch;
	}
	// 8248B7CC: 480000DC  b 0x8248b8a8
	pc = 0x8248B8A8; continue 'dispatch;
	// 8248B7D0: 1C9A0014  mulli r4, r26, 0x14
	ctx.r[4].s64 = ctx.r[26].s64 * 20;
	// 8248B7D4: 7D644050  subf r11, r4, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 8248B7D8: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8248B7DC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8248B7E0: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248B7E4: 419A0040  beq cr6, 0x8248b824
	if ctx.cr[6].eq {
	pc = 0x8248B824; continue 'dispatch;
	}
	// 8248B7E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248B7EC: 419A0028  beq cr6, 0x8248b814
	if ctx.cr[6].eq {
	pc = 0x8248B814; continue 'dispatch;
	}
	// 8248B7F0: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 8248B7F4: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8248B7F8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8248B7FC: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 8248B800: 80A70000  lwz r5, 0(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B804: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8248B808: 90A60000  stw r5, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8248B80C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 8248B810: 4200FFF0  bdnz 0x8248b800
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B800; continue 'dispatch;
	}
	// 8248B814: 39290014  addi r9, r9, 0x14
	ctx.r[9].s64 = ctx.r[9].s64 + 20;
	// 8248B818: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8248B81C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248B820: 409AFFC8  bne cr6, 0x8248b7e8
	if !ctx.cr[6].eq {
	pc = 0x8248B7E8; continue 'dispatch;
	}
	// 8248B824: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248B828: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B82C: 419A003C  beq cr6, 0x8248b868
	if ctx.cr[6].eq {
	pc = 0x8248B868; continue 'dispatch;
	}
	// 8248B830: 7D445A14  add r10, r4, r11
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8248B834: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 8248B838: 394AFFEC  addi r10, r10, -0x14
	ctx.r[10].s64 = ctx.r[10].s64 + -20;
	// 8248B83C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8248B840: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8248B844: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8248B848: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8248B84C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B850: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B854: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8248B858: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8248B85C: 4200FFF0  bdnz 0x8248b84c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B84C; continue 'dispatch;
	}
	// 8248B860: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8248B864: 409AFFD0  bne cr6, 0x8248b834
	if !ctx.cr[6].eq {
	pc = 0x8248B834; continue 'dispatch;
	}
	// 8248B868: 7CE4FA14  add r7, r4, r31
	ctx.r[7].u64 = ctx.r[4].u64 + ctx.r[31].u64;
	// 8248B86C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8248B870: 7F1F3840  cmplw cr6, r31, r7
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8248B874: 419A0034  beq cr6, 0x8248b8a8
	if ctx.cr[6].eq {
	pc = 0x8248B8A8; continue 'dispatch;
	}
	// 8248B878: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8248B87C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8248B880: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8248B884: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8248B888: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B88C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248B890: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8248B894: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248B898: 4200FFF0  bdnz 0x8248b888
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248B888; continue 'dispatch;
	}
	// 8248B89C: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8248B8A0: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8248B8A4: 409AFFD4  bne cr6, 0x8248b878
	if !ctx.cr[6].eq {
	pc = 0x8248B878; continue 'dispatch;
	}
	// 8248B8A8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8248B8AC: 48D1C900  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248B8B0 size=88
    let mut pc: u32 = 0x8248B8B0;
    'dispatch: loop {
        match pc {
            0x8248B8B0 => {
    //   block [0x8248B8B0..0x8248B908)
	// 8248B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248B8B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248B8BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248B8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248B8C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248B8C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248B8CC: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B8D0: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B8D4: 4BFFFAED  bl 0x8248b3c0
	ctx.lr = 0x8248B8D8;
	sub_8248B3C0(ctx, base);
	// 8248B8D8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248B8DC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B8E0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248B8E4: 489668A5  bl 0x82df2188
	ctx.lr = 0x8248B8E8;
	sub_82DF2188(ctx, base);
	// 8248B8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248B8EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8248B8F0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248B8F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248B8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248B8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248B900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248B904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248B908 size=132
    let mut pc: u32 = 0x8248B908;
    'dispatch: loop {
        match pc {
            0x8248B908 => {
    //   block [0x8248B908..0x8248B98C)
	// 8248B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248B90C: 48D1C85D  bl 0x831a8168
	ctx.lr = 0x8248B910;
	sub_831A8130(ctx, base);
	// 8248B910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248B914: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248B918: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8248B91C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248B920: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8248B924: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B928: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B92C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248B930: 409A0044  bne cr6, 0x8248b974
	if !ctx.cr[6].eq {
	pc = 0x8248B974; continue 'dispatch;
	}
	// 8248B934: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248B938: 409A003C  bne cr6, 0x8248b974
	if !ctx.cr[6].eq {
	pc = 0x8248B974; continue 'dispatch;
	}
	// 8248B93C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248B940: 4BFFEE89  bl 0x8248a7c8
	ctx.lr = 0x8248B944;
	sub_8248A7C8(ctx, base);
	// 8248B944: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248B948: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B94C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248B950: 48000030  b 0x8248b980
	pc = 0x8248B980; continue 'dispatch;
	// 8248B954: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8248B958: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8248B95C: 4805D735  bl 0x824e9090
	ctx.lr = 0x8248B960;
	sub_824E9090(ctx, base);
	// 8248B960: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8248B964: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248B968: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248B96C: 4BFFF58D  bl 0x8248aef8
	ctx.lr = 0x8248B970;
	sub_8248AEF8(ctx, base);
	// 8248B970: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8248B974: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8248B978: 409AFFDC  bne cr6, 0x8248b954
	if !ctx.cr[6].eq {
	pc = 0x8248B954; continue 'dispatch;
	}
	// 8248B97C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8248B980: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248B984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248B988: 48D1C830  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248B990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8248B990 size=388
    let mut pc: u32 = 0x8248B990;
    'dispatch: loop {
        match pc {
            0x8248B990 => {
    //   block [0x8248B990..0x8248BB14)
	// 8248B990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248B994: 48D1C7D5  bl 0x831a8168
	ctx.lr = 0x8248B998;
	sub_831A8130(ctx, base);
	// 8248B998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248B99C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248B9A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248B9A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248B9A8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248B9AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248B9B0: 419A0014  beq cr6, 0x8248b9c4
	if ctx.cr[6].eq {
	pc = 0x8248B9C4; continue 'dispatch;
	}
	// 8248B9B4: 7D6A5810  subfc r11, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8248B9B8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8248B9BC: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8248B9C0: 480000AC  b 0x8248ba6c
	pc = 0x8248BA6C; continue 'dispatch;
	// 8248B9C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248B9C8: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8248B9CC: 7D3EF850  subf r9, r30, r31
	ctx.r[9].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 8248B9D0: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248B9D4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248B9D8: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8248B9DC: 409A0098  bne cr6, 0x8248ba74
	if !ctx.cr[6].eq {
	pc = 0x8248BA74; continue 'dispatch;
	}
	// 8248B9E0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248B9E4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248B9E8: 4080FFE8  bge 0x8248b9d0
	if !ctx.cr[0].lt {
	pc = 0x8248B9D0; continue 'dispatch;
	}
	// 8248B9EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248B9F0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248B9F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248B9F8: 40820094  bne 0x8248ba8c
	if !ctx.cr[0].eq {
	pc = 0x8248BA8C; continue 'dispatch;
	}
	// 8248B9FC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8248BA00: 7D3FF050  subf r9, r31, r30
	ctx.r[9].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8248BA04: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BA08: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248BA0C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8248BA10: 409A006C  bne cr6, 0x8248ba7c
	if !ctx.cr[6].eq {
	pc = 0x8248BA7C; continue 'dispatch;
	}
	// 8248BA14: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248BA18: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248BA1C: 4080FFE8  bge 0x8248ba04
	if !ctx.cr[0].lt {
	pc = 0x8248BA04; continue 'dispatch;
	}
	// 8248BA20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BA24: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BA28: 40820040  bne 0x8248ba68
	if !ctx.cr[0].eq {
	pc = 0x8248BA68; continue 'dispatch;
	}
	// 8248BA2C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8248BA30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8248BA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248BA38: 4BFFF8B9  bl 0x8248b2f0
	ctx.lr = 0x8248BA3C;
	sub_8248B2F0(ctx, base);
	// 8248BA3C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8248BA40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8248BA44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248BA48: 4BFFF8A9  bl 0x8248b2f0
	ctx.lr = 0x8248BA4C;
	sub_8248B2F0(ctx, base);
	// 8248BA4C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8248BA50: 578A063E  clrlwi r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8248BA54: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BA58: 409A002C  bne cr6, 0x8248ba84
	if !ctx.cr[6].eq {
	pc = 0x8248BA84; continue 'dispatch;
	}
	// 8248BA5C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8248BA60: 2F1D0040  cmpwi cr6, r29, 0x40
	ctx.cr[6].compare_i32(ctx.r[29].s32, 64, &mut ctx.xer);
	// 8248BA64: 4198FFCC  blt cr6, 0x8248ba30
	if ctx.cr[6].lt {
	pc = 0x8248BA30; continue 'dispatch;
	}
	// 8248BA68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8248BA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248BA70: 48D1C748  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8248BA74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BA78: 4BFFFF78  b 0x8248b9f0
	pc = 0x8248B9F0; continue 'dispatch;
	// 8248BA7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BA80: 4BFFFFA4  b 0x8248ba24
	pc = 0x8248BA24; continue 'dispatch;
	// 8248BA84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8248BA88: 4BFFFFE4  b 0x8248ba6c
	pc = 0x8248BA6C; continue 'dispatch;
	// 8248BA8C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8248BA90: 3B9E0008  addi r28, r30, 8
	ctx.r[28].s64 = ctx.r[30].s64 + 8;
	// 8248BA94: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8248BA98: 7D3FE050  subf r9, r31, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 8248BA9C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BAA0: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8248BAA4: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8248BAA8: 409A005C  bne cr6, 0x8248bb04
	if !ctx.cr[6].eq {
	pc = 0x8248BB04; continue 'dispatch;
	}
	// 8248BAAC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248BAB0: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248BAB4: 4080FFE8  bge 0x8248ba9c
	if !ctx.cr[0].lt {
	pc = 0x8248BA9C; continue 'dispatch;
	}
	// 8248BAB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BABC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BAC0: 4082FFA8  bne 0x8248ba68
	if !ctx.cr[0].eq {
	pc = 0x8248BA68; continue 'dispatch;
	}
	// 8248BAC4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8248BAC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248BACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248BAD0: 4BFFF821  bl 0x8248b2f0
	ctx.lr = 0x8248BAD4;
	sub_8248B2F0(ctx, base);
	// 8248BAD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248BAD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248BADC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8248BAE0: 4BFFF811  bl 0x8248b2f0
	ctx.lr = 0x8248BAE4;
	sub_8248B2F0(ctx, base);
	// 8248BAE4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8248BAE8: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8248BAEC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BAF0: 409A001C  bne cr6, 0x8248bb0c
	if !ctx.cr[6].eq {
	pc = 0x8248BB0C; continue 'dispatch;
	}
	// 8248BAF4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8248BAF8: 2F1E0040  cmpwi cr6, r30, 0x40
	ctx.cr[6].compare_i32(ctx.r[30].s32, 64, &mut ctx.xer);
	// 8248BAFC: 4198FFCC  blt cr6, 0x8248bac8
	if ctx.cr[6].lt {
	pc = 0x8248BAC8; continue 'dispatch;
	}
	// 8248BB00: 4BFFFF68  b 0x8248ba68
	pc = 0x8248BA68; continue 'dispatch;
	// 8248BB04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BB08: 4BFFFFB4  b 0x8248babc
	pc = 0x8248BABC; continue 'dispatch;
	// 8248BB0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248BB10: 4BFFFF5C  b 0x8248ba6c
	pc = 0x8248BA6C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248BB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248BB18 size=300
    let mut pc: u32 = 0x8248BB18;
    'dispatch: loop {
        match pc {
            0x8248BB18 => {
    //   block [0x8248BB18..0x8248BC44)
	// 8248BB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248BB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248BB20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248BB24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248BB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248BB2C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248BB30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248BB34: 816B6668  lwz r11, 0x6668(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26216 as u32) ) } as u64;
	// 8248BB38: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BB3C: 409A000C  bne cr6, 0x8248bb48
	if !ctx.cr[6].eq {
	pc = 0x8248BB48; continue 'dispatch;
	}
	// 8248BB40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8248BB44: 480000E0  b 0x8248bc24
	pc = 0x8248BC24; continue 'dispatch;
	// 8248BB48: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248BB4C: 816B666C  lwz r11, 0x666c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26220 as u32) ) } as u64;
	// 8248BB50: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BB54: 419A00CC  beq cr6, 0x8248bc20
	if ctx.cr[6].eq {
	pc = 0x8248BC20; continue 'dispatch;
	}
	// 8248BB58: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 8248BB5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248BB60: 807E6674  lwz r3, 0x6674(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248BB64: 4BFFF7F5  bl 0x8248b358
	ctx.lr = 0x8248BB68;
	sub_8248B358(ctx, base);
	// 8248BB68: 817E6674  lwz r11, 0x6674(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248BB6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248BB70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248BB74: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8248BB78: 4BFFF7E1  bl 0x8248b358
	ctx.lr = 0x8248BB7C;
	sub_8248B358(ctx, base);
	// 8248BB7C: E95E0008  ld r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 8248BB80: E9230000  ld r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8248BB84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BB88: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8248BB8C: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8248BB90: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8248BB94: 3921005C  addi r9, r1, 0x5c
	ctx.r[9].s64 = ctx.r[1].s64 + 92;
	// 8248BB98: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248BB9C: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8248BBA0: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248BBA4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BBA8: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8248BBAC: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 8248BBB0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248BBB4: 4098FFDC  bge cr6, 0x8248bb90
	if !ctx.cr[6].lt {
	pc = 0x8248BB90; continue 'dispatch;
	}
	// 8248BBB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BBBC: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8248BBC0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BBC4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248BBC8: 409A0074  bne cr6, 0x8248bc3c
	if !ctx.cr[6].eq {
	pc = 0x8248BC3C; continue 'dispatch;
	}
	// 8248BBCC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BBD0: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8248BBD4: 4080FFEC  bge 0x8248bbc0
	if !ctx.cr[0].lt {
	pc = 0x8248BBC0; continue 'dispatch;
	}
	// 8248BBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BBDC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BBE0: 41820040  beq 0x8248bc20
	if ctx.cr[0].eq {
	pc = 0x8248BC20; continue 'dispatch;
	}
	// 8248BBE4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248BBE8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248BBEC: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8248BBF0: 5548843E  srwi r8, r10, 0x10
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8248BBF4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248BBF8: 409AFF48  bne cr6, 0x8248bb40
	if !ctx.cr[6].eq {
	pc = 0x8248BB40; continue 'dispatch;
	}
	// 8248BBFC: 5549C63E  rlwinm r9, r10, 0x18, 0x18, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8248BC00: 5568063E  clrlwi r8, r11, 0x18
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8248BC04: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248BC08: 419A0018  beq cr6, 0x8248bc20
	if ctx.cr[6].eq {
	pc = 0x8248BC20; continue 'dispatch;
	}
	// 8248BC0C: 556BC63E  rlwinm r11, r11, 0x18, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8248BC10: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8248BC14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8248BC18: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248BC1C: 409A0008  bne cr6, 0x8248bc24
	if !ctx.cr[6].eq {
	pc = 0x8248BC24; continue 'dispatch;
	}
	// 8248BC20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8248BC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248BC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248BC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248BC30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248BC34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248BC38: 4E800020  blr
	return;
	// 8248BC3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BC40: 4BFFFF9C  b 0x8248bbdc
	pc = 0x8248BBDC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248BC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248BC48 size=520
    let mut pc: u32 = 0x8248BC48;
    'dispatch: loop {
        match pc {
            0x8248BC48 => {
    //   block [0x8248BC48..0x8248BE50)
	// 8248BC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248BC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248BC50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248BC54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248BC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248BC5C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248BC60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248BC64: 816B6668  lwz r11, 0x6668(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26216 as u32) ) } as u64;
	// 8248BC68: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BC6C: 419A01C8  beq cr6, 0x8248be34
	if ctx.cr[6].eq {
	pc = 0x8248BE34; continue 'dispatch;
	}
	// 8248BC70: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BC74: 419A01C0  beq cr6, 0x8248be34
	if ctx.cr[6].eq {
	pc = 0x8248BE34; continue 'dispatch;
	}
	// 8248BC78: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248BC7C: 816B666C  lwz r11, 0x666c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26220 as u32) ) } as u64;
	// 8248BC80: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BC84: 419A01A8  beq cr6, 0x8248be2c
	if ctx.cr[6].eq {
	pc = 0x8248BE2C; continue 'dispatch;
	}
	// 8248BC88: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248BC8C: 419A01A0  beq cr6, 0x8248be2c
	if ctx.cr[6].eq {
	pc = 0x8248BE2C; continue 'dispatch;
	}
	// 8248BC90: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 8248BC94: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248BC98: 807E6674  lwz r3, 0x6674(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248BC9C: 4BFFF6BD  bl 0x8248b358
	ctx.lr = 0x8248BCA0;
	sub_8248B358(ctx, base);
	// 8248BCA0: 817E6674  lwz r11, 0x6674(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248BCA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248BCA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248BCAC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8248BCB0: 4BFFF6A9  bl 0x8248b358
	ctx.lr = 0x8248BCB4;
	sub_8248B358(ctx, base);
	// 8248BCB4: E9030000  ld r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8248BCB8: 395E0008  addi r10, r30, 8
	ctx.r[10].s64 = ctx.r[30].s64 + 8;
	// 8248BCBC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8248BCC0: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8248BCC4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248BCC8: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8248BCCC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BCD0: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8248BCD4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BCD8: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8248BCDC: 7CE84038  and r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 & ctx.r[8].u64;
	// 8248BCE0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8248BCE4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248BCE8: 4080FFE4  bge 0x8248bccc
	if !ctx.cr[0].lt {
	pc = 0x8248BCCC; continue 'dispatch;
	}
	// 8248BCEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248BCF0: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8248BCF4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BCF8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248BCFC: 409A003C  bne cr6, 0x8248bd38
	if !ctx.cr[6].eq {
	pc = 0x8248BD38; continue 'dispatch;
	}
	// 8248BD00: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248BD04: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248BD08: 4080FFEC  bge 0x8248bcf4
	if !ctx.cr[0].lt {
	pc = 0x8248BCF4; continue 'dispatch;
	}
	// 8248BD0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BD10: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BD14: 4182005C  beq 0x8248bd70
	if ctx.cr[0].eq {
	pc = 0x8248BD70; continue 'dispatch;
	}
	// 8248BD18: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248BD1C: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248BD20: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8248BD24: 5548843E  srwi r8, r10, 0x10
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8248BD28: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248BD2C: 419A0014  beq cr6, 0x8248bd40
	if ctx.cr[6].eq {
	pc = 0x8248BD40; continue 'dispatch;
	}
	// 8248BD30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BD34: 48000034  b 0x8248bd68
	pc = 0x8248BD68; continue 'dispatch;
	// 8248BD38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BD3C: 4BFFFFD4  b 0x8248bd10
	pc = 0x8248BD10; continue 'dispatch;
	// 8248BD40: 5549C63E  rlwinm r9, r10, 0x18, 0x18, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8248BD44: 5568063E  clrlwi r8, r11, 0x18
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8248BD48: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248BD4C: 419A0018  beq cr6, 0x8248bd64
	if ctx.cr[6].eq {
	pc = 0x8248BD64; continue 'dispatch;
	}
	// 8248BD50: 556BC63E  rlwinm r11, r11, 0x18, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8248BD54: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8248BD58: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248BD5C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BD60: 409A0008  bne cr6, 0x8248bd68
	if !ctx.cr[6].eq {
	pc = 0x8248BD68; continue 'dispatch;
	}
	// 8248BD64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BD68: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BD6C: 408200C8  bne 0x8248be34
	if !ctx.cr[0].eq {
	pc = 0x8248BE34; continue 'dispatch;
	}
	// 8248BD70: E91E0000  ld r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8248BD74: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 8248BD78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248BD7C: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8248BD80: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8248BD84: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8248BD88: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BD8C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248BD90: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BD94: 3929FFFC  addi r9, r9, -4
	ctx.r[9].s64 = ctx.r[9].s64 + -4;
	// 8248BD98: 7D083838  and r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[7].u64;
	// 8248BD9C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8248BDA0: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248BDA4: 4080FFE4  bge 0x8248bd88
	if !ctx.cr[0].lt {
	pc = 0x8248BD88; continue 'dispatch;
	}
	// 8248BDA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BDAC: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8248BDB0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BDB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248BDB8: 409A003C  bne cr6, 0x8248bdf4
	if !ctx.cr[6].eq {
	pc = 0x8248BDF4; continue 'dispatch;
	}
	// 8248BDBC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BDC0: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8248BDC4: 4080FFEC  bge 0x8248bdb0
	if !ctx.cr[0].lt {
	pc = 0x8248BDB0; continue 'dispatch;
	}
	// 8248BDC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BDCC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BDD0: 4182005C  beq 0x8248be2c
	if ctx.cr[0].eq {
	pc = 0x8248BE2C; continue 'dispatch;
	}
	// 8248BDD4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248BDD8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248BDDC: 5569843E  srwi r9, r11, 0x10
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8248BDE0: 5548843E  srwi r8, r10, 0x10
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8248BDE4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248BDE8: 419A0014  beq cr6, 0x8248bdfc
	if ctx.cr[6].eq {
	pc = 0x8248BDFC; continue 'dispatch;
	}
	// 8248BDEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BDF0: 48000034  b 0x8248be24
	pc = 0x8248BE24; continue 'dispatch;
	// 8248BDF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BDF8: 4BFFFFD4  b 0x8248bdcc
	pc = 0x8248BDCC; continue 'dispatch;
	// 8248BDFC: 5549C63E  rlwinm r9, r10, 0x18, 0x18, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8248BE00: 5568063E  clrlwi r8, r11, 0x18
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8248BE04: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8248BE08: 419A0018  beq cr6, 0x8248be20
	if ctx.cr[6].eq {
	pc = 0x8248BE20; continue 'dispatch;
	}
	// 8248BE0C: 556BC63E  rlwinm r11, r11, 0x18, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8248BE10: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8248BE14: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248BE18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8248BE1C: 409A0008  bne cr6, 0x8248be24
	if !ctx.cr[6].eq {
	pc = 0x8248BE24; continue 'dispatch;
	}
	// 8248BE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BE24: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BE28: 4082000C  bne 0x8248be34
	if !ctx.cr[0].eq {
	pc = 0x8248BE34; continue 'dispatch;
	}
	// 8248BE2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8248BE30: 48000008  b 0x8248be38
	pc = 0x8248BE38; continue 'dispatch;
	// 8248BE34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8248BE38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248BE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248BE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248BE44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248BE48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248BE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248BE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248BE50 size=176
    let mut pc: u32 = 0x8248BE50;
    'dispatch: loop {
        match pc {
            0x8248BE50 => {
    //   block [0x8248BE50..0x8248BF00)
	// 8248BE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248BE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248BE58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248BE5C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248BE60: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8248BE64: F8A10080  std r5, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[5].u64 ) };
	// 8248BE68: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 8248BE6C: F8C10088  std r6, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[6].u64 ) };
	// 8248BE70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248BE74: F8E10090  std r7, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[7].u64 ) };
	// 8248BE78: 409A000C  bne cr6, 0x8248be84
	if !ctx.cr[6].eq {
	pc = 0x8248BE84; continue 'dispatch;
	}
	// 8248BE7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8248BE80: 48000010  b 0x8248be90
	pc = 0x8248BE90; continue 'dispatch;
	// 8248BE84: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248BE88: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8248BE8C: 7D4A43D6  divw r10, r10, r8
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[8].s32;
	// 8248BE90: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248BE94: 4098002C  bge cr6, 0x8248bec0
	if !ctx.cr[6].lt {
	pc = 0x8248BEC0; continue 'dispatch;
	}
	// 8248BE98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248BE9C: 419A0010  beq cr6, 0x8248beac
	if ctx.cr[6].eq {
	pc = 0x8248BEAC; continue 'dispatch;
	}
	// 8248BEA0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248BEA4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8248BEA8: 7D6B43D6  divw r11, r11, r8
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 8248BEAC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8248BEB0: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248BEB4: 7CAB4850  subf r5, r11, r9
	ctx.r[5].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8248BEB8: 4BFFF591  bl 0x8248b448
	ctx.lr = 0x8248BEBC;
	sub_8248B448(ctx, base);
	// 8248BEBC: 48000034  b 0x8248bef0
	pc = 0x8248BEF0; continue 'dispatch;
	// 8248BEC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248BEC4: 419A002C  beq cr6, 0x8248bef0
	if ctx.cr[6].eq {
	pc = 0x8248BEF0; continue 'dispatch;
	}
	// 8248BEC8: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248BECC: 7D4B3050  subf r10, r11, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 8248BED0: 7D4A43D6  divw r10, r10, r8
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[8].s32;
	// 8248BED4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248BED8: 40980018  bge cr6, 0x8248bef0
	if !ctx.cr[6].lt {
	pc = 0x8248BEF0; continue 'dispatch;
	}
	// 8248BEDC: 1D490014  mulli r10, r9, 0x14
	ctx.r[10].s64 = ctx.r[9].s64 * 20;
	// 8248BEE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248BEE4: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8248BEE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248BEEC: 4803FACD  bl 0x824cb9b8
	ctx.lr = 0x8248BEF0;
	sub_824CB9B8(ctx, base);
	// 8248BEF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248BEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248BEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248BEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248BF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248BF00 size=112
    let mut pc: u32 = 0x8248BF00;
    'dispatch: loop {
        match pc {
            0x8248BF00 => {
    //   block [0x8248BF00..0x8248BF70)
	// 8248BF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248BF04: 48D1C269  bl 0x831a816c
	ctx.lr = 0x8248BF08;
	sub_831A8130(ctx, base);
	// 8248BF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248BF0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248BF10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248BF14: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8248BF18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248BF1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248BF20: 419A0018  beq cr6, 0x8248bf38
	if ctx.cr[6].eq {
	pc = 0x8248BF38; continue 'dispatch;
	}
	// 8248BF24: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248BF28: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 8248BF2C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8248BF30: 7D2953D7  divw. r9, r9, r10
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8248BF34: 4082000C  bne 0x8248bf40
	if !ctx.cr[0].eq {
	pc = 0x8248BF40; continue 'dispatch;
	}
	// 8248BF38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8248BF3C: 4800000C  b 0x8248bf48
	pc = 0x8248BF48; continue 'dispatch;
	// 8248BF40: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8248BF44: 7FCB53D6  divw r30, r11, r10
	ctx.r[30].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8248BF48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8248BF4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248BF50: 4BFFF4F9  bl 0x8248b448
	ctx.lr = 0x8248BF54;
	sub_8248B448(ctx, base);
	// 8248BF54: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248BF58: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 8248BF5C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248BF60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248BF64: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248BF68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248BF6C: 48D1C250  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248BF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248BF70 size=88
    let mut pc: u32 = 0x8248BF70;
    'dispatch: loop {
        match pc {
            0x8248BF70 => {
    //   block [0x8248BF70..0x8248BFC8)
	// 8248BF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248BF74: 48D1C1F9  bl 0x831a816c
	ctx.lr = 0x8248BF78;
	sub_831A8130(ctx, base);
	// 8248BF78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248BF7C: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248BF80: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8248BF84: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248BF88: 48000028  b 0x8248bfb0
	pc = 0x8248BFB0; continue 'dispatch;
	// 8248BF8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8248BF90: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8248BF94: 4BFFF9FD  bl 0x8248b990
	ctx.lr = 0x8248BF98;
	sub_8248B990(ctx, base);
	// 8248BF98: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248BF9C: 4182000C  beq 0x8248bfa8
	if ctx.cr[0].eq {
	pc = 0x8248BFA8; continue 'dispatch;
	}
	// 8248BFA0: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248BFA4: 4800000C  b 0x8248bfb0
	pc = 0x8248BFB0; continue 'dispatch;
	// 8248BFA8: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8248BFAC: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BFB0: 897F0025  lbz r11, 0x25(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248BFB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248BFB8: 419AFFD4  beq cr6, 0x8248bf8c
	if ctx.cr[6].eq {
	pc = 0x8248BF8C; continue 'dispatch;
	}
	// 8248BFBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248BFC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248BFC4: 48D1C1F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248BFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248BFC8 size=80
    let mut pc: u32 = 0x8248BFC8;
    'dispatch: loop {
        match pc {
            0x8248BFC8 => {
    //   block [0x8248BFC8..0x8248C018)
	// 8248BFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248BFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248BFD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248BFD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248BFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248BFDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248BFE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248BFE4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248BFE8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248BFEC: 4BFFF91D  bl 0x8248b908
	ctx.lr = 0x8248BFF0;
	sub_8248B908(ctx, base);
	// 8248BFF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248BFF4: 4BE34275  bl 0x822c0268
	ctx.lr = 0x8248BFF8;
	sub_822C0268(ctx, base);
	// 8248BFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248BFFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8248C000: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248C004: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248C008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C010: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C018 size=92
    let mut pc: u32 = 0x8248C018;
    'dispatch: loop {
        match pc {
            0x8248C018 => {
    //   block [0x8248C018..0x8248C074)
	// 8248C018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248C024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C02C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248C030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C034: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8248C038: 4BFFE5B9  bl 0x8248a5f0
	ctx.lr = 0x8248C03C;
	sub_8248A5F0(ctx, base);
	// 8248C03C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8248C040: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248C044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C048: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248C04C: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8248C050: 794707E6  rldicr r7, r10, 0x20, 0x3f
	ctx.r[7].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 8248C054: E8CB0008  ld r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8248C058: 4BFFFDF9  bl 0x8248be50
	ctx.lr = 0x8248C05C;
	sub_8248BE50(ctx, base);
	// 8248C05C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248C060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C068: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248C06C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C078 size=172
    let mut pc: u32 = 0x8248C078;
    'dispatch: loop {
        match pc {
            0x8248C078 => {
    //   block [0x8248C078..0x8248C124)
	// 8248C078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C084: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C088: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8248C08C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 8248C090: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248C094: 409A000C  bne cr6, 0x8248c0a0
	if !ctx.cr[6].eq {
	pc = 0x8248C0A0; continue 'dispatch;
	}
	// 8248C098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8248C09C: 48000010  b 0x8248c0ac
	pc = 0x8248C0AC; continue 'dispatch;
	// 8248C0A0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248C0A4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8248C0A8: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 8248C0AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248C0B0: 419A0054  beq cr6, 0x8248c104
	if ctx.cr[6].eq {
	pc = 0x8248C104; continue 'dispatch;
	}
	// 8248C0B4: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248C0B8: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 8248C0BC: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8248C0C0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248C0C4: 40980040  bge cr6, 0x8248c104
	if !ctx.cr[6].lt {
	pc = 0x8248C104; continue 'dispatch;
	}
	// 8248C0C8: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248C0CC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8248C0D0: 419A0028  beq cr6, 0x8248c0f8
	if ctx.cr[6].eq {
	pc = 0x8248C0F8; continue 'dispatch;
	}
	// 8248C0D4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8248C0D8: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8248C0DC: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8248C0E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8248C0E4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C0E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248C0EC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C0F0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248C0F4: 4200FFF0  bdnz 0x8248c0e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248C0E4; continue 'dispatch;
	}
	// 8248C0F8: 39680014  addi r11, r8, 0x14
	ctx.r[11].s64 = ctx.r[8].s64 + 20;
	// 8248C0FC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248C100: 48000014  b 0x8248c114
	pc = 0x8248C114; continue 'dispatch;
	// 8248C104: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248C108: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248C10C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C110: 4BFFFDF1  bl 0x8248bf00
	ctx.lr = 0x8248C114;
	sub_8248BF00(ctx, base);
	// 8248C114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248C118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C128 size=236
    let mut pc: u32 = 0x8248C128;
    'dispatch: loop {
        match pc {
            0x8248C128 => {
    //   block [0x8248C128..0x8248C214)
	// 8248C128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C12C: 48D1C031  bl 0x831a815c
	ctx.lr = 0x8248C130;
	sub_831A8130(ctx, base);
	// 8248C130: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C134: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8248C138: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8248C13C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248C140: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8248C144: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 8248C148: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C14C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C150: 4800002C  b 0x8248c17c
	pc = 0x8248C17C; continue 'dispatch;
	// 8248C154: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 8248C158: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248C15C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8248C160: 4BFFF831  bl 0x8248b990
	ctx.lr = 0x8248C164;
	sub_8248B990(ctx, base);
	// 8248C164: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8248C168: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248C16C: 4182000C  beq 0x8248c178
	if ctx.cr[0].eq {
	pc = 0x8248C178; continue 'dispatch;
	}
	// 8248C170: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C174: 48000008  b 0x8248c17c
	pc = 0x8248C17C; continue 'dispatch;
	// 8248C178: 83BD0008  lwz r29, 8(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248C17C: 897D0025  lbz r11, 0x25(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(37 as u32) ) } as u64;
	// 8248C180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248C184: 419AFFD0  beq cr6, 0x8248c154
	if ctx.cr[6].eq {
	pc = 0x8248C154; continue 'dispatch;
	}
	// 8248C188: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8248C18C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248C190: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8248C194: 41820048  beq 0x8248c1dc
	if ctx.cr[0].eq {
	pc = 0x8248C1DC; continue 'dispatch;
	}
	// 8248C198: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C19C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C1A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C1A4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248C1A8: 409A002C  bne cr6, 0x8248c1d4
	if !ctx.cr[6].eq {
	pc = 0x8248C1D4; continue 'dispatch;
	}
	// 8248C1AC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8248C1B0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8248C1B4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8248C1B8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8248C1BC: 4BFFE71D  bl 0x8248a8d8
	ctx.lr = 0x8248C1C0;
	sub_8248A8D8(ctx, base);
	// 8248C1C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8248C1C4: 9B3F0004  stb r25, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u8 ) };
	// 8248C1C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C1CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248C1D0: 48000038  b 0x8248c208
	pc = 0x8248C208; continue 'dispatch;
	// 8248C1D4: 4BE6CDE5  bl 0x822f8fb8
	ctx.lr = 0x8248C1D8;
	sub_822F8FB8(ctx, base);
	// 8248C1D8: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8248C1DC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8248C1E0: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 8248C1E4: 4BFFF7AD  bl 0x8248b990
	ctx.lr = 0x8248C1E8;
	sub_8248B990(ctx, base);
	// 8248C1E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248C1EC: 41820010  beq 0x8248c1fc
	if ctx.cr[0].eq {
	pc = 0x8248C1FC; continue 'dispatch;
	}
	// 8248C1F0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8248C1F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C1F8: 4BFFFFB8  b 0x8248c1b0
	pc = 0x8248C1B0; continue 'dispatch;
	// 8248C1FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C200: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8248C204: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8248C208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C20C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8248C210: 48D1BF9C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C218 size=112
    let mut pc: u32 = 0x8248C218;
    'dispatch: loop {
        match pc {
            0x8248C218 => {
    //   block [0x8248C218..0x8248C288)
	// 8248C218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C21C: 48D1BF51  bl 0x831a816c
	ctx.lr = 0x8248C220;
	sub_831A8130(ctx, base);
	// 8248C220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C224: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248C228: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8248C22C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248C230: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248C234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C238: 4BFFFD39  bl 0x8248bf70
	ctx.lr = 0x8248C23C;
	sub_8248BF70(ctx, base);
	// 8248C23C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C240: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8248C244: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248C248: 419A0020  beq cr6, 0x8248c268
	if ctx.cr[6].eq {
	pc = 0x8248C268; continue 'dispatch;
	}
	// 8248C24C: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 8248C250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248C254: 4BFFF73D  bl 0x8248b990
	ctx.lr = 0x8248C258;
	sub_8248B990(ctx, base);
	// 8248C258: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248C25C: 4082000C  bne 0x8248c268
	if !ctx.cr[0].eq {
	pc = 0x8248C268; continue 'dispatch;
	}
	// 8248C260: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8248C264: 48000010  b 0x8248c274
	pc = 0x8248C274; continue 'dispatch;
	// 8248C268: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C26C: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8248C270: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8248C274: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248C27C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248C280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248C284: 48D1BF38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C288 size=228
    let mut pc: u32 = 0x8248C288;
    'dispatch: loop {
        match pc {
            0x8248C288 => {
    //   block [0x8248C288..0x8248C36C)
	// 8248C288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C290: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248C294: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C298: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C29C: 50A6422E  rlwimi r6, r5, 8, 8, 0x17
	ctx.r[6].u64 = (((ctx.r[5].u32).rotate_left(8) as u64) & 0x0000000000FFFF00) | (ctx.r[6].u64 & 0xFFFFFFFFFF0000FF);
	// 8248C2A0: F8810068  std r4, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[4].u64 ) };
	// 8248C2A4: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 8248C2A8: F8610060  std r3, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u64 ) };
	// 8248C2AC: 54CB023E  clrlwi r11, r6, 8
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 8248C2B0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8248C2B4: 5167402E  rlwimi r7, r11, 8, 0, 0x17
	ctx.r[7].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[7].u64 & 0xFFFFFFFF000000FF);
	// 8248C2B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C2BC: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8248C2C0: 809E6670  lwz r4, 0x6670(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26224 as u32) ) } as u64;
	// 8248C2C4: 4BFFFF55  bl 0x8248c218
	ctx.lr = 0x8248C2C8;
	sub_8248C218(ctx, base);
	// 8248C2C8: 817E6670  lwz r11, 0x6670(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26224 as u32) ) } as u64;
	// 8248C2CC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8248C2D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C2D4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8248C2D8: 419A000C  beq cr6, 0x8248c2e4
	if ctx.cr[6].eq {
	pc = 0x8248C2E4; continue 'dispatch;
	}
	// 8248C2DC: 806A0020  lwz r3, 0x20(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8248C2E0: 48000074  b 0x8248c354
	pc = 0x8248C354; continue 'dispatch;
	// 8248C2E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C2E8: 806B6674  lwz r3, 0x6674(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C2EC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C2F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248C2F4: 409A000C  bne cr6, 0x8248c300
	if !ctx.cr[6].eq {
	pc = 0x8248C300; continue 'dispatch;
	}
	// 8248C2F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8248C2FC: 48000014  b 0x8248c310
	pc = 0x8248C310; continue 'dispatch;
	// 8248C300: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248C304: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 8248C308: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8248C30C: 7FEB4BD6  divw r31, r11, r9
	ctx.r[31].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8248C310: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8248C314: 4BFFFD65  bl 0x8248c078
	ctx.lr = 0x8248C318;
	sub_8248C078(ctx, base);
	// 8248C318: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8248C31C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8248C320: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8248C324: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8248C328: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C32C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248C330: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C334: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8248C338: 4200FFF0  bdnz 0x8248c328
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8248C328; continue 'dispatch;
	}
	// 8248C33C: 93E10094  stw r31, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 8248C340: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8248C344: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C348: 809E6670  lwz r4, 0x6670(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26224 as u32) ) } as u64;
	// 8248C34C: 4BFFFDDD  bl 0x8248c128
	ctx.lr = 0x8248C350;
	sub_8248C128(ctx, base);
	// 8248C350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C354: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8248C358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C360: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248C364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C370 size=208
    let mut pc: u32 = 0x8248C370;
    'dispatch: loop {
        match pc {
            0x8248C370 => {
    //   block [0x8248C370..0x8248C440)
	// 8248C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C37C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C380: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C384: F8810088  std r4, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[4].u64 ) };
	// 8248C388: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248C38C: F8A10090  std r5, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[5].u64 ) };
	// 8248C390: 806B6674  lwz r3, 0x6674(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C394: 4BFFEFC5  bl 0x8248b358
	ctx.lr = 0x8248C398;
	sub_8248B358(ctx, base);
	// 8248C398: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248C39C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8248C3A0: 4BE58B21  bl 0x822e4ec0
	ctx.lr = 0x8248C3A4;
	sub_822E4EC0(ctx, base);
	// 8248C3A4: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8248C3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C3AC: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8248C3B0: 3941005C  addi r10, r1, 0x5c
	ctx.r[10].s64 = ctx.r[1].s64 + 92;
	// 8248C3B4: 3921008C  addi r9, r1, 0x8c
	ctx.r[9].s64 = ctx.r[1].s64 + 140;
	// 8248C3B8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248C3BC: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8248C3C0: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248C3C4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C3C8: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8248C3CC: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 8248C3D0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C3D4: 4098FFDC  bge cr6, 0x8248c3b0
	if !ctx.cr[6].lt {
	pc = 0x8248C3B0; continue 'dispatch;
	}
	// 8248C3D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C3DC: 4BE58AE5  bl 0x822e4ec0
	ctx.lr = 0x8248C3E0;
	sub_822E4EC0(ctx, base);
	// 8248C3E0: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8248C3E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C3E8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8248C3EC: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8248C3F0: 39210094  addi r9, r1, 0x94
	ctx.r[9].s64 = ctx.r[1].s64 + 148;
	// 8248C3F4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248C3F8: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8248C3FC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248C400: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C404: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8248C408: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 8248C40C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C410: 4098FFDC  bge cr6, 0x8248c3ec
	if !ctx.cr[6].lt {
	pc = 0x8248C3EC; continue 'dispatch;
	}
	// 8248C414: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8248C418: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8248C41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248C420: E8610058  ld r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8248C424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248C428: 4BFFFE61  bl 0x8248c288
	ctx.lr = 0x8248C42C;
	sub_8248C288(ctx, base);
	// 8248C42C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248C430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C440 size=280
    let mut pc: u32 = 0x8248C440;
    'dispatch: loop {
        match pc {
            0x8248C440 => {
    //   block [0x8248C440..0x8248C558)
	// 8248C440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C444: 48D1BD29  bl 0x831a816c
	ctx.lr = 0x8248C448;
	sub_831A8130(ctx, base);
	// 8248C448: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C44C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C450: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8248C454: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248C458: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8248C45C: 806B6674  lwz r3, 0x6674(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C460: 4BFFEEF9  bl 0x8248b358
	ctx.lr = 0x8248C464;
	sub_8248B358(ctx, base);
	// 8248C464: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248C468: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8248C46C: 4BE58A55  bl 0x822e4ec0
	ctx.lr = 0x8248C470;
	sub_822E4EC0(ctx, base);
	// 8248C470: FBA10050  std r29, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u64 ) };
	// 8248C474: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248C478: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8248C47C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C480: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248C484: 7D2948F8  nor r9, r9, r9
	ctx.r[9].u64 = !(ctx.r[9].u64 | ctx.r[9].u64);
	// 8248C488: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C48C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248C490: 4080FFEC  bge 0x8248c47c
	if !ctx.cr[0].lt {
	pc = 0x8248C47C; continue 'dispatch;
	}
	// 8248C494: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8248C498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C49C: E9210050  ld r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8248C4A0: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8248C4A4: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8248C4A8: 3941005C  addi r10, r1, 0x5c
	ctx.r[10].s64 = ctx.r[1].s64 + 92;
	// 8248C4AC: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 8248C4B0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248C4B4: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8248C4B8: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248C4BC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C4C0: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8248C4C4: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 8248C4C8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C4CC: 4098FFDC  bge cr6, 0x8248c4a8
	if !ctx.cr[6].lt {
	pc = 0x8248C4A8; continue 'dispatch;
	}
	// 8248C4D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8248C4D4: 4BE589ED  bl 0x822e4ec0
	ctx.lr = 0x8248C4D8;
	sub_822E4EC0(ctx, base);
	// 8248C4D8: FBE10050  std r31, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u64 ) };
	// 8248C4DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248C4E0: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 8248C4E4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C4E8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8248C4EC: 7D2948F8  nor r9, r9, r9
	ctx.r[9].u64 = !(ctx.r[9].u64 | ctx.r[9].u64);
	// 8248C4F0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C4F4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248C4F8: 4080FFEC  bge 0x8248c4e4
	if !ctx.cr[0].lt {
	pc = 0x8248C4E4; continue 'dispatch;
	}
	// 8248C4FC: E95E0008  ld r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 8248C500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C504: E9210050  ld r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8248C508: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8248C50C: F9210060  std r9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 8248C510: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8248C514: 39210064  addi r9, r1, 0x64
	ctx.r[9].s64 = ctx.r[1].s64 + 100;
	// 8248C518: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8248C51C: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8248C520: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8248C524: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C528: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8248C52C: 7D294038  and r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[8].u64;
	// 8248C530: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C534: 4098FFDC  bge cr6, 0x8248c510
	if !ctx.cr[6].lt {
	pc = 0x8248C510; continue 'dispatch;
	}
	// 8248C538: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8248C53C: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8248C540: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248C544: E8610058  ld r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8248C548: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248C54C: 4BFFFD3D  bl 0x8248c288
	ctx.lr = 0x8248C550;
	sub_8248C288(ctx, base);
	// 8248C550: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8248C554: 48D1BC68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C558 size=96
    let mut pc: u32 = 0x8248C558;
    'dispatch: loop {
        match pc {
            0x8248C558 => {
    //   block [0x8248C558..0x8248C5B8)
	// 8248C558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C55C: 48D1BC09  bl 0x831a8164
	ctx.lr = 0x8248C560;
	sub_831A8130(ctx, base);
	// 8248C560: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C564: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C568: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248C56C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8248C570: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8248C574: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8248C578: 806B6674  lwz r3, 0x6674(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C57C: 4BFFEDDD  bl 0x8248b358
	ctx.lr = 0x8248C580;
	sub_8248B358(ctx, base);
	// 8248C580: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8248C584: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C588: 4BE58939  bl 0x822e4ec0
	ctx.lr = 0x8248C58C;
	sub_822E4EC0(ctx, base);
	// 8248C58C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C590: EB7C0000  ld r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 8248C594: 4BE5892D  bl 0x822e4ec0
	ctx.lr = 0x8248C598;
	sub_822E4EC0(ctx, base);
	// 8248C598: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8248C59C: E89C0008  ld r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	// 8248C5A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8248C5A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8248C5A8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8248C5AC: 4BFFFCDD  bl 0x8248c288
	ctx.lr = 0x8248C5B0;
	sub_8248C288(ctx, base);
	// 8248C5B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8248C5B4: 48D1BC00  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C5B8 size=116
    let mut pc: u32 = 0x8248C5B8;
    'dispatch: loop {
        match pc {
            0x8248C5B8 => {
    //   block [0x8248C5B8..0x8248C62C)
	// 8248C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C5C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C5C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C5C8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8248C5CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C5D0: 3BEB6678  addi r31, r11, 0x6678
	ctx.r[31].s64 = ctx.r[11].s64 + 26232;
	// 8248C5D4: 816A6698  lwz r11, 0x6698(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26264 as u32) ) } as u64;
	// 8248C5D8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8248C5DC: 40820038  bne 0x8248c614
	if !ctx.cr[0].eq {
	pc = 0x8248C614; continue 'dispatch;
	}
	// 8248C5E0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8248C5E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C5E8: 916A6698  stw r11, 0x6698(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26264 as u32), ctx.r[11].u32 ) };
	// 8248C5EC: 48488BBD  bl 0x829151a8
	ctx.lr = 0x8248C5F0;
	sub_829151A8(ctx, base);
	// 8248C5F0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8248C5F4: 48488BB5  bl 0x829151a8
	ctx.lr = 0x8248C5F8;
	sub_829151A8(ctx, base);
	// 8248C5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C5FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8248C600: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8248C604: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 8248C608: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8248C60C: 386BF4C0  addi r3, r11, -0xb40
	ctx.r[3].s64 = ctx.r[11].s64 + -2880;
	// 8248C610: 48D1BEC9  bl 0x831a84d8
	ctx.lr = 0x8248C614;
	sub_831A84D8(ctx, base);
	// 8248C614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248C61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C630 size=72
    let mut pc: u32 = 0x8248C630;
    'dispatch: loop {
        match pc {
            0x8248C630 => {
    //   block [0x8248C630..0x8248C678)
	// 8248C630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C63C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C640: 4BFFFF79  bl 0x8248c5b8
	ctx.lr = 0x8248C644;
	sub_8248C5B8(ctx, base);
	// 8248C644: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248C64C: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 8248C650: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C654: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C658: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248C65C: 4E800421  bctrl
	ctx.lr = 0x8248C660;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248C660: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8248C664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248C668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C678 size=288
    let mut pc: u32 = 0x8248C678;
    'dispatch: loop {
        match pc {
            0x8248C678 => {
    //   block [0x8248C678..0x8248C798)
	// 8248C678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C67C: 48D1BAED  bl 0x831a8168
	ctx.lr = 0x8248C680;
	sub_831A8130(ctx, base);
	// 8248C680: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C684: 4BFFFFAD  bl 0x8248c630
	ctx.lr = 0x8248C688;
	sub_8248C630(ctx, base);
	// 8248C688: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248C68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248C690: 3BABBC30  addi r29, r11, -0x43d0
	ctx.r[29].s64 = ctx.r[11].s64 + -17360;
	// 8248C694: 38A0016F  li r5, 0x16f
	ctx.r[5].s64 = 367;
	// 8248C698: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8248C69C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8248C6A0: 4BE33D39  bl 0x822c03d8
	ctx.lr = 0x8248C6A4;
	sub_822C03D8(ctx, base);
	// 8248C6A4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8248C6A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8248C6AC: 41820010  beq 0x8248c6bc
	if ctx.cr[0].eq {
	pc = 0x8248C6BC; continue 'dispatch;
	}
	// 8248C6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C6B4: 4BFFE00D  bl 0x8248a6c0
	ctx.lr = 0x8248C6B8;
	sub_8248A6C0(ctx, base);
	// 8248C6B8: 48000008  b 0x8248c6c0
	pc = 0x8248C6C0; continue 'dispatch;
	// 8248C6BC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 8248C6C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C6C4: 83CB6670  lwz r30, 0x6670(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26224 as u32) ) } as u64;
	// 8248C6C8: 93EB6670  stw r31, 0x6670(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(26224 as u32), ctx.r[31].u32 ) };
	// 8248C6CC: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248C6D0: 41820014  beq 0x8248c6e4
	if ctx.cr[0].eq {
	pc = 0x8248C6E4; continue 'dispatch;
	}
	// 8248C6D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248C6D8: 4BFFF1D9  bl 0x8248b8b0
	ctx.lr = 0x8248C6DC;
	sub_8248B8B0(ctx, base);
	// 8248C6DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248C6E0: 4BE33B89  bl 0x822c0268
	ctx.lr = 0x8248C6E4;
	sub_822C0268(ctx, base);
	// 8248C6E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8248C6E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248C6EC: 38A00170  li r5, 0x170
	ctx.r[5].s64 = 368;
	// 8248C6F0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8248C6F4: 4BE33CE5  bl 0x822c03d8
	ctx.lr = 0x8248C6F8;
	sub_822C03D8(ctx, base);
	// 8248C6F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248C6FC: 41820014  beq 0x8248c710
	if ctx.cr[0].eq {
	pc = 0x8248C710; continue 'dispatch;
	}
	// 8248C700: 93830004  stw r28, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8248C704: 93830008  stw r28, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8248C708: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8248C70C: 48000008  b 0x8248c714
	pc = 0x8248C714; continue 'dispatch;
	// 8248C710: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8248C714: 3FE08335  lis r31, -0x7ccb
	ctx.r[31].s64 = -2093678592;
	// 8248C718: 83DF6674  lwz r30, 0x6674(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C71C: 907F6674  stw r3, 0x6674(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26228 as u32), ctx.r[3].u32 ) };
	// 8248C720: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248C724: 41820018  beq 0x8248c73c
	if ctx.cr[0].eq {
	pc = 0x8248C73C; continue 'dispatch;
	}
	// 8248C728: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248C72C: 4BFDD8D5  bl 0x8246a000
	ctx.lr = 0x8248C730;
	sub_8246A000(ctx, base);
	// 8248C730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248C734: 4BE33B35  bl 0x822c0268
	ctx.lr = 0x8248C738;
	sub_822C0268(ctx, base);
	// 8248C738: 807F6674  lwz r3, 0x6674(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C73C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248C740: 4BFFF8D9  bl 0x8248c018
	ctx.lr = 0x8248C744;
	sub_8248C018(ctx, base);
	// 8248C744: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8248C748: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8248C74C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C750: 916A6668  stw r11, 0x6668(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26216 as u32), ctx.r[11].u32 ) };
	// 8248C754: 4BE5876D  bl 0x822e4ec0
	ctx.lr = 0x8248C758;
	sub_822E4EC0(ctx, base);
	// 8248C758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248C75C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8248C760: 4BE58761  bl 0x822e4ec0
	ctx.lr = 0x8248C764;
	sub_822E4EC0(ctx, base);
	// 8248C764: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8248C768: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248C76C: E89F0000  ld r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8248C770: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8248C774: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8248C778: 4BFFFB11  bl 0x8248c288
	ctx.lr = 0x8248C77C;
	sub_8248C288(ctx, base);
	// 8248C77C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C780: 906B666C  stw r3, 0x666c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(26220 as u32), ctx.r[3].u32 ) };
	// 8248C784: 48006455  bl 0x82492bd8
	ctx.lr = 0x8248C788;
	sub_82492BD8(ctx, base);
	// 8248C788: 48007811  bl 0x82493f98
	ctx.lr = 0x8248C78C;
	sub_82493F98(ctx, base);
	// 8248C78C: 4800717D  bl 0x82493908
	ctx.lr = 0x8248C790;
	sub_82493908(ctx, base);
	// 8248C790: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8248C794: 48D1BA24  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C798 size=168
    let mut pc: u32 = 0x8248C798;
    'dispatch: loop {
        match pc {
            0x8248C798 => {
    //   block [0x8248C798..0x8248C840)
	// 8248C798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C7A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248C7A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C7A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C7AC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8248C7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C7B4: 83EA6670  lwz r31, 0x6670(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26224 as u32) ) } as u64;
	// 8248C7B8: 916A6670  stw r11, 0x6670(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26224 as u32), ctx.r[11].u32 ) };
	// 8248C7BC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248C7C0: 41820014  beq 0x8248c7d4
	if ctx.cr[0].eq {
	pc = 0x8248C7D4; continue 'dispatch;
	}
	// 8248C7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C7C8: 4BFFF0E9  bl 0x8248b8b0
	ctx.lr = 0x8248C7CC;
	sub_8248B8B0(ctx, base);
	// 8248C7CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C7D0: 4BE33A99  bl 0x822c0268
	ctx.lr = 0x8248C7D4;
	sub_822C0268(ctx, base);
	// 8248C7D4: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 8248C7D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C7DC: 809E6674  lwz r4, 0x6674(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C7E0: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248C7E4: 80A40004  lwz r5, 4(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248C7E8: 4803F1D1  bl 0x824cb9b8
	ctx.lr = 0x8248C7EC;
	sub_824CB9B8(ctx, base);
	// 8248C7EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C7F0: 83FE6674  lwz r31, 0x6674(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(26228 as u32) ) } as u64;
	// 8248C7F4: 917E6674  stw r11, 0x6674(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(26228 as u32), ctx.r[11].u32 ) };
	// 8248C7F8: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248C7FC: 41820014  beq 0x8248c810
	if ctx.cr[0].eq {
	pc = 0x8248C810; continue 'dispatch;
	}
	// 8248C800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C804: 4BFDD7FD  bl 0x8246a000
	ctx.lr = 0x8248C808;
	sub_8246A000(ctx, base);
	// 8248C808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C80C: 4BE33A5D  bl 0x822c0268
	ctx.lr = 0x8248C810;
	sub_822C0268(ctx, base);
	// 8248C810: 4BFFFDA9  bl 0x8248c5b8
	ctx.lr = 0x8248C814;
	sub_8248C5B8(ctx, base);
	// 8248C814: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8248C818: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8248C81C: 808A001C  lwz r4, 0x1c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8248C820: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8248C824: 48965965  bl 0x82df2188
	ctx.lr = 0x8248C828;
	sub_82DF2188(ctx, base);
	// 8248C828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248C82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C834: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248C838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C840 size=96
    let mut pc: u32 = 0x8248C840;
    'dispatch: loop {
        match pc {
            0x8248C840 => {
    //   block [0x8248C840..0x8248C8A0)
	// 8248C840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248C84C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248C858: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8248C85C: 4BE5D70D  bl 0x822e9f68
	ctx.lr = 0x8248C860;
	sub_822E9F68(ctx, base);
	// 8248C860: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248C864: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8248C868: 396BBC84  addi r11, r11, -0x437c
	ctx.r[11].s64 = ctx.r[11].s64 + -17276;
	// 8248C86C: 394ABC78  addi r10, r10, -0x4388
	ctx.r[10].s64 = ctx.r[10].s64 + -17288;
	// 8248C870: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248C874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C878: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8248C87C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248C880: 480027E9  bl 0x8248f068
	ctx.lr = 0x8248C884;
	sub_8248F068(ctx, base);
	// 8248C884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248C88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C894: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248C898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248C8A0 size=4
    let mut pc: u32 = 0x8248C8A0;
    'dispatch: loop {
        match pc {
            0x8248C8A0 => {
    //   block [0x8248C8A0..0x8248C8A4)
	// 8248C8A0: 4BE5D000  b 0x822e98a0
	sub_822E98A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C8A8 size=56
    let mut pc: u32 = 0x8248C8A8;
    'dispatch: loop {
        match pc {
            0x8248C8A8 => {
    //   block [0x8248C8A8..0x8248C8E0)
	// 8248C8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C8B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C8B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C8B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248C8BC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8248C8C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248C8C4: 4BE5D52D  bl 0x822e9df0
	ctx.lr = 0x8248C8C8;
	sub_822E9DF0(ctx, base);
	// 8248C8C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C8CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248C8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C8D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C8E0 size=196
    let mut pc: u32 = 0x8248C8E0;
    'dispatch: loop {
        match pc {
            0x8248C8E0 => {
    //   block [0x8248C8E0..0x8248C9A4)
	// 8248C8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248C8E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248C8EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248C8F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C8F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248C8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C8FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8248C900: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248C904: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248C908: 4BE34031  bl 0x822c0938
	ctx.lr = 0x8248C90C;
	sub_822C0938(ctx, base);
	// 8248C90C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248C910: 41820028  beq 0x8248c938
	if ctx.cr[0].eq {
	pc = 0x8248C938; continue 'dispatch;
	}
	// 8248C914: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248C918: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8248C91C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8248C920: 392BBCBC  addi r9, r11, -0x4344
	ctx.r[9].s64 = ctx.r[11].s64 + -17220;
	// 8248C924: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8248C928: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8248C92C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8248C930: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8248C934: 48000008  b 0x8248c93c
	pc = 0x8248C93C; continue 'dispatch;
	// 8248C938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8248C93C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8248C940: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248C944: 409A0044  bne cr6, 0x8248c988
	if !ctx.cr[6].eq {
	pc = 0x8248C988; continue 'dispatch;
	}
	// 8248C948: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8248C94C: 419A001C  beq cr6, 0x8248c968
	if ctx.cr[6].eq {
	pc = 0x8248C968; continue 'dispatch;
	}
	// 8248C950: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C954: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8248C958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248C95C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248C960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8248C964: 4E800421  bctrl
	ctx.lr = 0x8248C968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8248C968: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 8248C96C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8248C970: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248C974: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8248C978: 816B797C  lwz r11, 0x797c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31100 as u32) ) } as u64;
	// 8248C97C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8248C980: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8248C984: 4BE3367D  bl 0x822c0000
	ctx.lr = 0x8248C988;
	sub_822C0000(ctx, base);
	// 8248C988: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248C98C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248C990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248C994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248C998: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248C99C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248C9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248C9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248C9A8 size=188
    let mut pc: u32 = 0x8248C9A8;
    'dispatch: loop {
        match pc {
            0x8248C9A8 => {
    //   block [0x8248C9A8..0x8248CA64)
	// 8248C9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248C9AC: 48D1B7B1  bl 0x831a815c
	ctx.lr = 0x8248C9B0;
	sub_831A8130(ctx, base);
	// 8248C9B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248C9B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248C9B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8248C9BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248C9C0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8248C9C4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8248C9C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248C9CC: 388BBCCC  addi r4, r11, -0x4334
	ctx.r[4].s64 = ctx.r[11].s64 + -17204;
	// 8248C9D0: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 8248C9D4: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8248C9D8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8248C9DC: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 8248C9E0: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8248C9E4: 48965A05  bl 0x82df23e8
	ctx.lr = 0x8248C9E8;
	sub_82DF23E8(ctx, base);
	// 8248C9E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248C9EC: 41820018  beq 0x8248ca04
	if ctx.cr[0].eq {
	pc = 0x8248CA04; continue 'dispatch;
	}
	// 8248C9F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8248C9F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248C9F8: 4BFFFE49  bl 0x8248c840
	ctx.lr = 0x8248C9FC;
	sub_8248C840(ctx, base);
	// 8248C9FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248CA00: 48000008  b 0x8248ca08
	pc = 0x8248CA08; continue 'dispatch;
	// 8248CA04: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8248CA08: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8248CA0C: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 8248CA10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248CA14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248CA18: 4BFFFEC9  bl 0x8248c8e0
	ctx.lr = 0x8248CA1C;
	sub_8248C8E0(ctx, base);
	// 8248CA1C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8248CA20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248CA24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248CA28: 4BE335D9  bl 0x822c0000
	ctx.lr = 0x8248CA2C;
	sub_822C0000(ctx, base);
	// 8248CA2C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CA30: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248CA34: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8248CA38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8248CA3C: 4182000C  beq 0x8248ca48
	if ctx.cr[0].eq {
	pc = 0x8248CA48; continue 'dispatch;
	}
	// 8248CA40: 4BE5D989  bl 0x822ea3c8
	ctx.lr = 0x8248CA44;
	sub_822EA3C8(ctx, base);
	// 8248CA44: 48000008  b 0x8248ca4c
	pc = 0x8248CA4C; continue 'dispatch;
	// 8248CA48: 4BE5D819  bl 0x822ea260
	ctx.lr = 0x8248CA4C;
	sub_822EA260(ctx, base);
	// 8248CA4C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8248CA50: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CA54: 4BE5CE4D  bl 0x822e98a0
	ctx.lr = 0x8248CA58;
	sub_822E98A0(ctx, base);
	// 8248CA58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248CA5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8248CA60: 48D1B74C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248CA68 size=276
    let mut pc: u32 = 0x8248CA68;
    'dispatch: loop {
        match pc {
            0x8248CA68 => {
    //   block [0x8248CA68..0x8248CB7C)
	// 8248CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248CA6C: 48D1B701  bl 0x831a816c
	ctx.lr = 0x8248CA70;
	sub_831A8130(ctx, base);
	// 8248CA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248CA74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8248CA78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8248CA7C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CA80: 485D7271  bl 0x82a63cf0
	ctx.lr = 0x8248CA84;
	sub_82A63CF0(ctx, base);
	// 8248CA84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8248CA88: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8248CA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8248CA90: 386000EC  li r3, 0xec
	ctx.r[3].s64 = 236;
	// 8248CA94: 388BBCCC  addi r4, r11, -0x4334
	ctx.r[4].s64 = ctx.r[11].s64 + -17204;
	// 8248CA98: 41820054  beq 0x8248caec
	if ctx.cr[0].eq {
	pc = 0x8248CAEC; continue 'dispatch;
	}
	// 8248CA9C: 38A0005B  li r5, 0x5b
	ctx.r[5].s64 = 91;
	// 8248CAA0: 48965949  bl 0x82df23e8
	ctx.lr = 0x8248CAA4;
	sub_82DF23E8(ctx, base);
	// 8248CAA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248CAA8: 418200A0  beq 0x8248cb48
	if ctx.cr[0].eq {
	pc = 0x8248CB48; continue 'dispatch;
	}
	// 8248CAAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CAB0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CAB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248CAB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8248CABC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8248CAC0: 419A0024  beq cr6, 0x8248cae4
	if ctx.cr[6].eq {
	pc = 0x8248CAE4; continue 'dispatch;
	}
	// 8248CAC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248CAC8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248CACC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248CAD0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8248CAD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8248CAD8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8248CADC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248CAE0: 4082FFE8  bne 0x8248cac8
	if !ctx.cr[0].eq {
	pc = 0x8248CAC8; continue 'dispatch;
	}
	// 8248CAE4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8248CAE8: 48000050  b 0x8248cb38
	pc = 0x8248CB38; continue 'dispatch;
	// 8248CAEC: 38A0005E  li r5, 0x5e
	ctx.r[5].s64 = 94;
	// 8248CAF0: 489658F9  bl 0x82df23e8
	ctx.lr = 0x8248CAF4;
	sub_82DF23E8(ctx, base);
	// 8248CAF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8248CAF8: 41820050  beq 0x8248cb48
	if ctx.cr[0].eq {
	pc = 0x8248CB48; continue 'dispatch;
	}
	// 8248CAFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CB00: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CB04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248CB08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8248CB0C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8248CB10: 419A0024  beq cr6, 0x8248cb34
	if ctx.cr[6].eq {
	pc = 0x8248CB34; continue 'dispatch;
	}
	// 8248CB14: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248CB18: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248CB1C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248CB20: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8248CB24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8248CB28: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8248CB2C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248CB30: 4082FFE8  bne 0x8248cb18
	if !ctx.cr[0].eq {
	pc = 0x8248CB18; continue 'dispatch;
	}
	// 8248CB34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8248CB38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8248CB3C: 48001875  bl 0x8248e3b0
	ctx.lr = 0x8248CB40;
	sub_8248E3B0(ctx, base);
	// 8248CB40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248CB44: 48000008  b 0x8248cb4c
	pc = 0x8248CB4C; continue 'dispatch;
	// 8248CB48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8248CB4C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8248CB50: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8248CB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248CB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248CB5C: 4BE610F5  bl 0x822edc50
	ctx.lr = 0x8248CB60;
	sub_822EDC50(ctx, base);
	// 8248CB60: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8248CB64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8248CB68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8248CB6C: 4BE33495  bl 0x822c0000
	ctx.lr = 0x8248CB70;
	sub_822C0000(ctx, base);
	// 8248CB70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8248CB74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8248CB78: 48D1B644  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248CB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248CB80 size=48
    let mut pc: u32 = 0x8248CB80;
    'dispatch: loop {
        match pc {
            0x8248CB80 => {
    //   block [0x8248CB80..0x8248CBB0)
	// 8248CB80: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CB84: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CB88: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8248CB8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8248CB90: 409A0030  bne cr6, 0x8248cbc0
	if !ctx.cr[6].eq {
		sub_8248CBB0(ctx, base);
		return;
	}
	// 8248CB94: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CB98: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248CB9C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248CBA0: 40980010  bge cr6, 0x8248cbb0
	if !ctx.cr[6].lt {
		sub_8248CBB0(ctx, base);
		return;
	}
	// 8248CBA4: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8248CBA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CBAC: 48000008  b 0x8248cbb4
	sub_8248CBB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248CBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248CBB0 size=60
    let mut pc: u32 = 0x8248CBB0;
    'dispatch: loop {
        match pc {
            0x8248CBB0 => {
    //   block [0x8248CBB0..0x8248CBEC)
	// 8248CBB0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248CBB4: 892B0019  lbz r9, 0x19(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8248CBB8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248CBBC: 419AFFDC  beq cr6, 0x8248cb98
	if ctx.cr[6].eq {
		sub_8248CB80(ctx, base);
		return;
	}
	// 8248CBC0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CBC4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CBC8: 892B0019  lbz r9, 0x19(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8248CBCC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8248CBD0: 409A0030  bne cr6, 0x8248cc00
	if !ctx.cr[6].eq {
		sub_8248CBEC(ctx, base);
		return;
	}
	// 8248CBD4: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CBD8: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8248CBDC: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8248CBE0: 4098000C  bge cr6, 0x8248cbec
	if !ctx.cr[6].lt {
		sub_8248CBEC(ctx, base);
		return;
	}
	// 8248CBE4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248CBE8: 4800000C  b 0x8248cbf4
	sub_8248CBEC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248CBEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8248CBEC size=32
    let mut pc: u32 = 0x8248CBEC;
    'dispatch: loop {
        match pc {
            0x8248CBEC => {
    //   block [0x8248CBEC..0x8248CC0C)
	// 8248CBEC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8248CBF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CBF4: 88EB0019  lbz r7, 0x19(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 8248CBF8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8248CBFC: 419AFFDC  beq cr6, 0x8248cbd8
	if ctx.cr[6].eq {
		sub_8248CBB0(ctx, base);
		return;
	}
	// 8248CC00: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8248CC04: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8248CC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248CC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248CC10 size=124
    let mut pc: u32 = 0x8248CC10;
    'dispatch: loop {
        match pc {
            0x8248CC10 => {
    //   block [0x8248CC10..0x8248CC8C)
	// 8248CC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248CC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248CC18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248CC1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248CC20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248CC24: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8248CC28: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8248CC2C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8248CC30: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8248CC34: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CC38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8248CC3C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8248CC40: 419A0024  beq cr6, 0x8248cc64
	if ctx.cr[6].eq {
	pc = 0x8248CC64; continue 'dispatch;
	}
	// 8248CC44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8248CC48: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8248CC4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248CC50: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8248CC54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8248CC58: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8248CC5C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8248CC60: 4082FFE8  bne 0x8248cc48
	if !ctx.cr[0].eq {
	pc = 0x8248CC48; continue 'dispatch;
	}
	// 8248CC64: 80650004  lwz r3, 4(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8248CC68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8248CC6C: 419A0008  beq cr6, 0x8248cc74
	if ctx.cr[6].eq {
	pc = 0x8248CC74; continue 'dispatch;
	}
	// 8248CC70: 4BE33C21  bl 0x822c0890
	ctx.lr = 0x8248CC74;
	sub_822C0890(ctx, base);
	// 8248CC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8248CC78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8248CC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248CC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248CC84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248CC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248CC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248CC90 size=100
    let mut pc: u32 = 0x8248CC90;
    'dispatch: loop {
        match pc {
            0x8248CC90 => {
    //   block [0x8248CC90..0x8248CCF4)
	// 8248CC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248CC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248CC98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248CC9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248CCA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248CCA4: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8248CCA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248CCAC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8248CCB0: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8248CCB4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8248CCB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248CCBC: 4849220D  bl 0x8291eec8
	ctx.lr = 0x8248CCC0;
	sub_8291EEC8(ctx, base);
	// 8248CCC0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248CCC4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8248CCC8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248CCCC: 419A0010  beq cr6, 0x8248ccdc
	if ctx.cr[6].eq {
	pc = 0x8248CCDC; continue 'dispatch;
	}
	// 8248CCD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248CCD4: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248CCD8: 4BE5D441  bl 0x822ea118
	ctx.lr = 0x8248CCDC;
	sub_822EA118(ctx, base);
	// 8248CCDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248CCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248CCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248CCE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248CCEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248CCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8248CCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8248CCF8 size=100
    let mut pc: u32 = 0x8248CCF8;
    'dispatch: loop {
        match pc {
            0x8248CCF8 => {
    //   block [0x8248CCF8..0x8248CD5C)
	// 8248CCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8248CCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8248CD00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8248CD04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8248CD08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8248CD0C: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8248CD10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8248CD14: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8248CD18: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8248CD1C: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8248CD20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8248CD24: 484921A5  bl 0x8291eec8
	ctx.lr = 0x8248CD28;
	sub_8291EEC8(ctx, base);
	// 8248CD28: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8248CD2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8248CD30: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8248CD34: 419A0010  beq cr6, 0x8248cd44
	if ctx.cr[6].eq {
	pc = 0x8248CD44; continue 'dispatch;
	}
	// 8248CD38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8248CD3C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8248CD40: 4BFFFB61  bl 0x8248c8a0
	ctx.lr = 0x8248CD44;
	sub_8248C8A0(ctx, base);
	// 8248CD44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8248CD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8248CD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8248CD50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8248CD54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8248CD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


